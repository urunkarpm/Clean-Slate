use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::engine::exclusions::ExclusionConfig;
use super::DryRunOperation;
use super::os::OS;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSCleanupResult {
    pub category: String,
    pub cleared: bool,
    pub items_removed: u64,
    pub bytes_freed: u64,
    pub error: Option<String>,
}

/// Analyze OS-specific cleanup candidates
pub async fn analyze_os_specific(os: &OS) -> Vec<DryRunOperation> {
    let mut operations = Vec::new();

    match os {
        OS::Windows => {
            // Windows Temp files
            let windows_temp = PathBuf::from("C:\\Windows\\Temp");
            let (count, bytes) = analyze_directory(&windows_temp);
            if count > 0 || bytes > 0 {
                operations.push(DryRunOperation {
                    name: "Windows Temp Files".to_string(),
                    path: windows_temp.to_string_lossy().to_string(),
                    file_count: count,
                    bytes,
                    would_modify: false,
                });
            }

            // Prefetch files
            let prefetch = PathBuf::from("C:\\Windows\\Prefetch");
            let (count, bytes) = analyze_directory(&prefetch);
            if count > 0 || bytes > 0 {
                operations.push(DryRunOperation {
                    name: "Prefetch Files".to_string(),
                    path: prefetch.to_string_lossy().to_string(),
                    file_count: count,
                    bytes,
                    would_modify: false,
                });
            }

            // Windows Update cache
            let update_cache = PathBuf::from("C:\\Windows\\SoftwareDistribution\\Download");
            let (count, bytes) = analyze_directory(&update_cache);
            if count > 0 || bytes > 0 {
                operations.push(DryRunOperation {
                    name: "Windows Update Cache".to_string(),
                    path: update_cache.to_string_lossy().to_string(),
                    file_count: count,
                    bytes,
                    would_modify: false,
                });
            }

            // Thumbnail cache
            if let Some(userprofile) = std::env::var("USERPROFILE").ok() {
                let thumbcache_dir = PathBuf::from(format!(
                    "{}\\AppData\\Local\\Microsoft\\Windows\\Explorer",
                    userprofile
                ));
                let (count, bytes) = analyze_directory(&thumbcache_dir);
                if count > 0 || bytes > 0 {
                    operations.push(DryRunOperation {
                        name: "Thumbnail Cache".to_string(),
                        path: thumbcache_dir.to_string_lossy().to_string(),
                        file_count: count,
                        bytes,
                        would_modify: false,
                    });
                }
            }

            // Crash dumps
            let minidump = PathBuf::from("C:\\Windows\\Minidump");
            let (count, bytes) = analyze_directory(&minidump);
            if count > 0 || bytes > 0 {
                operations.push(DryRunOperation {
                    name: "Crash Dumps".to_string(),
                    path: minidump.to_string_lossy().to_string(),
                    file_count: count,
                    bytes,
                    would_modify: false,
                });
            }

            // Old Windows installations
            let windows_old = PathBuf::from("C:\\Windows.old");
            if windows_old.exists() {
                let (count, bytes) = analyze_directory_recursive(&windows_old);
                if count > 0 || bytes > 0 {
                    operations.push(DryRunOperation {
                        name: "Old Windows Installation".to_string(),
                        path: windows_old.to_string_lossy().to_string(),
                        file_count: count,
                        bytes,
                        would_modify: false,
                    });
                }
            }

            // Windows Error Reporting
            let wer = PathBuf::from("C:\\ProgramData\\Microsoft\\Windows\\WER");
            let (count, bytes) = analyze_directory(&wer);
            if count > 0 || bytes > 0 {
                operations.push(DryRunOperation {
                    name: "Windows Error Reporting".to_string(),
                    path: wer.to_string_lossy().to_string(),
                    file_count: count,
                    bytes,
                    would_modify: false,
                });
            }
        }

        OS::MacOS => {
            if let Some(home) = dirs::home_dir() {
                // User cache
                let user_cache = home.join("Library/Caches");
                let (count, bytes) = analyze_directory(&user_cache);
                if count > 0 || bytes > 0 {
                    operations.push(DryRunOperation {
                        name: "User Cache".to_string(),
                        path: user_cache.to_string_lossy().to_string(),
                        file_count: count,
                        bytes,
                        would_modify: false,
                    });
                }

                // System cache (requires admin)
                let system_cache = PathBuf::from("/Library/Caches");
                let (count, bytes) = analyze_directory(&system_cache);
                if count > 0 || bytes > 0 {
                    operations.push(DryRunOperation {
                        name: "System Cache".to_string(),
                        path: system_cache.to_string_lossy().to_string(),
                        file_count: count,
                        bytes,
                        would_modify: false,
                    });
                }

                // Application logs
                let user_logs = home.join("Library/Logs");
                let (count, bytes) = analyze_directory(&user_logs);
                if count > 0 || bytes > 0 {
                    operations.push(DryRunOperation {
                        name: "Application Logs".to_string(),
                        path: user_logs.to_string_lossy().to_string(),
                        file_count: count,
                        bytes,
                        would_modify: false,
                    });
                }

                // Old iOS backups
                let ios_backups = home.join("Library/Application Support/MobileSync/Backup");
                let (count, bytes) = analyze_directory(&ios_backups);
                if count > 0 || bytes > 0 {
                    operations.push(DryRunOperation {
                        name: "iOS Backups".to_string(),
                        path: ios_backups.to_string_lossy().to_string(),
                        file_count: count,
                        bytes,
                        would_modify: false,
                    });
                }

                // Xcode DerivedData
                let xcode_derived = home.join("Library/Developer/Xcode/DerivedData");
                let (count, bytes) = analyze_directory(&xcode_derived);
                if count > 0 || bytes > 0 {
                    operations.push(DryRunOperation {
                        name: "Xcode DerivedData".to_string(),
                        path: xcode_derived.to_string_lossy().to_string(),
                        file_count: count,
                        bytes,
                        would_modify: false,
                    });
                }

                // Adobe cache
                let adobe_cache = home.join("Library/Caches/Adobe");
                let (count, bytes) = analyze_directory(&adobe_cache);
                if count > 0 || bytes > 0 {
                    operations.push(DryRunOperation {
                        name: "Adobe Cache".to_string(),
                        path: adobe_cache.to_string_lossy().to_string(),
                        file_count: count,
                        bytes,
                        would_modify: false,
                    });
                }

                // Homebrew cache
                let brew_cache = PathBuf::from("/opt/homebrew/Library/Taps/homebrew/homebrew-core/.git");
                if !brew_cache.exists() {
                    // Try Intel location
                    let brew_cache = PathBuf::from("/usr/local/Homebrew/Library/Taps/homebrew/homebrew-core/.git");
                    let (count, bytes) = analyze_directory(&brew_cache.parent().unwrap_or(&brew_cache));
                    if count > 0 || bytes > 0 {
                        operations.push(DryRunOperation {
                            name: "Homebrew Cache".to_string(),
                            path: brew_cache.parent().unwrap_or(&brew_cache).to_string_lossy().to_string(),
                            file_count: count,
                            bytes,
                            would_modify: false,
                        });
                    }
                }
            }

            // Sleep image
            let sleepimage = PathBuf::from("/private/var/vm/sleepimage");
            if sleepimage.exists() {
                if let Ok(metadata) = std::fs::metadata(&sleepimage) {
                    operations.push(DryRunOperation {
                        name: "Sleep Image".to_string(),
                        path: sleepimage.to_string_lossy().to_string(),
                        file_count: 1,
                        bytes: metadata.len(),
                        would_modify: false,
                    });
                }
            }
        }

        OS::Linux => {
            // Package manager cache (APT)
            let apt_cache = PathBuf::from("/var/cache/apt/archives");
            let (count, bytes) = analyze_directory(&apt_cache);
            if count > 0 || bytes > 0 {
                operations.push(DryRunOperation {
                    name: "APT Package Cache".to_string(),
                    path: apt_cache.to_string_lossy().to_string(),
                    file_count: count,
                    bytes,
                    would_modify: false,
                });
            }

            // Systemd journal logs
            let journal_size = get_journal_size();
            if journal_size > 0 {
                operations.push(DryRunOperation {
                    name: "Systemd Journal Logs".to_string(),
                    path: "/var/log/journal".to_string(),
                    file_count: 1,
                    bytes: journal_size,
                    would_modify: false,
                });
            }

            // User cache
            if let Some(home) = dirs::home_dir() {
                let user_cache = home.join(".cache");
                let (count, bytes) = analyze_directory(&user_cache);
                if count > 0 || bytes > 0 {
                    operations.push(DryRunOperation {
                        name: "User Cache (~/.cache)".to_string(),
                        path: user_cache.to_string_lossy().to_string(),
                        file_count: count,
                        bytes,
                        would_modify: false,
                    });
                }

                // NPM cache
                let npm_cache = home.join(".npm");
                let (count, bytes) = analyze_directory(&npm_cache);
                if count > 0 || bytes > 0 {
                    operations.push(DryRunOperation {
                        name: "NPM Cache".to_string(),
                        path: npm_cache.to_string_lossy().to_string(),
                        file_count: count,
                        bytes,
                        would_modify: false,
                    });
                }

                // Cargo cache
                let cargo_cache = home.join(".cargo");
                let (count, bytes) = analyze_directory(&cargo_cache);
                if count > 0 || bytes > 0 {
                    operations.push(DryRunOperation {
                        name: "Cargo Cache".to_string(),
                        path: cargo_cache.to_string_lossy().to_string(),
                        file_count: count,
                        bytes,
                        would_modify: false,
                    });
                }

                // Gradle cache
                let gradle_cache = home.join(".gradle/caches");
                let (count, bytes) = analyze_directory(&gradle_cache);
                if count > 0 || bytes > 0 {
                    operations.push(DryRunOperation {
                        name: "Gradle Cache".to_string(),
                        path: gradle_cache.to_string_lossy().to_string(),
                        file_count: count,
                        bytes,
                        would_modify: false,
                    });
                }
            }

            // Core dumps
            let coredumps = PathBuf::from("/var/lib/systemd/coredump");
            let (count, bytes) = analyze_directory(&coredumps);
            if count > 0 || bytes > 0 {
                operations.push(DryRunOperation {
                    name: "Core Dumps".to_string(),
                    path: coredumps.to_string_lossy().to_string(),
                    file_count: count,
                    bytes,
                    would_modify: false,
                });
            }

            // Log files
            let var_log = PathBuf::from("/var/log");
            let (count, bytes) = analyze_log_files(&var_log);
            if count > 0 || bytes > 0 {
                operations.push(DryRunOperation {
                    name: "Rotated Log Files".to_string(),
                    path: var_log.to_string_lossy().to_string(),
                    file_count: count,
                    bytes,
                    would_modify: false,
                });
            }

            // Flatpak unused
            let flatpak_unused = check_flatpak_unused();
            if flatpak_unused > 0 {
                operations.push(DryRunOperation {
                    name: "Flatpak Unused Runtimes".to_string(),
                    path: "/var/lib/flatpak".to_string(),
                    file_count: 1,
                    bytes: flatpak_unused,
                    would_modify: false,
                });
            }
        }
    }

    operations
}

