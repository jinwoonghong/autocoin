# API Integration Document

Upbit API Integration Details for AutoCoin

## Table of Contents

- [Overview](#overview)
- [REST API](#rest-api)
- [WebSocket API](#websocket-api)
- [Authentication](#authentication)
- [Rate Limiting](#rate-limiting)
- [Error Handling](#error-handling)
- [Data Models](#data-models)
- [Dashboard API](#dashboard-api)
- [Indicators API](#indicators-api)
- [Backtest API](#backtest-api)
- [Strategy API](#strategy-api)

---

## Overview

AutoCoin은 Upbit Open API를 사용하여 시장 데이터를 수집하고 주문을 실행합니다. 이 문서는 Upbit API 통합의 상세 내용을 설명합니다.

### API Endpoints Summary

| Category | Base URL | Purpose |
|----------|----------|---------|
| REST API | `https://api.upbit.com/v1` | Quotation, Trade, Account API |
| WebSocket | `wss://api.upbit.com/websocket/v1` | Real-time market data |

---

## REST API

### Market Information

#### Get All Markets

전체 마켓 코드를 조회합니다.

**Endpoint**: `GET /market/all`

**Authentication**: Not required

**Response**:
```json
[
  {
    "market": "KRW-BTC",
    "korean_name": "비트코인",
    "english_name": "Bitcoin",
    "market_warning": "NONE"
  }
]
```

**Usage in AutoCoin**:
```rust
// src/upbit/client.rs
pub async fn get_markets(&self) -> Result<Vec<MarketInfo>> {
    self.get_auth("/market/all", None).await
}
```

#### Get Top KRW Markets

KRW 마켓 상위 N개 코인을 조회합니다.

**Endpoint**: `GET /market/all` (filtered)

**Implementation**:
```rust
pub async fn get_top_krw_markets(&self, top_n: usize) -> Result<Vec<String>> {
    let markets = self.get_markets().await?;
    let krw_markets: Vec<String> = markets
        .into_iter()
        .filter(|m| m.market.starts_with("KRW-"))
        .take(top_n)
        .map(|m| m.market)
        .collect();
    Ok(krw_markets)
}
```

### Candle Data

#### Get Minute Candles

분봉 데이터를 조회합니다.

**Endpoint**: `GET /candles/minutes/{unit}`

**Parameters**:
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| market | string | Yes | 마켓 코드 (예: KRW-BTC) |
| count | integer | No | 조회 개수 (최대 200, 기본값 1) |

**Unit Options**: `1`, `3`, `5`, `15`, `30`, `60`, `240`

**Response**:
```json
[
  {
    "market": "KRW-BTC",
    "candle_date_time_utc": "2024-01-01T00:00:00Z",
    "candle_date_time_kst": "2024-01-01T09:00:00Z",
    "opening_price": 50000000,
    "high_price": 51000000,
    "low_price": 49500000,
    "trade_price": 50500000,
    "timestamp": 1704067200000,
    "candle_acc_trade_volume": 10.5,
    "candle_acc_trade_price": 525000000
  }
]
```

### Account Information

#### Get Accounts

전체 계정 잔고를 조회합니다.

**Endpoint**: `GET /accounts`

**Authentication**: Required

**Response**:
```json
[
  {
    "currency": "KRW",
    "balance": 1000000,
    "locked": 0,
    "avg_buy_price": "0",
    "avg_buy_price_modified": false,
    "unit_currency": "KRW"
  },
  {
    "currency": "BTC",
    "balance": 0.001,
    "locked": 0,
    "avg_buy_price": "50000000",
    "avg_buy_price_modified": true,
    "unit_currency": "KRW"
  }
]
```

**Usage in AutoCoin**:
```rust
pub async fn get_krw_balance(&self) -> Result<f64> {
    let accounts = self.get_accounts().await?;
    let krw = accounts
        .iter()
        .find(|b| b.currency == "KRW")
        .map(|b| b.available)
        .unwrap_or(0.0);
    Ok(krw)
}
```

### Order Management

#### Place Order (Limit)

지정가 주문을 실행합니다.

**Endpoint**: `POST /orders`

**Authentication**: Required

**Request Body**:
```json
{
  "market": "KRW-BTC",
  "side": "bid",
  "volume": "0.001",
  "price": "50000000",
  "ord_type": "limit"
}
```

**Parameters**:
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| market | string | Yes | 마켓 코드 |
| side | string | Yes | `bid` (매수) 또는 `ask` (매도) |
| volume | string | Conditional | 주문 수량 |
| price | string | Conditional | 주문 가격 |
| ord_type | string | Yes | `limit` (지정가) 또는 `market` (시장가) |

**Response**:
```json
{
  "uuid": "cdd921b5-5487-47ad-b0ba-d33101295ab7",
  "side": "bid",
  "ord_type": "limit",
  "price": "50000000",
  "state": "wait",
  "market": "KRW-BTC",
  "created_at": "2024-01-01T00:00:00Z",
  "volume": "0.001",
  "remaining_volume": "0.001",
  "reserved_fee": "250",
  "remaining_fee": "250",
  "paid_fee": "0",
  "locked": "50025000",
  "executed_volume": "0",
  "trades_count": 0
}
```

#### Place Order (Market)

시장가 주문을 실행합니다.

**Endpoint**: `POST /orders`

**Request Body**:
```json
{
  "market": "KRW-BTC",
  "side": "bid",
  "volume": null,
  "price": "50000",
  "ord_type": "price"
}
```

**Note**: `ord_type: "price"`는 시장가 매수 주문입니다. `price` 필드에 총 주문 금액(KRW)을 지정합니다.

#### Get Order

주문 정보를 조회합니다.

**Endpoint**: `GET /order`

**Parameters**:
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| uuid | string | Conditional | 주문 UUID |
| identifier | string | Conditional | 조회용 사용자 지정 값 |

**Note**: `uuid` 또는 `identifier` 중 하나는 필수입니다.

#### Cancel Order

주문을 취소합니다.

**Endpoint**: `DELETE /order`

**Parameters**: Same as Get Order

---

## WebSocket API

### Connection

WebSocket 연결 URL:

```
wss://api.upbit.com/websocket/v1
```

### Subscription Format

WebSocket은 JSON 형식의 구독 요청을 받습니다:

```json
[
  { "ticket": "UNIQUE_TICKET" },
  {
    "type": "trade",
    "codes": ["KRW-BTC", "KRW-ETH"],
    "isOnlyRealtime": true
  }
]
```

### Channel Types

| Type | Description | Data Content |
|------|-------------|--------------|
| `trade` | 실시간 체결 데이터 | 체결 가격, 수량, 시간 |
| `ticker` | 실시간 시세 요약 | 현재가, 변화율, 거래량 |
| `orderbook` | 실시간 호가 | 매수/매도 호가 |

### Trade Channel Message

**Subscription**:
```json
[
  { "ticket": "test" },
  { "type": "trade", "codes": ["KRW-BTC"] }
]
```

**Response**:
```json
{
  "ty": "trade",
  "cd": "KRW-BTC",
  "tp": 50000000,
  "tv": 0.001,
  "ttms": 1704067200000,
  "ab": "ASK",
  "cp": 50000000,
  "cr": -0.01,
  "atp": 242865000000,
  "atp24h": 485730000000
}
```

| Field | Description |
|-------|-------------|
| `ty` | Type (trade) |
| `cd` | Market Code |
| `tp` | Trade Price (체결 가격) |
| `tv` | Trade Volume (체결 수량) |
| `ttms` | Trade Timestamp (체결 시각) |
| `cr` | Change Rate (전일 대비 변화율) |

### Ticker Channel Message

**Subscription**:
```json
[
  { "ticket": "test" },
  { "type": "ticker", "codes": ["KRW-BTC"] }
]
```

**Response**:
```json
{
  "ty": "ticker",
  "cd": "KRW-BTC",
  "tp": 50000000,
  "tv": "2428.65",
  "atp": 242865000000,
  "atp24h": 485730000000,
  "cr": -0.01,
  "h52w": 85000000,
  "l52w": 25000000,
  "ms": "ACTIVE"
}
```

---

## Authentication

### JWT Token Generation

Upbit API는 JWT(JSON Web Token) 기반 인증을 사용합니다.

**Algorithm**: HS256 (HMAC-SHA256)

**Payload Structure**:
```json
{
  "access_key": "YOUR_ACCESS_KEY",
  "nonce": "UNIQUE_UUID",
  "timestamp": 1704067200000
}
```

**Implementation in AutoCoin**:
```rust
// src/upbit/mod.rs
pub fn generate_jwt_token(
    access_key: &str,
    secret_key: &str,
) -> Result<String> {
    let header = json!({"alg": "HS256", "typ": "JWT"});
    let payload = json!({
        "access_key": access_key,
        "nonce": uuid::Uuid::new_v4().to_string(),
        "timestamp": Utc::now().timestamp_millis()
    });

    let header_encoded = base64_url_encode(&header.to_string());
    let payload_encoded = base64_url_encode(&payload.to_string());

    let message = format!("{}.{}", header_encoded, payload_encoded);
    let signature = hmac_sha256(secret_key, &message)?;
    let signature_encoded = base64_url_encode(&signature);

    Ok(format!("{}.{}", message, signature_encoded))
}
```

### Authorization Header

```http
Authorization: Bearer {JWT_TOKEN}
```

---

## Rate Limiting

### API Rate Limits

| Account Type | Limit |
|--------------|-------|
| General Member | 10 requests/second |
| Verified Member | 20 requests/second |
| Institutional Member | 60 requests/second |

### Rate Limiter Implementation

AutoCoin은 `governor` 크레이트를 사용하여 Rate Limit을 준수합니다:

```rust
// src/upbit/client.rs
struct RateLimiter {
    max_requests: u32,
    window_secs: f64,
    last_reset: std::time::Instant,
    request_count: u32,
}

impl RateLimiter {
    fn new(max_requests: u32, window_secs: f64) -> Self {
        Self {
            max_requests,
            window_secs,
            last_reset: std::time::Instant::now(),
            request_count: 0,
        }
    }

    async fn acquire(&mut self) {
        // Rate limit logic with automatic wait
        let elapsed = self.last_reset.elapsed().as_secs_f64();

        if elapsed >= self.window_secs {
            self.request_count = 0;
            self.last_reset = std::time::Instant::now();
        }

        if self.request_count >= self.max_requests {
            let wait_time = (self.window_secs - elapsed) as u64 + 1;
            sleep(StdDuration::from_secs(wait_time)).await;
            self.request_count = 0;
            self.last_reset = std::time::Instant::now();
        }

        self.request_count += 1;
    }
}
```

### Exponential Backoff

Rate Limit 초과 시 지수 백오프로 재시도합니다:

```rust
if status == StatusCode::TOO_MANY_REQUESTS {
    warn!("Rate limit exceeded, backing off");
    sleep(StdDuration::from_secs(5)).await;
    return Err(TradingError::RateLimitExceeded);
}
```

---

## Error Handling

### Error Response Format

```json
{
  "name": "invalid_query_parameter",
  "message": "Invalid query parameter"
}
```

### Common Error Codes

| Name | Description | Action |
|------|-------------|--------|
| `invalid_query_parameter` | 잘못된 쿼리 파라미터 | 파라미터 확인 |
| `insufficient_funds` | 잔고 부족 | 잔고 확인 후 재시도 |
| `under_min_order_total` | 최소 주문 금액 미만 | 주문 금액 조정 |
| `max_order_count_reached` | 최대 주문 횟수 도달 | 기존 주문 취소 후 재시도 |
| `rate_limit_exceeded` | Rate Limit 초과 | 잠시 대기 후 재시도 |

---

## Data Models

### MarketInfo

```rust
pub struct MarketInfo {
    pub market: String,      // 마켓 코드
    pub korean_name: String, // 한글 이름
    pub english_name: String, // 영문 이름
    pub market_warning: String, // 투자 경고
}
```

### TickerResponse

```rust
pub struct TickerResponse {
    pub market: String,
    pub trade_price: f64,
    pub change_rate: f64,
    pub acc_trade_volume: f64,
    pub acc_trade_price: f64,
}

impl Into<PriceTick> for TickerResponse {
    fn into(self) -> PriceTick {
        // Conversion logic
    }
}
```

### CandleResponse

```rust
pub struct CandleResponse {
    pub market: String,
    pub candle_date_time_utc: String,
    pub opening_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub trade_price: f64,
    pub candle_acc_trade_volume: f64,
    pub timestamp: i64,
}

impl Into<Candle> for CandleResponse {
    fn into(self) -> Candle {
        // Conversion logic
    }
}
```

### OrderRequest

```rust
pub struct OrderRequest {
    pub market: String,
    pub side: String,     // "bid" or "ask"
    pub volume: Option<String>,
    pub price: Option<String>,
    pub ord_type: String, // "limit", "market", "price"
}
```

### OrderResponse

```rust
pub struct OrderResponse {
    pub uuid: String,
    pub side: String,
    pub ord_type: String,
    pub price: String,
    pub state: String,
    pub market: String,
    pub created_at: String,
    pub volume: String,
    pub remaining_volume: String,
    pub executed_volume: String,
}

impl OrderResponse {
    pub fn to_order(self) -> Order {
        // Conversion logic
    }
}
```

---

## WebSocket Usage in AutoCoin

### Connection Management

```rust
pub struct MarketMonitor {
    ws_client: WebSocketClient,
    markets: Vec<String>,
}

impl MarketMonitor {
    pub async fn monitor(&self, tx: mpsc::Sender<PriceTick>) -> Result<()> {
        loop {
            match self.connect_and_subscribe().await {
                Ok(_) => {
                    if let Err(e) = self.handle_messages(tx.clone()).await {
                        error!("WebSocket error: {}", e);
                    }
                }
                Err(e) => {
                    error!("Connection failed: {}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }
}
```

### Auto-Reconnect

WebSocket 연결이 끊어지면 자동으로 재연결을 시도합니다:

```rust
// Exponential backoff for reconnection
let reconnect_delay = min(60, 2_u64.pow(retry_count));
tokio::time::sleep(Duration::from_secs(reconnect_delay)).await;
```

---

## Additional Resources

- [Upbit Open API Documentation](https://docs.upbit.com/)
- [Upbit API GitHub](https://github.com/youknowone/upbit-api-docs)
- [JWT Specification](https://tools.ietf.org/html/rfc7519)

---

## Dashboard API (v0.2.0)

CLI TUI 대시보드 API입니다.

### Dashboard Controller

```rust
pub struct DashboardController {
    data_rx: mpsc::Receiver<DashboardData>,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl DashboardController {
    pub fn new(data_rx: mpsc::Receiver<DashboardData>) -> Result<Self>;
    pub async fn run(&mut self) -> Result<()>;
    fn draw(&mut self) -> Result<()>;
    fn handle_input(&mut self, event: Event) -> Result<bool>;
}
```

### Dashboard Data Model

```rust
pub struct DashboardData {
    pub agent_states: HashMap<String, AgentState>,
    pub position: Option<PositionData>,
    pub balance: BalanceData,
    pub market_prices: Vec<CoinPrice>,
    pub notifications: Vec<Notification>,
}

pub struct AgentState {
    pub name: String,
    pub status: AgentStatus,  // Running, Idle, Error
    pub last_update: DateTime<Utc>,
}

pub struct PositionData {
    pub market: String,
    pub entry_price: f64,
    pub current_price: f64,
    pub amount: f64,
    pub pnl_percent: f64,
    pub pnl_value: f64,
}
```

### CLI Arguments

```bash
# 대시보드 모드 실행
autocoin --dashboard

# 데몬 모드 실행
autocoin --daemon

# 로그 레벨 지정
autocoin --dashboard --log-level debug

# 설정 파일 지정
autocoin --config ./config/custom.toml --dashboard
```

---

## Indicators API (v0.2.0)

기술적 분석 지표 API입니다.

### Indicator Trait

```rust
#[async_trait]
pub trait Indicator {
    fn update(&mut self, candle: &Candle) -> Option<IndicatorValue>;
    fn value(&self) -> Option<IndicatorValue>;
    fn reset(&mut self);
    fn name(&self) -> &str;
}
```

### RSI Indicator

```rust
pub struct RSI {
    pub period: usize,
    prices: VecDeque<f64>,
}

impl RSI {
    pub fn new(period: usize) -> Self;
    pub fn update(&mut self, price: f64) -> Option<f64>;
    pub fn is_oversold(&self, threshold: f64) -> bool;   // 기본 30
    pub fn is_overbought(&self, threshold: f64) -> bool;  // 기본 70
}
```

**사용 예시**:
```rust
let mut rsi = RSI::new(14);

// RSI 계산
if let Some(value) = rsi.update(current_price) {
    if value < 30.0 {
        println!("과매도 신호 (RSI: {:.2})", value);
    } else if value > 70.0 {
        println!("과매수 신호 (RSI: {:.2})", value);
    }
}
```

### MACD Indicator

```rust
pub struct MACD {
    pub fast_period: usize,   // 기본 12
    pub slow_period: usize,   // 기본 26
    pub signal_period: usize, // 기본 9
    fast_ema: EMA,
    slow_ema: EMA,
    signal_ema: EMA,
}

pub struct MACDValue {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

impl MACD {
    pub fn new(fast: usize, slow: usize, signal: usize) -> Self;
    pub fn update(&mut self, price: f64) -> Option<MACDValue>;
    pub fn is_bullish_cross(&self) -> bool;   // 매수 신호
    pub fn is_bearish_cross(&self) -> bool;   // 매도 신호
}
```

**사용 예시**:
```rust
let mut macd = MACD::new(12, 26, 9);

if let Some(value) = macd.update(current_price) {
    if macd.is_bullish_cross() {
        println!("골든크로스 매수 신호");
    } else if macd.is_bearish_cross() {
        println!("데드크로스 매도 신호");
    }
}
```

### Bollinger Bands Indicator

```rust
pub struct BollingerBands {
    pub period: usize,    // 기본 20
    pub std_dev: f64,     // 기본 2.0
    sma: SMA,
}

pub struct BollingerValue {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
    pub bandwidth: f64,
}

impl BollingerBands {
    pub fn new(period: usize, std_dev: f64) -> Self;
    pub fn update(&mut self, price: f64) -> Option<BollingerValue>;
    pub fn is_touching_upper(&self, price: f64) -> bool;
    pub fn is_touching_lower(&self, price: f64) -> bool;
    pub fn is_squeeze(&self) -> bool;  // 변동성 축소
}
```

### Moving Averages

```rust
pub struct SMA {
    pub period: usize,
    prices: VecDeque<f64>,
}

pub struct EMA {
    pub period: usize,
    pub multiplier: f64,
    pub current_ema: Option<f64>,
}

impl MovingAverage for SMA {
    fn update(&mut self, price: f64) -> Option<f64>;
    fn value(&self) -> Option<f64>;
}

impl MovingAverage for EMA {
    fn update(&mut self, price: f64) -> Option<f64>;
    fn value(&self) -> Option<f64>;
}
```

---

## Backtest API (v0.2.0)

백테스팅 엔진 API입니다.

### Historical Data Fetcher

```rust
pub struct HistoricalDataFetcher {
    client: UpbitClient,
    cache: SQLiteCache,
}

impl HistoricalDataFetcher {
    pub async fn fetch_candles(
        &self,
        market: &str,
        unit: TimeUnit,
        count: usize,
    ) -> Result<Vec<Candle>>;

    pub async fn fetch_range(
        &self,
        market: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<Candle>>;
}
```

### Backtest Simulator

```rust
pub struct BacktestSimulator {
    pub initial_balance: f64,
    pub commission_rate: f64,
    pub strategy: Box<dyn Strategy>,
}

pub struct BacktestResult {
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub roi: f64,
    pub win_rate: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub trades: Vec<Trade>,
    pub equity_curve: Vec<f64>,
}

impl BacktestSimulator {
    pub fn run(&self, candles: Vec<Candle>) -> Result<BacktestResult>;
    pub fn calculate_metrics(&self, result: &BacktestResult) -> PerformanceMetrics;
}
```

### Performance Metrics

```rust
pub struct PerformanceMetrics {
    // 수익성
    pub total_return: f64,
    pub annualized_return: f64,
    pub cagr: f64,

    // 리스크
    pub max_drawdown: f64,
    pub volatility: f64,
    pub var_95: f64,

    // 효율성
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub calmar_ratio: f64,

    // 거래 분석
    pub win_rate: f64,
    pub profit_factor: f64,
    pub avg_win: f64,
    pub avg_loss: f64,
    pub total_trades: usize,
}
```

### Parameter Optimizer

```rust
pub struct ParameterOptimizer {
    pub strategy: Box<dyn Strategy>,
    pub param_ranges: HashMap<String, ParamRange>,
    pub optimization_target: OptimizationTarget,
}

pub enum ParamRange {
    Integer { min: i32, max: i32, step: i32 },
    Float { min: f64, max: f64, step: f64 },
}

pub enum OptimizationTarget {
    ROI,
    SharpeRatio,
    WinRate,
}

impl ParameterOptimizer {
    pub fn grid_search(
        &self,
        candles: Vec<Candle>,
    ) -> Result<Vec<OptimizationResult>>;

    pub fn find_best(&self, results: Vec<OptimizationResult>) -> Option<Parameters>;
}

pub struct OptimizationResult {
    pub parameters: HashMap<String, f64>,
    pub metrics: PerformanceMetrics,
}
```

---

## Strategy API (v0.2.0)

트레이딩 전략 관리 API입니다.

### Strategy Trait

```rust
#[async_trait]
pub trait Strategy {
    fn on_candle(&mut self, candle: &Candle) -> Option<Signal>;
    fn get_name(&self) -> &str;
    fn get_parameters(&self) -> HashMap<String, f64>;
    fn set_parameters(&mut self, params: HashMap<String, f64>) -> Result<()>;
}
```

### Multi-Indicator Strategy

```rust
pub struct MultiIndicatorStrategy {
    pub indicators: Vec<Box<dyn Indicator>>,
    pub weights: HashMap<IndicatorType, f64>,
    pub threshold: f64,  // 매수/매도 임계값
}

pub struct IndicatorSignal {
    pub indicator: IndicatorType,
    pub signal_type: SignalType,  // Buy, Sell, Neutral
    pub confidence: f64,
    pub weight: f64,
}

impl MultiIndicatorStrategy {
    pub fn calculate_score(&self, signals: Vec<IndicatorSignal>) -> f64;
    pub fn generate_decision(&self, score: f64) -> Decision;
}
```

### Strategy Manager

```rust
pub struct StrategyManager {
    active_strategy: Box<dyn Strategy>,
    available_strategies: HashMap<String, Box<dyn Strategy>>,
}

impl StrategyManager {
    pub fn new(initial_strategy: Box<dyn Strategy>) -> Self;
    pub fn register_strategy(&mut self, name: String, strategy: Box<dyn Strategy>);
    pub fn switch_strategy(&mut self, name: &str) -> Result<()>;
    pub fn get_active_strategy(&self) -> &dyn Strategy;
    pub fn list_strategies(&self) -> Vec<String>;
}
```

**사용 예시**:
```rust
let mut manager = StrategyManager::new(Box::new(MomentumStrategy::new()));

// 전략 등록
manager.register_strategy("multi_indicator".to_string(), Box::new(
    MultiIndicatorStrategy::new()
));

// 전략 전환
manager.switch_strategy("multi_indicator")?;
```
