//! Historical Data Cache
//!
//! 과거 시세 데이터를 SQLite에 캐싱합니다.

use super::{CandleUnit, DateRangeParams};
use crate::error::{Result, TradingError};
use crate::types::Candle;
use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
use std::str::FromStr;

/// Historical Data Cache
#[derive(Clone)]
pub struct HistoricalCache {
    pool: SqlitePool,
}

impl HistoricalCache {
    /// 새로운 캐시 생성
    pub async fn new(db_path: &str) -> Result<Self> {
        // 연결 옵션 설정
        let options = SqliteConnectOptions::from_str(db_path)?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);

        // 연결 풀 생성
        let pool = SqlitePool::connect_with(options).await?;

        // 테이블 생성
        Self::run_migrations(&pool).await?;

        Ok(HistoricalCache { pool })
    }

    /// 마이그레이션 실행
    async fn run_migrations(pool: &SqlitePool) -> Result<()> {
        // 과거 캔들 데이터 테이블
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS historical_candles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                market TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                unit INTEGER NOT NULL,
                open_price REAL NOT NULL,
                high_price REAL NOT NULL,
                low_price REAL NOT NULL,
                close_price REAL NOT NULL,
                volume REAL NOT NULL,
                fetched_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now') * 1000),
                UNIQUE(market, timestamp, unit)
            )
            "#,
        )
        .execute(pool)
        .await?;

        // 인덱스 생성
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_historical_candles_market_time
            ON historical_candles(market, unit, timestamp DESC)
            "#,
        )
        .execute(pool)
        .await?;

        tracing::info!("Historical cache migrations completed");
        Ok(())
    }

    /// 연결 풀 반환
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// 캔들 데이터 저장 (벌크)
    pub async fn save_candles(
        &self,
        market: &str,
        candles: &[Candle],
        unit: CandleUnit,
    ) -> Result<()> {
        if candles.is_empty() {
            return Ok(());
        }

        let fetched_at = Utc::now().timestamp_millis();

        let mut tx = self.pool.begin().await?;

        for candle in candles {
            let timestamp_ms = candle.timestamp.timestamp_millis();

            sqlx::query(
                r#"
                INSERT OR REPLACE INTO historical_candles
                (market, timestamp, unit, open_price, high_price, low_price, close_price, volume, fetched_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(market)
            .bind(timestamp_ms)
            .bind(unit.as_u32() as i64)
            .bind(candle.open_price)
            .bind(candle.high_price)
            .bind(candle.low_price)
            .bind(candle.close_price)
            .bind(candle.volume)
            .bind(fetched_at)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        tracing::debug!(
            "Saved {} historical candles for {} (unit: {})",
            candles.len(),
            market,
            unit.as_u32()
        );

        Ok(())
    }

    /// 캐시된 캔들 데이터 조회
    pub async fn get_candles(
        &self,
        market: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        unit: CandleUnit,
    ) -> Result<Vec<Candle>> {
        let from_ms = from.timestamp_millis();
        let to_ms = to.timestamp_millis();

        let rows = sqlx::query_as::<_, CandleRow>(
            r#"
            SELECT * FROM historical_candles
            WHERE market = ? AND unit = ? AND timestamp >= ? AND timestamp <= ?
            ORDER BY timestamp ASC
            "#,
        )
        .bind(market)
        .bind(unit.as_u32() as i64)
        .bind(from_ms)
        .bind(to_ms)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.to_candle(market)).collect())
    }

    /// 최근 N개의 캔들 조회
    pub async fn get_recent_candles(
        &self,
        market: &str,
        limit: usize,
        unit: CandleUnit,
    ) -> Result<Vec<Candle>> {
        let rows = sqlx::query_as::<_, CandleRow>(
            r#"
            SELECT * FROM historical_candles
            WHERE market = ? AND unit = ?
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(market)
        .bind(unit.as_u32() as i64)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut candles: Vec<Candle> = rows
            .into_iter()
            .map(|r| r.to_candle(market))
            .collect();

        // 시간 순서대로 정렬 (오름차순)
        candles.sort_by_key(|c| c.timestamp);

        Ok(candles)
    }

    /// 특정 시간 범위의 캔들 수 조회
    pub async fn count_candles(
        &self,
        market: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        unit: CandleUnit,
    ) -> Result<usize> {
        let from_ms = from.timestamp_millis();
        let to_ms = to.timestamp_millis();

        let row = sqlx::query_as::<_, (i64,)>(
            r#"
            SELECT COUNT(*) FROM historical_candles
            WHERE market = ? AND unit = ? AND timestamp >= ? AND timestamp <= ?
            "#,
        )
        .bind(market)
        .bind(unit.as_u32() as i64)
        .bind(from_ms)
        .bind(to_ms)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0 as usize)
    }

    /// 오래된 캐시 정리
    pub async fn cleanup_old_candles(&self, days: u32) -> Result<u64> {
        let cutoff_ms = Utc::now().timestamp_millis() - (days as i64 * 24 * 60 * 60 * 1000);

        let result = sqlx::query("DELETE FROM historical_candles WHERE fetched_at < ?")
            .bind(cutoff_ms)
            .execute(&self.pool)
            .await?;

        tracing::info!("Cleaned up {} old historical candles", result.rows_affected());
        Ok(result.rows_affected())
    }

    /// 특정 마켓의 모든 캐시 삭제
    pub async fn clear_market(&self, market: &str) -> Result<u64> {
        let result = sqlx::query("DELETE FROM historical_candles WHERE market = ?")
            .bind(market)
            .execute(&self.pool)
            .await?;

        tracing::info!("Cleared {} candles for market {}", result.rows_affected(), market);
        Ok(result.rows_affected())
    }

    /// 전체 캐시 삭제
    pub async fn clear_all(&self) -> Result<u64> {
        let result = sqlx::query("DELETE FROM historical_candles")
            .execute(&self.pool)
            .await?;

        tracing::info!("Cleared all historical candles");
        Ok(result.rows_affected())
    }
}

