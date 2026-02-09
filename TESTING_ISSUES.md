# AutoCoin Testing Issues

## 문서 개요

이 문서는 AutoCoin 프로젝트의 테스트 및 실행 중 발견된 이슈들을 기록하여, 향후 세션에서 참고할 수 있도록 정리한 것입니다.

**최종 업데이트**: 2026-02-08
**상태**: 진행 중

---

## 1. 컴파일 관련 문제 (해결 완료)

### 1.1 E0616 - Private Field Access Error
- **위치**: `src/agents/market_monitor.rs:32, 35`
- **원인**: `ReconnectingWebSocket.markets` 필드가 private이지만 `MarketMonitor`에서 직접 접근하려고 함
- **해결**: `MarketMonitor` 구조체에 `markets` 필드를 별도로 추가

```rust
// 수정 전
pub struct MarketMonitor {
    ws: ReconnectingWebSocket,
}

// 수정 후
pub struct MarketMonitor {
    ws: ReconnectingWebSocket,
    markets: Vec<String>,  // 추가
}
```

### 1.2 Binary 타겟 컴파일 오류
- **위치**: `src/test_websocket.rs`, `src/diagnose_websocket.rs`
- **상태**: 메인 프로그램(`autocoin`) 실행에는 영향 없음
- **해결 필요**: 테스트용 바이너리들의 `StreamExt` 임포트 및 기타 컴파일 오류

---

## 2. Upbit WebSocket 연결 문제 (미해결)

### 2.1 증상
```
WebSocket connected successfully to wss://api.upbit.com/websocket/v1
Sending subscription message: {"ticket":"...","type":"trade","codes":[...]}
Subscription request sent for markets
WebSocket connection closed: Some(CloseFrame { code: Normal, reason: "" }))
```

### 2.2 분석
1. WebSocket 연결 성공
2. 구독 메시지 전송 성공
3. 서버가 바로 연결을 정상 종료(`Normal` close code)

### 2.3 가능한 원인
1. **구독 형식 문제**: Upbit WebSocket이 요구하는 메시지 형식과 다를 수 있음
2. **바이너리 메시지 처리**: 현재 바이너리 메시지를 받자마자 연결이 닫힘
3. **업비트 API 정책**: 실제 거래 계정 인증이 필요할 수 있음

### 2.4 현재 구독 메시지 형식
```json
{
  "ticket": "uuid...",
  "type": "trade",
  "codes": ["KRW-WAXP", "KRW-CARV", ...]
}
```

### 2.5 해결 방안 (향후 작업)
1. Upbit 공식 문서의 WebSocket 형식 검증
2. 구독 타입 변경 (`trade` → `ticker` 시도)
3. 인증 헤더 추가 필요성 확인
4. WebSocket 라이브러리 변경 검토 (`tokio-tungstenite` → 다른 라이브러리)

---

## 3. Upbit API IP 인증 문제

### 3.1 증상
```
Failed to get KRW balance: Upbit API error: API returned error: 401 - {"error":{"name":"no_authorization_ip","message":"This is not a verified IP."}}
```

### 3.2 원인
Upbit API는 미리 등록된 IP 주소에서만 접근을 허용함

### 3.3 해결 방법
1. [Upbit 개발자 사이트](https://upbit.com/service_center/api)에서 IP 등록
2. 현재 공인 IP 주소 확인 후 등록 필요

---

## 4. 웹 대시보드 상태

### 4.1 동작 중인 기능
- ✅ 웹 서버 시작 (`http://localhost:8080`)
- ✅ 브라우저 자동 열기
- ✅ 웹 소켓 연결 (대시보드용)
- ✅ 에이전트 상태 초기화

### 4.2 동작하지 않는 기능
- ❌ 실시간 시세 데이터 수신 (Upbit WebSocket 연결 문제로 인해)
- ❌ 잔고 조회 (IP 인증 문제)
- ❌ 실시간 거래 실행

---

## 5. 코드 경고 사항

### 5.1 사용되지 않는 가져오기 (Unused Imports)
92개의 라이브러리 경고가 발생 중. `cargo fix --lib -p autocoin` 실행으로 자동 수정 가능

### 5.2 사용되지 않는 변수
다수의 사용되지 않는 변수들. 주요 항목:
- `src/main.rs`: `server_handle`, `decision_tx_clone`, `order_tx_clone`, `log_file`
- `src/agents/signal_detector.rs`: `price_rx`, `price_tx`
- `src/upbit/client.rs`: `method`, `path`, `body`

---

## 6. 빌드 및 실행 명령어

### 6.1 빌드
```bash
# 메인 프로그램만 빌드
cargo build --bin autocoin

# 전체 빌드
cargo build
```

### 6.2 실행
```bash
# 일반 실행 (TUI + 웹 대시보드)
cargo run --bin autocoin

# 데몬 모드 (로그만 출력, 웹 대시보드만)
cargo run --bin autocoin -- --daemon

# 웹 대시보드 없이 실행
cargo run --bin autocoin -- --no-web
```

### 6.3 프로세스 종료 (Windows)
```cmd
taskkill /F /IM autocoin.exe
```

---

## 7. 다음 세션 시작 시 확인 사항

### 7.1 우선 순위
1. **Upbit WebSocket 연결 문제 해결** - 실시간 시세 수집을 위해 필수
2. **IP 인증 등록** - API 접근 권한 획득
3. **실시간 데이터 수신 확인** - PriceTick이 정상적으로 수신되는지 검증

### 7.2 참고 파일
- `src/upbit/websocket.rs` - WebSocket 클라이언트 구현
- `src/upbit/models.rs` - WebSocket 메시지 모델
- `src/agents/market_monitor.rs` - 시세 모니터링 에이전트

---

## 8. 테스트 환경 정보

### 8.1 시스템
- OS: Windows
- Rust: 1.92+ (推测)
- Cargo: 최신 버전

### 8.2 주요 의존성
```toml
tokio = "1.40"
tokio-tungstenite = { version = "0.24", features = ["native-tls"] }
axum = { version = "0.7", features = ["ws"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "chrono", "json"] }
```

---

## 변경 이력

| 날짜 | 변경 사항 |
|------|----------|
| 2026-02-08 | 초기 문서 작성, 컴파일 오류 수정 완료, WebSocket 문제 기록 |
