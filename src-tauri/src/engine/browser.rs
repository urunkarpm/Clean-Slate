use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserCleanupResult {
    pub browser: String,
    pub cache_cleared: bool,
    pub cookies_cleared: bool,
    pub local_storage_cleared: bool,
    pub items_removed: u64,
    pub error: Option<String>,
}

pub async fn clear_browser_cache(os: super::os::OS) -> Vec<BrowserCleanupResult> {
    let mut results = Vec::new();
    
    // Chrome cleanup
    let chrome_result = clear_chrome_data(&os).await;
    results.push(chrome_result);
    
    // Firefox cleanup
    let firefox_result = clear_firefox_data(&os).await;
    results.push(firefox_result);
    
    // Edge cleanup
    let edge_result = clear_edge_data(&os).await;
    results.push(edge_result);
    
    results
}

async fn clear_chrome_data(os: &super::os::OS) -> BrowserCleanupResult {
    let mut items_removed = 0u64;
    let mut error = None;
    
    let cache_dirs = match os {
        super::os::OS::Windows => vec![
            format!("{}\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\Cache", 
                    std::env::var("USERPROFILE").unwrap_or_default()),
        ],
        super::os::OS::MacOS => vec![
            dirs::home_dir()
                .map(|p| p.join("Library/Caches/Google/Chrome/Default/Cache"))
                .unwrap_or_else(|| PathBuf::from("/tmp")),
        ],
        super::os::OS::Linux => vec![
            dirs::home_dir()
                .map(|p| p.join(".cache/google-chrome/Default/Cache"))
                .unwrap_or_else(|| PathBuf::from("/tmp")),
        ],
    };
    
    for dir in cache_dirs {
        let path = match os {
            super::os::OS::Windows => PathBuf::from(dir),
            _ => {
                let expanded = dir.replace("~", &dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("/tmp"))
                    .to_string_lossy());
                PathBuf::from(expanded)
            }
        };
        
        if path.exists() {
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                if entry.path().is_file() {
                    if let Err(_) = std::fs::remove_file(entry.path()) {
                        error = Some(format!("Failed to remove file: {:?}", entry.path()));
                    } else {
                        items_removed += 1;
                    }
                }
            }
        }
    }
    
    BrowserCleanupResult {
        browser: "Chrome".to_string(),
        cache_cleared: items_removed > 0,
        cookies_cleared: false, // Separate implementation needed
        local_storage_cleared: false, // Separate implementation needed
        items_removed,
        error,
    }
}

async fn clear_firefox_data(_os: &super::os::OS) -> BrowserCleanupResult {
    // Placeholder for Firefox cleanup
    BrowserCleanupResult {
        browser: "Firefox".to_string(),
        cache_cleared: false,
        cookies_cleared: false,
        local_storage_cleared: false,
        items_removed: 0,
        error: None,
    }
}

async fn clear_edge_data(_os: &super::os::OS) -> BrowserCleanupResult {
    // Placeholder for Edge cleanup
    BrowserCleanupResult {
        browser: "Edge".to_string(),
        cache_cleared: false,
        cookies_cleared: false,
        local_storage_cleared: false,
        items_removed: 0,
        error: None,
    }
}
