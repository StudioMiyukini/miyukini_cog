-- =============================================
-- MIYUKINI FRAMEWORK - MIGRATION COMMERCE PAR DEVIS
-- Module: Commerce par Devis
-- Version: 1.0.0
-- Date: 2026-01-11
-- =============================================
--
-- Objectif :
-- - Cycle demande -> devis (révisions) -> facture -> confirmation paiement (manuelle)
-- - Outbox d’événements commerce.* (pour Emailing/Documents/Budget server-side)
-- - Intégration Budget (recette à paiement confirmé) via insert idempotent (dedupe_key)
--
-- Dépendances :
-- - Miyukini Framework - Migration Initiale.sql (helpers RLS, handle_updated_at)
-- - Miyukini Framework - Migration Budget.sql (budget_entries + budget_categories)
-- =============================================

-- ---------- ENUMS (compat Postgres sans IF NOT EXISTS) ----------
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'commerce_quote_request_status') THEN
    CREATE TYPE public.commerce_quote_request_status AS ENUM ('draft','submitted','qualified','cancelled');
  END IF;
END$$;

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'commerce_quote_status') THEN
    CREATE TYPE public.commerce_quote_status AS ENUM ('draft','sent','viewed','accepted','rejected','expired','cancelled');
  END IF;
END$$;

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'commerce_invoice_status') THEN
    CREATE TYPE public.commerce_invoice_status AS ENUM ('draft','issued','paid_confirmed','cancelled','credited');
  END IF;
END$$;

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'commerce_payment_method') THEN
    CREATE TYPE public.commerce_payment_method AS ENUM ('bank_transfer','cash','check','card_remote','other');
  END IF;
END$$;

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'commerce_quote_message_author_role') THEN
    CREATE TYPE public.commerce_quote_message_author_role AS ENUM ('requester','seller','admin');
  END IF;
END$$;

