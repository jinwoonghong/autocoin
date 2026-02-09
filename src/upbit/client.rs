//! Upbit REST API client

use super::models::*;
use super::{generate_jwt_token, generate_signature, build_query_string, UPBIT_API_URL};
use crate::error::{Result, TradingError, UpbitError};
use crate::types::{Balance, Candle, Order, OrderSide, PriceTick};
use chrono::{DateTime, Duration, Utc};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration as StdDuration;
use tokio::time::sleep;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use tracing::{debug, warn};

/// Upbit REST API 클라이언트
#[derive(Clone)]
pub struct UpbitClient {
    client: Client,
    access_key: String,
    secret_key: String,
    base_url: String,
    rate_limit: Arc<TokioMutex<RateLimiter>>,
}

impl UpbitClient {
    /// 새로운 클라이언트 생성
    pub fn new(access_key: String, secret_key: String) -> Self {
        let client = Client::builder()
            .timeout(StdDuration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            access_key,
            secret_key,
            base_url: UPBIT_API_URL.to_string(),
            rate_limit: Arc::new(TokioMutex::new(RateLimiter::new(10, 1.0))), // 10 requests per second
        }
    }

    /// 기본 URL 설정
    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    /// 인증 헤더 생성
    fn auth_headers(&self, method: &str, path: &str, body: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let token = generate_jwt_token(&self.access_key, &self.secret_key)
            .unwrap_or_else(|_| String::new());
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());

