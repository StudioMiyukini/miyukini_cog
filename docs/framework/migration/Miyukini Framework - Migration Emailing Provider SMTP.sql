-- =============================================
-- MIYUKINI FRAMEWORK - MIGRATION EMAILING (SMTP PROVIDER CONFIG)
-- Feature: Emailing (Gmail/SMTP)
-- Version: 1.0.0
-- Date: 2026-01-11
-- =============================================
--
-- Objectif :
-- - Stocker la configuration SMTP (host/port/login/mdp/from/reply-to) pilotable depuis le Back-Office Super Admin
-- - Protéger par RLS (super_admin uniquement)
--
-- Notes sécurité :
-- - Ce stockage contient des secrets (smtp_password). À terme, préférer Vault/Secrets.
-- - L’envoi d’email doit rester server-side (Edge Function) avec service_role.
--
-- Dépendances :
-- - Miyukini Framework - Migration Initiale.sql (helpers RLS, handle_updated_at, is_super_admin)
-- =============================================

-- ---------- ENUMS ----------
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'email_provider_type') THEN
    CREATE TYPE public.email_provider_type AS ENUM ('smtp','gmail_oauth2','gmail_smtp');
  END IF;
END$$;

-- ---------- TABLE ----------
CREATE TABLE IF NOT EXISTS public.email_provider_configs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  workspace_id UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000',
  provider public.email_provider_type NOT NULL DEFAULT 'smtp',
  is_active BOOLEAN NOT NULL DEFAULT true,

  -- SMTP (utilisé si provider='smtp' ou 'gmail_smtp')
  smtp_host TEXT NULL,
  smtp_port INT NULL,
  smtp_username TEXT NULL,
  smtp_password TEXT NULL,
  smtp_secure BOOLEAN NOT NULL DEFAULT false, -- true = SMTPS (465), false = STARTTLS (587)

  from_email TEXT NULL,
  from_name TEXT NULL,
  reply_to TEXT NULL,
  headers_json JSONB NOT NULL DEFAULT '{}'::jsonb,

  last_test_at TIMESTAMPTZ NULL,
  last_test_status TEXT NULL, -- ok|error
  last_test_error TEXT NULL,

  updated_by UUID NULL REFERENCES auth.users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Un seul provider config par workspace+provider
CREATE UNIQUE INDEX IF NOT EXISTS uq_email_provider_configs_workspace_provider
  ON public.email_provider_configs(workspace_id, provider);

-- Index utiles
CREATE INDEX IF NOT EXISTS idx_email_provider_configs_active
  ON public.email_provider_configs(is_active);

-- Contraintes minimales (SMTP)
DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1
    FROM pg_constraint
    WHERE conname = 'chk_email_provider_configs_smtp_required'
  ) THEN
    ALTER TABLE public.email_provider_configs
      ADD CONSTRAINT chk_email_provider_configs_smtp_required
      CHECK (
        provider <> 'smtp'
        OR (
          smtp_host IS NOT NULL AND length(smtp_host) > 1
          AND smtp_port IS NOT NULL AND smtp_port > 0 AND smtp_port < 65536
          AND from_email IS NOT NULL AND length(from_email) > 3
        )
      );
  END IF;
END$$;

-- ---------- updated_at trigger ----------
DROP TRIGGER IF EXISTS email_provider_configs_updated_at ON public.email_provider_configs;
CREATE TRIGGER email_provider_configs_updated_at
  BEFORE UPDATE ON public.email_provider_configs
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

-- ---------- RLS ----------
ALTER TABLE public.email_provider_configs ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS email_provider_configs_super_admin ON public.email_provider_configs;
CREATE POLICY email_provider_configs_super_admin ON public.email_provider_configs
  FOR ALL
  USING (public.is_super_admin())
  WITH CHECK (public.is_super_admin());

-- =============================================
-- FIN DE LA MIGRATION EMAILING (SMTP PROVIDER CONFIG)
-- =============================================

