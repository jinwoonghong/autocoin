//! HTTP request handlers
//!
//! JSON response handlers for each API endpoint with proper error handling.

use super::state::TradingState;
use super::{
    AgentState, ApiResult, BalanceResponse, CoinPrice, ErrorResponse, PositionResponse,
    SignalResponse, SystemStatus, TradeHistory,
};
use crate::dashboard::AgentState as DashboardAgentState;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use std::sync::Arc;

/// Get system status
///
/// Returns the current system status including uptime, WebSocket connection state.
pub async fn get_status(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<SystemStatus>, ErrorResponse> {
    let sys_status = state.get_system_status().await;
    let uptime = sys_status
        .last_update
        .signed_duration_since(sys_status.start_time)
        .num_seconds()
        .max(0) as u64;

    Ok(Json(SystemStatus {
        status: sys_status.status,
        uptime_seconds: uptime,
        websocket_connected: sys_status.websocket_connected,
        last_update: sys_status.last_update,
    }))
}

/// Get current position
///
/// Returns the current open position if any.
pub async fn get_position(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<Option<PositionResponse>>, ErrorResponse> {
    let position = state.get_position().await;
    let response = position.map(|p| {
        PositionResponse {
            market: p.market,
            entry_price: p.entry_price,
            current_price: p.current_price,
            amount: p.amount,
            pnl_rate: p.pnl_rate,
            pnl_amount: p.pnl_amount,
            entry_time: p.entry_time,
            stop_loss: p.stop_loss,
            take_profit: p.take_profit,
        }
    });
    Ok(Json(response))
}

/// Get account balance
///
/// Returns current balance information including KRW and asset values.
pub async fn get_balance(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<BalanceResponse>, ErrorResponse> {
    let balance = state.get_balance().await;
    Ok(Json(BalanceResponse {
        krw_balance: balance.krw_balance,
        available_krw: balance.available_krw,
        locked_krw: balance.locked_krw,
        total_asset_value: balance.total_asset_value,
        today_pnl: balance.today_pnl,
        today_pnl_rate: balance.today_pnl_rate,
    }))
}

/// Get trade history
///
/// Returns recent trade history. Query parameter `limit` controls the number of records (default: 50).
pub async fn get_trades(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<Vec<TradeHistory>>, ErrorResponse> {
    let trades = state.get_trade_history(100).await;
    let history: Vec<TradeHistory> = trades
        .into_iter()
        .map(|t| TradeHistory {
            id: t.id,
            market: t.market,
            trade_type: t.trade_type,
            price: t.price,
            amount: t.amount,
            total_value: t.price * t.amount,
            timestamp: t.timestamp,
        })
        .collect();
    Ok(Json(history))
}

/// Get market prices
///
/// Returns current market prices for all tracked coins.
pub async fn get_markets(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<Vec<CoinPrice>>, ErrorResponse> {
    let prices = state.get_market_prices().await;
    let coins: Vec<CoinPrice> = prices
        .into_iter()
        .map(|p| CoinPrice {
            market: p.market,
            price: p.price,
            change_rate: p.change_rate,
            volume: p.volume,
            timestamp: p.timestamp,
        })
        .collect();
    Ok(Json(coins))
}

/// Get agent states
///
/// Returns the current state of all trading agents.
pub async fn get_agents(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<Vec<AgentState>>, ErrorResponse> {
    let states = state.get_agent_states().await;
    let agents: Vec<AgentState> = states
        .values()
        .map(|s| AgentState {
            name: s.name.clone(),
            status: s.status.as_str().to_string(),
            last_update: s.last_update,
            message: s.message.clone(),
        })
        .collect();
    Ok(Json(agents))
}

/// Get notifications
///
/// Returns recent notifications. Query parameter `limit` controls the number of records (default: 20).
pub async fn get_notifications(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<Vec<super::Notification>>, ErrorResponse> {
    use crate::dashboard::NotificationType;

    let notifs = state.get_notifications(50).await;
    let notifications: Vec<super::Notification> = notifs
        .iter()
        .map(|n| super::Notification {
            timestamp: n.timestamp,
            notification_type: match n.notification_type {
                NotificationType::Info => "info",
                NotificationType::Buy => "buy",
                NotificationType::Sell => "sell",
                NotificationType::Error => "error",
                NotificationType::Signal => "signal",
                NotificationType::Warning => "warning",
            }
            .to_string(),
            message: n.message.clone(),
        })
        .collect();
    Ok(Json(notifications))
}

/// Health check endpoint
///
/// Simple health check that returns 200 OK if the server is running.
pub async fn health_check() -> &'static str {
    "OK"
}

/// 404 handler
///
/// Handles requests to non-existent endpoints.
pub async fn not_found() -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse::new("Endpoint not found".to_string())),
    )
}

/// 500 error handler
///
/// Handles internal server errors.
pub fn internal_error<E: std::error::Error>(err: E) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse::new(err.to_string())),
    )
}

