# Backend Architecture: SPEC-TRADING-003 Backtest Module

## Overview

This document describes the backtest module implementation for the AutoCoin trading system, following SPEC-TRADING-003 Section 5.4.

## Module Structure

```
src/backtest/
├── mod.rs          # Module exports, BacktestConfig
├── simulator.rs    # BacktestSimulator, BacktestResult, Trade
├── metrics.rs      # PerformanceMetrics, OptimizationTarget
└── optimizer.rs    # ParameterOptimizer, ParamRange
```

## Framework: Rust (1.93+)

### Type System
- Uses Rust's strong typing with `Send + Sync` bounds for thread safety
- Generic `Strategy` trait for pluggable trading strategies
- `HashMap<String, f64>` for flexible parameter storage

### Error Handling
- Uses `crate::error::{Result, TradingError}` for consistent error handling
- Returns `TradingError::InvalidParameter` for validation failures

## API Specification

### BacktestConfig

```rust
pub struct BacktestConfig {
    pub initial_balance: f64,     // Initial KRW balance (default: 1,000,000)
    pub commission_rate: f64,     // Commission rate (default: 0.0005 = 0.05%)
    pub slippage: f64,            // Slippage (default: 0.0)
    pub min_order_amount: f64,    // Minimum order amount (default: 5,000 KRW)
}
```

**Builder Pattern:**
```rust
let config = BacktestConfig::new(2_000_000.0)
    .with_commission(0.001)
    .with_slippage(0.001)
    .with_min_order(10000.0);
```

### BacktestSimulator

```rust
pub struct BacktestSimulator {
    config: BacktestConfig,
}

impl BacktestSimulator {
    pub fn new(config: BacktestConfig) -> Self;
    pub fn with_default_config() -> Self;

    pub fn run<S: Strategy>(
        &self,
        candles: Vec<Candle>,
        strategy: &mut S,
    ) -> Result<BacktestResult>;
}
```

### BacktestResult

```rust
pub struct BacktestResult {
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub roi: f64,
    pub win_rate: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub trades: Vec<Trade>,
    pub equity_curve: Vec<EquityPoint>,
    pub initial_balance: f64,
    pub final_balance: f64,
}

impl BacktestResult {
    pub fn calculate_metrics(&self) -> PerformanceMetrics;
}
```

### PerformanceMetrics

```rust
pub struct PerformanceMetrics {
    // Return metrics
    pub total_return: f64,
    pub annualized_return: f64,
    pub cagr: f64,

    // Risk metrics
    pub max_drawdown: f64,
    pub volatility: f64,
    pub var_95: f64,

    // Efficiency ratios
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub calmar_ratio: f64,

    // Trade analysis
    pub win_rate: f64,
    pub profit_factor: f64,
    pub avg_win: f64,
    pub avg_loss: f64,
    pub total_trades: usize,
}

impl PerformanceMetrics {
    pub fn overall_score(&self) -> f64;
    pub fn summary(&self) -> String;
    pub fn to_map(&self) -> HashMap<String, f64>;
}
```

### ParameterOptimizer

```rust
pub struct ParameterOptimizer {
    param_ranges: HashMap<String, ParamRange>,
    optimization_target: OptimizationTarget,
    backtest_config: BacktestConfig,
    max_parallel_jobs: usize,
}

pub enum ParamRange {
    Integer { min: i32, max: i32, step: i32 },
    Float { min: f64, max: f64, step: f64 },
}

pub enum OptimizationTarget {
    TotalReturn,
    SharpeRatio,
    WinRate,
    MinDrawdown,
    OverallScore,
    ProfitFactor,
}

impl ParameterOptimizer {
    pub fn grid_search<S: Strategy + Clone + Send + 'static>(
        &self,
        candles: Vec<Candle>,
        strategy_factory: impl Fn() -> S + Send + Sync + Clone + 'static,
    ) -> Result<Vec<OptimizationResult>>;

    pub fn find_best(&self, results: &[OptimizationResult]) -> Option<ParameterSet>;
}
```

## Database Schema

### Backtest Results Table (SQLite)

