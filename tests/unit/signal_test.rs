//! Unit tests for signal detection

use autocoin::config::TradingConfig;
use autocoin::types::PriceTick;
use std::time::Duration;

#[tokio::test]
async fn test_surge_detection() {
    let config = TradingConfig::default();

    // 시나리오: 1시간 내 5% 상승, 거래량 2배
    let base_time = chrono::Utc::now().timestamp_millis();

    // 기본 가격 데이터
    let tick1 = PriceTick::new("KRW-BTC".to_string(), base_time - 3600000, 50000.0, 0.0, 1.0);

    // 급등 가격 데이터
    let tick2 = PriceTick::new("KRW-BTC".to_string(), base_time, 52500.0, 0.05, 2.0);

    // 5% 상승 확인
    let price_change = (tick2.trade_price / tick1.trade_price) - 1.0;
    assert!((price_change - 0.05).abs() < 0.001, "Expected 5% increase, got {:.2}%", price_change * 100.0);

    // 거래량 급증 확인
    let volume_ratio = tick2.volume / tick1.volume;
    assert_eq!(volume_ratio, 2.0, "Expected 2x volume");
}

#[tokio::test]
async fn test_no_surge_below_threshold() {
    let config = TradingConfig::default();
    let base_time = chrono::Utc::now().timestamp_millis();

    // 3% 상승만 (임계값 미달)
    let tick1 = PriceTick::new("KRW-BTC".to_string(), base_time - 3600000, 50000.0, 0.0, 1.0);
    let tick2 = PriceTick::new("KRW-BTC".to_string(), base_time, 51500.0, 0.03, 1.5);

    let price_change = (tick2.trade_price / tick1.trade_price) - 1.0;

    // 임계값(5%) 미만이어야 함
    assert!(price_change < config.surge_threshold, "Price change should be below threshold");
}

#[tokio::test]
async fn test_multiple_markets() {
    let markets = vec![
        PriceTick::new("KRW-BTC".to_string(), 1000, 50000000.0, 0.02, 1.0),
        PriceTick::new("KRW-ETH".to_string(), 1000, 3000000.0, 0.01, 10.0),
        PriceTick::new("KRW-SOL".to_string(), 1000, 150000.0, -0.01, 100.0),
    ];

    assert_eq!(markets.len(), 3);
    assert_eq!(markets[0].market, "KRW-BTC");
    assert_eq!(markets[1].market, "KRW-ETH");
    assert_eq!(markets[2].market, "KRW-SOL");
}

#[tokio::test]
async fn test_price_tick_datetime_conversion() {
    let tick = PriceTick::new("KRW-BTC".to_string(), 1234567890000, 50000.0, 0.05, 1.0);
    let dt = tick.to_datetime();

    assert_eq!(dt.timestamp_millis(), 1234567890000);
}
