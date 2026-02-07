//! Web Dashboard Module
//!
//! Axum-based HTTP and WebSocket server for web dashboard.

pub mod handlers;
pub mod routes;
pub mod server;
pub mod state;
pub mod websocket;

// Public exports
pub use server::WebServer;
pub use state::{TradingState, WebSocketMessage};

use crate::dashboard::{AgentStatus, AgentState as DashboardAgentState, NotificationType};
use crate::types::trading::{Position, PriceTick, Signal};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Coin price for API responses
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
    /// Update timestamp
    pub timestamp: DateTime<Utc>,
}

impl CoinPrice {
    /// Create from PriceTick
    pub fn from_tick(tick: &PriceTick) -> Self {
        Self {
            market: tick.market.clone(),
            price: tick.trade_price,
            change_rate: tick.change_rate,
            volume: tick.volume,
            timestamp: tick.to_datetime(),
        }
    }
}

/// Agent state for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    /// Agent name
    pub name: String,
    /// Current status
    pub status: String,
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
    /// Additional message
    pub message: Option<String>,
}

impl AgentState {
    /// Create from dashboard agent state
    pub fn from_dashboard_state(state: &DashboardAgentState) -> Self {
        Self {
            name: state.name.clone(),
            status: state.status.as_str().to_string(),
            last_update: state.last_update,
            message: state.message.clone(),
        }
    }
}

/// Position data for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionResponse {
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

impl PositionResponse {
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

/// Balance data for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceResponse {
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

/// System status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    /// System status (running, stopped, error)
    pub status: String,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Connected WebSocket status
    pub websocket_connected: bool,
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
}

/// Trade history item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeHistory {
    /// Trade ID
    pub id: String,
    /// Market code
    pub market: String,
    /// Trade type (buy/sell)
    pub trade_type: String,
    /// Price
    pub price: f64,
    /// Amount
    pub amount: f64,
    /// Total value (KRW)
    pub total_value: f64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Notification for WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Notification type
    pub notification_type: String,
    /// Message
    pub message: String,
}

impl Notification {
    /// Create from dashboard notification
    pub fn from_dashboard_notif(notif: &crate::dashboard::Notification) -> Self {
        Self {
            timestamp: notif.timestamp,
            notification_type: match notif.notification_type {
                NotificationType::Info => "info",
                NotificationType::Buy => "buy",
                NotificationType::Sell => "sell",
                NotificationType::Error => "error",
                NotificationType::Signal => "signal",
                NotificationType::Warning => "warning",
            }
            .to_string(),
            message: notif.message.clone(),
        }
    }
}

/// Signal for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalResponse {
    /// Market code
    pub market: String,
    /// Signal type
    pub signal_type: String,
    /// Confidence (0.0 ~ 1.0)
    pub confidence: f64,
    /// Reason
    pub reason: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl SignalResponse {
    /// Create from Signal type
    pub fn from_signal(signal: &Signal) -> Self {
        Self {
            market: signal.market.clone(),
            signal_type: format!("{:?}", signal.signal_type),
            confidence: signal.confidence,
            reason: signal.reason.clone(),
            timestamp: signal.timestamp,
        }
    }
}

/// API error response
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
    /// Error code
    pub code: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl ErrorResponse {
    /// Create new error response
    pub fn new(error: String) -> Self {
        Self {
            error,
            code: None,
            timestamp: Utc::now(),
        }
    }

    /// Create with error code
    pub fn with_code(error: String, code: String) -> Self {
        Self {
            error,
            code: Some(code),
            timestamp: Utc::now(),
        }
    }
}

/// API result type
pub type ApiResult<T> = Result<T, ErrorResponse>;

/// Dashboard data aggregation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    /// Balance information
    pub balance: BalanceResponse,
    /// Current position (if any)
    pub position: Option<PositionResponse>,
    /// Agent states
    pub agents: Vec<AgentState>,
    /// Trade history
    pub trades: Vec<TradeHistory>,
    /// Notifications
    pub notifications: Vec<Notification>,
    /// Market prices
    pub market_prices: Vec<CoinPrice>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_price_from_tick() {
        let tick = PriceTick::new("KRW-BTC".to_string(), 1234567890, 50000000.0, 0.05, 0.001);
        let price = CoinPrice::from_tick(&tick);
        assert_eq!(price.market, "KRW-BTC");
        assert_eq!(price.price, 50000000.0);
    }

    #[test]
    fn test_error_response() {
        let error = ErrorResponse::new("Test error".to_string());
        assert_eq!(error.error, "Test error");
        assert!(error.code.is_none());
    }

    #[test]
    fn test_signal_response() {
        let signal = Signal::buy("KRW-BTC".to_string(), 0.8, "Surge detected".to_string());
        let response = SignalResponse::from_signal(&signal);
        assert_eq!(response.market, "KRW-BTC");
        assert_eq!(response.confidence, 0.8);
    }
}
