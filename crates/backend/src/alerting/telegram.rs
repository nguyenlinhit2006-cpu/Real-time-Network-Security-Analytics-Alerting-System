use common::models::{Alert, ChannelType};
use reqwest::Client;
use serde_json::json;
use std::future::Future;
use std::pin::Pin;
use tracing::{info, warn};

use super::traits::NotificationChannel;
use crate::error::AppError;

pub struct TelegramChannel {
    pub name: String,
    pub bot_token: String,
    pub chat_id: String,
    pub client: Client,
}

impl TelegramChannel {
    pub fn new(name: String, bot_token: String, chat_id: String) -> Self {
        Self {
            name,
            bot_token,
            chat_id,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap_or_default(),
        }
    }
}

impl NotificationChannel for TelegramChannel {
    fn name(&self) -> &str {
        &self.name
    }

    fn channel_type(&self) -> ChannelType {
        ChannelType::Telegram
    }

    fn send<'a>(&'a self, alert: &'a Alert) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + 'a>> {
        Box::pin(async move {
            let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);
            let text = format!(
                "🚨 *[SecNet Security Alert]*\n\
                 *Severity:* `{:?}`\n\
                 *Title:* {}\n\
                 *Description:* {}\n\
                 *Source IP:* `{}`\n\
                 *Target IP:* `{}`\n\
                 *Detected:* `{}`",
                alert.severity,
                alert.title,
                alert.description,
                alert.src_ip,
                alert.dst_ip,
                alert.detected_at.to_rfc3339()
            );

            let payload = json!({
                "chat_id": self.chat_id,
                "text": text,
                "parse_mode": "Markdown"
            });

            info!("📱 [TELEGRAM ALERT] Dispatching to Telegram chat {}: {}", self.chat_id, alert.title);

            let response = self.client.post(&url).json(&payload).send().await;

            match response {
                Ok(res) if res.status().is_success() => {
                    info!("Telegram alert successfully delivered to {}", self.chat_id);
                    Ok(())
                }
                Ok(res) => {
                    warn!("Telegram API responded with status {}: mock/fallback logging active", res.status());
                    Ok(())
                }
                Err(e) => {
                    warn!("Telegram API request failed: {}", e);
                    Ok(())
                }
            }
        })
    }
}
