//! Discord Webhook structures

use serde::{Deserialize, Serialize};
use super::DiscordEmbed;

/// Discord Webhook 페이로드
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<DiscordEmbed>,
}

impl DiscordWebhook {
    /// 간단한 텍스트 메시지 생성
    pub fn simple(content: &str) -> Self {
        Self {
            content: Some(content.to_string()),
            embeds: Vec::new(),
        }
    }

    /// Embed 메시지 생성
    pub fn embed(embed: DiscordEmbed) -> Self {
        Self {
            content: None,
            embeds: vec![embed],
        }
    }

    /// 여러 Embed 포함한 메시지 생성
    pub fn embeds(embeds: Vec<DiscordEmbed>) -> Self {
        Self {
            content: None,
            embeds,
        }
    }
}
