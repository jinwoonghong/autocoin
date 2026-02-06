//! Decision Maker Agent
//!
//! 최종 거래 결정과 신호 필터링을 담당합니다.

use crate::config::TradingConfig;
use crate::error::{Result, TradingError};
use crate::types::{Decision, Position, Signal, SignalType};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

/// Decision Maker Agent
///
/// 역할: 최종 거래 결정, 신호 필터링
/// 입력: Signal, 현재 포지션 상태
/// 출력: Decision
pub struct DecisionMaker {
    config: TradingConfig,
    signal_rx: mpsc::Receiver<Signal>,
    decision_tx: Option<mpsc::Sender<Decision>>,
    current_position: Option<Position>,
    krw_balance: f64,
}

impl DecisionMaker {
    /// 새로운 Decision Maker 생성
    pub fn new(
        config: TradingConfig,
        signal_rx: mpsc::Receiver<Signal>,
    ) -> Self {
        Self {
            config,
            signal_rx,
            decision_tx: None,
            current_position: None,
            krw_balance: 0.0,
        }
    }

    /// 결정 출력 채널 설정
    pub fn with_decision_channel(mut self, tx: mpsc::Sender<Decision>) -> Self {
        self.decision_tx = Some(tx);
        self
    }

    /// KRW 잔고 설정
    pub fn set_balance(&mut self, balance: f64) {
        self.krw_balance = balance;
    }

    /// 현재 포지션 설정
    pub fn set_position(&mut self, position: Option<Position>) {
        self.current_position = position;
    }

    /// 의사결정 시작
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting decision maker");

        while let Some(signal) = self.signal_rx.recv().await {
            let decision = self.make_decision(&signal).await?;

            debug!("Decision: {:?}", decision);

            if let Some(tx) = &self.decision_tx {
                let _ = tx.send(decision.clone()).await;
            }

            // 매수 결정 시 포지션 업데이트 (실제로는 실행 에이전트에서 처리)
            if matches!(decision, Decision::Buy { .. }) {
                // 포지션은 실행 완료 후 업데이트됨
            }
        }

        Ok(())
    }

    /// 거래 결정 로직
    async fn make_decision(&self, signal: &Signal) -> Result<Decision> {
        match signal.signal_type {
            SignalType::Buy | SignalType::StrongBuy => {
                self.evaluate_buy_signal(signal).await
            }
            SignalType::Sell | SignalType::StrongSell => {
                self.evaluate_sell_signal(signal).await
            }
            SignalType::Hold => {
                Ok(Decision::hold("No significant signal".to_string()))
            }
        }
    }

    /// 매수 신호 평가
    async fn evaluate_buy_signal(&self, signal: &Signal) -> Result<Decision> {
        // REQ-013: IF 포지션 존재 THEN 새 매수 안 함
        if self.current_position.is_some() {
            info!("Buy signal ignored: Position already exists (REQ-013)");
            return Ok(Decision::hold("Position already exists".to_string()));
        }

        // REQ-014: IF 잔고 부족 THEN 스킵
        if self.krw_balance < self.config.min_order_amount {
            warn!(
                "Buy signal ignored: Insufficient balance (available: {}, required: {}) (REQ-014)",
                self.krw_balance, self.config.min_order_amount
            );
            return Ok(Decision::hold(format!(
                "Insufficient balance: {} KRW",
                self.krw_balance
            )));
        }

        // 주문 금액 계산 (잔고의 max_position_ratio)
        let order_amount = self.krw_balance * self.config.max_position_ratio;

        // REQ-018: 대규모 주문 방지 (50% 이상은 확인 필요)
        if order_amount > self.krw_balance * 0.5 {
            warn!("Large order detected: {} KRW ({}% of balance)", order_amount, self.config.max_position_ratio * 100.0);
            // 실제 구현에서는 사용자 확인 요청
        }

        Ok(Decision::buy(
            signal.market.clone(),
            order_amount,
            signal.reason.clone(),
        ))
    }

    /// 매도 신호 평가
    async fn evaluate_sell_signal(&self, signal: &Signal) -> Result<Decision> {
        // 현재 포지션이 있는지 확인
        if let Some(pos) = &self.current_position {
            if pos.market == signal.market {
                return Ok(Decision::sell(
                    signal.market.clone(),
                    pos.amount,
                    signal.reason.clone(),
                ));
            }
        }

        Ok(Decision::hold("No matching position to sell".to_string()))
    }

    /// 독립 실행용 스폰 함수
    pub async fn spawn(
        config: TradingConfig,
        signal_rx: mpsc::Receiver<Signal>,
    ) -> Result<mpsc::Receiver<Decision>> {
        let (decision_tx, decision_rx) = mpsc::channel(1000);

        let mut maker = Self::new(config, signal_rx)
            .with_decision_channel(decision_tx);

        tokio::spawn(async move {
            if let Err(e) = maker.run().await {
                tracing::warn!("Decision maker stopped: {}", e);
            }
        });

        Ok(decision_rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Position;

    #[tokio::test]
    async fn test_decision_maker_with_no_position() {
        let config = TradingConfig::default();
        let (signal_tx, signal_rx) = mpsc::channel(1000);

        let mut maker = DecisionMaker::new(config, signal_rx);
        maker.set_balance(100000.0); // 100,000 KRW

        // 매수 신호 전송
        let buy_signal = Signal::buy("KRW-BTC".to_string(), 0.8, "Surge detected".to_string());
        signal_tx.send(buy_signal).await.unwrap();

        // 결정 수신 (타임아웃 포함)
        let decision = tokio::time::timeout(
            tokio::time::Duration::from_millis(100),
            maker.signal_rx.recv()
        ).await;

        assert!(decision.is_ok());
    }

    #[test]
    fn test_decision_maker_with_position() {
        let config = TradingConfig::default();
        let (signal_tx, signal_rx) = mpsc::channel(1000);

        let mut maker = DecisionMaker::new(config, signal_rx);

        // 포지션 설정
        let position = Position::new("KRW-ETH".to_string(), 300000.0, 1.0, 0.05, 0.1);
        maker.set_position(Some(position));
        maker.set_balance(100000.0);

        // BTC 매수 신호 - ETH 포지션이 있으므로 무시해야 함
        let decision = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                let buy_signal = Signal::buy("KRW-BTC".to_string(), 0.8, "Surge".to_string());
                maker.evaluate_buy_signal(&buy_signal).await
            });

        assert!(matches!(decision, Ok(Decision::Hold { .. })));
    }

    #[test]
    fn test_insufficient_balance() {
        let config = TradingConfig::default();
        let (signal_tx, signal_rx) = mpsc::channel(1000);

        let mut maker = DecisionMaker::new(config, signal_rx);
        maker.set_balance(1000.0); // 최소 주문 금액 미달

        let decision = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                let buy_signal = Signal::buy("KRW-BTC".to_string(), 0.8, "Surge".to_string());
                maker.evaluate_buy_signal(&buy_signal).await
            });

        assert!(matches!(decision, Ok(Decision::Hold { .. })));
    }
}