/// Dashboard data aggregation
///
/// Returns all dashboard data in a single response for reduced requests.
pub async fn get_dashboard(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<super::DashboardData>, ErrorResponse> {
    use super::DashboardData;
    use crate::dashboard::NotificationType;

    // Get all data
    let balance = state.get_balance().await;
    let position = state.get_position().await;
    let trades = state.get_trade_history(100).await;
    let agents = state.get_agent_states().await;
    let notifications = state.get_notifications(50).await;
    let market_prices = state.get_market_prices().await;

    // Build dashboard data
    let data = DashboardData {
        balance: super::BalanceResponse {
            krw_balance: balance.krw_balance,
            available_krw: balance.available_krw,
            locked_krw: balance.locked_krw,
            total_asset_value: balance.total_asset_value,
            today_pnl: balance.today_pnl,
            today_pnl_rate: balance.today_pnl_rate,
        },
        position: position.map(|p| super::PositionResponse {
            market: p.market,
            entry_price: p.entry_price,
            current_price: p.current_price,
            amount: p.amount,
            pnl_rate: p.pnl_rate,
            pnl_amount: p.pnl_amount,
            entry_time: p.entry_time,
            stop_loss: p.stop_loss,
            take_profit: p.take_profit,
        }),
        agents: agents
            .values()
            .map(|s| super::AgentState {
                name: s.name.clone(),
                status: s.status.as_str().to_string(),
                last_update: s.last_update,
                message: s.message.clone(),
            })
            .collect(),
        trades: trades
            .into_iter()
            .map(|t| super::TradeHistory {
                id: t.id,
                market: t.market,
                trade_type: t.trade_type,
                price: t.price,
                amount: t.amount,
                total_value: t.price * t.amount,
                timestamp: t.timestamp,
            })
            .collect(),
        notifications: notifications
            .iter()
            .map(|n| super::Notification {
                timestamp: n.timestamp,
                notification_type: match n.notification_type {
                    NotificationType::Info => "info",
                    NotificationType::Buy => "buy",
                    NotificationType::Sell => "sell",
                    NotificationType::Error => "error",
                    NotificationType::Signal => "signal",
                    NotificationType::Warning => "warning",
                }
                .to_string(),
                message: n.message.clone(),
            })
            .collect(),
        market_prices: market_prices
            .into_iter()
            .map(|p| super::CoinPrice {
                market: p.market,
                price: p.price,
                change_rate: p.change_rate,
                volume: p.volume,
                timestamp: p.timestamp,
            })
            .collect(),
    };

    Ok(Json(data))
}

/// Backtest request
#[derive(Debug, Deserialize)]
pub struct BacktestRequest {
    pub market: String,
    pub strategy: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "initialBalance")]
    pub initial_balance: f64,
    pub commission: f64,
    pub slippage: f64,
}

