use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExclusionConfig {
    pub excluded_paths: HashSet<String>,
    pub ignore_patterns: Vec<String>,
}

impl Default for ExclusionConfig {
    fn default() -> Self {
        ExclusionConfig {
            excluded_paths: HashSet::new(),
            ignore_patterns: vec![
                "*.git".to_string(),
                "node_modules".to_string(),
                ".env".to_string(),
                "*.log".to_string(),
            ],
        }
    }
}

impl ExclusionConfig {
    pub fn is_excluded(&self, path: &std::path::Path) -> bool {
        // Check exact path matches
        let path_str = path.to_string_lossy().to_string();
        if self.excluded_paths.contains(&path_str) {
            return true;
        }
        
        // Check pattern matches
        for pattern in &self.ignore_patterns {
            if self.matches_pattern(&path_str, pattern) {
                return true;
            }
        }
        
        false
    }
    
    fn matches_pattern(&self, path: &str, pattern: &str) -> bool {
        // Simple glob-like matching
        if pattern.starts_with("*.") {
            let ext = &pattern[1..];
            return path.ends_with(ext);
        }
        
        if pattern.ends_with("*") {
            let prefix = &pattern[..pattern.len()-1];
            return path.contains(prefix);
        }
        
        path.contains(pattern)
    }
    
    pub fn add_exclusion(&mut self, path: String) {
        self.excluded_paths.insert(path);
    }
    
    pub fn remove_exclusion(&mut self, path: &str) {
        self.excluded_paths.remove(path);
    }
}
