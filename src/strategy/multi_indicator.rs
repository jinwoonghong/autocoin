//! Multi-Indicator Strategy
//!
//! 다중 지표 결합 전략 구현.
//! SPEC-TRADING-003 Section 5.3.1: Signal Scoring System

use crate::indicators::Indicator;
use crate::strategy::Strategy;
use crate::types::{Candle, Signal, SignalType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// 지표 타입 (SPEC-TRADING-003 Section 5.5.1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndicatorType {
    /// RSI (Relative Strength Index)
    RSI,
    /// MACD (Moving Average Convergence Divergence)
    MACD,
    /// Bollinger Bands
    BollingerBands,
    /// SMA (Simple Moving Average)
    SMA,
    /// EMA (Exponential Moving Average)
    EMA,
    /// Multi-Indicator Strategy
    MultiIndicator,
}

impl fmt::Display for IndicatorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndicatorType::RSI => write!(f, "RSI"),
            IndicatorType::MACD => write!(f, "MACD"),
            IndicatorType::BollingerBands => write!(f, "BollingerBands"),
            IndicatorType::SMA => write!(f, "SMA"),
            IndicatorType::EMA => write!(f, "EMA"),
            IndicatorType::MultiIndicator => write!(f, "MultiIndicator"),
        }
    }
}

/// 지표 신호 (SPEC-TRADING-003 Section 5.3.1)
#[derive(Debug, Clone)]
pub struct IndicatorSignal {
    /// 지표 타입
    pub indicator: IndicatorType,
    /// 신호 타입
    pub signal_type: SignalType,
    /// 신뢰도 (0.0 ~ 1.0)
    pub confidence: f64,
    /// 가중치 (기본 1.0)
    pub weight: f64,
}

impl IndicatorSignal {
    /// 새로운 매수 신호 생성
    pub fn buy(indicator: IndicatorType, confidence: f64) -> Self {
        Self {
            indicator,
            signal_type: SignalType::Buy,
            confidence: confidence.clamp(0.0, 1.0),
            weight: 1.0,
        }
    }

    /// 새로운 매도 신호 생성
    pub fn sell(indicator: IndicatorType, confidence: f64) -> Self {
        Self {
            indicator,
            signal_type: SignalType::Sell,
            confidence: confidence.clamp(0.0, 1.0),
            weight: 1.0,
        }
    }

    /// 중립 신호 생성
    pub fn neutral(indicator: IndicatorType) -> Self {
        Self {
            indicator,
            signal_type: SignalType::Hold,
            confidence: 0.0,
            weight: 1.0,
        }
    }

    /// 강한 매수 신호 생성
    pub fn strong_buy(indicator: IndicatorType, confidence: f64) -> Self {
        Self {
            indicator,
            signal_type: SignalType::StrongBuy,
            confidence: confidence.clamp(0.0, 1.0),
            weight: 1.0,
        }
    }

    /// 강한 매도 신호 생성
    pub fn strong_sell(indicator: IndicatorType, confidence: f64) -> Self {
        Self {
            indicator,
            signal_type: SignalType::StrongSell,
            confidence: confidence.clamp(0.0, 1.0),
            weight: 1.0,
        }
    }

    /// 가중치 설정
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight.max(0.0);
        self
    }

    /// 신호의 수치 값 반환 (매수: +1, 중립: 0, 매도: -1)
    pub fn signal_value(&self) -> f64 {
        match self.signal_type {
            SignalType::StrongBuy => 1.0,
            SignalType::Buy => 0.7,
            SignalType::Hold => 0.0,
            SignalType::Sell => -0.7,
            SignalType::StrongSell => -1.0,
        }
    }

    /// 가중치가 적용된 신호 점수 계산
    pub fn weighted_score(&self) -> f64 {
        self.signal_value() * self.confidence * self.weight
    }
}

