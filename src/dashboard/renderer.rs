//! Dashboard Renderer
//!
//! Main TUI rendering logic for the dashboard.

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Frame, Terminal,
};
use std::io::{self, Stdout};
use tokio::sync::mpsc;
use tracing::{error, info};

use super::{
    layout::{DashboardLayout, LayoutConfig},
    widgets,
    AgentState,
    AgentStatus,
    ColorScheme,
    DashboardData,
};

/// User input action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserAction {
    /// Quit the application
    Quit,
    /// Pause trading
    Pause,
    /// Resume trading
    Resume,
    /// Show help
    Help,
    /// No action
    None,
}

/// Dashboard renderer
pub struct DashboardRenderer {
    /// Terminal instance
    terminal: Terminal<CrosstermBackend<Stdout>>,
    /// Color scheme
    colors: ColorScheme,
    /// Layout configuration
    layout: DashboardLayout,
}

impl DashboardRenderer {
    /// Create new dashboard renderer
    pub fn new() -> Result<Self, io::Error> {
        // Initialize terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        // Get initial terminal size
        let size = terminal.size().unwrap_or_else(|_| ratatui::layout::Rect::new(0, 0, 120, 40));

        let layout = DashboardLayout::from_terminal_size(size.width, size.height);

        Ok(Self {
            terminal,
            colors: ColorScheme::default(),
            layout,
        })
    }

    /// Render dashboard with data
    ///
    /// Returns Ok(()) on success, Err on terminal failure.
    /// Callers should handle render failures gracefully (REQ-120).
    pub fn render(&mut self, data: &DashboardData) -> Result<(), io::Error> {
        // Check if terminal size is too small (minimum 80x24)
        let current_size = self.terminal.size()?;
        if current_size.width < 80 || current_size.height < 24 {
            // Terminal too small, skip rendering but don't fail
            return Ok(());
        }

        self.terminal.draw(|frame| {
            let size = frame.size();

            // Update layout if terminal size changed (REQ-109)
            if size.width != self.layout.config.width || size.height != self.layout.config.height {
                self.layout = DashboardLayout::from_terminal_size(size.width, size.height);
            }

            // Calculate panel areas
            let panels = self.layout.calculate_panels(size);

            // Render header
            let title = format!(
                " AutoCoin Trading Bot v0.1.0                     {} ",
                data.timestamp.format("%Y-%m-%d %H:%M:%S %Z")
            );
            self.layout.render_header(frame, size, &title);

            // Collect agent states for display
            let agent_states: Vec<_> = data
                .agent_states
                .values()
                .cloned()
                .collect();

            // Render status panel
            widgets::render_status_panel(frame, panels.status, &agent_states, &self.colors);

            // Render position panel
            widgets::render_position_panel(frame, panels.position, data.position.as_ref(), &self.colors);

            // Render balance panel
            widgets::render_balance_panel(frame, panels.balance, &data.balance, &self.colors);

            // Render market panel
            widgets::render_market_panel(frame, panels.market, &data.market_prices, &self.colors);

            // Render notifications panel (only in normal mode)
            if !self.layout.config.is_compact() && panels.notifications.height > 0 {
                widgets::render_notifications_panel(
                    frame,
                    panels.notifications,
                    &data.notifications,
                    &self.colors,
                );
            }

            // Render help footer
            if panels.help.height > 0 {
                widgets::render_help_footer(frame, panels.help, &self.colors);
            }
        })?;

        Ok(())
    }

    /// Render with fallback to log on failure (REQ-120)
    ///
    /// This function wraps render() and logs any errors instead of propagating them.
    /// This ensures UI failures never block the trading loop.
    pub fn render_or_log(&mut self, data: &DashboardData) {
        if let Err(e) = self.render(data) {
            error!("Dashboard render error (continuing without UI): {}", e);
            // Log the data that would have been displayed
            log_dashboard_data(data);
        }
    }

/// Log dashboard data when UI rendering fails (fallback mode)
fn log_dashboard_data(data: &DashboardData) {
    info!("=== Dashboard Data (UI render failed, showing log view) ===");
    info!("Timestamp: {}", data.timestamp.format("%Y-%m-%d %H:%M:%S"));

    // Log agent states
    for (name, state) in &data.agent_states {
        info!("  Agent: {} - Status: {}", name, state.status.as_str());
    }

    // Log position
    if let Some(ref pos) = data.position {
        info!(
            "  Position: {} @ {:.0}, PnL: {:.2}%",
            pos.market, pos.current_price, pos.pnl_rate * 100.0
        );
    }

    // Log balance
    info!(
        "  Balance: {:.0} KRW available, {:.0} total",
        data.balance.available_krw, data.balance.total_asset_value
    );

    // Log recent notifications (last 5)
    let recent_notifications: Vec<_> = data.notifications.iter().rev().take(5).collect();
    if !recent_notifications.is_empty() {
        info!("  Recent notifications:");
        for notif in recent_notifications {
            info!("    - {}", notif.format());
        }
    }

    info!("=== End Dashboard Data ===");
}

