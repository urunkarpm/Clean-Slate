use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::engine::exclusions::ExclusionConfig;
use super::DryRunOperation;

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

pub async fn analyze_browser_cache(os: &super::os::OS) -> Vec<DryRunOperation> {
    let mut operations = Vec::new();

    for (name, dirs) in [
        ("Chrome Cache", get_chrome_cache_paths(os)),
        ("Firefox Cache", get_firefox_cache_paths(os)),
        ("Edge Cache", get_edge_cache_paths(os)),
    ] {
        for dir in dirs {
            let (count, bytes) = analyze_directory(&dir);
            operations.push(DryRunOperation {
                name: name.to_string(),
                path: dir.to_string_lossy().to_string(),
                file_count: count,
                bytes,
                would_modify: false,
            });
        }
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

/// Enumerate actual Firefox profile directories instead of using a glob pattern.
fn firefox_profile_dirs(base: PathBuf) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&base) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.ends_with(".default-release")
                    || name_str.ends_with(".default")
                    || name_str.contains("-release")
                {
                    dirs.push(path.join("cache2"));
                }
            }
        }
    }
    dirs
}

fn get_chrome_cache_paths(os: &super::os::OS) -> Vec<PathBuf> {
    match os {
        super::os::OS::Windows => vec![PathBuf::from(format!(
            "{}\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\Cache",
            std::env::var("USERPROFILE").unwrap_or_default()
        ))],
        super::os::OS::MacOS => vec![dirs::home_dir()
            .map(|p| p.join("Library/Caches/Google/Chrome/Default/Cache"))
            .unwrap_or_else(|| PathBuf::from("/tmp"))],
        super::os::OS::Linux => {
            let candidates = [
                ".cache/google-chrome/Default/Cache",
                ".cache/chromium/Default/Cache",
                ".cache/google-chrome-stable/Default/Cache",
            ];
            let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
            candidates.iter().map(|c| home.join(c)).collect()
        }
    }
}

fn get_firefox_cache_paths(os: &super::os::OS) -> Vec<PathBuf> {
    match os {
        super::os::OS::Windows => {
            let base = PathBuf::from(format!(
                "{}\\AppData\\Local\\Mozilla\\Firefox\\Profiles",
                std::env::var("USERPROFILE").unwrap_or_default()
            ));
            firefox_profile_dirs(base)
        }
        super::os::OS::MacOS => {
            let base = dirs::home_dir()
                .map(|p| p.join("Library/Caches/Firefox/Profiles"))
                .unwrap_or_else(|| PathBuf::from("/tmp"));
            firefox_profile_dirs(base)
        }
        super::os::OS::Linux => {
            let base = dirs::home_dir()
                .map(|p| p.join(".cache/mozilla/firefox"))
                .unwrap_or_else(|| PathBuf::from("/tmp"));
            firefox_profile_dirs(base)
        }
    }
}

fn get_edge_cache_paths(os: &super::os::OS) -> Vec<PathBuf> {
    match os {
        super::os::OS::Windows => vec![PathBuf::from(format!(
            "{}\\AppData\\Local\\Microsoft\\Edge\\User Data\\Default\\Cache",
            std::env::var("USERPROFILE").unwrap_or_default()
        ))],
        super::os::OS::MacOS => vec![dirs::home_dir()
            .map(|p| p.join("Library/Caches/Microsoft Edge/Default/Cache"))
            .unwrap_or_else(|| PathBuf::from("/tmp"))],
        super::os::OS::Linux => {
            let candidates = [
                ".cache/microsoft-edge/Default/Cache",
                ".cache/microsoft-edge-stable/Default/Cache",
            ];
            let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
            candidates.iter().map(|c| home.join(c)).collect()
        }
    }
}

pub async fn clear_browser_cache(
    os: super::os::OS,
    exclusions: &ExclusionConfig,
) -> Vec<BrowserCleanupResult> {
    vec![
        clear_chrome_data(&os, exclusions).await,
        clear_firefox_data(&os, exclusions).await,
        clear_edge_data(&os, exclusions).await,
    ]
}

async fn clear_cache_dirs(dirs: Vec<PathBuf>, exclusions: &ExclusionConfig) -> (u64, Option<String>) {
    let mut items_removed = 0u64;
    let mut last_error = None;

    for dir in dirs {
        if !dir.exists() {
            continue;
        }
        for entry in WalkDir::new(&dir).into_iter().filter_map(|e| e.ok()) {
            if exclusions.is_excluded(entry.path()) {
                continue;
            }
            if entry.path().is_file() {
                match std::fs::remove_file(entry.path()) {
                    Ok(_) => items_removed += 1,
                    Err(e) => last_error = Some(format!("Failed to remove {:?}: {}", entry.path(), e)),
                }
            }
        }
    }

    (items_removed, last_error)
}