/// 캔들 데이터베이스 행
#[derive(Debug, sqlx::FromRow)]
struct CandleRow {
    timestamp: i64,
    open_price: f64,
    high_price: f64,
    low_price: f64,
    close_price: f64,
    volume: f64,
}

impl CandleRow {
    fn to_candle(&self, market: &str) -> Candle {
        Candle {
            market: market.to_string(),
            timestamp: DateTime::from_timestamp_millis(self.timestamp)
                .unwrap_or_else(|| Utc::now()),
            open_price: self.open_price,
            high_price: self.high_price,
            low_price: self.low_price,
            close_price: self.close_price,
            volume: self.volume,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_creation() {
        let cache = HistoricalCache::new(":memory:").await;
        assert!(cache.is_ok());
    }

    #[tokio::test]
    async fn test_save_and_get_candles() {
        let cache = HistoricalCache::new(":memory:").await.unwrap();

        let now = Utc::now();
        let candles = vec![
            Candle::new(
                "KRW-BTC".to_string(),
                now - chrono::Duration::minutes(2),
                50000000.0,
                51000000.0,
                49000000.0,
                50500000.0,
                1.0,
            ),
            Candle::new(
                "KRW-BTC".to_string(),
                now - chrono::Duration::minutes(1),
                50500000.0,
                51500000.0,
                49500000.0,
                51000000.0,
                1.5,
            ),
        ];

        cache
            .save_candles("KRW-BTC", &candles, CandleUnit::OneMinute)
            .await
            .unwrap();

        let from = now - chrono::Duration::minutes(3);
        let retrieved = cache
            .get_candles("KRW-BTC", from, now, CandleUnit::OneMinute)
            .await
            .unwrap();

        assert_eq!(retrieved.len(), 2);
        assert_eq!(retrieved[0].market, "KRW-BTC");
    }

    #[tokio::test]
    async fn test_count_candles() {
        let cache = HistoricalCache::new(":memory:").await.unwrap();

        let now = Utc::now();
        let candles = vec![Candle::new(
            "KRW-BTC".to_string(),
            now - chrono::Duration::minutes(1),
            50000000.0,
            51000000.0,
            49000000.0,
            50500000.0,
            1.0,
        )];

        cache
            .save_candles("KRW-BTC", &candles, CandleUnit::OneMinute)
            .await
            .unwrap();

        let from = now - chrono::Duration::minutes(3);
        let count = cache
            .count_candles("KRW-BTC", from, now, CandleUnit::OneMinute)
            .await
            .unwrap();

        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_clear_market() {
        let cache = HistoricalCache::new(":memory:").await.unwrap();

        let now = Utc::now();
        let candles = vec![Candle::new(
            "KRW-BTC".to_string(),
            now - chrono::Duration::minutes(1),
            50000000.0,
            51000000.0,
            49000000.0,
            50500000.0,
            1.0,
        )];

        cache
            .save_candles("KRW-BTC", &candles, CandleUnit::OneMinute)
            .await
            .unwrap();

        let deleted = cache.clear_market("KRW-BTC").await.unwrap();
        assert_eq!(deleted, 1);

        let remaining = cache
            .get_recent_candles("KRW-BTC", 10, CandleUnit::OneMinute)
            .await
            .unwrap();
        assert_eq!(remaining.len(), 0);
    }
}
