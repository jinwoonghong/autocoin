# AutoCoin - Upbit Automated Trading Agent System

Upbit API를 활용한 자동 트레이딩 에이전트 시스템입니다.

## Features

- **Real-time Market Monitoring**: Upbit WebSocket을 통한 실시간 시세 수집
- **Signal Detection**: 모멘텀/서징 감지 알고리즘
- **Automated Trading**: 자동 매수/매도 실행
- **Risk Management**: 자동 손절/익절
- **Discord Notifications**: 거래 알림 전송
- **SQLite Persistence**: 거래 기록 저장

## Architecture

```
Market Monitor -> Signal Detector -> Decision Maker -> Execution Agent
                                          |                   |
                                          v                   v
                                    Risk Manager        Notification
```

## Requirements

- Rust 1.85+
- SQLite 3

## Installation

1. Repository 클론:
```bash
git clone <repository-url>
cd autocoin
```

2. 환경 변수 설정:
```bash
cp .env.example .env
```

3. `.env` 파일 편집:
```bash
UPBIT_ACCESS_KEY=your_access_key
UPBIT_SECRET_KEY=your_secret_key
DISCORD_WEBHOOK_URL=your_webhook_url
```

4. 빌드:
```bash
cargo build --release
```

## Usage

### Development Mode

```bash
cargo run
```

### Production Mode

```bash
./target/release/autocoin
```

### Configuration

환경 변수로 동작을 제어할 수 있습니다:

| 변수 | 설명 | 기본값 |
|------|------|--------|
| `TRADING_TARGET_COINS` | 모니터링할 코인 수 | 20 |
| `TARGET_PROFIT_RATE` | 목표 수익률 | 0.10 (10%) |
| `STOP_LOSS_RATE` | 손절률 | 0.05 (5%) |
| `SURGE_THRESHOLD` | 급등 감지 임계값 | 0.05 (5%) |
| `MIN_ORDER_AMOUNT_KRW` | 최소 주문 금액 | 5000 |
| `LOG_LEVEL` | 로그 레벨 | info |

## Testing

```bash
# 전체 테스트 실행
cargo test

# 테스트 커버리지 확인
cargo test --workspace

# 단위 테스트만
cargo test --lib

# 통합 테스트만
cargo test --test '*'
```

## Risk Warning

이 소프트웨어는 교육 목적으로 제공됩니다. 암호화폐 트레이딩은 높은 리스크가 있습니다.
본인의 판단 하에 사용하며, 발생하는 모든 손실에 대한 책임은 사용자에게 있습니다.

## License

MIT License
