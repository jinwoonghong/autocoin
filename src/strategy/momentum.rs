//! Momentum following strategy
//!
//! 가격 급등과 거래량 급증을 감지하는 모멘텀 전략입니다.

use crate::types::{PriceTick, Signal, SignalType};
use std::collections::VecDeque;

/// 모멘텀 전략
///
/// 감지 조건:
/// 1. 가격이 지정된 시간 내 N% 이상 상승
/// 2. 거래량이 평균의 M배 이상
pub struct MomentumStrategy {
    /// 급등 임계값 (예: 0.05 = 5%)
    surge_threshold: f64,
    /// 거래량 배수 (예: 2.0 = 평균의 2배)
    volume_multiplier: f64,
    /// 시간 프레임 (분)
    timeframe_minutes: u64,

    /// 가격 히스토리
    price_history: std::collections::HashMap<String, VecDeque<PriceTick>>,
    /// 최대 히스토리 크기
    max_history: usize,
}

impl MomentumStrategy {
    /// 새로운 모멘텀 전략 생성
    pub fn new(surge_threshold: f64, volume_multiplier: f64, timeframe_minutes: u64) -> Self {
        Self {
            surge_threshold,
            volume_multiplier,
            timeframe_minutes,
            price_history: std::collections::HashMap::new(),
            max_history: 1000,
        }
    }

    /// 가격 틱 추가 및 신호 분석
    pub fn update_and_analyze(&mut self, tick: PriceTick) -> Option<Signal> {
        self.add_tick(tick.clone());
        self.analyze(&tick)
    }

    /// 가격 틱 추가
    fn add_tick(&mut self, tick: PriceTick) {
        let history = self
            .price_history
            .entry(tick.market.clone())
            .or_insert_with(|| VecDeque::with_capacity(self.max_history));

        history.push_back(tick);

        if history.len() > self.max_history {
            history.pop_front();
        }
    }

    /// 시장 분석
    pub fn analyze(&self, tick: &PriceTick) -> Option<Signal> {
        let history = self.price_history.get(&tick.market)?;

        if history.len() < 2 {
            return None;
        }

        let timeframe_ms = self.timeframe_minutes as i64 * 60 * 1000;
        let now = tick.timestamp;

        // 시간 프레임 내 데이터 필터링
        let relevant: Vec<_> = history
            .iter()
            .filter(|t| now - t.timestamp <= timeframe_ms)
            .collect();

        if relevant.len() < 2 {
            return None;
        }

        let oldest = relevant.first()?;
        let latest = relevant.last()?;

        // 가격 상승률 계산
        let price_change = (latest.trade_price / oldest.trade_price) - 1.0;

        // 평균 거래량 계산
        let avg_volume: f64 = relevant.iter().map(|t| t.volume).sum::<f64>() / relevant.len() as f64;

        // 거래량 비율
        let volume_ratio = if avg_volume > 0.0 {
            tick.volume / avg_volume
        } else {
            1.0
        };

        // 매수 신호 조건 확인
        if price_change >= self.surge_threshold && volume_ratio >= self.volume_multiplier {
            let confidence = self.calculate_confidence(price_change, volume_ratio);
            return Some(Signal::buy(
                tick.market.clone(),
                confidence,
                format!(
                    "Price surged {:.2}% with {:.1}x volume",
                    price_change * 100.0,
                    volume_ratio
                ),
            ));
        }

        None
    }

    /// 신호 신뢰도 계산
    fn calculate_confidence(&self, price_change: f64, volume_ratio: f64) -> f64 {
        let price_score = (price_change / self.surge_threshold).min(2.0) / 2.0;
        let volume_score = (volume_ratio / self.volume_multiplier).min(3.0) / 3.0;

        (price_score * 0.6 + volume_score * 0.4).min(1.0)
    }

    /// 전략 파라미터 업데이트
    pub fn update_parameters(&mut self, surge_threshold: f64, volume_multiplier: f64) {
        self.surge_threshold = surge_threshold;
        self.volume_multiplier = volume_multiplier;
    }

    /// 히스토리 정리
    pub fn clear_history(&mut self) {
        self.price_history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_momentum_strategy_creation() {
        let strategy = MomentumStrategy::new(0.05, 2.0, 60);
        assert_eq!(strategy.surge_threshold, 0.05);
        assert_eq!(strategy.volume_multiplier, 2.0);
    }

    #[test]
    fn test_surge_detection() {
        let mut strategy = MomentumStrategy::new(0.05, 2.0, 60);
        let base_time = chrono::Utc::now().timestamp_millis();

        // 기본 데이터
        strategy.add_tick(PriceTick::new(
            "KRW-BTC".to_string(),
            base_time - 3600000,
            50000.0,
            0.0,
            1.0,
        ));

        // 급등 데이터 (5% 상승, 2배 거래량)
        let signal = strategy.update_and_analyze(PriceTick::new(
            "KRW-BTC".to_string(),
            base_time,
            52500.0,
            0.05,
            2.0,
        ));

        assert!(signal.is_some());
        assert!(matches!(signal.unwrap().signal_type, SignalType::Buy));
    }

    #[test]
    fn test_confidence_calculation() {
        let strategy = MomentumStrategy::new(0.05, 2.0, 60);

        // 10% 상승, 3배 거래량 = 높은 신뢰도
        let confidence = strategy.calculate_confidence(0.10, 3.0);
        assert!(confidence > 0.7);
    }

    #[test]
    fn test_no_signal_below_threshold() {
        let mut strategy = MomentumStrategy::new(0.05, 2.0, 60);
        let base_time = chrono::Utc::now().timestamp_millis();

        // 기본 데이터
        strategy.add_tick(PriceTick::new(
            "KRW-BTC".to_string(),
            base_time - 3600000,
            50000.0,
            0.0,
            1.0,
        ));

        // 3% 상승만 (임계값 미달)
        let signal = strategy.update_and_analyze(PriceTick::new(
            "KRW-BTC".to_string(),
            base_time,
            51500.0,
            0.03,
            1.5,
        ));

        assert!(signal.is_none());
    }
}
