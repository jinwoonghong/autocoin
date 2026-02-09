//! Technical Indicators Module
//!
//! 기술적 지표를 위한 공통 트레이트 및 구현체들.

use crate::error::{Result, TradingError};
use crate::types::Candle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 지표 값
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndicatorValue {
    pub timestamp: i64,
    pub value: f64,
    pub metadata: HashMap<String, String>,
}

impl IndicatorValue {
    /// 새로운 지표 값 생성
    pub fn new(timestamp: i64, value: f64) -> Self {
        Self {
            timestamp,
            value,
            metadata: HashMap::new(),
        }
    }

    /// 메타데이터 추가
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// 기술적 지표 트레이트
pub trait Indicator: Send + Sync {
    /// 캔들 데이터로 지표 업데이트
    fn update(&mut self, candle: &Candle) -> Option<IndicatorValue>;

    /// 현재 지표 값 조회
    fn value(&self) -> Option<IndicatorValue>;

    /// 지표 초기화
    fn reset(&mut self);

    /// 지표 이름
    fn name(&self) -> &str;

    /// 필요한 최소 캔들 개수
    fn min_periods(&self) -> usize;
}

/// 지표 파라미터 유효성 검증 (REQ-301)
pub fn validate_indicator_params(period: usize, min_value: f64) -> Result<()> {
    if period == 0 {
        return Err(TradingError::InvalidParameter(
            "Indicator period must be greater than 0".to_string(),
        ));
    }
    if min_value <= 0.0 {
        return Err(TradingError::InvalidParameter(
            "Base value must be greater than 0".to_string(),
        ));
    }
    Ok(())
}

/// 지표 캐시 관리자
pub struct IndicatorCache {
    /// 메모리 캐시 (market -> (indicator_type -> (timestamp -> value)))
    memory_cache: HashMap<String, HashMap<String, HashMap<i64, IndicatorValue>>>,
    /// 최대 캐시 크기 (지표당)
    max_cache_size: usize,
}

impl IndicatorCache {
    /// 새로운 캐시 관리자 생성
    pub fn new(max_cache_size: usize) -> Self {
        Self {
            memory_cache: HashMap::new(),
            max_cache_size,
        }
    }

    /// 캐시된 값 조회 (REQ-317)
    pub fn get(&self, market: &str, indicator_type: &str, timestamp: i64) -> Option<IndicatorValue> {
        self.memory_cache
            .get(market)?
            .get(indicator_type)?
            .get(&timestamp)
            .cloned()
    }

    /// 캐시에 값 저장 (REQ-304)
    pub fn put(&mut self, market: String, indicator_type: String, value: IndicatorValue) {
        let market_cache = self
            .memory_cache
            .entry(market)
            .or_insert_with(HashMap::new);
        let indicator_cache = market_cache
            .entry(indicator_type)
            .or_insert_with(HashMap::new);

        // LRU eviction: 캐시 크기 초과 시 오래된 항목 제거
        if indicator_cache.len() >= self.max_cache_size {
            // Clone the key to avoid borrow checker issue
            let oldest_key = indicator_cache.keys().min().cloned();
            if let Some(key) = oldest_key {
                indicator_cache.remove(&key);
            }
        }

        indicator_cache.insert(value.timestamp, value);
    }

    /// 캐시 비우기
    pub fn clear(&mut self) {
        self.memory_cache.clear();
    }

    /// 특정 마켓 캐시 비우기
    pub fn clear_market(&mut self, market: &str) {
        self.memory_cache.remove(market);
    }

    /// 특정 지표 캐시 비우기
    pub fn clear_indicator(&mut self, market: &str, indicator_type: &str) {
        if let Some(market_cache) = self.memory_cache.get_mut(market) {
            market_cache.remove(indicator_type);
        }
    }

