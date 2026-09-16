use common::models::{Alert, ChannelType};
use reqwest::Client;
use std::future::Future;
use std::pin::Pin;
use tracing::{info, warn};

use super::traits::NotificationChannel;
use crate::error::AppError;

pub struct WebhookChannel {
    pub name: String,
    pub endpoint_url: String,
    pub client: Client,
}

impl WebhookChannel {
    pub fn new(name: String, endpoint_url: String) -> Self {
        Self {
            name,
            endpoint_url,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap_or_default(),
        }
    }
}

impl NotificationChannel for WebhookChannel {
    fn name(&self) -> &str {
        &self.name
    }

    fn channel_type(&self) -> ChannelType {
        ChannelType::Webhook
    }

    fn send<'a>(&'a self, alert: &'a Alert) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + 'a>> {
        Box::pin(async move {
            info!("🔗 [WEBHOOK ALERT] Posting incident to {}: {}", self.endpoint_url, alert.title);

            let response = self
                .client
                .post(&self.endpoint_url)
                .json(alert)
                .send()
                .await;

            match response {
                Ok(res) if res.status().is_success() => {
                    info!("Webhook delivered successfully to {}", self.endpoint_url);
                    Ok(())
                }
                Ok(res) => {
                    warn!("Webhook responded with non-2xx status code: {}", res.status());
                    Ok(())
                }
                Err(e) => {
                    warn!("Webhook delivery failed: {}", e);
                    Ok(())
                }
            }
        })
    }
}
