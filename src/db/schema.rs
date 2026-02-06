//! Database row models for SQLx queries

use crate::types::{Candle, Position, PriceTick};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 포지션 DB 행
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PositionRow {
    pub id: String,
    pub market: String,
    pub entry_price: f64,
    pub amount: f64,
    pub entry_time: i64,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub exit_price: Option<f64>,
    pub exit_time: Option<i64>,
    pub pnl: Option<f64>,
    pub pnl_rate: Option<f64>,
    pub status: String,
}

impl PositionRow {
    pub fn to_position(&self) -> Position {
        Position {
            market: self.market.clone(),
            entry_price: self.entry_price,
            amount: self.amount,
            entry_time: DateTime::from_timestamp_millis(self.entry_time)
                .unwrap_or_else(|| Utc::now()),
            stop_loss: self.stop_loss,
            take_profit: self.take_profit,
        }
    }
}

/// 가격 틱 DB 행
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PriceTickRow {
    pub id: i64,
    pub market: String,
    pub timestamp: i64,
    pub trade_price: f64,
    pub change_rate: f64,
    pub volume: f64,
    pub trade_amount: f64,
}

impl PriceTickRow {
    pub fn to_price_tick(&self) -> PriceTick {
        PriceTick {
            market: self.market.clone(),
            timestamp: self.timestamp,
            trade_price: self.trade_price,
            change_rate: self.change_rate,
            volume: self.volume,
            trade_amount: self.trade_amount,
        }
    }
}

/// 캔들 DB 행
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CandleRow {
    pub id: i64,
    pub market: String,
    pub timestamp: i64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
    pub volume: f64,
}

impl CandleRow {
    pub fn to_candle(&self) -> Candle {
        Candle {
            market: self.market.clone(),
            timestamp: DateTime::from_timestamp_millis(self.timestamp)
                .unwrap_or_else(|| Utc::now()),
            open_price: self.open_price,
            high_price: self.high_price,
            low_price: self.low_price,
            close_price: self.close_price,
            volume: self.volume,
        }
    }
}

/// 주문 DB 행
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderRow {
    pub id: String,
    pub market: String,
    pub side: String,
    pub price: f64,
    pub volume: f64,
    pub status: String,
    pub created_at: i64,
    pub executed_volume: f64,
    pub executed_amount: f64,
    pub error: Option<String>,
}
