//! Web server implementation
//!
//! Main web server that manages HTTP and WebSocket connections.

use super::routes::{create_router, create_router_with_static};
use super::state::TradingState;
use super::websocket::WebSocketBroadcaster;
use super::{MarketPrice, PositionData, TradeRecord, WebSocketMessage};
use crate::dashboard::{AgentState as DashboardAgentState, BalanceData, Notification};
use crate::types::trading::PriceTick;
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tracing::{error, info};

/// Web server configuration
#[derive(Debug, Clone)]
pub struct WebServerConfig {
    /// Host address to bind to
    pub host: String,
    /// Port to listen on
    pub port: u16,
    /// Static assets directory
    pub static_dir: Option<String>,
    /// Enable CORS
    pub enable_cors: bool,
}

impl Default for WebServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            static_dir: None,
            enable_cors: true,
        }
    }
}

/// Web server
///
/// Manages the Axum HTTP server and provides access to shared state.
pub struct WebServer {
    /// Server configuration
    config: WebServerConfig,
    /// Shared trading state
    trading_state: Arc<TradingState>,
    /// WebSocket broadcaster
    broadcaster: Arc<WebSocketBroadcaster>,
    /// Shutdown sender
    shutdown_tx: Option<oneshot::Sender<()>>,
    /// Server start time
    start_time: Option<Instant>,
}

impl WebServer {
    /// Create new web server
    ///
    /// # Arguments
    ///
    /// * `config` - Server configuration
    /// * `trading_state` - Shared trading state
    pub fn new(config: WebServerConfig, trading_state: Arc<TradingState>) -> Self {
        Self {
            config,
            trading_state,
            broadcaster: Arc::new(WebSocketBroadcaster::new()),
            shutdown_tx: None,
            start_time: None,
        }
    }

    /// Create with default configuration
    ///
    /// # Arguments
    ///
    /// * `host` - Host address to bind to
    /// * `port` - Port to listen on
    /// * `trading_state` - Shared trading state
    pub fn with_defaults(host: String, port: u16, trading_state: Arc<TradingState>) -> Self {
        Self::new(
            WebServerConfig {
                host,
                port,
                ..Default::default()
            },
            trading_state,
        )
    }

    /// Start the web server
    ///
    /// Binds to the configured address and starts accepting connections.
    pub async fn start(&mut self) -> Result<()> {
        let addr: SocketAddr = format!("{}:{}", self.config.host, self.config.port)
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid address: {}", e))?;

        info!("Starting web server on http://{}", addr);

        // Create router
        let router = if let Some(ref static_dir) = self.config.static_dir {
            create_router_with_static(self.trading_state.clone(), static_dir)
        } else {
            create_router(self.trading_state.clone())
        };

        // Create listener
        let listener = TcpListener::bind(&addr).await?;
        info!("Web server listening on http://{}", addr);

        // Create shutdown channel
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        self.shutdown_tx = Some(shutdown_tx);

        // Record start time
        self.start_time = Some(Instant::now());

        // Update system status
        self.trading_state
            .update_system_status("running".to_string())
            .await;

        // Spawn server task
        let server = axum::serve(listener, router)
            .with_graceful_shutdown(async move {
                shutdown_rx.await.ok();
                info!("Web server shutting down...");
            });

        // Run server (this will block until shutdown)
        server.await?;

        info!("Web server stopped");
        Ok(())
    }

    /// Start server in background
    ///
    /// Starts the server in a separate task and returns immediately.
    pub async fn start_background(&mut self) -> Result<tokio::task::JoinHandle<Result<()>>> {
        let addr: SocketAddr = format!("{}:{}", self.config.host, self.config.port)
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid address: {}", e))?;

        info!("Starting web server in background on http://{}", addr);

        // Create router
        let router = if let Some(ref static_dir) = self.config.static_dir {
            create_router_with_static(self.trading_state.clone(), static_dir)
        } else {
            create_router(self.trading_state.clone())
        };

        // Create listener
        let listener = TcpListener::bind(&addr).await?;
        info!("Web server listening on http://{}", addr);

        // Create shutdown channel
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        self.shutdown_tx = Some(shutdown_tx);

        // Record start time
        self.start_time = Some(Instant::now());

        // Update system status
        self.trading_state
            .update_system_status("running".to_string())
            .await;

        // Clone broadcaster for the server task
        let broadcaster = self.broadcaster.clone();
        let trading_state = self.trading_state.clone();

        // Spawn server task
        let handle = tokio::spawn(async move {
            let server = axum::serve(listener, router)
                .with_graceful_shutdown(async move {
                    shutdown_rx.await.ok();
                    info!("Web server shutting down...");
                });

            server.await?;
            Ok(())
        });

        Ok(handle)
    }

    /// Stop the web server
    ///
    /// Signals the server to shutdown gracefully.
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            shutdown_tx
                .send(())
                .map_err(|e| anyhow::anyhow!("Failed to send shutdown signal: {}", e))?;

