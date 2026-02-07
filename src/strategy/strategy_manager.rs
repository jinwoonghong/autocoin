//! Strategy Manager
//!
//! 트레이딩 전략을 관리하고 전환하는 관리자입니다.
//! SPEC-TRADING-003 REQ-318: 현재 활성화된 전략이 변경되면 시스템은 새 전략의 파라미터를 로드해야 한다.

use crate::strategy::Strategy;
use crate::types::{Candle, Signal};
use std::collections::HashMap;
use std::fmt;

/// 전략 관리자
///
/// 여러 트레이딩 전략을 등록하고 관리하며,
/// 실행 중 전략 전환을 지원합니다.
///
/// # Examples
///
/// ```no_run
/// use autocoin::strategy::{Strategy, StrategyManager};
/// use autocoin::strategy::momentum::MomentumStrategy;
///
/// let mut manager = StrategyManager::new(
///     Box::new(MomentumStrategy::new(0.05, 2.0, 60))
/// );
///
/// // 전략 등록
/// manager.register_strategy(
///     "conservative".to_string(),
///     Box::new(MomentumStrategy::new(0.03, 3.0, 120))
/// );
///
/// // 전략 전환
/// manager.switch_strategy("conservative").unwrap();
/// ```
pub struct StrategyManager {
    /// 현재 활성화된 전략
    active_strategy: Box<dyn Strategy>,
    /// 사용 가능한 전략 맵 (이름 -> 전략)
    available_strategies: HashMap<String, Box<dyn Strategy>>,
    /// 현재 활성 전략 이름
    active_name: String,
}

impl StrategyManager {
    /// 새로운 전략 관리자 생성
    ///
    /// # Arguments
    ///
    /// * `initial_strategy` - 초기 활성화할 전략
    ///
    /// # Examples
    ///
    /// ```
    /// use autocoin::strategy::{Strategy, StrategyManager};
    /// use autocoin::strategy::momentum::MomentumStrategy;
    ///
    /// let manager = StrategyManager::new(
    ///     Box::new(MomentumStrategy::new(0.05, 2.0, 60))
    /// );
    /// ```
    pub fn new(initial_strategy: Box<dyn Strategy>) -> Self {
        let name = initial_strategy.get_name().to_string();
        Self {
            active_strategy: initial_strategy,
            available_strategies: HashMap::new(),
            active_name: name,
        }
    }

    /// 전략 전환
    ///
    /// 지정된 이름의 전략으로 활성 전략을 전환합니다.
    /// REQ-318: 현재 활성화된 전략이 변경되면 시스템은 새 전략의 파라미터를 로드해야 한다.
    ///
    /// # Arguments
    ///
    /// * `name` - 전환할 전략 이름
    ///
    /// # Returns
    ///
    /// 성공 시 `Ok(())`, 실패 시 `Err` 반환
    ///
    /// # Errors
    ///
    /// - 지정된 이름의 전략이 없는 경우
    ///
    /// # Examples
    ///
    /// ```
    /// use autocoin::strategy::{Strategy, StrategyManager};
    /// use autocoin::strategy::momentum::MomentumStrategy;
    ///
    /// let mut manager = StrategyManager::new(
    ///     Box::new(MomentumStrategy::new(0.05, 2.0, 60))
    /// );
    ///
    /// manager.register_strategy(
    ///     "conservative".to_string(),
    ///     Box::new(MomentumStrategy::new(0.03, 3.0, 120))
    /// );
    ///
    /// assert!(manager.switch_strategy("conservative").is_ok());
    /// assert!(manager.switch_strategy("unknown").is_err());
    /// ```
    pub fn switch_strategy(&mut self, name: &str) -> Result<(), StrategyError> {
        // 현재 전략과 동일한 경우 무시
        if self.active_name == name {
            return Ok(());
        }

        // 등록된 전략에서 검색
        if let Some(strategy) = self.available_strategies.remove(name) {
            // 기존 전략을 available_strategies에 다시 저장 (이름 변경)
            let old_name = self.active_name.clone();
            let old_strategy = std::mem::replace(&mut self.active_strategy, strategy);
            self.available_strategies.insert(old_name, old_strategy);

            // 새 전략 이름 설정
            self.active_name = name.to_string();

            // 전략 리셋 (REQ-318: 새 전략 파라미터 로드)
            self.active_strategy.reset();

            Ok(())
        } else {
            Err(StrategyError::NotFound(name.to_string()))
        }
    }

