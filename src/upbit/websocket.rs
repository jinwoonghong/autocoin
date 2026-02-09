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

        // Upbit WebSocket 연결
        info!("Connecting to WebSocket: {}", self.url);
        let ws_stream = tokio_tungstenite::connect_async(&self.url)
            .await
            .map_err(|e| {
                error!("WebSocket connection failed: {}", e);
                TradingError::WebSocket(format!("Connection failed: {}", e))
            })?
            .0;

        info!("WebSocket connected successfully to {}", self.url);

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // 구독 메시지 전송
        let sub_json = serde_json::to_string(&subscription)?;
        info!("Sending subscription message: {}", sub_json);

        ws_sender
            .send(Message::Text(sub_json))
            .await
            .map_err(|e| {
                error!("Failed to send subscription message: {}", e);
                TradingError::WebSocket(format!("Send subscription failed: {}", e))
            })?;

        info!("Subscription request sent for markets");

        // 메시지 수신 루프
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    debug!("Received text message: {}", text);
                    self.handle_message(&text, &tx).await?;
                }
                Ok(Message::Binary(data)) => {
                    // Upbit은 바이너리 메시지를 보냄
                    // 일반적으로 JSON 텍스트가 Binary 메시지로 옴
                    debug!("Received binary message, size: {} bytes", data.len());
                    if data.is_empty() {
                        warn!("Received empty binary message");
                        continue;
                    }

                    // 첫 바이트 확인 (123 = '{' = JSON 시작)
                    let first_byte = data[0];

                    if first_byte == 123 || first_byte == 91 {
                        // JSON 텍스트 ({ 또는 [) -> 전체 데이터를 UTF-8로 처리
                        if let Ok(text) = String::from_utf8(data) {
                            if let Err(e) = self.handle_message(&text, &tx).await {
                                warn!("Failed to handle JSON message: {}", e);
                            }
                        } else {
                            warn!("Failed to parse binary data as UTF-8");
                        }
                    } else if first_byte == 0 {
                        // 압축 플래그 형식 (0=plain text, 1=deflate)
                        let payload = &data[1..];
                        if let Ok(text) = String::from_utf8(payload.to_vec()) {
                            if let Err(e) = self.handle_message(&text, &tx).await {
                                warn!("Failed to handle message: {}", e);
                            }
                        }
                    } else {
                        // deflate 압축 시도
                        if let Ok(decompressed) = self.decompress(data) {
                            if let Err(e) = self.handle_message(&decompressed, &tx).await {
                                warn!("Failed to handle decompressed message: {}", e);
                            }
                        } else {
                            warn!("Failed to decompress binary message, first_byte: {}", first_byte);
                        }
                    }
                }
                Ok(Message::Close(close_frame)) => {
                    warn!("WebSocket connection closed: {:?}", close_frame);
                    return Err(TradingError::WebSocket("Connection closed".to_string()));
                }
                Ok(Message::Ping(data)) => {
                    debug!("Received ping, sending pong");
                    ws_sender
                        .send(Message::Pong(data))
                        .await
                        .map_err(|e| TradingError::WebSocket(e.to_string()))?;
                }
                Ok(Message::Pong(_)) => {
                    debug!("Received pong");
                }
                Ok(Message::Frame(_)) => {
                    // Raw frame, ignore
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
                WsMessage::Trade(trade) => {
                    debug!("Processing trade message for market: {}", trade.code);
                    PriceTick::from(trade)
                }
                WsMessage::Ticker(ticker) => {
                    debug!("Processing ticker message for market: {}", ticker.code);
                    PriceTick::from(ticker)
                }
            };

            // 채널로 전송 (수신자가 없으면 무시)
            if let Err(_) = tx.send(tick).await {
                debug!("Price tick channel closed, skipping");
            }
        } else {
            // 메시지가 파싱되지 않을 경우에도 로그 기록
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
    retry_count: u32,
}

