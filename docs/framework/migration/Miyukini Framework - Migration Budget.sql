-- =============================================
-- MIYUKINI FRAMEWORK - MIGRATION BUDGET
-- Module: Budget
-- Version: 1.0.0
-- Date: 2026-01-11
-- =============================================
--
-- Objectif :
-- - Centraliser les entrées budgétaires, catégories et plans
-- - Fournir un RLS strict (admin / workspace)
-- - Offrir les hooks pour automatiser recettes/dépenses
--
-- Dépendances :
-- - Miyukini Framework - Migration Initiale.sql (helpers RLS, handle_updated_at)
-- =============================================

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'budget_category_type') THEN
    CREATE TYPE public.budget_category_type AS ENUM ('income', 'expense', 'both');
  END IF;
END
$$;

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'budget_entry_type') THEN
    CREATE TYPE public.budget_entry_type AS ENUM ('income', 'expense', 'refund', 'transfer');
  END IF;
END
$$;

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'budget_entry_status') THEN
    CREATE TYPE public.budget_entry_status AS ENUM ('draft', 'validated', 'reconciled', 'archived');
  END IF;
END
$$;

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'budget_period_type') THEN
    CREATE TYPE public.budget_period_type AS ENUM ('month', 'quarter', 'year');
  END IF;
END
$$;

CREATE TABLE IF NOT EXISTS public.budget_categories (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  workspace_id UUID NOT NULL,
  name TEXT NOT NULL,
  type public.budget_category_type NOT NULL,
  parent_id UUID REFERENCES public.budget_categories(id) ON DELETE SET NULL,
  icon TEXT,
  color TEXT,
  is_active BOOLEAN NOT NULL DEFAULT true,
  account_code TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_budget_categories_workspace ON public.budget_categories(workspace_id);

CREATE TABLE IF NOT EXISTS public.budget_entries (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  workspace_id UUID NOT NULL,
  module_id TEXT NOT NULL,
  context_id TEXT,
  type public.budget_entry_type NOT NULL,
  status public.budget_entry_status NOT NULL DEFAULT 'draft',
  entry_date DATE NOT NULL,
  description TEXT,
  amount_net NUMERIC NOT NULL,
  amount_tax NUMERIC DEFAULT 0,
  amount_gross NUMERIC NOT NULL,
  currency TEXT NOT NULL DEFAULT 'EUR',
  fx_rate NUMERIC DEFAULT 1,
  category_id UUID REFERENCES public.budget_categories(id) ON DELETE SET NULL,
  cost_center_id UUID,
  tags TEXT[],
  notes TEXT,
  receipt_path TEXT,
  invoice_id UUID,
  payment_id UUID,
  dedupe_key TEXT UNIQUE,
  created_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
  updated_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_budget_entries_workspace ON public.budget_entries(workspace_id);
CREATE INDEX IF NOT EXISTS idx_budget_entries_module ON public.budget_entries(module_id);
CREATE INDEX IF NOT EXISTS idx_budget_entries_category ON public.budget_entries(category_id);

CREATE TABLE IF NOT EXISTS public.budget_plans (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  workspace_id UUID NOT NULL,
  module_id TEXT,
  context_id TEXT,
  period_type public.budget_period_type NOT NULL,
  start_date DATE NOT NULL,
  end_date DATE NOT NULL,
  currency TEXT NOT NULL DEFAULT 'EUR',
  created_by UUID REFERENCES auth.users(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS public.budget_plan_lines (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  plan_id UUID NOT NULL REFERENCES public.budget_plans(id) ON DELETE CASCADE,
  category_id UUID REFERENCES public.budget_categories(id) ON DELETE SET NULL,
  period_start DATE NOT NULL,
  amount_planned NUMERIC NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS public.budget_entry_audit (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  entry_id UUID REFERENCES public.budget_entries(id) ON DELETE CASCADE,
  action TEXT NOT NULL,
  actor_id UUID REFERENCES auth.users(id) ON DELETE SET NULL,
  payload JSONB,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Triggers updated_at
CREATE OR REPLACE FUNCTION public.handle_updated_at()
RETURNS trigger AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS budget_categories_updated_at ON public.budget_categories;
CREATE TRIGGER budget_categories_updated_at
  BEFORE UPDATE ON public.budget_categories
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

DROP TRIGGER IF EXISTS budget_entries_updated_at ON public.budget_entries;
CREATE TRIGGER budget_entries_updated_at
  BEFORE UPDATE ON public.budget_entries
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

DROP TRIGGER IF EXISTS budget_plans_updated_at ON public.budget_plans;
CREATE TRIGGER budget_plans_updated_at
  BEFORE UPDATE ON public.budget_plans
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

DROP TRIGGER IF EXISTS budget_plan_lines_updated_at ON public.budget_plan_lines;
CREATE TRIGGER budget_plan_lines_updated_at
  BEFORE UPDATE ON public.budget_plan_lines
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

-- RLS
ALTER TABLE public.budget_categories ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.budget_entries ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.budget_plans ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.budget_plan_lines ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.budget_entry_audit ENABLE ROW LEVEL SECURITY;

CREATE POLICY budget_categories_admin ON public.budget_categories
  FOR ALL
  USING (public.is_admin_user())
  WITH CHECK (public.is_admin_user());

CREATE POLICY budget_entries_admin ON public.budget_entries
  FOR ALL
  USING (
    public.is_admin_user()
    OR workspace_id = (auth.jwt() ->> 'workspace_id')::uuid
  )
  WITH CHECK (
    public.is_admin_user()
    OR workspace_id = (auth.jwt() ->> 'workspace_id')::uuid
  );

CREATE POLICY budget_plans_admin ON public.budget_plans
  FOR ALL
  USING (public.is_admin_user())
  WITH CHECK (public.is_admin_user());

CREATE POLICY budget_plan_lines_admin ON public.budget_plan_lines
  FOR ALL
  USING (public.is_admin_user())
  WITH CHECK (public.is_admin_user());

CREATE POLICY budget_entry_audit_admin ON public.budget_entry_audit
  FOR SELECT
  USING (public.is_admin_user());

-- Seed categories
INSERT INTO public.budget_categories (workspace_id, name, type, icon, color, account_code)
VALUES
  ('00000000-0000-0000-0000-000000000000', 'Ventes', 'income', 'icon-[tabler--wallet]', '#2f80ed', '70'),
  ('00000000-0000-0000-0000-000000000000', 'Prestations', 'income', 'icon-[tabler--sparkles]', '#219653', '71'),
  ('00000000-0000-0000-0000-000000000000', 'Dépenses opérationnelles', 'expense', 'icon-[tabler--eraser]', '#eb5757', '60'),
  ('00000000-0000-0000-0000-000000000000', 'Marketing', 'expense', 'icon-[tabler--bullhorn]', '#f2994a', '61')
ON CONFLICT DO NOTHING;

-- =============================================
-- FIN DE LA MIGRATION BUDGET
-- =============================================

