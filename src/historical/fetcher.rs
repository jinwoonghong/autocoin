//! Historical Data Fetcher
//!
//! Upbit API에서 과거 시세 데이터를 조회합니다.

use super::{CandleUnit, DateRangeParams, FetchParams};
use crate::error::{Result, TradingError, UpbitError};
use crate::types::Candle;
use chrono::{DateTime, Duration, Utc};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::time::Duration as StdDuration;
use tokio::time::sleep;
use tracing::{debug, warn};

/// Upbit 캔들 API 응답
#[derive(Debug, Deserialize)]
struct CandleResponse {
    market: String,
    candle_date_time_utc: String,
    candle_date_time_kst: String,
    opening_price: f64,
    high_price: f64,
    low_price: f64,
    trade_price: f64,
    timestamp: i64,
    candle_acc_trade_price: f64,
    candle_acc_trade_volume: f64,
    unit: u32,
}

impl From<CandleResponse> for Candle {
    fn from(resp: CandleResponse) -> Self {
        let timestamp = DateTime::parse_from_rfc3339(&resp.candle_date_time_utc)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Candle {
            market: resp.market,
            timestamp,
            open_price: resp.opening_price,
            high_price: resp.high_price,
            low_price: resp.low_price,
            close_price: resp.trade_price,
            volume: resp.candle_acc_trade_volume,
        }
    }
}

/// Rate Limiter (Token Bucket Algorithm)
struct RateLimiter {
    max_requests: u32,
    window_secs: u64,
    last_reset: std::time::Instant,
    request_count: u32,
}

impl RateLimiter {
    fn new(max_requests: u32, window_secs: u64) -> Self {
        Self {
            max_requests,
            window_secs,
            last_reset: std::time::Instant::now(),
            request_count: 0,
        }
    }

    async fn acquire(&mut self) {
        let elapsed = self.last_reset.elapsed().as_secs();

        if elapsed >= self.window_secs {
            self.request_count = 0;
            self.last_reset = std::time::Instant::now();
        }

        if self.request_count >= self.max_requests {
            let wait_time = self.window_secs - elapsed + 1;
            debug!("Rate limit reached, sleeping {} seconds", wait_time);
            sleep(StdDuration::from_secs(wait_time)).await;
            self.request_count = 0;
            self.last_reset = std::time::Instant::now();
        }

        self.request_count += 1;
    }
}

/// Historical Data Fetcher
pub struct HistoricalDataFetcher {
    client: Client,
    base_url: String,
    rate_limit: RateLimiter,
    max_candles_per_request: usize,
}

impl HistoricalDataFetcher {
    const MAX_CANDLES_PER_REQUEST: usize = 200;
    const DEFAULT_BASE_URL: &str = "https://api.upbit.com/v1";

    /// 새로운 fetcher 생성
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(StdDuration::from_secs(30))
            .build()
            .map_err(|e| UpbitError::RequestFailed(e.to_string()))?;