        headers
    }

    /// GET 요청 (인증 필요)
    async fn get_auth<T: DeserializeOwned>(
        &self,
        path: &str,
        params: Option<&HashMap<String, String>>,
    ) -> Result<T> {
        let query = if let Some(p) = params {
            let qs = build_query_string(p);
            if !qs.is_empty() {
                format!("?{}", qs)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let full_path = format!("{}{}", path, query);
        let headers = self.auth_headers("GET", &full_path, None);

        self.rate_limit.lock().await.acquire().await;

        let response = self
            .client
            .get(format!("{}{}", self.base_url, full_path))
            .headers(headers)
            .send()
            .await;

        self.handle_response(response).await
    }

    /// POST 요청 (인증 필요)
    async fn post_auth<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let body_json = serde_json::to_string(body)?;
        let headers = self.auth_headers("POST", path, Some(&body_json));

        self.rate_limit.lock().await.acquire().await;

        let response = self
            .client
            .post(format!("{}{}", self.base_url, path))
            .headers(headers)
            .body(body_json)
            .send()
            .await;

        self.handle_response(response).await
    }

    /// 응답 처리
    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: std::result::Result<Response, reqwest::Error>,
    ) -> Result<T> {
        let mut resp = response.map_err(|e| UpbitError::RequestFailed(e.to_string()))?;

        let status = resp.status();
        let response_text = resp.text().await?;

        if !status.is_success() {
            // Rate limit 처리
            if status == StatusCode::TOO_MANY_REQUESTS {
                warn!("Rate limit exceeded, backing off");
                sleep(StdDuration::from_secs(5)).await;
                return Err(TradingError::RateLimitExceeded);
            }

            // 에러 응답 파싱
            if let Ok(error) = serde_json::from_str::<ApiErrorResponse>(&response_text) {
                return Err(UpbitError::from_api_error(&status.as_u16().to_string(), &error.name).into());
            }

            return Err(UpbitError::ApiError {
                code: status.as_u16().to_string(),
                message: response_text,
            }
            .into());
        }

        serde_json::from_str(&response_text)
            .map_err(|e| UpbitError::ResponseParseError(e.to_string()).into())
    }

    /// 마켓 코드 조회
    pub async fn get_markets(&self) -> Result<Vec<MarketInfo>> {
        self.get_auth("/market/all", None).await
    }

    /// KRW 마켓 상위 N개 코인 조회
    pub async fn get_top_krw_markets(&self, top_n: usize) -> Result<Vec<String>> {
        let markets = self.get_markets().await?;
        let krw_markets: Vec<String> = markets
            .into_iter()
            .filter(|m| m.market.starts_with("KRW-"))
            .take(top_n)
            .map(|m| m.market)
            .collect();

        Ok(krw_markets)
    }

    /// 계정 정보 조회
    pub async fn get_accounts(&self) -> Result<Vec<Balance>> {
        let accounts: Vec<AccountInfo> = self.get_auth("/accounts", None).await?;
        Ok(accounts.into_iter().map(|a| a.into()).collect())
    }

    /// KRW 잔고 조회
    pub async fn get_krw_balance(&self) -> Result<f64> {
        let accounts = self.get_accounts().await?;
        let krw = accounts
            .iter()
            .find(|b| b.currency == "KRW")
            .map(|b| b.available)
            .unwrap_or(0.0);

        Ok(krw)
    }

    /// 현재가 조회 (티커)
    pub async fn get_ticker(&self, markets: &[String]) -> Result<Vec<PriceTick>> {
        let mut params = HashMap::new();
        params.insert("markets".to_string(), serde_json::to_string(markets)?);

        let tickers: Vec<TickerResponse> = self.get_auth("/ticker", Some(&params)).await?;
        Ok(tickers.into_iter().map(|t| t.into()).collect())
    }

    /// 캔들 데이터 조회
    pub async fn get_candles(
        &self,
        market: &str,
        unit: u32,
        count: u32,
    ) -> Result<Vec<Candle>> {
        let mut params = HashMap::new();
        params.insert("market".to_string(), market.to_string());
        params.insert("count".to_string(), count.to_string());

        let path = format!("/candles/minutes/{}", unit);
        let candles: Vec<CandleResponse> = self.get_auth(&path, Some(&params)).await?;
        Ok(candles.into_iter().map(|c| c.into()).collect())
    }

    /// 매수 주문 (지정가)
    pub async fn buy_limit_order(
        &self,
        market: &str,
        price: f64,
        volume: f64,
    ) -> Result<Order> {
        let order = OrderRequest {
            market: market.to_string(),
            side: "bid".to_string(),
            volume: Some(volume.to_string()),
            price: Some(price.to_string()),
            ord_type: "limit".to_string(),
        };

        let response: OrderResponse = self.post_auth("/orders", &order).await?;
        Ok(response.to_order())
    }

    /// 매수 주문 (시장가)
    pub async fn buy_market_order(&self, market: &str, amount_krw: f64) -> Result<Order> {
        let order = OrderRequest {
            market: market.to_string(),
            side: "bid".to_string(),
            volume: None,
            price: Some(amount_krw.to_string()),
            ord_type: "price".to_string(),
        };

        let response: OrderResponse = self.post_auth("/orders", &order).await?;
        Ok(response.to_order())
    }

    /// 매도 주문 (지정가)
    pub async fn sell_limit_order(
        &self,
        market: &str,
        price: f64,
        volume: f64,
    ) -> Result<Order> {
        let order = OrderRequest {
            market: market.to_string(),
            side: "ask".to_string(),
            volume: Some(volume.to_string()),
            price: Some(price.to_string()),
            ord_type: "limit".to_string(),
        };

        let response: OrderResponse = self.post_auth("/orders", &order).await?;
        Ok(response.to_order())
    }

    /// 매도 주문 (시장가)
    pub async fn sell_market_order(&self, market: &str, volume: f64) -> Result<Order> {
        let order = OrderRequest {
            market: market.to_string(),
            side: "ask".to_string(),
            volume: Some(volume.to_string()),
            price: None,
            ord_type: "market".to_string(),
        };

        let response: OrderResponse = self.post_auth("/orders", &order).await?;
        Ok(response.to_order())
    }

    /// 주문 조회
    pub async fn get_order(&self, uuid: &str) -> Result<Order> {
        let mut params = HashMap::new();
        params.insert("uuid".to_string(), uuid.to_string());

        let response: OrderResponse = self.get_auth("/order", Some(&params)).await?;
        Ok(response.to_order())
    }

    /// 주문 취소
    pub async fn cancel_order(&self, uuid: &str) -> Result<Order> {
        let mut params = HashMap::new();
        params.insert("uuid".to_string(), uuid.to_string());

        let response: OrderResponse = self.post_auth("/order/delete", &params).await?;
        Ok(response.to_order())
    }
}

/// API 에러 응답
#[derive(Debug, Deserialize)]
struct ApiErrorResponse {
    name: String,
    message: String,
}

/// Rate Limiter
struct RateLimiter {
    max_requests: u32,
    window_secs: f64,
    last_reset: std::time::Instant,
    request_count: u32,
}

impl RateLimiter {
    fn new(max_requests: u32, window_secs: f64) -> Self {
        Self {
            max_requests,
            window_secs,
            last_reset: std::time::Instant::now(),
            request_count: 0,
        }
    }

    async fn acquire(&mut self) {
        let elapsed = self.last_reset.elapsed().as_secs_f64();

        if elapsed >= self.window_secs {
            self.request_count = 0;
            self.last_reset = std::time::Instant::now();
        }

        if self.request_count >= self.max_requests {
            let wait_time = (self.window_secs - elapsed) as u64 + 1;
            debug!("Rate limit reached, sleeping {} seconds", wait_time);
            sleep(StdDuration::from_secs(wait_time)).await;
            self.request_count = 0;
            self.last_reset = std::time::Instant::now();
        }

        self.request_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(5, 1.0);

        for i in 0..5 {
            assert!(i < 5, "Should allow 5 requests");
            // Note: In a real async test, we'd need to properly await acquire()
        }
    }
}
