//! Configuration management
//!
//! 환경 변수와 설정 파일에서 시스템 설정을 로드합니다.
//!
//! # Environment Variables
//!
//! API 키는 환경 변수로 설정할 수 있으며, 환경 변수가 설정된 경우 설정 파일보다 우선합니다:
//!
//! - `UPBIT_ACCESS_KEY`: Upbit API Access Key
//! - `UPBIT_SECRET_KEY`: Upbit API Secret Key
//! - `UPBIT_API_URL`: Upbit REST API URL (optional, default: https://api.upbit.com/v1)
//! - `UPBIT_WS_URL`: Upbit WebSocket URL (optional, default: wss://api.upbit.com/websocket/v1)
//! - `DISCORD_WEBHOOK_URL`: Discord Webhook URL
//! - `LOG_LEVEL`: Log level (trace, debug, info, warn, error)
//!
//! # Configuration File
//!
//! `config.toml` 파일이 존재하지 않아도 환경 변수가 설정되어 있으면 정상 동작합니다.

use serde::{Deserialize, Serialize};
use std::path::Path;

pub use settings::{Settings, TradingConfig};

mod settings {
    use super::*;

    /// 메인 설정 구조체
    ///
    /// 모든 시스템 설정을 포함합니다.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Settings {
        /// Upbit API 설정
        pub upbit: UpbitConfig,
        /// 트레이딩 파라미터
        pub trading: TradingConfig,
        /// Discord 설정
        pub discord: DiscordConfig,
        /// 시스템 설정
        pub system: SystemConfig,
        /// 로그 설정
        pub logging: LoggingConfig,
    }

    impl Settings {
        /// 설정 파일과 환경 변수에서 설정 로드
        ///
        /// 설정 파일(config.toml)이 없어도 환경 변수로 API 키를 설정할 수 있습니다.
        /// 환경 변수: UPBIT_ACCESS_KEY, UPBIT_SECRET_KEY
        pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
            // .env 파일 로드
            dotenvy::dotenv().ok();

            // 기본값으로 설정 초기화
            let default_settings = Settings::default();

            // 설정 파일 로드 시도 (선택적)
            let file_settings = Self::load_from_file().unwrap_or_else(|_| Settings::default());

            // 환경 변수로 오버라이드 (파일 설정보다 우선)
            let settings = settings::with_env_override(file_settings);

            // 파일 로드 실패 시 환경 변수만으로 동작하는지 확인
            if settings.upbit.access_key.is_empty() && default_settings.upbit.access_key.is_empty() {
                // 파일도 없고 환경 변수도 없는 경우 - 기본값 반환 (나중에 validate()에서 검증됨)
            }

            Ok(settings)
        }

        /// 설정 파일에서만 로드 (파일이 없으면 오류)
        fn load_from_file() -> Result<Settings, Box<dyn std::error::Error>> {
            let config_path = "config.toml";
            if !Path::new(config_path).exists() {
                return Err("Config file not found".into());
            }

            let content = std::fs::read_to_string(config_path)?;
            let settings: Settings = toml::from_str(&content)?;
            Ok(settings)
        }

        /// 개발 환경용 설정 로드
        pub fn load_dev() -> Result<Self, Box<dyn std::error::Error>> {
            // .env 파일 로드
            dotenvy::dotenv().ok();

            // 개발용 기본값
            let default_settings = Settings::default();

            // 개발 설정 파일 로드 시도 (선택적)
            let file_settings = Self::load_from_file().unwrap_or(default_settings);

            // 환경 변수로 오버라이드
            Ok(settings::with_env_override(file_settings))
        }
    }

    impl Default for Settings {
        fn default() -> Self {
            Self {
                upbit: UpbitConfig::default(),
                trading: TradingConfig::default(),
                discord: DiscordConfig::default(),
                system: SystemConfig::default(),
                logging: LoggingConfig::default(),
            }
        }
    }

    /// 환경 변수로 설정 오버라이드
    fn with_env_override(mut settings: Settings) -> Settings {
        // Upbit API URL
        if let Ok(url) = std::env::var("UPBIT_API_URL") {
            settings.upbit.api_url = url;
        }
        if let Ok(url) = std::env::var("UPBIT_WS_URL") {
            settings.upbit.ws_url = url;
        }

        // API Key
        if let Ok(key) = std::env::var("UPBIT_ACCESS_KEY") {
            settings.upbit.access_key = key;
        }
        if let Ok(key) = std::env::var("UPBIT_SECRET_KEY") {
            settings.upbit.secret_key = key;
        }

        // 트레이딩 파라미터
        if let Ok(val) = std::env::var("TRADING_TARGET_COINS") {
            settings.trading.target_coins = val.parse().unwrap_or(20);
        }
        if let Ok(val) = std::env::var("TARGET_PROFIT_RATE") {
            settings.trading.profit_rate = val.parse().unwrap_or(0.10);
        }
        if let Ok(val) = std::env::var("STOP_LOSS_RATE") {
            settings.trading.stop_loss_rate = val.parse().unwrap_or(0.05);
        }
        if let Ok(val) = std::env::var("SURGE_THRESHOLD") {
            settings.trading.surge_threshold = val.parse().unwrap_or(0.05);
        }
        if let Ok(val) = std::env::var("MIN_ORDER_AMOUNT_KRW") {
            settings.trading.min_order_amount = val.parse().unwrap_or(5000.0);
        }

        // Discord
        if let Ok(url) = std::env::var("DISCORD_WEBHOOK_URL") {
            settings.discord.webhook_url = url;
        }

        // 시스템
        if let Ok(path) = std::env::var("DB_PATH") {
            settings.system.db_path = path;
        }
        if let Ok(level) = std::env::var("LOG_LEVEL") {
            settings.logging.level = level;
        }

        settings
    }

    /// Upbit API 설정
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpbitConfig {
        /// REST API URL
        pub api_url: String,
        /// WebSocket URL
        pub ws_url: String,
        /// Access Key
        pub access_key: String,
        /// Secret Key
        pub secret_key: String,
        /// Rate Limit (초당 요청 수)
        pub rate_limit: u32,
    }

    impl Default for UpbitConfig {
        fn default() -> Self {
            Self {
                api_url: "https://api.upbit.com/v1".to_string(),
                ws_url: "wss://api.upbit.com/websocket/v1".to_string(),
                access_key: String::new(),
                secret_key: String::new(),
                rate_limit: 10,
            }
        }
    }

    /// 트레이딩 파라미터 설정
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TradingConfig {
        /// 타겟 코인 수 (Top N)
        pub target_coins: usize,
        /// 목표 수익률 (예: 0.10 = 10%)
        pub profit_rate: f64,
        /// 손절률 (예: 0.05 = 5%)
        pub stop_loss_rate: f64,
        /// 급등 감지 임계값 (예: 0.05 = 5%)
        pub surge_threshold: f64,
        /// 급등 감지 시간 프레임 (분)
        pub surge_timeframe_minutes: u64,
        /// 거래량 배수 (예: 2.0 = 평균의 2배)
        pub volume_multiplier: f64,
        /// 최소 주문 금액 (KRW)
        pub min_order_amount: f64,
        /// 최대 포지션 비율 (잔고 대비)
        pub max_position_ratio: f64,
        /// 동시 보유 가능 최대 포지션 수
        pub max_positions: usize,
    }

    impl Default for TradingConfig {
        fn default() -> Self {
            Self {
                target_coins: 20,
                profit_rate: 0.10,
                stop_loss_rate: 0.05,
                surge_threshold: 0.05,
                surge_timeframe_minutes: 60,
                volume_multiplier: 2.0,
                min_order_amount: 5000.0,
                max_position_ratio: 0.5,
                max_positions: 1,
            }
        }
    }

    /// Discord 설정
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DiscordConfig {
        /// Webhook URL
        pub webhook_url: String,
        /// 알림 활성화 여부
        pub enabled: bool,
        /// 알림 타입 필터
        pub notify_on_buy: bool,
        pub notify_on_sell: bool,
        pub notify_on_signal: bool,
        pub notify_on_error: bool,
    }

    impl Default for DiscordConfig {
        fn default() -> Self {
            Self {
                webhook_url: String::new(),
                enabled: false,
                notify_on_buy: true,
                notify_on_sell: true,
                notify_on_signal: true,
                notify_on_error: true,
            }
        }
    }

    /// 시스템 설정
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SystemConfig {
        /// 데이터베이스 경로
        pub db_path: String,
        /// 데이터 디렉토리
        pub data_dir: String,
    }

    impl Default for SystemConfig {
        fn default() -> Self {
            Self {
                db_path: "./data/trading.db".to_string(),
                data_dir: "./data".to_string(),
            }
        }
    }

    /// 로그 설정
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LoggingConfig {
        /// 로그 레벨
        pub level: String,
        /// JSON 형식 로그
        pub json_format: bool,
        /// 로그 파일 경로
        pub log_file: Option<String>,
    }

    impl Default for LoggingConfig {
        fn default() -> Self {
            Self {
                level: "info".to_string(),
                json_format: true,
                log_file: None,
            }
        }
    }

    impl Settings {
        /// 설정 유효성 검증
        pub fn validate(&self) -> Result<(), String> {
            // Upbit API 키 검증
            if self.upbit.access_key.is_empty() {
                return Err("UPBIT_ACCESS_KEY is required".to_string());
            }
            if self.upbit.secret_key.is_empty() {
                return Err("UPBIT_SECRET_KEY is required".to_string());
            }

            // 트레이딩 파라미터 검증
            if self.trading.profit_rate <= 0.0 {
                return Err("TARGET_PROFIT_RATE must be positive".to_string());
            }
            if self.trading.stop_loss_rate < 0.0 || self.trading.stop_loss_rate >= 1.0 {
                return Err("STOP_LOSS_RATE must be between 0 and 1".to_string());
            }
            if self.trading.min_order_amount < 1000.0 {
                return Err("MIN_ORDER_AMOUNT_KRW must be at least 1000".to_string());
            }

            Ok(())
        }

        /// 디버그용 설정 출력 (민감 정보 마스킹)
        pub fn display_safe(&self) -> String {
            format!(
                "Settings {{\n  upbit: {{ api_url: {}, ws_url: {}, access_key: {}, rate_limit: {} }},\n  trading: {{ target_coins: {}, profit_rate: {}, stop_loss_rate: {} }},\n  discord: {{ enabled: {} }},\n  system: {{ db_path: {} }}\n}}",
                self.upbit.api_url,
                self.upbit.ws_url,
                mask_key(&self.upbit.access_key),
                self.upbit.rate_limit,
                self.trading.target_coins,
                self.trading.profit_rate,
                self.trading.stop_loss_rate,
                self.discord.enabled,
                self.system.db_path
            )
        }
    }

    /// API 키 마스킹 (디버깅용)
    fn mask_key(key: &str) -> String {
        if key.len() <= 8 {
            "***".to_string()
        } else {
            format!("{}...{}", &key[..4], &key[key.len() - 4..])
        }
    }
}

