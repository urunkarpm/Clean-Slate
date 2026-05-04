use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCleanupResult {
    pub operation: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DryRunOperation {
    pub name: String,
    pub path: String,
    pub file_count: u64,
    pub bytes: u64,
    pub would_modify: bool,
}

pub async fn analyze_network_cleanup(os: &super::os::OS) -> Vec<DryRunOperation> {
    let mut operations = Vec::new();
    
    // DNS flush - requires elevated privileges on most platforms
    // On Linux/macOS, this is skipped for safety
    operations.push(DryRunOperation {
        name: "DNS Flush".to_string(),
        path: "system".to_string(),
        file_count: 0,
        bytes: 0,
        would_modify: matches!(os, super::os::OS::Windows),
    });
    
    // Hosts file reset - requires elevated privileges
    // On Linux/macOS, this is skipped for safety to prevent system bricking
    operations.push(DryRunOperation {
        name: "Hosts File Reset".to_string(),
        path: os.get_hosts_file_path(),
        file_count: 0,
        bytes: 0,
        would_modify: matches!(os, super::os::OS::Windows),
    });
    
    operations
}

pub async fn flush_dns(os: super::os::OS) -> NetworkCleanupResult {
    match os {
        super::os::OS::Windows => {
            match std::process::Command::new("ipconfig").args(&["/flushdns"]).output() {
                Ok(output) => {
                    if output.status.success() {
                        NetworkCleanupResult {
                            operation: "DNS Flush".to_string(),
                            success: true,
                            error: None,
                        }
                    } else {
                        NetworkCleanupResult {
                            operation: "DNS Flush".to_string(),
                            success: false,
                            error: Some(format!("Command failed: {}", String::from_utf8_lossy(&output.stderr))),
                        }
                    }
                }
                Err(e) => NetworkCleanupResult {
                    operation: "DNS Flush".to_string(),
                    success: false,
                    error: Some(format!("Failed to execute command: {}", e)),
                },
            }
        }
        super::os::OS::MacOS => {
            match std::process::Command::new("killall").args(&["-HUP", "mDNSResponder"]).output() {
                Ok(output) => {
                    if output.status.success() {
                        NetworkCleanupResult {
                            operation: "DNS Flush".to_string(),
                            success: true,
                            error: None,
                        }
                    } else {
                        NetworkCleanupResult {
                            operation: "DNS Flush".to_string(),
                            success: false,
                            error: Some(format!("Command failed: {}", String::from_utf8_lossy(&output.stderr))),
                        }
                    }
                }
                Err(e) => NetworkCleanupResult {
                    operation: "DNS Flush".to_string(),
                    success: false,
                    error: Some(format!("Failed to execute command: {}", e)),
                },
            }
        }
        super::os::OS::Linux => {
            // Skip DNS flush on Linux as it requires sudo and varies by distribution
            // Modern Linux systems handle DNS caching differently (systemd-resolved, nscd, dnsmasq, etc.)
            NetworkCleanupResult {
                operation: "DNS Flush".to_string(),
                success: true,
                error: None,
            }
        }
    }
}

pub async fn reset_hosts_file(os: super::os::OS) -> NetworkCleanupResult {
    // Skip hosts file reset on Linux and macOS as it requires elevated privileges
    // and modifying /etc/hosts can brick the system if done incorrectly
    match os {
        super::os::OS::Linux | super::os::OS::MacOS => {
            NetworkCleanupResult {
                operation: "Hosts File Reset".to_string(),
                success: true,
                error: None,
            }
        }
        super::os::OS::Windows => {
            let hosts_path = os.get_hosts_file_path();
            let default_hosts = "# Copyright (c) 1993-2009 Microsoft Corp.\r\n#\r\n# This is a sample HOSTS file used by Microsoft TCP/IP for Windows.\r\n#\r\n127.0.0.1       localhost\r\n::1             localhost\r\n";
            
            match std::fs::write(&hosts_path, default_hosts) {
                Ok(_) => NetworkCleanupResult {
                    operation: "Hosts File Reset".to_string(),
                    success: true,
                    error: None,
                },
                Err(e) => NetworkCleanupResult {
                    operation: "Hosts File Reset".to_string(),
                    success: false,
                    error: Some(format!("Failed to reset hosts file: {}", e)),
                },
            }
        }
    }
}

pub async fn cleanup_network(os: super::os::OS) -> Vec<NetworkCleanupResult> {
    let mut results = Vec::new();
    
    let dns_result = flush_dns(os.clone()).await;
    results.push(dns_result);
    
    let hosts_result = reset_hosts_file(os).await;
    results.push(hosts_result);
    
    results
}
