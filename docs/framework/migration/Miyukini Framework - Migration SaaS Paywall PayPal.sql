-- =============================================
-- MIYUKINI FRAMEWORK - MIGRATION SAAS PAYWALL PAYPAL
-- Module: SaaS Paywall PayPal
-- Version: 1.0.0
-- Date: 2026-01-15 (Europe/Paris)
-- =============================================
--
-- Objectif :
-- - Tables pour le module SaaS Paywall (plans, subscriptions, transactions, history)
-- - Gestion des abonnements récurrents PayPal pour les prestataires
-- - Mise à jour automatique user_tier selon abonnement actif
--
-- Dépendances :
-- - Migration Initiale (profiles, helpers RLS)
-- - Migration Paiement PayPal Provider (paypal_provider_configs)
-- =============================================

begin;

-- ---------- ENUMS ----------
do $$
begin
  if not exists (select 1 from pg_type where typname = 'saas_paywall_plan_status') then
    create type public.saas_paywall_plan_status as enum (
      'draft',
      'active',
      'archived'
    );
  end if;

  if not exists (select 1 from pg_type where typname = 'saas_paywall_subscription_status') then
    create type public.saas_paywall_subscription_status as enum (
      'APPROVAL_PENDING',
      'APPROVED',
      'ACTIVE',
      'SUSPENDED',
      'CANCELLED',
      'EXPIRED'
    );
  end if;

  if not exists (select 1 from pg_type where typname = 'saas_paywall_transaction_status') then
    create type public.saas_paywall_transaction_status as enum (
      'COMPLETED',
      'DECLINED',
      'PENDING'
    );
  end if;
end $$;

