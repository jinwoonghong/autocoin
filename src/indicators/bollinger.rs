//! Bollinger Bands Indicator
//!
//! Bollinger Bands 지표 구현 - 변동성 기반 트레이딩 밴드

use crate::error::{Result, TradingError};
use crate::indicators::{validate_indicator_params, Indicator, IndicatorValue};
use crate::types::Candle;
use std::collections::VecDeque;

/// Bollinger Bands 값
#[derive(Debug, Clone, PartialEq)]
pub struct BollingerValue {
    /// 상단 밴드 (Upper Band)
    pub upper: f64,
    /// 중간 밴드 (Middle Band - SMA)
    pub middle: f64,
    /// 하단 밴드 (Lower Band)
    pub lower: f64,
    /// 밴드폭 (Bandwidth) - (upper - lower) / middle
    pub bandwidth: f64,
}

impl BollingerValue {
    /// 새로운 Bollinger Bands 값 생성
    pub fn new(upper: f64, middle: f64, lower: f64) -> Self {
        let bandwidth = if middle > 0.0 {
            (upper - lower) / middle
        } else {
            0.0
        };
        Self {
            upper,
            middle,
            lower,
            bandwidth,
        }
    }

    /// 현재 가격이 밴드 내부에 있는지 확인
    pub fn contains(&self, price: f64) -> bool {
        price >= self.lower && price <= self.upper
    }

    /// 상단 밴드로부터의 거리 (%)
    pub fn distance_from_upper(&self, price: f64) -> f64 {
        if self.upper > self.lower {
            ((self.upper - price) / (self.upper - self.lower)) * 100.0
        } else {
            0.0
        }
    }

    /// 하단 밴드로부터의 거리 (%)
    pub fn distance_from_lower(&self, price: f64) -> f64 {
        if self.upper > self.lower {
            ((price - self.lower) / (self.upper - self.lower)) * 100.0
        } else {
            0.0
        }
    }

    /// %B 계산 (가격이 밴드 내에서 어디에 위치하는지)
    pub fn percent_b(&self, price: f64) -> f64 {
        if self.upper == self.lower {
            0.5
        } else {
            (price - self.lower) / (self.upper - self.lower)
        }
    }
}

/// Bollinger Bands 지표
pub struct BollingerBands {
    /// 기간 (기본값 20)
    period: usize,
    /// 표준편차 배수 (기본값 2.0)
    std_dev: f64,
    /// 가격 저장소
    prices: VecDeque<f64>,
    /// 현재 밴드 값
    current_value: Option<BollingerValue>,
}

impl BollingerBands {
    /// 새로운 Bollinger Bands 지표 생성
    pub fn new(period: usize, std_dev: f64) -> Result<Self> {
        validate_indicator_params(period, std_dev)?;
        if std_dev <= 0.0 {
            return Err(TradingError::InvalidParameter(
                "Standard deviation multiplier must be greater than 0".to_string(),
            ));
        }
        Ok(Self {
            period,
            std_dev,
            prices: VecDeque::with_capacity(period),
            current_value: None,
        })
    }

    /// 기본 설정으로 Bollinger Bands 생성 (period=20, std_dev=2.0)
    pub fn default_params() -> Result<Self> {
        Self::new(20, 2.0)
    }

    /// 현재 Bollinger Bands 값 반환
    pub fn current_value(&self) -> Option<BollingerValue> {
        self.current_value.clone()
    }

    /// 상단 밴드 터치 감지 (REQ-309)
    ///
    /// 가격이 상단 밴드에 근접했는지 확인 (1% 이내)
    pub fn is_touching_upper(&self, price: f64) -> bool {
        if let Some(ref value) = self.current_value {
            let threshold = value.upper * 0.01; // 1% threshold
            price >= value.upper - threshold
        } else {
            false
        }
    }

    /// 하단 밸드 터치 감지 (REQ-310)
    ///
    /// 가격이 하단 밴드에 근접했는지 확인 (1% 이내)
    pub fn is_touching_lower(&self, price: f64) -> bool {
        if let Some(ref value) = self.current_value {
            let threshold = value.lower * 0.01; // 1% threshold
            price <= value.lower + threshold
        } else {
            false
        }
    }

    /// 스퀴즈 감지 (변동성 축소)
    ///
    /// 밴드폭이 0.1 미만인 경우 스퀴즈로 간주
    pub fn is_squeeze(&self) -> bool {
        if let Some(ref value) = self.current_value {
            value.bandwidth < 0.1
        } else {
            false
        }
    }

