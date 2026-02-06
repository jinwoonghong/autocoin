//! Error types for the trading system
//!
//! 모든 에이전트와 모듈에서 발생하는 에러를 통합 관리합니다.

use thiserror::Error;

pub use types::TradingError;

mod types {
    use super::Error;
    use std::fmt;

    /// 메인 트레이딩 에러 타입
    ///
    /// 시스템 전체에서 발생할 수 있는 모든 에러를 통합합니다.
    #[derive(Debug, Error)]
    pub enum TradingError {
        /// Upbit API 관련 에러
        #[error("Upbit API error: {0}")]
        UpbitApi(#[from] UpbitError),

        /// 데이터베이스 에러
        #[error("Database error: {0}")]
        Database(String),

        /// 설정 에러
        #[error("Configuration error: {0}")]
        Config(String),

        /// Rate Limit 초과
        #[error("Rate limit exceeded")]
        RateLimitExceeded,

        /// 잔고 부족
        #[error("Insufficient balance: required {required}, available {available}")]
        InsufficientBalance { required: f64, available: f64 },

        /// 주문 실패
        #[error("Order failed: {0}")]
        OrderFailed(String),

        /// WebSocket 연결 에러
        #[error("WebSocket error: {0}")]
        WebSocket(String),

        /// Discord 알림 전송 실패
        #[error("Discord notification failed: {0}")]
        DiscordNotification(String),

        /// 인증 에러
        #[error("Authentication error: {0}")]
        Authentication(String),

        /// 파싱 에러
        #[error("Parse error: {0}")]
        Parse(String),

        /// 유효하지 않은 파라미터
        #[error("Invalid parameter: {0}")]
        InvalidParameter(String),

        /// 리스크 관리 에러
        #[error("Risk management error: {0}")]
        RiskManagement(String),

        /// IO 에러
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),
    }

    impl TradingError {
        /// 재시도 가능한 에러인지 확인
        pub fn is_retryable(&self) -> bool {
            matches!(
                self,
                TradingError::RateLimitExceeded
                    | TradingError::UpbitApi(_)
                    | TradingError::WebSocket(_)
            )
        }

        /// 에러에 따른 재시도 대기 시간 반환
        pub fn retry_delay(&self) -> std::time::Duration {
            match self {
                TradingError::RateLimitExceeded => std::time::Duration::from_secs(5),
                TradingError::UpbitApi(_) => std::time::Duration::from_secs(1),
                TradingError::WebSocket(_) => std::time::Duration::from_secs(2),
                _ => std::time::Duration::ZERO,
            }
        }
    }

    /// Upbit API 관련 에러
    #[derive(Debug, Error)]
    pub enum UpbitError {
        /// HTTP 요청 실패
        #[error("HTTP request failed: {0}")]
        RequestFailed(String),

        /// JWT 토큰 생성 실패
        #[error("JWT token creation failed: {0}")]
        JwtCreationFailed(String),

        /// API 응답 파싱 실패
        #[error("Failed to parse API response: {0}")]
        ResponseParseError(String),

        /// API에서 에러 응답 수신
        #[error("API returned error: {code} - {message}")]
        ApiError { code: String, message: String },

        /// Rate Limit 초과
        #[error("Rate limit exceeded")]
        RateLimitError,

        /// 잘못된 API 키
        #[error("Invalid API credentials")]
        InvalidCredentials,

        /// 타임아웃
        #[error("Request timeout")]
        Timeout,
    }

    impl UpbitError {
        /// API 에러 코드로부터 에러 생성
        pub fn from_api_error(code: &str, message: &str) -> Self {
            match code {
                "429" => UpbitError::RateLimitError,
                "401" | "403" => UpbitError::InvalidCredentials,
                _ => UpbitError::ApiError {
                    code: code.to_string(),
                    message: message.to_string(),
                },
            }
        }
    }
}

/// 에러 결과 타입 (Result 별칭)
pub type Result<T> = std::result::Result<T, TradingError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_retryable() {
        let rate_limit_error = TradingError::RateLimitExceeded;
        assert!(rate_limit_error.is_retryable());

        let config_error = TradingError::Config("test".to_string());
        assert!(!config_error.is_retryable());
    }

    #[test]
    fn test_retry_delay() {
        let rate_limit_error = TradingError::RateLimitExceeded;
        assert_eq!(rate_limit_error.retry_delay(), std::time::Duration::from_secs(5));

        let api_error = TradingError::Upbit(UpbitError::RequestFailed("test".to_string()));
        assert_eq!(api_error.retry_delay(), std::time::Duration::from_secs(1));
    }

    #[test]
    fn test_insufficient_balance_error() {
        let error = TradingError::InsufficientBalance {
            required: 10000.0,
            available: 5000.0,
        };
        assert_eq!(
            error.to_string(),
            "Insufficient balance: required 10000, available 5000"
        );
    }
}