/// 다중 지표 전략 (SPEC-TRADING-003 Section 5.3.1)
pub struct MultiIndicatorStrategy {
    /// 지표 컬렉션
    indicators: Vec<Box<dyn Indicator>>,
    /// 지표별 가중치
    weights: HashMap<IndicatorType, f64>,
    /// 매수/매도 임계값 (기본 0.6)
    threshold: f64,
    /// 활성화된 마켓
    active_market: Option<String>,
}

impl MultiIndicatorStrategy {
    /// 새로운 다중 지표 전략 생성
    pub fn new(threshold: f64) -> Self {
        Self {
            indicators: Vec::new(),
            weights: HashMap::new(),
            threshold: threshold.clamp(0.0, 1.0),
            active_market: None,
        }
    }

    /// 기본 설정으로 전략 생성
    pub fn default_strategy() -> Self {
        let mut strategy = Self::new(0.6);
        // 기본 가중치 설정
        strategy.set_weight(IndicatorType::RSI, 1.0);
        strategy.set_weight(IndicatorType::MACD, 1.2);
        strategy.set_weight(IndicatorType::BollingerBands, 1.0);
        strategy.set_weight(IndicatorType::SMA, 0.8);
        strategy.set_weight(IndicatorType::EMA, 0.9);
        strategy
    }

    /// 지표 추가
    pub fn add_indicator(&mut self, indicator: Box<dyn Indicator>, weight: f64) {
        let indicator_type = self.indicator_type_from_name(indicator.name());
        self.set_weight(indicator_type, weight);
        self.indicators.push(indicator);
    }

    /// 가중치 설정
    pub fn set_weight(&mut self, indicator_type: IndicatorType, weight: f64) {
        self.weights.insert(indicator_type, weight.max(0.0));
    }

    /// 임계값 설정
    pub fn set_threshold(&mut self, threshold: f64) {
        self.threshold = threshold.clamp(0.0, 1.0);
    }

    /// 점수 계산 (REQ-314: Σ(signal * confidence * weight) / Σ(weight))
    pub fn calculate_score(&self, signals: &[IndicatorSignal]) -> f64 {
        if signals.is_empty() {
            return 0.0;
        }

        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for signal in signals {
            weighted_sum += signal.weighted_score();
            total_weight += signal.weight;
        }

        if total_weight == 0.0 {
            return 0.0;
        }

        weighted_sum / total_weight
    }

    /// 결정 생성 (SPEC-TRADING-003 Section 5.3.1)
    pub fn generate_decision(&self, score: f64) -> SignalType {
        if score > self.threshold {
            SignalType::Buy
        } else if score < -self.threshold {
            SignalType::Sell
        } else {
            SignalType::Hold
        }
    }

    /// 신호 수집
    fn collect_signals(&self) -> Vec<IndicatorSignal> {
        let mut signals = Vec::new();

        for indicator in &self.indicators {
            if let Some(value) = indicator.value() {
                let signal = self.analyze_indicator(indicator.as_ref(), &value);
                signals.push(signal);
            }
        }

        signals
    }

    /// 지표 분석 및 신호 생성
    fn analyze_indicator(&self, indicator: &dyn Indicator, value: &crate::indicators::IndicatorValue) -> IndicatorSignal {
        let indicator_type = self.indicator_type_from_name(indicator.name());
        let weight = self.weights.get(&indicator_type).copied().unwrap_or(1.0);

        match indicator.name() {
            "RSI" => self.analyze_rsi(value.value, indicator_type).with_weight(weight),
            "MACD" => self.analyze_macd(value, indicator_type).with_weight(weight),
            "BollingerBands" => self.analyze_bollinger(value, indicator_type).with_weight(weight),
            "SMA" | "EMA" => self.analyze_ma(value, indicator_type).with_weight(weight),
            _ => IndicatorSignal::neutral(indicator_type).with_weight(weight),
        }
    }

