//! Shared state for web server
//!
//! Thread-safe shared state for HTTP handlers and WebSocket connections.

use crate::dashboard::{AgentState as DashboardAgentState, BalanceData, Notification, PositionData};
use crate::types::trading::PriceTick;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared trading state
#[derive(Debug, Clone)]
pub struct TradingState {
    /// Current position (if any)
    pub position: Arc<RwLock<Option<PositionData>>>,
    /// Balance information
    pub balance: Arc<RwLock<BalanceData>>,
    /// Agent states (agent_name -> AgentState)
    pub agent_states: Arc<RwLock<HashMap<String, DashboardAgentState>>>,
    /// Market prices
    pub market_prices: Arc<RwLock<HashMap<String, MarketPrice>>>,
    /// Recent notifications
    pub notifications: Arc<RwLock<Vec<Notification>>>,
    /// System status
    pub system_status: Arc<RwLock<SystemStatus>>,
    /// Trade history
    pub trade_history: Arc<RwLock<Vec<TradeRecord>>>,
}

/// Market price with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPrice {
    /// Market code
    pub market: String,
    /// Current price
    pub price: f64,
    /// Change rate (24h)
    pub change_rate: f64,
    /// Volume
    pub volume: f64,
    /// Last update timestamp
    pub timestamp: DateTime<Utc>,
}

impl MarketPrice {
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

/// System status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    /// System status (running, stopped, error)
    pub status: String,
    /// Start time
    pub start_time: DateTime<Utc>,
    /// WebSocket connected
    pub websocket_connected: bool,
    /// Last update
    pub last_update: DateTime<Utc>,
}

impl Default for SystemStatus {
    fn default() -> Self {
        Self {
            status: "stopped".to_string(),
            start_time: Utc::now(),
            websocket_connected: false,
            last_update: Utc::now(),
        }
    }
}

/// Trade record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRecord {
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
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl TradeRecord {
    /// Create new trade record
    pub fn new(market: String, trade_type: String, price: f64, amount: f64) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            market,
            trade_type,
            price,
            amount,
            timestamp: Utc::now(),
        }
    }
}

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketMessage {
    /// Price update
    PriceUpdate {
        market: String,
        price: f64,
        change_rate: f64,
        timestamp: DateTime<Utc>,
    },
    /// Trade update
    TradeUpdate {
        market: String,
        trade_type: String,
        price: f64,
        amount: f64,
        timestamp: DateTime<Utc>,
    },
    /// Position update
    PositionUpdate {
        market: String,
        entry_price: f64,
        current_price: f64,
        pnl_rate: f64,
        timestamp: DateTime<Utc>,
    },
    /// Agent status update
    AgentStatus {
        agent: String,
        status: String,
        message: Option<String>,
        timestamp: DateTime<Utc>,
    },
    /// Balance update
    BalanceUpdate {
        krw_balance: f64,
        available_krw: f64,
        total_asset_value: f64,
        timestamp: DateTime<Utc>,
    },
    /// Notification
    Notification {
        notification_type: String,
        message: String,
        timestamp: DateTime<Utc>,
    },
    /// System status update
    SystemStatus {
        status: String,
        websocket_connected: bool,
        timestamp: DateTime<Utc>,
    },
}

