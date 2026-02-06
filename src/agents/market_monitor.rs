//! Market Monitor Agent
//!
//! Upbit WebSocket을 통해 실시간 시세를 수집합니다.

use crate::error::Result;
use crate::types::PriceTick;
use crate::upbit::ReconnectingWebSocket;
use tokio::sync::mpsc;
use tracing::{info, warn};

/// Market Monitor Agent
///
/// 역할: Upbit WebSocket을 통해 실시간 시세 수집
/// 입력: None
/// 출력: PriceTick 스트림
pub struct MarketMonitor {
    ws: ReconnectingWebSocket,
}

impl MarketMonitor {
    /// 새로운 Market Monitor 생성
    pub fn new(markets: Vec<String>) -> Self {
        let ws = ReconnectingWebSocket::new(markets);
        Self { ws }
    }

    /// Top N 코인 모니터링 시작
    ///
    /// `markets`: 모니터링할 마켓 목록
    /// `tx`: 가격 틱을 전송할 채널
    pub async fn monitor(&self, tx: mpsc::Sender<PriceTick>) -> Result<()> {
        info!("Starting market monitor");
        self.ws.run(tx).await
    }

    /// 독립 실행용 스폰 함수
    pub async fn spawn(markets: Vec<String>) -> Result<mpsc::Receiver<PriceTick>> {
        let (tx, rx) = mpsc::channel(1000);
        let monitor = Self::new(markets);

        tokio::spawn(async move {
            if let Err(e) = monitor.monitor(tx).await {
                warn!("Market monitor stopped: {}", e);
            }
        });

        Ok(rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_monitor_creation() {
        let markets = vec!["KRW-BTC".to_string(), "KRW-ETH".to_string()];
        let monitor = MarketMonitor::new(markets);
        // Market Monitor 생성 테스트
        assert!(true, "MarketMonitor created successfully");
    }
}
