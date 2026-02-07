# Implementation Plan: SPEC-TRADING-003

**TAG BLOCK**: `SPEC-ID: SPEC-TRADING-003`

---

## 1. Milestones (마일스톤)

### Milestone 1: Indicator Foundation (지표 기반) - Priority High

기술적 지표를 위한 기반 인프라를 구축합니다.

- **1.1** Indicator Trait 정의
- **1.2** 지표 캐싱 시스템 구현
- **1.3** SQLite에 지표 스토리지 스키마 추가
- **1.4** Candle 타입 정의
- **1.5** 기본 테스트 프레임워크 구현

### Milestone 2: RSI Implementation - Priority High

RSI (Relative Strength Index) 지표를 구현합니다.

- **2.1** RSI 구조체 및 Trait 구현
- **2.2** RS(Relative Strength) 계산 로직
- **2.3** 과매수/과매도 신호 생성
- **2.4** 파라미터 검증 (기간: 1~100)
- **2.5** 단위 테스트 작성

### Milestone 3: MACD Implementation - Priority High

MACD (Moving Average Convergence Divergence) 지표를 구현합니다.

- **3.1** EMA(Exponential Moving Average) 구현
- **3.2** MACD 구조체 및 Trait 구현
- **3.3** MACD 라인, 시그널 라인, 히스토그램 계산
- **3.4** 불리시/베어리시 교차 신호 생성
- **3.5** 단위 테스트 작성

### Milestone 4: Bollinger Bands Implementation - Priority High

Bollinger Bands 지표를 구현합니다.

- **4.1** SMA(Simple Moving Average) 구현
- **4.2** 표준편차 계산 로직
- **4.3** Bollinger Bands 구조체 및 Trait 구현
- **4.4** 상단/하단 밴드 터치 신호 생성
- **4.5** 밴드폭(squeeze) 감지
- **4.6** 단위 테스트 작성

### Milestone 5: Moving Averages Implementation - Priority Medium

다양한 이동평균선 지표를 구현합니다.

- **5.1** SMA 여러 기간 지원 (5, 10, 20, 50, 100, 200)
- **5.2** EMA 여러 기간 지원 (12, 26, 50)
- **5.3** 골든크로스/데드크로스 감지
- **5.4** 이동평균 기반 지지선/저항선 계산
- **5.5** 단위 테스트 작성

### Milestone 6: Multi-Indicator Strategy - Priority High

다중 지표 결합 전략을 구현합니다.

- **6.1** IndicatorSignal 구조체 정의
- **6.2** Signal Scoring System 구현
- **6.3** 가중치 설정 기능
- **6.4** 종합 점수 계산 로직
- **6.5** MultiIndicatorStrategy 구조체 구현
- **6.6** 통합 테스트 작성

### Milestone 7: Historical Data Fetcher - Priority High

과거 데이터 수집 기능을 구현합니다.

- **7.1** Upbit REST API 캔들 엔드포인트 연동
- **7.2** 청크 기반 데이터 로딩 (최대 200개/요청)
- **7.3** 날짜 범위 기반 데이터 조회
- **7.4** 데이터 캐싱 (SQLite)
- **7.5** 데이터 검증 로직

### Milestone 8: Backtest Simulator - Priority High

백테스팅 시뮬레이터를 구현합니다.

- **8.1** BacktestSimulator 구조체 구현
- **8.2** 가상 포지션 관리
- **8.3** 수수료 계산 (0.05%)
- **8.4** 거래 실행 로직
- **8.5** Trade 이력 저장
- **8.6** 자산 곡선(equity curve) 계산
- **8.7** 통합 테스트 작성

### Milestone 9: Performance Metrics - Priority Medium

성과 메트릭 계산을 구현합니다.

- **9.1** ROI, Win Rate 계산
- **9.2** Max Drawdown 계산
- **9.3** Sharpe Ratio 계산
- **9.4** Profit Factor 계산
- **9.5** PerformanceMetrics 구조체 구현
- **9.6** 단위 테스트 작성

### Milestone 10: Parameter Optimizer - Priority Medium

파라미터 최적화 도구를 구현합니다.

- **10.1** ParamRange 정의 (Integer, Float)
- **10.2** Grid Search 알고리즘 구현
- **10.3** 병렬 처리 지원 (rayon 또는 tokio)
- **10.4** 최적 파라미터 선택 로직
- **10.5** OptimizationResult 저장
- **10.6** 단위 테스트 작성

### Milestone 11: Strategy Manager - Priority Medium

전략 관리자를 구현합니다.

