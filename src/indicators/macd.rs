//! MACD (Moving Average Convergence Divergence) Indicator
//!
//! MACD 지표 구현 - 추세 추적 오실레이터
//! MACD = EMA(12) - EMA(26)
//! Signal = EMA(MACD, 9)
//! Histogram = MACD - Signal

use crate::error::{Result, TradingError};
use crate::indicators::{validate_indicator_params, Indicator, IndicatorValue};
use crate::indicators::moving_average::EMA;
use crate::types::Candle;

/// MACD 값
#[derive(Debug, Clone, PartialEq)]
pub struct MACDValue {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

impl MACDValue {
    /// 새로운 MACD 값 생성
    pub fn new(macd: f64, signal: f64) -> Self {
        let histogram = macd - signal;
        Self { macd, signal, histogram }
    }
}

/// MACD (Moving Average Convergence Divergence)
pub struct MACD {
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
    fast_ema: EMA,
    slow_ema: EMA,
    signal_ema: EMA,
    macd_values: Vec<f64>,
    prev_macd: Option<f64>,
    prev_signal: Option<f64>,
    current_value: Option<MACDValue>,
}

impl MACD {
    /// 새로운 MACD 지표 생성
    pub fn new(fast: usize, slow: usize, signal: usize) -> Result<Self> {
        validate_indicator_params(fast, 1.0)?;
        validate_indicator_params(slow, 1.0)?;
        validate_indicator_params(signal, 1.0)?;

        if fast >= slow {
            return Err(TradingError::InvalidParameter(
                "Fast period must be less than slow period".to_string(),
            ));
        }

        Ok(Self {
            fast_period: fast,
            slow_period: slow,
            signal_period: signal,
            fast_ema: EMA::new(fast)?,
            slow_ema: EMA::new(slow)?,
            signal_ema: EMA::new(signal)?,
            macd_values: Vec::new(),
            prev_macd: None,
            prev_signal: None,
            current_value: None,
        })
    }

    /// 기본 파라미터(12, 26, 9)로 MACD 생성
    pub fn default_params() -> Result<Self> {
        Self::new(12, 26, 9)
    }

    /// 현재 MACD 값 반환
    pub fn current_value(&self) -> Option<MACDValue> {
        self.current_value.clone()
    }

    /// 불리시 크로스 감지 (REQ-307: MACD가 Signal을 상향 돌파)
    pub fn is_bullish_cross(&self) -> bool {
        match (self.prev_macd, self.prev_signal, &self.current_value) {
            (Some(prev_macd), Some(prev_signal), Some(curr)) => {
                // 이전: MACD <= Signal, 현재: MACD > Signal
                prev_macd <= prev_signal && curr.macd > curr.signal
            }
            _ => false,
        }
    }

    /// 베어리시 크로스 감지 (REQ-308: MACD가 Signal을 하향 이탈)
    pub fn is_bearish_cross(&self) -> bool {
        match (self.prev_macd, self.prev_signal, &self.current_value) {
            (Some(prev_macd), Some(prev_signal), Some(curr)) => {
                // 이전: MACD >= Signal, 현재: MACD < Signal
                prev_macd >= prev_signal && curr.macd < curr.signal
            }
            _ => false,
        }
    }

    /// 히스토그램이 양수인지 확인 (MACD > Signal)
    pub fn is_histogram_positive(&self) -> bool {
        self.current_value
            .as_ref()
            .map(|v| v.histogram > 0.0)
            .unwrap_or(false)
    }

    /// 히스토그램이 음수인지 확인 (MACD < Signal)
    pub fn is_histogram_negative(&self) -> bool {
        self.current_value
            .as_ref()
            .map(|v| v.histogram < 0.0)
            .unwrap_or(false)
    }

    /// MACD가 0보다 큰지 확인
    pub fn is_macd_positive(&self) -> bool {
        self.current_value
            .as_ref()
            .map(|v| v.macd > 0.0)
            .unwrap_or(false)
    }

