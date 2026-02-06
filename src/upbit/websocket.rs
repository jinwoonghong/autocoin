//! Upbit WebSocket client

use super::models::{WsMessage, WsSubscription};
use crate::error::{Result, TradingError};
use crate::types::PriceTick;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, warn};

/// Upbit WebSocket 클라이언트
pub struct UpbitWebSocket {
    url: String,
}

impl UpbitWebSocket {
    /// 새로운 WebSocket 클라이언트 생성
    pub fn new(url: Option<String>) -> Self {
        Self {
            url: url.unwrap_or_else(|| "wss://api.upbit.com/websocket/v1".to_string()),
        }
    }

    /// WebSocket 연결 및 구독 시작
    ///
    /// `markets`: 구독할 마켓 목록 (예: ["KRW-BTC", "KRW-ETH"])
    /// `tx`: 가격 틱을 전송할 채널
    pub async fn connect_and_subscribe(
        &self,
        markets: Vec<String>,
        tx: mpsc::Sender<PriceTick>,
    ) -> Result<()> {
        let ticket = uuid::Uuid::new_v4().to_string();
        let subscription = WsSubscription::trade(ticket.clone(), markets);

        let ws_stream = tokio_tungstenite::connect_async(&self.url)
            .await
            .map_err(|e| TradingError::WebSocket(e.to_string()))?
            .0;

        info!("WebSocket connected to {}", self.url);

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // 구독 메시지 전송
        let sub_json = serde_json::to_string(&subscription)?;
        ws_sender
            .send(Message::Text(sub_json))
            .await
            .map_err(|e| TradingError::WebSocket(e.to_string()))?;

        info!("Subscribed to markets");

        // 메시지 수신 루프
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    self.handle_message(&text, &tx).await?;
                }
                Ok(Message::Binary(data)) => {
                    // Upbit은 압축된 바이너리 메시지를 보낼 수 있음
                    if let Ok(decompressed) = self.decompress(data) {
                        self.handle_message(&decompressed, &tx).await?;
                    }
                }
                Ok(Message::Close(_)) => {
                    warn!("WebSocket connection closed");
                    return Err(TradingError::WebSocket("Connection closed".to_string()));
                }
                Ok(Message::Ping(data)) => {
                    ws_sender
                        .send(Message::Pong(data))
                        .await
                        .map_err(|e| TradingError::WebSocket(e.to_string()))?;
                }
                Ok(Message::Pong(_)) => {
                    debug!("Received pong");
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    return Err(TradingError::WebSocket(e.to_string()));
                }
            }
        }

        Ok(())
    }

    /// 메시지 처리
    async fn handle_message(&self, text: &str, tx: &mpsc::Sender<PriceTick>) -> Result<()> {
        // Upbit WebSocket 메시지 파싱
        if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(text) {
            let tick = match ws_msg {
                WsMessage::Trade(trade) => PriceTick::from(trade),
                WsMessage::Ticker(ticker) => PriceTick::from(ticker),
            };

            // 채널로 전송 (수신자가 없으면 무시)
            if let Err(_) = tx.send(tick).await {
                debug!("Price tick channel closed, skipping");
            }
        } else {
            debug!("Failed to parse WebSocket message: {}", text);
        }

        Ok(())
    }

    /// 압축된 메시지 해제
    fn decompress(&self, data: Vec<u8>) -> Result<String> {
        // Upbit은 deflate 압축을 사용할 수 있음
        use flate2::read::DeflateDecoder;
        use std::io::Read;

        let mut decoder = DeflateDecoder::new(&data[..]);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| TradingError::WebSocket(format!("Decompression failed: {}", e)))?;

        String::from_utf8(decompressed)
            .map_err(|e| TradingError::WebSocket(format!("UTF-8 conversion failed: {}", e)))
    }
}

/// 자동 재연결이 있는 WebSocket 클라이언트
pub struct ReconnectingWebSocket {
    ws: UpbitWebSocket,
    markets: Vec<String>,
    max_retries: u32,
    retry_delay: std::time::Duration,
}

impl ReconnectingWebSocket {
    pub fn new(markets: Vec<String>) -> Self {
        Self {
            ws: UpbitWebSocket::new(None),
            markets,
            max_retries: 5,
            retry_delay: std::time::Duration::from_secs(5),
        }
    }

    pub fn with_max_retries(mut self, max: u32) -> Self {
        self.max_retries = max;
        self
    }

    pub fn with_retry_delay(mut self, delay: std::time::Duration) -> Self {
        self.retry_delay = delay;
        self
    }

    /// 자동 재연결로 실행
    pub async fn run(self, tx: mpsc::Sender<PriceTick>) -> Result<()> {
        let mut retry_count = 0;

        loop {
            match self.ws.connect_and_subscribe(self.markets.clone(), tx.clone()).await {
                Ok(_) => {
                    // 정상 종료 시 재시도 카운트 리셋
                    retry_count = 0;
                }
                Err(e) => {
                    warn!("WebSocket error: {}", e);
                    retry_count += 1;

                    if retry_count >= self.max_retries {
                        error!("Max retry attempts reached, giving up");
                        return Err(TradingError::WebSocket(
                            "Max retries exceeded".to_string(),
                        ));
                    }

                    info!("Retrying in {:?} (attempt {}/{})", self.retry_delay, retry_count, self.max_retries);
                    tokio::time::sleep(self.retry_delay).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_subscription_format() {
        let sub = WsSubscription::trade("test-ticket".to_string(), vec![
            "KRW-BTC".to_string(),
            "KRW-ETH".to_string(),
        ]);

        let json = serde_json::to_string(&sub).unwrap();
        assert!(json.contains("trade"));
        assert!(json.contains("KRW-BTC"));
    }
}
