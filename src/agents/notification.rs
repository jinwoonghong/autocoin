//! Notification Agent
//!
//! Discord로 거래 알림을 전송합니다.

use crate::db::Database;
use crate::error::Result;
use crate::types::{OrderResult, Signal};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

/// Discord Webhook 색상
pub mod colors {
    pub const BUY: i32 = 3066993;      // 초록
    pub const SELL: i32 = 15158332;     // 빨강
    pub const SIGNAL: i32 = 16776960;   // 노랑
    pub const ERROR: i32 = 15105570;    // 주황
    pub const INFO: i32 = 3447003;      // 파랑
}

/// Discord Embed 메시지
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordEmbed {
    title: String,
    color: i32,
    fields: Vec<EmbedField>,
    timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EmbedField {
    name: String,
    value: String,
    inline: bool,
}

/// Discord Webhook 페이로드
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordWebhook {
    embeds: Vec<DiscordEmbed>,
}

/// Notification Agent
///
/// 역할: Discord로 거래 알림 전송
/// 입력: OrderResult, Signal
/// 출력: Discord Webhook Call
pub struct NotificationAgent {
    webhook_url: String,
    client: Client,
    notify_on_buy: bool,
    notify_on_sell: bool,
    notify_on_signal: bool,
    notify_on_error: bool,
}

impl NotificationAgent {
    /// 새로운 Notification Agent 생성
    pub fn new(
        webhook_url: String,
        notify_on_buy: bool,
        notify_on_sell: bool,
        notify_on_signal: bool,
        notify_on_error: bool,
    ) -> Self {
        let client = Client::new();
        Self {
            webhook_url,
            client,
            notify_on_buy,
            notify_on_sell,
            notify_on_signal,
            notify_on_error,
        }
    }

    /// 비활성화된 Agent 생성
    pub fn disabled() -> Self {
        Self::new(String::new(), false, false, false, false)
    }

    /// 활성화 여부 확인
    pub fn is_enabled(&self) -> bool {
        !self.webhook_url.is_empty()
    }

    /// 매수 알림 전송 (REQ-009)
    pub async fn notify_buy(&self, market: &str, price: f64, amount: f64, confidence: f64) -> Result<()> {
        if !self.notify_on_buy || !self.is_enabled() {
            return Ok(());
        }

        let embed = DiscordEmbed {
            title: "매수 체결 알림".to_string(),
            color: colors::BUY,
            fields: vec![
                EmbedField {
                    name: "마켓".to_string(),
                    value: market.to_string(),
                    inline: true,
                },
                EmbedField {
                    name: "가격".to_string(),
                    value: format!("{:.0} KRW", price),
                    inline: true,
                },
                EmbedField {
                    name: "수량".to_string(),
                    value: format!("{:.6}", amount),
                    inline: true,
                },
                EmbedField {
                    name: "금액".to_string(),
                    value: format!("{:.0} KRW", price * amount),
                    inline: true,
                },
                EmbedField {
                    name: "신뢰도".to_string(),
                    value: format!("{:.0}%", confidence * 100.0),
                    inline: true,
                },
            ],
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        self.send_embed(embed).await
    }

    /// 매도 알림 전송 (REQ-009)
    pub async fn notify_sell(
        &self,
        market: &str,
        price: f64,
        amount: f64,
        profit: f64,
        profit_rate: f64,
    ) -> Result<()> {
        if !self.notify_on_sell || !self.is_enabled() {
            return Ok(());
        }

        let color = if profit_rate >= 0.0 { colors::SELL } else { colors::ERROR };
        let title = if profit_rate >= 0.0 {
            "익절 체결 알림".to_string()
        } else {
            "손절 체결 알림".to_string()
        };

        let embed = DiscordEmbed {
            title,
            color,
            fields: vec![
                EmbedField {
                    name: "마켓".to_string(),
                    value: market.to_string(),
                    inline: true,
                },
                EmbedField {
                    name: "가격".to_string(),
                    value: format!("{:.0} KRW", price),
                    inline: true,
                },
                EmbedField {
                    name: "수량".to_string(),
                    value: format!("{:.6}", amount),
                    inline: true,
                },
                EmbedField {
                    name: "총 수익/손실".to_string(),
                    value: format!("{:.0} KRW ({:+.2}%)", profit, profit_rate * 100.0),
                    inline: false,
                },
            ],
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        self.send_embed(embed).await
    }

    /// 신호 알림 전송
    pub async fn notify_signal(&self, signal: &Signal) -> Result<()> {
        if !self.notify_on_signal || !self.is_enabled() {
            return Ok(());
        }

        let title = match signal.signal_type {
            crate::types::SignalType::Buy => "매수 신호 감지".to_string(),
            crate::types::SignalType::StrongBuy => "강한 매수 신호 감지".to_string(),
            crate::types::SignalType::Sell => "매도 신호 감지".to_string(),
            crate::types::SignalType::StrongSell => "강한 매도 신호 감지".to_string(),
            crate::types::SignalType::Hold => return Ok(()),
        };

        let embed = DiscordEmbed {
            title,
            color: colors::SIGNAL,
            fields: vec![
                EmbedField {
                    name: "마켓".to_string(),
                    value: signal.market.clone(),
                    inline: true,
                },
                EmbedField {
                    name: "신뢰도".to_string(),
                    value: format!("{:.0}%", signal.confidence * 100.0),
                    inline: true,
                },
                EmbedField {
                    name: "사유".to_string(),
                    value: signal.reason.clone(),
                    inline: false,
                },
            ],
            timestamp: signal.timestamp.to_rfc3339(),
        };

        self.send_embed(embed).await
    }

    /// 에러 알림 전송
    pub async fn notify_error(&self, error: &str) -> Result<()> {
        if !self.notify_on_error || !self.is_enabled() {
            return Ok(());
        }

        let embed = DiscordEmbed {
            title: "시스템 에러".to_string(),
            color: colors::ERROR,
            fields: vec![
                EmbedField {
                    name: "에러".to_string(),
                    value: error.to_string(),
                    inline: false,
                },
                EmbedField {
                    name: "시간".to_string(),
                    value: chrono::Utc::now().to_rfc3339(),
                    inline: false,
                },
            ],
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        self.send_embed(embed).await
    }

    /// Embed 메시지 전송
    async fn send_embed(&self, embed: DiscordEmbed) -> Result<()> {
        let webhook = DiscordWebhook {
            embeds: vec![embed],
        };

        // REQ-016: Discord 실패 시 로그 기록하고 계속 실행
        match self
            .client
            .post(&self.webhook_url)
            .json(&webhook)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                info!("Discord notification sent successfully");
                Ok(())
            }
            Ok(resp) => {
                let status = resp.status();
                warn!("Discord notification failed: {}", status);
                // 실패해도 주요 기능 계속 실행 (REQ-016)
                Ok(())
            }
            Err(e) => {
                // REQ-016: 로그 기록하고 계속
                warn!("Discord notification error: {} (continuing operation)", e);
                Ok(())
            }
        }
    }

    /// 주문 결과 알림
    pub async fn notify_order_result(&self, result: &OrderResult) -> Result<()> {
        if !result.success {
            return self.notify_error(
                result
                    .error
                    .as_deref()
                    .unwrap_or("Unknown error"),
            )
            .await;
        }

        let order = &result.order;
        match order.side {
            crate::types::OrderSide::Bid => {
                // 매수 알림 - 신뢰도는 없으므로 기본값 사용
                self.notify_buy(&order.market, order.price, order.volume, 0.8)
                    .await
            }
            crate::types::OrderSide::Ask => {
                // 매도 알림 - PnL 정보 없이 기본 전송
                self.notify_sell(&order.market, order.price, order.volume, 0.0, 0.0)
                    .await
            }
        }
    }

    /// 독립 실행용 스폰 함수
    pub async fn spawn(
        webhook_url: String,
        mut order_rx: mpsc::Receiver<OrderResult>,
    ) -> Result<()> {
        let agent = Self::new(webhook_url, true, true, false, true);

        tokio::spawn(async move {
            while let Some(result) = order_rx.recv().await {
                if let Err(e) = agent.notify_order_result(&result).await {
                    error!("Failed to send notification: {}", e);
                }
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_notification_agent_disabled() {
        let agent = NotificationAgent::disabled();
        assert!(!agent.is_enabled());

        // 비활성화된 agent는 알림을 보내지 않음
        let result = agent.notify_buy("KRW-BTC", 50000.0, 0.001, 0.8).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_discord_embed_creation() {
        let embed = DiscordEmbed {
            title: "Test".to_string(),
            color: colors::INFO,
            fields: vec![],
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        let json = serde_json::to_string(&embed).unwrap();
        assert!(json.contains("Test"));
    }
}
