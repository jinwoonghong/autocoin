//! Parameter Optimizer
//!
//! 트레이딩 전략 파라미터 최적화를 수행합니다.
//! SPEC-TRADING-003 Section 5.4.4

use super::{
    metrics::{OptimizationTarget, PerformanceMetrics},
    simulator::{BacktestResult, BacktestSimulator},
    BacktestConfig,
};
use crate::error::{Result, TradingError};
use crate::strategy::Strategy;
use crate::types::Candle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

/// 파라미터 범위
///
/// 최적화할 파라미터의 타입과 범위를 정의합니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParamRange {
    /// 정수 범위
    Integer {
        min: i32,
        max: i32,
        step: i32,
    },
    /// 실수 범위
    Float {
        min: f64,
        max: f64,
        step: f64,
    },
}

impl ParamRange {
    /// 정수 범위 생성
    pub fn integer(min: i32, max: i32, step: i32) -> Self {
        ParamRange::Integer { min, max, step }
    }

    /// 실수 범위 생성
    pub fn float(min: f64, max: f64, step: f64) -> Self {
        ParamRange::Float { min, max, step }
    }

    /// 가능한 모든 값 생성 (그리드 서치용)
    pub fn generate_values(&self) -> Vec<f64> {
        match self {
            ParamRange::Integer { min, max, step } => {
                let mut values = Vec::new();
                let mut current = *min;
                while current <= *max {
                    values.push(current as f64);
                    current += step;
                }
                values
            }
            ParamRange::Float { min, max, step } => {
                let mut values = Vec::new();
                let mut current = *min;
                while current <= *max {
                    values.push(current);
                    current += step;
                }
                values
            }
        }
    }

    /// 범위 내 값의 개수 반환
    pub fn count(&self) -> usize {
        match self {
            ParamRange::Integer { min, max, step } => {
                if *step == 0 {
                    return 0;
                }
                ((max - min) / step).unsigned_abs() as usize + 1
            }
            ParamRange::Float { min, max, step } => {
                if *step == 0.0 {
                    return 0;
                }
                ((max - min) / step).abs() as usize + 1
            }
        }
    }

    /// 값이 범위 내에 있는지 확인
    pub fn contains(&self, value: f64) -> bool {
        match self {
            ParamRange::Integer { min, max, .. } => {
                value >= *min as f64 && value <= *max as f64 && value.fract() == 0.0
            }
            ParamRange::Float { min, max, .. } => value >= *min && value <= *max,
        }
    }
}

/// 파라미터 조합
pub type ParameterSet = HashMap<String, f64>;

/// 최적화 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    /// 파라미터 조합
    pub parameters: ParameterSet,
    /// 성과 메트릭
    pub metrics: PerformanceMetrics,
    /// 목표 함수 값
    pub objective_value: f64,
}

impl OptimizationResult {
    /// 새로운 최적화 결과 생성
    pub fn new(parameters: ParameterSet, metrics: PerformanceMetrics, objective_value: f64) -> Self {
        Self {
            parameters,
            metrics,
            objective_value,
        }
    }

    /// 목표 함수 값으로 비교
    pub fn is_better_than(&self, other: &OptimizationResult) -> bool {
        self.objective_value > other.objective_value
    }
}

/// 최적화 진행 상황 콜백
pub type ProgressCallback = Box<dyn Fn(usize, usize) + Send + Sync>;

/// 파라미터 최적화 도구
///
/// 그리드 서치를 통해 전략 파라미터를 최적화합니다.
/// REQ-313: 파라미터 최적화 완료 시 최적 파라미터 반환
/// REQ-322: 그리드 서치 제공
pub struct ParameterOptimizer {
    /// 파라미터 범위 정의
    param_ranges: HashMap<String, ParamRange>,
    /// 최적화 목표
    optimization_target: OptimizationTarget,
    /// 백테스트 설정
    backtest_config: BacktestConfig,
    /// 최대 병렬 작업 수
    max_parallel_jobs: usize,
}

