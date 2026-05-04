use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OS {
    Windows,
    MacOS,
    Linux,
}

impl OS {
    pub fn detect() -> Self {
        #[cfg(target_os = "windows")]
        return OS::Windows;
        
        #[cfg(target_os = "macos")]
        return OS::MacOS;
        
        #[cfg(target_os = "linux")]
        return OS::Linux;
        
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        return OS::Linux; // Default fallback
    }

    pub fn get_cache_dirs(&self) -> Vec<String> {
        match self {
            OS::Windows => vec![
                "{LOCALAPPDATA}\\Google\\Chrome\\User Data\\Default\\Cache".to_string(),
                "{LOCALAPPDATA}\\Mozilla\\Firefox\\Profiles\\*.default-release\\cache2".to_string(),
                "{LOCALAPPDATA}\\Microsoft\\Edge\\User Data\\Default\\Cache".to_string(),
            ],
            OS::MacOS => vec![
                "~/Library/Caches/Google/Chrome/Default/Cache".to_string(),
                "~/Library/Caches/Firefox/Profiles/*.default-release/cache2".to_string(),
                "~/Library/Caches/Microsoft Edge/Default/Cache".to_string(),
            ],
            OS::Linux => vec![
                "~/.cache/google-chrome/Default/Cache".to_string(),
                "~/.cache/mozilla/firefox/*.default-release/cache2".to_string(),
                "~/.cache/microsoft-edge/Default/Cache".to_string(),
            ],
        }
    }

    pub fn get_temp_dir(&self) -> String {
        match self {
            OS::Windows => std::env::var("TEMP").unwrap_or_else(|_| "C:\\Temp".to_string()),
            // On macOS and Linux, return user-specific temp directory to avoid system-wide /tmp cleanup
            OS::MacOS => dirs::cache_dir()
                .map(|p| p.join("temp"))
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .to_string_lossy()
                .to_string(),
            OS::Linux => dirs::cache_dir()
                .map(|p| p.join("temp"))
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .to_string_lossy()
                .to_string(),
        }
    }

    pub fn get_hosts_file_path(&self) -> String {
        match self {
            OS::Windows => "C:\\Windows\\System32\\drivers\\etc\\hosts".to_string(),
            OS::MacOS => "/etc/hosts".to_string(),
            OS::Linux => "/etc/hosts".to_string(),
        }
    }
}
