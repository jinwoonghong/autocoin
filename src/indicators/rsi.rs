//! RSI (Relative Strength Index) Indicator
//!
//! RSI 지표 구현 - 과매수/과매도 상태를 식별하는 모멘텀 오실레이터

use crate::error::{Result, TradingError};
use crate::indicators::{validate_indicator_params, Indicator, IndicatorValue};
use crate::types::Candle;

/// RSI (Relative Strength Index)
pub struct RSI {
    period: usize,
    prices: Vec<f64>,
    gains: Vec<f64>,
    losses: Vec<f64>,
    current_rsi: Option<f64>,
}

impl RSI {
    /// 새로운 RSI 지표 생성
    pub fn new(period: usize) -> Result<Self> {
        validate_indicator_params(period, 1.0)?;
        Ok(Self {
            period,
            prices: Vec::with_capacity(period + 1),
            gains: Vec::new(),
            losses: Vec::new(),
            current_rsi: None,
        })
    }

    /// 기본 기간(14)으로 RSI 생성
    pub fn default_period() -> Result<Self> {
        Self::new(14)
    }

    /// 과매도 상태 확인 (REQ-305)
    pub fn is_oversold(&self, threshold: f64) -> bool {
        if let Some(rsi) = self.current_rsi {
            rsi < threshold
        } else {
            false
        }
    }

    /// 과매수 상태 확인 (REQ-306)
    pub fn is_overbought(&self, threshold: f64) -> bool {
        if let Some(rsi) = self.current_rsi {
            rsi > threshold
        } else {
            false
        }
    }

    /// 현재 RSI 값 반환
    pub fn current_value(&self) -> Option<f64> {
        self.current_rsi
    }

    /// RSI 계산 (내부 헬퍼 함수)
    fn calculate_rsi(&mut self, current_price: f64) -> Option<f64> {
        // 이전 가격과의 차이 계산
        if let Some(&prev_price) = self.prices.last() {
            let change = current_price - prev_price;

            if change > 0.0 {
                self.gains.push(change);
                self.losses.push(0.0);
            } else {
                self.gains.push(0.0);
                self.losses.push(change.abs());
            }

            // 기간보다 많은 데이터가 있으면 가장 오래된 것 제거
            if self.gains.len() > self.period {
                self.gains.remove(0);
                self.losses.remove(0);
            }

            // 충분한 데이터가 있으면 RSI 계산
            if self.gains.len() >= self.period {
                // 평균 상승/하락 계산
                let avg_gain: f64 = self.gains.iter().sum::<f64>() / self.period as f64;
                let avg_loss: f64 = self.losses.iter().sum::<f64>() / self.period as f64;

                if avg_loss == 0.0 {
                    // 하락이 없으면 RSI = 100
                    self.current_rsi = Some(100.0);
                } else {
                    let rs = avg_gain / avg_loss;
                    let rsi = 100.0 - (100.0 / (1.0 + rs));
                    self.current_rsi = Some(rsi);
                }

                return self.current_rsi;
            }
        }

        None
    }
}

impl Indicator for RSI {
    fn update(&mut self, candle: &Candle) -> Option<IndicatorValue> {
        let price = candle.close_price;

        // 첫 가격 저장 (변화 계산용)
        if self.prices.is_empty() {
            self.prices.push(price);
            return None; // 아직 RSI 계산 불가
        }

        // RSI 계산
        if let Some(rsi) = self.calculate_rsi(price) {
            self.prices.push(price);

            // 기간 + 1개만 유지 (다음 비교용)
            if self.prices.len() > self.period + 1 {
                self.prices.remove(0);
            }

            Some(
                IndicatorValue::new(candle.timestamp.timestamp_millis(), rsi)
                    .with_metadata("period".to_string(), self.period.to_string())
                    .with_metadata("type".to_string(), "RSI".to_string())
                    .with_metadata("oversold_threshold".to_string(), "30".to_string())
                    .with_metadata("overbought_threshold".to_string(), "70".to_string()),
            )
        } else {
            self.prices.push(price);
            None
        }
    }

    fn value(&self) -> Option<IndicatorValue> {
        self.current_rsi.map(|v| {
            IndicatorValue::new(chrono::Utc::now().timestamp_millis(), v)
                .with_metadata("period".to_string(), self.period.to_string())
                .with_metadata("type".to_string(), "RSI".to_string())
        })
    }

