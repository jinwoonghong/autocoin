//! WebSocket handler for real-time updates
//!
//! Manages WebSocket connections and broadcasts updates to all connected clients.

use super::state::TradingState;
use super::WebSocketMessage;
use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures_util::{stream::StreamExt, sink::SinkExt};
use std::sync::Arc;
use tokio::sync::broadcast;

/// WebSocket upgrade handler
///
/// Upgrades HTTP connection to WebSocket and manages the connection.
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<TradingState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Handle WebSocket connection
///
/// Manages individual WebSocket connection, sending and receiving messages.
async fn handle_socket(socket: WebSocket, state: Arc<TradingState>) {
    tracing::info!("WebSocket connection established");

    // Split the socket into sender and receiver
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to the state's broadcaster
    let mut rx = state.subscribe_broadcaster();

    // Update connection status
    state.set_websocket_connected(true).await;
    let state_clone = state.clone();

    // Spawn a task to handle incoming messages from broadcaster
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                tracing::trace!("Broadcasting WebSocket message: {}", json);
                if sender.send(Message::Text(json.into())).await.is_err() {
                    tracing::debug!("WebSocket send error, breaking send loop");
                    break;
                }
            }
        }
    });

    // Handle incoming messages (just close detection, ping/pong is auto-handled by Axum)
    let recv_result = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => {
                    tracing::info!("WebSocket client requested close");
                    break;
                },
                _ => {
                    // Axum automatically handles Ping/Pong frames
                    tracing::trace!("WebSocket message received: {:?}", msg);
                }
            }
        }
    }).await;

    // Cancel the send task when recv completes
    send_task.abort();

    // Update connection status
    state_clone.set_websocket_connected(false).await;
    tracing::info!("WebSocket connection closed");

    // Log if there was an error
    if let Err(e) = recv_result {
        tracing::debug!("WebSocket receiver task error: {:?}", e);
    }
}

/// Helper functions to create WebSocket messages
fn create_price_update(market: String, price: f64, change_rate: f64) -> WebSocketMessage {
    use chrono::Utc;
    WebSocketMessage::PriceUpdate {
        market,
        price,
        change_rate,
        timestamp: Utc::now(),
    }
}

fn create_trade_update(
    market: String,
    trade_type: String,
    price: f64,
    amount: f64,
) -> WebSocketMessage {
    use chrono::Utc;
    WebSocketMessage::TradeUpdate {
        market,
        trade_type,
        price,
        amount,
        timestamp: Utc::now(),
    }
}

fn create_position_update(
    market: String,
    entry_price: f64,
    current_price: f64,
    pnl_rate: f64,
) -> WebSocketMessage {
    use chrono::Utc;
    WebSocketMessage::PositionUpdate {
        market,
        entry_price,
        current_price,
        pnl_rate,
        timestamp: Utc::now(),
    }
}

fn create_agent_status(
    agent: String,
    status: String,
    message: Option<String>,
) -> WebSocketMessage {
    use chrono::Utc;
    WebSocketMessage::AgentStatus {
        agent,
        status,
        message,
        timestamp: Utc::now(),
    }
}

fn create_balance_update(
    krw_balance: f64,
    available_krw: f64,
    total_asset_value: f64,
) -> WebSocketMessage {
    use chrono::Utc;
    WebSocketMessage::BalanceUpdate {
        krw_balance,
        available_krw,
        total_asset_value,
        timestamp: Utc::now(),
    }
}

fn create_notification(notification_type: String, message: String) -> WebSocketMessage {
    use chrono::Utc;
    WebSocketMessage::Notification {
        notification_type,
        message,
        timestamp: Utc::now(),
    }
}

fn create_system_status(status: String, websocket_connected: bool) -> WebSocketMessage {
    use chrono::Utc;
    WebSocketMessage::SystemStatus {
        status,
        websocket_connected,
        timestamp: Utc::now(),
    }
}

/// WebSocket broadcaster for managing connected clients
#[derive(Clone)]
pub struct WebSocketBroadcaster {
    sender: broadcast::Sender<WebSocketMessage>,
}

impl WebSocketBroadcaster {
    /// Create new broadcaster
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    /// Create new broadcaster with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    /// Subscribe to broadcasts
    pub fn subscribe(&self) -> broadcast::Receiver<WebSocketMessage> {
        self.sender.subscribe()
    }

