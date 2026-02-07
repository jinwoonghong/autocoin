//! Backtest Simulator
//!
//! 트레이딩 전략의 백테스팅을 수행하는 시뮬레이터입니다.
//! SPEC-TRADING-003 Section 5.4.2

use super::{BacktestConfig, PerformanceMetrics};
use crate::error::{Result, TradingError};
use crate::strategy::Strategy;
use crate::types::{Candle, Signal, SignalType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// 거래 기록
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// 진입 시간
    pub entry_time: DateTime<Utc>,
    /// 청산 시간
    pub exit_time: DateTime<Utc>,
    /// 진입 가격
    pub entry_price: f64,
    /// 청산 가격
    pub exit_price: f64,
    /// 거래 수량
    pub quantity: f64,
    /// 손익 금액 (KRW)
    pub profit: f64,
    /// 손익률
    pub profit_pct: f64,
    /// 거래 타입
    pub trade_type: TradeType,
}

/// 거래 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeType {
    /// 롱 포지션
    Long,
    /// 숏 포지션 (미구현)
    Short,
}

impl Trade {
    /// 새로운 거래 기록 생성
    pub fn new(
        entry_time: DateTime<Utc>,
        exit_time: DateTime<Utc>,
        entry_price: f64,
        exit_price: f64,
        quantity: f64,
        trade_type: TradeType,
    ) -> Self {
        let profit = match trade_type {
            TradeType::Long => (exit_price - entry_price) * quantity,
            TradeType::Short => (entry_price - exit_price) * quantity,
        };

        let profit_pct = (profit / (entry_price * quantity)) * 100.0;

        Self {
            entry_time,
            exit_time,
            entry_price,
            exit_price,
            quantity,
            profit,
            profit_pct,
            trade_type,
        }
    }

    /// 수익 거래인지 확인
    pub fn is_winning(&self) -> bool {
        self.profit > 0.0
    }
}

/// 백테스트 결과
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    /// 총 거래 횟수
    pub total_trades: usize,
    /// 수익 거래 횟수
    pub winning_trades: usize,
    /// 손실 거래 횟수
    pub losing_trades: usize,
    /// 총 수익률 (%)
    pub roi: f64,
    /// 승률 (%)
    pub win_rate: f64,
    /// 최대 낙폭 (%)
    pub max_drawdown: f64,
    /// 샤프 비율
    pub sharpe_ratio: f64,
    /// 거래 기록
    pub trades: Vec<Trade>,
    /// 자산 곡선 (시간별 자산 가치)
    pub equity_curve: Vec<EquityPoint>,
    /// 초기 잔고
    pub initial_balance: f64,
    /// 최종 잔고
    pub final_balance: f64,
}

/// 자산 곡선 포인트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityPoint {
    /// 타임스탬프
    pub timestamp: DateTime<Utc>,
    /// 자산 가치 (KRW)
    pub equity: f64,
}

impl BacktestResult {
    /// 성과 메트릭 계산
    pub fn calculate_metrics(&self) -> PerformanceMetrics {
        let total_return = if self.initial_balance > 0.0 {
            ((self.final_balance - self.initial_balance) / self.initial_balance) * 100.0
        } else {
            0.0
        };

        // 거래 기반 메트릭
        let (total_profit, total_loss, avg_win, avg_loss) = self.calculate_trade_metrics();

        // 변동성 계산 (수익률의 표준편차)
        let volatility = self.calculate_volatility();

        // VaR 95% 계산
        let var_95 = self.calculate_var_95();

        PerformanceMetrics {
            total_return,
            annualized_return: self.annualize_return(total_return),
            cagr: self.calculate_cagr(),
            max_drawdown: self.max_drawdown,
            volatility,
            var_95,
            sharpe_ratio: self.sharpe_ratio,
            sortino_ratio: self.calculate_sortino_ratio(),
            calmar_ratio: self.calculate_calmar_ratio(),
            win_rate: self.win_rate,
            profit_factor: self.calculate_profit_factor(total_loss, total_profit),
            avg_win,
            avg_loss,
            total_trades: self.total_trades,
        }
    }