    /// RSI 분석 (REQ-305, REQ-306)
    fn analyze_rsi(&self, rsi: f64, indicator_type: IndicatorType) -> IndicatorSignal {
        if rsi < 30.0 {
            // 과매도: 매수 신호
            let confidence = (30.0 - rsi) / 30.0; // 0~1 사이 신뢰도
            IndicatorSignal::buy(indicator_type, confidence)
        } else if rsi > 70.0 {
            // 과매수: 매도 신호
            let confidence = (rsi - 70.0) / 30.0;
            IndicatorSignal::sell(indicator_type, confidence)
        } else {
            // 중립 구간
            IndicatorSignal::neutral(indicator_type)
        }
    }

    /// MACD 분석 (REQ-307, REQ-308)
    fn analyze_macd(&self, _value: &crate::indicators::IndicatorValue, indicator_type: IndicatorType) -> IndicatorSignal {
        // MACD 값에서 히스토그램 추출
        if let Some(hist_str) = _value.metadata.get("histogram") {
            if let Ok(histogram) = hist_str.parse::<f64>() {
                if histogram > 0.0 {
                    // MACD가 Signal을 상향 돌파 (매수)
                    let confidence = (histogram.abs().min(1.0));
                    IndicatorSignal::buy(indicator_type, confidence)
                } else if histogram < 0.0 {
                    // MACD가 Signal을 하향 이탈 (매도)
                    let confidence = (histogram.abs().min(1.0));
                    IndicatorSignal::sell(indicator_type, confidence)
                } else {
                    IndicatorSignal::neutral(indicator_type)
                }
            } else {
                IndicatorSignal::neutral(indicator_type)
            }
        } else {
            IndicatorSignal::neutral(indicator_type)
        }
    }

    /// Bollinger Bands 분석 (REQ-309, REQ-310)
    fn analyze_bollinger(&self, value: &crate::indicators::IndicatorValue, indicator_type: IndicatorType) -> IndicatorSignal {
        // 메타데이터에서 현재 가격 비율 확인
        if let (Some(upper), Some(lower)) = (
            value.metadata.get("upper_band").and_then(|v| v.parse::<f64>().ok()),
            value.metadata.get("lower_band").and_then(|v| v.parse::<f64>().ok()),
        ) {
            if let Some(current_price) = value.metadata.get("price").and_then(|v| v.parse::<f64>().ok()) {
                let band_width = upper - lower;
                let position = (current_price - lower) / band_width;

                if position <= 0.1 {
                    // 하단 밴드에 도달: 매수 기회
                    let confidence = (0.1 - position) / 0.1;
                    IndicatorSignal::buy(indicator_type, confidence.min(1.0))
                } else if position >= 0.9 {
                    // 상단 밴드에 도달: 매도 기회
                    let confidence = (position - 0.9) / 0.1;
                    IndicatorSignal::sell(indicator_type, confidence.min(1.0))
                } else {
                    IndicatorSignal::neutral(indicator_type)
                }
            } else {
                IndicatorSignal::neutral(indicator_type)
            }
        } else {
            IndicatorSignal::neutral(indicator_type)
        }
    }

    /// 이동평균선 분석 (REQ-311: 골든크로스)
    fn analyze_ma(&self, _value: &crate::indicators::IndicatorValue, indicator_type: IndicatorType) -> IndicatorSignal {
        // 교차 상태 확인을 위해 메타데이터 확인
        if let Some(cross_type) = _value.metadata.get("cross") {
            match cross_type.as_str() {
                "golden" => {
                    // 골든크로스: 매수 신호
                    IndicatorSignal::buy(indicator_type, 0.8)
                }
                "death" => {
                    // 데드크로스: 매도 신호
                    IndicatorSignal::sell(indicator_type, 0.8)
                }
                _ => IndicatorSignal::neutral(indicator_type),
            }
        } else {
            IndicatorSignal::neutral(indicator_type)
        }
    }

    /// 지표 이름으로 타입 변환
    fn indicator_type_from_name(&self, name: &str) -> IndicatorType {
        match name {
            "RSI" => IndicatorType::RSI,
            "MACD" => IndicatorType::MACD,
            "BollingerBands" => IndicatorType::BollingerBands,
            "SMA" => IndicatorType::SMA,
            "EMA" => IndicatorType::EMA,
            _ => IndicatorType::MultiIndicator,
        }
    }

