# SPEC-TRADING-001: Upbit Automated Trading Agent System

## TAG BLOCK

```
SPEC-ID: SPEC-TRADING-001
Title: Upbit Automated Trading Agent System
Created: 2026-02-06
Status: Planned
Priority: High
Assigned: TBD
Lifecycle: spec-anchored
Language: Rust
```

---

## 1. Overview (개요)

### 1.1 Purpose (목적)

Upbit API를 활용한 자동 트레이이딩 에이전트 시스템을 구축하여, 시장 모니터링, 의사결정, 주문 실행을 완전 자동화로 수행합니다.

### 1.2 Scope (범위)

- **포함**: 시장 모니터링, 모멘텀 감지, 자동 매수/매도, 리스크 관리, Discord 알림
- **제외**: 타 거래소 연동, 선물/레버리지 거래, 고급 분석 지표 (RSI, MACD 등)

### 1.3 Target Market (대상 시장)

- Upbit KRW 마켓 상위 20개 코인 (시가총액 기준)
- 단일 포지션 집중 전략 (동시에 1개 코인만 보유)

---

## 2. Environment (환경)

### 2.1 System Environment

| 항목 | 내용 |
|------|------|
| 구현 언어 | Rust 1.85+ |
| OS | Linux (Ubuntu 24.04 LTS) |
| 데이터베이스 | SQLite (상태 저장) |
| WebSocket | Upbit 실시간 시세 WebSocket |
| REST API | Upbit Quotation/Trade API |

### 2.2 External Dependencies

| 의존성 | 용도 |
|--------|------|
| Upbit API | 시세 조회, 주문 실행, 계정 정보 |
| Discord Webhook | 거래 알림 전송 |
| `.env` 파일 | API 키 관리 (환경 변수) |

### 2.3 Rate Limiting

- Upbit API: 초당 10회 제한 (회원 기준)
- WebSocket: 연결당 300개 채널 구독 가능

---

## 3. Assumptions (가정)

### 3.1 Technical Assumptions

| 가정 | 신뢰도 | 검증 방법 |
|------|--------|-----------|
| Upbit API가 99.9% 가용성을 제공함 | 높음 | Upbit SLA 문서 확인 |
| WebSocket 연결이 안정적으로 유지됨 | 중간 | 재연결 메커니즘 구현 |
| Rust의 타입 안전성이 메모리 안전성을 보장함 | 높음 | Rust 언어 특성 |

### 3.2 Business Assumptions

| 가정 | 신뢰도 | 검증 방법 |
|------|--------|-----------|
| 모멘텀 전략이 수익성을 가짐 | 중간 | 백테스팅 검증 |
| KRW 마켓 상위 20개 코인이 충분한 유동성을 가짐 | 높음 | 거래량 데이터 확인 |
| 단일 포지션 전략이 리스크를 통제 가능함 | 중간 | 시뮬레이션 검증 |

### 3.3 Risk if Wrong

- **API 가용성**: 시스템 중단 시 손실 발생 가능 → 재연결 로직 강화
- **전략 수익성**: 백테스팅 부족 시 손실 → 사전 검증 필수

---

## 4. Requirements (요구사항 - EARS Format)

### 4.1 Ubiquitous Requirements (항시 활성 요구사항)

**REQ-001**: 시스템은 **항상** Upbit API Rate Limit을 준수해야 한다.

**REQ-002**: 시스템은 **항상** 모든 거래 로그를 SQLite에 저장해야 한다.

**REQ-003**: 시스템은 **항상** API 키를 `.env` 파일에서만 로드해야 한다 (하드코딩 금지).

**REQ-004**: 시스템은 **항상** 모든 에러를 구조화된 로그로 기록해야 한다.

### 4.2 Event-Driven Requirements (이벤트驱动 요구사항)

**REQ-005**: **WHEN** 특정 코인의 가격이 지정된 시간(예: 1시간) 내 N% 이상 상승하면 **THEN** 시스템은 매수 신호를 생성해야 한다.

**REQ-006**: **WHEN** 매수 신호가 생성되고 현재 포지션이 없으면 **THEN** 시스템은 자동으로 매수 주문을 실행해야 한다.

**REQ-007**: **WHEN** 보유 중인 코인이 목표 수익률에 도달하면 **THEN** 시스템은 전량 매도해야 한다.

