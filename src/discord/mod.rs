//! Discord webhook integration
//!
//! Discord로 알림을 전송하는 모듈입니다.

use crate::error::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub use webhook::DiscordWebhook;

mod webhook;

/// Discord Webhook 클라이언트
pub struct DiscordClient {
    webhook_url: String,
    client: Client,
}

impl DiscordClient {
    /// 새로운 클라이언트 생성
    pub fn new(webhook_url: String) -> Self {
        Self {
            webhook_url,
            client: Client::new(),
        }
    }

    /// 메시지 전송
    pub async fn send(&self, webhook: DiscordWebhook) -> Result<()> {
        let resp = self
            .client
            .post(&self.webhook_url)
            .json(&webhook)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(crate::error::TradingError::DiscordNotification(
                format!("Status: {}", resp.status()),
            ));
        }

        Ok(())
    }

    /// 간단한 텍스트 메시지 전송
    pub async fn send_message(&self, content: &str) -> Result<()> {
        let webhook = DiscordWebhook::simple(content);
        self.send(webhook).await
    }
}

/// Discord Embed 필드
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    #[serde(default)]
    pub inline: bool,
}

impl EmbedField {
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
            inline: false,
        }
    }

    pub fn inline(mut self) -> Self {
        self.inline = true;
        self
    }
}

/// Discord Embed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordEmbed {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub color: i32,
    #[serde(default)]
    pub fields: Vec<EmbedField>,
    pub timestamp: String,
}

impl DiscordEmbed {
    pub fn new(title: String, color: i32) -> Self {
        Self {
            title,
            description: None,
            color,
            fields: Vec::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn with_description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }

    pub fn add_field(mut self, field: EmbedField) -> Self {
        self.fields.push(field);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_field() {
        let field = EmbedField::new("Test".to_string(), "Value".to_string()).inline();
        assert!(field.inline);
    }

    #[test]
    fn test_discord_embed() {
        let embed = DiscordEmbed::new("Test".to_string(), 12345)
            .add_field(EmbedField::new("Field1".to_string(), "Value1".to_string()));

        assert_eq!(embed.title, "Test");
        assert_eq!(embed.fields.len(), 1);
    }
}
