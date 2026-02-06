//! Risk Manager Agent
//!
//! 포지션 및 리스크 관리, PnL 계산, 자동 손절/익절을 담당합니다.

use crate::config::TradingConfig;
use crate::db::Database;
use crate::error::Result;
use crate::types::{Decision, Position, PriceTick, PnL, RiskAction};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tracing::{info, warn};

/// Risk Manager Agent
///
/// 역할: 포지션 및 리스크 관리
/// 입력: 현재 포지션, 실시간 가격
/// 출력: RiskAction
pub struct RiskManager {
    config: TradingConfig,
    db: Database,
    price_rx: mpsc::Receiver<PriceTick>,
    risk_tx: Option<mpsc::Sender<Decision>>,
    current_position: Option<Position>,
}

impl RiskManager {
    /// 새로운 Risk Manager 생성
    pub fn new(
        config: TradingConfig,
        db: Database,
        price_rx: mpsc::Receiver<PriceTick>,
    ) -> Self {
        Self {
            config,
            db,
            price_rx,
            risk_tx: None,
            current_position: None,
        }
    }

    /// 리스크 액션 출력 채널 설정
    pub fn with_risk_channel(mut self, tx: mpsc::Sender<Decision>) -> Self {
        self.risk_tx = Some(tx);
        self
    }

    /// 포지션 설정
    pub fn set_position(&mut self, position: Option<Position>) {
        self.current_position = position;
    }

    /// 리스크 관리 시작
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting risk manager");

        // 시작 시 활성 포지션 로드
        self.load_active_position().await?;

        while let Some(tick) = self.price_rx.recv().await {
            // 포지션이 있으면 리스크 확인
            if let Some(position) = &self.current_position {
                if position.market == tick.market {
                    if let Some(action) = self.evaluate_risk(position, tick.trade_price).await? {
                        match action {
                            RiskAction::StopLoss { market, .. } => {
                                warn!("Stop loss triggered for {}", market);
                                self.send_sell_decision(&market, "Stop loss triggered".to_string()).await;
                            }
                            RiskAction::TakeProfit { market, .. } => {
                                info!("Take profit triggered for {}", market);
                                self.send_sell_decision(&market, "Take profit reached".to_string()).await;
                            }
                            RiskAction::Maintain => {
                                // 아무것도 하지 않음
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// 활성 포지션 로드
    async fn load_active_position(&mut self) -> Result<()> {
        let positions = self.db.get_all_active_positions().await?;

        // 단일 포지션만 지원
        if let Some(pos) = positions.first() {
            self.current_position = Some(pos.clone());
            info!("Loaded active position: {}", pos.market);
        }

        Ok(())
    }

    /// 리스크 평가
    async fn evaluate_risk(&self, position: &Position, current_price: f64) -> Result<Option<RiskAction>> {
        let pnl = position.calculate_pnl(current_price);

        // REQ-008: 손절가 도달 시 즉시 매도
        if position.should_stop_loss(current_price) {
            return Ok(Some(RiskAction::StopLoss {
                market: position.market.clone(),
                current_price,
                pnl_rate: pnl.profit_rate,
            }));
        }

        // REQ-007: 익절가 도달 시 매도
        if position.should_take_profit(current_price) {
            return Ok(Some(RiskAction::TakeProfit {
                market: position.market.clone(),
                current_price,
                pnl_rate: pnl.profit_rate,
            }));
        }

        Ok(Some(RiskAction::Maintain))
    }

    /// 매도 결정 전송
    async fn send_sell_decision(&self, market: &str, reason: String) {
        if let Some(tx) = &self.risk_tx {
            let amount = self.current_position
                .as_ref()
                .map(|p| p.amount)
                .unwrap_or(0.0);

            let decision = Decision::sell(market.to_string(), amount, reason);
            let _ = tx.send(decision).await;
        }
    }

    /// 포지션 업데이트 (주문 실행 후)
    pub async fn update_position(&mut self, position: Option<Position>) {
        self.current_position = position;
    }

    /// PnL 조회
    pub fn get_current_pnl(&self) -> Option<PnL> {
        // 실시간 PnL은 현재 가격이 필요
        self.current_position.as_ref().map(|p| {
            PnL {
                cost: p.entry_price * p.amount,
                value: p.entry_price * p.amount, // 현재 가격 필요
                profit: 0.0,
                profit_rate: 0.0,
            }
        })
    }

    /// 독립 실행용 스폰 함수
    pub async fn spawn(
        config: TradingConfig,
        db: Database,
        price_rx: mpsc::Receiver<PriceTick>,
    ) -> Result<mpsc::Receiver<Decision>> {
        let (risk_tx, risk_rx) = mpsc::channel(1000);

        let mut manager = Self::new(config, db, price_rx)
            .with_risk_channel(risk_tx);

        tokio::spawn(async move {
            if let Err(e) = manager.run().await {
                tracing::warn!("Risk manager stopped: {}", e);
            }
        });

        Ok(risk_rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stop_loss_detection() {
        let config = TradingConfig::default();
        let position = Position::new("KRW-BTC".to_string(), 100000.0, 0.001, 0.05, 0.1);

        // 손절가는 95,000
        assert!(position.should_stop_loss(94000.0));
        assert!(!position.should_stop_loss(96000.0));
    }

    #[tokio::test]
    async fn test_take_profit_detection() {
        let config = TradingConfig::default();
        let position = Position::new("KRW-BTC".to_string(), 100000.0, 0.001, 0.05, 0.1);

        // 익절가는 110,000
        assert!(position.should_take_profit(110000.0));
        assert!(!position.should_take_profit(109000.0));
    }

    #[test]
    fn test_pnl_calculation() {
        let position = Position::new("KRW-BTC".to_string(), 100000.0, 0.001, 0.05, 0.1);
        let pnl = position.calculate_pnl(110000.0);

        assert_eq!(pnl.cost, 100.0); // 100,000 * 0.001
        assert_eq!(pnl.value, 110.0); // 110,000 * 0.001
        assert!((pnl.profit - 10.0).abs() < 0.01); // 110 - 100
        assert!((pnl.profit_rate - 0.10).abs() < 0.01); // 10%
    }
}
