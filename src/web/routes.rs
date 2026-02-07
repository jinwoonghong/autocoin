//! API routes configuration
//!
//! Defines all HTTP routes for the web dashboard.

use super::handlers;
use super::websocket::websocket_handler;
use axum::{
    routing::{get, get_service},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use super::state::TradingState;

/// Create the main API router
///
/// Configures all routes with CORS and static file serving.
pub fn create_router(state: Arc<TradingState>) -> Router {
    // CORS layer configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // API routes
    let api_routes = Router::new()
        .route("/status", get(handlers::get_status))
        .route("/position", get(handlers::get_position))
        .route("/balance", get(handlers::get_balance))
        .route("/trades", get(handlers::get_trades))
        .route("/markets", get(handlers::get_markets))
        .route("/agents", get(handlers::get_agents))
        .route("/notifications", get(handlers::get_notifications))
        .route("/health", get(handlers::health_check))
        // Dashboard aggregation
        .route("/dashboard", get(handlers::get_dashboard))
        // Backtest endpoints
        .route("/backtest", post(handlers::run_backtest))
        // Settings endpoints
        .route("/settings", get(handlers::get_settings).put(handlers::update_settings))
        .route("/settings/strategy", put(handlers::update_strategy_settings))
        .route("/settings/risk", put(handlers::update_risk_settings))
        .route("/settings/notifications", put(handlers::update_notification_settings))
        // System control endpoints
        .route("/system/status", get(handlers::get_system_status))
        .route("/system/start", post(handlers::system_start))
        .route("/system/stop", post(handlers::system_stop))
        .route("/system/restart", post(handlers::system_restart))
        // Manual trading endpoints
        .route("/orders", post(handlers::create_order))
        .route("/position", delete(handlers::close_position));

    // WebSocket route
    let ws_routes = Router::new().route("/ws", get(websocket_handler));

    // Combine all routes
    Router::new()
        .nest("/api", api_routes)
        .nest("/ws", ws_routes)
        .route("/health", get(handlers::health_check))
        .nest_service("/assets", get_service(ServeDir::new("assets")))
        .fallback(handlers::not_found)
        .layer(cors)
        .with_state(state)
}

/// Create router with custom static directory
///
/// Allows specifying a custom directory for static assets.
pub fn create_router_with_static(state: Arc<TradingState>, static_dir: &str) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        .route("/status", get(handlers::get_status))
        .route("/position", get(handlers::get_position))
        .route("/balance", get(handlers::get_balance))
        .route("/trades", get(handlers::get_trades))
        .route("/markets", get(handlers::get_markets))
        .route("/agents", get(handlers::get_agents))
        .route("/notifications", get(handlers::get_notifications))
        .route("/health", get(handlers::health_check))
        // Dashboard aggregation
        .route("/dashboard", get(handlers::get_dashboard))
        // Backtest endpoints
        .route("/backtest", post(handlers::run_backtest))
        // Settings endpoints
        .route("/settings", get(handlers::get_settings).put(handlers::update_settings))
        .route("/settings/strategy", put(handlers::update_strategy_settings))
        .route("/settings/risk", put(handlers::update_risk_settings))
        .route("/settings/notifications", put(handlers::update_notification_settings))
        // System control endpoints
        .route("/system/status", get(handlers::get_system_status))
        .route("/system/start", post(handlers::system_start))
        .route("/system/stop", post(handlers::system_stop))
        .route("/system/restart", post(handlers::system_restart))
        // Manual trading endpoints
        .route("/orders", post(handlers::create_order))
        .route("/position", delete(handlers::close_position));

    Router::new()
        .nest("/api", api_routes)
        .route("/ws", get(websocket_handler))
        .route("/health", get(handlers::health_check))
        .nest_service("/assets", get_service(ServeDir::new(static_dir)))
        .fallback(handlers::not_found)
        .layer(cors)
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_router() {
        let state = Arc::new(TradingState::new());
        let router = create_router(state);
        // Router creation should not panic
        assert!(true);
    }

    #[test]
    fn test_create_router_with_static() {
        let state = Arc::new(TradingState::new());
        let router = create_router_with_static(state, "public");
        // Router creation should not panic
        assert!(true);
    }
}
