use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::engine::exclusions::ExclusionConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserCleanupResult {
    pub browser: String,
    pub cache_cleared: bool,
    pub cookies_cleared: bool,
    pub local_storage_cleared: bool,
    pub items_removed: u64,
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

pub async fn analyze_browser_cache(os: &super::os::OS) -> Vec<DryRunOperation> {
    let mut operations = Vec::new();
    
    // Analyze Chrome cache
    let chrome_dirs = get_chrome_cache_paths(os);
    for dir in chrome_dirs {
        let (count, bytes) = analyze_directory(&dir);
        operations.push(DryRunOperation {
            name: "Chrome Cache".to_string(),
            path: dir.to_string_lossy().to_string(),
            file_count: count,
            bytes,
            would_modify: false,
        });
    }
    
    // Analyze Firefox cache
    let firefox_dirs = get_firefox_cache_paths(os);
    for dir in firefox_dirs {
        let (count, bytes) = analyze_directory(&dir);
        operations.push(DryRunOperation {
            name: "Firefox Cache".to_string(),
            path: dir.to_string_lossy().to_string(),
            file_count: count,
            bytes,
            would_modify: false,
        });
    }
    
    // Analyze Edge cache
    let edge_dirs = get_edge_cache_paths(os);
    for dir in edge_dirs {
        let (count, bytes) = analyze_directory(&dir);
        operations.push(DryRunOperation {
            name: "Edge Cache".to_string(),
            path: dir.to_string_lossy().to_string(),
            file_count: count,
            bytes,
            would_modify: false,
        });
    }
    
    operations
}

fn analyze_directory(path: &PathBuf) -> (u64, u64) {
    let mut count = 0u64;
    let mut bytes = 0u64;
    
    if path.exists() && path.is_dir() {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                count += 1;
                if let Ok(metadata) = std::fs::metadata(entry.path()) {
                    bytes += metadata.len();
                }
            }
        }
    }
    
    (count, bytes)
}

fn get_chrome_cache_paths(os: &super::os::OS) -> Vec<PathBuf> {
    match os {
        super::os::OS::Windows => vec![
            PathBuf::from(format!("{}\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\Cache", 
                    std::env::var("USERPROFILE").unwrap_or_default())),
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
    }
}

fn get_firefox_cache_paths(os: &super::os::OS) -> Vec<PathBuf> {
    match os {
        super::os::OS::Windows => vec![
            PathBuf::from(format!("{}\\AppData\\Local\\Mozilla\\Firefox\\Profiles\\*.default-release\\cache2", 
                    std::env::var("USERPROFILE").unwrap_or_default())),
        ],
        super::os::OS::MacOS => vec![
            dirs::home_dir()
                .map(|p| p.join("Library/Caches/Firefox/Profiles/*.default-release/cache2"))
                .unwrap_or_else(|| PathBuf::from("/tmp")),
        ],
        super::os::OS::Linux => vec![
            dirs::home_dir()
                .map(|p| p.join(".cache/mozilla/firefox/*.default-release/cache2"))
                .unwrap_or_else(|| PathBuf::from("/tmp")),
        ],
    }
}

fn get_edge_cache_paths(os: &super::os::OS) -> Vec<PathBuf> {
    match os {
        super::os::OS::Windows => vec![
            PathBuf::from(format!("{}\\AppData\\Local\\Microsoft\\Edge\\User Data\\Default\\Cache", 
                    std::env::var("USERPROFILE").unwrap_or_default())),
        ],
        super::os::OS::MacOS => vec![
            dirs::home_dir()
                .map(|p| p.join("Library/Caches/Microsoft Edge/Default/Cache"))
                .unwrap_or_else(|| PathBuf::from("/tmp")),
        ],
        super::os::OS::Linux => vec![
            dirs::home_dir()
                .map(|p| p.join(".cache/microsoft-edge/Default/Cache"))
                .unwrap_or_else(|| PathBuf::from("/tmp")),
        ],
    }
}

pub async fn clear_browser_cache(os: super::os::OS, exclusions: &ExclusionConfig) -> Vec<BrowserCleanupResult> {
    let mut results = Vec::new();
    
    // Chrome cleanup
    let chrome_result = clear_chrome_data(&os, exclusions).await;
    results.push(chrome_result);
    
    // Firefox cleanup
    let firefox_result = clear_firefox_data(&os, exclusions).await;
    results.push(firefox_result);
    
    // Edge cleanup
    let edge_result = clear_edge_data(&os, exclusions).await;
    results.push(edge_result);
    
    results
}

async fn clear_chrome_data(os: &super::os::OS, exclusions: &ExclusionConfig) -> BrowserCleanupResult {
    let mut items_removed = 0u64;
    let mut error = None;
    
    let cache_dirs = get_chrome_cache_paths(os);
    
    for dir in cache_dirs {
        let path = dir;
        
        if path.exists() {
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                // Check exclusions
                if exclusions.is_excluded(entry.path()) {
                    continue;
                }
                
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
        cookies_cleared: false,
        local_storage_cleared: false,
        items_removed,
        error,
    }
}

async fn clear_firefox_data(_os: &super::os::OS, _exclusions: &ExclusionConfig) -> BrowserCleanupResult {
    BrowserCleanupResult {
        browser: "Firefox".to_string(),
        cache_cleared: false,
        cookies_cleared: false,
        local_storage_cleared: false,
        items_removed: 0,
        error: None,
    }
}

async fn clear_edge_data(_os: &super::os::OS, _exclusions: &ExclusionConfig) -> BrowserCleanupResult {
    BrowserCleanupResult {
        browser: "Edge".to_string(),
        cache_cleared: false,
        cookies_cleared: false,
        local_storage_cleared: false,
        items_removed: 0,
        error: None,
    }
}
