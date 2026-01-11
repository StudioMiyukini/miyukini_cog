-- =============================================
-- MIYUKINI FRAMEWORK - MIGRATION PAIEMENT (PAYPAL MODULE CONFIG)
-- Feature: Paiement / PayPal (configuration par module)
-- Version: 1.0.0
-- Date: 2026-01-11
-- =============================================
--
-- Objectif :
-- - Ajouter une configuration PayPal PAR MODULE (activation + mode + intent + urls + options)
-- - Permettre “toutes les possibilités” via un socle générique + `settings_json`
-- - Protéger par RLS (super_admin uniquement)
--
-- Dépendances :
-- - Migration Initiale (helpers RLS)
-- - Migration Paiement PayPal Provider (paypal_provider_configs)
-- =============================================

-- ---------- ENUMS ----------
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'paypal_payment_mode') THEN
    CREATE TYPE public.paypal_payment_mode AS ENUM ('orders','subscriptions');
  END IF;
END$$;

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'paypal_intent') THEN
    CREATE TYPE public.paypal_intent AS ENUM ('CAPTURE','AUTHORIZE');
  END IF;
END$$;

-- ---------- TABLE ----------
CREATE TABLE IF NOT EXISTS public.module_paypal_configs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  workspace_id UUID NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000',
  module_id TEXT NOT NULL,
  is_enabled BOOLEAN NOT NULL DEFAULT false,

  -- mode = paiement one-shot (orders) ou abonnements (subscriptions)
  mode public.paypal_payment_mode NOT NULL DEFAULT 'orders',

  -- intent PayPal Orders v2
  intent public.paypal_intent NOT NULL DEFAULT 'CAPTURE',

  currency TEXT NOT NULL DEFAULT 'EUR',
  success_url TEXT NULL,
  cancel_url TEXT NULL,

  -- toggles "toutes les possibilités"
  enable_refunds BOOLEAN NOT NULL DEFAULT true,
  enable_webhooks BOOLEAN NOT NULL DEFAULT false,
  enable_disputes BOOLEAN NOT NULL DEFAULT false,
  enable_payouts BOOLEAN NOT NULL DEFAULT false,

  -- extensible (ex: brand_name override, shipping prefs, landing_page, subscription plan ids, etc.)
  settings_json JSONB NOT NULL DEFAULT '{}'::jsonb,

  updated_by UUID NULL REFERENCES auth.users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_module_paypal_configs_workspace_module
  ON public.module_paypal_configs(workspace_id, module_id);

-- ---------- updated_at trigger ----------
DROP TRIGGER IF EXISTS module_paypal_configs_updated_at ON public.module_paypal_configs;
CREATE TRIGGER module_paypal_configs_updated_at
  BEFORE UPDATE ON public.module_paypal_configs
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

-- ---------- RLS ----------
ALTER TABLE public.module_paypal_configs ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS module_paypal_configs_super_admin ON public.module_paypal_configs;
CREATE POLICY module_paypal_configs_super_admin ON public.module_paypal_configs
  FOR ALL
  USING (public.is_super_admin())
  WITH CHECK (public.is_super_admin());

-- =============================================
-- FIN DE LA MIGRATION PAIEMENT (PAYPAL MODULE CONFIG)
-- =============================================

