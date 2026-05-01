// Prevent multiple instances of this module
#![allow(dead_code)]

pub mod logger;
pub mod engine;

use serde::{Deserialize, Serialize};
use crate::engine::{browser, system, network, validator, DryRunOperation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupProgress {
    pub phase: String,
    pub progress: f32,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DryRunResult {
    pub estimated_files_to_remove: u64,
    pub estimated_bytes_to_free: u64,
    pub operations: Vec<DryRunOperation>,
    pub warnings: Vec<String>,
}

#[tauri::command]
async fn run_dry_run() -> Result<DryRunResult, String> {
    use crate::engine::{os, browser, system, network, os_specific};
    use crate::logger::SecureLogger;
    
    let os_type = os::OS::detect();
    let logger = SecureLogger::new().map_err(|e| e.to_string())?;
    
    logger.log("INFO", "Starting dry-run analysis", None)
        .map_err(|e| e.to_string())?;
    
    let mut operations = Vec::new();
    let mut total_files = 0u64;
    let mut total_bytes = 0u64;
    let mut warnings = Vec::new();
    
    // Analyze browser caches (includes downloads, trash, screenshots)
    let browser_ops = browser::analyze_browser_cache(&os_type).await;
    for op in browser_ops {
        total_files += op.file_count;
        total_bytes += op.bytes;
        operations.push(op);
    }
    
    // Analyze system temp files
    let system_ops = system::analyze_system_cleanup(&os_type).await;
    for op in system_ops {
        total_files += op.file_count;
        total_bytes += op.bytes;
        operations.push(op);
    }
    
    // Analyze OS-specific cleanup candidates
    let os_ops = os_specific::analyze_os_specific(&os_type).await;
    for op in os_ops {
        total_files += op.file_count;
        total_bytes += op.bytes;
        operations.push(op);
    }
    
    // Check network operations (these would modify, not just estimate)
    let network_ops = network::analyze_network_cleanup(&os_type).await;
    for op in network_ops {
        if op.would_modify {
            warnings.push(format!("{} requires elevated privileges", op.name));
        }
        operations.push(op);
    }
    
    logger.log("INFO", &format!("Dry-run complete: {} files, {} bytes", total_files, total_bytes), None)
        .map_err(|e| e.to_string())?;
    
    Ok(DryRunResult {
        estimated_files_to_remove: total_files,
        estimated_bytes_to_free: total_bytes,
        operations,
        warnings,
    })
}

#[tauri::command]
async fn start_cleanup(dry_run_confirmed: bool) -> Result<CleanupSummary, String> {
    use crate::engine::{os, browser, system, network, validator, exclusions, os_specific};
    use crate::logger::SecureLogger;
    
    if !dry_run_confirmed {
        return Err("Dry-run must be confirmed before execution".to_string());
    }
    
    let os_type = os::OS::detect();
    let logger = SecureLogger::new().map_err(|e| e.to_string())?;
    let exclusion_config = exclusions::ExclusionConfig::default();
    
    logger.log("INFO", "Starting cleanup process", None)
        .map_err(|e| e.to_string())?;
    
    // Phase 1: Browser cleanup (includes downloads, trash, screenshots)
    logger.log("INFO", "Clearing browser caches", None).map_err(|e| e.to_string())?;
    let browser_results = browser::clear_browser_cache(os_type.clone(), &exclusion_config).await;

    // Phase 2: System cleanup
    logger.log("INFO", "Cleaning system files", None).map_err(|e| e.to_string())?;
    let system_results = system::cleanup_system(os_type.clone(), &exclusion_config).await;

    // Phase 3: OS-specific cleanup
    logger.log("INFO", "Cleaning OS-specific files", None).map_err(|e| e.to_string())?;
    let os_results = os_specific::cleanup_os_specific(os_type.clone(), &exclusion_config).await;

    // Phase 4: Network cleanup
    logger.log("INFO", "Resetting network settings", None).map_err(|e| e.to_string())?;
    let network_results = network::cleanup_network(os_type.clone()).await;

    // Phase 5: Validation
    logger.log("INFO", "Validating cleanup", None).map_err(|e| e.to_string())?;
    let validation_results = validator::validate_cleanup().await;

    logger.finalize().map_err(|e| e.to_string())?;
    
    Ok(CleanupSummary {
        browser_results,
        system_results,
        network_results,
        os_results,
        validation_results,
        log_path: logger.get_log_path().to_string_lossy().to_string(),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupSummary {
    pub browser_results: Vec<browser::BrowserCleanupResult>,
    pub system_results: Vec<system::SystemCleanupResult>,
    pub os_results: Vec<os_specific::OSCleanupResult>,
    pub network_results: Vec<network::NetworkCleanupResult>,
    pub validation_results: Vec<validator::ValidationResult>,
    pub log_path: String,
}

#[tauri::command]
fn get_os_info() -> OSInfo {
    use crate::engine::os::OS;
    
    let os_type = OS::detect();
    OSInfo {
        name: match os_type {
            OS::Windows => "Windows".to_string(),
            OS::MacOS => "macOS".to_string(),
            OS::Linux => "Linux".to_string(),
        },
        temp_dir: os_type.get_temp_dir(),
        hosts_file: os_type.get_hosts_file_path(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OSInfo {
    pub name: String,
    pub temp_dir: String,
    pub hosts_file: String,
}

#[tauri::command]
async fn check_browser_updates() -> Result<Vec<browser::BrowserUpdateInfo>, String> {
    use crate::engine::{os, browser};
    
    let os_type = os::OS::detect();
    let updates = browser::check_browser_updates(&os_type).await;
    
    Ok(updates)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![run_dry_run, start_cleanup, get_os_info, check_browser_updates])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
