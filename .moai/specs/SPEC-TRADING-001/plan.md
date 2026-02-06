# Implementation Plan: SPEC-TRADING-001

**TAG BLOCK**: `SPEC-ID: SPEC-TRADING-001`

---

## 1. Milestones (마일스톤)

### Milestone 1: Foundation (기반) - Priority High

Rust 프로젝트 구조와 기본 인프라를 구축합니다.

- **1.1** 프로젝트 초기화 (Cargo.toml, 디렉터리 구조)
- **1.2** 환경 변수 로더 (.env 설정)
- **1.3** 로깅 시스템 (tracing crate)
- **1.4** SQLite 데이터베이스 스키마 정의
- **1.5** 기본 에러 타입 정의

### Milestone 2: Upbit API Client - Priority High

Upbit API와 통신하는 클라이언트를 구현합니다.

- **2.1** REST API 클라이언트 구현
- **2.2** JWT 인증 구현
- **2.3** WebSocket 클라이언트 구현
- **2.4** Rate Limiter 구현
- **2.5** API 모의 서버 (테스트용)

### Milestone 3: Market Monitor Agent - Priority High

실시간 시계 모니터링을 구현합니다.

- **3.1** WebSocket 연결 관리
- **3.2** 시계 데이터 파싱
- **3.3** Top 20 코인 필터링
- **3.4** 채널 구독 관리
- **3.5** 연결 복구 로직

### Milestone 4: Signal Detector Agent - Priority High

모멘텀/서징 감지 로직을 구현합니다.

- **4.1** 가격 데이터 저장소 (In-memory + DB)
- **4.2** 이동평균 계산
- **4.3** 거래량 급증 감지
- **4.4** 가격 상승률 계산
- **4.5** 신호 신뢰도 점수 계산

### Milestone 5: Decision Maker Agent - Priority High

거래 결정 로직을 구현합니다.

- **5.1** 상태 머신 (No Position → In Position)
- **5.2** 신호 필터링 로직
- **5.3** 포지션 충돌 방지
- **5.4** 잔고 확인 로직
- **5.5** 결정 캐싱

### Milestone 6: Execution Agent - Priority High

주문 실행을 구현합니다.

- **6.1** 매수 주문 제출
- **6.2** 매도 주문 제출
- **6.3** 주문 상태 조회
- **6.4** 체결 확인
- **6.5** 에러 처리 및 재시도

### Milestone 7: Risk Manager Agent - Priority High

리스크 관리를 구현합니다.

- **7.1** 손절/익절 가격 계산
- **7.2** 실시간 PnL 계산
- **7.3** 자동 손절 로직
- **7.4** 자동 익절 로직
- **7.5** 일일 손실 제한 (선택)

### Milestone 8: Notification Agent - Priority Medium

Discord 알림을 구현합니다.

- **8.1** Webhook 클라이언트
- **8.2** Embed 메시지 포맷팅
- **8.3** 알림 타입별 색상/형식
- **8.4** 전송 실패 처리
- **8.5** 알림 필터링 (너무 많은 알림 방지)

### Milestone 9: State Persistence - Priority Medium

상태 지속성을 구현합니다.

- **9.1** SQLite 마이그레이션
- **9.2** 포지션 저장/복구
- **9.3** 거래 기록 저장
- **9.4** 시계 데이터 저장
- **9.5** 시작 시 상태 복원

### Milestone 10: Testing & Quality - Priority High

테스트와 품질 보증을 수행합니다.

- **10.1** 단위 테스트 (각 에이전트)
- **10.2** 통합 테스트 (Upbit Mock 사용)
- **10.3** 시뮬레이션 테스트
- **10.4** 스트레스 테스트
- **10.5** 보안 감사 (API Key 로깅 확인)

### Milestone 11: Documentation - Priority Low

문서화를 작성합니다.

- **11.1** README 작성
- **11.2** API 문서
- **11.3** 배포 가이드
- **11.4** 운영 가이드
- **11.5** 트러블슈팅 가이드

---

## 2. Technical Approach (기술적 접근)

### 2.1 Project Structure (프로젝트 구조)

