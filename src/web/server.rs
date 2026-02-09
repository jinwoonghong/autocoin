//! Web server implementation
//!
//! Main web server that manages HTTP and WebSocket connections.

use super::routes::{create_router, create_router_with_static};
use super::state::TradingState;
use super::{MarketPrice, PositionData, TradeRecord, WebSocketMessage};
use crate::dashboard::{AgentState as DashboardAgentState, BalanceData, Notification};
use crate::types::trading::PriceTick;
use anyhow::Result;
use chrono::Utc;
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
            host: "0.0.0.0".to_string(),
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
                .map_err(|e| anyhow::anyhow!("Failed to send shutdown signal: {e:?}"))?;

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

    /// Broadcast a WebSocket message
    fn broadcast(&self, msg: WebSocketMessage) {
        self.trading_state.broadcast(msg);
    }

    /// Update position and broadcast
    pub async fn update_position(&self, position: Option<PositionData>) {
        self.trading_state.update_position(position.clone()).await;

        if let Some(ref pos) = position {
            self.broadcast(WebSocketMessage::PositionUpdate {
                market: pos.market.clone(),
                entry_price: pos.entry_price,
                current_price: pos.current_price,
                pnl_rate: pos.pnl_rate,
                timestamp: Utc::now(),
            });
        }
    }

    /// Update balance and broadcast
    pub async fn update_balance(&self, balance: BalanceData) {
        self.trading_state.update_balance(balance.clone()).await;
        self.broadcast(WebSocketMessage::BalanceUpdate {
            krw_balance: balance.krw_balance,
            available_krw: balance.available_krw,
            total_asset_value: balance.total_asset_value,
            timestamp: Utc::now(),
        });
    }

    /// Update agent state and broadcast
    pub async fn update_agent_state(&self, name: String, state: DashboardAgentState) {
        self.trading_state
            .update_agent_state(name.clone(), state.clone())
            .await;
        self.broadcast(WebSocketMessage::AgentStatus {
            agent: name,
            status: state.status.as_str().to_string(),
            message: state.message,
            timestamp: Utc::now(),
        });
    }

    /// Update market price and broadcast
    pub async fn update_market_price(&self, price: MarketPrice) {
        self.trading_state
            .update_market_price(price.clone())
            .await;
        self.broadcast(WebSocketMessage::PriceUpdate {
            market: price.market,
            price: price.price,
            change_rate: price.change_rate,
            timestamp: Utc::now(),
        });
    }

    /// Update price from tick and broadcast
    pub async fn update_price_tick(&self, tick: &PriceTick) {
        self.trading_state.update_price_tick(tick).await;
        self.broadcast(WebSocketMessage::PriceUpdate {
            market: tick.market.clone(),
            price: tick.trade_price,
            change_rate: tick.change_rate,
            timestamp: Utc::now(),
        });
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

        self.broadcast(WebSocketMessage::Notification {
            notification_type: notif_type,
            message: notification.message,
            timestamp: Utc::now(),
        });
    }

    /// Add trade and broadcast
    pub async fn add_trade(&self, trade: TradeRecord) {
        self.trading_state.add_trade(trade.clone()).await;
        self.broadcast(WebSocketMessage::TradeUpdate {
            market: trade.market,
            trade_type: trade.trade_type,
            price: trade.price,
            amount: trade.amount,
            timestamp: Utc::now(),
        });
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
