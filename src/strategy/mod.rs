//! Trading strategies
//!
//! 다양한 트레이딩 전략을 구현합니다.

pub use momentum::MomentumStrategy;

mod momentum;

/// 전략 트레이트
pub trait Strategy {
    /// 신호 분석
    fn analyze(&self, data: &crate::types::PriceTick) -> Option<crate::types::Signal>;
}

/// 전략 팩토리
pub struct StrategyFactory;

impl StrategyFactory {
    /// 기본 모멘텀 전략 생성
    pub fn momentum() -> MomentumStrategy {
        MomentumStrategy::new(0.05, 2.0, 60)
    }
}
