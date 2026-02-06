//! Signal Detector Agent
//!
//! 모멘텀/서징 감지 및 매수/매도 신호 생성을 담당합니다.

use crate::config::TradingConfig;
use crate::types::{PriceTick, Signal, SignalType};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, info};

/// 가격 데이터 저장소
struct PriceHistory {
    ticks: Vec<PriceTick>,
    max_size: usize,
}

impl PriceHistory {
    fn new(max_size: usize) -> Self {
        Self {
            ticks: Vec::with_capacity(max_size),
            max_size,
        }
    }

    fn add(&mut self, tick: PriceTick) {
        self.ticks.push(tick);
        if self.ticks.len() > self.max_size {
            self.ticks.remove(0);
        }
    }

    fn get_latest(&self, market: &str) -> Option<&PriceTick> {
        self.ticks.iter().rev().find(|t| t.market == market)
    }

    fn get_in_timeframe(&self, market: &str, duration_ms: i64) -> Vec<&PriceTick> {
        let now = chrono::Utc::now().timestamp_millis();
        self.ticks
            .iter()
            .filter(|t| {
                t.market == market && (now - t.timestamp) <= duration_ms
            })
            .collect()
    }

    fn calculate_avg_volume(&self, market: &str, duration_ms: i64) -> f64 {
        let ticks = self.get_in_timeframe(market, duration_ms);
        if ticks.is_empty() {
            return 0.0;
        }

        let total: f64 = ticks.iter().map(|t| t.volume).sum();
        total / ticks.len() as f64
    }

    fn calculate_price_change(&self, market: &str, duration_ms: i64) -> Option<f64> {
        let ticks = self.get_in_timeframe(market, duration_ms);
        if ticks.len() < 2 {
            return None;
        }

        let oldest = ticks.first()?;
        let latest = ticks.last()?;

        if oldest.trade_price > 0.0 {
            Some((latest.trade_price / oldest.trade_price) - 1.0)
        } else {
            None
        }
    }
}

/// Signal Detector Agent
///
/// 역할: 모멘텀/서징 감지, 매수 신호 생성
/// 입력: PriceTick 스트림
/// 출력: Signal 스트림
pub struct SignalDetector {
    config: TradingConfig,
    history: PriceHistory,
    price_tx: mpsc::Sender<PriceTick>,
    signal_rx: mpsc::Receiver<PriceTick>,
    signal_tx: Option<mpsc::Sender<Signal>>,
}

impl SignalDetector {
    /// 새로운 Signal Detector 생성
    pub fn new(
        config: TradingConfig,
        price_rx: mpsc::Receiver<PriceTick>,
    ) -> (Self, mpsc::Sender<PriceTick>) {
        let (price_tx, price_rx) = mpsc::channel(1000);

        let detector = Self {
            config,
            history: PriceHistory::new(10000),
            price_tx,
            signal_rx: price_rx,
            signal_tx: None,
        };

        (detector, price_tx)
    }

    /// 신호 출력 채널 설정
    pub fn with_signal_channel(mut self, tx: mpsc::Sender<Signal>) -> Self {
        self.signal_tx = Some(tx);
        self
    }

    /// 신호 감지 시작
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting signal detector");

        while let Some(tick) = self.signal_rx.recv().await {
            // 가격 틱 저장
            self.history.add(tick.clone());

            // 신호 분석
            if let Some(signal) = self.analyze_market(&tick).await {
                debug!("Signal detected: {:?} for {}", signal.signal_type, signal.market);

                if let Some(tx) = &self.signal_tx {
                    let _ = tx.send(signal).await;
                }
            }
        }

        Ok(())
    }

    /// 시장 분석 및 신호 생성
    async fn analyze_market(&self, tick: &PriceTick) -> Option<Signal> {
        let timeframe_ms = self.config.surge_timeframe_minutes as i64 * 60 * 1000;

        // 가격 상승률 계산
        let price_change = self.history.calculate_price_change(&tick.market, timeframe_ms);

        // 거래량 평균 계산
        let avg_volume = self.history.calculate_avg_volume(&tick.market, timeframe_ms);

        // 급등 감지
        if let Some(change) = price_change {
            let volume_ratio = if avg_volume > 0.0 {
                tick.volume / avg_volume
            } else {
                1.0
            };

            // 매수 신호 조건:
            // 1. 가격 상승률이 임계값 이상
            // 2. 거래량이 평균의 N배 이상
            if change >= self.config.surge_threshold && volume_ratio >= self.config.volume_multiplier {
                let confidence = self.calculate_confidence(change, volume_ratio);
                return Some(Signal::buy(
                    tick.market.clone(),
                    confidence,
                    format!(
                        "Price surged {:.2}% with {:.1}x volume",
                        change * 100.0,
                        volume_ratio
                    ),
                ));
            }
        }

        None
    }

    /// 신호 신뢰도 계산
    fn calculate_confidence(&self, price_change: f64, volume_ratio: f64) -> f64 {
        let price_score = (price_change / self.config.surge_threshold).min(2.0) / 2.0;
        let volume_score = (volume_ratio / self.config.volume_multiplier).min(3.0) / 3.0;

        (price_score * 0.6 + volume_score * 0.4).min(1.0)
    }

    /// 독립 실행용 스폰 함수
    pub async fn spawn(
        config: TradingConfig,
        price_rx: mpsc::Receiver<PriceTick>,
    ) -> Result<mpsc::Receiver<Signal>> {
        let (detector, price_tx) = Self::new(config, price_rx);
        let (signal_tx, signal_rx) = mpsc::channel(1000);

        tokio::spawn(async move {
            let mut detector = detector.with_signal_channel(signal_tx);
            if let Err(e) = detector.run().await {
                tracing::warn!("Signal detector stopped: {}", e);
            }
        });

        Ok(signal_rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_history() {
        let mut history = PriceHistory::new(100);

        let tick1 = PriceTick::new("KRW-BTC".to_string(), 1000, 50000.0, 0.01, 1.0);
        let tick2 = PriceTick::new("KRW-BTC".to_string(), 2000, 52500.0, 0.05, 2.0);

        history.add(tick1);
        history.add(tick2);

        assert_eq!(history.ticks.len(), 2);
        assert!(history.get_latest("KRW-BTC").is_some());
    }

    #[test]
    fn test_price_change_calculation() {
        let mut history = PriceHistory::new(100);

        let base_time = chrono::Utc::now().timestamp_millis();

        // 5% 상승 시나리오
        history.add(PriceTick::new("KRW-BTC".to_string(), base_time - 3600000, 50000.0, 0.0, 1.0));
        history.add(PriceTick::new("KRW-BTC".to_string(), base_time, 52500.0, 0.05, 2.0));

        let change = history.calculate_price_change("KRW-BTC", 3600000 * 2);
        assert!(change.is_some());
        assert!((change.unwrap() - 0.05).abs() < 0.01);
    }

    #[test]
    fn test_confidence_calculation() {
        let config = TradingConfig::default();
        let detector = SignalDetector::new(config, {
            let (_, rx) = mpsc::channel(1);
            rx
        });

        // 10% 상승, 3배 거래량
        let confidence = detector.calculate_confidence(0.10, 3.0);
        assert!(confidence > 0.7);
    }
}
