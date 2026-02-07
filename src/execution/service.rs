//! Service Configuration Module
//!
//! Generate service configuration files for auto-startup.

use std::path::PathBuf;

/// Service configuration
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    /// Service name
    pub name: String,
    /// Service description
    pub description: String,
    /// Executable path
    pub executable: PathBuf,
    /// Command line arguments
    pub args: Vec<String>,
    /// Working directory
    pub working_dir: PathBuf,
    /// Auto-restart
    pub auto_restart: bool,
    /// Log file path
    pub log_file: Option<PathBuf>,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            name: "autocoin".to_string(),
            description: "Upbit Automated Trading Bot".to_string(),
            executable: PathBuf::from("autocoin"),
            args: vec!["--daemon".to_string()],
            working_dir: PathBuf::from("."),
            auto_restart: true,
            log_file: None,
        }
    }
}

/// Generate Linux systemd service file
pub fn generate_systemd_service(config: &ServiceConfig) -> String {
    let exec_command = if let Some(ref log_file) = config.log_file {
        format!(
            "{} {} >> {} 2>&1",
            config.executable.display(),
            config.args.join(" "),
            log_file.display()
        )
    } else {
        format!("{} {}", config.executable.display(), config.args.join(" "))
    };

    let restart = if config.auto_restart { "always" } else { "no" };

    format!(
        r#"[Unit]
Description={service_name}
After=network.target

[Service]
Type=simple
User={current_user}
WorkingDirectory={working_dir}
ExecStart={exec_command}
Restart={restart}
RestartSec=10
StandardOutput=journal
StandardError=journal

# Security settings
NoNewPrivileges=true
PrivateTmp=true

[Install]
WantedBy=multi-user.target
"#,
        service_name = config.description,
        current_user = std::env::var("USER").unwrap_or_else(|_| "autocoin".to_string()),
        working_dir = config.working_dir.display(),
        exec_command = exec_command
    )
}

/// Generate Windows Task Scheduler XML
pub fn generate_task_scheduler_xml(config: &ServiceConfig) -> String {
    let exec_command = format!(
        "{} {}",
        config.executable.display(),
        config.args.join(" ")
    );

    let working_dir = config.working_dir.display();

    let log_redirect = if let Some(ref log_file) = config.log_file {
        format!(">> {}", log_file.display())
    } else {
        String::new()
    };

    format!(
        r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.4" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <RegistrationInfo>
    <Date>{date}</Date>
    <Author>AutoCoin</Author>
    <Version>1.0</Version>
    <Description>{description}</Description>
  </RegistrationInfo>
  <Triggers>
    <BootTrigger>
      <Enabled>true</Enabled>
    </BootTrigger>
    <LogonTrigger>
      <Enabled>true</Enabled>
    </LogonTrigger>
  </Triggers>
  <Principals>
    <Principal id="Author">
      <UserId>{current_user}</UserId>
      <LogonType>InteractiveToken</LogonType>
      <RunLevel>LeastPrivilege</RunLevel>
    </Principal>
  </Principals>
  <Settings>
    <MultipleInstancesPolicy>IgnoreNew</MultipleInstancesPolicy>
    <DisallowStartIfOnBatteries>false</DisallowStartIfOnBatteries>
    <StopIfGoingOnBatteries>false</StopIfGoingOnBatteries>
    <AllowHardTerminate>true</AllowHardTerminate>
    <StartWhenAvailable>true</StartWhenAvailable>
    <RunOnlyIfNetworkAvailable>true</RunOnlyIfNetworkAvailable>
    <IdleSettings>
      <StopOnIdleEnd>false</StopOnIdleEnd>
      <RestartOnIdle>false</RestartOnIdle>
    </IdleSettings>
    <AllowStartOnDemand>true</AllowStartOnDemand>
    <Enabled>true</Enabled>
    <Hidden>false</Hidden>
    <RunOnlyIfIdle>false</RunOnlyIfIdle>
    <DisallowStartOnRemoteAppSession>false</DisallowStartOnRemoteAppSession>
    <UseUnifiedSchedulingEngine>true</UseUnifiedSchedulingEngine>
    <WakeToRun>false</WakeToRun>
    <ExecutionTimeLimit>PT0S</ExecutionTimeLimit>
    <Priority>7</Priority>
  </Settings>
  <Actions Context="Author">
    <Exec>
      <Command>{command}</Command>
      <Arguments>{arguments}{log_redirect}</Arguments>
      <WorkingDirectory>{working_dir}</WorkingDirectory>
    </Exec>
  </Actions>
</Task>
"#,
        date = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S"),
        description = config.description,
        current_user = std::env::var("USERNAME").unwrap_or_else(|_| "AUTOCON$".to_string()),
        command = config.executable.display(),
        arguments = config.args.join(" "),
        log_redirect = log_redirect,
        working_dir = working_dir
    )
}

/// Install helper commands (printed to user)
pub fn get_install_commands(config: &ServiceConfig) -> (String, String) {
    let systemd_service = generate_systemd_service(config);
    let scheduler_xml = generate_task_scheduler_xml(config);

    let linux_install = format!(
        r#"# Linux (systemd) Installation
# 1. Save service file:
sudo tee /etc/systemd/system/{service_name}.service <<'EOF'
{systemd_service}
EOF

# 2. Reload systemd and enable service:
sudo systemctl daemon-reload
sudo systemctl enable {service_name}
sudo systemctl start {service_name}

# 3. Check status:
sudo systemctl status {service_name}

# 4. View logs:
sudo journalctl -u {service_name} -f
"#,
        service_name = config.name,
        systemd_service = systemd_service.trim()
    );

    let windows_install = format!(
        r#"# Windows Task Scheduler Installation
# 1. Save XML to file: {service_name}.xml
# 2. Import task:
schtasks /Create /TN "{service_name}" /XML "{service_name}.xml"

# 3. Run task manually:
schtasks /Run /TN "{service_name}"

# 4. Delete task:
schtasks /Delete /TN "{service_name}" /F
"#,
        service_name = config.name
    );

    (linux_install, windows_install)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_config_default() {
        let config = ServiceConfig::default();
        assert_eq!(config.name, "autocoin");
        assert!(config.auto_restart);
    }

    #[test]
    fn test_generate_systemd_service() {
        let config = ServiceConfig::default();
        let service = generate_systemd_service(&config);
        assert!(service.contains("[Unit]"));
        assert!(service.contains("[Service]"));
        assert!(service.contains("After=network.target"));
    }

    #[test]
    fn test_generate_task_scheduler_xml() {
        let config = ServiceConfig::default();
        let xml = generate_task_scheduler_xml(&config);
        assert!(xml.contains("<?xml"));
        assert!(xml.contains("<Task"));
        assert!(xml.contains("<Triggers>"));
    }

    #[test]
    fn test_install_commands() {
        let config = ServiceConfig::default();
        let (linux, windows) = get_install_commands(&config);
        assert!(linux.contains("systemctl"));
        assert!(windows.contains("schtasks"));
    }
}