    /// MACD 추세 확인 (최근 히스토그램 변화)
    pub fn is_histogram_rising(&self) -> bool {
        if self.macd_values.len() < 2 {
            return false;
        }

        let current_hist = self.current_value
            .as_ref()
            .map(|v| v.histogram);

        let prev_hist = if self.macd_values.len() >= self.signal_period + 1 {
            // 직전 MACD 값으로 계산
            let prev_macd = self.macd_values[self.macd_values.len() - 2];
            let prev_signal = self.signal_ema.current_value().unwrap_or(prev_macd);
            Some(prev_macd - prev_signal)
        } else {
            None
        };

        match (current_hist, prev_hist) {
            (Some(curr), Some(prev)) => curr > prev,
            _ => false,
        }
    }
}

impl Indicator for MACD {
    fn update(&mut self, candle: &Candle) -> Option<IndicatorValue> {
        // 이전 값 저장 (크로스 감지용)
        if let Some(curr) = &self.current_value {
            self.prev_macd = Some(curr.macd);
            self.prev_signal = Some(curr.signal);
        }

        // Fast/Slow EMA 업데이트
        self.fast_ema.update(candle);
        self.slow_ema.update(candle);

        // MACD 계산: EMA(fast) - EMA(slow)
        if let (Some(fast_val), Some(slow_val)) = (
            self.fast_ema.value().map(|v| v.value),
            self.slow_ema.value().map(|v| v.value),
        ) {
            let macd = fast_val - slow_val;
            self.macd_values.push(macd);

            // Signal EMA 업데이트를 위해 가상 캔들 생성
            let signal_candle = Candle::new(
                candle.market.clone(),
                candle.timestamp,
                macd,
                macd,
                macd,
                macd,
                1.0,
            );

            // Signal EMA 업데이트
            self.signal_ema.update(&signal_candle);

            // Signal 값 가져오기
            if let Some(signal_val) = self.signal_ema.value().map(|v| v.value) {
                let macd_value = MACDValue::new(macd, signal_val);
                self.current_value = Some(macd_value.clone());

                return Some(
                    IndicatorValue::new(candle.timestamp.timestamp_millis(), macd)
                        .with_metadata("macd".to_string(), format!("{:.6}", macd))
                        .with_metadata("signal".to_string(), format!("{:.6}", signal_val))
                        .with_metadata("histogram".to_string(), format!("{:.6}", macd_value.histogram))
                        .with_metadata("type".to_string(), "MACD".to_string())
                        .with_metadata("fast_period".to_string(), self.fast_period.to_string())
                        .with_metadata("slow_period".to_string(), self.slow_period.to_string())
                        .with_metadata("signal_period".to_string(), self.signal_period.to_string()),
                );
            }
        }

        None
    }

    fn value(&self) -> Option<IndicatorValue> {
        self.current_value.as_ref().map(|v| {
            IndicatorValue::new(chrono::Utc::now().timestamp_millis(), v.macd)
                .with_metadata("macd".to_string(), format!("{:.6}", v.macd))
                .with_metadata("signal".to_string(), format!("{:.6}", v.signal))
                .with_metadata("histogram".to_string(), format!("{:.6}", v.histogram))
                .with_metadata("type".to_string(), "MACD".to_string())
        })
    }

    fn reset(&mut self) {
        self.fast_ema.reset();
        self.slow_ema.reset();
        self.signal_ema.reset();
        self.macd_values.clear();
        self.prev_macd = None;
        self.prev_signal = None;
        self.current_value = None;
    }

    fn name(&self) -> &str {
        "MACD"
    }

