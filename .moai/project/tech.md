# Tech Document

## Technology Stack Specifications

### Backend (Rust)

| 카테고리 | 기술 | 버전 | 용도 |
|----------|------|------|------|
| **언어** | Rust | 1.85+ | 시스템 구현 |
| **런타임** | Tokio | 1.40+ | 비동기 실행 |
| **웹 프레임워크** | Axum | 0.7+ | REST API / WebSocket |
| **데이터베이스** | SQLite (sqlx) | 0.8+ | 상태 저장 |
| **직렬화** | serde | 1.0+ | JSON 처리 |
| **HTTP 클라이언트** | reqwest | 0.12+ | Upbit API 호출 |
| **WebSocket** | tokio-tungstenite | 0.24+ | WebSocket 통신 |
| **TUI** | ratatui | 0.28+ | CLI 대시보드 |
| **CLI** | clap | 4.5+ | 커맨드 라인 파싱 |
| **로깅** | tracing | 0.1+ | 구조화 로깅 |
| **설정** | config | 0.14+ | 설정 관리 |
| **에러 처리** | anyhow, thiserror | 1.0+ | 에러 처리 |
| **Rate Limiting** | governor | 0.6+ | API 호출 제어 |

### Frontend (TypeScript/Next.js)

| 카테고리 | 기술 | 버전 | 용도 |
|----------|------|------|------|
| **언어** | TypeScript | 5.7+ | 타입 안전성 |
| **프레임워크** | Next.js | 16.1+ | React 프레임워크 |
| **UI 라이브러리** | React | 19.0+ | UI 구현 |
| **스타일링** | Tailwind CSS | 3.4+ | 유틸리티 스타일 |
| **컴포넌트** | shadcn/ui | latest | 기본 UI 컴포넌트 |
| **차트** | Recharts | 2.15+ | 데이터 시각화 |
| **상태 관리** | SWR | 2.3+ | 데이터 가져오기 |
| **아이콘** | Lucide React | 0.468+ | 아이콘 라이브러리 |
| **테마** | next-themes | 0.4+ | 다크 모드 |

---

## Development Environment

### Prerequisites (사전 요구사항)

| 도구 | 최소 버전 | 설명 |
|------|-----------|------|
| Rust | 1.85+ | 백엔드 개발 |
| Node.js | 20+ | 프론트엔드 개발 |
| npm | 10+ | 패키지 관리 |
| Git | 2.40+ | 버전 관리 |

### Environment Variables

```bash
# .env 파일

# Upbit API (필수)
UPBIT_ACCESS_KEY=your_access_key_here
UPBIT_SECRET_KEY=your_secret_key_here
UPBIT_API_URL=https://api.upbit.com/v1
UPBIT_WS_URL=wss://api.upbit.com/websocket/v1

# Trading Parameters
TRADING_TARGET_COINS=20
TARGET_PROFIT_RATE=0.10
STOP_LOSS_RATE=0.05
SURGE_THRESHOLD=0.05
SURGE_TIMEFRAME_MINUTES=60
VOLUME_MULTIPLIER=2.0
MIN_ORDER_AMOUNT_KRW=5000

# Discord Notification (선택)
DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/...

# System
LOG_LEVEL=info
RUST_LOG=info
DB_PATH=./data/trading.db

# Strategy Config
STRATEGY_CONFIG=./config/strategy.toml

# Web Server (v0.3.0+)
WEB_SERVER_HOST=0.0.0.0
WEB_SERVER_PORT=8080
```

---

## Build Tools

### Backend Build

```bash
# 개발 빌드
cargo build

# 릴리스 빌드
cargo build --release

# 실행
cargo run
cargo run --release

# CLI 대시보드 모드
cargo run -- --dashboard

# 웹 서버 모드
cargo run -- --web

# 백그라운드 데몬 모드
cargo run -- --daemon
```

### Frontend Build

```bash
# 의존성 설치
cd web
npm install

# 개발 서버
npm run dev

# 프로덕션 빌드
npm run build

# 프로덕션 실행
npm start

# 타입 체크
npm run type-check

# 린트
npm run lint
```

---

## Testing Strategy

### Backend Testing