    /// 현재 활성 전략 반환
    ///
    /// # Examples
    ///
    /// ```
    /// use autocoin::strategy::{Strategy, StrategyManager};
    /// use autocoin::strategy::momentum::MomentumStrategy;
    ///
    /// let manager = StrategyManager::new(
    ///     Box::new(MomentumStrategy::new(0.05, 2.0, 60))
    /// );
    ///
    /// let active = manager.get_active_strategy();
    /// assert_eq!(active.get_name(), "Momentum");
    /// ```
    pub fn get_active_strategy(&self) -> &dyn Strategy {
        self.active_strategy.as_ref()
    }

    /// 현재 활성 전략의 가변 참조 반환
    ///
    /// 내부 상태를 변경해야 하는 경우 사용합니다.
    pub(crate) fn get_active_strategy_mut(&mut self) -> &mut dyn Strategy {
        self.active_strategy.as_mut()
    }

    /// 등록된 전략 이름 목록 반환
    ///
    /// # Examples
    ///
    /// ```
    /// use autocoin::strategy::{Strategy, StrategyManager};
    /// use autocoin::strategy::momentum::MomentumStrategy;
    ///
    /// let mut manager = StrategyManager::new(
    ///     Box::new(MomentumStrategy::new(0.05, 2.0, 60))
    /// );
    ///
    /// manager.register_strategy(
    ///     "conservative".to_string(),
    ///     Box::new(MomentumStrategy::new(0.03, 3.0, 120))
    /// );
    ///
    /// let strategies = manager.list_strategies();
    /// assert!(strategies.contains(&"conservative".to_string()));
    /// ```
    pub fn list_strategies(&self) -> Vec<String> {
        let mut names = self.available_strategies.keys().cloned().collect::<Vec<_>>();
        names.push(self.active_name.clone());
        names.sort();
        names.dedup();
        names
    }

    /// 전략 등록
    ///
    /// 새로운 전략을 등록합니다. 동일한 이름의 전략이 이미 있는 경우 교체합니다.
    ///
    /// # Arguments
    ///
    /// * `name` - 전략 이름
    /// * `strategy` - 등록할 전략
    ///
    /// # Examples
    ///
    /// ```
    /// use autocoin::strategy::{Strategy, StrategyManager};
    /// use autocoin::strategy::momentum::MomentumStrategy;
    ///
    /// let mut manager = StrategyManager::new(
    ///     Box::new(MomentumStrategy::new(0.05, 2.0, 60))
    /// );
    ///
    /// manager.register_strategy(
    ///     "conservative".to_string(),
    ///     Box::new(MomentumStrategy::new(0.03, 3.0, 120))
    /// );
    /// ```
    pub fn register_strategy(&mut self, name: String, strategy: Box<dyn Strategy>) {
        self.available_strategies.insert(name, strategy);
    }

    /// 전략 제거
    ///
    /// 등록된 전략을 제거합니다. 활성 전략은 제거할 수 없습니다.
    ///
    /// # Arguments
    ///
    /// * `name` - 제거할 전략 이름
    ///
    /// # Returns
    ///
    /// 제거된 전략을 반환, 활성 전략이거나 없는 경우 Err
    pub fn unregister_strategy(&mut self, name: &str) -> Result<Box<dyn Strategy>, StrategyError> {
        if name == self.active_name {
            return Err(StrategyError::ActiveStrategyCannotBeRemoved);
        }

        self.available_strategies
            .remove(name)
            .ok_or_else(|| StrategyError::NotFound(name.to_string()))
    }

    /// 현재 활성 전략 이름 반환
    pub fn get_active_name(&self) -> &str {
        &self.active_name
    }