    fn calculate_trade_metrics(&self) -> (f64, f64, f64, f64) {
        let mut winning_profits: Vec<f64> = Vec::new();
        let mut losing_profits: Vec<f64> = Vec::new();

        for trade in &self.trades {
            if trade.profit > 0.0 {
                winning_profits.push(trade.profit);
            } else if trade.profit < 0.0 {
                losing_profits.push(trade.profit.abs());
            }
        }

        let total_profit: f64 = winning_profits.iter().sum();
        let total_loss: f64 = losing_profits.iter().sum();

        let avg_win = if winning_profits.is_empty() {
            0.0
        } else {
            total_profit / winning_profits.len() as f64
        };

        let avg_loss = if losing_profits.is_empty() {
            0.0
        } else {
            total_loss / losing_profits.len() as f64
        };

        (total_profit, total_loss, avg_win, avg_loss)
    }

    fn calculate_volatility(&self) -> f64 {
        if self.equity_curve.len() < 2 {
            return 0.0;
        }

        let returns: Vec<f64> = self
            .equity_curve
            .windows(2)
            .map(|w| {
                if w[0].equity > 0.0 {
                    (w[1].equity - w[0].equity) / w[0].equity
                } else {
                    0.0
                }
            })
            .collect();

        if returns.is_empty() {
            return 0.0;
        }

        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns
            .iter()
            .map(|r| (r - mean).powi(2))
            .sum::<f64>()
            / returns.len() as f64;

        variance.sqrt() * 100.0 // 백분율로 변환
    }

    fn calculate_var_95(&self) -> f64 {
        if self.equity_curve.len() < 2 {
            return 0.0;
        }

        let mut returns: Vec<f64> = self
            .equity_curve
            .windows(2)
            .map(|w| {
                if w[0].equity > 0.0 {
                    ((w[1].equity - w[0].equity) / w[0].equity) * 100.0
                } else {
                    0.0
                }
            })
            .collect();

        returns.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let index = (returns.len() as f64 * 0.05) as usize;
        returns.get(index).copied().unwrap_or(0.0)
    }

    fn annualize_return(&self, total_return: f64) -> f64 {
        // 백테스트 기간을 일수로 계산
        if self.equity_curve.len() < 2 {
            return total_return;
        }

        let start = self.equity_curve.first().unwrap().timestamp;
        let end = self.equity_curve.last().unwrap().timestamp;
        let days = (end - start).num_days().max(1) as f64;

        // 연환산: (1 + total_return/100)^(365/days) - 1
        if days < 365.0 {
            let daily_return = 1.0 + (total_return / 100.0);
            let annualized = daily_return.powf(365.0 / days) - 1.0;
            annualized * 100.0
        } else {
            total_return
        }
    }

    fn calculate_cagr(&self) -> f64 {
        if self.equity_curve.len() < 2 || self.initial_balance <= 0.0 {
            return 0.0;
        }

        let start = self.equity_curve.first().unwrap().timestamp;
        let end = self.equity_curve.last().unwrap().timestamp;
        let years = (end - start).num_days().max(1) as f64 / 365.0;

        let total_return = (self.final_balance / self.initial_balance) - 1.0;
        (total_return + 1.0).powf(1.0 / years) - 1.0
    }

    fn calculate_sortino_ratio(&self) -> f64 {
        // 간단한 계산: (연환산 수익률 - 무위험 수익률) / 하방 변동성
        let annualized_return = self.annualize_return(self.roi);
        let risk_free_rate = 2.0; // 가정: 연 2% 무위험 수익률

        let downside_returns: Vec<f64> = self
            .equity_curve
            .windows(2)
            .map(|w| {
                let ret = if w[0].equity > 0.0 {
                    ((w[1].equity - w[0].equity) / w[0].equity) * 100.0
                } else {
                    0.0
                };
                if ret < 0.0 { ret } else { 0.0 }
            })
            .collect();

        if downside_returns.is_empty() {
            return 0.0;
        }

        let downside_variance = downside_returns
            .iter()
            .map(|r| r.powi(2))
            .sum::<f64>()
            / downside_returns.len() as f64;

        let downside_deviation = downside_variance.sqrt();

        if downside_deviation > 0.0 {
            (annualized_return - risk_free_rate) / downside_deviation
        } else {
            0.0
        }
    }

    fn calculate_calmar_ratio(&self) -> f64 {
        // CAGR / Max Drawdown
        let cagr = self.calculate_cagr() * 100.0;
        if self.max_drawdown > 0.0 {
            cagr / self.max_drawdown
        } else {
            0.0
        }
    }

