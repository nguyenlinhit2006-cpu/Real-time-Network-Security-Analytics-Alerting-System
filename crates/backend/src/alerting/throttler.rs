use dashmap::DashMap;
use ipnetwork::IpNetwork;
use std::time::{Duration, Instant};
use uuid::Uuid;

pub struct AlertThrottler {
    cache: DashMap<(Option<Uuid>, IpNetwork), Instant>,
    window: Duration,
}

impl AlertThrottler {
    pub fn new(window_seconds: u64) -> Self {
        Self {
            cache: DashMap::new(),
            window: Duration::from_secs(window_seconds),
        }
    }

    /// Returns true if this alert should be suppressed due to recent duplicate sending
    pub fn should_throttle(&self, rule_id: Option<Uuid>, src_ip: IpNetwork) -> bool {
        let key = (rule_id, src_ip);
        let now = Instant::now();

        if let Some(mut last_seen) = self.cache.get_mut(&key) {
            if now.duration_since(*last_seen) < self.window {
                return true;
            }
            *last_seen = now;
            false
        } else {
            self.cache.insert(key, now);
            false
        }
    }
}
