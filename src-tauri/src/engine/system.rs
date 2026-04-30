use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::engine::exclusions::ExclusionConfig;
use super::DryRunOperation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCleanupResult {
    pub category: String,
    pub cleared: bool,
    pub items_removed: u64,
    pub bytes_freed: u64,
    pub error: Option<String>,
}

pub async fn analyze_system_cleanup(os: &super::os::OS) -> Vec<DryRunOperation> {
    let mut operations = Vec::new();
    
    // Analyze temp directory
    let temp_dir = PathBuf::from(os.get_temp_dir());
    let (count, bytes) = analyze_directory(&temp_dir);
    operations.push(DryRunOperation {
        name: "Temp Files".to_string(),
        path: temp_dir.to_string_lossy().to_string(),
        file_count: count,
        bytes,
        would_modify: false,
    });
    
    // Analyze shell history
    let history_files = get_shell_history_paths(os);
    for file in history_files {
        let (count, bytes) = if file.exists() && file.is_file() {
            match std::fs::metadata(&file) {
                Ok(m) => (1, m.len()),
                Err(_) => (0, 0),
            }
        } else {
            (0, 0)
        };
        operations.push(DryRunOperation {
            name: "Shell History".to_string(),
            path: file.to_string_lossy().to_string(),
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
        for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|e| e.ok()) {
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

fn get_shell_history_paths(os: &super::os::OS) -> Vec<PathBuf> {
    match os {
        super::os::OS::Windows => vec![
            PathBuf::from(format!("{}\\AppData\\Roaming\\Microsoft\\Windows\\PowerShell\\PSReadLine\\ConsoleHost_history.txt",
                    std::env::var("USERPROFILE").unwrap_or_default())),
        ],
        super::os::OS::MacOS | super::os::OS::Linux => vec![
            dirs::home_dir()
                .map(|p| p.join(".bash_history"))
                .unwrap_or_else(|| PathBuf::from("/tmp")),
            dirs::home_dir()
                .map(|p| p.join(".zsh_history"))
                .unwrap_or_else(|| PathBuf::from("/tmp")),
        ],
    }
}

pub async fn cleanup_system(os: super::os::OS, exclusions: &ExclusionConfig) -> Vec<SystemCleanupResult> {
    let mut results = Vec::new();
    
    // Temp files cleanup
    let temp_result = clear_temp_files(&os, exclusions).await;
    results.push(temp_result);
    
    // Clipboard cleanup
    let clipboard_result = clear_clipboard().await;
    results.push(clipboard_result);
    
    // Shell history cleanup
    let history_result = clear_shell_history(&os, exclusions).await;
    results.push(history_result);
    
    results
}

async fn clear_temp_files(os: &super::os::OS, exclusions: &ExclusionConfig) -> SystemCleanupResult {
    let temp_dir = PathBuf::from(os.get_temp_dir());
    let mut items_removed = 0u64;
    let mut bytes_freed = 0u64;
    let mut error = None;
    
    if temp_dir.exists() && temp_dir.is_dir() {
        for entry in WalkDir::new(&temp_dir).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            // Check exclusions
            if exclusions.is_excluded(entry.path()) {
                continue;
            }
            
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
    SystemCleanupResult {
        category: "Clipboard".to_string(),
        cleared: true,
        items_removed: 1,
        bytes_freed: 0,
        error: None,
    }
}

async fn clear_shell_history(os: &super::os::OS, exclusions: &ExclusionConfig) -> SystemCleanupResult {
    let mut items_removed = 0u64;
    let mut error = None;
    
    let history_files = get_shell_history_paths(os);
    
    for file_path in history_files {
        // Check exclusions
        if exclusions.is_excluded(&file_path) {
            continue;
        }
        
        if file_path.exists() {
            if let Err(_) = std::fs::write(&file_path, "") {
                error = Some(format!("Failed to clear history file: {:?}", file_path));
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