    /// Handle user input (non-blocking)
    pub fn handle_input(&mut self) -> Result<UserAction, io::Error> {
        // Check for event with timeout (0 = poll)
        if event::poll(std::time::Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                return Ok(self.map_key_event(key));
            }
        }

        Ok(UserAction::None)
    }

    /// Map key event to action
    fn map_key_event(&self, key: KeyEvent) -> UserAction {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => UserAction::Quit,
            KeyCode::Char('p') | KeyCode::Char('P') => UserAction::Pause,
            KeyCode::Char('r') | KeyCode::Char('R') => UserAction::Resume,
            KeyCode::Char('?') => UserAction::Help,
            KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                UserAction::Quit
            }
            _ => UserAction::None,
        }
    }

    /// Cleanup terminal on shutdown
    pub fn cleanup(mut self) -> Result<(), io::Error> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

impl Drop for DashboardRenderer {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
    }
}

/// Run dashboard UI in a separate task
pub async fn run_dashboard(
    mut data_rx: mpsc::Receiver<DashboardData>,
    action_tx: mpsc::Sender<UserAction>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting dashboard UI");

    let mut renderer = DashboardRenderer::new()?;

    // Initial render with empty data
    let initial_data = DashboardData::new();
    renderer.render_or_log(&initial_data);

    // Main UI loop with error recovery
    let mut consecutive_render_errors = 0;
    const MAX_RENDER_ERRORS: usize = 5;

    loop {
        // Check for data updates
        match data_rx.try_recv() {
            Ok(data) => {
                // Use render_or_log for graceful fallback (REQ-120)
                renderer.render_or_log(&data);
                consecutive_render_errors = 0; // Reset on success
            }
            Err(mpsc::error::TryRecvError::Empty) => {
                // No new data, continue
            }
            Err(mpsc::error::TryRecvError::Closed) => {
                info!("Data channel closed, shutting down dashboard");
                break;
            }
        }

        // Handle user input (with error handling)
        match renderer.handle_input() {
            Ok(UserAction::Quit) => {
                info!("Quit action received, shutting down dashboard");
                let _ = action_tx.send(UserAction::Quit).await;
                break;
            }
            Ok(UserAction::Pause) => {
                info!("Pause action received");
                let _ = action_tx.send(UserAction::Pause).await;
            }
            Ok(UserAction::Resume) => {
                info!("Resume action received");
                let _ = action_tx.send(UserAction::Resume).await;
            }
            Ok(UserAction::Help) => {
                // Help action - could show help overlay
                info!("Help action received");
            }
            Ok(UserAction::None) => {
                // No action, continue
            }
            Err(e) => {
                error!("Input handling error: {}, continuing", e);
                consecutive_render_errors += 1;
                if consecutive_render_errors >= MAX_RENDER_ERRORS {
                    error!("Too many consecutive errors, shutting down dashboard");
                    break;
                }
            }
        }

        // Small delay to prevent busy waiting
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    // Cleanup
    renderer.cleanup()?;
    info!("Dashboard UI stopped");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dashboard::{AgentState, AgentStatus};

    #[test]
    fn test_user_action_mapping() {
        let renderer = DashboardRenderer::new().unwrap();

        // Test quit key
        let action = renderer.map_key_event(KeyEvent::new(KeyCode::Char('q'), event::KeyModifiers::empty()));
        assert_eq!(action, UserAction::Quit);

        // Test pause key
        let action = renderer.map_key_event(KeyEvent::new(KeyCode::Char('p'), event::KeyModifiers::empty()));
        assert_eq!(action, UserAction::Pause);

        // Test resume key
        let action = renderer.map_key_event(KeyEvent::new(KeyCode::Char('r'), event::KeyModifiers::empty()));
        assert_eq!(action, UserAction::Resume);
    }
}