- **11.1** StrategyManager 구조체 구현
- **11.2** 전략 등록 시스템
- **11.3** 전략 전환 기능
- **11.4** 전략별 파라미터 관리
- **11.5** 활성 전략 조회
- **11.6** 통합 테스트 작성

### Milestone 12: Integration & Testing - Priority High

통합 및 테스트를 수행합니다.

- **12.1** 기존 시스템(SPEC-TRADING-001)과 통합
- **12.2** 엔드투엔드 백테스팅 테스트
- **12.3** 지표 정확도 검증 (표준 라이브러리와 비교)
- **12.4** 성능 벤치마크
- **12.5** 스트레스 테스트 (대용량 데이터)

### Milestone 13: Documentation - Priority Low

문서화를 작성합니다.

- **13.1** 지표 공식 문서화
- **13.2** 백테스팅 사용 가이드
- **13.3** API 문서 업데이트
- **13.4** README에 예시 추가
- **13.5** 백테스팅 결과 해석 가이드

---

## 2. Technical Approach (기술적 접근)

### 2.1 Project Structure Changes (프로젝트 구조 변경)

```
autocoin/
├── src/
│   ├── indicators/           # 신규
│   │   ├── mod.rs
│   │   ├── rsi.rs
│   │   ├── macd.rs
│   │   ├── bollinger.rs
│   │   └── moving_average.rs
│   ├── backtest/             # 신규
│   │   ├── mod.rs
│   │   ├── simulator.rs
│   │   ├── metrics.rs
│   │   └── optimizer.rs
│   ├── strategy/             # 확장
│   │   ├── mod.rs            # 기존
│   │   ├── momentum.rs       # 기존
│   │   ├── multi_indicator.rs   # 신규
│   │   └── strategy_manager.rs  # 신규
│   └── types/                # 확장
│       └── trading.rs        # IndicatorType, Candle 추가
└── tests/
    └── indicators/           # 신규
        ├── rsi_test.rs
        ├── macd_test.rs
        ├── bollinger_test.rs
        └── backtest_test.rs
```

### 2.2 Dependencies (의존성)

기존 Cargo.toml에 추가:

```toml
[dependencies]
# 기존 의존성 유지

# 백테스팅을 위한 통계/수학 라이브러리 (선택)
# statrs = "0.17"  # 통계 계산용

# 병렬 처리 (선택)
# rayon = "1.10"   # 데이터 병렬 처리

# 시각화 (선택, REQ-323)
# plotters = "0.3"  # 차트 생성
```

### 2.3 Database Schema Changes (데이터베이스 스키마 변경)

```sql
-- 지표 캐시 테이블
CREATE TABLE IF NOT EXISTS indicator_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    market TEXT NOT NULL,
    indicator_type TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    value REAL NOT NULL,
    metadata TEXT,  -- JSON 형식
    created_at INTEGER NOT NULL,
    UNIQUE(market, indicator_type, timestamp)
);

-- 백테스팅 결과 테이블
CREATE TABLE IF NOT EXISTS backtest_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    strategy_name TEXT NOT NULL,
    market TEXT NOT NULL,
    start_date INTEGER NOT NULL,
    end_date INTEGER NOT NULL,
    total_trades INTEGER NOT NULL,
    winning_trades INTEGER NOT NULL,
    losing_trades INTEGER NOT NULL,
    roi REAL NOT NULL,
    win_rate REAL NOT NULL,
    max_drawdown REAL NOT NULL,
    sharpe_ratio REAL,
    parameters TEXT NOT NULL,  -- JSON 형식
    created_at INTEGER NOT NULL
);

-- 백테스팅 거래 이력 테이블
CREATE TABLE IF NOT EXISTS backtest_trades (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    backtest_id INTEGER NOT NULL,
    entry_time INTEGER NOT NULL,
    exit_time INTEGER NOT NULL,
    entry_price REAL NOT NULL,
    exit_price REAL NOT NULL,
    quantity REAL NOT NULL,
    profit REAL NOT NULL,
    profit_pct REAL NOT NULL,
    FOREIGN KEY (backtest_id) REFERENCES backtest_results(id)
);

-- 인덱스
CREATE INDEX IF NOT EXISTS idx_indicator_cache_lookup
ON indicator_cache(market, indicator_type, timestamp);

CREATE INDEX IF NOT EXISTS idx_backtest_results_strategy
ON backtest_results(strategy_name, market);
```

### 2.4 Design Patterns (디자인 패턴)

#### Trait-Based Polymorphism

