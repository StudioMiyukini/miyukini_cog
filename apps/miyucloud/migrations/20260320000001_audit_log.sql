-- ============================================================
-- Miyukini Cloud: Persistent Audit Log
-- ISO 27001 A.8.15/A.8.16, RGPD Art.5(2), HDS mandatory
-- ============================================================

CREATE SCHEMA IF NOT EXISTS audit;

CREATE TABLE IF NOT EXISTS audit.events (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    timestamp       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id         UUID,
    username        TEXT,
    action          VARCHAR(64) NOT NULL,
    resource_type   VARCHAR(32) NOT NULL,
    resource_id     TEXT,
    ip_address      INET,
    user_agent      TEXT,
    details         JSONB,
    outcome         VARCHAR(16) NOT NULL DEFAULT 'success'
);

-- Query by time range (most common access pattern for dashboards)
CREATE INDEX IF NOT EXISTS idx_audit_timestamp
    ON audit.events (timestamp DESC);

-- Query by user (GDPR right of access, incident investigation)
CREATE INDEX IF NOT EXISTS idx_audit_user
    ON audit.events (user_id, timestamp DESC);

-- Query by action type (security monitoring)
CREATE INDEX IF NOT EXISTS idx_audit_action
    ON audit.events (action, timestamp DESC);

-- Query by outcome (find failures/denials)
CREATE INDEX IF NOT EXISTS idx_audit_outcome
    ON audit.events (outcome, timestamp DESC)
    WHERE outcome != 'success';

-- Composite for admin dashboard filtering
CREATE INDEX IF NOT EXISTS idx_audit_user_action
    ON audit.events (user_id, action, timestamp DESC);
