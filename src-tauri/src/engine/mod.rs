// Prevent multiple instances of this module
#![allow(dead_code)]

pub mod os;
pub mod browser;
pub mod system;
pub mod network;
pub mod validator;
pub mod exclusions;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DryRunOperation {
    pub name: String,
    pub path: String,
    pub file_count: u64,
    pub bytes: u64,
    pub would_modify: bool,
}
