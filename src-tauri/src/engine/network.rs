use serde::{Deserialize, Serialize};
use super::DryRunOperation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCleanupResult {
    pub operation: String,
    pub success: bool,
    pub error: Option<String>,
}

pub async fn analyze_network_cleanup(os: &super::os::OS) -> Vec<DryRunOperation> {
    vec![
        DryRunOperation {
            name: "DNS Flush".to_string(),
            path: "system".to_string(),
            file_count: 0,
            bytes: 0,
            would_modify: true,
        },
        DryRunOperation {
            name: "Hosts File Reset".to_string(),
            path: os.get_hosts_file_path(),
            file_count: 0,
            bytes: 0,
            would_modify: true,
        },
    ]
}

pub async fn flush_dns(os: super::os::OS) -> NetworkCleanupResult {
    let output = match os {
        super::os::OS::Windows => std::process::Command::new("ipconfig")
            .args(["/flushdns"])
            .output(),

        super::os::OS::MacOS => {
            // dscacheutil doesn't need privileges; mDNSResponder killall may need sudo
            let _ = std::process::Command::new("dscacheutil")
                .args(["-flushcache"])
                .output();
            std::process::Command::new("killall")
                .args(["-HUP", "mDNSResponder"])
                .output()
        }

        super::os::OS::Linux => {
            // resolvectl works without sudo on systemd-resolved systems via polkit
            match std::process::Command::new("resolvectl")
                .args(["flush-caches"])
                .output()
            {
                Ok(o) if o.status.success() => Ok(o),
                _ => {
                    // fallback: restart the resolver service
                    std::process::Command::new("systemctl")
                        .args(["restart", "systemd-resolved"])
                        .output()
                }
            }
        }
    };

    match output {
        Ok(o) if o.status.success() => NetworkCleanupResult {
            operation: "DNS Flush".to_string(),
            success: true,
            error: None,
        },
        Ok(o) => NetworkCleanupResult {
            operation: "DNS Flush".to_string(),
            success: false,
            error: Some(format!(
                "Command failed: {}",
                String::from_utf8_lossy(&o.stderr).trim()
            )),
        },
        Err(e) => NetworkCleanupResult {
            operation: "DNS Flush".to_string(),
            success: false,
            error: Some(format!("Failed to run command: {}", e)),
        },
    }
}

pub async fn reset_hosts_file(os: super::os::OS) -> NetworkCleanupResult {
    let hosts_path = os.get_hosts_file_path();

    let default_hosts = match &os {
        super::os::OS::Windows => {
            "# Copyright (c) 1993-2009 Microsoft Corp.\r\n\
             #\r\n\
             127.0.0.1       localhost\r\n\
             ::1             localhost\r\n"
        }
        _ => {
            "# Host Database\n\
             127.0.0.1       localhost\n\
             ::1             localhost\n"
        }
    };

    // Try direct write first (works if app has privileges or on Windows as admin)
    if let Ok(()) = std::fs::write(&hosts_path, default_hosts) {
        return NetworkCleanupResult {
            operation: "Hosts File Reset".to_string(),
            success: true,
            error: None,
        };
    }

    // Unprivileged — elevate via platform mechanism
    match os {
        super::os::OS::Windows => NetworkCleanupResult {
            operation: "Hosts File Reset".to_string(),
            success: false,
            error: Some("Run app as Administrator to reset hosts file".to_string()),
        },

        super::os::OS::Linux => {
            // Write content to temp file, then use pkexec to copy with elevation
            let temp = std::env::temp_dir().join("cleanslate_hosts.tmp");
            if let Err(e) = std::fs::write(&temp, default_hosts) {
                return NetworkCleanupResult {
                    operation: "Hosts File Reset".to_string(),
                    success: false,
                    error: Some(format!("Failed to write temp file: {}", e)),
                };
            }

            let result = std::process::Command::new("pkexec")
                .args(["cp", temp.to_str().unwrap_or(""), &hosts_path])
                .output();

            let _ = std::fs::remove_file(&temp);

            match result {
                Ok(o) if o.status.success() => NetworkCleanupResult {
                    operation: "Hosts File Reset".to_string(),
                    success: true,
                    error: None,
                },
                Ok(o) => NetworkCleanupResult {
                    operation: "Hosts File Reset".to_string(),
                    success: false,
                    error: Some(format!(
                        "pkexec failed: {}",
                        String::from_utf8_lossy(&o.stderr).trim()
                    )),
                },
                Err(e) => NetworkCleanupResult {
                    operation: "Hosts File Reset".to_string(),
                    success: false,
                    error: Some(format!("pkexec not available: {}", e)),
                },
            }
        }

        super::os::OS::MacOS => {
            // Use osascript to run with administrator privileges
            let script = format!(
                "do shell script \"echo '{}' > {}\" with administrator privileges",
                default_hosts.replace('\'', "'\\''"),
                hosts_path
            );

            match std::process::Command::new("osascript")
                .args(["-e", &script])
                .output()
            {
                Ok(o) if o.status.success() => NetworkCleanupResult {
                    operation: "Hosts File Reset".to_string(),
                    success: true,
                    error: None,
                },
                Ok(o) => NetworkCleanupResult {
                    operation: "Hosts File Reset".to_string(),
                    success: false,
                    error: Some(format!(
                        "osascript failed: {}",
                        String::from_utf8_lossy(&o.stderr).trim()
                    )),
                },
                Err(e) => NetworkCleanupResult {
                    operation: "Hosts File Reset".to_string(),
                    success: false,
                    error: Some(format!("osascript not available: {}", e)),
                },
            }
        }
    }
}

pub async fn cleanup_network(os: super::os::OS) -> Vec<NetworkCleanupResult> {
    vec![
        flush_dns(os.clone()).await,
        reset_hosts_file(os).await,
    ]
}
