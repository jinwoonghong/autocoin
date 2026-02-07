//! Performance Metrics
//!
//! 백테스트 성과 메트릭을 계산하고 저장합니다.
//! SPEC-TRADING-003 Section 5.4.3

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 성과 메트릭
///
/// 백테스트 결과의 다양한 성과 지표를 포함합니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    // === 수익성 지표 ===
    /// 총 수익률 (%)
    pub total_return: f64,
    /// 연환산 수익률 (%)
    pub annualized_return: f64,
    /// 연평균 성장률 (CAGR)
    pub cagr: f64,

    // === 리스크 지표 ===
    /// 최대 낙폭 (%)
    pub max_drawdown: f64,
    /// 변동성 (%)
    pub volatility: f64,
    /// 95% VaR (Value at Risk, %)
    pub var_95: f64,

    // === 효율성 지표 ===
    /// 샤프 비율
    pub sharpe_ratio: f64,
    /// 소르티노 비율
    pub sortino_ratio: f64,
    /// 칼마 비율
    pub calmar_ratio: f64,

    // === 거래 분석 ===
    /// 승률 (%)
    pub win_rate: f64,
    /// 수익 요인 (Profit Factor: 총 수익 / 총 손실)
    pub profit_factor: f64,
    /// 평균 수익 (KRW)
    pub avg_win: f64,
    /// 평균 손실 (KRW)
    pub avg_loss: f64,
    /// 총 거래 횟수
    pub total_trades: usize,
}

impl PerformanceMetrics {
    /// 빈 메트릭 생성
    pub fn empty() -> Self {
        Self {
            total_return: 0.0,
            annualized_return: 0.0,
            cagr: 0.0,
            max_drawdown: 0.0,
            volatility: 0.0,
            var_95: 0.0,
            sharpe_ratio: 0.0,
            sortino_ratio: 0.0,
            calmar_ratio: 0.0,
            win_rate: 0.0,
            profit_factor: 0.0,
            avg_win: 0.0,
            avg_loss: 0.0,
            total_trades: 0,
        }
    }

    /// 샤프 비율 기준으로 성과 평가
    ///
    /// * < 1.0: 불량
    /// * 1.0 - 2.0: 양호
    /// * 2.0 - 3.0: 우수
    /// * \> 3.0: 탁월
    pub fn sharpe_rating(&self) -> SharpeRating {
        if self.sharpe_ratio < 1.0 {
            SharpeRating::Poor
        } else if self.sharpe_ratio < 2.0 {
            SharpeRating::Good
        } else if self.sharpe_ratio < 3.0 {
            SharpeRating::Excellent
        } else {
            SharpeRating::Outstanding
        }
    }

    /// 전체 성과 평가 점수 계산 (0 ~ 100)
    pub fn overall_score(&self) -> f64 {
        let mut score = 0.0;

        // 수익성 (30점)
        let return_score = (self.total_return.max(-100.0) / 100.0 * 30.0).min(30.0).max(0.0);
        score += return_score;

        // 샤프 비율 (25점)
        let sharpe_score = (self.sharpe_ratio.max(0.0) / 4.0 * 25.0).min(25.0);
        score += sharpe_score;

        // 승률 (20점)
        let win_rate_score = (self.win_rate / 100.0 * 20.0);
        score += win_rate_score;

        // Profit Factor (15점)
        let profit_factor_score = (self.profit_factor.min(3.0) / 3.0 * 15.0);
        score += profit_factor_score;

        // 최대 낙폭 (10점, 낮을수록 좋음)
        let drawdown_score = ((100.0 - self.max_drawdown.min(100.0)) / 100.0 * 10.0).max(0.0);
        score += drawdown_score;

        score
    }

    /// 메트릭 요약 문자열 반환
    pub fn summary(&self) -> String {
        format!(
            "Performance Summary:\n\
             - Total Return: {:.2}%\n\
             - Annualized Return: {:.2}%\n\
             - Max Drawdown: {:.2}%\n\
             - Sharpe Ratio: {:.2}\n\
             - Win Rate: {:.2}%\n\
             - Profit Factor: {:.2}\n\
             - Total Trades: {}\n\
             - Overall Score: {:.1}/100",
            self.total_return,
            self.annualized_return,
            self.max_drawdown,
            self.sharpe_ratio,
            self.win_rate,
            self.profit_factor,
            self.total_trades,
            self.overall_score()
        )
    }