    /// Broadcast message to all subscribers
    pub fn broadcast(&self, msg: WebSocketMessage) -> Result<usize, broadcast::error::SendError<WebSocketMessage>> {
        self.sender.send(msg)
    }

    /// Get receiver count
    pub fn receiver_count(&self) -> usize {
        self.sender.receiver_count()
    }

    /// Create price update and broadcast
    pub fn broadcast_price(&self, market: String, price: f64, change_rate: f64) {
        let msg = create_price_update(market, price, change_rate);
        let _ = self.broadcast(msg);
    }

    /// Create trade update and broadcast
    pub fn broadcast_trade(&self, market: String, trade_type: String, price: f64, amount: f64) {
        let msg = create_trade_update(market, trade_type, price, amount);
        let _ = self.broadcast(msg);
    }

    /// Create position update and broadcast
    pub fn broadcast_position(&self, market: String, entry_price: f64, current_price: f64, pnl_rate: f64) {
        let msg = create_position_update(market, entry_price, current_price, pnl_rate);
        let _ = self.broadcast(msg);
    }

    /// Create agent status and broadcast
    pub fn broadcast_agent_status(&self, agent: String, status: String, message: Option<String>) {
        let msg = create_agent_status(agent, status, message);
        let _ = self.broadcast(msg);
    }

    /// Create balance update and broadcast
    pub fn broadcast_balance(&self, krw_balance: f64, available_krw: f64, total_asset_value: f64) {
        let msg = create_balance_update(krw_balance, available_krw, total_asset_value);
        let _ = self.broadcast(msg);
    }

    /// Create notification and broadcast
    pub fn broadcast_notification(&self, notification_type: String, message: String) {
        let msg = create_notification(notification_type, message);
        let _ = self.broadcast(msg);
    }

    /// Create system status and broadcast
    pub fn broadcast_system_status(&self, status: String, websocket_connected: bool) {
        let msg = create_system_status(status, websocket_connected);
        let _ = self.broadcast(msg);
    }
}

impl Default for WebSocketBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_broadcaster_creation() {
        let broadcaster = WebSocketBroadcaster::new();
        assert_eq!(broadcaster.receiver_count(), 0);
    }

    #[test]
    fn test_websocket_broadcaster_with_capacity() {
        let broadcaster = WebSocketBroadcaster::with_capacity(200);
        assert_eq!(broadcaster.receiver_count(), 0);
    }

    #[test]
    fn test_websocket_broadcaster_subscribe() {
        let broadcaster = WebSocketBroadcaster::new();
        let _rx = broadcaster.subscribe();
        assert_eq!(broadcaster.receiver_count(), 1);
    }

    #[test]
    fn test_create_price_update() {
        let msg = create_price_update("KRW-BTC".to_string(), 50000000.0, 0.05);
        match msg {
            WebSocketMessage::PriceUpdate { market, price, change_rate, .. } => {
                assert_eq!(market, "KRW-BTC");
                assert_eq!(price, 50000000.0);
                assert_eq!(change_rate, 0.05);
            }
            _ => panic!("Expected PriceUpdate"),
        }
    }

    #[test]
    fn test_create_trade_update() {
        let msg = create_trade_update("KRW-BTC".to_string(), "buy".to_string(), 50000000.0, 0.001);
        match msg {
            WebSocketMessage::TradeUpdate { market, trade_type, price, amount, .. } => {
                assert_eq!(market, "KRW-BTC");
                assert_eq!(trade_type, "buy");
                assert_eq!(price, 50000000.0);
                assert_eq!(amount, 0.001);
            }
            _ => panic!("Expected TradeUpdate"),
        }
    }

    #[test]
    fn test_broadcaster_broadcast_price() {
        let broadcaster = WebSocketBroadcaster::new();
        broadcaster.broadcast_price("KRW-BTC".to_string(), 50000000.0, 0.05);
        // Should not panic
    }

    #[test]
    fn test_broadcaster_broadcast_trade() {
        let broadcaster = WebSocketBroadcaster::new();
        broadcaster.broadcast_trade("KRW-BTC".to_string(), "buy".to_string(), 50000000.0, 0.001);
        // Should not panic
    }
}
