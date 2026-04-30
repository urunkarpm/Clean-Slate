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
pub struct BrowserUpdateInfo {
    pub name: String,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub is_up_to_date: bool,
    pub update_available: bool,
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

pub async fn check_browser_updates(os: &super::os::OS) -> Vec<BrowserUpdateInfo> {
    let mut updates = Vec::new();
    
    // Check Chrome updates
    let chrome_info = check_chrome_update(os).await;
    updates.push(chrome_info);
    
    // Check Firefox updates
    let firefox_info = check_firefox_update(os).await;
    updates.push(firefox_info);
    
    // Check Edge updates
    let edge_info = check_edge_update(os).await;
    updates.push(edge_info);
    
    updates
}

async fn check_chrome_update(os: &super::os::OS) -> BrowserUpdateInfo {
    // Simulated version check - in production this would query the browser's update API
    // or check against a version endpoint
    let current_version = get_installed_chrome_version(os).await;
    
    match current_version {
        Ok(version) => {
            // Simulate checking for latest version (in real implementation, this would call an API)
            let latest_version = simulate_get_latest_version("chrome").await;
            
            let is_up_to_date = version == latest_version;
            
            BrowserUpdateInfo {
                name: "Chrome".to_string(),
                current_version: version,
                latest_version: Some(latest_version.clone()),
                is_up_to_date,
                update_available: !is_up_to_date,
                error: None,
            }
        }
        Err(e) => BrowserUpdateInfo {
            name: "Chrome".to_string(),
            current_version: "Unknown".to_string(),
            latest_version: None,
            is_up_to_date: false,
            update_available: false,
            error: Some(e),
        },
    }
}

async fn check_firefox_update(os: &super::os::OS) -> BrowserUpdateInfo {
    let current_version = get_installed_firefox_version(os).await;
    
    match current_version {
        Ok(version) => {
            let latest_version = simulate_get_latest_version("firefox").await;
            let is_up_to_date = version == latest_version;
            
            BrowserUpdateInfo {
                name: "Firefox".to_string(),
                current_version: version,
                latest_version: Some(latest_version.clone()),
                is_up_to_date,
                update_available: !is_up_to_date,
                error: None,
            }
        }
        Err(e) => BrowserUpdateInfo {
            name: "Firefox".to_string(),
            current_version: "Unknown".to_string(),
            latest_version: None,
            is_up_to_date: false,
            update_available: false,
            error: Some(e),
        },
    }
}

async fn check_edge_update(os: &super::os::OS) -> BrowserUpdateInfo {
    let current_version = get_installed_edge_version(os).await;
    
    match current_version {
        Ok(version) => {
            let latest_version = simulate_get_latest_version("edge").await;
            let is_up_to_date = version == latest_version;
            
            BrowserUpdateInfo {
                name: "Edge".to_string(),
                current_version: version,
                latest_version: Some(latest_version.clone()),
                is_up_to_date,
                update_available: !is_up_to_date,
                error: None,
            }
        }
        Err(e) => BrowserUpdateInfo {
            name: "Edge".to_string(),
            current_version: "Unknown".to_string(),
            latest_version: None,
            is_up_to_date: false,
            update_available: false,
            error: Some(e),
        },
    }
}

async fn get_installed_chrome_version(os: &super::os::OS) -> Result<String, String> {
    // In production, this would read from the browser's installation directory or registry
    // For now, return a simulated version
    match os {
        super::os::OS::Windows => {
            let chrome_path = PathBuf::from(format!(
                "{}\\AppData\\Local\\Google\\Chrome\\Application\\chrome.exe",
                std::env::var("USERPROFILE").unwrap_or_default()
            ));
            if chrome_path.exists() {
                Ok("120.0.6099.109".to_string())
            } else {
                Err("Chrome not installed".to_string())
            }
        }
        super::os::OS::MacOS => {
            let chrome_path = dirs::home_dir()
                .map(|p| p.join("Applications/Google Chrome.app"))
                .unwrap_or_else(|| PathBuf::from("/tmp"));
            if chrome_path.exists() {
                Ok("120.0.6099.109".to_string())
            } else {
                Err("Chrome not installed".to_string())
            }
        }
        super::os::OS::Linux => {
            let chrome_path = PathBuf::from("/usr/bin/google-chrome");
            if chrome_path.exists() {
                Ok("120.0.6099.109".to_string())
            } else {
                Err("Chrome not installed".to_string())
            }
        }
    }
}

async fn get_installed_firefox_version(os: &super::os::OS) -> Result<String, String> {
    match os {
        super::os::OS::Windows => {
            let firefox_path = PathBuf::from(format!(
                "{}\\Program Files\\Mozilla Firefox\\firefox.exe",
                std::env::var("PROGRAMFILES").unwrap_or_default()
            ));
            if firefox_path.exists() {
                Ok("121.0".to_string())
            } else {
                Err("Firefox not installed".to_string())
            }
        }
        super::os::OS::MacOS => {
            let firefox_path = dirs::home_dir()
                .map(|p| p.join("Applications/Firefox.app"))
                .unwrap_or_else(|| PathBuf::from("/tmp"));
            if firefox_path.exists() {
                Ok("121.0".to_string())
            } else {
                Err("Firefox not installed".to_string())
            }
        }
        super::os::OS::Linux => {
            let firefox_path = PathBuf::from("/usr/bin/firefox");
            if firefox_path.exists() {
                Ok("121.0".to_string())
            } else {
                Err("Firefox not installed".to_string())
            }
        }
    }
}

async fn get_installed_edge_version(os: &super::os::OS) -> Result<String, String> {
    match os {
        super::os::OS::Windows => {
            let edge_path = PathBuf::from(format!(
                "{}\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe",
                std::env::var("PROGRAMFILES(X86)").unwrap_or_default()
            ));
            if edge_path.exists() {
                Ok("120.0.2210.91".to_string())
            } else {
                Err("Edge not installed".to_string())
            }
        }
        super::os::OS::MacOS => {
            let edge_path = dirs::home_dir()
                .map(|p| p.join("Applications/Microsoft Edge.app"))
                .unwrap_or_else(|| PathBuf::from("/tmp"));
            if edge_path.exists() {
                Ok("120.0.2210.91".to_string())
            } else {
                Err("Edge not installed".to_string())
            }
        }
        super::os::OS::Linux => {
            let edge_path = PathBuf::from("/usr/bin/microsoft-edge");
            if edge_path.exists() {
                Ok("120.0.2210.91".to_string())
            } else {
                Err("Edge not installed".to_string())
            }
        }
    }
}

async fn simulate_get_latest_version(browser: &str) -> String {
    // In production, this would call the browser's official update API
    // For simulation purposes, return a fixed "latest" version
    match browser {
        "chrome" => "120.0.6099.130".to_string(),
        "firefox" => "121.0".to_string(),
        "edge" => "120.0.2210.133".to_string(),
        _ => "0.0.0".to_string(),
    }
}
