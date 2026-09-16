use chrono::Utc;
use common::models::{Alert, AlertSeverity, AlertStatus, DetectionRule as RuleModel, RuleType, TrafficEvent};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::DetectionRule;

pub struct ZScoreAnomalyDetector {
    rule_id: Option<Uuid>,
    is_enabled: bool,
    z_threshold: f64,
    window_size: usize,
    history: VecDeque<f64>,
    last_alert_time: Option<Instant>,
}

impl ZScoreAnomalyDetector {
    pub fn new(z_threshold: f64, window_size: usize) -> Self {
        Self {
            rule_id: None,
            is_enabled: true,
            z_threshold,
            window_size,
            history: VecDeque::with_capacity(window_size),
            last_alert_time: None,
        }
    }

    fn calculate_z_score(&self, value: f64) -> Option<f64> {
        if self.history.len() < 10 {
            return None; // Not enough baseline samples yet
        }

        let mean = self.history.iter().sum::<f64>() / self.history.len() as f64;
        let variance = self.history.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / self.history.len() as f64;
        let std_dev = variance.sqrt().max(1.0);

        Some((value - mean) / std_dev)
    }
}

impl DetectionRule for ZScoreAnomalyDetector {
    fn name(&self) -> &str {
        "Traffic Volume Anomaly (Z-Score)"
    }

    fn rule_type(&self) -> RuleType {
        RuleType::Anomaly
    }

    fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    fn update_config(&mut self, config: &RuleModel) {
        self.rule_id = Some(config.id);
        self.is_enabled = config.is_enabled;
        self.z_threshold = config.threshold_value;
    }

    fn evaluate(&mut self, event: &TrafficEvent) -> Option<Alert> {
        if !self.is_enabled {
            return None;
        }

        let metric_value = event.bytes_transferred as f64;

        let z_opt = self.calculate_z_score(metric_value);

        // Update sliding window history
        if self.history.len() >= self.window_size {
            self.history.pop_front();
        }
        self.history.push_back(metric_value);

        if let Some(z_score) = z_opt {
            if z_score >= self.z_threshold {
                let now = Instant::now();
                if let Some(last_alert) = self.last_alert_time {
                    if now.duration_since(last_alert) < Duration::from_secs(30) {
                        return None;
                    }
                }

                self.last_alert_time = Some(now);

                return Some(Alert {
                    id: Uuid::new_v4(),
                    rule_id: self.rule_id,
                    severity: AlertSeverity::Medium,
                    title: "Traffic Volume Spike Anomaly Detected".to_string(),
                    description: format!(
                        "Abnormal traffic spike of {} bytes detected (Z-Score: {:.2}, threshold: {:.2})",
                        event.bytes_transferred, z_score, self.z_threshold
                    ),
                    src_ip: event.src_ip,
                    dst_ip: event.dst_ip,
                    detected_at: Utc::now(),
                    status: AlertStatus::Open,
                    acknowledged_by: None,
                    resolved_at: None,
                });
            }
        }

        None
    }
}
