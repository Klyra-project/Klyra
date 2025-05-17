CREATE TABLE IF NOT EXISTS logs (
    deployment_id TEXT,        -- The deployment that this log line pertains to.
    klyra_service_name TEXT, -- The klyra service which created this log.
    tx_timestamp DATETIME,      -- Unix epoch timestamp.
    data BLOB                  -- Log fields object.
);
