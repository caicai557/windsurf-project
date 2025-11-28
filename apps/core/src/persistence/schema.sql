CREATE TABLE IF NOT EXISTS workflow_instances (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL,
    flow_definition_id TEXT NOT NULL,
    state JSON NOT NULL,
    status TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_account_status ON workflow_instances(account_id, status);
