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

/// 전략 팩토리
pub struct StrategyFactory;

impl StrategyFactory {
    /// 기본 모멘텀 전략 생성
    pub fn momentum() -> momentum::MomentumStrategy {
        momentum::MomentumStrategy::new(0.05, 2.0, 60)
    }
}