async fn clear_chrome_data(os: &super::os::OS, exclusions: &ExclusionConfig) -> BrowserCleanupResult {
    let dirs = get_chrome_cache_paths(os);
    let (items_removed, error) = clear_cache_dirs(dirs, exclusions).await;
    BrowserCleanupResult {
        browser: "Chrome".to_string(),
        cache_cleared: items_removed > 0,
        cookies_cleared: false,
        local_storage_cleared: false,
        items_removed,
        error,
    }
}

async fn clear_firefox_data(os: &super::os::OS, exclusions: &ExclusionConfig) -> BrowserCleanupResult {
    let dirs = get_firefox_cache_paths(os);
    let (items_removed, error) = clear_cache_dirs(dirs, exclusions).await;
    BrowserCleanupResult {
        browser: "Firefox".to_string(),
        cache_cleared: items_removed > 0,
        cookies_cleared: false,
        local_storage_cleared: false,
        items_removed,
        error,
    }
}

async fn clear_edge_data(os: &super::os::OS, exclusions: &ExclusionConfig) -> BrowserCleanupResult {
    let dirs = get_edge_cache_paths(os);
    let (items_removed, error) = clear_cache_dirs(dirs, exclusions).await;
    BrowserCleanupResult {
        browser: "Edge".to_string(),
        cache_cleared: items_removed > 0,
        cookies_cleared: false,
        local_storage_cleared: false,
        items_removed,
        error,
    }
}

pub async fn check_browser_updates(os: &super::os::OS) -> Vec<BrowserUpdateInfo> {
    vec![
        check_chrome_update(os).await,
        check_firefox_update(os).await,
        check_edge_update(os).await,
    ]
}

fn run_version_cmd(candidates: &[&str], version_flag: &str) -> Result<String, String> {
    for cmd in candidates {
        let output = std::process::Command::new(cmd)
            .arg(version_flag)
            .output();
        if let Ok(out) = output {
            if out.status.success() {
                let text = String::from_utf8_lossy(&out.stdout).to_string();
                // Extract first version-like token "digits.digits..."
                if let Some(token) = text.split_whitespace().find(|t| {
                    t.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
                }) {
                    return Ok(token.trim_end_matches('.').to_string());
                }
            }
        }
    }
    Err("Not installed".to_string())
}

async fn get_installed_chrome_version(os: &super::os::OS) -> Result<String, String> {
    match os {
        super::os::OS::Windows => {
            let path = PathBuf::from(format!(
                "{}\\AppData\\Local\\Google\\Chrome\\Application\\chrome.exe",
                std::env::var("USERPROFILE").unwrap_or_default()
            ));
            if path.exists() {
                run_version_cmd(&["chrome", "chrome.exe"], "--version")
            } else {
                Err("Chrome not installed".to_string())
            }
        }
        super::os::OS::MacOS => {
            run_version_cmd(&["/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"], "--version")
        }
        super::os::OS::Linux => {
            run_version_cmd(
                &["google-chrome", "google-chrome-stable", "chromium-browser", "chromium"],
                "--version",
            )
        }
    }
}

async fn get_installed_firefox_version(os: &super::os::OS) -> Result<String, String> {
    match os {
        super::os::OS::Windows => {
            run_version_cmd(&["firefox"], "--version")
        }
        super::os::OS::MacOS => {
            run_version_cmd(&["/Applications/Firefox.app/Contents/MacOS/firefox"], "--version")
        }
        super::os::OS::Linux => {
            run_version_cmd(&["firefox", "firefox-esr"], "--version")
        }
    }
}

async fn get_installed_edge_version(os: &super::os::OS) -> Result<String, String> {
    match os {
        super::os::OS::Windows => {
            run_version_cmd(&["msedge", "microsoft-edge"], "--version")
        }
        super::os::OS::MacOS => {
            run_version_cmd(
                &["/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge"],
                "--version",
            )
        }
        super::os::OS::Linux => {
            run_version_cmd(&["microsoft-edge", "microsoft-edge-stable"], "--version")
        }
    }
}

async fn check_chrome_update(os: &super::os::OS) -> BrowserUpdateInfo {
    match get_installed_chrome_version(os).await {
        Ok(version) => BrowserUpdateInfo {
            name: "Chrome".to_string(),
            current_version: version,
            latest_version: None,
            is_up_to_date: true,
            update_available: false,
            error: None,
        },
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
    match get_installed_firefox_version(os).await {
        Ok(version) => BrowserUpdateInfo {
            name: "Firefox".to_string(),
            current_version: version,
            latest_version: None,
            is_up_to_date: true,
            update_available: false,
            error: None,
        },
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
    match get_installed_edge_version(os).await {
        Ok(version) => BrowserUpdateInfo {
            name: "Edge".to_string(),
            current_version: version,
            latest_version: None,
            is_up_to_date: true,
            update_available: false,
            error: None,
        },
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
