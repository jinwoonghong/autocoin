//! Moving Average Indicators
//!
//! 단순 이동평균(SMA)과 지수 이동평균(EMA) 지표 구현.

use crate::error::{Result, TradingError};
use crate::indicators::{validate_indicator_params, Indicator, IndicatorValue};
use crate::types::Candle;
use std::collections::VecDeque;

/// 단순 이동평균 (Simple Moving Average)
pub struct SMA {
    period: usize,
    prices: VecDeque<f64>,
    current_value: Option<f64>,
}

impl SMA {
    /// 새로운 SMA 지표 생성
    pub fn new(period: usize) -> Result<Self> {
        validate_indicator_params(period, 1.0)?;
        Ok(Self {
            period,
            prices: VecDeque::with_capacity(period),
            current_value: None,
        })
    }

    /// 기본 기간(20)으로 SMA 생성
    pub fn default_period() -> Result<Self> {
        Self::new(20)
    }
}

impl Indicator for SMA {
    fn update(&mut self, candle: &Candle) -> Option<IndicatorValue> {
        // 가격 추가
        if self.prices.len() >= self.period {
            self.prices.pop_front();
        }
        self.prices.push_back(candle.close_price);

        // 충분한 데이터가 있으면 계산 (REQ-319)
        if self.prices.len() >= self.period {
            let sum: f64 = self.prices.iter().sum();
            let avg = sum / self.prices.len() as f64;
            self.current_value = Some(avg);

            Some(
                IndicatorValue::new(candle.timestamp.timestamp_millis(), avg)
                    .with_metadata("period".to_string(), self.period.to_string())
                    .with_metadata("type".to_string(), "SMA".to_string()),
            )
        } else {
            None
        }
    }

    fn value(&self) -> Option<IndicatorValue> {
        self.current_value.map(|v| {
            IndicatorValue::new(chrono::Utc::now().timestamp_millis(), v)
                .with_metadata("period".to_string(), self.period.to_string())
                .with_metadata("type".to_string(), "SMA".to_string())
        })
    }

    fn reset(&mut self) {
        self.prices.clear();
        self.current_value = None;
    }

    fn name(&self) -> &str {
        "SMA"
    }

    fn min_periods(&self) -> usize {
        self.period
    }
}

/// 지수 이동평균 (Exponential Moving Average)
pub struct EMA {
    period: usize,
    multiplier: f64,
    current_ema: Option<f64>,
    alpha: f64,
}

impl EMA {
    /// 새로운 EMA 지표 생성
    pub fn new(period: usize) -> Result<Self> {
        validate_indicator_params(period, 1.0)?;
        let multiplier = 2.0 / (period as f64 + 1.0);
        let alpha = multiplier;
        Ok(Self {
            period,
            multiplier,
            current_ema: None,
            alpha,
        })
    }

    /// 기본 기간(12)으로 EMA 생성
    pub fn default_period() -> Result<Self> {
        Self::new(12)
    }

    /// 26기간 EMA 생성 (MACD용)
    pub fn slow_period() -> Result<Self> {
        Self::new(26)
    }
}

impl Indicator for EMA {
    fn update(&mut self, candle: &Candle) -> Option<IndicatorValue> {
        let price = candle.close_price;

        match self.current_ema {
            None => {
                // 첫 데이터: SMA로 초기화
                self.current_ema = Some(price);
            }
            Some(prev_ema) => {
                // EMA 계산: EMA = (price * multiplier) + (prev_ema * (1 - multiplier))
                let new_ema = (price * self.multiplier) + (prev_ema * (1.0 - self.multiplier));
                self.current_ema = Some(new_ema);
            }
        }

        self.current_value().map(|v| {
            IndicatorValue::new(candle.timestamp.timestamp_millis(), v)
                .with_metadata("period".to_string(), self.period.to_string())
                .with_metadata("type".to_string(), "EMA".to_string())
                .with_metadata("multiplier".to_string(), format!("{:.6}", self.multiplier))
        })
    }