    fn reset(&mut self) {
        self.prices.clear();
        self.gains.clear();
        self.losses.clear();
        self.current_rsi = None;
    }

    fn name(&self) -> &str {
        "RSI"
    }

    fn min_periods(&self) -> usize {
        self.period + 1 // 첫 가격 + period 개의 변화 필요
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
    fn test_rsi_creation() {
        let rsi = RSI::new(14).unwrap();
        assert_eq!(rsi.period, 14);
        assert_eq!(rsi.name(), "RSI");
        assert_eq!(rsi.min_periods(), 15); // 14 + 1
        assert!(rsi.value().is_none());
        assert!(!rsi.is_oversold(30.0));
        assert!(!rsi.is_overbought(70.0));
    }

    #[test]
    fn test_rsi_invalid_period() {
        let result = RSI::new(0);
        assert!(result.is_err());
        assert!(matches!(result, Err(TradingError::InvalidParameter(_))));
    }

    #[test]
    fn test_rsi_insufficient_data() {
        let mut rsi = RSI::new(14).unwrap();
        let candle = create_test_candle(50000.0);

        // 데이터 부족하면 None 반환 (REQ-319)
        assert!(rsi.update(&candle).is_none());
        assert!(rsi.value().is_none());
    }

    #[test]
    fn test_rsi_calculation_uptrend() {
        let mut rsi = RSI::new(3).unwrap(); // 기간 3 (최소 4개 캔들 필요)

        // 상승 추세: 100 -> 110 -> 120 -> 130 -> 140
        let prices = vec![100.0, 110.0, 120.0, 130.0, 140.0];
        let mut result = None;

        for price in prices {
            let candle = create_test_candle(price);
            result = rsi.update(&candle);
        }

        // 상승 추세이므로 RSI가 높음 (> 50)
        assert!(result.is_some());
        let rsi_value = result.unwrap().value;
        assert!(rsi_value > 50.0, "RSI should be > 50 for uptrend, got {}", rsi_value);
    }

    #[test]
    fn test_rsi_calculation_downtrend() {
        let mut rsi = RSI::new(3).unwrap();

        // 하락 추세: 140 -> 130 -> 120 -> 110 -> 100
        let prices = vec![140.0, 130.0, 120.0, 110.0, 100.0];
        let mut result = None;

        for price in prices {
            let candle = create_test_candle(price);
            result = rsi.update(&candle);
        }

        // 하락 추세이므로 RSI가 낮음 (< 50)
        assert!(result.is_some());
        let rsi_value = result.unwrap().value;
        assert!(
            rsi_value < 50.0,
            "RSI should be < 50 for downtrend, got {}",
            rsi_value
        );
    }

    #[test]
    fn test_rsi_oversold_detection() {
        let mut rsi = RSI::new(3).unwrap();

        // 강한 하락 추세로 과매도 상태 유발
        let prices: Vec<f64> = vec![100.0, 90.0, 80.0, 70.0, 60.0, 50.0];
        let mut result = None;

        for price in prices {
            let candle = create_test_candle(price);
            result = rsi.update(&candle);
        }

        assert!(result.is_some());
        assert!(rsi.is_oversold(30.0), "Should detect oversold (RSI < 30)"); // REQ-305
    }

    #[test]
    fn test_rsi_overbought_detection() {
        let mut rsi = RSI::new(3).unwrap();

        // 강한 상승 추세로 과매수 상태 유발
        let prices: Vec<f64> = vec![100.0, 110.0, 120.0, 130.0, 140.0, 150.0];
        let mut result = None;

        for price in prices {
            let candle = create_test_candle(price);
            result = rsi.update(&candle);
        }

        assert!(result.is_some());
        assert!(rsi.is_overbought(70.0), "Should detect overbought (RSI > 70)"); // REQ-306
    }

    #[test]
    fn test_rsi_neutral_range() {
        let mut rsi = RSI::new(3).unwrap();

        // 횡보: 100 -> 102 -> 98 -> 101 -> 99 -> 100
        let prices: Vec<f64> = vec![100.0, 102.0, 98.0, 101.0, 99.0, 100.0];
        let mut result = None;

        for price in prices {
            let candle = create_test_candle(price);
            result = rsi.update(&candle);
        }

        assert!(result.is_some());
        let rsi_value = result.unwrap().value;
        // 횡보이므로 RSI는 30~70 사이 (중립 구간)
        assert!(
            rsi_value >= 30.0 && rsi_value <= 70.0,
            "RSI should be in neutral range [30, 70], got {}",
            rsi_value
        );
        assert!(!rsi.is_oversold(30.0));
        assert!(!rsi.is_overbought(70.0));
    }

    #[test]
    fn test_rsi_reset() {
        let mut rsi = RSI::new(3).unwrap();

        let prices = vec![100.0, 110.0, 120.0, 130.0];
        for price in prices {
            let candle = create_test_candle(price);
            rsi.update(&candle);
        }

        assert!(rsi.value().is_some());

        rsi.reset();
        assert!(rsi.value().is_none());
        assert!(rsi.prices.is_empty());
        assert!(rsi.gains.is_empty());
        assert!(rsi.losses.is_empty());
    }

    #[test]
    fn test_rsi_default_period() {
        let rsi = RSI::default_period().unwrap();
        assert_eq!(rsi.period, 14);
    }

    #[test]
    fn test_rsi_metadata() {
        let mut rsi = RSI::new(5).unwrap();

        // 6개 캔들로 RSI 계산 (기간 5 + 1)
        for i in 1..=6 {
            let candle = create_test_candle(100.0 + (i as f64) * 10.0);
            rsi.update(&candle);
        }

        let value = rsi.value();
        assert!(value.is_some());

        let metadata = &value.unwrap().metadata;
        assert_eq!(metadata.get("period"), Some(&"5".to_string()));
        assert_eq!(metadata.get("type"), Some(&"RSI".to_string()));
        assert_eq!(metadata.get("oversold_threshold"), Some(&"30".to_string()));
        assert_eq!(metadata.get("overbought_threshold"), Some(&"70".to_string()));
    }

    #[test]
    fn test_rsi_extreme_values() {
        let mut rsi = RSI::new(2).unwrap();

        // 모든 캔들이 상승 -> RSI = 100
        let prices: Vec<f64> = vec![100.0, 110.0, 120.0, 130.0];
        let mut result = None;

        for price in prices {
            let candle = create_test_candle(price);
            result = rsi.update(&candle);
        }

        assert!(result.is_some());
        let rsi_value = result.unwrap().value;
        // 하락이 없으므로 RSI가 100에 가까워야 함
        assert!(
            rsi_value > 90.0,
            "RSI should be close to 100 for all gains, got {}",
            rsi_value
        );
    }

    #[test]
    fn test_rsi_custom_thresholds() {
        let mut rsi = RSI::new(3).unwrap();

        // 과매도 유발 (강한 하락)
        let prices: Vec<f64> = vec![100.0, 85.0, 70.0, 55.0, 40.0];
        for price in prices {
            let candle = create_test_candle(price);
            rsi.update(&candle);
        }

        // 기본 임계값 (30)
        assert!(rsi.is_oversold(30.0));

        // 더 엄격한 임계값 (20)
        assert!(rsi.is_oversold(20.0));

        // 덜 엄격한 임계값 (40)
        assert!(rsi.is_oversold(40.0));
    }

    #[test]
    fn test_rsi_formula_accuracy() {
        let mut rsi = RSI::new(2).unwrap();

        // 계산 가능한 예제
        // Price: 100 -> 110 -> 100
        // Changes: +10, -10
        // Gains: 10, 0 (avg = 5)
        // Losses: 0, 10 (avg = 5)
        // RS = 5/5 = 1
        // RSI = 100 - (100/(1+1)) = 50

        let candle1 = create_test_candle(100.0);
        let candle2 = create_test_candle(110.0);
        let candle3 = create_test_candle(100.0);

        rsi.update(&candle1); // 초기 데이터
        rsi.update(&candle2); // +10 gain
        let result = rsi.update(&candle3); // -10 loss

        assert!(result.is_some());
        let rsi_value = result.unwrap().value;
        // 이상적인 RSI = 50 (동일한 상승/하락)
        assert!(
            (rsi_value - 50.0).abs() < 1.0,
            "RSI should be close to 50 for balanced gains/losses, got {}",
            rsi_value
        );
    }

    #[test]
    fn test_rsi_min_periods_requirement() {
        let rsi = RSI::new(14).unwrap();
        // 기간 14이면 최소 15개 캔들 필요 (첫 가격 + 14개 변화)
        assert_eq!(rsi.min_periods(), 15); // REQ-319
    }
}
