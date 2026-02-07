# SPEC-TRADING-003: Advanced Trading Strategies for AutoCoin

## TAG BLOCK

```
SPEC-ID: SPEC-TRADING-003
Title: Advanced Trading Strategies for AutoCoin
Created: 2026-02-07
Status: Planned
Priority: Medium
Assigned: TBD
Lifecycle: spec-anchored
Language: Rust
Related: SPEC-TRADING-001, SPEC-TRADING-002
```

---

## 1. Overview (개요)

### 1.1 Purpose (목적)

AutoCoin 트레이딩 시스템에 고급 기술적 분석 지표와 백테스팅 기능을 추가하여, 다양한 시장 상황에서 더 정교한 매수/매도 신호를 생성하고 전략의 수익성을 사전 검증할 수 있게 합니다.

### 1.2 Scope (범위)

**포함**:
- RSI (Relative Strength Index) 지표 구현
- MACD (Moving Average Convergence Divergence) 지표 구현
- Bollinger Bands 지표 구현
- 이동평균선 (SMA, EMA) 지표 구현
- 다중 지표 결합 전략
- 신호 점수화 시스템
- 백테스팅 엔진
- 파라미터 최적화 도구
- 전략 성과 비교 기능

**제외**:
- 머신러닝 기반 예측 모델
- 고빈도 거래 (HFT) 전략
- 선물/레버리지 거래 전략
- 타 거래소 데이터 통합

### 1.3 Background (배경)

SPEC-TRADING-001에서 구현된 모멘텀 전략은 단순하지만 제한적입니다. 시장의 다양한 상황(횡보, 추세, 변동성)에서 더 나은 의사결정을 위해 기술적 분석 도구와 백테스팅 기능이 필요합니다.

---

## 2. Environment (환경)

### 2.1 System Environment

| 항목 | 내용 |
|------|------|
| 구현 언어 | Rust 1.85+ |
| 데이터베이스 | SQLite (백테스팅 결과, 지표 캐싱) |
| 데이터 소스 | Upbit REST API (과거 캔들 데이터) |
| 의존 SPEC | SPEC-TRADING-001 (기반 시스템) |

### 2.2 External Dependencies

| 의존성 | 용도 |
|--------|------|
| Upbit API | 과거 캔들 데이터 조회 |
| SQLite | 백테스팅 결과 저장, 지표 캐싱 |
| 기존 시스템 | Market Monitor, Signal Detector, Decision Maker |

### 2.3 Technical Constraints

- 지표 계산: 100ms 이내 완료 (티커당)
- 백테스팅: 청크 기반 데이터 로딩으로 메모리 제어
- 정확도: 표준 지표 공식과 일치해야 함

---

## 3. Assumptions (가정)

### 3.1 Technical Assumptions

| 가정 | 신뢰도 | 검증 방법 |
|------|--------|-----------|
| Upbit API가 최소 1년치 과거 데이터를 제공함 | 높음 | Upbit API 문서 확인 |
| 지표 계산이 실시간으로 가능함 | 높음 | 벤치마크 테스트 |
| SQLite가 백테스팅 데이터를 효율적으로 저장함 | 중간 | 성능 테스트 |

### 3.2 Business Assumptions

| 가정 | 신뢰도 | 검증 방법 |
|------|--------|-----------|
| 다중 지표 결합이 단일 지표보다 우수함 | 중간 | 백테스팅 비교 |
| RSI 과매수/과매도 신호가 유용함 | 중간 | 과거 데이터 검증 |
| MACD 교차 신호가 추세 반전을 포착함 | 중간 | 백테스팅 검증 |

### 3.3 Risk if Wrong

- **지표 효과**: 백테스팅 결과와 실제 성과 차이 → Paper Trading으로 검증
- **성능 저하**: 다중 지표 계산으로 지연 발생 → 캐싱 및 병렬 처리

---

## 4. Requirements (요구사항 - EARS Format)

### 4.1 Ubiquitous Requirements (항시 활성 요구사항)

**REQ-301**: 시스템은 **항상** 지표 파라미터의 유효성을 검증해야 한다 (기간 > 0, 기준가 > 0).

**REQ-302**: 시스템은 **항상** 백테스팅 결과를 데이터베이스에 저장해야 한다.

