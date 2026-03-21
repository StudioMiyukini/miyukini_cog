-- ============================================================
-- Miyukini Cloud: GDPR Consent Management
-- RGPD Art. 6-7 (Consent tracking)
-- ============================================================

CREATE TABLE IF NOT EXISTS auth.consents (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    purpose     VARCHAR(64) NOT NULL,
    granted     BOOLEAN NOT NULL,
    granted_at  TIMESTAMPTZ,
    revoked_at  TIMESTAMPTZ,
    ip_address  INET,
    version     VARCHAR(16) NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_consents_user
    ON auth.consents (user_id, purpose);

-- Enable RLS on consents
ALTER TABLE auth.consents ENABLE ROW LEVEL SECURITY;
ALTER TABLE auth.consents FORCE ROW LEVEL SECURITY;

DO $$ BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies WHERE tablename = 'consents' AND schemaname = 'auth' AND policyname = 'consents_user_isolation'
    ) THEN
        CREATE POLICY consents_user_isolation ON auth.consents
            USING (
                user_id = auth.current_app_user_id()
                OR auth.current_app_user_id() IS NULL
            );
    END IF;
END $$;
