//! Dashboard Module
//!
//! Terminal User Interface (TUI) for real-time monitoring.

pub mod colors;
pub mod layout;
pub mod renderer;
pub mod widgets;

// Public exports
pub use renderer::DashboardRenderer;
pub use layout::{DashboardLayout, LayoutConfig};
pub use colors::ColorScheme;

use crate::types::{Position, PriceTick};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dashboard data shared between agents and UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    /// Agent states
    pub agent_states: HashMap<String, AgentState>,
    /// Current position (if any)
    pub position: Option<PositionData>,
    /// Balance information
    pub balance: BalanceData,
    /// Market prices
    pub market_prices: Vec<CoinPrice>,
    /// Notifications buffer
    pub notifications: Vec<Notification>,
    /// Timestamp of this data
    pub timestamp: DateTime<Utc>,
}

impl DashboardData {
    /// Create new empty dashboard data
    pub fn new() -> Self {
        Self {
            agent_states: HashMap::new(),
            position: None,
            balance: BalanceData::default(),
            market_prices: Vec::new(),
            notifications: Vec::new(),
            timestamp: Utc::now(),
        }
    }

    /// Update agent state
    pub fn update_agent_state(&mut self, agent: String, state: AgentState) {
        self.agent_states.insert(agent, state);
        self.timestamp = Utc::now();
    }

    /// Update position
    pub fn update_position(&mut self, position: Option<PositionData>) {
        self.position = position;
        self.timestamp = Utc::now();
    }

    /// Update balance
    pub fn update_balance(&mut self, balance: BalanceData) {
        self.balance = balance;
        self.timestamp = Utc::now();
    }

    /// Add market price
    pub fn add_market_price(&mut self, price: CoinPrice) {
        // Update existing or add new
        if let Some(existing) = self.market_prices.iter_mut().find(|p| p.market == price.market) {
            *existing = price;
        } else {
            self.market_prices.push(price);
        }
        self.timestamp = Utc::now();
    }

    /// Add notification
    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
        // Keep only last 50 notifications
        if self.notifications.len() > 50 {
            self.notifications.remove(0);
        }
        self.timestamp = Utc::now();
    }

    /// Clear old notifications (keep last 20)
    pub fn clear_old_notifications(&mut self) {
        if self.notifications.len() > 20 {
            self.notifications = self.notifications.split_off(self.notifications.len() - 20);
        }
    }
}

impl Default for DashboardData {
    fn default() -> Self {
        Self::new()
    }
}

/// Agent state for dashboard display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    /// Agent name
    pub name: String,
    /// Current status
    pub status: AgentStatus,
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
    /// Additional message
    pub message: Option<String>,
}

impl AgentState {
    /// Create new agent state
    pub fn new(name: String, status: AgentStatus) -> Self {
        Self {
            name,
            status,
            last_update: Utc::now(),
            message: None,
        }
    }

    /// Create with message
    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }
}

/// Agent status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    /// Running normally
    Running,
    /// Idle/waiting
    Idle,
    /// Error state
    Error,
    /// Disconnected
    Disconnected,
}

impl AgentStatus {
    /// Get display name
    pub fn as_str(&self) -> &str {
        match self {
            AgentStatus::Running => "Running",
            AgentStatus::Idle => "Idle",
            AgentStatus::Error => "Error",
            AgentStatus::Disconnected => "Disconnected",
        }
    }
}

/// Position data for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionData {
    /// Market code
    pub market: String,
    /// Entry price
    pub entry_price: f64,
    /// Current price
    pub current_price: f64,
    /// Amount
    pub amount: f64,
    /// PnL percentage
    pub pnl_rate: f64,
    /// PnL amount (KRW)
    pub pnl_amount: f64,
    /// Entry time
    pub entry_time: DateTime<Utc>,
    /// Stop loss price
    pub stop_loss: f64,
    /// Take profit price
    pub take_profit: f64,
}

impl PositionData {
    /// Create from Position type
    pub fn from_position(position: &Position, current_price: f64) -> Self {
        let pnl = position.calculate_pnl(current_price);
        Self {
            market: position.market.clone(),
            entry_price: position.entry_price,
            current_price,
            amount: position.amount,
            pnl_rate: pnl.profit_rate,
            pnl_amount: pnl.profit,
            entry_time: position.entry_time,
            stop_loss: position.stop_loss,
            take_profit: position.take_profit,
        }
    }
}

/// Balance data for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceData {
    /// KRW balance
    pub krw_balance: f64,
    /// Available KRW
    pub available_krw: f64,
    /// Locked KRW (in orders)
    pub locked_krw: f64,
    /// Total asset value (KRW + crypto)
    pub total_asset_value: f64,
    /// Today's PnL
    pub today_pnl: f64,
    /// Today's PnL rate
    pub today_pnl_rate: f64,
}

impl Default for BalanceData {
    fn default() -> Self {
        Self {
            krw_balance: 0.0,
            available_krw: 0.0,
            locked_krw: 0.0,
            total_asset_value: 0.0,
            today_pnl: 0.0,
            today_pnl_rate: 0.0,
        }
    }
}

