//! Database models for complex queries

use serde::{Deserialize, Serialize};

/// 거래 통계
#[derive(Debug, Serialize, Deserialize)]
pub struct TradingStats {
    pub total_trades: i64,
    pub winning_trades: i64,
    pub losing_trades: i64,
    pub total_pnl: f64,
    pub win_rate: f64,
}

/// 일일 거래 요약
#[derive(Debug, Serialize, Deserialize)]
pub struct DailySummary {
    pub date: String,
    pub trades: i64,
    pub pnl: f64,
}