fn analyze_directory(path: &std::path::Path) -> (u64, u64) {
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

fn analyze_directory_recursive(path: &std::path::Path) -> (u64, u64) {
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

fn analyze_log_files(path: &PathBuf) -> (u64, u64) {
    let mut count = 0u64;
    let mut bytes = 0u64;

    if path.exists() && path.is_dir() {
        for entry in WalkDir::new(path).max_depth(2).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                let file_name = entry.path().file_name().and_then(|n| n.to_str()).unwrap_or("");
                // Only count rotated logs (*.old, *.gz, *.1, etc.)
                if file_name.ends_with(".old")
                    || file_name.ends_with(".gz")
                    || file_name.chars().last().map(|c| c.is_ascii_digit()).unwrap_or(false)
                {
                    count += 1;
                    if let Ok(metadata) = std::fs::metadata(entry.path()) {
                        bytes += metadata.len();
                    }
                }
            }
        }
    }

    (count, bytes)
}

fn get_journal_size() -> u64 {
    let output = std::process::Command::new("journalctl")
        .args(["--disk-usage"])
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            // Parse "Archived and active journals take up X.XG in the file system."
            for token in text.split_whitespace() {
                if let Some(size) = parse_size(token) {
                    return size;
                }
            }
        }
    }
    0
}

