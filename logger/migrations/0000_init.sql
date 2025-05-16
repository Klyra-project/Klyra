CREATE TABLE IF NOT EXISTS logs (
    deployment_id TEXT,        -- The deployment that this log line pertains to.
    klyra_service_name TEXT, -- The klyra service which created this log.
    timestamp INTEGER,         -- Unix epoch timestamp.
    level INTEGER,             -- The log level
    fields TEXT                -- Log fields object.
);