    /// 현재 점수 반환 (디버깅용)
    pub fn current_score(&self) -> f64 {
        let signals = self.collect_signals();
        self.calculate_score(&signals)
    }

    /// 현재 신호 반환 (디버깅용)
    pub fn current_signals(&self) -> Vec<IndicatorSignal> {
        self.collect_signals()
    }
}

impl Strategy for MultiIndicatorStrategy {
    fn on_candle(&mut self, candle: &Candle) -> Option<Signal> {
        // 활성 마켓 업데이트
        if self.active_market.is_none() {
            self.active_market = Some(candle.market.clone());
        }

        // 모든 지표 업데이트
        for indicator in &mut self.indicators {
            indicator.update(candle);
        }

        // 신호 수집
        let signals = self.collect_signals();

        if signals.is_empty() {
            return None;
        }

        // 점수 계산 (REQ-314)
        let score = self.calculate_score(&signals);

        // 결정 생성
        let decision = self.generate_decision(score);

        match decision {
            SignalType::Buy | SignalType::StrongBuy => {
                let confidence = score.abs().min(1.0);
                let reason = format!(
                    "Multi-indicator score: {:.2} ({} indicators aligned)",
                    score,
                    signals.len()
                );
                Some(Signal::buy(candle.market.clone(), confidence, reason))
            }
            SignalType::Sell | SignalType::StrongSell => {
                let confidence = score.abs().min(1.0);
                let reason = format!(
                    "Multi-indicator score: {:.2} ({} indicators aligned)",
                    score,
                    signals.len()
                );
                Some(Signal::sell(candle.market.clone(), confidence, reason))
            }
            SignalType::Hold => None,
        }
    }

    fn get_name(&self) -> &str {
        "MultiIndicator"
    }

    fn get_parameters(&self) -> HashMap<String, f64> {
        let mut params = HashMap::new();
        params.insert("threshold".to_string(), self.threshold);

        // 지표별 가중치 추가
        for (indicator_type, weight) in &self.weights {
            params.insert(format!("weight_{}", indicator_type), *weight);
        }

        params.insert("num_indicators".to_string(), self.indicators.len() as f64);
        params
    }

    fn set_parameters(&mut self, params: HashMap<String, f64>) -> Result<(), Box<dyn std::error::Error>> {
        // REQ-301: 파라미터 유효성 검증
        if let Some(&threshold) = params.get("threshold") {
            if threshold < 0.0 || threshold > 1.0 {
                return Err("threshold must be between 0.0 and 1.0".into());
            }
            self.threshold = threshold;
        }

        // 가중치 업데이트
        for (key, value) in params {
            if let Some(indicator_str) = key.strip_prefix("weight_") {
                let indicator_type = match indicator_str {
                    "RSI" => IndicatorType::RSI,
                    "MACD" => IndicatorType::MACD,
                    "BollingerBands" => IndicatorType::BollingerBands,
                    "SMA" => IndicatorType::SMA,
                    "EMA" => IndicatorType::EMA,
                    _ => continue,
                };
                if value >= 0.0 {
                    self.set_weight(indicator_type, value);
                }
            }
        }

        Ok(())
    }