fn parse_size(s: &str) -> Option<u64> {
    let s = s.trim();
    if s.ends_with('G') || s.ends_with('g') {
        s[..s.len()-1].parse::<f64>().ok().map(|v| (v * 1024.0 * 1024.0 * 1024.0) as u64)
    } else if s.ends_with('M') || s.ends_with('m') {
        s[..s.len()-1].parse::<f64>().ok().map(|v| (v * 1024.0 * 1024.0) as u64)
    } else if s.ends_with('K') || s.ends_with('k') {
        s[..s.len()-1].parse::<f64>().ok().map(|v| (v * 1024.0) as u64)
    } else {
        s.parse::<u64>().ok()
    }
}

fn check_flatpak_unused() -> u64 {
    let output = std::process::Command::new("flatpak")
        .args(["uninstall", "--unused", "--dry-run"])
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            // Rough estimate based on output
            return text.len() as u64 * 100; // Very rough estimate
        }
    }
    0
}

/// Perform OS-specific cleanup
pub async fn cleanup_os_specific(os: OS, exclusions: &ExclusionConfig) -> Vec<OSCleanupResult> {
    let mut results = Vec::new();

    match os {
        OS::Windows => {
            // Windows Temp
            let temp_result = clear_windows_temp(&exclusions).await;
            results.push(temp_result);

            // Prefetch
            let prefetch_result = clear_prefetch(&exclusions).await;
            results.push(prefetch_result);

            // Windows Update cache
            let update_result = clear_windows_update_cache(&exclusions).await;
            results.push(update_result);

            // Thumbnail cache
            let thumb_result = clear_thumbnail_cache(&exclusions).await;
            results.push(thumb_result);

            // Crash dumps
            let dump_result = clear_crash_dumps(&exclusions).await;
            results.push(dump_result);

            // Windows Error Reporting
            let wer_result = clear_wer(&exclusions).await;
            results.push(wer_result);
        }

        OS::MacOS => {
            // User cache
            let user_cache_result = clear_macos_user_cache(&exclusions).await;
            results.push(user_cache_result);

            // Application logs
            let logs_result = clear_macos_logs(&exclusions).await;
            results.push(logs_result);

            // Xcode DerivedData
            let xcode_result = clear_xcode_derived_data(&exclusions).await;
            results.push(xcode_result);

            // Adobe cache
            let adobe_result = clear_adobe_cache(&exclusions).await;
            results.push(adobe_result);
        }

        OS::Linux => {
            // User cache
            let user_cache_result = clear_linux_user_cache(&exclusions).await;
            results.push(user_cache_result);

            // NPM cache
            let npm_result = clear_npm_cache(&exclusions).await;
            results.push(npm_result);

            // Cargo cache
            let cargo_result = clear_cargo_cache(&exclusions).await;
            results.push(cargo_result);
        }
    }

    results
}

