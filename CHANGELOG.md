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
- [ ] Web-based monitoring dashboard
- [ ] Machine learning-based price prediction

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