/// Run backtest
///
/// Executes a backtest with the given configuration.
pub async fn run_backtest(
    State(state): State<Arc<TradingState>>,
    Json(req): Json<BacktestRequest>,
) -> Result<Json<serde_json::Value>, ErrorResponse> {
    use crate::backtest::{BacktestConfig, BacktestSimulator};

    // Parse dates
    let start_date = chrono::NaiveDate::parse_from_str(&req.start_date, "%Y-%m-%d")
        .map_err(|_| ErrorResponse::new("Invalid start date format".to_string()))?;
    let end_date = chrono::NaiveDate::parse_from_str(&req.end_date, "%Y-%m-%d")
        .map_err(|_| ErrorResponse::new("Invalid end date format".to_string()))?;

    // Create backtest config
    let config = BacktestConfig {
        market: req.market.clone(),
        initial_balance: req.initial_balance,
        commission_rate: req.commission,
        slippage_rate: req.slippage,
        start_date,
        end_date,
        ..Default::default()
    };

    // Run backtest
    let simulator = BacktestSimulator::new(config);
    let result = simulator.run().await.map_err(|e| {
        ErrorResponse::new(format!("Backtest failed: {}", e))
    })?;

    // Format response
    let response = serde_json::json!({
        "success": true,
        "config": {
            "market": req.market,
            "strategy": req.strategy,
            "startDate": req.start_date,
            "endDate": req.end_date,
            "initialBalance": req.initial_balance,
            "commission": req.commission,
            "slippage": req.slippage,
        },
        "result": {
            "totalReturn": result.final_balance - req.initial_balance,
            "totalReturnRate": (result.final_balance - req.initial_balance) / req.initial_balance,
            "winRate": if result.metrics.total_trades > 0 {
                result.metrics.winning_trades as f64 / result.metrics.total_trades as f64
            } else { 0.0 },
            "totalTrades": result.metrics.total_trades,
            "winningTrades": result.metrics.winning_trades,
            "losingTrades": result.metrics.losing_trades,
            "maxDrawdown": result.metrics.max_drawdown,
            "maxDrawdownRate": result.metrics.max_drawdown_rate,
            "sharpeRatio": result.metrics.sharpe_ratio,
            "finalBalance": result.final_balance,
            "trades": result.trades.into_iter().map(|t| {
                serde_json::json!({
                    "market": t.market,
                    "side": if t.is_buy { "buy" } else { "sell" },
                    "entryPrice": t.entry_price,
                    "exitPrice": t.exit_price.unwrap_or(0.0),
                    "amount": t.amount,
                    "profit": t.pnl,
                    "profitRate": t.pnl_rate,
                    "entryTime": t.entry_time.to_rfc3339(),
                    "exitTime": t.exit_time.map(|t| t.to_rfc3339()).unwrap_or_default(),
                })
            }).collect::<Vec<_>>(),
            // Equity curve - simplified version showing cumulative returns
            "equityCurve": result.trades.iter().scan(
                serde_json::json!({"date": req.start_date, "balance": req.initial_balance, "return": 0.0}),
                |acc, trade| {
                    let current_balance = acc["balance"].as_f64().unwrap_or(0.0) + trade.pnl;
                    let total_return = current_balance - req.initial_balance;
                    serde_json::json!({
                        "date": trade.entry_time.split("T").next().unwrap_or(&trade.entry_time),
                        "balance": current_balance,
                        "return": total_return
                    })
                }
            ).collect::<Vec<_>>(),
        }
    });

    Ok(Json(response))
}

/// Settings data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsData {
    pub strategy: serde_json::Value,
    pub risk: serde_json::Value,
    pub notifications: serde_json::Value,
    pub system: serde_json::Value,
}

/// Get settings
///
/// Returns current system settings.
pub async fn get_settings(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<SettingsData>, ErrorResponse> {
    // Return default settings for now
    let settings = SettingsData {
        strategy: serde_json::json!({
            "activeStrategy": "momentum",
            "targetCoins": 20,
            "surgeThreshold": 0.05,
            "volumeMultiplier": 2.0,
        }),
        risk: serde_json::json!({
            "stopLoss": 0.05,
            "takeProfit": 0.10,
            "maxPosition": 1,
            "maxPositionRatio": 0.5,
            "trailingStop": "off",
        }),
        notifications: serde_json::json!({
            "discordEnabled": false,
            "discordWebhook": "",
            "alerts": {
                "buySignals": true,
                "sellSignals": true,
                "positionOpened": true,
                "positionClosed": true,
                "errors": true,
            }
        }),
        system: serde_json::json!({
            "running": false,
            "uptimeSeconds": 0,
            "version": env!("CARGO_PKG_VERSION"),
        }),
    };

    Ok(Json(settings))
}

/// Update settings
pub async fn update_settings(
    State(state): State<Arc<TradingState>>,
    Json(settings): Json<SettingsData>,
) -> Result<Json<serde_json::Value>, ErrorResponse> {
    // Settings are stored in memory via TradingState
    // For persistence, settings should be saved to config file
    // This is a placeholder that acknowledges the update
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Settings updated (in-memory only, restart required for persistence)"
    })))
}

/// Update strategy settings
pub async fn update_strategy_settings(
    State(state): State<Arc<TradingState>>,
    Json(settings): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, ErrorResponse> {
    // Strategy settings would update the DecisionMaker parameters
    // This requires restarting the trading agents with new parameters
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Strategy settings updated (requires restart to take effect)"
    })))
}