    /// 사용자 정의 임계값으로 스퀴즈 감지
    pub fn is_squeeze_with_threshold(&self, threshold: f64) -> bool {
        if let Some(ref value) = self.current_value {
            value.bandwidth < threshold
        } else {
            false
        }
    }

    /// 확장 감지 (변동성 확대)
    ///
    /// 밴드폭이 0.4 이상인 경우 확장으로 간주
    pub fn is_expanding(&self) -> bool {
        if let Some(ref value) = self.current_value {
            value.bandwidth >= 0.4
        } else {
            false
        }
    }

    /// 밴드 돌파 상단 확인 (가격이 상단 밴드 위)
    pub fn is_above_upper(&self, price: f64) -> bool {
        if let Some(ref value) = self.current_value {
            price > value.upper
        } else {
            false
        }
    }

    /// 밴드 돌파 하단 확인 (가격이 하단 밴드 아래)
    pub fn is_below_lower(&self, price: f64) -> bool {
        if let Some(ref value) = self.current_value {
            price < value.lower
        } else {
            false
        }
    }

    /// SMA 및 표준편차 계산 (내부 헬퍼 함수)
    fn calculate_bands(&mut self) -> Option<BollingerValue> {
        if self.prices.len() < self.period {
            return None;
        }

        // SMA 계산
        let sum: f64 = self.prices.iter().sum();
        let sma = sum / self.period as f64;

        // 표준편차 계산
        let variance: f64 = self.prices.iter().map(|&p| (p - sma).powi(2)).sum();
        let std_dev = (variance / self.period as f64).sqrt();

        // 밴드 계산
        let upper = sma + (self.std_dev * std_dev);
        let lower = sma - (self.std_dev * std_dev);

        let value = BollingerValue::new(upper, sma, lower);
        self.current_value = Some(value.clone());
        Some(value)
    }
}

impl Indicator for BollingerBands {
    fn update(&mut self, candle: &Candle) -> Option<IndicatorValue> {
        // 가격 추가
        if self.prices.len() >= self.period {
            self.prices.pop_front();
        }
        self.prices.push_back(candle.close_price);

        // 충분한 데이터가 있으면 계산 (REQ-319)
        if let Some(bollinger) = self.calculate_bands() {
            // 중간 밴드 값 반환
            Some(
                IndicatorValue::new(candle.timestamp.timestamp_millis(), bollinger.middle)
                    .with_metadata("period".to_string(), self.period.to_string())
                    .with_metadata("std_dev".to_string(), self.std_dev.to_string())
                    .with_metadata("type".to_string(), "BollingerBands".to_string())
                    .with_metadata("upper".to_string(), bollinger.upper.to_string())
                    .with_metadata("lower".to_string(), bollinger.lower.to_string())
                    .with_metadata("bandwidth".to_string(), bollinger.bandwidth.to_string()),
            )
        } else {
            None
        }
    }

    fn value(&self) -> Option<IndicatorValue> {
        self.current_value.as_ref().map(|v| {
            IndicatorValue::new(chrono::Utc::now().timestamp_millis(), v.middle)
                .with_metadata("period".to_string(), self.period.to_string())
                .with_metadata("std_dev".to_string(), self.std_dev.to_string())
                .with_metadata("type".to_string(), "BollingerBands".to_string())
                .with_metadata("upper".to_string(), v.upper.to_string())
                .with_metadata("lower".to_string(), v.lower.to_string())
                .with_metadata("bandwidth".to_string(), v.bandwidth.to_string())
        })
    }

    fn reset(&mut self) {
        self.prices.clear();
        self.current_value = None;
    }

    fn name(&self) -> &str {
        "BollingerBands"
    }