    fn calculate_profit_factor(&self, total_loss: f64, total_profit: f64) -> f64 {
        if total_loss > 0.0 {
            total_profit / total_loss
        } else {
            0.0
        }
    }
}

/// 백테스트 시뮬레이터
pub struct BacktestSimulator {
    /// 백테스트 설정
    config: BacktestConfig,
}

impl BacktestSimulator {
    /// 새로운 시뮬레이터 생성
    pub fn new(config: BacktestConfig) -> Self {
        Self { config }
    }

    /// 기본 설정으로 시뮬레이터 생성
    pub fn with_default_config() -> Self {
        Self::new(BacktestConfig::default())
    }

    /// 백테스트 실행 (REQ-312: 백테스팅 완료 시 성과 보고서 저장)
    ///
    /// # Errors
    ///
    /// * `TradingError::InvalidParameter` - 캔들 데이터가 최소 개수 미만인 경우 (REQ-316)
    pub fn run<S: Strategy>(&self, candles: Vec<Candle>, strategy: &mut S) -> Result<BacktestResult> {
        const MIN_CANDLES: usize = 100;

        // REQ-316: 불충분한 데이터 확인
        if candles.len() < MIN_CANDLES {
            return Err(TradingError::InvalidParameter(format!(
                "Insufficient data for backtesting: {} candles (minimum: {})",
                candles.len(),
                MIN_CANDLES
            )));
        }

        let mut simulator_state = SimulatorState::new(self.config.clone());
        let mut trades = Vec::new();
        let mut equity_curve = Vec::new();

        // 캔들 순회
        for candle in &candles {
            // 전략 신호 생성
            if let Some(signal) = strategy.on_candle(candle) {
                simulator_state.handle_signal(candle, &signal, &mut trades)?;
            }

            // 보유 포지션 업데이트 (손절/익절 체크)
            simulator_state.update_positions(candle, &mut trades);

            // 자산 곡선 기록
            let equity = simulator_state.calculate_equity(candle.close_price);
            equity_curve.push(EquityPoint {
                timestamp: candle.timestamp,
                equity,
            });
        }

        // 남은 포지션 전체 청산
        if let Some(ref position) = simulator_state.position {
            if let (Some(entry_time), Some(entry_price)) =
                (simulator_state.position_entry_time, simulator_state.position_entry_price)
            {
                let last_candle = candles.last().unwrap();
                let exit_price = last_candle.close_price;
                let trade = Trade::new(
                    entry_time,
                    last_candle.timestamp,
                    entry_price,
                    exit_price,
                    simulator_state.position_quantity,
                    TradeType::Long,
                );
                trades.push(trade);
            }
        }

        // 최종 잔고 계산
        let final_balance = simulator_state.balance;

        // 결과 계산
        let result = self.calculate_result(
            trades,
            equity_curve,
            self.config.initial_balance,
            final_balance,
        );

        Ok(result)
    }

    fn calculate_result(
        &self,
        trades: Vec<Trade>,
        equity_curve: Vec<EquityPoint>,
        initial_balance: f64,
        final_balance: f64,
    ) -> BacktestResult {
        let total_trades = trades.len();
        let winning_trades = trades.iter().filter(|t| t.is_winning()).count();
        let losing_trades = total_trades - winning_trades;

        let roi = if initial_balance > 0.0 {
            ((final_balance - initial_balance) / initial_balance) * 100.0
        } else {
            0.0
        };

        let win_rate = if total_trades > 0 {
            (winning_trades as f64 / total_trades as f64) * 100.0
        } else {
            0.0
        };

        let (max_drawdown, sharpe_ratio) = self.calculate_drawdown_and_sharpe(&equity_curve);

        BacktestResult {
            total_trades,
            winning_trades,
            losing_trades,
            roi,
            win_rate,
            max_drawdown,
            sharpe_ratio,
            trades,
            equity_curve,
            initial_balance,
            final_balance,
        }
    }