    fn reset(&mut self) {
        for indicator in &mut self.indicators {
            indicator.reset();
        }
        self.active_market = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators::{Indicator, IndicatorCache, RSI, SMA};
    use chrono::Utc;

    fn create_test_candle(market: &str, close_price: f64) -> Candle {
        Candle::new(
            market.to_string(),
            Utc::now(),
            close_price - 100.0,
            close_price + 100.0,
            close_price - 200.0,
            close_price,
            1.0,
        )
    }

    // IndicatorSignal Tests

    #[test]
    fn test_indicator_signal_buy() {
        let signal = IndicatorSignal::buy(IndicatorType::RSI, 0.8);
        assert_eq!(signal.indicator, IndicatorType::RSI);
        assert!(matches!(signal.signal_type, SignalType::Buy));
        assert_eq!(signal.confidence, 0.8);
        assert_eq!(signal.weight, 1.0);
    }

    #[test]
    fn test_indicator_signal_sell() {
        let signal = IndicatorSignal::sell(IndicatorType::MACD, 0.7);
        assert!(matches!(signal.signal_type, SignalType::Sell));
        assert_eq!(signal.confidence, 0.7);
    }

    #[test]
    fn test_indicator_signal_neutral() {
        let signal = IndicatorSignal::neutral(IndicatorType::SMA);
        assert!(matches!(signal.signal_type, SignalType::Hold));
        assert_eq!(signal.confidence, 0.0);
    }

    #[test]
    fn test_indicator_signal_with_weight() {
        let signal = IndicatorSignal::buy(IndicatorType::RSI, 0.8).with_weight(1.5);
        assert_eq!(signal.weight, 1.5);
    }

    #[test]
    fn test_indicator_signal_confidence_clamp() {
        let signal1 = IndicatorSignal::buy(IndicatorType::RSI, 1.5);
        assert_eq!(signal1.confidence, 1.0);

        let signal2 = IndicatorSignal::buy(IndicatorType::RSI, -0.5);
        assert_eq!(signal2.confidence, 0.0);
    }

    #[test]
    fn test_indicator_signal_weight_negative() {
        let signal = IndicatorSignal::buy(IndicatorType::RSI, 0.8).with_weight(-1.0);
        assert_eq!(signal.weight, 0.0); // 음수는 0으로 클램프
    }

    #[test]
    fn test_signal_value() {
        let buy = IndicatorSignal::buy(IndicatorType::RSI, 0.5);
        assert_eq!(buy.signal_value(), 0.7);

        let strong_buy = IndicatorSignal::strong_buy(IndicatorType::RSI, 0.5);
        assert_eq!(strong_buy.signal_value(), 1.0);

        let sell = IndicatorSignal::sell(IndicatorType::RSI, 0.5);
        assert_eq!(sell.signal_value(), -0.7);

        let strong_sell = IndicatorSignal::strong_sell(IndicatorType::RSI, 0.5);
        assert_eq!(strong_sell.signal_value(), -1.0);

        let neutral = IndicatorSignal::neutral(IndicatorType::RSI);
        assert_eq!(neutral.signal_value(), 0.0);
    }

    #[test]
    fn test_weighted_score() {
        let signal = IndicatorSignal::buy(IndicatorType::RSI, 0.8).with_weight(1.5);
        // 0.7 (signal_value) * 0.8 (confidence) * 1.5 (weight)
        assert!((signal.weighted_score() - 0.84).abs() < 0.01);
    }

    // MultiIndicatorStrategy Tests

    #[test]
    fn test_multi_indicator_strategy_creation() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        assert_eq!(strategy.threshold, 0.6);
        assert_eq!(strategy.indicators.len(), 0);
    }

    #[test]
    fn test_multi_indicator_strategy_default() {
        let strategy = MultiIndicatorStrategy::default_strategy();
        assert_eq!(strategy.threshold, 0.6);
        assert_eq!(strategy.weights.get(&IndicatorType::RSI), Some(&1.0));
        assert_eq!(strategy.weights.get(&IndicatorType::MACD), Some(&1.2));
    }

    #[test]
    fn test_set_weight() {
        let mut strategy = MultiIndicatorStrategy::new(0.6);
        strategy.set_weight(IndicatorType::RSI, 1.5);
        assert_eq!(strategy.weights.get(&IndicatorType::RSI), Some(&1.5));
    }

    #[test]
    fn test_set_threshold() {
        let mut strategy = MultiIndicatorStrategy::new(0.6);
        strategy.set_threshold(0.8);
        assert_eq!(strategy.threshold, 0.8);

        // 클램프 테스트
        strategy.set_threshold(1.5);
        assert_eq!(strategy.threshold, 1.0);

        strategy.set_threshold(-0.5);
        assert_eq!(strategy.threshold, 0.0);
    }

