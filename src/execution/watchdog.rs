//! Watchdog Module
//!
//! Process monitoring and auto-restart functionality.

use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

/// Watchdog configuration
#[derive(Debug, Clone)]
pub struct WatchdogConfig {
    /// Maximum restart attempts
    pub max_restarts: usize,
    /// Delay between restart attempts (seconds)
    pub restart_delay_secs: u64,
    /// Health check interval (seconds)
    pub health_check_interval_secs: u64,
    /// Command to run
    pub command: String,
    /// Command arguments
    pub args: Vec<String>,
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        Self {
            max_restarts: 3,
            restart_delay_secs: 10,
            health_check_interval_secs: 30,
            command: "autocoin".to_string(),
            args: vec!["--daemon".to_string()],
        }
    }
}

/// Watchdog for process monitoring
pub struct Watchdog {
    config: WatchdogConfig,
    restart_count: usize,
}

impl Watchdog {
    /// Create new watchdog
    pub fn new(config: WatchdogConfig) -> Self {
        Self {
            config,
            restart_count: 0,
        }
    }

    /// Create watchdog with default config
    pub fn with_default(command: String, args: Vec<String>) -> Self {
        let mut config = WatchdogConfig::default();
        config.command = command;
        config.args = args;
        Self::new(config)
    }

    /// Run watchdog (monitors and restarts process)
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Watchdog started: max_restarts={}, delay={}s",
              self.config.max_restarts,
              self.config.restart_delay_secs);

        loop {
            match self.spawn_process().await {
                Ok(exit_status) => {
                    self.restart_count += 1;

                    if self.restart_count >= self.config.max_restarts {
                        error!(
                            "Process failed {} times, stopping watchdog",
                            self.restart_count
                        );
                        return Err("Max restart attempts reached".into());
                    }

                    warn!(
                        "Process exited (status: {:?}), restarting in {}s (attempt {}/{})",
                        exit_status,
                        self.config.restart_delay_secs,
                        self.restart_count,
                        self.config.max_restarts
                    );

                    sleep(Duration::from_secs(self.config.restart_delay_secs)).await;
                }
                Err(e) => {
                    self.restart_count += 1;

                    if self.restart_count >= self.config.max_restarts {
                        error!("Failed to spawn process {} times: {}", self.restart_count, e);
                        return Err(format!("Max restart attempts reached: {}", e).into());
                    }

                    error!("Failed to spawn process: {}, retrying in {}s (attempt {}/{})",
                           e,
                           self.config.restart_delay_secs,
                           self.restart_count,
                           self.config.max_restarts);

                    sleep(Duration::from_secs(self.config.restart_delay_secs)).await;
                }
            }
        }
    }

    /// Spawn child process
    async fn spawn_process(&self) -> Result<std::process::ExitStatus, std::io::Error> {
        info!("Spawning process: {} {:?}",
              self.config.command,
              self.config.args);

        let mut child = Command::new(&self.config.command)
            .args(&self.config.args)
            .spawn()?;

        // Wait for process to exit
        let exit_status = child.wait()?;

        Ok(exit_status)
    }

    /// Reset restart count
    pub fn reset_restart_count(&mut self) {
        self.restart_count = 0;
        info!("Restart count reset");
    }

    /// Get current restart count
    pub fn restart_count(&self) -> usize {
        self.restart_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watchdog_config_default() {
        let config = WatchdogConfig::default();
        assert_eq!(config.max_restarts, 3);
        assert_eq!(config.restart_delay_secs, 10);
    }

    #[test]
    fn test_watchdog_creation() {
        let watchdog = Watchdog::with_default("test".to_string(), vec!["--daemon".to_string()]);
        assert_eq!(watchdog.config.command, "test");
        assert_eq!(watchdog.config.args, vec!["--daemon"]);
        assert_eq!(watchdog.restart_count(), 0);
    }

    #[test]
    fn test_restart_count() {
        let mut watchdog = Watchdog::with_default("test".to_string(), vec![]);
        assert_eq!(watchdog.restart_count(), 0);
        watchdog.reset_restart_count();
        assert_eq!(watchdog.restart_count(), 0);
    }
}