/// Coin price for market panel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinPrice {
    /// Market code
    pub market: String,
    /// Current price
    pub price: f64,
    /// Change rate (24h)
    pub change_rate: f64,
    /// Volume (24h)
    pub volume: f64,
}

impl CoinPrice {
    /// Create from PriceTick
    pub fn from_tick(tick: &PriceTick) -> Self {
        Self {
            market: tick.market.clone(),
            price: tick.trade_price,
            change_rate: tick.change_rate,
            volume: tick.volume,
        }
    }
}

/// Notification for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Notification type
    pub notification_type: NotificationType,
    /// Message
    pub message: String,
}

impl Notification {
    /// Create new notification
    pub fn new(notification_type: NotificationType, message: String) -> Self {
        Self {
            timestamp: Utc::now(),
            notification_type,
            message,
        }
    }

    /// Format for display
    pub fn format(&self) -> String {
        let time = self.timestamp.format("%H:%M:%S");
        let type_prefix = match self.notification_type {
            NotificationType::Info => "[INFO]",
            NotificationType::Buy => "[BUY]",
            NotificationType::Sell => "[SELL]",
            NotificationType::Error => "[ERROR]",
            NotificationType::Signal => "[SIGNAL]",
            NotificationType::Warning => "[WARN]",
        };
        format!("{} {} {}", time, type_prefix, self.message)
    }
}

/// Notification type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationType {
    /// General info
    Info,
    /// Buy executed
    Buy,
    /// Sell executed
    Sell,
    /// Error occurred
    Error,
    /// Trading signal
    Signal,
    /// Warning
    Warning,
}

/// Mask API key for display (REQ-113)
///
/// Shows first 4 and last 4 characters, masks the middle.
///
/// # Examples
///
/// ```
/// use autocoin::dashboard::mask_api_key;
///
/// assert_eq!(mask_api_key("abcd1234efgh5678"), "abcd****5678");
/// assert_eq!(mask_api_key("short"), "short");
/// ```
pub fn mask_api_key(key: &str) -> String {
    if key.len() <= 8 {
        // Too short to mask meaningfully, return as-is with warning
        format!("{} (short)", key)
    } else {
        let prefix = &key[..4];
        let suffix = &key[key.len() - 4..];
        format!("{}****{}", prefix, suffix)
    }
}

/// Check if terminal supports color (REQ-119)
///
/// Checks environment variables and terminal capabilities.
pub fn supports_color() -> bool {
    // Check CLICOLOR environment variable (macOS/BSD convention)
    if let Ok clicolor) = std::env::var("CLICOLOR") {
        if clicolor != "0" && !clicolor.is_empty() {
            return true;
        }
    }

    // Check COLORTERM variable
    if let Ok(colorterm) = std::env::var("COLORTERM") {
        if !colorterm.is_empty() {
            return true;
        }
    }

    // Check TERM variable for known color terminals
    if let Ok(term) = std::env::var("TERM") {
        if term.contains("color") || term.contains("ansi") || term.contains("xterm") {
            return true;
        }
    }

    // Windows: check if supporting ANSI colors
    #[cfg(windows)]
    {
        if let Ok(_) = std::env::var("WT_SESSION") {
            // Windows Terminal
            return true;
        }
        // Modern Windows 10+ supports ANSI
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_data_creation() {
        let data = DashboardData::new();
        assert!(data.agent_states.is_empty());
        assert!(data.position.is_none());
        assert!(data.market_prices.is_empty());
        assert!(data.notifications.is_empty());
    }

    #[test]
    fn test_agent_state_creation() {
        let state = AgentState::new("MarketMonitor".to_string(), AgentStatus::Running);
        assert_eq!(state.name, "MarketMonitor");
        assert_eq!(state.status, AgentStatus::Running);
    }

    #[test]
    fn test_notification_limit() {
        let mut data = DashboardData::new();
        for i in 0..100 {
            data.add_notification(Notification::new(
                NotificationType::Info,
                format!("Notification {}", i),
            ));
        }
        assert_eq!(data.notifications.len(), 50);
        // Should keep latest 50
        assert_eq!(data.notifications[0].message, "Notification 50");
    }

    #[test]
    fn test_coin_price_from_tick() {
        let tick = PriceTick::new("KRW-BTC".to_string(), 1234567890, 50000000.0, 0.05, 0.001);
        let price = CoinPrice::from_tick(&tick);
        assert_eq!(price.market, "KRW-BTC");
        assert_eq!(price.price, 50000000.0);
        assert_eq!(price.change_rate, 0.05);
    }

    #[test]
    fn test_mask_api_key() {
        // Normal case
        assert_eq!(mask_api_key("abcd1234efgh5678"), "abcd****5678");
        assert_eq!(mask_api_key("A1B2C3D4E5F6G7H8"), "A1B2****G7H8");

        // Short key (less than 8 characters)
        assert_eq!(mask_api_key("short"), "short (short)");
        assert_eq!(mask_api_key("tiny"), "tiny (short)");

        // Exactly 8 characters
        assert_eq!(mask_api_key("12345678"), "1234****5678");
    }

    #[test]
    fn test_supports_color() {
        // This test just verifies the function runs without panicking
        // The actual result depends on environment
        let _ = supports_color();
    }
}
