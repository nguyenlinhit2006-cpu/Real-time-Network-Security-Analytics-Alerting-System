-- 0007_create_notification_channels_table.sql
-- Table: notification_channels (configured notification targets)

CREATE TABLE IF NOT EXISTS notification_channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    type channel_type NOT NULL,
    config_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    min_severity alert_severity NOT NULL DEFAULT 'medium',
    is_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_notification_channels_is_enabled ON notification_channels(is_enabled);