    /// HashMap으로 변환 (저장용)
    pub fn to_map(&self) -> HashMap<String, f64> {
        let mut map = HashMap::new();
        map.insert("total_return".to_string(), self.total_return);
        map.insert("annualized_return".to_string(), self.annualized_return);
        map.insert("cagr".to_string(), self.cagr);
        map.insert("max_drawdown".to_string(), self.max_drawdown);
        map.insert("volatility".to_string(), self.volatility);
        map.insert("var_95".to_string(), self.var_95);
        map.insert("sharpe_ratio".to_string(), self.sharpe_ratio);
        map.insert("sortino_ratio".to_string(), self.sortino_ratio);
        map.insert("calmar_ratio".to_string(), self.calmar_ratio);
        map.insert("win_rate".to_string(), self.win_rate);
        map.insert("profit_factor".to_string(), self.profit_factor);
        map.insert("avg_win".to_string(), self.avg_win);
        map.insert("avg_loss".to_string(), self.avg_loss);
        map.insert("total_trades".to_string(), self.total_trades as f64);
        map
    }

    /// HashMap에서 복원
    pub fn from_map(map: &HashMap<String, f64>) -> Option<Self> {
        Some(Self {
            total_return: *map.get("total_return")?,
            annualized_return: *map.get("annualized_return")?,
            cagr: *map.get("cagr")?,
            max_drawdown: *map.get("max_drawdown")?,
            volatility: *map.get("volatility")?,
            var_95: *map.get("var_95")?,
            sharpe_ratio: *map.get("sharpe_ratio")?,
            sortino_ratio: *map.get("sortino_ratio")?,
            calmar_ratio: *map.get("calmar_ratio")?,
            win_rate: *map.get("win_rate")?,
            profit_factor: *map.get("profit_factor")?,
            avg_win: *map.get("avg_win")?,
            avg_loss: *map.get("avg_loss")?,
            total_trades: *map.get("total_trades")? as usize,
        })
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::empty()
    }
}

/// 샤프 비율 등급
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharpeRating {
    /// 불량 (< 1.0)
    Poor,
    /// 양호 (1.0 - 2.0)
    Good,
    /// 우수 (2.0 - 3.0)
    Excellent,
    /// 탁월 (> 3.0)
    Outstanding,
}

impl SharpeRating {
    /// 등급 이름 반환
    pub fn name(&self) -> &str {
        match self {
            SharpeRating::Poor => "Poor",
            SharpeRating::Good => "Good",
            SharpeRating::Excellent => "Excellent",
            SharpeRating::Outstanding => "Outstanding",
        }
    }

    /// 등급 설명 반환
    pub fn description(&self) -> &str {
        match self {
            SharpeRating::Poor => "Risk-adjusted returns are below market expectations",
            SharpeRating::Good => "Acceptable risk-adjusted performance",
            SharpeRating::Excellent => "Strong risk-adjusted performance",
            SharpeRating::Outstanding => "Exceptional risk-adjusted returns",
        }
    }
}

/// 최적화 목표
///
/// 파라미터 최적화 시 사용할 목표 함수입니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationTarget {
    /// 총 수익률 최대화
    TotalReturn,
    /// 샤프 비율 최대화
    SharpeRatio,
    /// 승률 최대화
    WinRate,
    /// 최대 낙폭 최소화
    MinDrawdown,
    /// 종합 점수 최대화
    OverallScore,
    /// Profit Factor 최대화
    ProfitFactor,
}

impl OptimizationTarget {
    /// 모든 최적화 목표 반환
    pub fn all() -> Vec<OptimizationTarget> {
        vec![
            OptimizationTarget::TotalReturn,
            OptimizationTarget::SharpeRatio,
            OptimizationTarget::WinRate,
            OptimizationTarget::MinDrawdown,
            OptimizationTarget::OverallScore,
            OptimizationTarget::ProfitFactor,
        ]
    }

    /// 목표 이름 반환
    pub fn name(&self) -> &str {
        match self {
            OptimizationTarget::TotalReturn => "Total Return",
            OptimizationTarget::SharpeRatio => "Sharpe Ratio",
            OptimizationTarget::WinRate => "Win Rate",
            OptimizationTarget::MinDrawdown => "Min Drawdown",
            OptimizationTarget::OverallScore => "Overall Score",
            OptimizationTarget::ProfitFactor => "Profit Factor",
        }
    }