impl ReconnectingWebSocket {
    pub fn new(markets: Vec<String>) -> Self {
        Self {
            ws: UpbitWebSocket::new(None),
            markets,
            max_retries: 5,
            retry_delay: std::time::Duration::from_secs(5),
            retry_count: 0,
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
    pub async fn run(mut self, tx: mpsc::Sender<PriceTick>) -> Result<()> {
        loop {
            // 재시도 카운트 리셋 (성공 시)
            self.retry_count = 0;

            match self.ws.connect_and_subscribe(self.markets.clone(), tx.clone()).await {
                Ok(_) => {
                    info!("WebSocket connection completed successfully");
                    // 연결이 정상 종료되면 재시도 로직 계속
                }
                Err(e) => {
                    self.retry_count += 1;

                    if self.retry_count >= self.max_retries {
                        error!("Max retry attempts ({}) reached, giving up. Last error: {}", self.max_retries, e);
                        return Err(TradingError::WebSocket(
                            "Max retries exceeded".to_string(),
                        ));
                    }

                    warn!("WebSocket error (attempt {}/{}): {}", self.retry_count, self.max_retries, e);
                    info!("Retrying in {:?} (attempt {}/{})", self.retry_delay, self.retry_count, self.max_retries);
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
        assert!(json.contains("KRW-ETH"));
        assert!(json.contains("test-ticket"));
        println!("Subscription JSON: {}", json);
    }

    #[tokio::test]
    async fn test_websocket_url() {
        let ws = UpbitWebSocket::new(None);
        assert_eq!(ws.url, "wss://api.upbit.com/websocket/v1");

        let ws_custom = UpbitWebSocket::new(Some("wss://custom.example.com/ws".to_string()));
        assert_eq!(ws_custom.url, "wss://custom.example.com/ws");
    }

    #[tokio::test]
    async fn test_reconnecting_websocket_creation() {
        let markets = vec!["KRW-BTC".to_string()];
        let ws = ReconnectingWebSocket::new(markets);
        assert_eq!(ws.markets, vec!["KRW-BTC".to_string()]);
        assert_eq!(ws.max_retries, 5);
        assert_eq!(ws.retry_delay, std::time::Duration::from_secs(5));
        assert_eq!(ws.retry_count, 0);
    }

    #[tokio::test]
    async fn test_message_parsing() {
        // Test trade message parsing
        let trade_json = r#"{
            "type": "trade",
            "code": "KRW-BTC",
            "timestamp": 1640995200000,
            "trade_price": 50000000.0,
            "change_price": 1000000.0,
            "change_rate": 0.02,
            "trade_volume": 0.001,
            "ask_bid": "bid"
        }"#;

        let ws_msg: WsMessage = serde_json::from_str(trade_json).unwrap();
        match ws_msg {
            WsMessage::Trade(trade) => {
                assert_eq!(trade.code, "KRW-BTC");
                assert_eq!(trade.trade_price, 50000000.0);
            }
            _ => panic!("Expected trade message"),
        }

        // Test ticker message parsing
        let ticker_json = r#"{
            "type": "ticker",
            "code": "KRW-BTC",
            "opening_price": 49000000.0,
            "high_price": 51000000.0,
            "low_price": 48500000.0,
            "trade_price": 50000000.0,
            "prev_closing_price": 49000000.0,
            "change": "RISE",
            "change_price": 1000000.0,
            "change_rate": 0.02,
            "trade_volume": 1.0,
            "acc_trade_volume": 100.0,
            "acc_trade_price": 5000000000.0,
            "timestamp": 1640995200000
        }"#;

        let ws_msg: WsMessage = serde_json::from_str(ticker_json).unwrap();
        match ws_msg {
            WsMessage::Ticker(ticker) => {
                assert_eq!(ticker.code, "KRW-BTC");
                assert_eq!(ticker.trade_price, 50000000.0);
            }
            _ => panic!("Expected ticker message"),
        }
    }
}
