//! Strategy Module
//!
//! 트레이딩 전략을 위한 공통 트레이트 및 구현체들.

use crate::types::{Candle, Signal};
use std::collections::HashMap;

/// 트레이딩 전략 트레이트
///
/// 모든 트레이딩 전략이 구현해야 하는 공통 인터페이스입니다.
/// SPEC-TRADING-003 REQ-301: 파라미터 유효성 검증 포함
pub trait Strategy: Send + Sync {
    /// 캔들 데이터 수신 및 신호 분석
    ///
    /// 새로운 캔들 데이터를 수신하고, 분석 결과로 신호를 반환합니다.
    /// 신호가 없는 경우 None을 반환합니다.
    fn on_candle(&mut self, candle: &Candle) -> Option<Signal>;

    /// 전략 이름 반환
    fn get_name(&self) -> &str;

    /// 파라미터 조회
    ///
    /// 현재 전략의 파라미터를 맵으로 반환합니다.
    fn get_parameters(&self) -> HashMap<String, f64>;

    /// 파라미터 설정
    ///
    /// 파라미터를 유효성 검사 후 설정합니다.
    /// REQ-301: 시스템은 항상 지표 파라미터의 유효성을 검증해야 한다 (기간 > 0, 기준가 > 0)
    fn set_parameters(&mut self, params: HashMap<String, f64>) -> Result<(), Box<dyn std::error::Error>>;

    /// 전략 초기화
    ///
    /// 내부 상태를 초기화합니다 (히스토리 정리 등).
    fn reset(&mut self);
}

pub mod momentum;
pub mod strategy_manager;
pub mod multi_indicator;

// Re-exports
pub use strategy_manager::{StrategyManager, StrategyError};
pub use multi_indicator::{IndicatorSignal, IndicatorType, MultiIndicatorStrategy};

/// 전략 팩토리
///
/// 다양한 전략과 전략 관리자를 생성하는 팩토리입니다.
pub struct StrategyFactory;

impl StrategyFactory {
    /// 기본 모멘텀 전략 생성
    pub fn momentum() -> momentum::MomentumStrategy {
        momentum::MomentumStrategy::new(0.05, 2.0, 60)
    }

    /// 커스텀 파라미터로 모멘텀 전략 생성
    pub fn momentum_with_params(
        surge_threshold: f64,
        volume_multiplier: f64,
        timeframe_minutes: u64,
    ) -> momentum::MomentumStrategy {
        momentum::MomentumStrategy::new(surge_threshold, volume_multiplier, timeframe_minutes)
    }

    /// 보수적 전략 생성 (더 낮은 임계값, 더 긴 시간 프레임)
    pub fn conservative_momentum() -> momentum::MomentumStrategy {
        momentum::MomentumStrategy::new(0.03, 3.0, 120)
    }

    /// 공격적 전략 생성 (더 높은 임계값, 더 짧은 시간 프레임)
    pub fn aggressive_momentum() -> momentum::MomentumStrategy {
        momentum::MomentumStrategy::new(0.08, 1.5, 30)
    }

    /// 기본 다중 지표 전략 생성 (SPEC-TRADING-003 Section 5.3.1)
    pub fn multi_indicator() -> MultiIndicatorStrategy {
        MultiIndicatorStrategy::default_strategy()
    }

    /// 커스텀 임계값으로 다중 지표 전략 생성
    pub fn multi_indicator_with_threshold(threshold: f64) -> MultiIndicatorStrategy {
        MultiIndicatorStrategy::new(threshold)
    }

    /// 전략 관리자 생성 (기본 모멘텀 전략으로 초기화)
    pub fn manager() -> StrategyManager {
        StrategyManager::new(Box::new(Self::momentum()))
    }

    /// 여러 전략이 미리 로드된 전략 관리자 생성
    pub fn manager_with_presets() -> StrategyManager {
        let mut manager = StrategyManager::new(Box::new(Self::momentum()));

        // 보수적 전략 등록
        manager.register_strategy(
            "conservative".to_string(),
            Box::new(Self::conservative_momentum()),
        );

        // 공격적 전략 등록
        manager.register_strategy(
            "aggressive".to_string(),
            Box::new(Self::aggressive_momentum()),
        );

        manager
    }
}
