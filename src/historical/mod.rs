//! Historical Data Module
//!
//! 과거 시세 데이터 조회 및 캐싱

pub mod fetcher;
pub mod cache;

pub use fetcher::HistoricalDataFetcher;
pub use cache::HistoricalCache;

use crate::error::{Result, TradingError};
use crate::types::Candle;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 캔들 조회 파라미터
#[derive(Debug, Clone)]
pub struct FetchParams {
    /// 마켓 코드 (예: "KRW-BTC")
    pub market: String,
    /// 요청할 캔들 수
    pub count: usize,
    /// 조회 종료 시간 (None이면 현재 시간)
    pub to: Option<DateTime<Utc>>,
}

impl FetchParams {
    pub fn new(market: String, count: usize) -> Self {
        Self {
            market,
            count,
            to: None,
        }
    }

    pub fn with_to(mut self, to: DateTime<Utc>) -> Self {
        self.to = Some(to);
        self
    }
}

/// 날짜 범위 조회 파라미터
#[derive(Debug, Clone)]
pub struct DateRangeParams {
    /// 마켓 코드
    pub market: String,
    /// 시작 시간
    pub from: DateTime<Utc>,
    /// 종료 시간
    pub to: DateTime<Utc>,
}

impl DateRangeParams {
    pub fn new(market: String, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        Self { market, from, to }
    }
}

/// 캔들 유닛 (분 단위)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandleUnit {
    OneMinute = 1,
    ThreeMinutes = 3,
    FiveMinutes = 5,
    TenMinutes = 10,
    FifteenMinutes = 15,
    ThirtyMinutes = 30,
    SixtyMinutes = 60,
    TwoHundredFortyMinutes = 240,
}

impl CandleUnit {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn as_minutes(&self) -> u64 {
        *self as u64
    }
}

impl Default for CandleUnit {
    fn default() -> Self {
        Self::OneMinute
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_params() {
        let params = FetchParams::new("KRW-BTC".to_string(), 100);
        assert_eq!(params.market, "KRW-BTC");
        assert_eq!(params.count, 100);
        assert!(params.to.is_none());

        let params_with_to = params.with_to(Utc::now());
        assert!(params_with_to.to.is_some());
    }

    #[test]
    fn test_candle_unit() {
        assert_eq!(CandleUnit::OneMinute.as_u32(), 1);
        assert_eq!(CandleUnit::FiveMinutes.as_minutes(), 5);
        assert_eq!(CandleUnit::SixtyMinutes.as_minutes(), 60);
    }

    #[test]
    fn test_date_range_params() {
        let from = Utc::now() - chrono::Duration::hours(24);
        let to = Utc::now();
        let params = DateRangeParams::new("KRW-BTC".to_string(), from, to);
        assert_eq!(params.market, "KRW-BTC");
    }
}
