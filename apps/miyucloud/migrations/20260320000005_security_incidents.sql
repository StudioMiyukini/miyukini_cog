-- ============================================================
-- Miyukini Cloud: Security Incident Tracking
-- ISO 27001 A.5.24-28, RGPD Art. 33-34, HDS mandatory
-- ============================================================

CREATE TABLE IF NOT EXISTS auth.security_incidents (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    severity        VARCHAR(16) NOT NULL DEFAULT 'medium',  -- low, medium, high, critical
    status          VARCHAR(32) NOT NULL DEFAULT 'detected', -- detected, assessed, notified_authority, notified_subjects, resolved
    title           TEXT NOT NULL,
    description     TEXT,
    detected_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    assessed_at     TIMESTAMPTZ,
    notified_at     TIMESTAMPTZ,
    resolved_at     TIMESTAMPTZ,
    affected_users  INTEGER DEFAULT 0,
    reporter_id     UUID REFERENCES auth.users(id),
    details         JSONB,
    remediation     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_incidents_status
    ON auth.security_incidents (status, detected_at DESC);

CREATE INDEX IF NOT EXISTS idx_incidents_severity
    ON auth.security_incidents (severity, detected_at DESC);
