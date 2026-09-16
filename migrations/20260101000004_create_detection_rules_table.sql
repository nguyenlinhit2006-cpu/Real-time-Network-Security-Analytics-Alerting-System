-- 0004_create_detection_rules_table.sql
-- Table: detection_rules (configurable rules for real-time traffic analysis)

CREATE TABLE IF NOT EXISTS detection_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    rule_type rule_type NOT NULL,
    condition_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    severity alert_severity NOT NULL DEFAULT 'medium',
    is_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    threshold_value DOUBLE PRECISION NOT NULL DEFAULT 100.0,
    time_window_seconds INT NOT NULL DEFAULT 60,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT check_time_window_positive CHECK (time_window_seconds > 0),
    CONSTRAINT check_threshold_non_negative CHECK (threshold_value >= 0)
);

CREATE INDEX IF NOT EXISTS idx_detection_rules_is_enabled ON detection_rules(is_enabled);
CREATE INDEX IF NOT EXISTS idx_detection_rules_rule_type ON detection_rules(rule_type);