    /// 캔들 데이터 처리 및 신호 생성
    ///
    /// 현재 활성화된 전략에 캔들 데이터를 전달하고 신호를 받습니다.
    ///
    /// # Arguments
    ///
    /// * `candle` - 캔들 데이터
    ///
    /// # Returns
    ///
    /// 신호가 있는 경우 `Some(Signal)`, 없는 경우 `None`
    pub fn on_candle(&mut self, candle: &Candle) -> Option<Signal> {
        self.active_strategy.on_candle(candle)
    }

    /// 활성 전략의 파라미터 조회
    pub fn get_parameters(&self) -> HashMap<String, f64> {
        self.active_strategy.get_parameters()
    }

    /// 활성 전략의 파라미터 설정
    ///
    /// REQ-301: 시스템은 항상 지표 파라미터의 유효성을 검증해야 한다
    pub fn set_parameters(&mut self, params: HashMap<String, f64>) -> Result<(), Box<dyn std::error::Error>> {
        self.active_strategy.set_parameters(params)
    }

    /// 활성 전략 초기화
    pub fn reset(&mut self) {
        self.active_strategy.reset();
    }

    /// 전략 존재 여부 확인
    pub fn has_strategy(&self, name: &str) -> bool {
        name == self.active_name || self.available_strategies.contains_key(name)
    }

    /// 등록된 전략 수 반환
    pub fn strategy_count(&self) -> usize {
        self.available_strategies.len() + 1 // +1 for active strategy
    }
}

/// 전략 관리자 관련 에러
#[derive(Debug, Clone)]
pub enum StrategyError {
    /// 전략을 찾을 수 없음
    NotFound(String),
    /// 활성 전략은 제거할 수 없음
    ActiveStrategyCannotBeRemoved,
}

impl fmt::Display for StrategyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(name) => write!(f, "Strategy '{}' not found", name),
            Self::ActiveStrategyCannotBeRemoved => {
                write!(f, "Active strategy cannot be removed")
            }
        }
    }
}

impl std::error::Error for StrategyError {}

/// Strategy 트레이트 구현 (위임 패턴)
///
/// StrategyManager 자체도 Strategy 트레이트를 구현하여
/// 다른 전략과 동일한 인터페이스로 사용할 수 있습니다.
impl Strategy for StrategyManager {
    fn on_candle(&mut self, candle: &Candle) -> Option<Signal> {
        self.active_strategy.on_candle(candle)
    }

    fn get_name(&self) -> &str {
        self.active_strategy.get_name()
    }

    fn get_parameters(&self) -> HashMap<String, f64> {
        self.active_strategy.get_parameters()
    }

    fn set_parameters(&mut self, params: HashMap<String, f64>) -> Result<(), Box<dyn std::error::Error>> {
        self.active_strategy.set_parameters(params)
    }

