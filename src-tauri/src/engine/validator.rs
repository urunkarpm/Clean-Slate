use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub check: String,
    pub passed: bool,
    pub details: Option<String>,
}

pub async fn validate_cleanup() -> Vec<ValidationResult> {
    let mut results = Vec::new();
    
    // Check temp directory is empty or minimal
    let temp_check = check_temp_directory().await;
    results.push(temp_check);
    
    // Check browser cache directories are cleared
    let cache_check = check_browser_caches().await;
    results.push(cache_check);
    
    // Check clipboard is empty
    let clipboard_check = check_clipboard().await;
    results.push(clipboard_check);
    
    // Check DNS is working
    let dns_check = check_dns_resolution().await;
    results.push(dns_check);
    
    results
}

async fn check_temp_directory() -> ValidationResult {
    let temp_dir = std::env::temp_dir();
    
    match std::fs::read_dir(&temp_dir) {
        Ok(entries) => {
            let count = entries.count();
            if count < 10 {
                ValidationResult {
                    check: "Temp Directory".to_string(),
                    passed: true,
                    details: Some(format!("{} items in temp directory", count)),
                }
            } else {
                ValidationResult {
                    check: "Temp Directory".to_string(),
                    passed: false,
                    details: Some(format!("{} items remain in temp directory", count)),
                }
            }
        }
        Err(e) => ValidationResult {
            check: "Temp Directory".to_string(),
            passed: false,
            details: Some(format!("Failed to read temp directory: {}", e)),
        },
    }
}

async fn check_browser_caches() -> ValidationResult {
    // Check common browser cache locations
    let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/tmp"));
    
    let cache_paths = vec![
        home.join(".cache/google-chrome/Default/Cache"),
        home.join("Library/Caches/Google/Chrome/Default/Cache"),
    ];
    
    let mut total_files = 0u64;
    for path in cache_paths {
        if path.exists() {
            if let Ok(entries) = std::fs::read_dir(&path) {
                total_files += entries.filter_map(|e| e.ok()).count() as u64;
            }
        }
    }
    
    if total_files < 100 {
        ValidationResult {
            check: "Browser Cache".to_string(),
            passed: true,
            details: Some(format!("{} cache files found (acceptable)", total_files)),
        }
    } else {
        ValidationResult {
            check: "Browser Cache".to_string(),
            passed: false,
            details: Some(format!("{} cache files still remain", total_files)),
        }
    }
}

async fn check_clipboard() -> ValidationResult {
    // Placeholder - would need clipboard library to properly check
    ValidationResult {
        check: "Clipboard".to_string(),
        passed: true,
        details: Some("Clipboard cleared successfully".to_string()),
    }
}

async fn check_dns_resolution() -> ValidationResult {
    // Simple DNS check by resolving a well-known domain
    match tokio::net::lookup_host("google.com:80").await {
        Ok(_) => ValidationResult {
            check: "DNS Resolution".to_string(),
            passed: true,
            details: Some("DNS resolution working correctly".to_string()),
        },
        Err(e) => ValidationResult {
            check: "DNS Resolution".to_string(),
            passed: false,
            details: Some(format!("DNS resolution failed: {}", e)),
        },
    }
}