-- ---------- Tables ----------
CREATE TABLE IF NOT EXISTS public.commerce_quote_requests (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  workspace_id UUID NULL,
  requested_by UUID NULL REFERENCES public.profiles(id) ON DELETE SET NULL,
  assigned_to UUID NULL REFERENCES public.profiles(id) ON DELETE SET NULL,
  status public.commerce_quote_request_status NOT NULL DEFAULT 'draft',
  title TEXT NOT NULL,
  description TEXT NULL,
  tags TEXT[] NULL,
  category TEXT NULL,
  contact_email TEXT NULL,
  contact_phone TEXT NULL,
  preferred_contact_channel TEXT NULL,
  meta JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_commerce_quote_requests_requested_by ON public.commerce_quote_requests(requested_by);
CREATE INDEX IF NOT EXISTS idx_commerce_quote_requests_status ON public.commerce_quote_requests(status);

CREATE TABLE IF NOT EXISTS public.commerce_quotes (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  workspace_id UUID NULL,
  quote_request_id UUID NOT NULL REFERENCES public.commerce_quote_requests(id) ON DELETE CASCADE,
  requester_id UUID NULL REFERENCES public.profiles(id) ON DELETE SET NULL,
  assigned_to UUID NULL REFERENCES public.profiles(id) ON DELETE SET NULL,
  quote_number TEXT NOT NULL,
  status public.commerce_quote_status NOT NULL DEFAULT 'draft',
  revision INT NOT NULL DEFAULT 1,
  valid_until DATE NULL,
  currency TEXT NOT NULL DEFAULT 'EUR',
  subtotal_net NUMERIC NULL,
  tax_amount NUMERIC NULL,
  total_gross NUMERIC NULL,
  terms TEXT NULL,
  internal_notes TEXT NULL,
  sent_at TIMESTAMPTZ NULL,
  accepted_at TIMESTAMPTZ NULL,
  rejected_at TIMESTAMPTZ NULL,
  created_by UUID NULL REFERENCES public.profiles(id) ON DELETE SET NULL,
  updated_by UUID NULL REFERENCES public.profiles(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_commerce_quotes_number ON public.commerce_quotes(quote_number);
CREATE INDEX IF NOT EXISTS idx_commerce_quotes_requester ON public.commerce_quotes(requester_id);
CREATE INDEX IF NOT EXISTS idx_commerce_quotes_status ON public.commerce_quotes(status);

CREATE TABLE IF NOT EXISTS public.commerce_quote_items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  quote_id UUID NOT NULL REFERENCES public.commerce_quotes(id) ON DELETE CASCADE,
  label TEXT NOT NULL,
  description TEXT NULL,
  quantity NUMERIC NOT NULL DEFAULT 1,
  unit_price_net NUMERIC NOT NULL DEFAULT 0,
  tax_rate NUMERIC NOT NULL DEFAULT 0,
  line_total_net NUMERIC NOT NULL DEFAULT 0,
  position INT NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_commerce_quote_items_quote ON public.commerce_quote_items(quote_id);

CREATE TABLE IF NOT EXISTS public.commerce_quote_messages (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  quote_id UUID NOT NULL REFERENCES public.commerce_quotes(id) ON DELETE CASCADE,
  author_id UUID NULL REFERENCES public.profiles(id) ON DELETE SET NULL,
  author_role public.commerce_quote_message_author_role NOT NULL,
  message TEXT NOT NULL,
  is_internal BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_commerce_quote_messages_quote ON public.commerce_quote_messages(quote_id);

CREATE TABLE IF NOT EXISTS public.commerce_quote_audit (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  quote_id UUID NOT NULL REFERENCES public.commerce_quotes(id) ON DELETE CASCADE,
  event_type TEXT NOT NULL,
  actor_id UUID NULL REFERENCES public.profiles(id) ON DELETE SET NULL,
  payload JSONB NOT NULL DEFAULT '{}'::jsonb,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_commerce_quote_audit_quote ON public.commerce_quote_audit(quote_id);

CREATE TABLE IF NOT EXISTS public.commerce_quote_invoices (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  quote_id UUID NOT NULL REFERENCES public.commerce_quotes(id) ON DELETE CASCADE,
  requester_id UUID NULL REFERENCES public.profiles(id) ON DELETE SET NULL,
  invoice_number TEXT NOT NULL,
  status public.commerce_invoice_status NOT NULL DEFAULT 'draft',
  issued_at TIMESTAMPTZ NULL,
  due_date DATE NULL,
  currency TEXT NOT NULL DEFAULT 'EUR',
  subtotal_net NUMERIC NULL,
  tax_amount NUMERIC NULL,
  total_gross NUMERIC NULL,
  created_by UUID NULL REFERENCES public.profiles(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_commerce_invoices_number ON public.commerce_quote_invoices(invoice_number);
CREATE INDEX IF NOT EXISTS idx_commerce_invoices_quote ON public.commerce_quote_invoices(quote_id);

CREATE TABLE IF NOT EXISTS public.commerce_invoice_payments (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  invoice_id UUID NOT NULL REFERENCES public.commerce_quote_invoices(id) ON DELETE CASCADE,
  method public.commerce_payment_method NOT NULL,
  reference TEXT NULL,
  amount NUMERIC NOT NULL,
  paid_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  confirmed_by UUID NULL REFERENCES public.profiles(id) ON DELETE SET NULL,
  proof_document_ref JSONB NULL,
  notes TEXT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_commerce_invoice_payments_invoice ON public.commerce_invoice_payments(invoice_id);

-- Outbox d’événements (pour exécution server-side Emailing/Documents/Budget)
CREATE TABLE IF NOT EXISTS public.commerce_outbox (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  event_type TEXT NOT NULL,
  dedupe_key TEXT NOT NULL,
  payload JSONB NOT NULL DEFAULT '{}'::jsonb,
  status TEXT NOT NULL DEFAULT 'pending', -- pending|processing|success|error
  attempt_count INT NOT NULL DEFAULT 0,
  next_retry_at TIMESTAMPTZ NULL,
  last_error TEXT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_commerce_outbox_dedupe ON public.commerce_outbox(dedupe_key);
CREATE INDEX IF NOT EXISTS idx_commerce_outbox_status ON public.commerce_outbox(status);

-- ---------- updated_at triggers ----------
DROP TRIGGER IF EXISTS commerce_quote_requests_updated_at ON public.commerce_quote_requests;
CREATE TRIGGER commerce_quote_requests_updated_at
  BEFORE UPDATE ON public.commerce_quote_requests
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

DROP TRIGGER IF EXISTS commerce_quotes_updated_at ON public.commerce_quotes;
CREATE TRIGGER commerce_quotes_updated_at
  BEFORE UPDATE ON public.commerce_quotes
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

DROP TRIGGER IF EXISTS commerce_quote_items_updated_at ON public.commerce_quote_items;
CREATE TRIGGER commerce_quote_items_updated_at
  BEFORE UPDATE ON public.commerce_quote_items
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

DROP TRIGGER IF EXISTS commerce_quote_invoices_updated_at ON public.commerce_quote_invoices;
CREATE TRIGGER commerce_quote_invoices_updated_at
  BEFORE UPDATE ON public.commerce_quote_invoices
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

DROP TRIGGER IF EXISTS commerce_outbox_updated_at ON public.commerce_outbox;
CREATE TRIGGER commerce_outbox_updated_at
  BEFORE UPDATE ON public.commerce_outbox
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

-- ---------- Sequences + helper functions ----------
CREATE SEQUENCE IF NOT EXISTS public.commerce_quote_number_seq;
CREATE SEQUENCE IF NOT EXISTS public.commerce_invoice_number_seq;

CREATE OR REPLACE FUNCTION public.commerce_next_quote_number()
RETURNS TEXT
LANGUAGE SQL
AS $$
  SELECT 'DEV-' || to_char(now(), 'YYYY') || '-' || lpad(nextval('public.commerce_quote_number_seq')::text, 5, '0');
$$;

CREATE OR REPLACE FUNCTION public.commerce_next_invoice_number()
RETURNS TEXT
LANGUAGE SQL
AS $$
  SELECT 'FAC-' || to_char(now(), 'YYYY') || '-' || lpad(nextval('public.commerce_invoice_number_seq')::text, 5, '0');
$$;

-- ---------- RPCs (transitions contrôlées + idempotence outbox) ----------
CREATE OR REPLACE FUNCTION public.commerce_create_quote_from_request(p_request_id UUID)
RETURNS UUID
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
  v_req public.commerce_quote_requests;
  v_quote_id UUID;
  v_quote_number TEXT;
BEGIN
  SELECT * INTO v_req FROM public.commerce_quote_requests WHERE id = p_request_id;
  IF NOT FOUND THEN
    RAISE EXCEPTION 'request_not_found';
  END IF;

  IF NOT public.is_admin_user() THEN
    RAISE EXCEPTION 'forbidden';
  END IF;

  v_quote_number := public.commerce_next_quote_number();

  INSERT INTO public.commerce_quotes(
    quote_request_id, requester_id, assigned_to, quote_number, status, revision, created_by, updated_by
  ) VALUES (
    v_req.id, v_req.requested_by, v_req.assigned_to, v_quote_number, 'draft', 1, auth.uid(), auth.uid()
  )
  RETURNING id INTO v_quote_id;

  INSERT INTO public.commerce_quote_items(quote_id, label, description, quantity, unit_price_net, tax_rate, line_total_net, position)
  VALUES (v_quote_id, 'Prestation', NULL, 1, 0, 0, 0, 1);

  INSERT INTO public.commerce_quote_audit(quote_id, event_type, actor_id, payload)
  VALUES (v_quote_id, 'created', auth.uid(), jsonb_build_object('request_id', v_req.id));

  INSERT INTO public.commerce_outbox(event_type, dedupe_key, payload)
  VALUES (
    'commerce.quote.created',
    'commerce.quote.created:' || v_quote_id::text,
    jsonb_build_object('quote_id', v_quote_id, 'request_id', v_req.id, 'quote_number', v_quote_number)
  )
  ON CONFLICT (dedupe_key) DO NOTHING;

  RETURN v_quote_id;
END;
$$;

GRANT EXECUTE ON FUNCTION public.commerce_create_quote_from_request(UUID) TO authenticated;

CREATE OR REPLACE FUNCTION public.commerce_send_quote(p_quote_id UUID)
RETURNS VOID
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
  v_quote public.commerce_quotes;
BEGIN
  IF NOT public.is_admin_user() THEN
    RAISE EXCEPTION 'forbidden';
  END IF;
  SELECT * INTO v_quote FROM public.commerce_quotes WHERE id = p_quote_id;
  IF NOT FOUND THEN
    RAISE EXCEPTION 'quote_not_found';
  END IF;

  UPDATE public.commerce_quotes
    SET status = 'sent', sent_at = now(), updated_by = auth.uid()
    WHERE id = p_quote_id AND status = 'draft';

  INSERT INTO public.commerce_quote_audit(quote_id, event_type, actor_id, payload)
  VALUES (p_quote_id, 'sent', auth.uid(), '{}'::jsonb);

  INSERT INTO public.commerce_outbox(event_type, dedupe_key, payload)
  VALUES (
    'commerce.quote.sent',
    'commerce.quote.sent:' || p_quote_id::text,
    jsonb_build_object('quote_id', p_quote_id, 'quote_number', v_quote.quote_number, 'requester_id', v_quote.requester_id)
  )
  ON CONFLICT (dedupe_key) DO NOTHING;
END;
$$;

GRANT EXECUTE ON FUNCTION public.commerce_send_quote(UUID) TO authenticated;

CREATE OR REPLACE FUNCTION public.commerce_accept_quote(p_quote_id UUID)
RETURNS VOID
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
  v_quote public.commerce_quotes;
BEGIN
  SELECT * INTO v_quote FROM public.commerce_quotes WHERE id = p_quote_id;
  IF NOT FOUND THEN
    RAISE EXCEPTION 'quote_not_found';
  END IF;

  IF auth.uid() IS NULL OR v_quote.requester_id IS NULL OR auth.uid() <> v_quote.requester_id THEN
    RAISE EXCEPTION 'forbidden';
  END IF;

  UPDATE public.commerce_quotes
    SET status='accepted', accepted_at=now(), updated_by=auth.uid()
    WHERE id=p_quote_id AND status IN ('sent','viewed');

  INSERT INTO public.commerce_quote_audit(quote_id, event_type, actor_id, payload)
  VALUES (p_quote_id, 'accepted', auth.uid(), '{}'::jsonb);

  INSERT INTO public.commerce_outbox(event_type, dedupe_key, payload)
  VALUES (
    'commerce.quote.accepted',
    'commerce.quote.accepted:' || p_quote_id::text,
    jsonb_build_object('quote_id', p_quote_id, 'quote_number', v_quote.quote_number, 'requester_id', v_quote.requester_id)
  )
  ON CONFLICT (dedupe_key) DO NOTHING;
END;
$$;

GRANT EXECUTE ON FUNCTION public.commerce_accept_quote(UUID) TO authenticated;

CREATE OR REPLACE FUNCTION public.commerce_reject_quote(p_quote_id UUID, p_reason TEXT DEFAULT NULL)
RETURNS VOID
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
  v_quote public.commerce_quotes;
BEGIN
  SELECT * INTO v_quote FROM public.commerce_quotes WHERE id = p_quote_id;
  IF NOT FOUND THEN
    RAISE EXCEPTION 'quote_not_found';
  END IF;

  IF auth.uid() IS NULL OR v_quote.requester_id IS NULL OR auth.uid() <> v_quote.requester_id THEN
    RAISE EXCEPTION 'forbidden';
  END IF;

  UPDATE public.commerce_quotes
    SET status='rejected', rejected_at=now(), updated_by=auth.uid()
    WHERE id=p_quote_id AND status IN ('sent','viewed');

  INSERT INTO public.commerce_quote_audit(quote_id, event_type, actor_id, payload)
  VALUES (p_quote_id, 'rejected', auth.uid(), jsonb_build_object('reason', p_reason));

  INSERT INTO public.commerce_outbox(event_type, dedupe_key, payload)
  VALUES (
    'commerce.quote.rejected',
    'commerce.quote.rejected:' || p_quote_id::text,
    jsonb_build_object('quote_id', p_quote_id, 'quote_number', v_quote.quote_number, 'requester_id', v_quote.requester_id, 'reason', p_reason)
  )
  ON CONFLICT (dedupe_key) DO NOTHING;
END;
$$;

GRANT EXECUTE ON FUNCTION public.commerce_reject_quote(UUID, TEXT) TO authenticated;

CREATE OR REPLACE FUNCTION public.commerce_create_invoice_from_quote(p_quote_id UUID)
RETURNS UUID
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
  v_quote public.commerce_quotes;
  v_invoice_id UUID;
  v_invoice_number TEXT;
BEGIN
  IF NOT public.is_admin_user() THEN
    RAISE EXCEPTION 'forbidden';
  END IF;

  SELECT * INTO v_quote FROM public.commerce_quotes WHERE id = p_quote_id;
  IF NOT FOUND THEN
    RAISE EXCEPTION 'quote_not_found';
  END IF;

  IF v_quote.status <> 'accepted' THEN
    RAISE EXCEPTION 'quote_not_accepted';
  END IF;

  v_invoice_number := public.commerce_next_invoice_number();

  INSERT INTO public.commerce_quote_invoices(
    quote_id, requester_id, invoice_number, status, issued_at, currency, subtotal_net, tax_amount, total_gross, created_by
  ) VALUES (
    v_quote.id, v_quote.requester_id, v_invoice_number, 'issued', now(), v_quote.currency, v_quote.subtotal_net, v_quote.tax_amount, v_quote.total_gross, auth.uid()
  )
  RETURNING id INTO v_invoice_id;

  INSERT INTO public.commerce_outbox(event_type, dedupe_key, payload)
  VALUES (
    'commerce.invoice.issued',
    'commerce.invoice.issued:' || v_invoice_id::text,
    jsonb_build_object('invoice_id', v_invoice_id, 'invoice_number', v_invoice_number, 'quote_id', v_quote.id, 'quote_number', v_quote.quote_number, 'requester_id', v_quote.requester_id)
  )
  ON CONFLICT (dedupe_key) DO NOTHING;

  RETURN v_invoice_id;
END;
$$;

GRANT EXECUTE ON FUNCTION public.commerce_create_invoice_from_quote(UUID) TO authenticated;

CREATE OR REPLACE FUNCTION public.commerce_confirm_payment(
  p_invoice_id UUID,
  p_method public.commerce_payment_method,
  p_reference TEXT DEFAULT NULL,
  p_amount NUMERIC DEFAULT NULL,
  p_notes TEXT DEFAULT NULL
)
RETURNS UUID
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
  v_invoice public.commerce_quote_invoices;
  v_payment_id UUID;
  v_amount NUMERIC;
  v_workspace UUID := '00000000-0000-0000-0000-000000000000';
  v_category_id UUID;
BEGIN
  IF NOT public.is_admin_user() THEN
    RAISE EXCEPTION 'forbidden';
  END IF;

  SELECT * INTO v_invoice FROM public.commerce_quote_invoices WHERE id = p_invoice_id;
  IF NOT FOUND THEN
    RAISE EXCEPTION 'invoice_not_found';
  END IF;

  v_amount := COALESCE(p_amount, v_invoice.total_gross, 0);

  INSERT INTO public.commerce_invoice_payments(invoice_id, method, reference, amount, confirmed_by, notes)
  VALUES (p_invoice_id, p_method, p_reference, v_amount, auth.uid(), p_notes)
  RETURNING id INTO v_payment_id;

  UPDATE public.commerce_quote_invoices
    SET status='paid_confirmed', updated_at=now()
    WHERE id=p_invoice_id;

  -- Outbox event
  INSERT INTO public.commerce_outbox(event_type, dedupe_key, payload)
  VALUES (
    'commerce.payment.confirmed',
    'commerce.payment.confirmed:' || v_payment_id::text,
    jsonb_build_object('payment_id', v_payment_id, 'invoice_id', p_invoice_id, 'amount', v_amount, 'method', p_method::text, 'reference', p_reference, 'requester_id', v_invoice.requester_id, 'confirmed_at', now())
  )
  ON CONFLICT (dedupe_key) DO NOTHING;

  -- Budget integration (income). Idempotent via budget_entries.dedupe_key UNIQUE (standard Budget)
  SELECT id INTO v_category_id
    FROM public.budget_categories
    WHERE workspace_id = v_workspace AND (name ILIKE 'Prestations' OR name ILIKE 'Ventes')
    ORDER BY (name ILIKE 'Prestations') DESC
    LIMIT 1;

  INSERT INTO public.budget_entries(
    workspace_id, module_id, context_id, type, status, entry_date,
    description, amount_net, amount_tax, amount_gross, currency,
    category_id, tags, notes, invoice_id, dedupe_key, created_by, updated_by
  ) VALUES (
    v_workspace,
    'commerce-devis',
    v_invoice.quote_id::text,
    'income',
    'validated',
    (now() AT TIME ZONE 'Europe/Paris')::date,
    'Paiement confirmé ' || v_invoice.invoice_number,
    v_amount,
    0,
    v_amount,
    COALESCE(v_invoice.currency, 'EUR'),
    v_category_id,
    ARRAY['commerce-devis', v_invoice.invoice_number],
    p_notes,
    NULL,
    'commerce.payment.confirmed:' || v_payment_id::text,
    auth.uid(),
    auth.uid()
  )
  ON CONFLICT (dedupe_key) DO NOTHING;

  RETURN v_payment_id;
END;
$$;

GRANT EXECUTE ON FUNCTION public.commerce_confirm_payment(UUID, public.commerce_payment_method, TEXT, NUMERIC, TEXT) TO authenticated;

-- ---------- RLS ----------
ALTER TABLE public.commerce_quote_requests ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.commerce_quotes ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.commerce_quote_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.commerce_quote_messages ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.commerce_quote_audit ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.commerce_quote_invoices ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.commerce_invoice_payments ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.commerce_outbox ENABLE ROW LEVEL SECURITY;

-- quote_requests: public insert allowed (requested_by NULL + contact_email required) OR authenticated insert as self
DROP POLICY IF EXISTS commerce_qr_select ON public.commerce_quote_requests;
CREATE POLICY commerce_qr_select ON public.commerce_quote_requests
  FOR SELECT
  USING (public.is_admin_user() OR (requested_by IS NOT NULL AND auth.uid() = requested_by));

DROP POLICY IF EXISTS commerce_qr_insert ON public.commerce_quote_requests;
CREATE POLICY commerce_qr_insert ON public.commerce_quote_requests
  FOR INSERT
  WITH CHECK (
    public.is_admin_user()
    OR (requested_by IS NOT NULL AND auth.uid() = requested_by)
    OR (requested_by IS NULL AND contact_email IS NOT NULL AND length(contact_email) > 3)
  );

DROP POLICY IF EXISTS commerce_qr_update ON public.commerce_quote_requests;
CREATE POLICY commerce_qr_update ON public.commerce_quote_requests
  FOR UPDATE
  USING (
    public.is_admin_user()
    OR (requested_by IS NOT NULL AND auth.uid() = requested_by AND status IN ('draft','submitted'))
  )
  WITH CHECK (
    public.is_admin_user()
    OR (requested_by IS NOT NULL AND auth.uid() = requested_by AND status IN ('draft','submitted'))
  );

-- quotes: requester can read own; admin all; writes via RPC (admin)
DROP POLICY IF EXISTS commerce_quotes_select ON public.commerce_quotes;
CREATE POLICY commerce_quotes_select ON public.commerce_quotes
  FOR SELECT
  USING (public.is_admin_user() OR (requester_id IS NOT NULL AND auth.uid() = requester_id));

DROP POLICY IF EXISTS commerce_quotes_write_admin ON public.commerce_quotes;
CREATE POLICY commerce_quotes_write_admin ON public.commerce_quotes
  FOR ALL
  USING (public.is_admin_user())
  WITH CHECK (public.is_admin_user());

-- quote_items/messages/audit: follow parent quote visibility, writes admin for items, messages allow requester for external messages
DROP POLICY IF EXISTS commerce_quote_items_select ON public.commerce_quote_items;
CREATE POLICY commerce_quote_items_select ON public.commerce_quote_items
  FOR SELECT
  USING (
    public.is_admin_user()
    OR EXISTS (
      SELECT 1 FROM public.commerce_quotes q
      WHERE q.id = commerce_quote_items.quote_id AND q.requester_id IS NOT NULL AND q.requester_id = auth.uid()
    )
  );

DROP POLICY IF EXISTS commerce_quote_items_write_admin ON public.commerce_quote_items;
CREATE POLICY commerce_quote_items_write_admin ON public.commerce_quote_items
  FOR ALL
  USING (public.is_admin_user())
  WITH CHECK (public.is_admin_user());

DROP POLICY IF EXISTS commerce_quote_messages_select ON public.commerce_quote_messages;
CREATE POLICY commerce_quote_messages_select ON public.commerce_quote_messages
  FOR SELECT
  USING (
    public.is_admin_user()
    OR EXISTS (
      SELECT 1 FROM public.commerce_quotes q
      WHERE q.id = commerce_quote_messages.quote_id AND q.requester_id IS NOT NULL AND q.requester_id = auth.uid()
    )
  );

DROP POLICY IF EXISTS commerce_quote_messages_insert ON public.commerce_quote_messages;
CREATE POLICY commerce_quote_messages_insert ON public.commerce_quote_messages
  FOR INSERT
  WITH CHECK (
    public.is_admin_user()
    OR (
      author_id IS NOT NULL AND auth.uid() = author_id AND is_internal = false
      AND EXISTS (
        SELECT 1 FROM public.commerce_quotes q
        WHERE q.id = commerce_quote_messages.quote_id AND q.requester_id IS NOT NULL AND q.requester_id = auth.uid()
      )
    )
  );

DROP POLICY IF EXISTS commerce_quote_audit_select ON public.commerce_quote_audit;
CREATE POLICY commerce_quote_audit_select ON public.commerce_quote_audit
  FOR SELECT
  USING (public.is_admin_user());

-- invoices: requester can read own; writes admin
DROP POLICY IF EXISTS commerce_invoices_select ON public.commerce_quote_invoices;
CREATE POLICY commerce_invoices_select ON public.commerce_quote_invoices
  FOR SELECT
  USING (public.is_admin_user() OR (requester_id IS NOT NULL AND auth.uid() = requester_id));

DROP POLICY IF EXISTS commerce_invoices_write_admin ON public.commerce_quote_invoices;
CREATE POLICY commerce_invoices_write_admin ON public.commerce_quote_invoices
  FOR ALL
  USING (public.is_admin_user())
  WITH CHECK (public.is_admin_user());

DROP POLICY IF EXISTS commerce_payments_select ON public.commerce_invoice_payments;
CREATE POLICY commerce_payments_select ON public.commerce_invoice_payments
  FOR SELECT
  USING (public.is_admin_user());

DROP POLICY IF EXISTS commerce_payments_write_admin ON public.commerce_invoice_payments;
CREATE POLICY commerce_payments_write_admin ON public.commerce_invoice_payments
  FOR ALL
  USING (public.is_admin_user())
  WITH CHECK (public.is_admin_user());

-- outbox: admin only
DROP POLICY IF EXISTS commerce_outbox_admin ON public.commerce_outbox;
CREATE POLICY commerce_outbox_admin ON public.commerce_outbox
  FOR ALL
  USING (public.is_admin_user())
  WITH CHECK (public.is_admin_user());

-- ---------- Seed module activation row ----------
INSERT INTO public.framework_modules (module_id, enabled)
VALUES ('commerce-devis', false)
ON CONFLICT (module_id) DO NOTHING;

-- =============================================
-- FIN DE LA MIGRATION COMMERCE PAR DEVIS
-- =============================================