**REQ-303**: 시스템은 **항상** 표준 공식을 따르는 지표 값을 계산해야 한다.

**REQ-304**: 시스템은 **항상** 지표 계산 결과를 캐싱해야 한다 (중복 계산 방지).

### 4.2 Event-Driven Requirements (이벤트驱动 요구사항)

**REQ-305**: **WHEN** RSI가 30 미만이면 **THEN** 시스템은 과매도(oversold) 신호를 생성해야 한다.

**REQ-306**: **WHEN** RSI가 70 초과이면 **THEN** 시스템은 과매수(overbought) 신호를 생성해야 한다.

**REQ-307**: **WHEN** MACD 라인이 시그널 라인을 상향 돌파하면 **THEN** 시스템은 매수 신호를 생성해야 한다.

**REQ-308**: **WHEN** MACD 라인이 시그널 라인을 하향 이탈하면 **THEN** 시스템은 매도 신호를 생성해야 한다.

**REQ-309**: **WHEN** 가격이 하단 볼린저 밴드에 도달하면 **THEN** 시스템은 매수 기회 신호를 생성해야 한다.

**REQ-310**: **WHEN** 가격이 상단 볼린저 밴드에 도달하면 **THEN** 시스템은 매도 기회 신호를 생성해야 한다.

**REQ-311**: **WHEN** 단기 이동평균이 장기 이동평균을 상향 돌파하면 **THEN** 시스템은 골든크로스 매수 신호를 생성해야 한다.

**REQ-312**: **WHEN** 백테스팅이 완료되면 **THEN** 시스템은 성과 보고서를 저장해야 한다.

**REQ-313**: **WHEN** 파라미터 최적화가 완료되면 **THEN** 시스템은 최적 파라미터 조합을 반환해야 한다.

**REQ-314**: **WHEN** 다중 지표가 결합된 신호를 생성하면 **THEN** 시스템은 가중치 기반 점수를 계산해야 한다.

### 4.3 State-Driven Requirements (상태 기반 요구사항)

**REQ-315**: **IF** 지표 파라미터가 유효하지 않으면 **THEN** 시스템은 기본값을 사용해야 한다.

**REQ-316**: **IF** 백테스팅 데이터가 불충분하면 (최소 100개 캔들) **THEN** 시스템은 시뮬레이션을 건너뛰어야 한다.

**REQ-317**: **IF** 지표 캐시가 존재하면 **THEN** 시스템은 캐시된 값을 반환해야 한다.

**REQ-318**: **IF** 현재 활성화된 전략이 변경되면 **THEN** 시스템은 새 전략의 파라미터를 로드해야 한다.

### 4.4 Unwanted Requirements (금지 사항)

**REQ-319**: 시스템은 **절대로** 불완전한 과거 데이터로 거래 결정을 해서는 안 된다.

**REQ-320**: 시스템은 **절대로** 백테스팅 결과를 실제 수익을 보장하는 것으로 해석해서는 안 된다.

**REQ-321**: 시스템은 **절대로** 유효하지 않은 파라미터로 지표를 계산해서는 안 된다.

### 4.5 Optional Requirements (선택 사항)

**REQ-322**: **가능하면** 시스템은 그리드 서치를 통한 파라미터 최적화를 제공해야 한다.

**REQ-323**: **가능하면** 시스템은 백테스팅 결과를 시각화하는 차트를 생성해야 한다.

**REQ-324**: **가능하면** 시스템은 여러 전략을 병렬로 실행하여 성과를 비교해야 한다.

---

## 5. Specifications (상세 설명)

### 5.1 Module Structure (모듈 구조)

```
src/
├── indicators/
│   ├── mod.rs              # 지표 모듈 메인
│   ├── rsi.rs              # RSI 지표
│   ├── macd.rs             # MACD 지표
│   ├── bollinger.rs        # Bollinger Bands 지표
│   └── moving_average.rs   # SMA, EMA 지표
├── backtest/
│   ├── mod.rs              # 백테스팅 모듈 메인
│   ├── simulator.rs        # 시뮬레이터 엔진
│   ├── metrics.rs          # 성과 메트릭 (ROI, Win Rate, Max Drawdown)
│   └── optimizer.rs        # 파라미터 최적화
└── strategy/
    ├── mod.rs              # 기존 전략 모듈
    ├── momentum.rs         # 기존 모멘텀 전략
    ├── multi_indicator.rs  # 다중 지표 결합 전략 (신규)
    └── strategy_manager.rs # 전략 관리자 (신규)
```

