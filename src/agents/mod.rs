//! Multi-agent trading system
//!
//! 6개의 독립적인 에이전트로 구성된 트레이딩 시스템입니다.

pub use market_monitor::MarketMonitor;
pub use signal_detector::SignalDetector;
pub use decision_maker::DecisionMaker;
pub use executor::ExecutionAgent;
pub use risk_manager::RiskManager;
pub use notification::NotificationAgent;

mod market_monitor;
mod signal_detector;
mod decision_maker;
mod executor;
mod risk_manager;
mod notification;