async fn clear_windows_temp(exclusions: &ExclusionConfig) -> OSCleanupResult {
    let temp_paths = vec![
        PathBuf::from("C:\\Windows\\Temp"),
        std::env::temp_dir(),
    ];

    let mut items_removed = 0u64;
    let mut bytes_freed = 0u64;
    let mut error = None;

    for dir in temp_paths {
        if !dir.exists() || !dir.is_dir() {
            continue;
        }

        for entry in WalkDir::new(&dir).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            if exclusions.is_excluded(entry.path()) {
                continue;
            }

            if entry.path().is_file() {
                if let Ok(metadata) = std::fs::metadata(entry.path()) {
                    bytes_freed += metadata.len();
                }
                if let Err(e) = std::fs::remove_file(entry.path()) {
                    error = Some(format!("Failed to remove file: {}", e));
                } else {
                    items_removed += 1;
                }
            }
        }
    }

    OSCleanupResult {
        category: "Windows Temp".to_string(),
        cleared: items_removed > 0,
        items_removed,
        bytes_freed,
        error,
    }
}

async fn clear_prefetch(exclusions: &ExclusionConfig) -> OSCleanupResult {
    let prefetch = PathBuf::from("C:\\Windows\\Prefetch");
    clear_directory(&prefetch, exclusions, "Prefetch").await
}

async fn clear_windows_update_cache(exclusions: &ExclusionConfig) -> OSCleanupResult {
    let update_cache = PathBuf::from("C:\\Windows\\SoftwareDistribution\\Download");
    clear_directory(&update_cache, exclusions, "Windows Update Cache").await
}