### 5.2 Technical Indicators (기술적 지표)

#### 5.2.1 RSI (Relative Strength Index)

| 속성 | 값 |
|------|------|
| 공식 | 100 - (100 / (1 + RS)) |
| 파라미터 | 기간 (기본 14) |
| 신호 | < 30: 과매도, > 70: 과매수 |
| 용도 | 모멘텀 오실레이터 |

**구현 구조**:
```rust
struct RSI {
    period: usize,      // 기본 14
    prices: Vec<f64>,   // 가격 이력
}

impl RSI {
    fn new(period: usize) -> Self;
    fn update(&mut self, price: f64) -> Option<f64>;
    fn is_oversold(&self, threshold: f64) -> bool;   // 기본 30
    fn is_overbought(&self, threshold: f64) -> bool;  // 기본 70
}
```

#### 5.2.2 MACD (Moving Average Convergence Divergence)

| 속성 | 값 |
|------|------|
| 공식 | MACD = EMA(12) - EMA(26) |
| 시그널 | EMA(MACD, 9) |
| 히스토그램 | MACD - Signal |
| 신호 | 상향 돌파: 매수, 하향 이탈: 매도 |

**구현 구조**:
```rust
struct MACD {
    fast_period: usize,   // 기본 12
    slow_period: usize,   // 기본 26
    signal_period: usize, // 기본 9
    fast_ema: EMA,
    slow_ema: EMA,
    signal_ema: EMA,
}

struct MACDValue {
    macd: f64,
    signal: f64,
    histogram: f64,
}

impl MACD {
    fn new(fast: usize, slow: usize, signal: usize) -> Self;
    fn update(&mut self, price: f64) -> Option<MACDValue>;
    fn is_bullish_cross(&self) -> bool;   // MACD가 Signal을 상향 돌파
    fn is_bearish_cross(&self) -> bool;   // MACD가 Signal을 하향 이탈
}
```

#### 5.2.3 Bollinger Bands

| 속성 | 값 |
|------|------|
| 중심선 | SMA(20) |
| 상단 밴드 | SMA + (2 * 표준편차) |
| 하단 밴드 | SMA - (2 * 표준편차) |
| 신호 | 가격이 밴드 터치 시 반대 방향 기대 |

**구현 구조**:
```rust
struct BollingerBands {
    period: usize,       // 기본 20
    std_dev: f64,        // 기본 2.0
    sma: SMA,
}

struct BollingerValue {
    upper: f64,
    middle: f64,
    lower: f64,
    bandwidth: f64,      // (upper - lower) / middle
}

impl BollingerBands {
    fn new(period: usize, std_dev: f64) -> Self;
    fn update(&mut self, price: f64) -> Option<BollingerValue>;
    fn is_touching_upper(&self, price: f64) -> bool;
    fn is_touching_lower(&self, price: f64) -> bool;
    fn is_squeeze(&self) -> bool;  // bandwidth < 0.1 (변동성 축소)
}
```

#### 5.2.4 Moving Averages (이동평균선)

| 타입 | 공식 | 용도 |
|------|------|------|
| SMA | Σ(price) / n | 단순 추세 |
| EMA | (price * k) + (prev_ema * (1-k)) | 최근 가중 |

**구현 구조**:
```rust
trait MovingAverage {
    fn update(&mut self, price: f64) -> Option<f64>;
    fn value(&self) -> Option<f64>;
}

struct SMA {
    period: usize,
    prices: VecDeque<f64>,
}

struct EMA {
    period: usize,
    multiplier: f64,
    current_ema: Option<f64>,
}
```

### 5.3 Multi-Indicator Strategy (다중 지표 전략)

#### 5.3.1 Signal Scoring System (신호 점수화)

각 지표에서 생성된 신호에 가중치를 부여하고 종합 점수를 계산합니다.

