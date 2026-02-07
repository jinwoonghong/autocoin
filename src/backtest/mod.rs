//! Backtest Module
//!
//! 백테스팅 엔진과 성과 분석 도구를 제공합니다.
//! SPEC-TRADING-003 Section 5.4

pub mod metrics;
pub mod optimizer;
pub mod simulator;

pub use metrics::{OptimizationTarget, PerformanceMetrics};
pub use optimizer::{OptimizationResult, ParamRange, ParameterOptimizer};
pub use simulator::{BacktestResult, BacktestSimulator, Trade};

// Re-exports for convenience
pub use crate::types::Candle;

/// 백테스팅 설정
#[derive(Debug, Clone)]
pub struct BacktestConfig {
    /// 초기 잔고 (KRW)
    pub initial_balance: f64,
    /// 수수료율 (기본 0.0005 = 0.05%)
    pub commission_rate: f64,
    /// 슬리피지 (기본 0.0)
    pub slippage: f64,
    /// 최소 주문 금액
    pub min_order_amount: f64,
}

impl Default for BacktestConfig {
    fn default() -> Self {
        Self {
            initial_balance: 1_000_000.0, // 100만 KRW
            commission_rate: 0.0005,      // 0.05%
            slippage: 0.0,
            min_order_amount: 5000.0,     // 5,000 KRW
        }
    }
}

impl BacktestConfig {
    /// 새로운 설정 생성
    pub fn new(initial_balance: f64) -> Self {
        Self {
            initial_balance,
            ..Default::default()
        }
    }

    /// 수수료율 설정
    pub fn with_commission(mut self, rate: f64) -> Self {
        self.commission_rate = rate;
        self
    }

    /// 슬리피지 설정
    pub fn with_slippage(mut self, slippage: f64) -> Self {
        self.slippage = slippage;
        self
    }

    /// 최소 주문 금액 설정
    pub fn with_min_order(mut self, amount: f64) -> Self {
        self.min_order_amount = amount;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtest_config_default() {
        let config = BacktestConfig::default();
        assert_eq!(config.initial_balance, 1_000_000.0);
        assert_eq!(config.commission_rate, 0.0005);
        assert_eq!(config.slippage, 0.0);
        assert_eq!(config.min_order_amount, 5000.0);
    }

    #[test]
    fn test_backtest_config_builder() {
        let config = BacktestConfig::new(2_000_000.0)
            .with_commission(0.001)
            .with_slippage(0.001)
            .with_min_order(10000.0);

        assert_eq!(config.initial_balance, 2_000_000.0);
        assert_eq!(config.commission_rate, 0.001);
        assert_eq!(config.slippage, 0.001);
        assert_eq!(config.min_order_amount, 10000.0);
    }
}
