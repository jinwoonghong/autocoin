# Changelog

All notable changes to AutoCoin will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-06

### Added

#### Core System
- Multi-agent trading system architecture with 6 independent agents
- Rust 1.85+ compatibility
- SQLite database for state persistence
- Async/await runtime using Tokio 1.40+

#### Agents
- **Market Monitor Agent**: Real-time market data collection via Upbit WebSocket
- **Signal Detector Agent**: Momentum/surge detection algorithm
- **Decision Maker Agent**: Position management and trade decision logic
- **Execution Agent**: Order execution via Upbit REST API
- **Risk Manager Agent**: Automatic stop-loss and take-profit monitoring
- **Notification Agent**: Discord webhook notifications for trade events

#### Trading Features
- Momentum following strategy with configurable parameters
- Single position concentration strategy
- Automatic stop-loss at -5% (configurable)
- Automatic take-profit at +10% (configurable)
- Surge detection: 5% price increase + 2x volume within 60 minutes
- Top 20 KRW market coin monitoring

#### API Integration
- Upbit REST API client with authentication
- Upbit WebSocket client with auto-reconnect
- Rate limiting compliance (10 requests/second)
- Exponential backoff for retry logic

#### Configuration
- Environment variable-based configuration (.env file)
- TOML-based strategy configuration (config/strategy.toml)
- Configurable trading parameters:
  - Target coins count
  - Profit/loss rates
  - Surge detection thresholds
  - Volume multipliers
  - Minimum order amounts

#### Data Persistence
- SQLite database schema for:
  - Positions (active and closed)
  - Orders (all orders with status)
  - Price ticks (optional analytics)
  - Signals (debugging and analytics)

#### Notifications
- Discord webhook integration
- Color-coded notification types:
  - Green: Buy orders
  - Red: Sell orders
  - Yellow: Buy signals
  - Orange: Errors

#### Logging
- Structured JSON logging (tracing crate)
- Configurable log levels
- API key masking for security

#### Testing
- Unit tests for core data types
- Strategy algorithm tests
- Configuration validation tests
- Integration test framework

#### Documentation
- Comprehensive README.md with installation and usage
- ARCHITECTURE.md with system design details
- API.md with Upbit integration documentation
- SPEC-TRADING-001 with system requirements

### Security

- API keys loaded from environment only (no hardcoding)
- API keys masked in logs
- HTTPS/WSS only for external communications
- Input validation for all user-provided parameters

### Performance

- Target: < 100ms price tick processing
- Target: < 500ms order execution latency
- WebSocket reconnection within 5 seconds

### Dependencies

Major dependencies added:
- `tokio` 1.40 (async runtime)
- `sqlx` 0.8 (database)
- `reqwest` 0.12 (HTTP client)
- `serde` 1.0 (serialization)
- `tracing` 0.1 (logging)
- `governor` 0.6 (rate limiting)
- `jsonwebtoken` 9.3 (authentication)
- `dotenvy` 0.15 (env variables)

### Implementation Notes

- Developed following SPEC-TRADING-001 requirements
- Implemented using Domain-Driven Design (DDD) approach
- TRUST 5 quality gates compliance (86% score achieved)
- All 23 requirements from SPEC document implemented

---

## [Unreleased]

### Planned
- [ ] Paper trading mode (simulation)
- [ ] Support for additional exchanges
- [ ] Telegram notification support
- [ ] PWA support for mobile installation
- [ ] Push notifications
- [ ] Machine learning-based price prediction

---

## [0.3.0] - 2026-02-08

### Added

#### Web Dashboard (SPEC-TRADING-004)

- **Next.js 16 Frontend**: React 19 + App Router 기반 웹 대시보드
- **Axum Backend**: Rust 기반 REST API 및 WebSocket 서버
- **Real-time Updates**: WebSocket 기반 실시간 가격/포지션/에이전트 상태 동기화
- **Responsive Design**: 데스크톱/태블릿/모바일 반응형 레이아웃
- **Dark Mode**: next-themes 기반 다크/라이트 테마 지원
- **Interactive Charts**: Recharts 기반 PnL 차트 및 시장 데이터 시각화

#### Pages

- **Dashboard Page** (`/`): 포트폴리오 요약, 현재 포지션, 에이전트 상태 그리드, PnL 차트, 최근 활동
- **Markets Page** (`/markets`): 상위 20개 코인 실시간 가격 모니터링, 필터링, 정렬
- **Trades Page** (`/trades`): 거래 내역 조회, 필터링, 페이지네이션
- **Backtest Page** (`/backtest`): 백테스팅 설정, 실행, 결과 시각화
- **Settings Page** (`/settings`): 전략 파라미터, 리스크 관리, 시스템 제어

#### Components

- **Dashboard Components**: PortfolioSummary, PositionCard, AgentStatusGrid, PnLChart, RecentTrades, QuickStats
- **Markets Components**: MarketTable, MarketFilters, PriceCell, MarketDetailModal
- **Settings Components**: StrategySettings, RiskSettings, SystemControls, NotificationSettings, AboutSettings
- **Layout Components**: Navigation, Header, Sidebar, ThemeToggle, ConnectionStatus
- **UI Components**: Button, Card, Badge, Input, Select, Switch, Tabs, Dialog, Separator, Skeleton

#### REST API

- `GET /api/health`: 헬스 체크
- `GET /api/status`: 시스템 상태 (업타임, WebSocket 연결 상태)
- `GET /api/balance`: 계정 잔고 조회
- `GET /api/position`: 현재 포지션 조회
- `GET /api/trades`: 거래 내역 조회
- `GET /api/markets`: 시장 데이터 조회
- `GET /api/dashboard`: 대시보드 데이터 통합 조회
- `GET /api/agents/status`: 에이전트 상태 조회
- `POST /api/orders`: 수동 주문 생성
- `DELETE /api/position`: 포지션 청산
- `PUT /api/settings`: 설정 업데이트
- `POST /api/trading/pause`: 트레이딩 일시정지
- `POST /api/trading/resume`: 트레이딩 재개