async fn clear_thumbnail_cache(exclusions: &ExclusionConfig) -> OSCleanupResult {
    if let Some(userprofile) = std::env::var("USERPROFILE").ok() {
        let thumbcache = PathBuf::from(format!(
            "{}\\AppData\\Local\\Microsoft\\Windows\\Explorer",
            userprofile
        ));
        return clear_directory(&thumbcache, exclusions, "Thumbnail Cache").await;
    }

    OSCleanupResult {
        category: "Thumbnail Cache".to_string(),
        cleared: false,
        items_removed: 0,
        bytes_freed: 0,
        error: Some("Could not determine user profile".to_string()),
    }
}

async fn clear_crash_dumps(exclusions: &ExclusionConfig) -> OSCleanupResult {
    let minidump = PathBuf::from("C:\\Windows\\Minidump");
    clear_directory(&minidump, exclusions, "Crash Dumps").await
}

async fn clear_wer(exclusions: &ExclusionConfig) -> OSCleanupResult {
    let wer = PathBuf::from("C:\\ProgramData\\Microsoft\\Windows\\WER");
    clear_directory(&wer, exclusions, "Windows Error Reporting").await
}

async fn clear_macos_user_cache(exclusions: &ExclusionConfig) -> OSCleanupResult {
    if let Some(home) = dirs::home_dir() {
        let cache_dir = home.join("Library/Caches");
        return clear_directory_contents(&cache_dir, exclusions, "macOS User Cache").await;
    }

    OSCleanupResult {
        category: "macOS User Cache".to_string(),
        cleared: false,
        items_removed: 0,
        bytes_freed: 0,
        error: Some("Could not determine home directory".to_string()),
    }
}

async fn clear_macos_logs(exclusions: &ExclusionConfig) -> OSCleanupResult {
    if let Some(home) = dirs::home_dir() {
        let logs_dir = home.join("Library/Logs");
        return clear_old_logs(&logs_dir, exclusions).await;
    }

    OSCleanupResult {
        category: "macOS Logs".to_string(),
        cleared: false,
        items_removed: 0,
        bytes_freed: 0,
        error: Some("Could not determine home directory".to_string()),
    }
}

async fn clear_xcode_derived_data(exclusions: &ExclusionConfig) -> OSCleanupResult {
    if let Some(home) = dirs::home_dir() {
        let derived_data = home.join("Library/Developer/Xcode/DerivedData");
        return clear_directory(&derived_data, exclusions, "Xcode DerivedData").await;
    }

    OSCleanupResult {
        category: "Xcode DerivedData".to_string(),
        cleared: false,
        items_removed: 0,
        bytes_freed: 0,
        error: Some("Could not determine home directory".to_string()),
    }
}

async fn clear_adobe_cache(exclusions: &ExclusionConfig) -> OSCleanupResult {
    if let Some(home) = dirs::home_dir() {
        let adobe_cache = home.join("Library/Caches/Adobe");
        return clear_directory(&adobe_cache, exclusions, "Adobe Cache").await;
    }

    OSCleanupResult {
        category: "Adobe Cache".to_string(),
        cleared: false,
        items_removed: 0,
        bytes_freed: 0,
        error: Some("Could not determine home directory".to_string()),
    }
}

async fn clear_linux_user_cache(exclusions: &ExclusionConfig) -> OSCleanupResult {
    if let Some(home) = dirs::home_dir() {
        let cache_dir = home.join(".cache");
        return clear_directory_contents(&cache_dir, exclusions, "Linux User Cache").await;
    }

    OSCleanupResult {
        category: "Linux User Cache".to_string(),
        cleared: false,
        items_removed: 0,
        bytes_freed: 0,
        error: Some("Could not determine home directory".to_string()),
    }
}

async fn clear_npm_cache(exclusions: &ExclusionConfig) -> OSCleanupResult {
    if let Some(home) = dirs::home_dir() {
        let npm_cache = home.join(".npm");
        return clear_directory(&npm_cache, exclusions, "NPM Cache").await;
    }

    OSCleanupResult {
        category: "NPM Cache".to_string(),
        cleared: false,
        items_removed: 0,
        bytes_freed: 0,
        error: Some("Could not determine home directory".to_string()),
    }
}

