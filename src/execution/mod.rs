//! Execution Module
//!
//! Process management, watchdog, and service configuration.

pub mod service;
pub mod watchdog;

// Public exports
pub use service::{generate_systemd_service, generate_task_scheduler_xml, ServiceConfig};
pub use watchdog::{Watchdog, WatchdogConfig};

use std::path::PathBuf;

/// Get current executable path
pub fn get_executable_path() -> Result<PathBuf, String> {
    std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))
}

/// Get project root directory
pub fn get_project_root() -> Result<PathBuf, String> {
    let exe_path = get_executable_path()?;
    exe_path
        .parent()
        .ok_or_else(|| "No parent directory".to_string())
        .map(|p| p.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_executable_path() {
        let path = get_executable_path().unwrap();
        assert!(path.is_absolute());
    }

    #[test]
    fn test_get_project_root() {
        let root = get_project_root().unwrap();
        assert!(root.is_absolute());
    }
}