```rust
#[async_trait]
pub trait Indicator: Send + Sync {
    // 모든 지표가 구현해야 할 공통 인터페이스
    fn update(&mut self, candle: &Candle) -> Option<IndicatorValue>;
    fn value(&self) -> Option<IndicatorValue>;
    fn reset(&mut self);
    fn name(&self) -> &str;

    // 기본 구현 제공
    fn is_ready(&self) -> bool {
        self.value().is_some()
    }
}
```

#### Strategy Pattern

```rust
pub trait Strategy: Send + Sync {
    fn on_candle(&mut self, candle: &Candle) -> Option<Signal>;
    fn get_name(&self) -> &str;
    fn get_parameters(&self) -> HashMap<String, f64>;
    fn set_parameters(&mut self, params: HashMap<String, f64>) -> Result<()>;
}

// 구현 예시
impl Strategy for MultiIndicatorStrategy {
    fn on_candle(&mut self, candle: &Candle) -> Option<Signal> {
        // 다중 지표 기반 신호 생성
    }
}
```

#### Builder Pattern for Configuration

```rust
struct MultiIndicatorStrategyBuilder {
    indicators: Vec<Box<dyn Indicator>>,
    weights: HashMap<IndicatorType, f64>,
    threshold: f64,
}

impl MultiIndicatorStrategyBuilder {
    fn new() -> Self {
        Self {
            indicators: Vec::new(),
            weights: HashMap::new(),
            threshold: 0.6,
        }
    }

    fn add_indicator(mut self, indicator: Box<dyn Indicator>, weight: f64) -> Self {
        let name = indicator.name().to_string();
        self.indicators.push(indicator);
        self.weights.insert(name, weight);
        self
    }

    fn threshold(mut self, value: f64) -> Self {
        self.threshold = value;
        self
    }

    fn build(self) -> Result<MultiIndicatorStrategy> {
        if self.indicators.is_empty() {
            return Err(Error::NoIndicators);
        }
        Ok(MultiIndicatorStrategy {
            indicators: self.indicators,
            weights: self.weights,
            threshold: self.threshold,
        })
    }
}
```

### 2.5 Testing Strategy (테스트 전략)

#### Unit Tests (단위 테스트)

각 지표별 정확도 검증:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsi_formula() {
        // 알려진 입력값으로 RSI 계산 검증
        let mut rsi = RSI::new(14);
        // 표준 데이터 세트 입력
        for price in &test_data() {
            rsi.update(price);
        }
        // 예상 RSI 값과 비교
        assert!((rsi.value().unwrap() - expected_value).abs() < 0.01);
    }
}
```

#### Integration Tests (통합 테스트)

백테스팅 플로우 테스트:

```rust
#[tokio::test]
async fn test_backtest_flow() {
    // 1. 과거 데이터 로드
    let candles = fetch_test_data().await;

    // 2. 전략 설정
    let strategy = create_test_strategy();

    // 3. 백테스팅 실행
    let simulator = BacktestSimulator::new(1_000_000.0, 0.0005, strategy);
    let result = simulator.run(candles).unwrap();

    // 4. 결과 검증
    assert!(result.total_trades > 0);
    assert!(!result.trades.is_empty());
}
```

#### Property-Based Testing (속성 기반 테스트)

```rust
// 지표 계산 불변성 검증
#[test]
fn test_rsi_bounds() {
    let mut rsi = RSI::new(14);
    for _ in 0..1000 {
        let price = random_price();
        rsi.update(price);
        if let Some(value) = rsi.value() {
            assert!(value >= 0.0 && value <= 100.0, "RSI must be 0-100");
        }
    }
}
```

### 2.6 Performance Optimization (성능 최적화)

#### Indicator Caching (지표 캐싱)

```rust
struct CachedIndicator<T> {
    inner: T,
    cache: HashMap<i64, IndicatorValue>,
}

impl<T: Indicator> Indicator for CachedIndicator<T> {
    fn update(&mut self, candle: &Candle) -> Option<IndicatorValue> {
        if let Some(cached) = self.cache.get(&candle.timestamp) {
            return Some(cached.clone());
        }

        let value = self.inner.update(candle)?;
        self.cache.insert(candle.timestamp, value.clone());
        Some(value)
    }
}
```

#### Chunked Backtesting (청크 기반 백테스팅)

```rust
impl BacktestSimulator {
    fn run_chunked(&self, candles: Vec<Candle>, chunk_size: usize) -> Result<BacktestResult> {
        let mut all_results = Vec::new();

        for chunk in candles.chunks(chunk_size) {
            let result = self.run(chunk.to_vec())?;
            all_results.push(result);
        }

        // 결과 병합
        self.merge_results(all_results)
    }
}
```

#### Parallel Optimization (병렬 최적화)

```rust
// rayon을 사용한 그리드 서치 병렬화
use rayon::prelude::*;

