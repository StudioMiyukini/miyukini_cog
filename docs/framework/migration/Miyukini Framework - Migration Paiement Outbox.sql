-- =============================================
-- MIYUKINI FRAMEWORK - MIGRATION PAIEMENT (OUTBOX)
-- Feature: Paiement (providers PayPal/Stripe/...)
-- Version: 1.0.0
-- Date: 2026-01-11
-- =============================================
--
-- Objectif :
-- - Disposer d’une outbox générique pour les événements Paiement (webhooks, captures, refunds)
-- - Idempotence via `dedupe_key` UNIQUE
--
-- Dépendances :
-- - Migration Initiale (handle_updated_at, helpers RLS)
-- =============================================

CREATE TABLE IF NOT EXISTS public.payment_outbox (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  provider TEXT NOT NULL, -- ex: paypal
  event_type TEXT NOT NULL, -- ex: payment.paypal.webhook.received
  dedupe_key TEXT NOT NULL,
  module_id TEXT NULL, -- si l’event est rattaché à un module
  payload JSONB NOT NULL DEFAULT '{}'::jsonb,
  status TEXT NOT NULL DEFAULT 'pending', -- pending|processing|success|error
  attempt_count INT NOT NULL DEFAULT 0,
  next_retry_at TIMESTAMPTZ NULL,
  last_error TEXT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_payment_outbox_dedupe ON public.payment_outbox(dedupe_key);
CREATE INDEX IF NOT EXISTS idx_payment_outbox_status ON public.payment_outbox(status);
CREATE INDEX IF NOT EXISTS idx_payment_outbox_module_id ON public.payment_outbox(module_id);

-- updated_at trigger
DROP TRIGGER IF EXISTS payment_outbox_updated_at ON public.payment_outbox;
CREATE TRIGGER payment_outbox_updated_at
  BEFORE UPDATE ON public.payment_outbox
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

-- RLS : admin/super_admin (écriture worker via service role)
ALTER TABLE public.payment_outbox ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS payment_outbox_admin ON public.payment_outbox;
CREATE POLICY payment_outbox_admin ON public.payment_outbox
  FOR ALL
  USING (public.is_admin_user())
  WITH CHECK (public.is_admin_user());

-- =============================================
-- FIN DE LA MIGRATION PAIEMENT (OUTBOX)
-- =============================================