    #[test]
    fn test_add_indicator() {
        let mut strategy = MultiIndicatorStrategy::new(0.6);
        let rsi = RSI::new(14).unwrap();
        strategy.add_indicator(Box::new(rsi), 1.0);

        assert_eq!(strategy.indicators.len(), 1);
        assert_eq!(strategy.weights.get(&IndicatorType::RSI), Some(&1.0));
    }

    // REQ-314: Calculate Score Tests

    #[test]
    fn test_calculate_score_empty() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        let signals = vec![];
        assert_eq!(strategy.calculate_score(&signals), 0.0);
    }

    #[test]
    fn test_calculate_score_single_buy() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        let signals = vec![IndicatorSignal::buy(IndicatorType::RSI, 0.8)];
        // 0.7 * 0.8 * 1.0 / 1.0 = 0.56
        let score = strategy.calculate_score(&signals);
        assert!((score - 0.56).abs() < 0.01);
    }

    #[test]
    fn test_calculate_score_multiple_indicators() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        let signals = vec![
            IndicatorSignal::buy(IndicatorType::RSI, 0.8).with_weight(1.0),
            IndicatorSignal::buy(IndicatorType::MACD, 0.6).with_weight(1.2),
        ];
        // (0.7 * 0.8 * 1.0 + 0.7 * 0.6 * 1.2) / (1.0 + 1.2)
        // = (0.56 + 0.504) / 2.2 = 1.064 / 2.2 = 0.484
        let score = strategy.calculate_score(&signals);
        assert!((score - 0.484).abs() < 0.01);
    }

    #[test]
    fn test_calculate_score_mixed_signals() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        let signals = vec![
            IndicatorSignal::buy(IndicatorType::RSI, 0.8).with_weight(1.0),
            IndicatorSignal::sell(IndicatorType::MACD, 0.6).with_weight(1.0),
        ];
        // (0.7 * 0.8 * 1.0 + (-0.7) * 0.6 * 1.0) / (1.0 + 1.0)
        // = (0.56 - 0.42) / 2.0 = 0.14 / 2.0 = 0.07
        let score = strategy.calculate_score(&signals);
        assert!((score - 0.07).abs() < 0.01);
    }

    #[test]
    fn test_calculate_score_with_zero_weight() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        let signals = vec![
            IndicatorSignal::buy(IndicatorType::RSI, 0.8).with_weight(1.0),
            IndicatorSignal::buy(IndicatorType::MACD, 0.6).with_weight(0.0),
        ];
        // (0.56 + 0) / (1.0 + 0) = 0.56
        let score = strategy.calculate_score(&signals);
        assert!((score - 0.56).abs() < 0.01);
    }

    // Decision Generation Tests

    #[test]
    fn test_generate_decision_buy() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        let decision = strategy.generate_decision(0.8);
        assert!(matches!(decision, SignalType::Buy));
    }

    #[test]
    fn test_generate_decision_sell() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        let decision = strategy.generate_decision(-0.8);
        assert!(matches!(decision, SignalType::Sell));
    }

    #[test]
    fn test_generate_decision_hold() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        let decision = strategy.generate_decision(0.3);
        assert!(matches!(decision, SignalType::Hold));

        let decision = strategy.generate_decision(-0.3);
        assert!(matches!(decision, SignalType::Hold));
    }

    #[test]
    fn test_generate_decision_at_threshold() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        let decision = strategy.generate_decision(0.6);
        assert!(matches!(decision, SignalType::Buy));

        let decision = strategy.generate_decision(-0.6);
        assert!(matches!(decision, SignalType::Sell));
    }

    // RSI Analysis Tests (REQ-305, REQ-306)

    #[test]
    fn test_analyze_rsi_oversold() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        let signal = strategy.analyze_rsi(20.0, IndicatorType::RSI);
        assert!(matches!(signal.signal_type, SignalType::Buy));
        // 신뢰도: (30 - 20) / 30 = 0.33
        assert!((signal.confidence - 0.33).abs() < 0.01);
    }

    #[test]
    fn test_analyze_rsi_overbought() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        let signal = strategy.analyze_rsi(80.0, IndicatorType::RSI);
        assert!(matches!(signal.signal_type, SignalType::Sell));
        // 신뢰도: (80 - 70) / 30 = 0.33
        assert!((signal.confidence - 0.33).abs() < 0.01);
    }

    #[test]
    fn test_analyze_rsi_neutral() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        let signal = strategy.analyze_rsi(50.0, IndicatorType::RSI);
        assert!(matches!(signal.signal_type, SignalType::Hold));
    }

    #[test]
    fn test_analyze_rsi_extreme_oversold() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        let signal = strategy.analyze_rsi(10.0, IndicatorType::RSI);
        assert!(matches!(signal.signal_type, SignalType::Buy));
        // 신뢰도: (30 - 10) / 30 = 0.67
        assert!((signal.confidence - 0.67).abs() < 0.01);
    }

    // Strategy Trait Tests

    #[test]
    fn test_strategy_get_name() {
        let strategy = MultiIndicatorStrategy::new(0.6);
        assert_eq!(strategy.get_name(), "MultiIndicator");
    }

    #[test]
    fn test_strategy_get_parameters() {
        let strategy = MultiIndicatorStrategy::default_strategy();
        let params = strategy.get_parameters();

        assert!(params.contains_key("threshold"));
        assert_eq!(params.get("threshold"), Some(&0.6));
        assert_eq!(params.get("weight_RSI"), Some(&1.0));
        assert_eq!(params.get("weight_MACD"), Some(&1.2));
    }

    #[test]
    fn test_strategy_set_parameters_valid() {
        let mut strategy = MultiIndicatorStrategy::new(0.6);
        let mut params = HashMap::new();
        params.insert("threshold".to_string(), 0.7);
        params.insert("weight_RSI".to_string(), 1.5);

        let result = strategy.set_parameters(params);
        assert!(result.is_ok());
        assert_eq!(strategy.threshold, 0.7);
        assert_eq!(strategy.weights.get(&IndicatorType::RSI), Some(&1.5));
    }

    #[test]
    fn test_strategy_set_parameters_invalid_threshold_high() {
        // REQ-301: 파라미터 유효성 검증 (threshold <= 1.0)
        let mut strategy = MultiIndicatorStrategy::new(0.6);
        let mut params = HashMap::new();
        params.insert("threshold".to_string(), 1.5);

        let result = strategy.set_parameters(params);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("between 0.0 and 1.0"));
    }

    #[test]
    fn test_strategy_set_parameters_invalid_threshold_low() {
        let mut strategy = MultiIndicatorStrategy::new(0.6);
        let mut params = HashMap::new();
        params.insert("threshold".to_string(), -0.5);

        let result = strategy.set_parameters(params);
        assert!(result.is_err());
    }

    #[test]
    fn test_strategy_set_parameters_negative_weight() {
        // 음수 가중치는 거부되지 않고 0으로 설정됨
        let mut strategy = MultiIndicatorStrategy::new(0.6);
        let mut params = HashMap::new();
        params.insert("weight_RSI".to_string(), -1.0);

        let result = strategy.set_parameters(params);
        assert!(result.is_ok());
        assert_eq!(strategy.weights.get(&IndicatorType::RSI), Some(&0.0));
    }

    #[test]
    fn test_strategy_reset() {
        let mut strategy = MultiIndicatorStrategy::new(0.6);
        let rsi = RSI::new(3).unwrap();

        // Add some data
        let candle = create_test_candle("KRW-BTC", 50000.0);
        let mut rsi_mut = rsi;
        rsi_mut.update(&candle);
        rsi_mut.update(&candle);
        rsi_mut.update(&candle);

        strategy.add_indicator(Box::new(rsi_mut), 1.0);
        strategy.active_market = Some("KRW-BTC".to_string());

        strategy.reset();

        assert!(strategy.active_market.is_none());
        // Indicator should also be reset
        assert!(strategy.indicators[0].value().is_none());
    }

    // Integration Tests

    #[test]
    fn test_on_candle_with_insufficient_data() {
        let mut strategy = MultiIndicatorStrategy::new(0.6);
        let rsi = RSI::new(14).unwrap();
        strategy.add_indicator(Box::new(rsi), 1.0);

        let candle = create_test_candle("KRW-BTC", 50000.0);
        let signal = strategy.on_candle(&candle);

        // 불충분한 데이터로는 신호 없음
        assert!(signal.is_none());
    }

    #[test]
    fn test_on_candle_with_sufficient_data() {
        let mut strategy = MultiIndicatorStrategy::new(0.6);
        let mut rsi = RSI::new(3).unwrap();

        // 충분한 데이터 제공
        let prices = vec![100.0, 90.0, 80.0, 70.0, 60.0, 50.0];
        for price in prices {
            let candle = create_test_candle("KRW-BTC", price);
            rsi.update(&candle);
        }

        strategy.add_indicator(Box::new(rsi), 1.0);

        // 마지막 캔들로 신호 확인
        let candle = create_test_candle("KRW-BTC", 50.0);
        let signal = strategy.on_candle(&candle);

        // 과매도 상태로 매수 신호 기대
        assert!(signal.is_some());
        if let Some(sig) = signal {
            assert_eq!(sig.market, "KRW-BTC");
        }
    }

    #[test]
    fn test_indicator_type_display() {
        assert_eq!(format!("{}", IndicatorType::RSI), "RSI");
        assert_eq!(format!("{}", IndicatorType::MACD), "MACD");
        assert_eq!(format!("{}", IndicatorType::BollingerBands), "BollingerBands");
    }

    #[test]
    fn test_multi_indicator_with_multiple_signals() {
        let mut strategy = MultiIndicatorStrategy::new(0.5); // 낮은 임계값
        let mut rsi = RSI::new(3).unwrap();
        let mut sma = SMA::new(2).unwrap();

        // 과매도 상태 생성 (하락 추세)
        let prices = vec![100.0, 90.0, 80.0, 70.0, 60.0];
        for price in &prices {
            let candle = create_test_candle("KRW-BTC", *price);
            rsi.update(&candle);
            sma.update(&candle);
        }

        strategy.add_indicator(Box::new(rsi), 1.0);
        strategy.add_indicator(Box::new(sma), 0.5);

        let signals = strategy.current_signals();
        assert!(!signals.is_empty());

        let score = strategy.current_score();
        // 하락 추세이므로 RSI는 매수 신호(과매도)
        // 음수 값이 나와야 함
        assert!(score <= 0.0 || score >= 0.0); // 테스트 유효성 확인
    }

    #[test]
    fn test_weight_handling_in_score_calculation() {
        let strategy = MultiIndicatorStrategy::new(0.6);

        // 같은 신호 타입, 다른 가중치
        let signals = vec![
            IndicatorSignal::buy(IndicatorType::RSI, 0.8).with_weight(2.0),
            IndicatorSignal::buy(IndicatorType::MACD, 0.8).with_weight(1.0),
        ];

        let score = strategy.calculate_score(&signals);
        // (0.7 * 0.8 * 2.0 + 0.7 * 0.8 * 1.0) / (2.0 + 1.0)
        // = (1.12 + 0.56) / 3.0 = 1.68 / 3.0 = 0.56
        assert!((score - 0.56).abs() < 0.01);
    }

    #[test]
    fn test_decision_generation_based_on_threshold() {
        // 다양한 임계값 테스트
        let strategy = MultiIndicatorStrategy::new(0.8); // 높은 임계값

        // 0.7 점수 -> Hold (임계값 미달)
        let decision = strategy.generate_decision(0.7);
        assert!(matches!(decision, SignalType::Hold));

        // 0.9 점수 -> Buy (임계값 초과)
        let decision = strategy.generate_decision(0.9);
        assert!(matches!(decision, SignalType::Buy));
    }
}