```
autocoin/
├── Cargo.toml
├── .env.example
├── .gitignore
├── data/
│   └── trading.db
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config/
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── error/
│   │   ├── mod.rs
│   │   └── types.rs
│   ├── db/
│   │   ├── mod.rs
│   │   ├── schema.rs
│   │   └── models.rs
│   ├── agents/
│   │   ├── mod.rs
│   │   ├── market_monitor.rs
│   │   ├── signal_detector.rs
│   │   ├── decision_maker.rs
│   │   ├── executor.rs
│   │   ├── risk_manager.rs
│   │   └── notification.rs
│   ├── upbit/
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   ├── websocket.rs
│   │   └── models.rs
│   ├── discord/
│   │   ├── mod.rs
│   │   └── webhook.rs
│   ├── strategy/
│   │   ├── mod.rs
│   │   └── momentum.rs
│   └── types/
│       ├── mod.rs
│       └── trading.rs
└── tests/
    ├── integration/
    │   ├── trading_flow_test.rs
    │   └── api_test.rs
    └── unit/
        ├── signal_test.rs
        └── risk_test.rs
```

### 2.2 Dependencies (의존성)

```toml
[dependencies]
# Async Runtime
tokio = { version = "1.40", features = ["full"] }
tokio-tungstenite = "0.24"

# Database
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }

# Web
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Authentication
jsonwebtoken = "9.3"
hmac = "0.12"
sha2 = "0.10"

# Error Handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tracing-appender = "0.2"

# Configuration
dotenvy = "0.15"
config = "0.14"

# Time
chrono = "0.4"

# Concurrency
async-trait = "0.1"

# Discord Notification
serde_urlencoded = "0.7"

[dev-dependencies]
mockito = "1.5"
tokio-test = "0.4"
wiremock = "0.6"
```

### 2.3 Architecture Patterns (아키텍처 패턴)

#### Actor Model with Channels

각 에이전트는 독립적인 태스크로 실행되며, 채널을 통해 통신합니다.

```rust
// 메시지 버스 아키텍처
struct MessageBus {
    price_rx: mpsc::Receiver<PriceTick>,
    signal_tx: mpsc::Sender<Signal>,
    decision_tx: mpsc::Sender<Decision>,
    order_rx: mpsc::Receiver<OrderResult>,
}

// 각 에이전트는 독립적인 tokio::task
async fn run_market_monitor(config: Config) -> Result<()> {
    let (tx, rx) = mpsc::channel(1000);
    // WebSocket 연결 및 메시지 루프
}

async fn run_signal_detector(
    price_rx: mpsc::Receiver<PriceTick>,
    signal_tx: mpsc::Sender<Signal>,
) -> Result<()> {
    // 신호 감지 루프
}
```

#### State Machine for Decision Making

```rust
enum TradingState {
    Idle,
    Watching { market: String },
    InPosition { position: Position },
}

impl TradingState {
    async fn on_price_tick(&mut self, tick: PriceTick) -> Action;
    async fn on_signal(&mut self, signal: Signal) -> Action;
}
```

### 2.4 Error Handling Strategy (에러 처리 전략)

```rust
#[derive(Debug, thiserror::Error)]
pub enum TradingError {
    #[error("Upbit API error: {0}")]
    UpbitApi(#[from] UpbitError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: f64, available: f64 },

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

// 에러 복구 전략
impl TradingError {
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::RateLimitExceeded | Self::UpbitApi(_))
    }

    pub fn retry_delay(&self) -> Duration {
        match self {
            Self::RateLimitExceeded => Duration::from_secs(5),
            Self::UpbitApi(_) => Duration::from_secs(1),
            _ => Duration::ZERO,
        }
    }
}
```

### 2.5 Testing Strategy (테스트 전략)

#### Unit Tests (단위 테스트)

- 각 에이전트의 독립적 동작 테스트
- Mock을 사용한 Upbit API 격리
- Property-based testing (선택)

#### Integration Tests (통합 테스트)

- WireMock을 사용한 Upbit API 모의
- 전체 거래 흐름 테스트
- SQLite 인메모리 데이터베이스 사용