    /// 메트릭에서 목표 값 추출
    pub fn extract_value(&self, metrics: &PerformanceMetrics) -> f64 {
        match self {
            OptimizationTarget::TotalReturn => metrics.total_return,
            OptimizationTarget::SharpeRatio => metrics.sharpe_ratio,
            OptimizationTarget::WinRate => metrics.win_rate,
            OptimizationTarget::MinDrawdown => -metrics.max_drawdown, // 최소화를 위해 음수
            OptimizationTarget::OverallScore => metrics.overall_score(),
            OptimizationTarget::ProfitFactor => metrics.profit_factor,
        }
    }
}

/// 성과 비교 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    /// 기준 전략 메트릭
    pub baseline: PerformanceMetrics,
    /// 비교 전략 메트릭
    pub comparison: PerformanceMetrics,
    /// 수익률 차이 (%)
    pub return_diff: f64,
    /// 샤프 비율 차이
    pub sharpe_diff: f64,
    /// 승률 차이 (%)
    pub win_rate_diff: f64,
    /// 최대 낙폭 차이 (%)
    pub drawdown_diff: f64,
}

impl PerformanceComparison {
    /// 두 메트릭 비교
    pub fn compare(baseline: PerformanceMetrics, comparison: PerformanceMetrics) -> Self {
        Self {
            return_diff: comparison.total_return - baseline.total_return,
            sharpe_diff: comparison.sharpe_ratio - baseline.sharpe_ratio,
            win_rate_diff: comparison.win_rate - baseline.win_rate,
            drawdown_diff: comparison.max_drawdown - baseline.max_drawdown,
            baseline,
            comparison,
        }
    }

    /// 비교 결과가 기준보다 나은지 확인
    pub fn is_better(&self) -> bool {
        // 단순한 판정: 수익률과 샤프 비율이 모두 나아야 함
        self.return_diff > 0.0 && self.sharpe_diff > 0.0
    }

