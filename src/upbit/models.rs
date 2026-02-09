//! Upbit API response models

use crate::types::{OrderSide, OrderStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 마켓 코드 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketInfo {
    pub market: String,
    pub korean_name: String,
    pub english_name: String,
    pub market_warning: Option<String>,
}

/// 캔들 응답 (Upbit API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleResponse {
    pub market: String,
    pub candle_date_time_utc: String,
    pub candle_date_time_kst: String,
    pub opening_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub trade_price: f64,
    pub timestamp: i64,
    pub candle_acc_trade_price: f64,
    pub candle_acc_trade_volume: f64,
    pub unit: i32,
}

impl From<CandleResponse> for crate::types::Candle {
    fn from(c: CandleResponse) -> Self {
        let timestamp = DateTime::parse_from_rfc3339(&c.candle_date_time_utc)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Self {
            market: c.market,
            timestamp,
            open_price: c.opening_price,
            high_price: c.high_price,
            low_price: c.low_price,
            close_price: c.trade_price,
            volume: c.candle_acc_trade_volume,
        }
    }
}

/// 티커 응답 (Upbit API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerResponse {
    pub market: String,
    pub trade_date: String,
    pub trade_time: String,
    pub trade_date_kst: String,
    pub trade_time_kst: String,
    pub opening_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub trade_price: f64,
    pub prev_closing_price: f64,
    pub change: String,
    pub change_price: f64,
    pub change_rate: f64,
    pub signed_change_price: f64,
    pub signed_change_rate: f64,
    pub trade_volume: f64,
    pub acc_trade_price: f64,
    pub acc_trade_price_24h: f64,
    pub acc_trade_volume: f64,
    pub acc_trade_volume_24h: f64,
    pub highest_52_week_price: f64,
    pub highest_52_week_date: String,
    pub lowest_52_week_price: f64,
    pub lowest_52_week_date: String,
    pub timestamp: i64,
}

impl From<TickerResponse> for crate::types::PriceTick {
    fn from(t: TickerResponse) -> Self {
        Self {
            market: t.market,
            timestamp: t.timestamp,
            trade_price: t.trade_price,
            change_rate: t.signed_change_rate,
            volume: t.trade_volume,
            trade_amount: t.acc_trade_price,
        }
    }
}

/// 계정 정보 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub currency: String,
    pub balance: f64,
    pub locked: f64,
    pub avg_buy_price: f64,
    pub avg_buy_price_modified: bool,
    pub unit_currency: String,
}

impl From<AccountInfo> for crate::types::Balance {
    fn from(a: AccountInfo) -> Self {
        Self {
            currency: a.currency,
            balance: a.balance,
            locked: a.locked,
            available: a.balance - a.locked,
            avg_buy_price: a.avg_buy_price,
        }
    }
}

/// 주문 요청
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub market: String,
    pub side: String, // "bid" or "ask"
    pub volume: Option<String>, // 수량 (지정가)
    pub price: Option<String>, // 가격 (지정가)
    pub ord_type: String, // "limit"
}

/// 주문 응답
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub uuid: String,
    pub side: String,
    pub ord_type: String,
    pub price: f64,
    pub state: String,
    pub market: String,
    pub created_at: String,
    pub volume: f64,
    pub remaining_volume: f64,
    pub reserved_fee: f64,
    pub remaining_fee: f64,
    pub paid_fee: f64,
    pub locked: f64,
    pub executed_volume: f64,
    pub executed_amount: f64,
}

impl OrderResponse {
    pub fn to_order(&self) -> crate::types::Order {
        let side = match self.side.as_str() {
            "bid" => OrderSide::Bid,
            "ask" => OrderSide::Ask,
            _ => panic!("Invalid order side: {}", self.side),
        };

        let status = match self.state.as_str() {
            "wait" => OrderStatus::Waiting,
            "done" => OrderStatus::Executed,
            "cancel" => OrderStatus::Canceled,
            _ => OrderStatus::Failed,
        };

        let created_at = DateTime::parse_from_rfc3339(&self.created_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        crate::types::Order {
            id: self.uuid.clone(),
            market: self.market.clone(),
            side,
            price: self.price,
            volume: self.volume,
            status,
            created_at,
            executed_volume: self.executed_volume,
            executed_amount: self.executed_amount,
        }
    }
}

/// WebSocket 메시지 타입
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "trade")]
    Trade(TradeMessage),
    #[serde(rename = "ticker")]
    Ticker(TickerMessage),
}

/// WebSocket 체결 메시지
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeMessage {
    pub ty: String, // "trade"
    pub code: String,
    pub timestamp: i64,
    pub trade_price: f64,
    pub change_price: f64,
    pub change_rate: f64,
    pub trade_volume: f64,
    pub ask_bid: String,
}

impl From<TradeMessage> for crate::types::PriceTick {
    fn from(t: TradeMessage) -> Self {
        Self {
            market: t.code,
            timestamp: t.timestamp,
            trade_price: t.trade_price,
            change_rate: t.change_rate,
            volume: t.trade_volume,
            trade_amount: t.trade_price * t.trade_volume,
        }
    }
}

/// WebSocket 티커 메시지
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerMessage {
    pub ty: String, // "ticker"
    pub code: String,
    pub opening_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub trade_price: f64,
    pub prev_closing_price: f64,
    pub change: String,
    pub change_price: f64,
    pub change_rate: f64,
    pub trade_volume: f64,
    pub acc_trade_volume: f64,
    pub acc_trade_price: f64,
    pub timestamp: i64,
}

impl From<TickerMessage> for crate::types::PriceTick {
    fn from(t: TickerMessage) -> Self {
        Self {
            market: t.code,
            timestamp: t.timestamp,
            trade_price: t.trade_price,
            change_rate: t.change_rate,
            volume: t.trade_volume,
            trade_amount: t.acc_trade_price,
        }
    }
}

/// WebSocket 구독 요청
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsSubscription {
    #[serde(rename = "ticket")]
    pub ticket: String,
    #[serde(rename = "type")]
    pub subscription_type: String,
    pub codes: Vec<String>,
}

impl WsSubscription {
    pub fn new(ticket: String, subscription_type: &str, codes: Vec<String>) -> Self {
        Self {
            ticket,
            subscription_type: subscription_type.to_string(),
            codes,
        }
    }

    pub fn trade(ticket: String, codes: Vec<String>) -> Self {
        Self::new(ticket, "trade", codes)
    }

    pub fn ticker(ticket: String, codes: Vec<String>) -> Self {
        Self::new(ticket, "ticker", codes)
    }
}
