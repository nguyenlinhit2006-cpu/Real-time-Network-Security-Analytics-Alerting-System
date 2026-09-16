-- 0005_create_traffic_events_hypertable.sql
-- Table: traffic_events (time-series hypertable for captured network traffic)

CREATE TABLE IF NOT EXISTS traffic_events (
    time TIMESTAMPTZ NOT NULL,
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    src_ip INET NOT NULL,
    dst_ip INET NOT NULL,
    src_port INT NOT NULL,
    dst_port INT NOT NULL,
    protocol VARCHAR(10) NOT NULL,
    bytes_transferred BIGINT NOT NULL DEFAULT 0,
    packet_count INT NOT NULL DEFAULT 1,
    flags VARCHAR(20) NOT NULL DEFAULT '',
    interface_name VARCHAR(32) NOT NULL DEFAULT 'eth0',
    PRIMARY KEY (time, id),
    CONSTRAINT check_src_port_range CHECK (src_port >= 0 AND src_port <= 65535),
    CONSTRAINT check_dst_port_range CHECK (dst_port >= 0 AND dst_port <= 65535),
    CONSTRAINT check_bytes_positive CHECK (bytes_transferred >= 0),
    CONSTRAINT check_packets_positive CHECK (packet_count >= 1)
);

-- Convert table into TimescaleDB Hypertable with 1-day chunk interval
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'timescaledb') THEN
        PERFORM create_hypertable(
            'traffic_events',
            'time',
            chunk_time_interval => INTERVAL '1 day',
            if_not_exists => TRUE
        );
        RAISE NOTICE 'Converted traffic_events to TimescaleDB hypertable successfully.';
    ELSE
        RAISE NOTICE 'TimescaleDB extension not active; traffic_events created as standard table.';
    END IF;
END $$;
