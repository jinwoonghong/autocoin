# Upbit WebSocket 연결 문제 해결 가이드

## 문제 요약

AutoCoin 트레이딩 봇이 Upbit WebSocket API에 연결하지 못하고 다음과 같은 오류를 발생시킵니다:

```
WARN autocoin::upbit::websocket: WebSocket connection closed
WARN autocoin::upbit::websocket: WebSocket error: Connection closed
ERROR autocoin::upbit::websocket: Max retry attempts reached, giving up
ERROR autocoin: Market monitor error: WebSocket error: Max retries exceeded
```

## 원인 분석

### 1. 구독 메시지 형식 문제
Upbit WebSocket은 특정 형식의 구독 메시지를 요구합니다. 메시지 형식이 올바르지 않으면 연결이 즉시 종료될 수 있습니다.

### 2. TLS/SSL 설정 문제
WebSocket URL `wss://api.upbit.com/websocket/v1`은 TLS 1.2 이상을 요구합니다. 일부 환경에서는 TLS 버전 불일치가 발생할 수 있습니다.

### 3. 네트워크 문제
프록시, 방화벽, 또는 네트워크 제한이 WebSocket 연결을 차단할 수 있습니다.

### 4. 시간 문제
연결 후 구독 메시지를 전송하는 시간이 너무 늦으면 서버가 연결을 종료할 수 있습니다.

## 해결 방법

### 1. 구독 메시지 형식 확인

Upbit WebSocket 요청 형식은 다음과 같아야 합니다:

```json
{
  "ticket": "유일한-식별자",
  "type": "ticker 또는 trade",
  "codes": ["KRW-BTC", "KRW-ETH"]
}
```

**중요 사항:**
- `ticket`: 각 연결마다 고유한 UUID 또는 고유한 문자열 사용
- `type`: `"ticker"` (실시간 호가) 또는 `"trade"` (체결 정보)
- `codes`: 마켓 코드 목록 (최대 100개)

### 2. 연결 재시도 로직 개선

수정된 WebSocket 코드는 다음을 개선했습니다:
- 연결 실패 시 상세한 로깅
- 구독 메시지 전송 확인
- 자동 재연결 로직 강화
- 메시지 수신 시 디버깅 로깅

### 3. 테스트 스크립트 실행

WebSocket 연결을 테스트하려면:

```bash
cargo run --bin test-websocket
```

이 스크립트는 5초 동안 WebSocket 연결을 테스트하고 수신되는 메시지를 출력합니다.

### 4. 환경별 설정

#### Windows 환경
```toml
[dependencies]
tokio-tungstenite = { version = "0.24", features = ["native-tls"] }
```

#### Linux/macOS 환경
```toml
[dependencies]
tokio-tungstenite = { version = "0.24", features = ["native-tls", "vendored"] }
```

### 5. 방화벽 및 프록시 확인

- 회사 네트워크에서는 방화벽이 WebSocket 연결을 차단할 수 있습니다
- 프록시를 사용하는 경우 `wss://` 프로토콜을 지원하는지 확인
- 포트 443 (TLS)이 열려 있는지 확인

## 대체 솔루션

### 1. HTTP 폴링 방식 사용
WebSocket 연결이 지속적으로 실패하는 경우, REST API를 사용한 폴링으로 전환할 수 있습니다:

```rust
// 예시: 5초마다 티커 정보 요청
async fn fetch_ticker_loop(markets: Vec<String>) {
    loop {
        for market in markets.iter() {
            if let Ok(ticker) = upbit_client.get_ticker(market).await {
                // 티커 처리
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
```

### 2. 다른 WebSocket 라이브러리 사용
`tokio-tungstenite` 대신 `futures-rs`를 직접 사용할 수 있습니다:

```toml
[dependencies]
futures = "0.3"
tokio = { version = "1.0", features = ["full"] }
```

## 모니터링 로깅

디버깅을 위해 다음 로그 레벨을 설정하세요:

```bash
RUST_LOG=info cargo run
```

또는 더 자세한 로그:

```bash
RUST_LOG=debug cargo run
```

## 지원

문제가 지속될 경우:
1. 테스트 스크립트 실행 결과 확인
2. 네트워크 연결 테스트 (`curl -I wss://api.upbit.com/websocket/v1`)
3. 방화벽/프록시 관리자 문의
4. Upbit API 상태 확인 (https://portal.upbit.com/api-status)

## 추가 자료

- [Upbit WebSocket API 문서](https://docs.upbit.com/docs/upbit-quotation-websocket)
- [Tokio Tungstenite 문서](https://docs.rs/tokio-tungstenite)
- [Rust WebSocket 예제](https://github.com/snapview/tokio-tungstenite/blob/master/examples/)