**REQ-008**: **WHEN** 보유 중인 코인이 손절률에 도달하면 **THEN** 시스템은 즉시 전량 매도해야 한다.

**REQ-009**: **WHEN** 매수 또는 매도 주문이 체결되면 **THEN** 시스템은 Discord로 알림을 전송해야 한다.

**REQ-010**: **WHEN** WebSocket 연결이 끊어지면 **THEN** 시스템은 자동으로 재연결을 시도해야 한다.

**REQ-011**: **WHEN** API 호출이 Rate Limit에 도달하면 **THEN** 시스템은 Exponential Backoff로 재시도해야 한다.

**REQ-012**: **WHEN** 시스템이 시작되면 **THEN** 이전 상태를 SQLite에서 복원해야 한다.

### 4.3 State-Driven Requirements (상태 기반 요구사항)

**REQ-013**: **IF** 현재 포지션이 존재하면 **THEN** 시스템은 새로운 매수를 하지 않아야 한다.

**REQ-014**: **IF** 계정 잔고가 최소 주문 금액보다 적으면 **THEN** 시스템은 매수를 건너뛰고 로그를 기록해야 한다.

**REQ-015**: **IF** 시장이 매매 불가 상태(정비 시간 등)이면 **THEN** 시스템은 주문을 제출하지 않아야 한다.

**REQ-016**: **IF** Discord Webhook 전송이 실패하면 **THEN** 시스템은 로그를 기록하고 주요 기능을 계속 실행해야 한다.

### 4.4 Unwanted Requirements (금지 사항)

**REQ-017**: 시스템은 **절대로** API 키를 로그 파일에 출력해서는 안 된다.

**REQ-018**: 시스템은 **절대로** 사용자 확인 없이 대규모 주문(잔고의 50% 이상)을 실행해서는 안 된다.

**REQ-019**: 시스템은 **절대로** 손절가/목표가를 실시간으로 수동 변경할 수 없어야 한다 (구성 파일로만 제어).

**REQ-020**: 시스템은 **절대로** Discord Webhook URL을 소스 코드에 포함해서는 안 된다.

### 4.5 Optional Requirements (선택 사항)

**REQ-021**: **가능하면** 시스템은 수익/손실 통계 대시보드를 제공해야 한다.

**REQ-022**: **가능하면** 시스템은 백테스팅 모드를 지원해야 한다.

**REQ-023**: **가능하면** 시스템은 여러 거래 전략을 플러그인으로 지원해야 한다.

---

## 5. Specifications (상세 설명)

### 5.1 Multi-Agent Architecture (멀티 에이전트 아키텍처)

```
┌─────────────────────────────────────────────────────────────┐
│                    Trading Agent System                     │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │    Market    │───▶│   Signal     │───▶│   Decision   │  │
│  │   Monitor    │    │   Detector   │    │    Maker     │  │
│  │   Agent      │    │   Agent      │    │   Agent      │  │
│  └──────────────┘    └──────────────┘    └──────────────┘  │
│         │                                       │           │
│         ▼                                       ▼           │
│  ┌──────────────┐                      ┌──────────────┐   │
│  │              │                      │   Execution  │   │
│  │   State      │◀─────────────────────│   Agent      │   │
│  │  Manager     │                      └──────────────┘   │
│  │  (SQLite)    │                              │           │
│  └──────────────┘                              ▼           │
│                                          ┌──────────────┐  │
│                                          │    Risk      │  │
│                                          │   Manager    │  │
│                                          └──────────────┘  │
│                                                 │          │
│                                                 ▼          │
│                                          ┌──────────────┐  │
│                                          │ Notification│  │
│                                          │   Agent     │  │
│                                          └──────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### 5.2 Agent Definitions (에이전트 정의)

#### 5.2.1 Market Monitor Agent

| 항목 | 내용 |
|------|------|
| 역할 | Upbit WebSocket을 통해 실시간 시세 수집 |
| 입력 | WebSocket 시세 메시지 |
| 출력 | 정규화된 가격 데이터 (CandleStick) |
| 책임 | 연결 유지, 데이터 검증 |

**구현 포맷**:
```rust
struct MarketMonitor {
    ws_client: WebSocketClient,
    price_sender: mpsc::Sender<PriceTick>,
}

