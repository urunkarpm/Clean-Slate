// Prevent multiple instances of this module
#![allow(dead_code)]

pub mod logger;
pub mod engine;

use tauri::Manager;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupProgress {
    pub phase: String,
    pub progress: f32,
    pub message: String,
    pub details: Option<String>,
}

#[tauri::command]
async fn start_cleanup() -> Result<CleanupSummary, String> {
    use crate::engine::{os, browser, system, network, validator};
    use crate::logger::SecureLogger;
    
    let os_type = os::OS::detect();
    let logger = SecureLogger::new().map_err(|e| e.to_string())?;
    
    logger.log("INFO", "Starting cleanup process", None)
        .map_err(|e| e.to_string())?;
    
    // Phase 1: Browser cleanup
    logger.log("INFO", "Clearing browser caches", None)?;
    let browser_results = browser::clear_browser_cache(os_type.clone()).await;
    
    // Phase 2: System cleanup
    logger.log("INFO", "Cleaning system files", None)?;
    let system_results = system::cleanup_system(os_type.clone()).await;
    
    // Phase 3: Network cleanup
    logger.log("INFO", "Resetting network settings", None)?;
    let network_results = network::cleanup_network(os_type.clone()).await;
    
    // Phase 4: Validation
    logger.log("INFO", "Validating cleanup", None)?;
    let validation_results = validator::validate_cleanup().await;
    
    logger.finalize()?;
    
    Ok(CleanupSummary {
        browser_results,
        system_results,
        network_results,
        validation_results,
        log_path: logger.get_log_path().to_string_lossy().to_string(),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupSummary {
    pub browser_results: Vec<browser::BrowserCleanupResult>,
    pub system_results: Vec<system::SystemCleanupResult>,
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![start_cleanup, get_os_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