```sql
CREATE TABLE backtest_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    strategy_name TEXT NOT NULL,
    parameters TEXT NOT NULL,  -- JSON serialized
    initial_balance REAL NOT NULL,
    final_balance REAL NOT NULL,
    total_trades INTEGER NOT NULL,
    winning_trades INTEGER NOT NULL,
    roi REAL NOT NULL,
    win_rate REAL NOT NULL,
    max_drawdown REAL NOT NULL,
    sharpe_ratio REAL NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

## Testing Strategy

### Unit Tests

Each module includes comprehensive unit tests:

1. **simulator.rs** (947 lines)
   - `test_trade_creation`: Verify trade record generation
   - `test_trade_winning_losing`: Verify profit/loss calculation
   - `test_backtest_insufficient_data`: REQ-316 compliance
   - `test_backtest_with_sufficient_data`: Full simulation test
   - `test_equity_curve_generation`: Verify equity curve integrity

2. **metrics.rs** (562 lines)
   - `test_sharpe_rating_*`: Verify Sharpe ratio categorization
   - `test_overall_score`: Verify score calculation
   - `test_performance_comparison`: Verify comparison logic
   - `test_optimization_target_extract`: Verify metric extraction

3. **optimizer.rs** (810 lines)
   - `test_param_range_*`: Verify parameter range generation
   - `test_generate_combinations`: Verify grid search combinations
   - `test_too_many_combinations`: Verify combination limit enforcement
   - `test_grid_search_small`: Integration test with mock strategy

### Test Coverage Target: 85%+

## Integration with Existing System

### Public API (via lib.rs)

```rust
pub use backtest::{
    BacktestConfig,
    BacktestResult,
    BacktestSimulator,
    OptimizationResult,
    OptimizationTarget,
    ParamRange,
    ParameterOptimizer,
    PerformanceMetrics,
    Trade,
};
```

### Dependencies

- `crate::types::Candle` - Candle data structure
- `crate::types::Signal` - Trading signals
- `crate::strategy::Strategy` - Strategy trait
- `crate::error::{Result, TradingError}` - Error handling

### New Dependencies Added

```toml
rand = "0.8"      # For RandomSearchOptimizer
num_cpus = "1.16" # For parallel execution
```

## Performance Considerations

1. **Chunk-based Loading**: Backtest processes candles sequentially to control memory
2. **Parallel Optimization**: Uses `rayon` or thread pools for grid search
3. **Combination Limit**: Maximum 100,000 combinations to prevent runaway execution

## SPEC Compliance

### REQ-301: Parameter Validation
- Implemented in `validate_indicator_params()` in indicators module
- Strategy trait requires `set_parameters()` validation

### REQ-312: Backtest Completion Reporting
- `BacktestResult::calculate_metrics()` generates comprehensive report
- Results can be serialized to JSON for database storage

### REQ-313: Parameter Optimization Result
- `ParameterOptimizer::find_best()` returns optimal parameters
- Full result set available for analysis

### REQ-316: Insufficient Data Handling
- `BacktestSimulator::run()` returns error if candles < 100
- Error message clearly states minimum requirement

### REQ-322: Grid Search Implementation
- `ParameterOptimizer::grid_search()` performs exhaustive search
- Progress callback available for UI updates

## Usage Example

```rust
use autocoin::{
    BacktestConfig, BacktestSimulator, StrategyFactory,
    ParameterOptimizer, ParamRange, OptimizationTarget,
};

// Simple backtest
let config = BacktestConfig::new(1_000_000.0);
let simulator = BacktestSimulator::new(config);
let mut strategy = StrategyFactory::momentum();

let result = simulator.run(candles, &mut strategy)?;
println!("ROI: {:.2}%, Sharpe: {:.2}", result.roi, result.sharpe_ratio);

// Parameter optimization
let mut param_ranges = HashMap::new();
param_ranges.insert("surge_threshold".to_string(),
    ParamRange::float(0.01, 0.1, 0.01));
param_ranges.insert("volume_multiplier".to_string(),
    ParamRange::float(1.0, 5.0, 0.5));

let optimizer = ParameterOptimizer::new(param_ranges, OptimizationTarget::SharpeRatio);
let results = optimizer.grid_search(candles, || StrategyFactory::momentum())?;

if let Some(best_params) = optimizer.find_best(&results) {
    println!("Best parameters: {:?}", best_params);
}
```

## File Summary

| File | Lines | Description |
|------|-------|-------------|
| mod.rs | 94 | Module exports, BacktestConfig |
| simulator.rs | 947 | BacktestSimulator, BacktestResult, Trade implementation |
| metrics.rs | 562 | PerformanceMetrics, OptimizationTarget, PerformanceComparison |
| optimizer.rs | 810 | ParameterOptimizer, grid search, random search |
| **Total** | **2,413** | Complete backtest module |

---

Generated: 2026-02-07
SPEC: SPEC-TRADING-003
Section: 5.4 Backtesting Module
