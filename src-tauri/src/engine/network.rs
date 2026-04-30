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
    operations.push(DryRunOperation {
        name: "DNS Flush".to_string(),
        path: "system".to_string(),
        file_count: 0,
        bytes: 0,
        would_modify: true,
    });
    
    // Hosts file reset - requires elevated privileges
    operations.push(DryRunOperation {
        name: "Hosts File Reset".to_string(),
        path: os.get_hosts_file_path(),
        file_count: 0,
        bytes: 0,
        would_modify: true,
    });
    
    operations
}

pub async fn flush_dns(os: super::os::OS) -> NetworkCleanupResult {
    let command = match os {
        super::os::OS::Windows => ("ipconfig", &["/flushdns"]),
        super::os::OS::MacOS => ("sudo", &["killall", "-HUP", "mDNSResponder"]),
        super::os::OS::Linux => ("sudo", &["systemd-resolve", "--flush-caches"]),
    };
    
    match std::process::Command::new(command.0).args(command.1).output() {
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

pub async fn reset_hosts_file(os: super::os::OS) -> NetworkCleanupResult {
    let hosts_path = os.get_hosts_file_path();
    let default_hosts = match os {
        super::os::OS::Windows => "# Copyright (c) 1993-2009 Microsoft Corp.\r\n#\r\n# This is a sample HOSTS file used by Microsoft TCP/IP for Windows.\r\n#\r\n127.0.0.1       localhost\r\n::1             localhost\r\n",
        _ => "# Host Database\n#\n# localhost is used to configure the loopback interface\n# when the system is booting.  Do not change this entry.\n##\n127.0.0.1       localhost\n::1             localhost\n",
    };
    
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

pub async fn cleanup_network(os: super::os::OS) -> Vec<NetworkCleanupResult> {
    let mut results = Vec::new();
    
    let dns_result = flush_dns(os.clone()).await;
    results.push(dns_result);
    
    let hosts_result = reset_hosts_file(os).await;
    results.push(hosts_result);
    
    results
}