impl MarketMonitor {
    async fn monitor_top_20(&self) -> Result<()>;
    async fn handle_websocket_message(&mut self, msg: Message) -> Result<()>;
}
```

#### 5.2.2 Signal Detector Agent

| 항목 | 내용 |
|------|------|
| 역할 | 모멘텀/서징 감지, 매수 신호 생성 |
| 입력 | PriceTick 스트림 |
| 출력 | Signal (Buy, Sell, Hold) |
| 책임 | 기술적 계산, 신뢰도 점수 계산 |

**감지 로직**:
- 단기 이동평균(5분) > 장기 이동평균(1시간)
- 거래량 급증 (직전 1시간 평균의 2배 이상)
- 가격 상승률 > 임계값 (기본 5%)

#### 5.2.3 Decision Maker Agent

| 항목 | 내용 |
|------|------|
| 역할 | 최종 거래 결정, 신호 필터링 |
| 입력 | Signal, 현재 포지션 상태 |
| 출력 | Decision (Execute, Skip) |
| 책임 | 리스크 평가, 포지션 충돌 방지 |

**의사결정 로직**:
```rust
enum Decision {
    Buy { coin: String, amount: KRW },
    Sell { coin: String, amount: f64 },
    Hold,
}

fn make_decision(signal: Signal, position: Option<Position>) -> Decision {
    match (signal, position) {
        (Signal::Buy(coin), None) => Decision::Buy { ... },
        (Signal::Sell(coin), Some(pos)) if pos.coin == coin => Decision::Sell { ... },
        _ => Decision::Hold,
    }
}
```

#### 5.2.4 Execution Agent

| 항목 | 내용 |
|------|------|
| 역할 | Upbit API를 통한 주문 실행 |
| 입력 | Decision |
| 출력 | OrderResult |
| 책임 | 주문 제출, 체결 확인, 에러 처리 |

#### 5.2.5 Risk Manager Agent

| 항목 | 내용 |
|------|------|
| 역할 | 포지션 및 리스크 관리 |
| 입력 | 현재 포지션, 실시간 가격 |
| 출력 | RiskAction (Maintain, StopLoss, TakeProfit) |
| 파라미터 | 손절률(기본 -5%), 익절률(기본 +10%) |

#### 5.2.6 Notification Agent

| 항목 | 내용 |
|------|------|
| 역할 | Discord로 거래 알림 전송 |
| 입력 | OrderResult, SystemEvent |
| 출력 | Discord Webhook Call |
| 형식 | Embed 메시지 |

### 5.3 Data Models (데이터 모델)

#### 5.3.1 Core Data Structures

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PriceTick {
    market: String,        // e.g., "KRW-BTC"
    timestamp: i64,
    trade_price: f64,
    change_rate: f64,
    volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Signal {
    market: String,
    signal_type: SignalType,
    confidence: f64,       // 0.0 ~ 1.0
    reason: String,
    timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum SignalType {
    Buy,
    Sell,
    StrongBuy,
    StrongSell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Position {
    market: String,
    entry_price: f64,
    amount: f64,
    entry_time: i64,
    stop_loss: f64,
    take_profit: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Order {
    id: String,
    market: String,
    side: OrderSide,       // Bid or Ask
    price: f64,
    volume: f64,
    status: OrderStatus,
    created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum OrderSide {
    Bid,   // 매수
    Ask,   // 매도
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum OrderStatus {
    Waiting,
    Executed,
    Canceled,
    Failed,
}
```

### 5.4 Upbit API Integration

#### 5.4.1 Required API Endpoints

| 카테고리 | Endpoint | 용도 |
|----------|----------|------|
| Quotation | `/v1/market/all` | 마켓 코드 조회 |
| Quotation | `/v1/candles/minutes/1` | 1분봉 데이터 |
| Trade | `/v1/orders` | 주문 조회 |
| Trade | `/v1/orders` (POST) | 주문 제출 |
| Account | `/v1/accounts` | 계정 잔고 조회 |

#### 5.4.2 WebSocket Channels

| 채널 | 설명 |
|------|------|
| `trade` | 실시간 체결 데이터 |
| `ticker` | 실시간 시세 요약 |

#### 5.4.3 Authentication

- JWT 토큰 기반 인증
- API Secret Key로 서명 생성
- 헤더: `Authorization: Bearer {access_token}`

### 5.5 Discord Integration

#### 5.5.1 Webhook Format

