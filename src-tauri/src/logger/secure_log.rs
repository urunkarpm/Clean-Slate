use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

pub struct SecureLogger {
    log_path: PathBuf,
    session_id: String,
}

impl SecureLogger {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let session_id = Uuid::new_v4().to_string();
        let log_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("CleanSlateQA")
            .join("logs");
        
        std::fs::create_dir_all(&log_dir)?;
        
        let log_path = log_dir.join(format!("cleanup_{}.json", session_id));
        
        Ok(SecureLogger {
            log_path,
            session_id,
        })
    }
    
    pub fn log(&self, level: &str, message: &str, details: Option<serde_json::Value>) -> Result<(), Box<dyn std::error::Error>> {
        let entry = LogEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono_lite_timestamp(),
            level: level.to_string(),
            message: message.to_string(),
            details,
        };
        
        let mut file = if self.log_path.exists() {
            OpenOptions::new()
                .append(true)
                .open(&self.log_path)?
        } else {
            // Create new file with array start
            let mut f = File::create(&self.log_path)?;
            writeln!(f, "[")?;
            f
        };
        
        // Check if file is empty or just has "["
        let metadata = std::fs::metadata(&self.log_path)?;
        if metadata.len() > 1 {
            // Add comma before new entry
            write!(file, ",")?;
        }
        
        let json = serde_json::to_string(&entry)?;
        writeln!(file, "  {}", json)?;
        
        Ok(())
    }
    
    pub fn finalize(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Close the JSON array
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.log_path)?;
        writeln!(file, "]")?;
        
        // Set user-only permissions on Unix-like systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&self.log_path)?.permissions();
            perms.set_mode(0o600); // User read/write only
            std::fs::set_permissions(&self.log_path, perms)?;
        }
        
        Ok(())
    }
    
    pub fn get_log_path(&self) -> &PathBuf {
        &self.log_path
    }
    
    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}

fn chrono_lite_timestamp() -> String {
    // Simple timestamp without external dependency
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let nanos = now.subsec_nanos();
    format!("{}.{:09}", secs, nanos)
}