    /// 캐시 크기 조회
    pub fn size(&self) -> usize {
        self.memory_cache
            .values()
            .map(|m| m.values().map(|i| i.len()).sum::<usize>())
            .sum()
    }
}

impl Default for IndicatorCache {
    fn default() -> Self {
        Self::new(1000) // 기본 최대 1000개 캐시
    }
}

// 서브모듈
pub mod bollinger;
pub mod moving_average;
pub mod rsi;
pub mod macd;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Candle;
    use chrono::Utc;

    struct TestIndicator {
        values: Vec<f64>,
        period: usize,
    }

    impl TestIndicator {
        fn new(period: usize) -> Self {
            Self {
                values: Vec::new(),
                period,
            }
        }
    }

    impl Indicator for TestIndicator {
        fn update(&mut self, candle: &Candle) -> Option<IndicatorValue> {
            self.values.push(candle.close_price);
            if self.values.len() >= self.period {
                let sum: f64 = self.values.iter().sum();
                let avg = sum / self.values.len() as f64;
                Some(IndicatorValue::new(candle.timestamp.timestamp_millis(), avg))
            } else {
                None
            }
        }

        fn value(&self) -> Option<IndicatorValue> {
            if self.values.len() >= self.period {
                let sum: f64 = self.values.iter().sum();
                let avg = sum / self.values.len() as f64;
                let timestamp = Utc::now().timestamp_millis();
                Some(IndicatorValue::new(timestamp, avg))
            } else {
                None
            }
        }

        fn reset(&mut self) {
            self.values.clear();
        }

        fn name(&self) -> &str {
            "test_indicator"
        }

        fn min_periods(&self) -> usize {
            self.period
        }
    }

    #[test]
    fn test_indicator_value_creation() {
        let value = IndicatorValue::new(1234567890, 100.5);
        assert_eq!(value.timestamp, 1234567890);
        assert_eq!(value.value, 100.5);
        assert!(value.metadata.is_empty());
    }

    #[test]
    fn test_indicator_value_with_metadata() {
        let value = IndicatorValue::new(1234567890, 100.5)
            .with_metadata("period".to_string(), "14".to_string())
            .with_metadata("type".to_string(), "RSI".to_string());

        assert_eq!(value.metadata.len(), 2);
        assert_eq!(value.metadata.get("period"), Some(&"14".to_string()));
        assert_eq!(value.metadata.get("type"), Some(&"RSI".to_string()));
    }

    #[test]
    fn test_validate_indicator_params_valid() {
        assert!(validate_indicator_params(14, 50.0).is_ok());
        assert!(validate_indicator_params(1, 0.01).is_ok());
    }

    #[test]
    fn test_validate_indicator_params_invalid_period() {
        let result = validate_indicator_params(0, 50.0);
        assert!(result.is_err());
        assert!(matches!(result, Err(TradingError::InvalidParameter(_))));
    }

