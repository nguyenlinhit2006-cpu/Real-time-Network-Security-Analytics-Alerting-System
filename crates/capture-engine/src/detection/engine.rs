use common::models::{Alert, DetectionRule as RuleModel, TrafficEvent};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::{error, info, warn};

use super::arp_spoof::ArpSpoofDetector;
use super::brute_force::BruteForceDetector;
use super::dns_tunneling::DnsTunnelDetector;
use super::port_scan::PortScanDetector;
use super::syn_flood::SynFloodDetector;
use super::zscore_anomaly::ZScoreAnomalyDetector;
use super::DetectionRule;

pub struct DetectionEngine {
    rules: Vec<Box<dyn DetectionRule>>,
    alert_tx: Sender<Alert>,
}

impl DetectionEngine {
    pub fn new(alert_tx: Sender<Alert>) -> Self {
        let rules: Vec<Box<dyn DetectionRule>> = vec![
            Box::new(PortScanDetector::new(15, 10)),
            Box::new(SynFloodDetector::new(200, 5)),
            Box::new(BruteForceDetector::new(5, 30)),
            Box::new(ArpSpoofDetector::new()),
            Box::new(DnsTunnelDetector::new(3.8, 30)),
            Box::new(ZScoreAnomalyDetector::new(3.0, 100)),
        ];

        Self { rules, alert_tx }
    }

    /// Update detection rules dynamically from database configuration
    pub async fn reload_rules_from_db(&mut self, pool: &PgPool) -> Result<(), sqlx::Error> {
        let db_rules = sqlx::query_as::<_, RuleModel>(
            "SELECT id, name, rule_type, condition_json, severity, is_enabled, threshold_value, time_window_seconds, created_at, updated_at FROM detection_rules"
        )
        .fetch_all(pool)
        .await?;

        for db_rule in db_rules {
            for rule in &mut self.rules {
                if rule.name() == db_rule.name {
                    rule.update_config(&db_rule);
                    info!("Updated configuration for rule: {}", rule.name());
                }
            }
        }

        Ok(())
    }

    /// Process a single event through all detection rules
    pub async fn process_event(&mut self, event: &TrafficEvent) {
        for rule in &mut self.rules {
            if let Some(alert) = rule.evaluate(event) {
                info!("🚨 [ALERT TRIGGERED] {} - {}", alert.title, alert.description);
                if let Err(e) = self.alert_tx.send(alert).await {
                    error!("Failed to forward alert to alerting channel: {}", e);
                }
            }
        }
    }

    /// Process a batch of events (optimized for high-throughput pipeline > 10,000 pkts/sec)
    pub async fn process_batch(&mut self, events: &[TrafficEvent]) {
        for event in events {
            for rule in &mut self.rules {
                if let Some(alert) = rule.evaluate(event) {
                    info!("🚨 [ALERT TRIGGERED] {} - {}", alert.title, alert.description);
                    if let Err(e) = self.alert_tx.send(alert).await {
                        error!("Failed to forward alert to alerting channel: {}", e);
                    }
                }
            }
        }
    }
}

/// Spawns background task to persist alerts to PostgreSQL and dispatch them
pub fn spawn_alert_persister(mut alert_rx: Receiver<Alert>, pool: Option<Arc<PgPool>>) {
    tokio::spawn(async move {
        while let Some(alert) = alert_rx.recv().await {
            if let Some(ref pool) = pool {
                let result = sqlx::query(
                    r#"
                    INSERT INTO alerts (id, rule_id, severity, title, description, src_ip, dst_ip, detected_at, status)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                    "#,
                )
                .bind(alert.id)
                .bind(alert.rule_id)
                .bind(alert.severity)
                .bind(&alert.title)
                .bind(&alert.description)
                .bind(alert.src_ip)
                .bind(alert.dst_ip)
                .bind(alert.detected_at)
                .bind(alert.status)
                .execute(pool.as_ref())
                .await;

                if let Err(e) = result {
                    warn!("Failed to persist alert to database: {}", e);
                } else {
                    info!("Persisted alert {} successfully to database", alert.id);
                }
            }
        }
    });
}
