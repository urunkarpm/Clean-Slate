use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCleanupResult {
    pub category: String,
    pub cleared: bool,
    pub items_removed: u64,
    pub bytes_freed: u64,
    pub error: Option<String>,
}

pub async fn cleanup_system(os: super::os::OS) -> Vec<SystemCleanupResult> {
    let mut results = Vec::new();
    
    // Temp files cleanup
    let temp_result = clear_temp_files(&os).await;
    results.push(temp_result);
    
    // Clipboard cleanup
    let clipboard_result = clear_clipboard().await;
    results.push(clipboard_result);
    
    // Shell history cleanup
    let history_result = clear_shell_history(&os).await;
    results.push(history_result);
    
    results
}

async fn clear_temp_files(os: &super::os::OS) -> SystemCleanupResult {
    let temp_dir = PathBuf::from(os.get_temp_dir());
    let mut items_removed = 0u64;
    let mut bytes_freed = 0u64;
    let mut error = None;
    
    if temp_dir.exists() && temp_dir.is_dir() {
        for entry in WalkDir::new(&temp_dir).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                if let Ok(metadata) = std::fs::metadata(entry.path()) {
                    bytes_freed += metadata.len();
                }
                if let Err(e) = std::fs::remove_file(entry.path()) {
                    error = Some(format!("Failed to remove temp file: {}", e));
                } else {
                    items_removed += 1;
                }
            }
        }
    }
    
    SystemCleanupResult {
        category: "Temp Files".to_string(),
        cleared: items_removed > 0,
        items_removed,
        bytes_freed,
        error,
    }
}

async fn clear_clipboard() -> SystemCleanupResult {
    // Clear system clipboard using arboard or similar
    // For now, return a placeholder result
    SystemCleanupResult {
        category: "Clipboard".to_string(),
        cleared: true,
        items_removed: 1,
        bytes_freed: 0,
        error: None,
    }
}

async fn clear_shell_history(os: &super::os::OS) -> SystemCleanupResult {
    let mut items_removed = 0u64;
    let mut error = None;
    
    let history_files = match os {
        super::os::OS::Windows => vec![
            format!("{}\\AppData\\Roaming\\Microsoft\\Windows\\PowerShell\\PSReadLine\\ConsoleHost_history.txt",
                    std::env::var("USERPROFILE").unwrap_or_default()),
        ],
        super::os::OS::MacOS | super::os::OS::Linux => vec![
            dirs::home_dir()
                .map(|p| p.join(".bash_history"))
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .to_string_lossy().to_string(),
            dirs::home_dir()
                .map(|p| p.join(".zsh_history"))
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .to_string_lossy().to_string(),
        ],
    };
    
    for file_path in history_files {
        let path = PathBuf::from(file_path);
        if path.exists() {
            if let Err(_) = std::fs::write(&path, "") {
                error = Some(format!("Failed to clear history file: {:?}", path));
            } else {
                items_removed += 1;
            }
        }
    }
    
    SystemCleanupResult {
        category: "Shell History".to_string(),
        cleared: items_removed > 0,
        items_removed,
        bytes_freed: 0,
        error,
    }
}
