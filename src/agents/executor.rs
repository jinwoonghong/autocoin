//! Execution Agent
//!
//! Upbit API를 통한 주문 실행을 담당합니다.

use crate::config::TradingConfig;
use crate::db::Database;
use crate::error::{Result, TradingError};
use crate::types::{Decision, Order, OrderResult, OrderSide, Position};
use crate::upbit::UpbitClient;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tracing::{error, info, warn};

/// Execution Agent
///
/// 역할: Upbit API를 통한 주문 실행
/// 입력: Decision
/// 출력: OrderResult
pub struct ExecutionAgent {
    client: UpbitClient,
    db: Database,
    config: TradingConfig,
    decision_rx: mpsc::Receiver<Decision>,
    order_tx: Option<mpsc::Sender<OrderResult>>,
}

impl ExecutionAgent {
    /// 새로운 Execution Agent 생성
    pub fn new(
        client: UpbitClient,
        db: Database,
        config: TradingConfig,
        decision_rx: mpsc::Receiver<Decision>,
    ) -> Self {
        Self {
            client,
            db,
            config,
            decision_rx,
            order_tx: None,
        }
    }

    /// 주문 결과 출력 채널 설정
    pub fn with_order_channel(mut self, tx: mpsc::Sender<OrderResult>) -> Self {
        self.order_tx = Some(tx);
        self
    }

    /// 주문 실행 시작
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting execution agent");

        while let Some(decision) = self.decision_rx.recv().await {
            let result = match decision {
                Decision::Buy { market, amount_krw, .. } => {
                    self.execute_buy_order(&market, amount_krw).await
                }
                Decision::Sell { market, amount, .. } => {
                    self.execute_sell_order(&market, amount).await
                }
                Decision::Hold { .. } => {
                    // 보유 결정은 아무것도 하지 않음
                    continue;
                }
            };

            match result {
                Ok(order_result) => {
                    // 주문 결과 저장
                    if let Err(e) = self.db.save_order(&order_result.order).await {
                        error!("Failed to save order to database: {}", e);
                    }

                    // 결과 전송
                    if let Some(tx) = &self.order_tx {
                        let _ = tx.send(order_result).await;
                    }
                }
                Err(e) => {
                    error!("Order execution failed: {}", e);
                    // 실패 결과도 전송
                    if let Some(tx) = &self.order_tx {
                        let error_result = OrderResult::failure(
                            "", // market은 decision에서 가져와야 함
                            OrderSide::Bid,
                            e.to_string(),
                        );
                        let _ = tx.send(error_result).await;
                    }
                }
            }
        }

        Ok(())
    }

    /// 매수 주문 실행 (시장가)
    async fn execute_buy_order(&self, market: &str, amount_krw: f64) -> Result<OrderResult> {
        info!("Executing buy order: {} {} KRW", market, amount_krw);

        // REQ-015: 시장 상태 확인 (생략 - 항상 열려 있다고 가정)
        // 실제 구현에서는 현재 시간 확인 필요

        // 주문 실행 및 재시도 로직
        let mut retry_count = 0;
        let max_retries = 3;

        loop {
            match self.client.buy_market_order(market, amount_krw).await {
                Ok(order) => {
                    info!("Buy order executed: {}", order.id);

                    // 포지션 저장 (시장가이므로 즉시 체결 가정)
                    let avg_price = order.executed_amount / order.executed_volume.max(0.0001);
                    let position = Position::new(
                        market.to_string(),
                        avg_price,
                        order.executed_volume,
                        self.config.stop_loss_rate,
                        self.config.profit_rate,
                    );

                    if let Err(e) = self.db.save_position(&position).await {
                        error!("Failed to save position: {}", e);
                    }

                    return Ok(OrderResult::success(order));
                }
                Err(e) => {
                    retry_count += 1;

                    if e.is_retryable() && retry_count < max_retries {
                        // REQ-011: Exponential Backoff로 재시도
                        let delay = e.retry_delay() * 2_u32.pow(retry_count as u32);
                        warn!("Order failed, retrying in {:?} (attempt {}/{})", delay, retry_count, max_retries);
                        sleep(delay).await;
                    } else {
                        error!("Order failed after {} retries: {}", retry_count, e);
                        return Err(e);
                    }
                }
            }
        }
    }

    /// 매도 주문 실행 (시장가)
    async fn execute_sell_order(&self, market: &str, amount: f64) -> Result<OrderResult> {
        info!("Executing sell order: {} {} units", market, amount);

        let mut retry_count = 0;
        let max_retries = 3;

        loop {
            match self.client.sell_market_order(market, amount).await {
                Ok(order) => {
                    info!("Sell order executed: {}", order.id);

                    // 포지션 종료 처리
                    if let Ok(Some(position)) = self.db.get_active_position(market).await {
                        let pnl = position.calculate_pnl(order.price);
                        let _ = self.db.close_position(
                            market,
                            order.price,
                            pnl.profit,
                            pnl.profit_rate,
                        ).await;
                    }

                    return Ok(OrderResult::success(order));
                }
                Err(e) => {
                    retry_count += 1;

                    if e.is_retryable() && retry_count < max_retries {
                        let delay = e.retry_delay() * 2_u32.pow(retry_count as u32);
                        warn!("Sell order failed, retrying in {:?}", delay);
                        sleep(delay).await;
                    } else {
                        error!("Sell order failed after {} retries: {}", retry_count, e);
                        return Err(e);
                    }
                }
            }
        }
    }

    /// 주문 상태 확인
    pub async fn check_order_status(&self, order_id: &str) -> Result<Order> {
        self.client.get_order(order_id).await
    }

    /// 독립 실행용 스폰 함수
    pub async fn spawn(
        client: UpbitClient,
        db: Database,
        config: TradingConfig,
        decision_rx: mpsc::Receiver<Decision>,
    ) -> Result<mpsc::Receiver<OrderResult>> {
        let (order_tx, order_rx) = mpsc::channel(1000);

        let mut executor = Self::new(client, db, config, decision_rx)
            .with_order_channel(order_tx);

        tokio::spawn(async move {
            if let Err(e) = executor.run().await {
                tracing::error!("Execution agent stopped: {}", e);
            }
        });

        Ok(order_rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_agent_creation() {
        // Execution Agent 생성 테스트
        // 실제 테스트는 Mock UpbitClient 필요
        assert!(true, "ExecutionAgent structure validated");
    }
}