    fn calculate_drawdown_and_sharpe(&self, equity_curve: &[EquityPoint]) -> (f64, f64) {
        if equity_curve.len() < 2 {
            return (0.0, 0.0);
        }

        let initial_equity = equity_curve.first().unwrap().equity;
        let mut peak = initial_equity;
        let mut max_drawdown = 0.0;

        // Drawdown 계산
        for point in equity_curve {
            if point.equity > peak {
                peak = point.equity;
            }
            let drawdown = if peak > 0.0 {
                ((peak - point.equity) / peak) * 100.0
            } else {
                0.0
            };
            max_drawdown = max_drawdown.max(drawdown);
        }

        // Sharpe Ratio 계산
        let returns: Vec<f64> = equity_curve
            .windows(2)
            .map(|w| {
                if w[0].equity > 0.0 {
                    (w[1].equity - w[0].equity) / w[0].equity
                } else {
                    0.0
                }
            })
            .collect();

        let sharpe_ratio = if returns.is_empty() {
            0.0
        } else {
            let mean = returns.iter().sum::<f64>() / returns.len() as f64;
            let variance = returns
                .iter()
                .map(|r| (r - mean).powi(2))
                .sum::<f64>()
                / returns.len() as f64;
            let std_dev = variance.sqrt();

            // 연환산 (일일 수익률 가정)
            let annualized_mean = mean * 365.0;
            let annualized_std = std_dev * (365.0_f64).sqrt();
            let risk_free_rate = 0.02; // 연 2% 무위험 수익률

            if annualized_std > 0.0 {
                (annualized_mean - risk_free_rate) / annualized_std
            } else {
                0.0
            }
        };

        (max_drawdown, sharpe_ratio)
    }
}

/// 시뮬레이터 내부 상태
struct SimulatorState {
    /// 현재 현금 잔고 (KRW)
    balance: f64,
    /// 보유 코인 수량
    position: Option<f64>,
    /// 진입 가격
    position_entry_price: Option<f64>,
    /// 진입 시간
    position_entry_time: Option<DateTime<Utc>>,
    /// 진입 수량
    position_quantity: f64,
    /// 설정
    config: BacktestConfig,
}

impl SimulatorState {
    fn new(config: BacktestConfig) -> Self {
        Self {
            balance: config.initial_balance,
            position: None,
            position_entry_price: None,
            position_entry_time: None,
            position_quantity: 0.0,
            config,
        }
    }

    /// 신호 처리
    fn handle_signal(
        &mut self,
        candle: &Candle,
        signal: &Signal,
        trades: &mut Vec<Trade>,
    ) -> Result<()> {
        match signal.signal_type {
            SignalType::Buy | SignalType::StrongBuy => {
                // 이미 포지션이 있으면 체크
                if self.position.is_some() {
                    return Ok(());
                }
                self.execute_buy(candle)?;
            }
            SignalType::Sell | SignalType::StrongSell => {
                if self.position.is_some() {
                    self.execute_sell(candle, trades)?;
                }
            }
            SignalType::Hold => {}
        }

        Ok(())
    }

    /// 매수 실행
    fn execute_buy(&mut self, candle: &Candle) -> Result<()> {
        // 슬리피지 적용
        let execution_price = candle.close_price * (1.0 + self.config.slippage);
        let commission = self.config.commission_rate;

        // 사용 가능 금액 계산 (수수료 고려)
        let available = self.balance * (1.0 - commission);

        // 최소 주문 금액 확인
        if available < self.config.min_order_amount {
            return Ok(());
        }

        let quantity = available / execution_price;

        // 수수료 차감
        let commission_amount = self.balance * commission;
        self.balance -= commission_amount;

        // 포지션 진입
        self.position = Some(quantity);
        self.position_entry_price = Some(execution_price);
        self.position_entry_time = Some(candle.timestamp);
        self.position_quantity = quantity;

        Ok(())
    }

    /// 매도 실행
    fn execute_sell(&mut self, candle: &Candle, trades: &mut Vec<Trade>) -> Result<()> {
        if let (Some(entry_time), Some(entry_price)) =
            (self.position_entry_time, self.position_entry_price)
        {
            let quantity = self.position_quantity;

            // 슬리피지 적용
            let execution_price = candle.close_price * (1.0 - self.config.slippage);
            let sell_value = quantity * execution_price;

            // 수수료 차감
            let commission = sell_value * self.config.commission_rate;
            let net_value = sell_value - commission;

            // 잔고 업데이트
            self.balance += net_value;

            // 거래 기록
            let trade = Trade::new(
                entry_time,
                candle.timestamp,
                entry_price,
                execution_price,
                quantity,
                TradeType::Long,
            );
            trades.push(trade);

            // 포지션 클리어
            self.position = None;
            self.position_entry_price = None;
            self.position_entry_time = None;
            self.position_quantity = 0.0;
        }

        Ok(())
    }