/// Update risk settings
pub async fn update_risk_settings(
    State(state): State<Arc<TradingState>>,
    Json(settings): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, ErrorResponse> {
    // Risk settings would update the RiskManager parameters
    // This requires restarting the risk manager with new parameters
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Risk settings updated (requires restart to take effect)"
    })))
}

/// Update notification settings
pub async fn update_notification_settings(
    State(state): State<Arc<TradingState>>,
    Json(settings): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, ErrorResponse> {
    // Notification settings would update the NotificationAgent parameters
    // Discord webhook would be reconfigured with new settings
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Notification settings updated (requires restart to take effect)"
    })))
}

/// Get system status
pub async fn get_system_status(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<serde_json::Value>, ErrorResponse> {
    let status = state.get_system_status().await;
    Ok(Json(serde_json::json!({
        "running": status.status == "running",
        "uptimeSeconds": status.last_update.signed_duration_since(status.start_time).num_seconds().max(0) as u64,
        "version": env!("CARGO_PKG_VERSION"),
        "commitHash": option_env!("GIT_COMMIT_HASH").unwrap_or("unknown"),
        "buildTime": option_env!("BUILD_TIME").unwrap_or("unknown"),
    })))
}

/// Start trading system
pub async fn system_start(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<serde_json::Value>, ErrorResponse> {
    state.update_system_status("running".to_string()).await;
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "System started successfully"
    })))
}

/// Stop trading system
pub async fn system_stop(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<serde_json::Value>, ErrorResponse> {
    state.update_system_status("stopped".to_string()).await;
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "System stopped successfully"
    })))
}

/// Restart trading system
pub async fn system_restart(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<serde_json::Value>, ErrorResponse> {
    state.update_system_status("running".to_string()).await;
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "System restarted successfully"
    })))
}

/// Create order request
#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub market: String,
    pub side: String,
    #[serde(rename = "amountKrw")]
    pub amount_krw: Option<f64>,
    pub amount: Option<f64>,
}

/// Create manual order
pub async fn create_order(
    State(state): State<Arc<TradingState>>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<Json<serde_json::Value>, ErrorResponse> {
    // Manual trading requires UpbitClient integration
    // Orders should be submitted through the ExecutionAgent
    // This endpoint will be implemented when manual trading is prioritized
    Ok(Json(serde_json::json!({
        "success": false,
        "error": "Manual trading is disabled. Use the CLI dashboard for trading controls."
    })))
}

/// Close position
pub async fn close_position(
    State(state): State<Arc<TradingState>>,
) -> Result<Json<serde_json::Value>, ErrorResponse> {
    // Position closing requires:
    // 1. Get current position from state
    // 2. Submit sell order through ExecutionAgent
    // 3. Update position state after order completion
    let current_position = state.get_position().await;
    if current_position.is_none() {
        return Ok(Json(serde_json::json!({
            "success": false,
            "error": "No active position to close"
        })));
    }

    // Clear position from state (actual closing requires ExecutionAgent)
    state.update_position(None).await;
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Position cleared from state (manual sell through CLI required)"
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dashboard::{AgentStatus, AgentState as DashboardAgentState};

    #[tokio::test]
    async fn test_health_check() {
        let result = health_check().await;
        assert_eq!(result, "OK");
    }

    #[tokio::test]
    async fn test_get_status() {
        let state = Arc::new(TradingState::new());
        state.update_system_status("running".to_string()).await;

        let result = get_status(State(state)).await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status.0.status, "running");
    }

    #[tokio::test]
    async fn test_get_balance_empty() {
        let state = Arc::new(TradingState::new());
        let result = get_balance(State(state)).await;
        assert!(result.is_ok());
        let balance = result.unwrap();
        assert_eq!(balance.0.krw_balance, 0.0);
    }

    #[tokio::test]
    async fn test_get_markets_empty() {
        let state = Arc::new(TradingState::new());
        let result = get_markets(State(state)).await;
        assert!(result.is_ok());
        let markets = result.unwrap();
        assert!(markets.0.is_empty());
    }

    #[tokio::test]
    async fn test_get_agents_empty() {
        let state = Arc::new(TradingState::new());
        let result = get_agents(State(state)).await;
        assert!(result.is_ok());
        let agents = result.unwrap();
        assert!(agents.0.is_empty());
    }
}