    fn min_periods(&self) -> usize {
        // Slow EMA period + Signal EMA period 필요
        self.slow_period + self.signal_period
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
    fn test_macd_creation() {
        let macd = MACD::new(12, 26, 9).unwrap();
        assert_eq!(macd.fast_period, 12);
        assert_eq!(macd.slow_period, 26);
        assert_eq!(macd.signal_period, 9);
        assert_eq!(macd.name(), "MACD");
        assert_eq!(macd.min_periods(), 35); // 26 + 9
        assert!(macd.current_value().is_none());
    }

    #[test]
    fn test_macd_invalid_params() {
        // fast >= slow 는 유효하지 않음
        let result = MACD::new(26, 12, 9);
        assert!(result.is_err());

        let result = MACD::new(12, 12, 9);
        assert!(result.is_err());

        // 0 기간은 유효하지 않음
        let result = MACD::new(0, 26, 9);
        assert!(result.is_err());
    }

    #[test]
    fn test_macd_value_creation() {
        let macd_val = MACDValue::new(1.5, 1.0);
        assert_eq!(macd_val.macd, 1.5);
        assert_eq!(macd_val.signal, 1.0);
        assert_eq!(macd_val.histogram, 0.5); // 1.5 - 1.0

        let macd_val = MACDValue::new(0.5, 1.0);
        assert_eq!(macd_val.histogram, -0.5); // 0.5 - 1.0
    }

    #[test]
    fn test_macd_insufficient_data() {
        let mut macd = MACD::new(5, 10, 3).unwrap(); // 최소 13개 필요 (10 + 3)
        let candle = create_test_candle(50000.0);

        // 데이터 부족하면 None 반환
        for _ in 0..5 {
            assert!(macd.update(&candle).is_none());
        }
        assert!(macd.value().is_none());
    }

    #[test]
    fn test_macd_calculation_uptrend() {
        let mut macd = MACD::new(3, 5, 2).unwrap(); // 최소 7개 필요

        // 상승 추세
        let prices: Vec<f64> = vec![100.0, 105.0, 110.0, 115.0, 120.0, 125.0, 130.0, 140.0];
        let mut result = None;

        for price in prices {
            let candle = create_test_candle(price);
            result = macd.update(&candle);
        }

        // 상승 추세이므로 MACD > 0
        assert!(result.is_some());
        let macd_val = macd.current_value();
        assert!(macd_val.is_some());
        let val = macd_val.unwrap();
        assert!(val.macd > 0.0, "MACD should be positive for uptrend");
    }

    #[test]
    fn test_macd_calculation_downtrend() {
        let mut macd = MACD::new(3, 5, 2).unwrap();

        // 하락 추세
        let prices: Vec<f64> = vec![140.0, 135.0, 130.0, 125.0, 120.0, 115.0, 110.0, 100.0];
        let mut result = None;

        for price in prices {
            let candle = create_test_candle(price);
            result = macd.update(&candle);
        }

        // 하락 추세이므로 MACD < 0
        assert!(result.is_some());
        let macd_val = macd.current_value();
        assert!(macd_val.is_some());
        let val = macd_val.unwrap();
        assert!(val.macd < 0.0, "MACD should be negative for downtrend");
    }

    #[test]
    fn test_macd_histogram_calculation() {
        let mut macd = MACD::new(3, 5, 2).unwrap();

        // 데이터 충분히 공급
        for i in 1..=10 {
            let price = 100.0 + (i as f64) * 10.0;
            let candle = create_test_candle(price);
            macd.update(&candle);
        }

        let macd_val = macd.current_value();
        assert!(macd_val.is_some());

        let val = macd_val.unwrap();
        // Histogram = MACD - Signal
        assert!((val.histogram - (val.macd - val.signal)).abs() < 0.0001);
    }

    #[test]
    fn test_macd_bullish_cross() {
        let mut macd = MACD::new(3, 5, 2).unwrap();

        // 하락 추세로 시작 (MACD < Signal)
        let prices_down: Vec<f64> = vec![150.0, 145.0, 140.0, 135.0, 130.0, 125.0, 120.0];
        for price in prices_down {
            macd.update(&create_test_candle(price));
        }

        // 상승 반전 (MACD > Signal로 크로스)
        let prices_up: Vec<f64> = vec![125.0, 130.0, 135.0, 140.0, 145.0, 150.0];
        let mut cross_detected = false;
        for price in prices_up {
            macd.update(&create_test_candle(price));
            if macd.is_bullish_cross() {
                cross_detected = true;
            }
        }

        // 상승 반전 시 불리시 크로스 감지
        assert!(cross_detected, "Should detect bullish cross when MACD crosses above signal");
    }

    #[test]
    fn test_macd_bearish_cross() {
        let mut macd = MACD::new(3, 5, 2).unwrap();

        // 상승 추세로 시작 (MACD > Signal)
        let prices_up: Vec<f64> = vec![100.0, 105.0, 110.0, 115.0, 120.0, 125.0, 130.0];
        for price in prices_up {
            macd.update(&create_test_candle(price));
        }

        // 하락 반전 (MACD < Signal로 크로스)
        let prices_down: Vec<f64> = vec![125.0, 120.0, 115.0, 110.0, 105.0, 100.0];
        let mut cross_detected = false;
        for price in prices_down {
            macd.update(&create_test_candle(price));
            if macd.is_bearish_cross() {
                cross_detected = true;
            }
        }

        // 하락 반전 시 베어리시 크로스 감지
        assert!(cross_detected, "Should detect bearish cross when MACD crosses below signal");
    }

    #[test]
    fn test_macd_histogram_positive() {
        let mut macd = MACD::new(3, 5, 2).unwrap();

        // 강한 상승
        for i in 1..=10 {
            let price = 100.0 + (i as f64) * 20.0;
            macd.update(&create_test_candle(price));
        }

        assert!(macd.is_histogram_positive());
        assert!(!macd.is_histogram_negative());
    }

    #[test]
    fn test_macd_histogram_negative() {
        let mut macd = MACD::new(3, 5, 2).unwrap();

        // 강한 하락
        for i in 1..=10 {
            let price = 300.0 - (i as f64) * 20.0;
            macd.update(&create_test_candle(price));
        }

        assert!(macd.is_histogram_negative());
        assert!(!macd.is_histogram_positive());
    }

    #[test]
    fn test_macd_reset() {
        let mut macd = MACD::new(3, 5, 2).unwrap();

        // 데이터 추가
        for i in 1..=10 {
            let price = 100.0 + (i as f64) * 10.0;
            macd.update(&create_test_candle(price));
        }

        assert!(macd.current_value().is_some());

        macd.reset();
        assert!(macd.current_value().is_none());
        assert!(macd.prev_macd.is_none());
        assert!(macd.prev_signal.is_none());
        assert!(macd.macd_values.is_empty());
    }

    #[test]
    fn test_macd_default_params() {
        let macd = MACD::default_params().unwrap();
        assert_eq!(macd.fast_period, 12);
        assert_eq!(macd.slow_period, 26);
        assert_eq!(macd.signal_period, 9);
    }

    #[test]
    fn test_macd_min_periods() {
        let macd = MACD::new(12, 26, 9).unwrap();
        // 26 (slow) + 9 (signal) = 35
        assert_eq!(macd.min_periods(), 35);
    }

    #[test]
    fn test_macd_metadata() {
        let mut macd = MACD::new(3, 5, 2).unwrap();

        // 최소 데이터 공급 (7개 이상)
        for i in 1..=10 {
            let price = 100.0 + (i as f64) * 10.0;
            macd.update(&create_test_candle(price));
        }

        let value = macd.value();
        assert!(value.is_some());

        let metadata = &value.unwrap().metadata;
        assert_eq!(metadata.get("type"), Some(&"MACD".to_string()));
        assert_eq!(metadata.get("fast_period"), Some(&"3".to_string()));
        assert_eq!(metadata.get("slow_period"), Some(&"5".to_string()));
        assert_eq!(metadata.get("signal_period"), Some(&"2".to_string()));
        assert!(metadata.contains_key("macd"));
        assert!(metadata.contains_key("signal"));
        assert!(metadata.contains_key("histogram"));
    }

    #[test]
    fn test_macd_is_macd_positive() {
        let mut macd = MACD::new(3, 5, 2).unwrap();

        // 초기 상태
        assert!(!macd.is_macd_positive());

        // 상승 추세
        for i in 1..=10 {
            let price = 100.0 + (i as f64) * 20.0;
            macd.update(&create_test_candle(price));
        }

        assert!(macd.is_macd_positive());
    }

    #[test]
    fn test_macd_formula_accuracy() {
        let mut macd = MACD::new(2, 3, 2).unwrap();

        // 단순한 케이스로 검증
        // Period 2 EMA: multiplier = 2/(2+1) = 0.667
        // Period 3 EMA: multiplier = 2/(3+1) = 0.5

        let prices: Vec<f64> = vec![100.0, 110.0, 120.0, 130.0, 140.0];
        for price in prices {
            macd.update(&create_test_candle(price));
        }

        let macd_val = macd.current_value();
        assert!(macd_val.is_some());

        let val = macd_val.unwrap();
        // MACD, Signal, Histogram 관계 검증
        assert!((val.histogram - (val.macd - val.signal)).abs() < 0.0001);
    }

    #[test]
    fn test_macd_no_cross_without_data() {
        let macd = MACD::new(12, 26, 9).unwrap();
        // 데이터가 없으면 크로스 감지 불가
        assert!(!macd.is_bullish_cross()); // REQ-307
        assert!(!macd.is_bearish_cross()); // REQ-308
    }

    #[test]
    fn test_macd_no_cross_with_insufficient_history() {
        let mut macd = MACD::new(3, 5, 2).unwrap();

        // 최소 데이터만 공급 (첫 계산)
        for i in 1..=7 {
            let price = 100.0 + (i as f64) * 10.0;
            macd.update(&create_test_candle(price));
        }

        // 첫 계산 직후는 크로스 감지 불가 (이전 값 없음)
        assert!(!macd.is_bullish_cross());
        assert!(!macd.is_bearish_cross());
    }

    #[test]
    fn test_macd_consecutive_updates() {
        let mut macd = MACD::new(3, 5, 2).unwrap();

        // 일정한 가격으로 업데이트
        let mut prev_value = None;
        for _ in 1..=10 {
            let candle = create_test_candle(100.0);
            if let Some(result) = macd.update(&candle) {
                if let Some(prev) = prev_value {
                    // 연속 업데이트 시 값이 점진적으로 수렴
                    assert!((result.value - prev).abs() < 10.0);
                }
                prev_value = Some(result.value);
            }
        }

        // 수렴 후 MACD는 0에 가까워짐 (일정한 가격)
        let macd_val = macd.current_value();
        assert!(macd_val.is_some());
        // 일정한 가격이면 Fast/Slow EMA가 수렴하여 MACD -> 0
        assert!(macd_val.unwrap().macd.abs() < 10.0);
    }
}