impl TradingState {
    /// Create new shared trading state
    pub fn new() -> Self {
        Self {
            position: Arc::new(RwLock::new(None)),
            balance: Arc::new(RwLock::new(BalanceData::default())),
            agent_states: Arc::new(RwLock::new(HashMap::new())),
            market_prices: Arc::new(RwLock::new(HashMap::new())),
            notifications: Arc::new(RwLock::new(Vec::new())),
            system_status: Arc::new(RwLock::new(SystemStatus::default())),
            trade_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Update position
    pub async fn update_position(&self, position: Option<PositionData>) {
        *self.position.write().await = position;
        self.system_status.write().await.last_update = Utc::now();
    }

    /// Update balance
    pub async fn update_balance(&self, balance: BalanceData) {
        *self.balance.write().await = balance;
        self.system_status.write().await.last_update = Utc::now();
    }

    /// Update agent state
    pub async fn update_agent_state(&self, name: String, state: DashboardAgentState) {
        self.agent_states.write().await.insert(name, state);
        self.system_status.write().await.last_update = Utc::now();
    }

    /// Update market price
    pub async fn update_market_price(&self, price: MarketPrice) {
        self.market_prices
            .write()
            .await
            .insert(price.market.clone(), price);
        self.system_status.write().await.last_update = Utc::now();
    }

    /// Update market price from tick
    pub async fn update_price_tick(&self, tick: &PriceTick) {
        let price = MarketPrice::from_tick(tick);
        self.market_prices
            .write()
            .await
            .insert(price.market.clone(), price);
        self.system_status.write().await.last_update = Utc::now();
    }

    /// Add notification
    pub async fn add_notification(&self, notification: Notification) {
        let mut notifications = self.notifications.write().await;
        notifications.push(notification);
        // Keep only last 100 notifications
        if notifications.len() > 100 {
            notifications.remove(0);
        }
        self.system_status.write().await.last_update = Utc::now();
    }

    /// Add trade record
    pub async fn add_trade(&self, trade: TradeRecord) {
        let mut history = self.trade_history.write().await;
        history.push(trade);
        // Keep only last 1000 trades
        if history.len() > 1000 {
            history.remove(0);
        }
        self.system_status.write().await.last_update = Utc::now();
    }

    /// Update system status
    pub async fn update_system_status(&self, status: String) {
        let mut sys_status = self.system_status.write().await;
        sys_status.status = status;
        sys_status.last_update = Utc::now();
    }

    /// Set WebSocket connection status
    pub async fn set_websocket_connected(&self, connected: bool) {
        let mut sys_status = self.system_status.write().await;
        sys_status.websocket_connected = connected;
        sys_status.last_update = Utc::now();
    }

    /// Get all market prices
    pub async fn get_market_prices(&self) -> Vec<MarketPrice> {
        self.market_prices
            .read()
            .await
            .values()
            .cloned()
            .collect()
    }

    /// Get all agent states
    pub async fn get_agent_states(&self) -> HashMap<String, DashboardAgentState> {
        self.agent_states.read().await.clone()
    }

    /// Get current balance
    pub async fn get_balance(&self) -> BalanceData {
        self.balance.read().await.clone()
    }

    /// Get current position
    pub async fn get_position(&self) -> Option<PositionData> {
        self.position.read().await.clone()
    }

    /// Get recent notifications
    pub async fn get_notifications(&self, limit: usize) -> Vec<Notification> {
        let notifications = self.notifications.read().await;
        let start = if notifications.len() > limit {
            notifications.len() - limit
        } else {
            0
        };
        notifications[start..].to_vec()
    }

    /// Get trade history
    pub async fn get_trade_history(&self, limit: usize) -> Vec<TradeRecord> {
        let history = self.trade_history.read().await;
        let start = if history.len() > limit {
            history.len() - limit
        } else {
            0
        };
        history[start..].to_vec()
    }

    /// Get system status
    pub async fn get_system_status(&self) -> SystemStatus {
        self.system_status.read().await.clone()
    }
}

impl Default for TradingState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_trading_state_creation() {
        let state = TradingState::new();
        let position = state.get_position().await;
        assert!(position.is_none());

        let balance = state.get_balance().await;
        assert_eq!(balance.krw_balance, 0.0);
    }

    #[tokio::test]
    async fn test_market_price_update() {
        let state = TradingState::new();
        let tick = PriceTick::new("KRW-BTC".to_string(), 1234567890, 50000000.0, 0.05, 0.001);
        state.update_price_tick(&tick).await;

        let prices = state.get_market_prices().await;
        assert_eq!(prices.len(), 1);
        assert_eq!(prices[0].market, "KRW-BTC");
        assert_eq!(prices[0].price, 50000000.0);
    }

    #[tokio::test]
    async fn test_agent_state_update() {
        let state = TradingState::new();
        use crate::dashboard::{AgentState, AgentStatus};

        let agent_state = AgentState::new("TestAgent".to_string(), AgentStatus::Running);
        state
            .update_agent_state("TestAgent".to_string(), agent_state)
            .await;

        let states = state.get_agent_states().await;
        assert_eq!(states.len(), 1);
        assert!(states.contains_key("TestAgent"));
    }

    #[tokio::test]
    async fn test_notification_limit() {
        let state = TradingState::new();
        use crate::dashboard::{Notification, NotificationType};

        for i in 0..150 {
            state
                .add_notification(Notification::new(
                    NotificationType::Info,
                    format!("Notification {}", i),
                ))
                .await;
        }

        let notifications = state.get_notifications(200).await;
        assert_eq!(notifications.len(), 100);
    }

    #[tokio::test]
    async fn test_system_status_update() {
        let state = TradingState::new();
        state.update_system_status("running".to_string()).await;

        let status = state.get_system_status().await;
        assert_eq!(status.status, "running");
    }
}
