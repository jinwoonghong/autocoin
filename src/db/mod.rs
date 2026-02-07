//! Database module for SQLite persistence
//!
//! 포지션, 거래 기록, 시세 데이터를 SQLite에 저장합니다.

use crate::error::{Result, TradingError};
use crate::types::{Candle, Order, Position, PriceTick};
use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
use std::path::Path;
use std::str::FromStr;

pub use models::*;
pub use schema::*;

mod models;
mod schema;

/// 데이터베이스 연결 풀
#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// 새로운 데이터베이스 연결 생성
    pub async fn new(db_path: &str) -> Result<Self> {
        // 데이터 디렉토리 생성
        if let Some(parent) = Path::new(db_path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // 연결 옵션 설정
        let options = SqliteConnectOptions::from_str(db_path)?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);

        // 연결 풀 생성
        let pool = SqlitePool::connect_with(options).await?;

        // 마이그레이션 실행
        Self::run_migrations(&pool).await?;

        Ok(Database { pool })
    }

    /// 연결 풀 반환
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// 마이그레이션 실행
    async fn run_migrations(pool: &SqlitePool) -> Result<()> {
        // 포지션 테이블
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS positions (
                id TEXT PRIMARY KEY,
                market TEXT NOT NULL,
                entry_price REAL NOT NULL,
                amount REAL NOT NULL,
                entry_time INTEGER NOT NULL,
                stop_loss REAL NOT NULL,
                take_profit REAL NOT NULL,
                exit_price REAL,
                exit_time INTEGER,
                pnl REAL,
                pnl_rate REAL,
                status TEXT NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
                updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000)
            )
            "#,
        )
        .execute(pool)
        .await?;

        // 주문 테이블
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS orders (
                id TEXT PRIMARY KEY,
                market TEXT NOT NULL,
                side TEXT NOT NULL,
                price REAL NOT NULL,
                volume REAL NOT NULL,
                status TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                executed_volume REAL DEFAULT 0,
                executed_amount REAL DEFAULT 0,
                error TEXT,
                created_timestamp INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000)
            )
            "#,
        )
        .execute(pool)
        .await?;

        // 시세 데이터 테이블
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS price_ticks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                market TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                trade_price REAL NOT NULL,
                change_rate REAL NOT NULL,
                volume REAL NOT NULL,
                trade_amount REAL NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
                UNIQUE(market, timestamp)
            )
            "#,
        )
        .execute(pool)
        .await?;

        // 시세 데이터 인덱스
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_price_ticks_market_time
            ON price_ticks(market, timestamp DESC)
            "#,
        )
        .execute(pool)
        .await?;

        // 캔들 데이터 테이블
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS candles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                market TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                open_price REAL NOT NULL,
                high_price REAL NOT NULL,
                low_price REAL NOT NULL,
                close_price REAL NOT NULL,
                volume REAL NOT NULL,
                unit INTEGER NOT NULL DEFAULT 1,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
                UNIQUE(market, timestamp, unit)
            )
            "#,
        )
        .execute(pool)
        .await?;

        // 캔들 인덱스
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_candles_market_time
            ON candles(market, timestamp DESC)
            "#,
        )
        .execute(pool)
        .await?;

        // 시스템 상태 테이블
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS system_state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000)
            )
            "#,
        )
        .execute(pool)
        .await?;

        // 지표 캐시 테이블 (REQ-304)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS indicator_cache (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                market TEXT NOT NULL,
                indicator_type TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                value REAL NOT NULL,
                metadata TEXT,
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
                UNIQUE(market, indicator_type, timestamp)
            )
            "#,
        )
        .execute(pool)
        .await?;

        // 지표 캐시 인덱스
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_indicator_cache_market_time
            ON indicator_cache(market, indicator_type, timestamp DESC)
            "#,
        )
        .execute(pool)
        .await?;

        tracing::info!("Database migrations completed");
        Ok(())
    }

    /// 활성 포지션 조회
    pub async fn get_active_position(&self, market: &str) -> Result<Option<Position>> {
        let row = sqlx::query_as::<_, PositionRow>(
            "SELECT * FROM positions WHERE market = ? AND status = 'active' LIMIT 1",
        )
        .bind(market)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.to_position()))
    }

    /// 모든 활성 포지션 조회
    pub async fn get_all_active_positions(&self) -> Result<Vec<Position>> {
        let rows = sqlx::query_as::<_, PositionRow>(
            "SELECT * FROM positions WHERE status = 'active'",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.to_position()).collect())
    }

    /// 포지션 저장
    pub async fn save_position(&self, position: &Position) -> Result<()> {
        let id = uuid::Uuid::new_v4().to_string();
        let timestamp_ms = position.entry_time.timestamp_millis();

        sqlx::query(
            r#"
            INSERT INTO positions (
                id, market, entry_price, amount, entry_time,
                stop_loss, take_profit, status
            ) VALUES (?, ?, ?, ?, ?, ?, ?, 'active')
            "#,
        )
        .bind(&id)
        .bind(&position.market)
        .bind(position.entry_price)
        .bind(position.amount)
        .bind(timestamp_ms)
        .bind(position.stop_loss)
        .bind(position.take_profit)
        .execute(&self.pool)
        .await?;

        tracing::info!(market = %position.market, "Position saved to database");
        Ok(())
    }

    /// 포지션 종료 (매도 후)
    pub async fn close_position(
        &self,
        market: &str,
        exit_price: f64,
        pnl: f64,
        pnl_rate: f64,
    ) -> Result<()> {
        let exit_time = Utc::now().timestamp_millis();

        sqlx::query(
            r#"
            UPDATE positions
            SET exit_price = ?,
                exit_time = ?,
                pnl = ?,
                pnl_rate = ?,
                status = 'closed',
                updated_at = ?
            WHERE market = ? AND status = 'active'
            "#,
        )
        .bind(exit_price)
        .bind(exit_time)
        .bind(pnl)
        .bind(pnl_rate)
        .bind(Utc::now().timestamp_millis())
        .bind(market)
        .execute(&self.pool)
        .await?;

        tracing::info!(market = %market, pnl_rate = %pnl_rate, "Position closed");
        Ok(())
    }

    /// 주문 저장
    pub async fn save_order(&self, order: &Order) -> Result<()> {
        let side_str = match order.side {
            crate::types::OrderSide::Bid => "bid",
            crate::types::OrderSide::Ask => "ask",
        };
        let status_str = match order.status {
            crate::types::OrderStatus::Waiting => "waiting",
            crate::types::OrderStatus::Executed => "executed",
            crate::types::OrderStatus::Canceled => "canceled",
            crate::types::OrderStatus::Failed => "failed",
        };
        let timestamp_ms = order.created_at.timestamp_millis();

        sqlx::query(
            r#"
            INSERT INTO orders (
                id, market, side, price, volume, status,
                executed_volume, executed_amount, created_timestamp
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                status = excluded.status,
                executed_volume = excluded.executed_volume,
                executed_amount = excluded.executed_amount
            "#,
        )
        .bind(&order.id)
        .bind(&order.market)
        .bind(side_str)
        .bind(order.price)
        .bind(order.volume)
        .bind(status_str)
        .bind(order.executed_volume)
        .bind(order.executed_amount)
        .bind(timestamp_ms)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 시세 데이터 저장
    pub async fn save_price_tick(&self, tick: &PriceTick) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO price_ticks
            (market, timestamp, trade_price, change_rate, volume, trade_amount)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&tick.market)
        .bind(tick.timestamp)
        .bind(tick.trade_price)
        .bind(tick.change_rate)
        .bind(tick.volume)
        .bind(tick.trade_amount)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 최근 시세 데이터 조회
    pub async fn get_recent_price_ticks(
        &self,
        market: &str,
        limit: usize,
    ) -> Result<Vec<PriceTick>> {
        let rows = sqlx::query_as::<_, PriceTickRow>(
            "SELECT * FROM price_ticks WHERE market = ? ORDER BY timestamp DESC LIMIT ?",
        )
        .bind(market)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.to_price_tick()).collect())
    }

    /// 캔들 데이터 저장
    pub async fn save_candle(&self, candle: &Candle) -> Result<()> {
        let timestamp_ms = candle.timestamp.timestamp_millis();

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO candles
            (market, timestamp, open_price, high_price, low_price, close_price, volume)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&candle.market)
        .bind(timestamp_ms)
        .bind(candle.open_price)
        .bind(candle.high_price)
        .bind(candle.low_price)
        .bind(candle.close_price)
        .bind(candle.volume)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 캔들 데이터 조회
    pub async fn get_candles(
        &self,
        market: &str,
        limit: usize,
    ) -> Result<Vec<Candle>> {
        let rows = sqlx::query_as::<_, CandleRow>(
            "SELECT * FROM candles WHERE market = ? ORDER BY timestamp DESC LIMIT ?",
        )
        .bind(market)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.to_candle()).collect())
    }

    /// 시스템 상태 저장
    pub async fn save_state(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO system_state (key, value, updated_at)
            VALUES (?, ?, ?)
            ON CONFLICT(key) DO UPDATE SET
                value = excluded.value,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(key)
        .bind(value)
        .bind(Utc::now().timestamp_millis())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 시스템 상태 조회
    pub async fn get_state(&self, key: &str) -> Result<Option<String>> {
        let row = sqlx::query_as::<_(sqlx::types::JsonValue)>(
            "SELECT value FROM system_state WHERE key = ?",
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|v| v.to_string()))
    }

    /// 오래된 시세 데이터 정리 (N일 이전)
    pub async fn cleanup_old_price_ticks(&self, days: u32) -> Result<u64> {
        let cutoff_ms = Utc::now()
            .timestamp_millis()
            - (days as i64 * 24 * 60 * 60 * 1000);

        let result = sqlx::query("DELETE FROM price_ticks WHERE timestamp < ?")
            .bind(cutoff_ms)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_in_memory() -> Result<()> {
        let db = Database::new(":memory:").await?;
        assert!(db.pool().is_closed() == false);

        // 포지션 저장/조회 테스트
        let position = Position::new("KRW-BTC".to_string(), 50000000.0, 0.001, 0.05, 0.1);
        db.save_position(&position).await?;

        let retrieved = db.get_active_position("KRW-BTC").await?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().market, "KRW-BTC");

        Ok(())
    }
}
