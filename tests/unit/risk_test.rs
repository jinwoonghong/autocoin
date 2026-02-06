//! Unit tests for risk management

use autocoin::config::TradingConfig;
use autocoin::types::Position;

#[tokio::test]
async fn test_stop_loss_trigger() {
    let position = Position::new(
        "KRW-BTC".to_string(),
        100000.0, // 진입가
        0.001,    // 수량
        0.05,     // 손절률 5%
        0.10,     // 익절률 10%
    );

    // 손절가는 95,000 KRW
    assert_eq!(position.stop_loss, 95000.0);

    // 손절가 도달 확인
    assert!(position.should_stop_loss(94000.0), "Should trigger stop loss at 94,000");
    assert!(position.should_stop_loss(95000.0), "Should trigger stop loss at 95,000");
    assert!(!position.should_stop_loss(96000.0), "Should NOT trigger stop loss at 96,000");
}

#[tokio::test]
async fn test_take_profit_trigger() {
    let position = Position::new(
        "KRW-BTC".to_string(),
        100000.0,
        0.001,
        0.05,
        0.10,
    );

    // 익절가는 110,000 KRW
    assert_eq!(position.take_profit, 110000.0);

    // 익절가 도달 확인
    assert!(position.should_take_profit(110000.0), "Should trigger take profit at 110,000");
    assert!(position.should_take_profit(111000.0), "Should trigger take profit at 111,000");
    assert!(!position.should_take_profit(109000.0), "Should NOT trigger take profit at 109,000");
}

#[tokio::test]
async fn test_pnl_calculation() {
    let position = Position::new(
        "KRW-BTC".to_string(),
        100000.0,
        0.001,
        0.05,
        0.10,
    );

    // 10% 수익 시나리오
    let pnl = position.calculate_pnl(110000.0);

    assert_eq!(pnl.cost, 100.0, "Cost should be 100 KRW");
    assert_eq!(pnl.value, 110.0, "Value should be 110 KRW");
    assert!((pnl.profit - 10.0).abs() < 0.01, "Profit should be 10 KRW");
    assert!((pnl.profit_rate - 0.10).abs() < 0.01, "Profit rate should be 10%");
}

#[tokio::test]
async fn test_pnl_loss_calculation() {
    let position = Position::new(
        "KRW-BTC".to_string(),
        100000.0,
        0.001,
        0.05,
        0.10,
    );

    // 5% 손실 시나리오
    let pnl = position.calculate_pnl(95000.0);

    assert_eq!(pnl.cost, 100.0);
    assert_eq!(pnl.value, 95.0);
    assert!((pnl.profit - (-5.0)).abs() < 0.01, "Loss should be -5 KRW");
    assert!((pnl.profit_rate - (-0.05)).abs() < 0.01, "Loss rate should be -5%");
}

#[tokio::test]
async fn test_position_creation() {
    let position = Position::new(
        "KRW-ETH".to_string(),
        3000000.0,
        0.5,
        0.03,
        0.07,
    );

    assert_eq!(position.market, "KRW-ETH");
    assert_eq!(position.entry_price, 3000000.0);
    assert_eq!(position.amount, 0.5);
    assert_eq!(position.stop_loss, 2910000.0); // 3% below
    assert_eq!(position.take_profit, 3210000.0); // 7% above
}

#[test]
fn test_trading_config_defaults() {
    let config = TradingConfig::default();

    assert_eq!(config.target_coins, 20);
    assert_eq!(config.profit_rate, 0.10);
    assert_eq!(config.stop_loss_rate, 0.05);
    assert_eq!(config.surge_threshold, 0.05);
    assert_eq!(config.min_order_amount, 5000.0);
}
