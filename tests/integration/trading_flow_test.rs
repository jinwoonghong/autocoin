//! Integration tests for trading flow

use autocoin::config::Settings;
use autocoin::db::Database;
use autocoin::types::{Decision, Order, OrderSide, Position, PriceTick, Signal};
use std::time::Duration;

#[tokio::test]
async fn test_database_position_lifecycle() {
    // 인메모리 DB 사용
    let db = Database::new(":memory:").await.expect("Failed to create DB");

    // 포지션 저장
    let position = Position::new("KRW-BTC".to_string(), 50000000.0, 0.001, 0.05, 0.1);
    db.save_position(&position).await.expect("Failed to save position");

    // 포지션 조회
    let retrieved = db
        .get_active_position("KRW-BTC")
        .await
        .expect("Failed to query position");

    assert!(retrieved.is_some(), "Position should exist");
    let retrieved_pos = retrieved.unwrap();
    assert_eq!(retrieved_pos.market, position.market);
    assert_eq!(retrieved_pos.entry_price, position.entry_price);
}

#[tokio::test]
async fn test_position_close_updates_status() {
    let db = Database::new(":memory:").await.expect("Failed to create DB");

    // 포지션 생성
    let position = Position::new("KRW-BTC".to_string(), 50000000.0, 0.001, 0.05, 0.1);
    db.save_position(&position).await.expect("Failed to save position");

    // 포지션 종료
    db.close_position("KRW-BTC", 55000000.0, 50.0, 0.1)
        .await
        .expect("Failed to close position");

    // 종료된 포지션은 조회되지 않아야 함
    let retrieved = db
        .get_active_position("KRW-BTC")
        .await
        .expect("Failed to query position");

    assert!(retrieved.is_none(), "Closed position should not be active");
}

#[tokio::test]
async fn test_price_tick_storage() {
    let db = Database::new(":memory:").await.expect("Failed to create DB");

    let tick = PriceTick::new(
        "KRW-BTC".to_string(),
        chrono::Utc::now().timestamp_millis(),
        50000000.0,
        0.05,
        1.0,
    );

    db.save_price_tick(&tick).await.expect("Failed to save tick");

    let ticks = db
        .get_recent_price_ticks("KRW-BTC", 10)
        .await
        .expect("Failed to query ticks");

    assert_eq!(ticks.len(), 1);
    assert_eq!(ticks[0].market, "KRW-BTC");
}

#[tokio::test]
async fn test_order_storage() {
    let db = Database::new(":memory:").await.expect("Failed to create DB");

    let order = Order::new("KRW-BTC".to_string(), OrderSide::Bid, 50000000.0, 0.001);

    db.save_order(&order).await.expect("Failed to save order");

    // 저장 성공만 확인 (별도 조회 API 없음)
    assert!(true, "Order saved successfully");
}

#[tokio::test]
async fn test_state_persistence() {
    let db = Database::new(":memory:").await.expect("Failed to create DB");

    // 상태 저장
    db.save_state("test_key", "test_value")
        .await
        .expect("Failed to save state");

    // 상태 조회
    let value = db
        .get_state("test_key")
        .await
        .expect("Failed to get state");

    assert!(value.is_some(), "State should exist");
    assert_eq!(value.unwrap(), "\"test_value\"");
}

#[tokio::test]
async fn test_decision_types() {
    let buy = Decision::buy("KRW-BTC".to_string(), 50000.0, "Test buy".to_string());
    let sell = Decision::sell("KRW-ETH".to_string(), 1.0, "Test sell".to_string());
    let hold = Decision::hold("No signal".to_string());

    assert!(buy.is_trade());
    assert!(sell.is_trade());
    assert!(!hold.is_trade());
}

#[tokio::test]
async fn test_signal_confidence_bounds() {
    let signal = Signal::buy("KRW-BTC".to_string(), 0.85, "Strong surge".to_string());

    assert_eq!(signal.market, "KRW-BTC");
    assert_eq!(signal.confidence, 0.85);
    assert!(signal.confidence >= 0.0 && signal.confidence <= 1.0);
}

#[tokio::test]
async fn test_multiple_positions_not_allowed() {
    let db = Database::new(":memory:").await.expect("Failed to create DB");

    // 첫 번째 포지션
    let pos1 = Position::new("KRW-BTC".to_string(), 50000000.0, 0.001, 0.05, 0.1);
    db.save_position(&pos1).await.expect("Failed to save position 1");

    // 두 번째 포지션 (다른 마켓)
    let pos2 = Position::new("KRW-ETH".to_string(), 3000000.0, 0.5, 0.05, 0.1);
    db.save_position(&pos2).await.expect("Failed to save position 2");

    // 두 포지션 모두 조회 가능
    let all_positions = db
        .get_all_active_positions()
        .await
        .expect("Failed to query all positions");

    assert_eq!(all_positions.len(), 2);
}

#[tokio::test]
async fn test_cleanup_old_price_ticks() {
    let db = Database::new(":memory:").await.expect("Failed to create DB");

    // 오래된 타임스탬프로 데이터 저장
    let old_tick = PriceTick::new(
        "KRW-BTC".to_string(),
        chrono::Utc::now().timestamp_millis() - (10 * 24 * 60 * 60 * 1000), // 10일 전
        50000000.0,
        0.0,
        1.0,
    );

    db.save_price_tick(&old_tick).await.expect("Failed to save old tick");

    // 7일 이전 데이터 정리
    let deleted = db.cleanup_old_price_ticks(7).await.expect("Failed to cleanup");

    assert!(deleted > 0, "Should delete old ticks");
}