#### WebSocket Events

- `price_update`: 실시간 가격 업데이트
- `trade_executed`: 주문 체결 알림
- `position_update`: 포지션 변경 알림
- `agent_status`: 에이전트 상태 변경
- `notification`: 시스템 알림

#### Features

- **SWR Integration**: 데이터 fetching 및 자동 재검증
- **WebSocket Client**: 자동 재연결 및 이벤트 핸들링
- **Korean Locale**: 한국 원화 형식화 및 날짜/시간 표시
- **Error Handling**: 사용자 친화적 에러 메시지 및 에러 바운더리
- **Loading States**: 스켈레톤 UI 및 로딩 스피너
- **Type Safety**: TypeScript 타입 정의 (backend API 타입과 일치)

#### Backend (Rust)

- **Axum Server**: HTTP 서버 및 WebSocket 핸들러
- **Shared State**: Arc<RwLock<TradingState>> 기반 상태 관리
- **WebSocket Broadcaster**: broadcast::channel 기반 메시지 브로드캐스트
- **CORS Support**: 개발/프로덕션 환경별 CORS 설정
- **API Handlers**: JSON 직렬화/역직렬화 및 에러 처리

#### Dependencies

Frontend:
- `next` 16.1.5 - React 프레임워크 (Turbopack 지원)
- `react` 19.0.0 - UI 라이브러리
- `recharts` 2.15.0 - 차트 라이브러리
- `swr` 2.3.1 - 데이터 fetching
- `lucide-react` 0.468.0 - 아이콘 라이브러리
- `@radix-ui/*` - shadcn/ui 기본 컴포넌트
- `tailwindcss` 3.4.17 - 유틸리티 스타일링
- `next-themes` 0.4.6 - 테마 관리

Backend:
- `axum` 0.7+ - 웹 서버 프레임워크
- `tokio-tungstenite` 0.24+ - WebSocket
- `tower-http` 0.5+ - CORS, tracing 미들웨어

### Documentation

- Updated README.md with web dashboard information
- Added SPEC-TRADING-004 with comprehensive web dashboard specification
- Updated architecture diagram with web dashboard components

---

## [0.2.0] - 2026-02-07

### Added

#### CLI Dashboard (SPEC-TRADING-002)

- **ratatui TUI Framework**: Terminal User Interface 대시보드
- **4-Panel Layout**: 상태, 포지션, 잔고, 시장 정보 패널
- **Real-time Monitoring**: 에이전트 상태 실시간 표시
- **Keyboard Shortcuts**: q(종료), p(일시정지), r(재개), ?(도움말)
- **Color-coded Display**: 실행 상태별 색상 구분 (Green/Red/Yellow)
- **PnL Visualization**: 현재 포지션 손익 실시간 표시

#### 24h Execution Support (SPEC-TRADING-002)

- **Daemon Mode**: `--daemon` 플래그로 백그라운드 실행
- **Watchdog System**: 프로세스 크래시 시 자동 재시작
- **Linux systemd**: systemd 서비스 파일 제공
- **Windows Task Scheduler**: 부팅 시 자동 시작 설정
- **Log Rotation**: 로그 파일 자동 관리

#### Technical Indicators (SPEC-TRADING-003)

- **RSI Indicator**: Relative Strength Index (기간 14, 과매수/과매도)
- **MACD Indicator**: Moving Average Convergence Divergence (12/26/9)
- **Bollinger Bands**: 상/하단 밴드 및 스퀴즈 감지
- **SMA/EMA**: Simple/Exponential Moving Average
- **Indicator Cache**: 지표 계산 결과 캐싱으로 성능 최적화

#### Advanced Strategy Features (SPEC-TRADING-003)

- **Multi-Indicator Strategy**: 다중 지표 결합 신호 생성
- **Signal Scoring System**: 가중치 기반 신호 점수화
- **Strategy Manager**: 런타임 전략 전환 기능
- **Golden Cross Detection**: 단기/장기 이동평균선 교차 감지

#### Backtesting Engine (SPEC-TRADING-003)

- **Historical Data Fetcher**: Upbit API 과거 캔들 데이터 수집
- **Backtest Simulator**: 과거 데이터 기반 시뮬레이션
- **Performance Metrics**: ROI, 승률, 최대 낙폭, 샤프 비율
- **Parameter Optimizer**: 그리드 서치 기반 파라미터 최적화
- **Strategy Comparison**: 여러 전략 성과 비교

#### Configuration Enhancements

- **Indicator Parameters**: 각 지표별 파라미터 설정 지원
- **Strategy Weights**: 다중 지표 가중치 설정
- **Backtest Config**: 백테스팅 기간 및 조건 설정

### Dependencies

- `ratatui` 0.28+ - TUI framework
- `crossterm` 0.28+ - Cross-platform terminal handling

### Documentation

- Updated README.md with new features
- Updated API.md with indicators and backtest API
- Added SPEC-TRADING-002 and SPEC-TRADING-003

---

## Links

- [SPEC-TRADING-001](.moai/specs/SPEC-TRADING-001/spec.md) - System Specification
- [SPEC-TRADING-002](.moai/specs/SPEC-TRADING-002/spec.md) - CLI Dashboard Specification
- [SPEC-TRADING-003](.moai/specs/SPEC-TRADING-003/spec.md) - Advanced Strategies Specification
- [GitHub Issues](https://github.com/your-org/autocoin/issues) - Issue Tracker