```rust
struct IndicatorSignal {
    indicator: IndicatorType,
    signal_type: SignalType,    // Buy, Sell, Neutral
    confidence: f64,             // 0.0 ~ 1.0
    weight: f64,                 // 가중치 (기본 1.0)
}

struct MultiIndicatorStrategy {
    indicators: Vec<Box<dyn Indicator>>,
    weights: HashMap<IndicatorType, f64>,
    threshold: f64,              // 매수/매도 임계값 (기본 0.6)
}

impl MultiIndicatorStrategy {
    fn calculate_score(&self, signals: Vec<IndicatorSignal>) -> f64 {
        // Σ(signal * confidence * weight) / Σ(weight)
    }

    fn generate_decision(&self, score: f64) -> Decision {
        if score > self.threshold {
            Decision::Buy { confidence: score }
        } else if score < -self.threshold {
            Decision::Sell { confidence: score.abs() }
        } else {
            Decision::Hold
        }
    }
}
```

#### 5.3.2 Strategy Manager (전략 관리자)

```rust
struct StrategyManager {
    active_strategy: Box<dyn Strategy>,
    available_strategies: HashMap<String, Box<dyn Strategy>>,
}

impl StrategyManager {
    fn switch_strategy(&mut self, name: &str) -> Result<()>;
    fn get_active_strategy(&self) -> &dyn Strategy;
    fn list_strategies(&self) -> Vec<String>;
}
```

### 5.4 Backtesting Module (백테스팅 모듈)

#### 5.4.1 Data Fetching (데이터 수집)

```rust
struct HistoricalDataFetcher {
    client: UpbitClient,
    cache: SQLiteCache,
}

impl HistoricalDataFetcher {
    async fn fetch_candles(
        &self,
        market: &str,
        unit: TimeUnit,    // Minute, Hour, Day
        count: usize,      // 최대 200개 (Upbit 제한)
    ) -> Result<Vec<Candle>>;

    async fn fetch_range(
        &self,
        market: &str,
        start: DateTime,
        end: DateTime,
    ) -> Result<Vec<Candle>>;  // 청크 기반 로딩
}
```

#### 5.4.2 Simulator (시뮬레이터)

```rust
struct BacktestSimulator {
    initial_balance: f64,
    commission_rate: f64,  // 0.0005 (0.05%)
    strategy: Box<dyn Strategy>,
}

struct BacktestResult {
    total_trades: usize,
    winning_trades: usize,
    losing_trades: usize,
    roi: f64,                    // 총 수익률
    win_rate: f64,               // 승률
    max_drawdown: f64,           // 최대 낙폭
    sharpe_ratio: f64,           // 샤프 비율 (선택)
    trades: Vec<Trade>,
    equity_curve: Vec<f64>,      // 자산 곡선
}

impl BacktestSimulator {
    fn run(
        &self,
        candles: Vec<Candle>,
    ) -> Result<BacktestResult>;

    fn calculate_metrics(&self, result: &BacktestResult) -> PerformanceMetrics;
}
```

#### 5.4.3 Performance Metrics (성과 메트릭)

```rust
struct PerformanceMetrics {
    // 수익성
    total_return: f64,           // 총 수익률
    annualized_return: f64,      // 연환산 수익률
    cagr: f64,                   // 연평균 성장률

    // 리스크
    max_drawdown: f64,           // 최대 낙폭
    volatility: f64,             // 변동성
    var_95: f64,                 // 95% VaR (Value at Risk)

    // 효율성
    sharpe_ratio: f64,           // 샤프 비율
    sortino_ratio: f64,          // 소르티노 비율
    calmar_ratio: f64,           // 칼마 비율

    // 거래 분석
    win_rate: f64,               // 승률
    profit_factor: f64,          // 수익/손실 비율
    avg_win: f64,                // 평균 수익
    avg_loss: f64,               // 평균 손실
    total_trades: usize,
}
```

#### 5.4.4 Parameter Optimizer (파라미터 최적화)

```rust
struct ParameterOptimizer {
    strategy: Box<dyn Strategy>,
    param_ranges: HashMap<String, ParamRange>,
    optimization_target: OptimizationTarget,  // ROI, Sharpe, WinRate
}

enum ParamRange {
    Integer { min: i32, max: i32, step: i32 },
    Float { min: f64, max: f64, step: f64 },
}

impl ParameterOptimizer {
    // 그리드 서치
    fn grid_search(
        &self,
        candles: Vec<Candle>,
    ) -> Result<Vec<OptimizationResult>>;

    // 최적의 파라미터 반환
    fn find_best(&self, results: Vec<OptimizationResult>) -> Option<Parameters>;
}

struct OptimizationResult {
    parameters: HashMap<String, f64>,
    metrics: PerformanceMetrics,
}
```

