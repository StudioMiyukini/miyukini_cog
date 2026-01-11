-- =============================================
-- MIYUKINI FRAMEWORK - MIGRATION PAIEMENT (PAYPAL PROVIDER CONFIG)
-- Feature: Paiement / PayPal
-- Version: 1.0.0
-- Date: 2026-01-11
-- =============================================
--
-- Objectif :
-- - Stocker la configuration PayPal (sandbox/live, client_id/secret, webhook) pilotable depuis le Back-Office Super Admin
-- - Protéger par RLS (super_admin uniquement)
--
-- Notes sécurité :
-- - Ce stockage contient des secrets (client_secret). À terme, préférer Vault/Secrets.
-- - Les appels PayPal doivent se faire server-side (Edge Function) avec logs/audit.
--
-- Dépendances :
-- - Miyukini Framework - Migration Initiale.sql (helpers RLS, handle_updated_at, is_super_admin)
-- =============================================

-- ---------- ENUMS ----------
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'paypal_environment') THEN
    CREATE TYPE public.paypal_environment AS ENUM ('sandbox','live');
  END IF;
END$$;

-- ---------- TABLE ----------
CREATE TABLE IF NOT EXISTS public.paypal_provider_configs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  workspace_id UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000',
  is_active BOOLEAN NOT NULL DEFAULT false,
  environment public.paypal_environment NOT NULL DEFAULT 'sandbox',

  client_id TEXT NOT NULL,
  client_secret TEXT NULL,

  merchant_id TEXT NULL,
  webhook_id TEXT NULL,

  brand_name TEXT NULL,
  return_url TEXT NULL,
  cancel_url TEXT NULL,

  last_test_at TIMESTAMPTZ NULL,
  last_test_status TEXT NULL, -- ok|error
  last_test_error TEXT NULL,

  updated_by UUID NULL REFERENCES auth.users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_paypal_provider_configs_workspace
  ON public.paypal_provider_configs(workspace_id);

-- ---------- updated_at trigger ----------
DROP TRIGGER IF EXISTS paypal_provider_configs_updated_at ON public.paypal_provider_configs;
CREATE TRIGGER paypal_provider_configs_updated_at
  BEFORE UPDATE ON public.paypal_provider_configs
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

-- ---------- RLS ----------
ALTER TABLE public.paypal_provider_configs ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS paypal_provider_configs_super_admin ON public.paypal_provider_configs;
CREATE POLICY paypal_provider_configs_super_admin ON public.paypal_provider_configs
  FOR ALL
  USING (public.is_super_admin())
  WITH CHECK (public.is_super_admin());

-- =============================================
-- FIN DE LA MIGRATION PAIEMENT (PAYPAL PROVIDER CONFIG)
-- =============================================