```json
{
  "embeds": [
    {
      "title": "매수 체결 알림",
      "color": 3066993,
      "fields": [
        { "name": "마켓", "value": "KRW-BTC" },
        { "name": "가격", "value": "95,000,000 KRW" },
        { "name": "수량", "value": "0.001 BTC" },
        { "name": "신뢰도", "value": "85%" }
      ],
      "timestamp": "2026-02-06T10:30:00Z"
    }
  ]
}
```

#### 5.5.2 Notification Types

| 타입 | 조건 | 색상 |
|------|------|------|
| BUY | 매수 체결 | 초록 (3066993) |
| SELL | 매도 체결 | 빨강 (15158332) |
| SIGNAL | 매수 신호 감지 | 노랑 (16776960) |
| ERROR | 에러 발생 | 주황 (15105570) |
| INFO | 일반 정보 | 파랑 (3447003) |

### 5.6 Configuration (설정)

#### 5.6.1 Environment Variables

```bash
# Upbit API
UPBIT_ACCESS_KEY=your_access_key
UPBIT_SECRET_KEY=your_secret_key
UPBIT_API_URL=https://api.upbit.com/v1

# Trading Parameters
TRADING_TARGET_COINS=20           # Top N coins
TARGET_PROFIT_RATE=0.10          # 10% profit
STOP_LOSS_RATE=0.05              # 5% loss
SURGE_THRESHOLD=0.05             # 5% surge
MIN_ORDER_AMOUNT_KRW=5000        # 최소 주문 금액

# Discord
DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/...

# System
LOG_LEVEL=info
DB_PATH=./data/trading.db
```

#### 5.6.2 Strategy Configuration (TOML)

```toml
[strategy]
name = "momentum_following"
version = "1.0"

[strategy.surge_detection]
timeframe_minutes = 60
threshold_percent = 5.0
volume_multiplier = 2.0

[strategy.position]
max_positions = 1
max_position_ratio = 0.5

[strategy.risk]
stop_loss_percent = 5.0
take_profit_percent = 10.0
trailing_stop_enabled = false

[strategy.targets]
top_n_coins = 20
min_volume_24h = 1_000_000_000  # 10억 원 이상
```

---

## 6. Non-Functional Requirements (비기능 요구사항)

### 6.1 Performance

| 항목 | 목표 |
|------|------|
| 시세 처리 지연 | < 100ms |
| 주문 실행 지연 | < 500ms |
| WebSocket 재연결 | < 5초 |

### 6.2 Reliability

| 항목 | 목표 |
|------|------|
| 가동률 | 99.5% (월간) |
| 데이터 손실 | 0% (상태 저장 보장) |
| 재시도 횟수 | 최대 5회 |

### 6.3 Security

| 항목 | 조치 |
|------|------|
| API Key 관리 | `.env` 파일, Git 제외 |
| 로그 민감화 | API 키 마스킹 |
| 통신 보안 | HTTPS/WSS만 사용 |

### 6.4 Observability

| 항목 | 구현 |
|------|------|
| 로그 | 구조화된 JSON 로그 |
| 메트릭 | Prometheus 포맷 (선택) |
| 트레이싱 | UUID 기반 요청 추적 |

---

## 7. Traceability (추적성)

| REQ | 관련 에이전트 | 테스트 시나리오 |
|-----|--------------|----------------|
| REQ-001 | Execution | rate_limit_test |
| REQ-002 | State Manager | persistence_test |
| REQ-003 | All | config_loading_test |
| REQ-004 | All | error_logging_test |
| REQ-005 | Signal Detector | surge_detection_test |
| REQ-006 | Decision Maker | buy_signal_execution_test |
| REQ-007 | Risk Manager | take_profit_test |
| REQ-008 | Risk Manager | stop_loss_test |
| REQ-009 | Notification | discord_notification_test |
| REQ-010 | Market Monitor | websocket_reconnect_test |
| REQ-011 | Execution | rate_limit_retry_test |
| REQ-012 | State Manager | state_recovery_test |
| REQ-013 | Decision Maker | single_position_test |
| REQ-014 | Decision Maker | insufficient_balance_test |
| REQ-015 | Execution | market_closed_test |
| REQ-016 | Notification | discord_failure_test |
| REQ-017 | All | api_key_logging_test |
| REQ-018 | Risk Manager | max_order_size_test |