    /// 비교 요약 문자열 반환
    pub fn summary(&self) -> String {
        format!(
            "Performance Comparison:\n\
             - Return Difference: {:+.2}%\n\
             - Sharpe Ratio Difference: {:+.2}\n\
             - Win Rate Difference: {:+.2}%\n\
             - Max Drawdown Difference: {:+.2}%\n\
             - Overall: {}",
            self.return_diff,
            self.sharpe_diff,
            self.win_rate_diff,
            self.drawdown_diff,
            if self.is_better() { "Better" } else { "Worse" }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_metrics() -> PerformanceMetrics {
        PerformanceMetrics {
            total_return: 25.5,
            annualized_return: 30.2,
            cagr: 0.25,
            max_drawdown: 15.3,
            volatility: 12.5,
            var_95: -3.2,
            sharpe_ratio: 1.8,
            sortino_ratio: 2.1,
            calmar_ratio: 1.6,
            win_rate: 55.0,
            profit_factor: 1.8,
            avg_win: 50000.0,
            avg_loss: 35000.0,
            total_trades: 100,
        }
    }

    #[test]
    fn test_performance_metrics_empty() {
        let metrics = PerformanceMetrics::empty();
        assert_eq!(metrics.total_return, 0.0);
        assert_eq!(metrics.total_trades, 0);
    }

    #[test]
    fn test_sharpe_rating_poor() {
        let metrics = PerformanceMetrics {
            sharpe_ratio: 0.5,
            ..Default::default()
        };
        assert_eq!(metrics.sharpe_rating(), SharpeRating::Poor);
    }

    #[test]
    fn test_sharpe_rating_good() {
        let metrics = PerformanceMetrics {
            sharpe_ratio: 1.5,
            ..Default::default()
        };
        assert_eq!(metrics.sharpe_rating(), SharpeRating::Good);
    }

    #[test]
    fn test_sharpe_rating_excellent() {
        let metrics = PerformanceMetrics {
            sharpe_ratio: 2.5,
            ..Default::default()
        };
        assert_eq!(metrics.sharpe_rating(), SharpeRating::Excellent);
    }

    #[test]
    fn test_sharpe_rating_outstanding() {
        let metrics = PerformanceMetrics {
            sharpe_ratio: 3.5,
            ..Default::default()
        };
        assert_eq!(metrics.sharpe_rating(), SharpeRating::Outstanding);
    }

    #[test]
    fn test_overall_score() {
        let metrics = create_test_metrics();
        let score = metrics.overall_score();

        assert!(score >= 0.0 && score <= 100.0);
        assert!(score > 50.0); // 좋은 메트릭이면 50점 이상
    }

    #[test]
    fn test_sharpe_rating_description() {
        assert_eq!(SharpeRating::Good.name(), "Good");
        assert!(!SharpeRating::Good.description().is_empty());
    }

    #[test]
    fn test_optimization_target_names() {
        assert_eq!(OptimizationTarget::TotalReturn.name(), "Total Return");
        assert_eq!(OptimizationTarget::SharpeRatio.name(), "Sharpe Ratio");
        assert_eq!(OptimizationTarget::WinRate.name(), "Win Rate");
    }

    #[test]
    fn test_optimization_target_extract_value() {
        let metrics = create_test_metrics();

        assert_eq!(
            OptimizationTarget::TotalReturn.extract_value(&metrics),
            25.5
        );
        assert_eq!(
            OptimizationTarget::SharpeRatio.extract_value(&metrics),
            1.8
        );
        assert_eq!(OptimizationTarget::WinRate.extract_value(&metrics), 55.0);
        // MinDrawdown은 음수로 반환
        assert_eq!(
            OptimizationTarget::MinDrawdown.extract_value(&metrics),
            -15.3
        );
    }

    #[test]
    fn test_optimization_target_all() {
        let targets = OptimizationTarget::all();
        assert_eq!(targets.len(), 6);
    }

    #[test]
    fn test_performance_metrics_to_map() {
        let metrics = create_test_metrics();
        let map = metrics.to_map();

        assert_eq!(map.len(), 15);
        assert_eq!(map.get("total_return"), Some(&25.5));
        assert_eq!(map.get("sharpe_ratio"), Some(&1.8));
    }

    #[test]
    fn test_performance_metrics_from_map() {
        let metrics = create_test_metrics();
        let map = metrics.to_map();
        let restored = PerformanceMetrics::from_map(&map);

        assert!(restored.is_some());
        let restored = restored.unwrap();
        assert_eq!(restored.total_return, metrics.total_return);
        assert_eq!(restored.sharpe_ratio, metrics.sharpe_ratio);
    }

    #[test]
    fn test_performance_metrics_from_map_incomplete() {
        let mut map = std::collections::HashMap::new();
        map.insert("total_return".to_string(), 25.5);

        let result = PerformanceMetrics::from_map(&map);
        assert!(result.is_none());
    }

    #[test]
    fn test_performance_comparison() {
        let baseline = PerformanceMetrics {
            total_return: 10.0,
            sharpe_ratio: 1.0,
            win_rate: 50.0,
            max_drawdown: 20.0,
            ..Default::default()
        };

        let comparison = PerformanceMetrics {
            total_return: 15.0,
            sharpe_ratio: 1.5,
            win_rate: 55.0,
            max_drawdown: 18.0,
            ..Default::default()
        };

        let result = PerformanceComparison::compare(baseline.clone(), comparison.clone());

        assert_eq!(result.return_diff, 5.0);
        assert_eq!(result.sharpe_diff, 0.5);
        assert_eq!(result.win_rate_diff, 5.0);
        assert_eq!(result.drawdown_diff, -2.0); // 최대 낙폭이 줄어듦
        assert!(result.is_better());
    }

    #[test]
    fn test_performance_comparison_worse() {
        let baseline = PerformanceMetrics {
            total_return: 15.0,
            sharpe_ratio: 1.5,
            win_rate: 55.0,
            max_drawdown: 18.0,
            ..Default::default()
        };

        let comparison = PerformanceMetrics {
            total_return: 10.0,
            sharpe_ratio: 1.0,
            win_rate: 50.0,
            max_drawdown: 20.0,
            ..Default::default()
        };

        let result = PerformanceComparison::compare(baseline, comparison);

        assert!(!result.is_better());
    }

    #[test]
    fn test_metrics_summary() {
        let metrics = create_test_metrics();
        let summary = metrics.summary();

        assert!(summary.contains("25.50%")); // Total Return
        assert!(summary.contains("1.80")); // Sharpe Ratio
        assert!(summary.contains("55.00%")); // Win Rate
    }

    #[test]
    fn test_comparison_summary() {
        let baseline = PerformanceMetrics {
            total_return: 10.0,
            sharpe_ratio: 1.0,
            win_rate: 50.0,
            max_drawdown: 20.0,
            ..Default::default()
        };

        let comparison = PerformanceMetrics {
            total_return: 15.0,
            sharpe_ratio: 1.5,
            win_rate: 55.0,
            max_drawdown: 18.0,
            ..Default::default()
        };

        let result = PerformanceComparison::compare(baseline, comparison);
        let summary = result.summary();

        assert!(summary.contains("+5.00%"));
        assert!(summary.contains("Better"));
    }
}