        Ok(Self {
            client,
            base_url: Self::DEFAULT_BASE_URL.to_string(),
            rate_limit: RateLimiter::new(10, 1), // 10 requests per second
            max_candles_per_request: Self::MAX_CANDLES_PER_REQUEST,
        })
    }

    /// 기본 URL 설정
    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    /// 캔들 데이터 조회 (청크 단위)
    pub async fn fetch_candles(
        &mut self,
        market: &str,
        count: usize,
        to: Option<DateTime<Utc>>,
        unit: CandleUnit,
    ) -> Result<Vec<Candle>> {
        let mut all_candles = Vec::with_capacity(count);
        let mut remaining = count;
        let mut current_to = to.unwrap_or_else(Utc::now);

        while remaining > 0 {
            let chunk_size = remaining.min(self.max_candles_per_request);

            debug!(
                "Fetching {} candles for {} (ending at {:?})",
                chunk_size, market, current_to
            );

            let chunk = self
                .fetch_candles_chunk(market, chunk_size, current_to, unit)
                .await?;

            if chunk.is_empty() {
                warn!("No more candles available for {}", market);
                break;
            }

            // 가장 오래된 캔들의 시간을 다음 요청의 to로 설정
            if let Some(oldest) = chunk.last() {
                current_to = oldest.timestamp - Duration::minutes(unit.as_minutes() as i64);
            }

            all_candles.extend(chunk);
            remaining = count.saturating_sub(all_candles.len());

            // 더 이상 데이터가 없는 것 같으면 중단
            if all_candles.len() < chunk_size {
                debug!("Insufficient data available for {} (requested {}, got {})",
                       market, count, all_candles.len());
                break;
            }
        }

        // 시간 순서대로 정렬 (오름차순)
        all_candles.sort_by_key(|c| c.timestamp);

        Ok(all_candles)
    }

    /// 단일 청크 조회
    async fn fetch_candles_chunk(
        &mut self,
        market: &str,
        count: usize,
        to: DateTime<Utc>,
        unit: CandleUnit,
    ) -> Result<Vec<Candle>> {
        self.rate_limit.acquire().await;

        let url = format!("{}/candles/minutes/{}", self.base_url, unit.as_u32());

        let response = self
            .client
            .get(&url)
            .query(&[
                ("market", market),
                ("count", &count.to_string()),
                ("to", &to.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()),
            ])
            .send()
            .await;

        let mut resp = response
            .map_err(|e| UpbitError::RequestFailed(e.to_string()))?;

        let status = resp.status();
        let response_text = resp.text().await?;

        if !status.is_success() {
            if status == StatusCode::TOO_MANY_REQUESTS {
                warn!("Rate limit exceeded, backing off");
                sleep(StdDuration::from_secs(5)).await;
                return Err(TradingError::RateLimitExceeded);
            }

            return Err(UpbitError::ApiError {
                code: status.as_u16().to_string(),
                message: response_text,
            }
            .into());
        }

        let candles: Vec<CandleResponse> = serde_json::from_str(&response_text)
            .map_err(|e| UpbitError::ResponseParseError(e.to_string()))?;

        // Upbit API는 최신 순으로 반환하므로 뒤집어야 함
        let mut candles: Vec<Candle> = candles.into_iter().map(|c| c.into()).collect();
        candles.reverse();

        Ok(candles)
    }

    /// 날짜 범위로 조회
    pub async fn fetch_candles_by_date_range(
        &mut self,
        params: DateRangeParams,
        unit: CandleUnit,
    ) -> Result<Vec<Candle>> {
        let duration_ms = (params.to - params.from).num_milliseconds();
        let unit_ms = (unit.as_minutes() * 60 * 1000) as i64;

        if duration_ms <= 0 {
            return Err(TradingError::InvalidParameter(
                "Invalid date range: 'from' must be before 'to'".to_string(),
            ));
        }

        let estimated_candles = (duration_ms / unit_ms) as usize;
        let all_candles = self
            .fetch_candles(&params.market, estimated_candles, Some(params.to), unit)
            .await?;

        // 날짜 범위 필터링
        let filtered: Vec<Candle> = all_candles
            .into_iter()
            .filter(|c| c.timestamp >= params.from && c.timestamp <= params.to)
            .collect();

        Ok(filtered)
    }

    /// Rate limiter 직접 액세스 (테스트용)
    #[cfg(test)]
    pub async fn test_acquire_rate_limit(&mut self) {
        self.rate_limit.acquire().await;
    }
}

impl Default for HistoricalDataFetcher {
    fn default() -> Self {
        Self::new().expect("Failed to create HistoricalDataFetcher")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetcher_creation() {
        let fetcher = HistoricalDataFetcher::new();
        assert!(fetcher.is_ok());
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(5, 1);

        // 첫 5개 요청은 즉시 처리
        for _ in 0..5 {
            limiter.acquire().await;
        }

        // 6번째 요청은 대기 필요
        let start = std::time::Instant::now();
        limiter.acquire().await;
        let elapsed = start.elapsed();

        assert!(elapsed.as_secs() >= 1);
    }

    #[test]
    fn test_candle_response_conversion() {
        let resp = CandleResponse {
            market: "KRW-BTC".to_string(),
            candle_date_time_utc: "2024-01-01T00:00:00Z".to_string(),
            candle_date_time_kst: "2024-01-01T09:00:00Z".to_string(),
            opening_price: 50000000.0,
            high_price: 51000000.0,
            low_price: 49000000.0,
            trade_price: 50500000.0,
            timestamp: 1704067200000,
            candle_acc_trade_price: 1000000000.0,
            candle_acc_trade_volume: 20.0,
            unit: 1,
        };

        let candle: Candle = resp.into();
        assert_eq!(candle.market, "KRW-BTC");
        assert_eq!(candle.open_price, 50000000.0);
        assert_eq!(candle.high_price, 51000000.0);
        assert_eq!(candle.low_price, 49000000.0);
        assert_eq!(candle.close_price, 50500000.0);
        assert_eq!(candle.volume, 20.0);
    }
}