-- ---------- TABLE saas_paywall_plans ----------
create table if not exists public.saas_paywall_plans (
  id uuid primary key default gen_random_uuid(),
  name text not null,
  description text null,
  price_monthly numeric not null,
  price_yearly numeric null,
  currency text not null default 'EUR',
  billing_cycle text not null default 'monthly', -- monthly, yearly, both
  tier text not null, -- free, starter, pro, enterprise (référence profiles.tier)
  features jsonb not null default '{}'::jsonb,
  paypal_plan_id text unique null, -- ID plan PayPal créé via API
  status public.saas_paywall_plan_status not null default 'draft',
  sort_order integer not null default 0,
  is_featured boolean not null default false,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index if not exists idx_saas_paywall_plans_status on public.saas_paywall_plans(status);
create index if not exists idx_saas_paywall_plans_tier on public.saas_paywall_plans(tier);
create index if not exists idx_saas_paywall_plans_sort on public.saas_paywall_plans(sort_order);

comment on table public.saas_paywall_plans is 'Plans d''abonnement SaaS Paywall PayPal.';

-- ---------- TABLE saas_paywall_subscriptions ----------
create table if not exists public.saas_paywall_subscriptions (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references public.profiles(id) on delete cascade,
  plan_id uuid not null references public.saas_paywall_plans(id) on delete restrict,
  paypal_subscription_id text unique not null, -- ID subscription PayPal
  status public.saas_paywall_subscription_status not null default 'APPROVAL_PENDING',
  start_date timestamptz null,
  end_date timestamptz null, -- NULL si actif
  current_period_start timestamptz null,
  current_period_end timestamptz null,
  next_billing_date timestamptz null,
  cancel_at_period_end boolean not null default false,
  cancelled_at timestamptz null,
  suspended_at timestamptz null,
  suspension_reason text null,
  metadata jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create unique index if not exists uq_saas_paywall_subscriptions_user_active 
  on public.saas_paywall_subscriptions(user_id) 
  where status in ('ACTIVE', 'APPROVED', 'APPROVAL_PENDING');
create index if not exists idx_saas_paywall_subscriptions_user_id on public.saas_paywall_subscriptions(user_id);
create index if not exists idx_saas_paywall_subscriptions_plan_id on public.saas_paywall_subscriptions(plan_id);
create index if not exists idx_saas_paywall_subscriptions_status on public.saas_paywall_subscriptions(status);
create index if not exists idx_saas_paywall_subscriptions_paypal_id on public.saas_paywall_subscriptions(paypal_subscription_id);

comment on table public.saas_paywall_subscriptions is 'Abonnements actifs des utilisateurs au SaaS Paywall.';

-- ---------- TABLE saas_paywall_subscription_transactions ----------
create table if not exists public.saas_paywall_subscription_transactions (
  id uuid primary key default gen_random_uuid(),
  subscription_id uuid not null references public.saas_paywall_subscriptions(id) on delete cascade,
  paypal_transaction_id text unique not null, -- ID transaction PayPal
  billing_cycle integer not null, -- numéro du cycle (1, 2, 3...)
  status public.saas_paywall_transaction_status not null,
  amount numeric not null,
  currency text not null default 'EUR',
  transaction_time timestamptz not null,
  failure_reason text null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index if not exists idx_saas_paywall_transactions_subscription_id on public.saas_paywall_subscription_transactions(subscription_id);
create index if not exists idx_saas_paywall_transactions_status on public.saas_paywall_subscription_transactions(status);
create index if not exists idx_saas_paywall_transactions_paypal_id on public.saas_paywall_subscription_transactions(paypal_transaction_id);

comment on table public.saas_paywall_subscription_transactions is 'Transactions récurrentes des abonnements (chaque cycle de facturation).';

-- ---------- TABLE saas_paywall_subscription_history ----------
create table if not exists public.saas_paywall_subscription_history (
  id uuid primary key default gen_random_uuid(),
  subscription_id uuid not null references public.saas_paywall_subscriptions(id) on delete cascade,
  event_type text not null, -- created, activated, suspended, cancelled, tier_updated, plan_changed
  old_tier text null,
  new_tier text null,
  old_plan_id uuid null references public.saas_paywall_plans(id) on delete set null,
  new_plan_id uuid null references public.saas_paywall_plans(id) on delete set null,
  actor_id uuid null references public.profiles(id) on delete set null, -- NULL si système
  metadata jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now()
);

create index if not exists idx_saas_paywall_history_subscription_id on public.saas_paywall_subscription_history(subscription_id);
create index if not exists idx_saas_paywall_history_event_type on public.saas_paywall_subscription_history(event_type);
create index if not exists idx_saas_paywall_history_actor_id on public.saas_paywall_subscription_history(actor_id);

comment on table public.saas_paywall_subscription_history is 'Historique des changements d''abonnements (audit trail).';

-- ---------- TRIGGERS updated_at ----------
drop trigger if exists saas_paywall_plans_updated_at on public.saas_paywall_plans;
create trigger saas_paywall_plans_updated_at
  before update on public.saas_paywall_plans
  for each row
  execute function public.handle_updated_at();

drop trigger if exists saas_paywall_subscriptions_updated_at on public.saas_paywall_subscriptions;
create trigger saas_paywall_subscriptions_updated_at
  before update on public.saas_paywall_subscriptions
  for each row
  execute function public.handle_updated_at();

drop trigger if exists saas_paywall_subscription_transactions_updated_at on public.saas_paywall_subscription_transactions;
create trigger saas_paywall_subscription_transactions_updated_at
  before update on public.saas_paywall_subscription_transactions
  for each row
  execute function public.handle_updated_at();

-- ---------- RLS POLICIES ----------
alter table public.saas_paywall_plans enable row level security;
alter table public.saas_paywall_subscriptions enable row level security;
alter table public.saas_paywall_subscription_transactions enable row level security;
alter table public.saas_paywall_subscription_history enable row level security;

-- Plans : lecture publique si active, écriture super_admin uniquement
drop policy if exists saas_paywall_plans_select_public on public.saas_paywall_plans;
create policy saas_paywall_plans_select_public on public.saas_paywall_plans
  for select
  using (status = 'active' or public.is_admin_user());

drop policy if exists saas_paywall_plans_modify_super_admin on public.saas_paywall_plans;
create policy saas_paywall_plans_modify_super_admin on public.saas_paywall_plans
  for all
  using (public.is_super_admin())
  with check (public.is_super_admin());

-- Subscriptions : user voit son propre abonnement, admin voit tous
drop policy if exists saas_paywall_subscriptions_select_user on public.saas_paywall_subscriptions;
create policy saas_paywall_subscriptions_select_user on public.saas_paywall_subscriptions
  for select
  using (auth.uid() = user_id or public.is_admin_user());

drop policy if exists saas_paywall_subscriptions_insert_authenticated on public.saas_paywall_subscriptions;
create policy saas_paywall_subscriptions_insert_authenticated on public.saas_paywall_subscriptions
  for insert
  with check (auth.uid() = user_id);

-- Transactions : user voit ses propres transactions, admin voit tous
drop policy if exists saas_paywall_transactions_select_user on public.saas_paywall_subscription_transactions;
create policy saas_paywall_transactions_select_user on public.saas_paywall_subscription_transactions
  for select
  using (
    exists (
      select 1 from public.saas_paywall_subscriptions s
      where s.id = saas_paywall_subscription_transactions.subscription_id
      and (s.user_id = auth.uid() or public.is_admin_user())
    )
  );

-- History : user voit son historique, admin voit tous
drop policy if exists saas_paywall_history_select_user on public.saas_paywall_subscription_history;
create policy saas_paywall_history_select_user on public.saas_paywall_subscription_history
  for select
  using (
    exists (
      select 1 from public.saas_paywall_subscriptions s
      where s.id = saas_paywall_subscription_history.subscription_id
      and (s.user_id = auth.uid() or public.is_admin_user())
    )
  );

-- ---------- PLAN PRESTATAIRE PAR DÉFAUT ----------
-- Création du plan "Prestataire" à 10€/mois
insert into public.saas_paywall_plans (
  name,
  description,
  price_monthly,
  currency,
  billing_cycle,
  tier,
  features,
  status,
  sort_order,
  is_featured
) values (
  'Prestataire',
  'Abonnement mensuel pour les prestataires du service',
  10.00,
  'EUR',
  'monthly',
  'pro', -- tier associé
  '{
    "features": [
      "Accès complet au module Booking",
      "Gestion illimitée de prestations",
      "Planning et réservations",
      "Profil public personnalisable",
      "Support prioritaire"
    ],
    "limits": {
      "services": -1,
      "bookings_per_month": -1,
      "storage_gb": 10
    }
  }'::jsonb,
  'draft', -- sera activé après création du plan PayPal
  1,
  true
) on conflict do nothing;

commit;

-- =============================================
-- FIN DE LA MIGRATION SAAS PAYWALL PAYPAL
-- =============================================