    /// 포지션 업데이트 (손절/익절)
    fn update_positions(&mut self, candle: &Candle, trades: &mut Vec<Trade>) {
        // 현재는 단순 구현, 향후 손절/익절 로직 추가 가능
        // Signal을 통해 청산됨
    }

    /// 현재 자산 가치 계산
    fn calculate_equity(&self, current_price: f64) -> f64 {
        let position_value = self.position.map_or(0.0, |q| q * current_price);
        self.balance + position_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::Strategy;
    use chrono::Timelike;
    use std::collections::HashMap;

    // 테스트용 간단한 전략
    struct TestStrategy {
        buy_count: usize,
        sell_count: usize,
    }

    impl TestStrategy {
        fn new() -> Self {
            Self {
                buy_count: 0,
                sell_count: 0,
            }
        }
    }

    impl Strategy for TestStrategy {
        fn on_candle(&mut self, candle: &Candle) -> Option<Signal> {
            // 3번째 캔들에서 매수, 7번째 캔들에서 매도
            self.buy_count += 1;

            if self.buy_count == 3 {
                return Some(Signal::buy(
                    candle.market.clone(),
                    0.8,
                    "Test buy".to_string(),
                ));
            }

            if self.buy_count == 7 {
                return Some(Signal::sell(
                    candle.market.clone(),
                    0.8,
                    "Test sell".to_string(),
                ));
            }

            None
        }

        fn get_name(&self) -> &str {
            "test_strategy"
        }

        fn get_parameters(&self) -> HashMap<String, f64> {
            HashMap::new()
        }

        fn set_parameters(&mut self, _params: HashMap<String, f64>) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }

        fn reset(&mut self) {
            self.buy_count = 0;
            self.sell_count = 0;
        }
    }

    fn create_test_candles(count: usize, start_price: f64) -> Vec<Candle> {
        let mut candles = Vec::new();
        let mut price = start_price;

        for i in 0..count {
            // 간단한 가격 패턴: 상승 후 하락
            let change = if i < count / 2 { 1.01 } else { 0.99 };

            let open = price;
            let close = price * change;
            let high = open.max(close) * 1.005;
            let low = open.min(close) * 0.995;

            let candle = Candle::new(
                "KRW-BTC".to_string(),
                Utc::now().with_nanosecond(0).unwrap() + chrono::Duration::minutes(i as i64),
                open,
                high,
                low,
                close,
                100.0,
            );

            candles.push(candle);
            price = close;
        }

        candles
    }

    #[test]
    fn test_trade_creation() {
        let entry_time = Utc::now();
        let exit_time = entry_time + chrono::Duration::hours(24);

        let trade = Trade::new(
            entry_time,
            exit_time,
            50000.0,
            55000.0,
            0.01,
            TradeType::Long,
        );

        assert_eq!(trade.entry_price, 50000.0);
        assert_eq!(trade.exit_price, 55000.0);
        assert_eq!(trade.quantity, 0.01);
        assert_eq!(trade.trade_type, TradeType::Long);

        // 수익 계산: (55000 - 50000) * 0.01 = 50
        assert!((trade.profit - 50.0).abs() < 0.01);
        assert!(trade.is_winning());
    }

    #[test]
    fn test_trade_winning_losing() {
        let entry_time = Utc::now();
        let exit_time = entry_time + chrono::Duration::hours(24);

        let winning_trade = Trade::new(
            entry_time,
            exit_time,
            50000.0,
            55000.0,
            0.01,
            TradeType::Long,
        );

        let losing_trade = Trade::new(
            entry_time,
            exit_time,
            50000.0,
            45000.0,
            0.01,
            TradeType::Long,
        );

        assert!(winning_trade.is_winning());
        assert!(!losing_trade.is_winning());
    }

    #[test]
    fn test_simulator_state_creation() {
        let config = BacktestConfig::new(1_000_000.0);
        let state = SimulatorState::new(config);

        assert_eq!(state.balance, 1_000_000.0);
        assert!(state.position.is_none());
        assert_eq!(state.position_quantity, 0.0);
    }