    fn value(&self) -> Option<IndicatorValue> {
        self.current_value().map(|v| {
            IndicatorValue::new(chrono::Utc::now().timestamp_millis(), v)
                .with_metadata("period".to_string(), self.period.to_string())
                .with_metadata("type".to_string(), "EMA".to_string())
        })
    }

    fn reset(&mut self) {
        self.current_ema = None;
    }

    fn name(&self) -> &str {
        "EMA"
    }

    fn min_periods(&self) -> usize {
        self.period
    }
}

impl EMA {
    pub(crate) fn current_value(&self) -> Option<f64> {
        self.current_ema
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
    fn test_sma_creation() {
        let sma = SMA::new(14).unwrap();
        assert_eq!(sma.period, 14);
        assert_eq!(sma.name(), "SMA");
        assert_eq!(sma.min_periods(), 14);
        assert!(sma.value().is_none());
    }

    #[test]
    fn test_sma_invalid_period() {
        let result = SMA::new(0);
        assert!(result.is_err());
        assert!(matches!(result, Err(TradingError::InvalidParameter(_))));
    }

    #[test]
    fn test_sma_calculation() {
        let mut sma = SMA::new(3).unwrap();

        // 1번째 캔들: 데이터 부족
        let candle1 = create_test_candle(100.0);
        assert!(sma.update(&candle1).is_none());

        // 2번째 캔들: 데이터 부족
        let candle2 = create_test_candle(200.0);
        assert!(sma.update(&candle2).is_none());

        // 3번째 캔들: SMA 계산 가능
        let candle3 = create_test_candle(300.0);
        let result = sma.update(&candle3);
        assert!(result.is_some());
        // (100 + 200 + 300) / 3 = 200
        assert!((result.unwrap().value - 200.0).abs() < 0.01);

        // 4번째 캔들: 이동평균
        let candle4 = create_test_candle(400.0);
        let result = sma.update(&candle4);
        assert!(result.is_some());
        // (200 + 300 + 400) / 3 = 300
        assert!((result.unwrap().value - 300.0).abs() < 0.01);
    }

    #[test]
    fn test_sma_reset() {
        let mut sma = SMA::new(2).unwrap();

        let candle1 = create_test_candle(100.0);
        let candle2 = create_test_candle(200.0);
        sma.update(&candle1);
        sma.update(&candle2);
        assert!(sma.value().is_some());

        sma.reset();
        assert!(sma.value().is_none());
        assert_eq!(sma.prices.len(), 0);
    }

    #[test]
    fn test_ema_creation() {
        let ema = EMA::new(12).unwrap();
        assert_eq!(ema.period, 12);
        assert_eq!(ema.name(), "EMA");
        assert_eq!(ema.min_periods(), 12);
        // multiplier = 2 / (12 + 1) = 0.153846
        assert!((ema.multiplier - 0.153846).abs() < 0.0001);
    }

    #[test]
    fn test_ema_invalid_period() {
        let result = EMA::new(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_ema_first_value() {
        let mut ema = EMA::new(3).unwrap();
        let candle = create_test_candle(100.0);

        let result = ema.update(&candle);
        assert!(result.is_some());
        // 첫 값은 그 가격 자체
        assert!((result.unwrap().value - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_ema_calculation() {
        let mut ema = EMA::new(3).unwrap();
        // multiplier = 2 / (3 + 1) = 0.5

        // 첫 번째: EMA = 100
        let candle1 = create_test_candle(100.0);
        let result1 = ema.update(&candle1).unwrap();
        assert!((result1.value - 100.0).abs() < 0.01);

        // 두 번째: EMA = (200 * 0.5) + (100 * 0.5) = 150
        let candle2 = create_test_candle(200.0);
        let result2 = ema.update(&candle2).unwrap();
        assert!((result2.value - 150.0).abs() < 0.01);

        // 세 번째: EMA = (300 * 0.5) + (150 * 0.5) = 225
        let candle3 = create_test_candle(300.0);
        let result3 = ema.update(&candle3).unwrap();
        assert!((result3.value - 225.0).abs() < 0.01);
    }

    #[test]
    fn test_ema_reset() {
        let mut ema = EMA::new(3).unwrap();
        let candle = create_test_candle(100.0);

        ema.update(&candle);
        assert!(ema.value().is_some());

        ema.reset();
        assert!(ema.value().is_none());
    }

    #[test]
    fn test_indicator_value_metadata() {
        let mut sma = SMA::new(5).unwrap();
        let candle1 = create_test_candle(100.0);
        let candle2 = create_test_candle(200.0);
        let candle3 = create_test_candle(300.0);
        let candle4 = create_test_candle(400.0);
        let candle5 = create_test_candle(500.0);

        sma.update(&candle1);
        sma.update(&candle2);
        sma.update(&candle3);
        sma.update(&candle4);
        let result = sma.update(&candle5).unwrap();

        assert_eq!(result.metadata.get("period"), Some(&"5".to_string()));
        assert_eq!(result.metadata.get("type"), Some(&"SMA".to_string()));
    }

    #[test]
    fn test_ema_metadata() {
        let mut ema = EMA::new(12).unwrap();
        let candle = create_test_candle(100.0);
        let result = ema.update(&candle).unwrap();

        assert_eq!(result.metadata.get("period"), Some(&"12".to_string()));
        assert_eq!(result.metadata.get("type"), Some(&"EMA".to_string()));
        assert!(result.metadata.contains_key("multiplier"));
    }

    #[test]
    fn test_default_period_methods() {
        let sma = SMA::default_period().unwrap();
        assert_eq!(sma.period, 20);

        let ema = EMA::default_period().unwrap();
        assert_eq!(ema.period, 12);

        let ema_slow = EMA::slow_period().unwrap();
        assert_eq!(ema_slow.period, 26);
    }

    #[test]
    fn test_golden_cross_detection() {
        // 골든크로스: 단기 이동평균이 장기 이동평균을 상향 돌파
        let mut sma_short = SMA::new(3).unwrap();
        let mut sma_long = SMA::new(5).unwrap();

        let candles: Vec<f64> = vec![100.0, 110.0, 120.0, 130.0, 140.0, 150.0];
        let mut prev_short = None;
        let mut prev_long = None;
        let mut golden_cross_detected = false;

        for price in candles {
            let candle = create_test_candle(price);
            if let (Some(short_val), Some(long_val)) =
                (sma_short.update(&candle), sma_long.update(&candle))
            {
                if let (Some(ps), Some(pl)) = (prev_short, prev_long) {
                    // 이전에는 short < long 이었는데, 이제 short > long
                    if ps <= pl && short_val.value > long_val.value {
                        golden_cross_detected = true;
                    }
                }
                prev_short = Some(short_val);
                prev_long = Some(long_val);
            }
        }

        // 상승 추세이므로 골든크로스 발생
        assert!(golden_cross_detected);
    }

    #[test]
    fn test_death_cross_detection() {
        // 데드크로스: 단기 이동평균이 장기 이동평균을 하향 이탈
        let mut sma_short = SMA::new(3).unwrap();
        let mut sma_long = SMA::new(5).unwrap();

        let candles: Vec<f64> = vec![150.0, 140.0, 130.0, 120.0, 110.0, 100.0];
        let mut prev_short = None;
        let mut prev_long = None;
        let mut death_cross_detected = false;

        for price in candles {
            let candle = create_test_candle(price);
            if let (Some(short_val), Some(long_val)) =
                (sma_short.update(&candle), sma_long.update(&candle))
            {
                if let (Some(ps), Some(pl)) = (prev_short, prev_long) {
                    // 이전에는 short >= long 이었는데, 이제 short < long
                    if ps >= pl && short_val.value < long_val.value {
                        death_cross_detected = true;
                    }
                }
                prev_short = Some(short_val);
                prev_long = Some(long_val);
            }
        }

        // 하락 추세이므로 데드크로스 발생
        assert!(death_cross_detected);
    }

    #[test]
    fn test_insufficient_data_sma() {
        let sma = SMA::new(10).unwrap();
        let candle = create_test_candle(100.0);
        assert!(sma.update(&candle).is_none());
        assert_eq!(sma.min_periods(), 10); // REQ-319
    }
}
