//! Upbit API client
//!
//! REST API와 WebSocket을 통한 Upbit 거래소 통신을 담당합니다.

use crate::error::{Result, TradingError, UpbitError};
use crate::types::{Balance, Candle, Order, OrderSide, OrderStatus, PriceTick};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashMap;
use std::time::Duration;

pub use client::UpbitClient;
pub use models::*;
pub use websocket::{ReconnectingWebSocket, UpbitWebSocket};

mod client;
mod models;
mod websocket;

/// Upbit API 기본 URL
const UPBIT_API_URL: &str = "https://api.upbit.com/v1";

/// Upbit WebSocket URL
const UPBIT_WS_URL: &str = "wss://api.upbit.com/websocket/v1";

/// JWT 토큰 생성
pub fn generate_jwt_token(access_key: &str, secret_key: &str) -> Result<String> {
    use jsonwebtoken::{encode, EncodingKey, Header};

    let now = Utc::now().timestamp();
    let payload = serde_json::json!({
        "access_key": access_key,
        "nonce": uuid::Uuid::new_v4().to_string(),
        "timestamp": now,
    });

    let token = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .map_err(|e| UpbitError::JwtCreationFailed(e.to_string()))?;

    Ok(token)
}

/// 요청 서명 생성 (JWT 방식)
pub fn generate_signature(
    access_key: &str,
    secret_key: &str,
    method: &str,
    path: &str,
    body: Option<&str>,
) -> String {
    let now = Utc::now().timestamp_millis();
    let payload = if let Some(b) = body {
        format!("{}{}{}{}", method, path, b, now)
    } else {
        format!("{}{}{}", method, path, now)
    };

    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(payload.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    format!(
        "{}?signature={}&timestamp={}",
        path, signature, now
    )
}

/// 쿼리 파라미터 문자열 생성
fn build_query_string(params: &HashMap<String, String>) -> String {
    if params.is_empty() {
        String::new()
    } else {
        let sorted: Vec<_> = params.iter().collect();
        sorted
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_string_builder() {
        let mut params = HashMap::new();
        params.insert("market".to_string(), "KRW-BTC".to_string());
        params.insert("count".to_string(), "10".to_string());

        let query = build_query_string(&params);
        assert!(query.contains("market=KRW-BTC"));
        assert!(query.contains("count=10"));
    }
}