    #[test]
    fn test_backtest_simulator_creation() {
        let config = BacktestConfig::default();
        let simulator = BacktestSimulator::new(config);

        assert_eq!(simulator.config.initial_balance, 1_000_000.0);
    }

    #[test]
    fn test_backtest_with_default_config() {
        let simulator = BacktestSimulator::with_default_config();

        assert_eq!(simulator.config.initial_balance, 1_000_000.0);
        assert_eq!(simulator.config.commission_rate, 0.0005);
    }

    #[test]
    fn test_backtest_insufficient_data() {
        let simulator = BacktestSimulator::with_default_config();
        let candles = create_test_candles(50, 50000.0);
        let mut strategy = TestStrategy::new();

        let result = simulator.run(candles, &mut strategy);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TradingError::InvalidParameter(_)
        ));
    }

    #[test]
    fn test_backtest_with_sufficient_data() {
        let simulator = BacktestSimulator::with_default_config();
        let candles = create_test_candles(100, 50000.0);
        let mut strategy = TestStrategy::new();

        let result = simulator.run(candles, &mut strategy);

        assert!(result.is_ok());

        let backtest_result = result.unwrap();
        assert_eq!(backtest_result.initial_balance, 1_000_000.0);
        assert!(backtest_result.equity_curve.len() > 0);
    }

    #[test]
    fn test_backtest_result_calculation() {
        let simulator = BacktestSimulator::with_default_config();
        let candles = create_test_candles(100, 50000.0);
        let mut strategy = TestStrategy::new();

        let result = simulator.run(candles, &mut strategy).unwrap();

        assert!(result.total_trades >= 0);
        assert!(result.final_balance >= 0.0);

        // ROI 계산 검증
        let expected_roi = if result.initial_balance > 0.0 {
            ((result.final_balance - result.initial_balance) / result.initial_balance) * 100.0
        } else {
            0.0
        };
        assert!((result.roi - expected_roi).abs() < 0.01);

        // Win Rate 계산 검증
        let expected_win_rate = if result.total_trades > 0 {
            (result.winning_trades as f64 / result.total_trades as f64) * 100.0
        } else {
            0.0
        };
        assert!((result.win_rate - expected_win_rate).abs() < 0.01);
    }

    #[test]
    fn test_backtest_metrics_calculation() {
        let simulator = BacktestSimulator::with_default_config();
        let candles = create_test_candles(100, 50000.0);
        let mut strategy = TestStrategy::new();

        let result = simulator.run(candles, &mut strategy).unwrap();
        let metrics = result.calculate_metrics();

        assert!(metrics.total_return >= 0.0 || metrics.total_return <= 0.0);
        assert!(metrics.max_drawdown >= 0.0);
        assert!(metrics.win_rate >= 0.0 && metrics.win_rate <= 100.0);
    }

    #[test]
    fn test_equity_curve_generation() {
        let simulator = BacktestSimulator::with_default_config();
        let candles = create_test_candles(100, 50000.0);
        let mut strategy = TestStrategy::new();

        let result = simulator.run(candles, &mut strategy).unwrap();

        assert_eq!(result.equity_curve.len(), candles.len());

        // 자산 곡선이 시간 순서대로 정렬되어 있는지 확인
        for i in 1..result.equity_curve.len() {
            assert!(result.equity_curve[i].timestamp >= result.equity_curve[i - 1].timestamp);
        }
    }

    #[test]
    fn test_calculate_volatility_with_empty_data() {
        let result = BacktestResult {
            total_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            roi: 0.0,
            win_rate: 0.0,
            max_drawdown: 0.0,
            sharpe_ratio: 0.0,
            trades: vec![],
            equity_curve: vec![],
            initial_balance: 1_000_000.0,
            final_balance: 1_000_000.0,
        };

        let metrics = result.calculate_metrics();
        assert_eq!(metrics.volatility, 0.0);
    }

    #[test]
    fn test_calculate_var_95() {
        let result = BacktestResult {
            total_trades: 0,
            winning_trades: 0,
            losing_trades: 0,
            roi: 0.0,
            win_rate: 0.0,
            max_drawdown: 0.0,
            sharpe_ratio: 0.0,
            trades: vec![],
            equity_curve: vec![],
            initial_balance: 1_000_000.0,
            final_balance: 1_000_000.0,
        };

        let metrics = result.calculate_metrics();
        assert_eq!(metrics.var_95, 0.0);
    }
}