### 5.5 Data Models (데이터 모델)

#### 5.5.1 Extended Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
enum IndicatorType {
    RSI,
    MACD,
    BollingerBands,
    SMA,
    EMA,
    MultiIndicator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IndicatorValue {
    indicator: IndicatorType,
    value: f64,
    timestamp: i64,
    metadata: Option<HashMap<String, f64>>,  // 기간, 파라미터 등
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Candle {
    market: String,
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Trade {
    entry_time: i64,
    exit_time: i64,
    entry_price: f64,
    exit_price: f64,
    quantity: f64,
    profit: f64,
    profit_pct: f64,
}
```

### 5.6 API Design (API 설계)

#### 5.6.1 Indicator API

```rust
#[async_trait]
pub trait Indicator {
    fn update(&mut self, candle: &Candle) -> Option<IndicatorValue>;
    fn value(&self) -> Option<IndicatorValue>;
    fn reset(&mut self);
    fn name(&self) -> &str;
}
```

#### 5.6.2 Strategy API

```rust
#[async_trait]
pub trait Strategy {
    fn on_candle(&mut self, candle: &Candle) -> Option<Signal>;
    fn get_name(&self) -> &str;
    fn get_parameters(&self) -> HashMap<String, f64>;
    fn set_parameters(&mut self, params: HashMap<String, f64>) -> Result<()>;
}
```

---

## 6. Non-Functional Requirements (비기능 요구사항)

### 6.1 Performance

| 항목 | 목표 |
|------|------|
| 지표 계산 지연 | < 100ms (티커당) |
| 백테스팅 1년 데이터 | < 10초 |
| 파라미터 최적화 (100 조합) | < 5분 |

### 6.2 Memory

| 항목 | 목표 |
|------|------|
| 지표 캐시 | < 100MB |
| 백테스팅 메모리 | 청크 기반 로딩 (최대 1000캔들) |

### 6.3 Accuracy

| 항목 | 목표 |
|------|------|
| 지표 계산 정확도 | 표준 공식과 99.9% 일치 |
| 백테스팅 재현성 | 동일 입력 시 100% 일치 |

### 6.4 Extensibility

| 항목 | 구현 |
|------|------|
| 새 지표 추가 | Trait 구현으로 확장 |
| 새 전략 추가 | Strategy Trait 구현 |

---

## 7. Traceability (추적성)

| REQ | 관련 모듈 | 테스트 시나리오 |
|-----|----------|----------------|
| REQ-301 | 모든 지표 | invalid_parameter_test |
| REQ-302 | Backtest | persistence_test |
| REQ-303 | 모든 지표 | formula_accuracy_test |
| REQ-304 | Indicator Cache | cache_hit_test |
| REQ-305 | RSI | rsi_oversold_test |
| REQ-306 | RSI | rsi_overbought_test |
| REQ-307 | MACD | macd_bullish_cross_test |
| REQ-308 | MACD | macd_bearish_cross_test |
| REQ-309 | Bollinger | bollinger_lower_touch_test |
| REQ-310 | Bollinger | bollinger_upper_touch_test |
| REQ-311 | Moving Average | golden_cross_test |
| REQ-312 | Simulator | backtest_completion_test |
| REQ-313 | Optimizer | parameter_optimization_test |
| REQ-314 | Multi-Indicator | signal_scoring_test |
| REQ-315 | 모든 지표 | default_parameter_fallback_test |
| REQ-316 | Simulator | insufficient_data_skip_test |
| REQ-317 | Indicator Cache | cache_return_test |
| REQ-318 | Strategy Manager | strategy_switch_test |
| REQ-319 | Decision Maker | incomplete_data_protection_test |
| REQ-320 | Documentation | backtest_disclaimer_test |
| REQ-321 | 모든 지표 | validation_before_calculation_test |
| REQ-322 | Optimizer | grid_search_test |
| REQ-323 | Visualization | chart_generation_test |
| REQ-324 | Strategy Manager | parallel_comparison_test |

---

**Traceability**: `SPEC-ID: SPEC-TRADING-003` → Specification Phase Complete