| 테스트 유형 | 도구 | 커버리지 목표 |
|-------------|------|---------------|
| 단위 테스트 | cargo test | 85%+ |
| 통합 테스트 | wiremock, mockito | 80%+ |
| 벤치마크 | criterion | - |

**테스트 실행**:
```bash
# 전체 테스트
cargo test

# 특정 모듈 테스트
cargo test types::trading
cargo test strategy::momentum

# 커버리지 확인 (tarpaulin)
cargo tarpaulin --out Html

# 벤치마크
cargo bench
```

### Frontend Testing

| 테스트 유형 | 도구 | 설명 |
|-------------|------|------|
| 컴포넌트 테스트 | Vitest | React 컴포넌트 테스트 |
| E2E 테스트 | Playwright | 사용자 흐름 테스트 |
| 타입 체크 | tsc | 컴파일 타임 검증 |

---

## CI/CD Pipeline

### Quality Gates

| Phase | LSP Errors | LSP Warnings | Type Errors | Coverage |
|-------|------------|--------------|-------------|----------|
| **plan** | Baseline | Baseline | Baseline | - |
| **run** | 0 | 0 | 0 | 85%+ |
| **sync** | 0 | 10 | 0 | 85%+ |

### GitHub Actions Workflow

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  backend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check

  frontend:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - run: cd web && npm install
      - run: cd web && npm run type-check
      - run: cd web && npm run lint
```

---

## Performance Requirements

### Backend Performance

| 항목 | 목표 | 측정 방법 |
|------|------|-----------|
| 시세 처리 지연 | < 100ms | tracing span |
| 주문 실행 지연 | < 500ms | API 응답 시간 |
| WebSocket 재연결 | < 5초 | 재연결 로직 |
| API Rate Limit 준수 | 100% | governor 크레이트 |

### Frontend Performance

| 항목 | 목표 | 측정 방법 |
|------|------|-----------|
| FCP (First Contentful Paint) | < 500ms | Lighthouse |
| LCP (Largest Contentful Paint) | < 2.5s | Lighthouse |
| TTI (Time to Interactive) | < 3.5s | Lighthouse |
| WebSocket 메시지 지연 | < 50ms | 클라이언트 로그 |

---

## Security Requirements

### API Key Management

| 항목 | 구현 |
|------|------|
| 저장 위치 | `.env` 파일 (Git 제외) |
| 로깅 | 마스킹 처리 |
| 전송 | HTTPS/WSS만 사용 |

### CORS Configuration

```rust
// 개발 환경
CorsLayer::permissive()

// 프로덕션 (권장)
CorsLayer::new()
    .allow_origin("https://yourdomain.com")
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers(Any)
```

### Input Validation

| 입력 | 검증 |
|------|------|
| API 키 | 형식 검증 |
| 전략 파라미터 | 범위 검증 |
| 주문 금액 | 최소/최대값 검증 |

---

## Deployment

### Development Deployment

```bash
# 터미널 1: 백엔드
cargo run -- --web

# 터미널 2: 프론트엔드
cd web && npm run dev
```

### Production Deployment

#### Linux (systemd)

```ini
# /etc/systemd/system/autocoin.service
[Unit]
Description=AutoCoin Trading Bot
After=network.target

[Service]
Type=simple
User=autocoin
WorkingDirectory=/opt/autocoin
ExecStart=/opt/autocoin/target/release/autocoin --web
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

#### Windows (Task Scheduler)

```xml
<!-- taskscheduler/autocoin.xml -->
<Task>
  <Actions>
    <Exec>
      <Command>C:\path\to\autocoin.exe</Command>
      <Arguments>--daemon</Arguments>
    </Exec>
  </Actions>
  <Triggers>
    <BootTrigger />
  </Triggers>
</Task>
```

#### Docker (선택)

```dockerfile
# Dockerfile
FROM rust:1.85-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/autocoin /usr/local/bin/
EXPOSE 8080
CMD ["autocoin", "--web"]
```

---

## Monitoring & Observability

### Logging

| 레벨 | 사용 사례 |
|------|-----------|
| ERROR | 주문 실패, API 오류 |
| WARN | 재연결, 재시도 |
| INFO | 주문 체결, 포지션 변경 |
| DEBUG | 상세 디버깅 정보 |

