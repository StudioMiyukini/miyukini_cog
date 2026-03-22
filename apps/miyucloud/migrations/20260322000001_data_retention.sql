-- Miyukini Cloud: Data Retention Policies — RGPD Art. 5(1)(e)

-- Add last_activity to sessions for idle timeout tracking
ALTER TABLE auth.sessions ADD COLUMN IF NOT EXISTS last_activity TIMESTAMPTZ DEFAULT NOW();

-- Add password_reset_token and password_reset_expires for password reset flow
ALTER TABLE auth.users ADD COLUMN IF NOT EXISTS password_reset_token TEXT;
ALTER TABLE auth.users ADD COLUMN IF NOT EXISTS password_reset_expires TIMESTAMPTZ;

-- Index for cleanup queries
CREATE INDEX IF NOT EXISTS idx_sessions_last_activity
    ON auth.sessions (last_activity)
    WHERE NOT revoked;

-- Function to auto-update last_activity when session is used
CREATE OR REPLACE FUNCTION auth.touch_session_activity()
RETURNS trigger AS $FUNC$
BEGIN
    NEW.last_activity = NOW();
    RETURN NEW;
END;
$FUNC$ LANGUAGE plpgsql;

-- Auto-cleanup function for old audit logs (retention policy)
-- Can be called via pg_cron or application background task
CREATE OR REPLACE FUNCTION audit.cleanup_old_events(retention_days INTEGER DEFAULT 365)
RETURNS INTEGER AS $FUNC$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM audit.events
    WHERE timestamp < NOW() - make_interval(days => retention_days);
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$FUNC$ LANGUAGE plpgsql;

-- Auto-cleanup function for expired sessions
CREATE OR REPLACE FUNCTION auth.cleanup_expired_sessions()
RETURNS INTEGER AS $FUNC$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM auth.sessions
    WHERE revoked = true AND expires_at < NOW() - INTERVAL '30 days';
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$FUNC$ LANGUAGE plpgsql;