#### Simulation Tests (시뮬레이션)

- 과거 데이터로 백테스팅
- Paper Trading 모드
- 다양한 시나리오 시뮬레이션

### 2.6 Deployment Strategy (배포 전략)

#### Development Environment

```bash
cargo run --bin autocoin
```

#### Production Environment

```bash
# systemd service
[Unit]
Description=AutoCoin Trading Bot
After=network.target

[Service]
Type=simple
User=autocoin
WorkingDirectory=/opt/autocoin
Environment="RUST_LOG=info"
ExecStart=/opt/autocoin/target/release/autocoin
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

#### Docker Support (선택)

```dockerfile
FROM rust:1.85-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/autocoin /usr/local/bin/
CMD ["autocoin"]
```

---

## 3. Risks and Mitigation (리스크 및 대응)

### 3.1 Technical Risks

| 리스크 | 영향 | 확률 | 대응 방안 |
|--------|------|------|----------|
| Upbit API 장애 | 높음 | 중간 | 재시도 로직, 장애 알림 |
| WebSocket 연결 끊김 | 중간 | 높음 | 자동 재연결, 하트비트 |
| Rate Limit 초과 | 중간 | 중간 | Exponential Backoff |
| 데이터베이스 손상 | 높음 | 낮음 | 정기 백업, WAL 모드 |

### 3.2 Trading Risks

| 리스크 | 영향 | 확률 | 대응 방안 |
|--------|------|------|----------|
| 시장 급락 | 높음 | 중간 | 손절가 엄격 준수 |
| 유동성 부족 | 높음 | 낮음 | Top 20 코인만 거래 |
| 잔고 부족 | 중간 | 중간 | 사전 잔고 확인 |
| 오동작으로 인한 손실 | 높음 | 낮음 | Paper Trading 모드로 먼저 검증 |

### 3.3 Operational Risks

| 리스크 | 영향 | 확률 | 대응 방안 |
|--------|------|------|----------|
| 서버 재부팅 | 중간 | 중간 | 상태 자동 복구 |
| API Key 노출 | 높음 | 낮음 | .env Git 제외, 권한 관리 |
| Discord Webhook 실패 | 낮음 | 중간 | 로그에 기록, 주요 기능 계속 |

---

## 4. Dependencies Between Milestones

```
M1 (Foundation)
    ├─→ M2 (Upbit API Client)
    │       ├─→ M3 (Market Monitor)
    │       │       └─→ M4 (Signal Detector)
    │       │               └─→ M5 (Decision Maker)
    │       │                       └─→ M6 (Execution)
    │       │                               └─→ M7 (Risk Manager)
    │       │
    │       └─→ M8 (Notification)
    │
    └─→ M9 (State Persistence)

M10 (Testing) - 병렬 진행 가능
M11 (Documentation) - 병렬 진행 가능
```

---

## 5. Success Criteria (성공 기준)

### 5.1 Functional Criteria

- [ ] Upbit API로 시계 조회 가능
- [ ] WebSocket 연결 유지 및 자동 복구
- [ ] 모멘텀 신호 감지 정확도 70% 이상
- [ ] 매수/매도 주문 정상 실행
- [ ] 손절/익절 자동 실행
- [ ] Discord 알림 정상 전송

### 5.2 Non-Functional Criteria

- [ ] 24시간 연속 가동 안정성
- [ ] API Rate Limit 준수
- [ ] 로그 기록 완결성
- [ ] 단위 테스트 커버리지 85% 이상
- [ ] 메모리 사용량 500MB 이하

---

## 6. Next Steps (다음 단계)

1. **환경 설정**: Rust 설치, 프로젝트 초기화
2. **Upbit API 키 발급**: 개발용 키 획득
3. **Discord Webhook 생성**: 알림 채널 설정
4. **첫 번째 커밋**: Milestone 1 완료 후 Git 초기화
5. **개발 시작**: Milestone 2부터 순차적 구현

---

**Traceability**: `SPEC-ID: SPEC-TRADING-001` → Plan Phase Complete
