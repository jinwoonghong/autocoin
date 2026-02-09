//! WebSocket 진단 스크립트
//!
//! 이 스크립트는 WebSocket 연결 문제를 진단합니다.

use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;
use tracing::{info, warn, error};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 로깅 설정
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("=== WebSocket 진단 시작 ===\n");

    // 1. 기본 연결 테스트
    println!("1. 기본 WebSocket 연결 테스트");
    let url = "wss://api.upbit.com/websocket/v1";

    let start = Instant::now();
    match tokio_tungstenite::connect_async(url).await {
        Ok((ws_stream, _response)) => {
            let duration = start.elapsed();
            println!("✅ 연결 성공 ({}ms)", duration.as_millis());
            println!("   응답: {:?}", _response);

            // 연결 닫기
            let mut ws_stream = ws_stream;
            ws_stream.close(None).await?;
        }
        Err(e) => {
            println!("❌ 연결 실패: {}", e);
            println!("\n가능한 원인:");
            println!("   - 네트워크 연결 문제");
            println!("   - 방화벽이 WebSocket 차단");
            println!("   - TLS 버전 불일치");
            println!("   - DNS 문제");
        }
    }

    // 2. 구독 메시지 테스트
    println!("\n2. 구독 메시지 테스트");

    match tokio_tungstenite::connect_async(url).await {
        Ok((ws_stream, _)) => {
            let (mut ws_sender, mut ws_receiver) = ws_stream.split();

            // 구독 메시지
            let subscription = r#"{
                "ticket": "diagnose-12345",
                "type": "ticker",
                "codes": ["KRW-BTC"]
            }"#;

            println!("📤 전송 메시지: {}", subscription);

            ws_sender.send(Message::Text(subscription.to_string())).await?;

            // 응답 대기
            let mut response_received = false;
            let mut timeout = tokio::time::sleep(tokio::time::Duration::from_secs(10));

            loop {
                tokio::select! {
                    Some(msg) = ws_receiver.next() => {
                        match msg {
                            Ok(Message::Text(text)) => {
                                response_received = true;
                                println!("✅ 수신 메시지: {}", text);

                                // 메시지 유형 확인
                                if text.contains("\"type\"") {
                                    if text.contains("\"error\"") {
                                        println!("❌ 서버 오류 응답");
                                    } else {
                                        println!("✅ 유효한 응답 수신");
                                    }
                                }
                                break;
                            }
                            Ok(Message::Binary(data)) => {
                                response_received = true;
                                println!("✅ 수신 바이너리 메시지 ({} bytes)", data.len());
                                break;
                            }
                            Ok(Message::Close(_)) => {
                                println!("❌ 연결이 닫혔습니다");
                                break;
                            }
                            Err(e) => {
                                error!("❌ 수신 오류: {}", e);
                                break;
                            }
                            _ => {}
                        }
                    }
                    _ = &mut timeout => {
                        println!("⏰ 응답 타임아웃");
                        break;
                    }
                }
            }

            ws_sender.send(Message::Close(None)).await?;

            if !response_received {
                println!("\n⚠️  응답이 수신되지 않음");
                println!("가능한 원인:");
                println!("   - 구독 메시지 형식 오류");
                println!("   - 마켓 코드 오류");
                println!("   - 서버 문제");
            }
        }
        Err(e) => {
            println!("❌ 연결 실패: {}", e);
        }
    }

    // 3. 네트워크 테스트
    println!("\n3. 네트워크 테스트");

    #[cfg(windows)]
    {
        use std::process::Command;

        println!("DNS 확인:");
        let output = Command::new("nslookup")
            .arg("api.upbit.com")
            .output();

        match output {
            Ok(result) => {
                println!("{}", String::from_utf8_lossy(&result.stdout));
            }
            Err(_) => {
                println!("❌ nslookup 실패");
            }
        }

        println!("Telnet 테스트:");
        let output = Command::new("telnet")
            .arg("api.upbit.com")
            .arg("443")
            .output();

        match output {
            Ok(result) => {
                println!("{}", String::from_utf8_lossy(&result.stdout));
            }
            Err(_) => {
                println!("❌ telnet 실패 (WebSocket 차단 가능성)");
            }
        }
    }

    #[cfg(unix)]
    {
        use std::process::Command;

        println!("Dig 테스트:");
        let output = Command::new("dig")
            .arg("api.upbit.com")
            .output();

        match output {
            Ok(result) => {
                println!("{}", String::from_utf8_lossy(&result.stdout));
            }
            Err(_) => {
                println!("❌ dig 실패");
            }
        }
    }

    println!("\n=== 진단 완료 ===");
    println!("위 결과를 바탕으로 문제를 확인하고 WEBSOCKET_FIX.md의 해결 방법을 참조하세요.");

    Ok(())
}