### Metrics (선택 사항)

| 메트릭 | 설명 |
|--------|------|
| `autocoin_trades_total` | 총 거래 횟수 |
| `autocoin_pnl` | 총 PnL |
| `autocoin_api_calls` | API 호출 횟수 |
| `autocoin_websocket_reconnects` | WebSocket 재연결 횟수 |

---

## Technical Constraints

### Upbit API Constraints

| 제약 | 값 | 설명 |
|------|-----|------|
| Rate Limit | 10회/초 | 회원 기준 |
| WebSocket Channels | 300개/연결 | 구독 제한 |
| Order Size | 최소 5,000 KRW | 최소 주문 금액 |

### System Constraints

| 제약 | 값 | 설명 |
|------|-----|------|
| SQLite Concurrent Writes | 1 | WAL 모드로 완화 |
| WebSocket Connections | 무제한 | 실제로는 메모리 제한 |
| Position Count | 1개 | 단일 포지션 전략 |

---

## Dependency Management

### Backend Dependencies (Cargo.toml)

```toml
[dependencies]
# Async Runtime
tokio = { version = "1.40", features = ["full"] }
tokio-tungstenite = { version = "0.24", features = ["native-tls"] }

# Database
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "chrono"] }

# Web
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
axum = "0.7"
tower = "0.5"
tower-http = { version = "0.5", features = ["cors", "fs"] }

# Authentication
jsonwebtoken = "9.3"
hmac = "0.12"
sha2 = "0.10"
base64 = "0.22"

# Error Handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Configuration
dotenvy = "0.15"
config = "0.14"
toml = "0.8"

# Time
chrono = { version = "0.4", features = ["serde"] }

# TUI Framework
ratatui = "0.28"
crossterm = "0.28"

# CLI Parsing
clap = { version = "4.5", features = ["derive"] }

# Rate Limiting
governor = "0.6"

[dev-dependencies]
mockito = "1.5"
tokio-test = "0.4"
wiremock = "0.6"
criterion = "0.5"
```

### Frontend Dependencies (package.json)

```json
{
  "dependencies": {
    "next": "^16.1.5",
    "react": "^19.0.0",
    "react-dom": "^19.0.0",
    "recharts": "^2.15.0",
    "swr": "^2.3.1",
    "tailwindcss": "^3.4.17",
    "lucide-react": "^0.468.0",
    "next-themes": "^0.4.6",
    "@radix-ui/react-slot": "^1.1.1",
    "@radix-ui/react-separator": "^1.1.1",
    "@radix-ui/react-dialog": "^1.1.4",
    "@radix-ui/react-dropdown-menu": "^2.1.3",
    "@radix-ui/react-tabs": "^1.1.3",
    "@radix-ui/react-switch": "^1.1.3",
    "@radix-ui/react-label": "^2.1.2",
    "@radix-ui/react-select": "^2.1.4"
  },
  "devDependencies": {
    "@types/node": "^22",
    "@types/react": "^19",
    "@types/react-dom": "^19",
    "typescript": "^5.7",
    "eslint": "^9",
    "eslint-config-next": "^16.1.5",
    "tailwindcss": "^3.4.17",
    "postcss": "^8.4.49",
    "autoprefixer": "^10.4.20"
  }
}
```

---

## TRUST 5 Compliance

### Tested (테스트)
- 단위 테스트: 85%+ 커버리지 목표
- 통합 테스트: mockito, wiremock 활용
- 백테스팅: 과거 데이터로 전략 검증

### Readable (가독성)
- 영어 코멘트 작성
- 명확한 명명 규칙
- rustfmt 적용

### Unified (통일성)
- 일관된 에러 처리 패턴
- 표준 라이브러리 선호
- 코드 스타일 통일

### Secured (보안)
- API 키 환경 변수 관리
- HTTPS/WSS만 사용
- 입력 유효성 검증

### Trackable (추적 가능성)
- 구조화 로그 (tracing)
- UUID 기반 요청 추적
- Git 커밋 메시지 규칙

---

## HISTORY

| Version | Date | Changes |
|---------|------|---------|
| v1.0.0 | 2026-02-08 | 최초 작성 (SPEC-TRADING-001~004 통합) |