            // Update system status
            self.trading_state
                .update_system_status("stopped".to_string())
                .await;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Server not running"))
        }
    }

    /// Get server uptime in seconds
    pub fn uptime(&self) -> Option<Duration> {
        self.start_time.map(|t| t.elapsed())
    }

    /// Get shared trading state
    pub fn trading_state(&self) -> Arc<TradingState> {
        self.trading_state.clone()
    }

    /// Get WebSocket broadcaster
    pub fn broadcaster(&self) -> Arc<WebSocketBroadcaster> {
        self.broadcaster.clone()
    }

    /// Update position and broadcast
    pub async fn update_position(&self, position: Option<PositionData>) {
        self.trading_state.update_position(position.clone()).await;

        if let Some(ref pos) = position {
            self.broadcaster.broadcast_position(
                pos.market.clone(),
                pos.entry_price,
                pos.current_price,
                pos.pnl_rate,
            );
        }
    }

    /// Update balance and broadcast
    pub async fn update_balance(&self, balance: BalanceData) {
        self.trading_state.update_balance(balance.clone()).await;
        self.broadcaster.broadcast_balance(
            balance.krw_balance,
            balance.available_krw,
            balance.total_asset_value,
        );
    }

    /// Update agent state and broadcast
    pub async fn update_agent_state(&self, name: String, state: DashboardAgentState) {
        self.trading_state
            .update_agent_state(name.clone(), state.clone())
            .await;
        self.broadcaster.broadcast_agent_status(
            name,
            state.status.as_str().to_string(),
            state.message,
        );
    }

    /// Update market price and broadcast
    pub async fn update_market_price(&self, price: MarketPrice) {
        self.trading_state
            .update_market_price(price.clone())
            .await;
        self.broadcaster
            .broadcast_price(price.market, price.price, price.change_rate);
    }

    /// Update price from tick and broadcast
    pub async fn update_price_tick(&self, tick: &PriceTick) {
        self.trading_state.update_price_tick(tick).await;
        self.broadcaster.broadcast_price(
            tick.market.clone(),
            tick.trade_price,
            tick.change_rate,
        );
    }

    /// Add notification and broadcast
    pub async fn add_notification(&self, notification: Notification) {
        use crate::dashboard::NotificationType;

        self.trading_state.add_notification(notification.clone()).await;

        let notif_type = match notification.notification_type {
            NotificationType::Info => "info",
            NotificationType::Buy => "buy",
            NotificationType::Sell => "sell",
            NotificationType::Error => "error",
            NotificationType::Signal => "signal",
            NotificationType::Warning => "warning",
        }
        .to_string();

        self.broadcaster
            .broadcast_notification(notif_type, notification.message);
    }

    /// Add trade and broadcast
    pub async fn add_trade(&self, trade: TradeRecord) {
        self.trading_state.add_trade(trade.clone()).await;
        self.broadcaster.broadcast_trade(
            trade.market.clone(),
            trade.trade_type.clone(),
            trade.price,
            trade.amount,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_server_config_default() {
        let config = WebServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert!(config.static_dir.is_none());
        assert!(config.enable_cors);
    }

    #[test]
    fn test_web_server_creation() {
        let trading_state = Arc::new(TradingState::new());
        let server = WebServer::with_defaults("127.0.0.1".to_string(), 8080, trading_state);
        assert_eq!(server.config.host, "127.0.0.1");
        assert_eq!(server.config.port, 8080);
        assert!(server.start_time.is_none());
    }

    #[tokio::test]
    async fn test_web_server_uptime() {
        let trading_state = Arc::new(TradingState::new());
        let mut server = WebServer::with_defaults("127.0.0.1".to_string(), 8080, trading_state);
        assert!(server.uptime().is_none());
    }

    #[tokio::test]
    async fn test_update_price_tick() {
        let trading_state = Arc::new(TradingState::new());
        let server = WebServer::with_defaults("127.0.0.1".to_string(), 8080, trading_state);

        let tick = PriceTick::new("KRW-BTC".to_string(), 1234567890, 50000000.0, 0.05, 0.001);
        server.update_price_tick(&tick).await;

        let prices = server.trading_state().get_market_prices().await;
        assert_eq!(prices.len(), 1);
        assert_eq!(prices[0].market, "KRW-BTC");
    }

    #[tokio::test]
    async fn test_update_agent_state() {
        use crate::dashboard::{AgentState as DashboardAgentState, AgentStatus};

        let trading_state = Arc::new(TradingState::new());
        let server = WebServer::with_defaults("127.0.0.1".to_string(), 8080, trading_state);

        let agent_state = DashboardAgentState::new("TestAgent".to_string(), AgentStatus::Running);
        server
            .update_agent_state("TestAgent".to_string(), agent_state)
            .await;

        let states = server.trading_state().get_agent_states().await;
        assert!(states.contains_key("TestAgent"));
    }
}