    #[test]
    fn test_validate_indicator_params_invalid_value() {
        let result = validate_indicator_params(14, 0.0);
        assert!(result.is_err());
        assert!(matches!(result, Err(TradingError::InvalidParameter(_))));

        let result = validate_indicator_params(14, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_indicator_cache_put_and_get() {
        let mut cache = IndicatorCache::new(10);
        let market = "KRW-BTC".to_string();
        let indicator_type = "SMA".to_string();
        let value = IndicatorValue::new(1234567890, 50000.0);

        cache.put(market.clone(), indicator_type.clone(), value.clone());

        let retrieved = cache.get(&market, &indicator_type, 1234567890);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().value, 50000.0);
    }

    #[test]
    fn test_indicator_cache_miss() {
        let cache = IndicatorCache::new(10);
        let result = cache.get("KRW-BTC", "SMA", 1234567890);
        assert!(result.is_none());
    }

    #[test]
    fn test_indicator_cache_lru_eviction() {
        let mut cache = IndicatorCache::new(3); // 최대 3개
        let market = "KRW-BTC".to_string();
        let indicator_type = "SMA".to_string();

        // 3개 추가
        for i in 1..=3 {
            let value = IndicatorValue::new(i, i as f64 * 1000.0);
            cache.put(market.clone(), indicator_type.clone(), value);
        }

        assert_eq!(cache.size(), 3);

        // 4번째 추가 (가장 오래된 것 제거됨)
        let value = IndicatorValue::new(4, 4000.0);
        cache.put(market.clone(), indicator_type.clone(), value);

        assert_eq!(cache.size(), 3);
        assert!(cache.get(&market, &indicator_type, 1).is_none()); // 가장 오래된 것 제거됨
        assert!(cache.get(&market, &indicator_type, 4).is_some()); // 새로운 것 존재
    }

    #[test]
    fn test_indicator_cache_clear() {
        let mut cache = IndicatorCache::new(10);
        let market = "KRW-BTC".to_string();
        let indicator_type = "SMA".to_string();
        let value = IndicatorValue::new(1234567890, 50000.0);

        cache.put(market.clone(), indicator_type.clone(), value);
        assert_eq!(cache.size(), 1);

        cache.clear();
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_indicator_cache_clear_market() {
        let mut cache = IndicatorCache::new(10);
        let market1 = "KRW-BTC".to_string();
        let market2 = "KRW-ETH".to_string();
        let indicator_type = "SMA".to_string();

        cache.put(market1.clone(), indicator_type.clone(), IndicatorValue::new(1, 100.0));
        cache.put(market2.clone(), indicator_type.clone(), IndicatorValue::new(2, 200.0));

        assert_eq!(cache.size(), 2);

        cache.clear_market(&market1);
        assert_eq!(cache.size(), 1);
        assert!(cache.get(&market1, &indicator_type, 1).is_none());
        assert!(cache.get(&market2, &indicator_type, 2).is_some());
    }

    #[test]
    fn test_indicator_cache_clear_indicator() {
        let mut cache = IndicatorCache::new(10);
        let market = "KRW-BTC".to_string();
        let indicator_type1 = "SMA".to_string();
        let indicator_type2 = "RSI".to_string();

        cache.put(
            market.clone(),
            indicator_type1.clone(),
            IndicatorValue::new(1, 100.0),
        );
        cache.put(
            market.clone(),
            indicator_type2.clone(),
            IndicatorValue::new(2, 50.0),
        );

        assert_eq!(cache.size(), 2);

        cache.clear_indicator(&market, &indicator_type1);
        assert_eq!(cache.size(), 1);
        assert!(cache.get(&market, &indicator_type1, 1).is_none());
        assert!(cache.get(&market, &indicator_type2, 2).is_some());
    }

    #[test]
    fn test_indicator_trait_impl() {
        let mut indicator = TestIndicator::new(3);
        assert_eq!(indicator.name(), "test_indicator");
        assert_eq!(indicator.min_periods(), 3);

        // 충분하지 않은 데이터로 업데이트
        let candle = Candle::new(
            "KRW-BTC".to_string(),
            Utc::now(),
            50000.0,
            51000.0,
            49000.0,
            50500.0,
            1.0,
        );
        let result = indicator.update(&candle);
        assert!(result.is_none()); // 아직 값 없음
        assert!(indicator.value().is_none());

        // 충분한 데이터로 업데이트
        indicator.update(&candle);
        indicator.update(&candle);
        let result = indicator.update(&candle);
        assert!(result.is_some());

        // reset 테스트
        indicator.reset();
        assert!(indicator.value().is_none());
    }

    #[test]
    fn test_insufficient_data_protection() {
        let indicator = TestIndicator::new(10);
        let candle = Candle::new(
            "KRW-BTC".to_string(),
            Utc::now(),
            50000.0,
            51000.0,
            49000.0,
            50500.0,
            1.0,
        );

        // 불충분한 데이터 (REQ-319)
        assert!(indicator.value().is_none());
        assert_eq!(indicator.min_periods(), 10);
    }
}