    fn reset(&mut self) {
        self.active_strategy.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::momentum::MomentumStrategy;
    use chrono::Utc;

    // 테스트용 더미 전략
    struct DummyStrategy {
        name: String,
        params: HashMap<String, f64>,
    }

    impl DummyStrategy {
        fn new(name: &str) -> Self {
            let mut params = HashMap::new();
            params.insert("param1".to_string(), 1.0);
            Self {
                name: name.to_string(),
                params,
            }
        }

        fn with_params(name: &str, param1: f64) -> Self {
            let mut params = HashMap::new();
            params.insert("param1".to_string(), param1);
            Self {
                name: name.to_string(),
                params,
            }
        }
    }

    impl Strategy for DummyStrategy {
        fn on_candle(&mut self, _candle: &Candle) -> Option<Signal> {
            None
        }

        fn get_name(&self) -> &str {
            &self.name
        }

        fn get_parameters(&self) -> HashMap<String, f64> {
            self.params.clone()
        }

        fn set_parameters(&mut self, params: HashMap<String, f64>) -> Result<(), Box<dyn std::error::Error>> {
            if let Some(&value) = params.get("param1") {
                if value < 0.0 {
                    return Err("param1 must be non-negative".into());
                }
                self.params.insert("param1".to_string(), value);
            }
            Ok(())
        }

        fn reset(&mut self) {
            self.params.insert("reset_count".to_string(), 1.0);
        }
    }

    #[test]
    fn test_strategy_manager_creation() {
        let initial = Box::new(DummyStrategy::new("initial"));
        let manager = StrategyManager::new(initial);

        assert_eq!(manager.get_active_name(), "initial");
        assert_eq!(manager.strategy_count(), 1);
    }

    #[test]
    fn test_strategy_registration() {
        let mut manager = StrategyManager::new(Box::new(DummyStrategy::new("initial")));

        manager.register_strategy("strategy1".to_string(), Box::new(DummyStrategy::new("s1")));
        manager.register_strategy("strategy2".to_string(), Box::new(DummyStrategy::new("s2")));

        assert_eq!(manager.strategy_count(), 3);
        assert!(manager.has_strategy("strategy1"));
        assert!(manager.has_strategy("strategy2"));
    }

    #[test]
    fn test_list_strategies() {
        let mut manager = StrategyManager::new(Box::new(DummyStrategy::new("initial")));

        manager.register_strategy("conservative".to_string(), Box::new(DummyStrategy::new("c")));
        manager.register_strategy("aggressive".to_string(), Box::new(DummyStrategy::new("a")));

        let strategies = manager.list_strategies();
        assert_eq!(strategies.len(), 3);
        assert!(strategies.contains(&"initial".to_string()));
        assert!(strategies.contains(&"conservative".to_string()));
        assert!(strategies.contains(&"aggressive".to_string()));
    }

    #[test]
    fn test_switch_strategy() {
        // REQ-318: 현재 활성화된 전략이 변경되면 시스템은 새 전략의 파라미터를 로드해야 한다
        let mut manager = StrategyManager::new(Box::new(DummyStrategy::new("initial")));

        manager.register_strategy("conservative".to_string(), Box::new(DummyStrategy::new("conservative")));

        let result = manager.switch_strategy("conservative");
        assert!(result.is_ok());
        assert_eq!(manager.get_active_name(), "conservative");
    }

    #[test]
    fn test_switch_strategy_not_found() {
        let mut manager = StrategyManager::new(Box::new(DummyStrategy::new("initial")));

        let result = manager.switch_strategy("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StrategyError::NotFound(_)));
    }

    #[test]
    fn test_switch_strategy_same_as_current() {
        let mut manager = StrategyManager::new(Box::new(DummyStrategy::new("initial")));

        // 현재와 동일한 전략으로 전환 시도
        let result = manager.switch_strategy("initial");
        assert!(result.is_ok());
        assert_eq!(manager.get_active_name(), "initial");
    }

    #[test]
    fn test_switch_strategy_preserves_old_strategy() {
        let mut manager = StrategyManager::new(Box::new(DummyStrategy::with_params("initial", 5.0)));

        manager.register_strategy("conservative".to_string(), Box::new(DummyStrategy::with_params("conservative", 1.0)));

        // 전환
        manager.switch_strategy("conservative").unwrap();

        // 다시 원래대로 전환
        manager.switch_strategy("initial").unwrap();

        let params = manager.get_parameters();
        assert!((params.get("param1").unwrap() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_get_active_strategy() {
        let manager = StrategyManager::new(Box::new(DummyStrategy::new("test")));

        let active = manager.get_active_strategy();
        assert_eq!(active.get_name(), "test");
    }

    #[test]
    fn test_unregister_strategy() {
        let mut manager = StrategyManager::new(Box::new(DummyStrategy::new("initial")));

        manager.register_strategy("removable".to_string(), Box::new(DummyStrategy::new("r")));

        let result = manager.unregister_strategy("removable");
        assert!(result.is_ok());
        assert!(!manager.has_strategy("removable"));
    }

    #[test]
    fn test_unregister_active_strategy_fails() {
        let mut manager = StrategyManager::new(Box::new(DummyStrategy::new("initial")));

        let result = manager.unregister_strategy("initial");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            StrategyError::ActiveStrategyCannotBeRemoved
        ));
    }

    #[test]
    fn test_unregister_nonexistent_strategy() {
        let mut manager = StrategyManager::new(Box::new(DummyStrategy::new("initial")));

        let result = manager.unregister_strategy("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_on_candle_delegates_to_active_strategy() {
        let mut manager = StrategyManager::new(Box::new(MomentumStrategy::new(0.05, 2.0, 60)));

        let candle = Candle::new(
            "KRW-BTC".to_string(),
            Utc::now(),
            50000.0,
            50500.0,
            49500.0,
            50000.0,
            1.0,
        );

        // DummyStrategy는 항상 None 반환
        let signal = manager.on_candle(&candle);
        // MomentumStrategy는 충분한 데이터가 없으면 None 반환
        assert!(signal.is_none());
    }

    #[test]
    fn test_get_parameters_delegates_to_active() {
        let mut manager = StrategyManager::new(Box::new(MomentumStrategy::new(0.05, 2.0, 60)));

        let params = manager.get_parameters();
        assert_eq!(params.len(), 3);
        assert!((params.get("surge_threshold").unwrap() - 0.05).abs() < 0.001);
    }

    #[test]
    fn test_set_parameters_delegates_to_active() {
        let mut manager = StrategyManager::new(Box::new(MomentumStrategy::new(0.05, 2.0, 60)));

        let mut params = HashMap::new();
        params.insert("surge_threshold".to_string(), 0.10);

        let result = manager.set_parameters(params);
        assert!(result.is_ok());

        let updated = manager.get_parameters();
        assert!((updated.get("surge_threshold").unwrap() - 0.10).abs() < 0.001);
    }

    #[test]
    fn test_set_parameters_invalid_delegates_to_active() {
        // REQ-301: 파라미터 유효성 검증
        let mut manager = StrategyManager::new(Box::new(MomentumStrategy::new(0.05, 2.0, 60)));

        let mut params = HashMap::new();
        params.insert("surge_threshold".to_string(), -0.05); // Invalid

        let result = manager.set_parameters(params);
        assert!(result.is_err());
    }

    #[test]
    fn test_reset_delegates_to_active() {
        let mut manager = StrategyManager::new(Box::new(MomentumStrategy::new(0.05, 2.0, 60)));

        // reset 호출은 전략의 내부 상태를 초기화
        manager.reset();
        // MomentumStrategy의 경우 히스토리가 정리됨
        // 명시적인 검증은 어렵지만 panic이 발생하지 않는지 확인
    }

    #[test]
    fn test_strategy_trait_implementation() {
        let manager = StrategyManager::new(Box::new(DummyStrategy::new("test")));

        // Strategy 트레이트 메서드 호출 가능 확인
        assert_eq!(manager.get_name(), "test");
        assert!(!manager.get_parameters().is_empty());
    }

    #[test]
    fn test_strategy_error_display() {
        let err = StrategyError::NotFound("test".to_string());
        assert_eq!(format!("{}", err), "Strategy 'test' not found");

        let err = StrategyError::ActiveStrategyCannotBeRemoved;
        assert_eq!(format!("{}", err), "Active strategy cannot be removed");
    }

    #[test]
    fn test_switch_with_momentum_strategies() {
        let mut manager = StrategyManager::new(Box::new(MomentumStrategy::new(0.05, 2.0, 60)));

        manager.register_strategy(
            "conservative".to_string(),
            Box::new(MomentumStrategy::new(0.03, 3.0, 120))
        );

        // 전환 전 파라미터 확인
        let params_before = manager.get_parameters();
        assert!((params_before.get("surge_threshold").unwrap() - 0.05).abs() < 0.001);

        // 전환
        assert!(manager.switch_strategy("conservative").is_ok());

        // 전환 후 파라미터 확인 (REQ-318)
        let params_after = manager.get_parameters();
        assert!((params_after.get("surge_threshold").unwrap() - 0.03).abs() < 0.001);
    }

    #[test]
    fn test_multiple_switches() {
        let mut manager = StrategyManager::new(Box::new(DummyStrategy::new("s1")));

        manager.register_strategy("s2".to_string(), Box::new(DummyStrategy::new("s2")));
        manager.register_strategy("s3".to_string(), Box::new(DummyStrategy::new("s3")));

        assert!(manager.switch_strategy("s2").is_ok());
        assert_eq!(manager.get_active_name(), "s2");

        assert!(manager.switch_strategy("s3").is_ok());
        assert_eq!(manager.get_active_name(), "s3");

        assert!(manager.switch_strategy("s1").is_ok());
        assert_eq!(manager.get_active_name(), "s1");
    }
}