    fn min_periods(&self) -> usize {
        self.period
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_candle(close_price: f64) -> Candle {
        Candle::new(
            "KRW-BTC".to_string(),
            Utc::now(),
            close_price - 100.0,
            close_price + 100.0,
            close_price - 200.0,
            close_price,
            1.0,
        )
    }

    #[test]
    fn test_bollinger_bands_creation() {
        let bb = BollingerBands::new(20, 2.0).unwrap();
        assert_eq!(bb.period, 20);
        assert_eq!(bb.std_dev, 2.0);
        assert_eq!(bb.name(), "BollingerBands");
        assert_eq!(bb.min_periods(), 20);
        assert!(bb.value().is_none());
        assert!(bb.current_value().is_none());
    }

    #[test]
    fn test_bollinger_bands_invalid_period() {
        let result = BollingerBands::new(0, 2.0);
        assert!(result.is_err());
        assert!(matches!(result, Err(TradingError::InvalidParameter(_))));
    }

    #[test]
    fn test_bollinger_bands_invalid_std_dev() {
        let result = BollingerBands::new(20, 0.0);
        assert!(result.is_err());
        assert!(matches!(result, Err(TradingError::InvalidParameter(_))));

        let result = BollingerBands::new(20, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_bollinger_bands_default_params() {
        let bb = BollingerBands::default_params().unwrap();
        assert_eq!(bb.period, 20);
        assert_eq!(bb.std_dev, 2.0);
    }

    #[test]
    fn test_bollinger_bands_insufficient_data() {
        let mut bb = BollingerBands::new(20, 2.0).unwrap();
        let candle = create_test_candle(50000.0);

        // 데이터 부족하면 None 반환 (REQ-319)
        assert!(bb.update(&candle).is_none());
        assert!(bb.value().is_none());
        assert!(bb.current_value().is_none());
    }

    #[test]
    fn test_bollinger_bands_calculation_constant_prices() {
        let mut bb = BollingerBands::new(3, 2.0).unwrap();

        // 모든 가격이 동일: 100
        for _ in 0..3 {
            let candle = create_test_candle(100.0);
            bb.update(&candle);
        }

        let result = bb.current_value();
        assert!(result.is_some());

        let value = result.unwrap();
        // 상/하단 밴드는 모두 SMA와 같음 (표준편차 = 0)
        assert!((value.middle - 100.0).abs() < 0.01);
        assert!((value.upper - 100.0).abs() < 0.01);
        assert!((value.lower - 100.0).abs() < 0.01);
        assert_eq!(value.bandwidth, 0.0);
    }

    #[test]
    fn test_bollinger_bands_calculation_varying_prices() {
        let mut bb = BollingerBands::new(5, 2.0).unwrap();

        // 변동성 있는 가격 시계열
        let prices = vec![100.0, 102.0, 98.0, 105.0, 95.0];
        for price in prices {
            let candle = create_test_candle(price);
            bb.update(&candle);
        }

        let result = bb.current_value();
        assert!(result.is_some());

        let value = result.unwrap();
        // SMA = (100 + 102 + 98 + 105 + 95) / 5 = 100
        assert!((value.middle - 100.0).abs() < 0.01);

        // 표준편차 계산
        // 분산 = ((0)^2 + (2)^2 + (-2)^2 + (5)^2 + (-5)^2) / 5 = 58 / 5 = 11.6
        // 표준편차 = sqrt(11.6) ≈ 3.405
        // 상단 밴드 = 100 + 2 * 3.405 ≈ 106.81
        // 하단 밴드 = 100 - 2 * 3.405 ≈ 93.19
        assert!(value.upper > value.middle);
        assert!(value.lower < value.middle);
        assert!((value.upper - 106.81).abs() < 0.1);
        assert!((value.lower - 93.19).abs() < 0.1);

        // 밴드폭 = (upper - lower) / middle
        let expected_bandwidth = (value.upper - value.lower) / value.middle;
        assert!((value.bandwidth - expected_bandwidth).abs() < 0.01);
    }

    #[test]
    fn test_bollinger_value_contains() {
        let value = BollingerValue::new(110.0, 100.0, 90.0);

        assert!(value.contains(100.0)); // 중간
        assert!(value.contains(110.0)); // 상단
        assert!(value.contains(90.0));  // 하단
        assert!(!value.contains(111.0)); // 상단 초과
        assert!(!value.contains(89.0));  // 하단 미달
    }

    #[test]
    fn test_bollinger_value_percent_b() {
        let value = BollingerValue::new(110.0, 100.0, 90.0);

        // 중간 = 0.5
        assert!((value.percent_b(100.0) - 0.5).abs() < 0.01);
        // 상단 = 1.0
        assert!((value.percent_b(110.0) - 1.0).abs() < 0.01);
        // 하단 = 0.0
        assert!((value.percent_b(90.0) - 0.0).abs() < 0.01);
        // 초과 = 1.0보다 큼
        assert!(value.percent_b(115.0) > 1.0);
        // 미달 = 0.0보다 작음
        assert!(value.percent_b(85.0) < 0.0);
    }

    #[test]
    fn test_bollinger_value_distance_from_upper() {
        let value = BollingerValue::new(110.0, 100.0, 90.0);

        // 상단에서 거리 0%
        assert!((value.distance_from_upper(110.0) - 0.0).abs() < 0.01);
        // 하단에서 거리 100%
        assert!((value.distance_from_upper(90.0) - 100.0).abs() < 0.01);
        // 중간에서 거리 50%
        assert!((value.distance_from_upper(100.0) - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_bollinger_value_distance_from_lower() {
        let value = BollingerValue::new(110.0, 100.0, 90.0);

        // 하단에서 거리 0%
        assert!((value.distance_from_lower(90.0) - 0.0).abs() < 0.01);
        // 상단에서 거리 100%
        assert!((value.distance_from_lower(110.0) - 100.0).abs() < 0.01);
        // 중간에서 거리 50%
        assert!((value.distance_from_lower(100.0) - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_touching_upper_detection() {
        let mut bb = BollingerBands::new(5, 2.0).unwrap();

        // 데이터로 밴드 형성
        let prices = vec![100.0, 102.0, 98.0, 105.0, 95.0];
        for price in prices {
            let candle = create_test_candle(price);
            bb.update(&candle);
        }

        let value = bb.current_value().unwrap();

        // 상단 밴드 터치 (REQ-309)
        assert!(bb.is_touching_upper(value.upper));
        assert!(bb.is_touching_upper(value.upper - (value.upper * 0.005))); // 0.5% 내
        assert!(!bb.is_touching_upper(value.upper - (value.upper * 0.02))); // 2% 밖
    }

    #[test]
    fn test_touching_lower_detection() {
        let mut bb = BollingerBands::new(5, 2.0).unwrap();

        // 데이터로 밴드 형성
        let prices = vec![100.0, 102.0, 98.0, 105.0, 95.0];
        for price in prices {
            let candle = create_test_candle(price);
            bb.update(&candle);
        }

        let value = bb.current_value().unwrap();

        // 하단 밴드 터치 (REQ-310)
        assert!(bb.is_touching_lower(value.lower));
        assert!(bb.is_touching_lower(value.lower + (value.lower * 0.005))); // 0.5% 내
        assert!(!bb.is_touching_lower(value.lower + (value.lower * 0.02))); // 2% 밖
    }

    #[test]
    fn test_squeeze_detection() {
        let mut bb = BollingerBands::new(3, 2.0).unwrap();

        // 낮은 변동성: 100, 101, 99
        let prices = vec![100.0, 101.0, 99.0];
        for price in prices {
            let candle = create_test_candle(price);
            bb.update(&candle);
        }

        // 밴드폭이 좁으므로 스퀴즈 감지
        assert!(bb.is_squeeze());
    }

    #[test]
    fn test_no_squeeze_high_volatility() {
        let mut bb = BollingerBands::new(3, 2.0).unwrap();

        // 높은 변동성: 100, 150, 50
        let prices = vec![100.0, 150.0, 50.0];
        for price in prices {
            let candle = create_test_candle(price);
            bb.update(&candle);
        }

        // 높은 변동성이므로 스퀴즈 아님
        assert!(!bb.is_squeeze());
        assert!(bb.is_expanding());
    }

    #[test]
    fn test_squeeze_with_custom_threshold() {
        let mut bb = BollingerBands::new(3, 2.0).unwrap();

        let prices = vec![100.0, 102.0, 98.0];
        for price in prices {
            let candle = create_test_candle(price);
            bb.update(&candle);
        }

        // 낮은 임계값으로는 스퀴즈
        assert!(bb.is_squeeze_with_threshold(0.15));
        // 높은 임계값으로는 스퀴즈 아님
        assert!(!bb.is_squeeze_with_threshold(0.01));
    }

    #[test]
    fn test_above_upper_band() {
        let mut bb = BollingerBands::new(3, 2.0).unwrap();

        let prices = vec![100.0, 102.0, 98.0];
        for price in prices {
            let candle = create_test_candle(price);
            bb.update(&candle);
        }

        let value = bb.current_value().unwrap();

        assert!(bb.is_above_upper(value.upper + 1.0));
        assert!(!bb.is_above_upper(value.upper));
        assert!(!bb.is_above_upper(value.middle));
    }

    #[test]
    fn test_below_lower_band() {
        let mut bb = BollingerBands::new(3, 2.0).unwrap();

        let prices = vec![100.0, 102.0, 98.0];
        for price in prices {
            let candle = create_test_candle(price);
            bb.update(&candle);
        }

        let value = bb.current_value().unwrap();

        assert!(bb.is_below_lower(value.lower - 1.0));
        assert!(!bb.is_below_lower(value.lower));
        assert!(!bb.is_below_lower(value.middle));
    }

    #[test]
    fn test_reset() {
        let mut bb = BollingerBands::new(3, 2.0).unwrap();

        let prices = vec![100.0, 102.0, 98.0];
        for price in prices {
            let candle = create_test_candle(price);
            bb.update(&candle);
        }

        assert!(bb.value().is_some());
        assert_eq!(bb.prices.len(), 3);

        bb.reset();
        assert!(bb.value().is_none());
        assert!(bb.current_value().is_none());
        assert_eq!(bb.prices.len(), 0);
    }

    #[test]
    fn test_indicator_metadata() {
        let mut bb = BollingerBands::new(5, 2.5).unwrap();

        // 5개 캔들로 밴드 계산
        let prices = vec![100.0, 105.0, 95.0, 110.0, 90.0];
        for price in prices {
            let candle = create_test_candle(price);
            bb.update(&candle);
        }

        let value = bb.value();
        assert!(value.is_some());

        let metadata = &value.unwrap().metadata;
        assert_eq!(metadata.get("period"), Some(&"5".to_string()));
        assert_eq!(metadata.get("std_dev"), Some(&"2.5".to_string()));
        assert_eq!(metadata.get("type"), Some(&"BollingerBands".to_string()));
        assert!(metadata.contains_key("upper"));
        assert!(metadata.contains_key("lower"));
        assert!(metadata.contains_key("bandwidth"));
    }

    #[test]
    fn test_update_with_new_candle() {
        let mut bb = BollingerBands::new(3, 2.0).unwrap();

        // 첫 3개 캔들
        let candles = vec![100.0, 105.0, 95.0];
        for price in candles {
            let candle = create_test_candle(price);
            bb.update(&candle);
        }

        let first_value = bb.current_value().unwrap();
        assert!((first_value.middle - 100.0).abs() < 0.01);

        // 4번째 캔들 (가장 오래된 100.0 제거, 110.0 추가)
        let new_candle = create_test_candle(110.0);
        bb.update(&new_candle);

        let second_value = bb.current_value().unwrap();
        // 새 SMA = (105 + 95 + 110) / 3 ≈ 103.33
        assert!((second_value.middle - 103.33).abs() < 0.1);
    }

    #[test]
    fn test_bandwidth_edge_cases() {
        // middle이 0인 경우
        let value = BollingerValue::new(10.0, 0.0, -10.0);
        assert_eq!(value.bandwidth, 0.0);

        // upper == lower인 경우
        let value = BollingerValue::new(100.0, 100.0, 100.0);
        assert_eq!(value.bandwidth, 0.0);
    }

    #[test]
    fn test_percent_b_edge_case() {
        // upper == lower인 경우
        let value = BollingerValue::new(100.0, 100.0, 100.0);
        assert_eq!(value.percent_b(100.0), 0.5);
    }

    #[test]
    fn test_min_periods_requirement() {
        let bb = BollingerBands::new(20, 2.0).unwrap();
        // 기간 20이면 최소 20개 캔들 필요
        assert_eq!(bb.min_periods(), 20); // REQ-319
    }

    #[test]
    fn test_bollinger_bands_update_returns_none_until_full() {
        let mut bb = BollingerBands::new(5, 2.0).unwrap();

        // 4개 캔들 - 아직 값 없음
        for i in 1..=4 {
            let candle = create_test_candle(100.0 + i as f64);
            assert!(bb.update(&candle).is_none());
        }

        // 5번째 캔들 - 값 반환
        let candle = create_test_candle(105.0);
        let result = bb.update(&candle);
        assert!(result.is_some());
    }

    #[test]
    fn test_different_std_dev_multiplier() {
        let mut bb1 = BollingerBands::new(3, 1.0).unwrap();
        let mut bb2 = BollingerBands::new(3, 2.0).unwrap();

        let prices = vec![100.0, 105.0, 95.0];
        for price in &prices {
            let candle = create_test_candle(*price);
            bb1.update(&candle);
            bb2.update(&candle);
        }

        let value1 = bb1.current_value().unwrap();
        let value2 = bb2.current_value().unwrap();

        // 중간 밴드는 같음
        assert!((value1.middle - value2.middle).abs() < 0.01);

        // std_dev 2.0이 더 넓은 밴드
        assert!(value2.upper > value1.upper);
        assert!(value2.lower < value1.lower);
    }
}