impl ParameterOptimizer {
    /// 새로운 최적화 도구 생성
    pub fn new(
        param_ranges: HashMap<String, ParamRange>,
        optimization_target: OptimizationTarget,
    ) -> Self {
        Self {
            param_ranges,
            optimization_target,
            backtest_config: BacktestConfig::default(),
            max_parallel_jobs: num_cpus::get(),
        }
    }

    /// 백테스트 설정
    pub fn with_backtest_config(mut self, config: BacktestConfig) -> Self {
        self.backtest_config = config;
        self
    }

    /// 최대 병렬 작업 수 설정
    pub fn with_max_parallel_jobs(mut self, max: usize) -> Self {
        self.max_parallel_jobs = max.max(1);
        self
    }

    /// 그리드 서치 실행 (REQ-322)
    ///
    /// 모든 파라미터 조합에 대해 백테스트를 실행하고 최적의 파라미터를 찾습니다.
    ///
    /// # Errors
    ///
    /// * `TradingError::InvalidParameter` - 파라미터 범위가 유효하지 않은 경우
    pub fn grid_search<S: Strategy + Clone + Send + 'static>(
        &self,
        candles: Vec<Candle>,
        strategy_factory: impl Fn() -> S + Send + Sync + Clone + 'static,
    ) -> Result<Vec<OptimizationResult>> {
        // 파라미터 조합 생성
        let param_combinations = self.generate_combinations()?;

        if param_combinations.is_empty() {
            return Ok(Vec::new());
        }

        let total_combinations = param_combinations.len();

        // 병렬 실행을 위한 준비
        let candles = Arc::new(candles);
        let results = Arc::new(Mutex::new(Vec::with_capacity(total_combinations)));
        let backtest_config = self.backtest_config.clone();
        let optimization_target = self.optimization_target;

        // 챙크로 나누어 병렬 실행
        let chunk_size = (total_combinations + self.max_parallel_jobs - 1) / self.max_parallel_jobs;

        let mut handles = Vec::new();

        for chunk in param_combinations.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let candles = Arc::clone(&candles);
            let results = Arc::clone(&results);
            let strategy_factory = strategy_factory.clone();
            let backtest_config = backtest_config.clone();

            let handle = thread::spawn(move || {
                for params in chunk {
                    if let Ok(result) = Self::run_single_backtest(
                        &strategy_factory,
                        &candles,
                        params,
                        &backtest_config,
                        optimization_target,
                    ) {
                        let mut results = results.lock().unwrap();
                        results.push(result);
                    }
                }
            });

            handles.push(handle);
        }

        // 모든 스레드 완료 대기
        for handle in handles {
            let _ = handle.join();
        }

        let mut results = Arc::try_unwrap(results)
            .unwrap()
            .into_inner()
            .unwrap();

        // 목표 함수 값으로 정렬
        results.sort_by(|a, b| {
            b.objective_value
                .partial_cmp(&a.objective_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(results)
    }

    /// 진행 상황 콜백과 함께 그리드 서치 실행
    pub fn grid_search_with_progress<S: Strategy + Clone + Send + 'static>(
        &self,
        candles: Vec<Candle>,
        strategy_factory: impl Fn() -> S + Send + Sync + Clone + 'static,
        progress_callback: ProgressCallback,
    ) -> Result<Vec<OptimizationResult>> {
        let param_combinations = self.generate_combinations()?;
        let total = param_combinations.len();

        let candles = Arc::new(candles);
        let results = Arc::new(Mutex::new(Vec::new()));
        let completed = Arc::new(Mutex::new(0));
        let backtest_config = self.backtest_config.clone();
        let optimization_target = self.optimization_target;

        let chunk_size = (total + self.max_parallel_jobs - 1) / self.max_parallel_jobs;

        let mut handles = Vec::new();

        for chunk in param_combinations.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let candles = Arc::clone(&candles);
            let results = Arc::clone(&results);
            let completed = Arc::clone(&completed);
            let strategy_factory = strategy_factory.clone();
            let backtest_config = backtest_config.clone();
            let progress_callback = &progress_callback;

            let handle = thread::spawn(move || {
                for _params in chunk {
                    if let Ok(result) = Self::run_single_backtest(
                        &strategy_factory,
                        &candles,
                        _params,
                        &backtest_config,
                        optimization_target,
                    ) {
                        let mut results = results.lock().unwrap();
                        results.push(result);
                    }

                    // 진행 상황 업데이트
                    let mut count = completed.lock().unwrap();
                    *count += 1;
                    progress_callback(*count, total);
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.join();
        }

        let mut results = Arc::try_unwrap(results)
            .unwrap()
            .into_inner()
            .unwrap();

        results.sort_by(|a, b| {
            b.objective_value
                .partial_cmp(&a.objective_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(results)
    }

    /// 최적의 파라미터 반환 (REQ-313)
    pub fn find_best(&self, results: &[OptimizationResult]) -> Option<ParameterSet> {
        results.first().map(|r| r.parameters.clone())
    }

    /// 결과에서 상위 N개 반환
    pub fn top_n(&self, results: &[OptimizationResult], n: usize) -> Vec<OptimizationResult> {
        results.iter().take(n).cloned().collect()
    }

    /// 파라미터 조합 생성
    fn generate_combinations(&self) -> Result<Vec<ParameterSet>> {
        if self.param_ranges.is_empty() {
            return Ok(Vec::new());
        }

        // 각 파라미터의 가능한 값 목록 생성
        let param_values: Vec<(&String, Vec<f64>)> = self
            .param_ranges
            .iter()
            .map(|(name, range)| (name, range.generate_values()))
            .collect();

        // 조합 수 확인 (너무 많으면 경고)
        let total_combinations: usize = param_values.iter().map(|(_, v)| v.len()).product();

        const MAX_COMBINATIONS: usize = 100_000;

        if total_combinations > MAX_COMBINATIONS {
            return Err(TradingError::InvalidParameter(format!(
                "Too many parameter combinations: {} (max: {}). \
                 Consider reducing parameter ranges or using a different optimization method.",
                total_combinations, MAX_COMBINATIONS
            )));
        }

        // 모든 조합 생성 (재귀적)
        let mut combinations = Vec::new();
        let mut current = HashMap::new();

        Self::generate_combinations_recursive(&param_values, 0, &mut current, &mut combinations);

        Ok(combinations)
    }

    fn generate_combinations_recursive(
        param_values: &[(&String, Vec<f64>)],
        depth: usize,
        current: &mut ParameterSet,
        results: &mut Vec<ParameterSet>,
    ) {
        if depth >= param_values.len() {
            results.push(current.clone());
            return;
        }

        let (name, values) = &param_values[depth];

        for value in values {
            current.insert(name.clone(), *value);
            Self::generate_combinations_recursive(param_values, depth + 1, current, results);
        }

        current.remove(name);
    }

    /// 단일 백테스트 실행
    fn run_single_backtest<S: Strategy>(
        strategy_factory: &impl Fn() -> S,
        candles: &[Candle],
        parameters: ParameterSet,
        config: &BacktestConfig,
        target: OptimizationTarget,
    ) -> Result<OptimizationResult> {
        let mut strategy = strategy_factory();
        strategy.set_parameters(parameters.clone())?;

        let simulator = BacktestSimulator::new(config.clone());
        let candles = candles.to_vec();

        let backtest_result = simulator.run(candles, &mut strategy)?;
        let metrics = backtest_result.calculate_metrics();
        let objective_value = target.extract_value(&metrics);

        Ok(OptimizationResult::new(parameters, metrics, objective_value))
    }

    /// 총 조합 수 반환
    pub fn total_combinations(&self) -> usize {
        self.param_ranges
            .values()
            .map(|range| range.count())
            .product()
    }

    /// 파라미터 범위 추가
    pub fn add_parameter(&mut self, name: String, range: ParamRange) {
        self.param_ranges.insert(name, range);
    }

    /// 파라미터 범위 제거
    pub fn remove_parameter(&mut self, name: &str) -> Option<ParamRange> {
        self.param_ranges.remove(name)
    }

    /// 최적화 목표 변경
    pub fn with_optimization_target(mut self, target: OptimizationTarget) -> Self {
        self.optimization_target = target;
        self
    }
}

/// 간단한 랜덤 서치 최적화 도구
///
/// 그리드 서치가 너무 오래 걸릴 때 사용할 수 있는 대안입니다.
pub struct RandomSearchOptimizer {
    /// 기본 최적화 도구
    optimizer: ParameterOptimizer,
    /// 최대 반복 횟수
    max_iterations: usize,
}

impl RandomSearchOptimizer {
    /// 새로운 랜덤 서치 최적화 도구 생성
    pub fn new(
        param_ranges: HashMap<String, ParamRange>,
        optimization_target: OptimizationTarget,
        max_iterations: usize,
    ) -> Self {
        Self {
            optimizer: ParameterOptimizer::new(param_ranges, optimization_target),
            max_iterations,
        }
    }

    /// 랜덤 서치 실행
    pub fn optimize<S: Strategy + Clone + Send + 'static>(
        &self,
        candles: Vec<Candle>,
        strategy_factory: impl Fn() -> S + Send + Sync + Clone + 'static,
    ) -> Result<Vec<OptimizationResult>> {
        use rand::Rng;

        let mut results = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..self.max_iterations {
            let mut params = HashMap::new();

            for (name, range) in &self.optimizer.param_ranges {
                let values = range.generate_values();
                if !values.is_empty() {
                    let index = rng.gen_range(0..values.len());
                    params.insert(name.clone(), values[index]);
                }
            }

            if let Ok(result) = ParameterOptimizer::run_single_backtest(
                &strategy_factory,
                &candles,
                params,
                &self.optimizer.backtest_config,
                self.optimizer.optimization_target,
            ) {
                results.push(result);
            }
        }

        results.sort_by(|a, b| {
            b.objective_value
                .partial_cmp(&a.objective_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::Strategy;
    use chrono::{Timelike, Utc};
    use std::collections::HashMap;

    // 테스트용 전략
    #[derive(Clone)]
    struct TestStrategy {
        threshold: f64,
        period: usize,
    }

    impl TestStrategy {
        fn new() -> Self {
            Self {
                threshold: 0.02,
                period: 14,
            }
        }
    }

    impl Strategy for TestStrategy {
        fn on_candle(&mut self, _candle: &Candle) -> Option<crate::types::Signal> {
            None // 테스트에서는 신호 생성 안 함
        }

        fn get_name(&self) -> &str {
            "test_strategy"
        }

        fn get_parameters(&self) -> HashMap<String, f64> {
            let mut params = HashMap::new();
            params.insert("threshold".to_string(), self.threshold);
            params.insert("period".to_string(), self.period as f64);
            params
        }

        fn set_parameters(&mut self, params: HashMap<String, f64>) -> Result<(), Box<dyn std::error::Error>> {
            if let Some(&threshold) = params.get("threshold") {
                self.threshold = threshold;
            }
            if let Some(&period) = params.get("period") {
                self.period = period as usize;
            }
            Ok(())
        }

        fn reset(&mut self) {}
    }

    fn create_test_candles(count: usize) -> Vec<Candle> {
        let mut candles = Vec::new();
        let mut price = 50000.0;

        for i in 0..count {
            let change = 1.0 + (i as f64 * 0.001 - 0.005);
            let open = price;
            let close = price * change;
            let high = open.max(close) * 1.002;
            let low = open.min(close) * 0.998;

            candles.push(Candle::new(
                "KRW-BTC".to_string(),
                Utc::now().with_nanosecond(0).unwrap() + chrono::Duration::minutes(i as i64),
                open,
                high,
                low,
                close,
                100.0,
            ));

            price = close;
        }

        candles
    }

    #[test]
    fn test_param_range_integer() {
        let range = ParamRange::integer(5, 15, 2);
        let values = range.generate_values();

        assert_eq!(values, vec![5.0, 7.0, 9.0, 11.0, 13.0, 15.0]);
        assert_eq!(range.count(), 6);
    }

    #[test]
    fn test_param_range_float() {
        let range = ParamRange::float(0.1, 0.5, 0.1);
        let values = range.generate_values();

        assert_eq!(values.len(), 5);
        assert_eq!(values[0], 0.1);
        assert_eq!(values[4], 0.5);
    }

    #[test]
    fn test_param_range_contains() {
        let int_range = ParamRange::integer(5, 15, 2);
        assert!(int_range.contains(7.0));
        assert!(int_range.contains(15.0));
        assert!(!int_range.contains(8.0)); // 스텝에 맞지 않음
        assert!(!int_range.contains(7.5)); // 정수가 아님

        let float_range = ParamRange::float(0.1, 0.5, 0.1);
        assert!(float_range.contains(0.2));
        assert!(!float_range.contains(0.15));
    }

    #[test]
    fn test_parameter_optimizer_creation() {
        let mut param_ranges = HashMap::new();
        param_ranges.insert("threshold".to_string(), ParamRange::float(0.01, 0.1, 0.01));
        param_ranges.insert("period".to_string(), ParamRange::integer(10, 30, 5));

        let optimizer = ParameterOptimizer::new(param_ranges, OptimizationTarget::SharpeRatio);

        assert_eq!(optimizer.total_combinations(), 10 * 5); // 10 threshold values * 5 period values
    }

    #[test]
    fn test_parameter_optimizer_with_config() {
        let mut param_ranges = HashMap::new();
        param_ranges.insert("threshold".to_string(), ParamRange::float(0.01, 0.05, 0.01));

        let optimizer = ParameterOptimizer::new(param_ranges, OptimizationTarget::TotalReturn)
            .with_backtest_config(BacktestConfig::new(2_000_000.0))
            .with_max_parallel_jobs(4);

        assert_eq!(optimizer.backtest_config.initial_balance, 2_000_000.0);
        assert_eq!(optimizer.max_parallel_jobs, 4);
    }

    #[test]
    fn test_parameter_optimizer_add_remove() {
        let mut optimizer = ParameterOptimizer::new(
            HashMap::new(),
            OptimizationTarget::SharpeRatio,
        );

        assert_eq!(optimizer.total_combinations(), 0);

        optimizer.add_parameter("threshold".to_string(), ParamRange::integer(1, 10, 1));
        assert_eq!(optimizer.total_combinations(), 10);

        optimizer.add_parameter("period".to_string(), ParamRange::integer(5, 15, 5));
        assert_eq!(optimizer.total_combinations(), 10 * 3);

        let removed = optimizer.remove_parameter("threshold");
        assert!(removed.is_some());
        assert_eq!(optimizer.total_combinations(), 3);
    }

    #[test]
    fn test_generate_combinations() {
        let mut param_ranges = HashMap::new();
        param_ranges.insert("a".to_string(), ParamRange::integer(1, 3, 1)); // 1, 2, 3
        param_ranges.insert("b".to_string(), ParamRange::integer(10, 20, 10)); // 10, 20

        let optimizer = ParameterOptimizer::new(param_ranges, OptimizationTarget::SharpeRatio);
        let combinations = optimizer.generate_combinations().unwrap();

        assert_eq!(combinations.len(), 6); // 3 * 2

        // 조합 확인
        assert_eq!(combinations[0].get("a"), Some(&1.0));
        assert_eq!(combinations[0].get("b"), Some(&10.0));

        assert_eq!(combinations[5].get("a"), Some(&3.0));
        assert_eq!(combinations[5].get("b"), Some(&20.0));
    }

    #[test]
    fn test_too_many_combinations() {
        let mut param_ranges = HashMap::new();
        // 너무 많은 조합 생성
        param_ranges.insert("a".to_string(), ParamRange::integer(1, 500, 1));
        param_ranges.insert("b".to_string(), ParamRange::integer(1, 500, 1));

        let optimizer = ParameterOptimizer::new(param_ranges, OptimizationTarget::SharpeRatio);
        let result = optimizer.generate_combinations();

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TradingError::InvalidParameter(_)));
    }

    #[test]
    fn test_optimization_result_comparison() {
        let params1 = {
            let mut p = HashMap::new();
            p.insert("a".to_string(), 1.0);
            p
        };
        let params2 = {
            let mut p = HashMap::new();
            p.insert("a".to_string(), 2.0);
            p
        };

        let result1 = OptimizationResult::new(params1, PerformanceMetrics::empty(), 10.0);
        let result2 = OptimizationResult::new(params2, PerformanceMetrics::empty(), 20.0);

        assert!(result2.is_better_than(&result1));
        assert!(!result1.is_better_than(&result2));
    }

    #[test]
    fn test_find_best() {
        let mut results = Vec::new();

        for i in 1..=5 {
            let mut params = HashMap::new();
            params.insert("value".to_string(), i as f64);
            results.push(OptimizationResult::new(
                params,
                PerformanceMetrics::empty(),
                i as f64 * 10.0,
            ));
        }

        // 정렬 필요
        results.sort_by(|a, b| {
            b.objective_value
                .partial_cmp(&a.objective_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut param_ranges = HashMap::new();
        param_ranges.insert("value".to_string(), ParamRange::integer(1, 10, 1));
        let optimizer = ParameterOptimizer::new(param_ranges, OptimizationTarget::SharpeRatio);

        let best = optimizer.find_best(&results);
        assert!(best.is_some());
        assert_eq!(best.unwrap().get("value"), Some(&5.0));
    }

    #[test]
    fn test_top_n() {
        let mut results = Vec::new();

        for i in 1..=10 {
            let mut params = HashMap::new();
            params.insert("value".to_string(), i as f64);
            results.push(OptimizationResult::new(
                params,
                PerformanceMetrics::empty(),
                i as f64 * 10.0,
            ));
        }

        results.sort_by(|a, b| {
            b.objective_value
                .partial_cmp(&a.objective_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut param_ranges = HashMap::new();
        param_ranges.insert("value".to_string(), ParamRange::integer(1, 10, 1));
        let optimizer = ParameterOptimizer::new(param_ranges, OptimizationTarget::SharpeRatio);

        let top_3 = optimizer.top_n(&results, 3);
        assert_eq!(top_3.len(), 3);
        assert_eq!(top_3[0].parameters.get("value"), Some(&10.0));
        assert_eq!(top_3[2].parameters.get("value"), Some(&8.0));
    }

    #[test]
    fn test_grid_search_small() {
        let candles = create_test_candles(100);

        let mut param_ranges = HashMap::new();
        param_ranges.insert("threshold".to_string(), ParamRange::float(0.01, 0.02, 0.01));

        let optimizer = ParameterOptimizer::new(param_ranges, OptimizationTarget::SharpeRatio);

        let results = optimizer.grid_search(candles, || TestStrategy::new());

        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 2); // 0.01, 0.02
    }

    #[test]
    fn test_optimization_target_extract() {
        let metrics = PerformanceMetrics {
            total_return: 25.0,
            sharpe_ratio: 1.5,
            win_rate: 60.0,
            max_drawdown: 15.0,
            profit_factor: 1.8,
            ..Default::default()
        };

        assert_eq!(
            OptimizationTarget::TotalReturn.extract_value(&metrics),
            25.0
        );
        assert_eq!(
            OptimizationTarget::SharpeRatio.extract_value(&metrics),
            1.5
        );
        assert_eq!(
            OptimizationTarget::MinDrawdown.extract_value(&metrics),
            -15.0
        );
    }
}
