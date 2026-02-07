//! # AutoCoin - Upbit Automated Trading Agent System
//!
//! Multi-agent trading system for Upbit cryptocurrency exchange.
//!
//! ## Architecture
//!
//! The system consists of six main agents:
//! - **Market Monitor**: Real-time price monitoring via WebSocket
//! - **Signal Detector**: Momentum and surge detection
//! - **Decision Maker**: Trading decision logic
//! - **Execution Agent**: Order execution with rate limiting
//! - **Risk Manager**: PnL calculation and stop-loss/take-profit
//! - **Notification Agent**: Discord alerts
//!
//! ## License
//!
//! MIT License

// Public modules
pub mod config;
pub mod dashboard;
pub mod error;
pub mod execution;
pub mod historical;
pub mod indicators;
pub mod types;

// Private modules
mod db;
mod agents;
mod upbit;
mod discord;
mod strategy;

// Re-exports for convenience
pub use error::TradingError;
pub use indicators::{Indicator, IndicatorCache, IndicatorValue};
pub use strategy::{Strategy, StrategyFactory};
pub use strategy::momentum::MomentumStrategy;
pub use types::trading::*;