async fn clear_cargo_cache(exclusions: &ExclusionConfig) -> OSCleanupResult {
    if let Some(home) = dirs::home_dir() {
        let cargo_cache = home.join(".cargo");
        return clear_directory(&cargo_cache, exclusions, "Cargo Cache").await;
    }

    OSCleanupResult {
        category: "Cargo Cache".to_string(),
        cleared: false,
        items_removed: 0,
        bytes_freed: 0,
        error: Some("Could not determine home directory".to_string()),
    }
}

async fn clear_directory(path: &PathBuf, exclusions: &ExclusionConfig, category: &str) -> OSCleanupResult {
    let mut items_removed = 0u64;
    let mut bytes_freed = 0u64;
    let mut error = None;

    if !path.exists() || !path.is_dir() {
        return OSCleanupResult {
            category: category.to_string(),
            cleared: false,
            items_removed: 0,
            bytes_freed: 0,
            error: Some("Directory does not exist".to_string()),
        };
    }

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if exclusions.is_excluded(entry.path()) {
            continue;
        }

        if entry.path().is_file() {
            if let Ok(metadata) = std::fs::metadata(entry.path()) {
                bytes_freed += metadata.len();
            }
            if let Err(e) = std::fs::remove_file(entry.path()) {
                error = Some(format!("Failed to remove file: {}", e));
            } else {
                items_removed += 1;
            }
        }
    }

    OSCleanupResult {
        category: category.to_string(),
        cleared: items_removed > 0,
        items_removed,
        bytes_freed,
        error,
    }
}

async fn clear_directory_contents(path: &PathBuf, exclusions: &ExclusionConfig, category: &str) -> OSCleanupResult {
    let mut items_removed = 0u64;
    let mut bytes_freed = 0u64;
    let mut error = None;

    if !path.exists() || !path.is_dir() {
        return OSCleanupResult {
            category: category.to_string(),
            cleared: false,
            items_removed: 0,
            bytes_freed: 0,
            error: Some("Directory does not exist".to_string()),
        };
    }

    // Clear contents but not the directory itself
    for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        if entry.path() == path {
            continue;
        }

        if exclusions.is_excluded(entry.path()) {
            continue;
        }

        if entry.path().is_file() {
            if let Ok(metadata) = std::fs::metadata(entry.path()) {
                bytes_freed += metadata.len();
            }
            if let Err(e) = std::fs::remove_file(entry.path()) {
                error = Some(format!("Failed to remove file: {}", e));
            } else {
                items_removed += 1;
            }
        } else if entry.path().is_dir() {
            if let Err(e) = std::fs::remove_dir_all(entry.path()) {
                error = Some(format!("Failed to remove directory: {}", e));
            } else {
                items_removed += 1;
            }
        }
    }

    OSCleanupResult {
        category: category.to_string(),
        cleared: items_removed > 0,
        items_removed,
        bytes_freed,
        error,
    }
}

async fn clear_old_logs(path: &PathBuf, exclusions: &ExclusionConfig) -> OSCleanupResult {
    let mut items_removed = 0u64;
    let mut bytes_freed = 0u64;
    let mut error = None;

    if !path.exists() || !path.is_dir() {
        return OSCleanupResult {
            category: "Old Logs".to_string(),
            cleared: false,
            items_removed: 0,
            bytes_freed: 0,
            error: Some("Directory does not exist".to_string()),
        };
    }

    for entry in WalkDir::new(path).max_depth(2).into_iter().filter_map(|e| e.ok()) {
        if exclusions.is_excluded(entry.path()) {
            continue;
        }

        if entry.path().is_file() {
            let file_name = entry.path().file_name().and_then(|n| n.to_str()).unwrap_or("");
            // Only remove rotated logs
            if file_name.ends_with(".old")
                || file_name.ends_with(".gz")
                || file_name.ends_with(".log")
            {
                if let Ok(metadata) = std::fs::metadata(entry.path()) {
                    bytes_freed += metadata.len();
                }
                if let Err(e) = std::fs::remove_file(entry.path()) {
                    error = Some(format!("Failed to remove log file: {}", e));
                } else {
                    items_removed += 1;
                }
            }
        }
    }

    OSCleanupResult {
        category: "Old Logs".to_string(),
        cleared: items_removed > 0,
        items_removed,
        bytes_freed,
        error,
    }
}
