//! WebSocket 테스트 스크립트
//!
//! 이 스크립트는 Upbit WebSocket 연결을 테스트합니다.

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;
use serde_json::json;
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅 설정
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let url = "wss://api.upbit.com/websocket/v1";
    let markets = vec!["KRW-BTC".to_string()];

    info!("Connecting to Upbit WebSocket: {}", url);

    // WebSocket 연결
    let (ws_stream, _) = tokio_tungstenite::connect_async(url)
        .await
        .map_err(|e| {
            error!("WebSocket 연결 실패: {}", e);
            format!("WebSocket 연결 실패: {}", e)
        })?;

    info!("WebSocket 연결 성공");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // 구독 메시지 생성 및 전송
    let subscription = json!({
        "ticket": "test-ticket-123",
        "type": "ticker",
        "codes": markets
    });

    let sub_json = subscription.to_string();
    info!("구독 메시지 전송: {}", sub_json);

    ws_sender.send(Message::Text(sub_json)).await?;

    // 메시지 수신 (5초 동안 테스트)
    let mut message_count = 0;
    let mut timeout = tokio::time::sleep(tokio::time::Duration::from_secs(5));

    loop {
        tokio::select! {
            Some(msg) = ws_receiver.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        message_count += 1;
                        info!("수신 메시지 #{}: {}", message_count, text);
                    }
                    Ok(Message::Binary(data)) => {
                        message_count += 1;
                        info!("수신 바이너리 메시지 #{}: {} bytes", message_count, data.len());
                    }
                    Ok(Message::Close(_)) => {
                        warn!("WebSocket 연결이 닫혔습니다");
                        break;
                    }
                    Ok(Message::Ping(data)) => {
                        info!("Ping 수신, Pong 전송");
                        ws_sender.send(Message::Pong(data)).await?;
                    }
                    Ok(Message::Pong(_)) => {
                        info!("Pong 수신");
                    }
                    Err(e) => {
                        error!("WebSocket 오류: {}", e);
                        break;
                    }
                    _ => {
                        info!("기타 메시지 수신");
                    }
                }
            }
            _ = &mut timeout => {
                info!("테스트 타임아웃: {}개 메시지 수신", message_count);
                break;
            }
        }
    }

    info!("WebSocket 테스트 완료. 총 {}개 메시지 수신", message_count);

    // 연결 종료
    ws_sender.send(Message::Close(None)).await?;

    Ok(())
}