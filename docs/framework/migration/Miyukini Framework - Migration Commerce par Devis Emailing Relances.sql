-- =============================================
-- MIYUKINI FRAMEWORK - MIGRATION COMMERCE PAR DEVIS (EMAILING RELANCES)
-- Module: Commerce par Devis
-- Version: 1.0.0
-- Date: 2026-01-11
-- =============================================
--
-- Objectif :
-- - Ajouter une table de relances (scheduler) pour produire l’événement `commerce.quote.reminder.due`
-- - Garantir l’idempotence via `dedupe_key` UNIQUE
-- - Préparer l’exécution server-side (Edge Function cron / worker) avec index `status + run_at`
--
-- Dépendances :
-- - Miyukini Framework - Migration Initiale.sql (helpers RLS, handle_updated_at)
-- - Miyukini Framework - Migration Commerce par Devis.sql (tables commerce_quotes)
-- =============================================

-- ---------- ENUMS (compat Postgres sans IF NOT EXISTS) ----------
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'commerce_quote_reminder_status') THEN
    CREATE TYPE public.commerce_quote_reminder_status AS ENUM ('scheduled','sent','cancelled','error');
  END IF;
END$$;

-- ---------- TABLE ----------
CREATE TABLE IF NOT EXISTS public.commerce_quote_reminders (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  quote_id UUID NOT NULL REFERENCES public.commerce_quotes(id) ON DELETE CASCADE,
  recipient_email TEXT NOT NULL,
  sequence INT NOT NULL DEFAULT 1,
  run_at TIMESTAMPTZ NOT NULL,
  status public.commerce_quote_reminder_status NOT NULL DEFAULT 'scheduled',
  sent_at TIMESTAMPTZ NULL,
  attempt_count INT NOT NULL DEFAULT 0,
  next_retry_at TIMESTAMPTZ NULL,
  last_error TEXT NULL,
  payload JSONB NOT NULL DEFAULT '{}'::jsonb,
  dedupe_key TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Contraintes / indexes
CREATE UNIQUE INDEX IF NOT EXISTS uq_commerce_quote_reminders_dedupe ON public.commerce_quote_reminders(dedupe_key);
CREATE UNIQUE INDEX IF NOT EXISTS uq_commerce_quote_reminders_quote_sequence ON public.commerce_quote_reminders(quote_id, sequence);
CREATE INDEX IF NOT EXISTS idx_commerce_quote_reminders_status_run_at ON public.commerce_quote_reminders(status, run_at);
CREATE INDEX IF NOT EXISTS idx_commerce_quote_reminders_quote_id ON public.commerce_quote_reminders(quote_id);

-- ---------- updated_at trigger ----------
DROP TRIGGER IF EXISTS commerce_quote_reminders_updated_at ON public.commerce_quote_reminders;
CREATE TRIGGER commerce_quote_reminders_updated_at
  BEFORE UPDATE ON public.commerce_quote_reminders
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

-- ---------- RLS ----------
ALTER TABLE public.commerce_quote_reminders ENABLE ROW LEVEL SECURITY;

-- Admin only (le worker utilisera le service_role, qui bypass RLS)
DROP POLICY IF EXISTS commerce_quote_reminders_admin ON public.commerce_quote_reminders;
CREATE POLICY commerce_quote_reminders_admin ON public.commerce_quote_reminders
  FOR ALL
  USING (public.is_admin_user())
  WITH CHECK (public.is_admin_user());

-- =============================================
-- FIN DE LA MIGRATION COMMERCE PAR DEVIS (EMAILING RELANCES)
-- =============================================