impl ParameterOptimizer {
    fn grid_search_parallel(&self, candles: Vec<Candle>) -> Result<Vec<OptimizationResult>> {
        let param_combinations = self.generate_combinations();

        let results: Vec<_> = param_combinations
            .par_iter()  // 병렬 반복자
            .map(|params| {
                let mut strategy = self.strategy.clone_with_params(params);
                let simulator = BacktestSimulator::new(/* ... */);
                let result = simulator.run(candles.clone()).unwrap();
                OptimizationResult {
                    parameters: params.clone(),
                    metrics: result.calculate_metrics(),
                }
            })
            .collect();

        Ok(results)
    }
}
```

---

## 3. Dependencies Between Milestones

```
M1 (Indicator Foundation)
    ├─→ M2 (RSI)
    ├─→ M3 (MACD)
    │       └─→ M5 (Moving Averages - EMA 재사용)
    ├─→ M4 (Bollinger Bands)
    │       └─→ M5 (Moving Averages - SMA 재사용)
    └─→ M6 (Multi-Indicator Strategy)
            └─→ M11 (Strategy Manager)

M2, M3, M4, M5 ──→ M6 (Multi-Indicator)

M7 (Historical Data Fetcher)
    └─→ M8 (Backtest Simulator)
            └─→ M9 (Performance Metrics)
                    └─→ M10 (Parameter Optimizer)

M8 ──→ M12 (Integration & Testing)

M6, M11 ──→ M12 (Integration & Testing)

M12 ──→ M13 (Documentation)
```

---

## 4. Risks and Mitigation (리스크 및 대응)

### 4.1 Technical Risks

| 리스크 | 영향 | 확률 | 대응 방안 |
|--------|------|------|----------|
| 지표 계산 오차 | 높음 | 중간 | 표준 라이브러리와 비교 검증 |
| 백테스팅 성능 저하 | 중간 | 높음 | 청크 처리, 캐싱, 병렬화 |
| 메모리 과다 사용 | 중간 | 중간 | 청크 기반 로딩, 캐시 크기 제한 |
| 데이터 불일치 | 높음 | 낮음 | 데이터 검증, 타임스탬프 정렬 |

### 4.2 Business Risks

| 리스크 | 영향 | 확률 | 대응 방안 |
|--------|------|------|----------|
| 백테스팅 과대평가 | 높음 | 높음 | Paper Trading으로 2차 검증 |
| 파라미터 과최적화 | 높음 | 중간 | 트레이닝/검증 데이터 분리 |
| 실제 성과와 차이 | 높음 | 높음 | SLippage 모델링, 시장 충격 고려 |

### 4.3 Operational Risks

| 리스크 | 영향 | 확률 | 대응 방안 |
|--------|------|------|----------|
| Upbit API 과거 데이터 제한 | 중간 | 중간 | 최대 200개/요청, 청크 처리 |
| 전략 전환 오류 | 중간 | 낮음 | 파라미터 검증, 롤백 메커니즘 |

---

## 5. Success Criteria (성공 기준)

### 5.1 Functional Criteria

- [ ] RSI, MACD, Bollinger Bands, SMA/EMA 지표 구현 완료
- [ ] 다중 지표 결합 전략 동작
- [ ] 백테스팅 엔진으로 1년치 데이터 시뮬레이션 가능
- [ ] 파라미터 최적화로 최적 조합 탐색 가능
- [ ] 전략 전환 기능 동작

### 5.2 Non-Functional Criteria

- [ ] 지표 계산 < 100ms (티커당)
- [ ] 백테스팅 1년 데이터 < 10초
- [ ] 단위 테스트 커버리지 85% 이상
- [ ] 지표 공식 정확도 99.9%
- [ ] 메모리 사용량 500MB 이하

---

## 6. Next Steps (다음 단계)

1. **기존 시스템 분석**: SPEC-TRADING-001의 Signal Detector와 Strategy 모듈 확인
2. **개발 환경 설정**: 테스트용 과거 데이터 확보
3. **Milestone 1 시작**: Indicator Trait 및 기반 구현
4. **지표 순차 구현**: M2 → M3 → M4 → M5
5. **백테스팅 구현**: M7 → M8 → M9 → M10
6. **통합 및 검증**: M12로 전체 시스템 검증

---

**Traceability**: `SPEC-ID: SPEC-TRADING-003` → Plan Phase Complete