/// 전략 설정 (TOML 파일에서 로드)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    pub strategy: StrategySettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategySettings {
    pub name: String,
    pub version: String,
    pub surge_detection: SurgeDetectionConfig,
    pub position: PositionConfig,
    pub risk: RiskConfig,
    pub targets: TargetsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurgeDetectionConfig {
    pub timeframe_minutes: u64,
    pub threshold_percent: f64,
    pub volume_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionConfig {
    pub max_positions: usize,
    pub max_position_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfig {
    pub stop_loss_percent: f64,
    pub take_profit_percent: f64,
    pub trailing_stop_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetsConfig {
    pub top_n_coins: usize,
    pub min_volume_24h: u64,
}

impl StrategyConfig {
    /// 파일에서 전략 설정 로드
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: StrategyConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// 기본 전략 설정 반환
    pub fn default_config() -> Self {
        Self {
            strategy: StrategySettings {
                name: "momentum_following".to_string(),
                version: "1.0".to_string(),
                surge_detection: SurgeDetectionConfig {
                    timeframe_minutes: 60,
                    threshold_percent: 5.0,
                    volume_multiplier: 2.0,
                },
                position: PositionConfig {
                    max_positions: 1,
                    max_position_ratio: 0.5,
                },
                risk: RiskConfig {
                    stop_loss_percent: 5.0,
                    take_profit_percent: 10.0,
                    trailing_stop_enabled: false,
                },
                targets: TargetsConfig {
                    top_n_coins: 20,
                    min_volume_24h: 1_000_000_000,
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_default() {
        let upbit = UpbitConfig::default();
        assert_eq!(upbit.api_url, "https://api.upbit.com/v1");
        assert_eq!(upbit.rate_limit, 10);
    }

    #[test]
    fn test_trading_config_default() {
        let trading = TradingConfig::default();
        assert_eq!(trading.target_coins, 20);
        assert_eq!(trading.profit_rate, 0.10);
        assert_eq!(trading.stop_loss_rate, 0.05);
    }

    #[test]
    fn test_key_masking() {
        assert_eq!(settings::mask_key("short"), "***");
        assert!(settings::mask_key("very_long_key_here").contains("..."));
        assert!(!settings::mask_key("very_long_key_here").contains("very_long"));
    }

    #[test]
    fn test_strategy_config_default() {
        let config = StrategyConfig::default_config();
        assert_eq!(config.strategy.name, "momentum_following");
        assert_eq!(config.strategy.surge_detection.timeframe_minutes, 60);
    }

    #[test]
    fn test_settings_validate_missing_key() {
        let settings = Settings {
            upbit: UpbitConfig {
                access_key: String::new(),
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(settings.validate().is_err());
    }
}
