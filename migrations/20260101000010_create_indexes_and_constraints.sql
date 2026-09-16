-- 0010_create_indexes_and_constraints.sql
-- High-performance composite indexes for real-time analytics queries

CREATE INDEX IF NOT EXISTS idx_traffic_src_time ON traffic_events(src_ip, time DESC);
CREATE INDEX IF NOT EXISTS idx_traffic_dst_time ON traffic_events(dst_ip, time DESC);
CREATE INDEX IF NOT EXISTS idx_traffic_dst_port_time ON traffic_events(dst_port, time DESC);
CREATE INDEX IF NOT EXISTS idx_traffic_protocol_time ON traffic_events(protocol, time DESC);

CREATE INDEX IF NOT EXISTS idx_alerts_unresolved ON alerts(detected_at DESC) WHERE status != 'resolved';
CREATE INDEX IF NOT EXISTS idx_alerts_critical ON alerts(detected_at DESC) WHERE severity = 'critical';

CREATE INDEX IF NOT EXISTS idx_devices_trusted ON devices(is_trusted);
