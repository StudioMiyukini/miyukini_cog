-- =============================================
-- MIYUKINI FRAMEWORK - MIGRATION SAAS PAYWALL MULTI PAYMENT METHODS
-- Module: SaaS Paywall (extension pour plusieurs modes de paiement)
-- Version: 1.0.0
-- Date: 2026-01-15 (Europe/Paris)
-- =============================================
--
-- Objectif :
-- - Étendre le système de paywall pour supporter plusieurs modes de paiement
-- - Ajouter support virement bancaire et autres méthodes manuelles
-- - Créer table pour paiements manuels (virements) avec confirmation admin
--
-- Dépendances :
-- - Migration SaaS Paywall PayPal (saas_paywall_plans, saas_paywall_subscriptions)
-- - Migration Commerce par Devis (commerce_payment_method enum)
-- =============================================

begin;

-- ---------- ENUM PAYMENT METHOD (réutiliser celui de Commerce si existe, sinon créer) ----------
do $$
begin
  if not exists (select 1 from pg_type where typname = 'commerce_payment_method') then
    create type public.commerce_payment_method as enum (
      'bank_transfer',
      'cash',
      'check',
      'card_remote',
      'paypal',
      'other'
    );
  else
    -- Ajouter 'paypal' si l'enum existe déjà mais ne le contient pas
    if not exists (
      select 1 from pg_enum 
      where enumlabel = 'paypal' 
      and enumtypid = (select oid from pg_type where typname = 'commerce_payment_method')
    ) then
      alter type public.commerce_payment_method add value 'paypal';
    end if;
  end if;
end $$;

-- ---------- ENUM MANUAL PAYMENT STATUS ----------
do $$
begin
  if not exists (select 1 from pg_type where typname = 'saas_paywall_manual_payment_status') then
    create type public.saas_paywall_manual_payment_status as enum (
      'pending',
      'confirmed',
      'rejected',
      'cancelled'
    );
  end if;
end $$;

-- ---------- MODIFIER TABLE saas_paywall_subscriptions ----------
-- Ajouter payment_method et rendre paypal_subscription_id nullable
alter table public.saas_paywall_subscriptions
  add column if not exists payment_method public.commerce_payment_method not null default 'paypal',
  alter column paypal_subscription_id drop not null;

-- Commentaire
comment on column public.saas_paywall_subscriptions.payment_method is 'Méthode de paiement : paypal (automatique) ou bank_transfer (manuel)';

-- Index pour les requêtes par méthode de paiement
create index if not exists idx_saas_paywall_subscriptions_payment_method 
  on public.saas_paywall_subscriptions(payment_method);

-- ---------- TABLE saas_paywall_manual_payments ----------
-- Pour les paiements manuels (virements bancaires, chèques, etc.)
create table if not exists public.saas_paywall_manual_payments (
  id uuid primary key default gen_random_uuid(),
  subscription_id uuid not null references public.saas_paywall_subscriptions(id) on delete cascade,
  payment_method public.commerce_payment_method not null default 'bank_transfer',
  amount numeric not null,
  currency text not null default 'EUR',
  reference text null, -- Référence du virement/chèque
  status public.saas_paywall_manual_payment_status not null default 'pending',
  due_date date not null, -- Date d'échéance du paiement
  paid_at timestamptz null, -- Date réelle du paiement (si connue)
  confirmed_at timestamptz null,
  confirmed_by uuid null references public.profiles(id) on delete set null,
  rejected_at timestamptz null,
  rejected_by uuid null references public.profiles(id) on delete set null,
  rejection_reason text null,
  proof_document_ref jsonb null, -- Référence vers document de preuve (bucket/path)
  notes text null,
  billing_cycle integer not null default 1, -- Numéro du cycle (1, 2, 3...)
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index if not exists idx_saas_paywall_manual_payments_subscription_id 
  on public.saas_paywall_manual_payments(subscription_id);
create index if not exists idx_saas_paywall_manual_payments_status 
  on public.saas_paywall_manual_payments(status);
create index if not exists idx_saas_paywall_manual_payments_due_date 
  on public.saas_paywall_manual_payments(due_date);
create index if not exists idx_saas_paywall_manual_payments_payment_method 
  on public.saas_paywall_manual_payments(payment_method);

comment on table public.saas_paywall_manual_payments is 'Paiements manuels pour abonnements (virements bancaires, chèques, etc.) nécessitant confirmation admin.';

-- ---------- TRIGGER updated_at ----------
drop trigger if exists saas_paywall_manual_payments_updated_at on public.saas_paywall_manual_payments;
create trigger saas_paywall_manual_payments_updated_at
  before update on public.saas_paywall_manual_payments
  for each row
  execute function public.handle_updated_at();

-- ---------- RLS POLICIES ----------
alter table public.saas_paywall_manual_payments enable row level security;

-- User voit ses propres paiements en attente, admin voit tous
drop policy if exists saas_paywall_manual_payments_select_user on public.saas_paywall_manual_payments;
create policy saas_paywall_manual_payments_select_user on public.saas_paywall_manual_payments
  for select
  using (
    exists (
      select 1 from public.saas_paywall_subscriptions s
      where s.id = saas_paywall_manual_payments.subscription_id
      and (s.user_id = auth.uid() or public.is_admin_user())
    )
  );

-- User peut créer ses propres paiements en attente, admin peut tout
drop policy if exists saas_paywall_manual_payments_insert_user on public.saas_paywall_manual_payments;
create policy saas_paywall_manual_payments_insert_user on public.saas_paywall_manual_payments
  for insert
  with check (
    exists (
      select 1 from public.saas_paywall_subscriptions s
      where s.id = saas_paywall_manual_payments.subscription_id
      and (s.user_id = auth.uid() or public.is_admin_user())
    )
  );

-- Seul admin peut modifier (confirmer/rejeter)
drop policy if exists saas_paywall_manual_payments_update_admin on public.saas_paywall_manual_payments;
create policy saas_paywall_manual_payments_update_admin on public.saas_paywall_manual_payments
  for update
  using (public.is_admin_user())
  with check (public.is_admin_user());

-- ---------- FUNCTION: Confirmer un paiement manuel ----------
create or replace function public.saas_paywall_confirm_manual_payment(
  p_payment_id uuid,
  p_reference text default null,
  p_paid_at timestamptz default null,
  p_notes text default null
)
returns uuid
language plpgsql
security definer
as $$
declare
  v_payment public.saas_paywall_manual_payments;
  v_subscription public.saas_paywall_subscriptions;
  v_plan public.saas_paywall_plans;
  v_transaction_id uuid;
begin
  if not public.is_admin_user() then
    raise exception 'forbidden';
  end if;

  -- Charger le paiement
  select * into v_payment from public.saas_paywall_manual_payments where id = p_payment_id;
  if not found then
    raise exception 'payment_not_found';
  end if;

  if v_payment.status != 'pending' then
    raise exception 'payment_already_processed';
  end if;

  -- Charger l'abonnement et le plan
  select * into v_subscription from public.saas_paywall_subscriptions where id = v_payment.subscription_id;
  select * into v_plan from public.saas_paywall_plans where id = v_subscription.plan_id;

  -- Confirmer le paiement
  update public.saas_paywall_manual_payments
  set 
    status = 'confirmed',
    reference = coalesce(p_reference, reference),
    paid_at = coalesce(p_paid_at, now()),
    confirmed_at = now(),
    confirmed_by = auth.uid(),
    notes = coalesce(p_notes, notes),
    updated_at = now()
  where id = p_payment_id;

  -- Créer une transaction dans saas_paywall_subscription_transactions
  -- (pour compatibilité avec l'historique existant)
  insert into public.saas_paywall_subscription_transactions (
    subscription_id,
    paypal_transaction_id, -- Réutilisé pour stocker l'ID du paiement manuel
    billing_cycle,
    status,
    amount,
    currency,
    transaction_time
  ) values (
    v_subscription.id,
    'manual_' || p_payment_id::text,
    v_payment.billing_cycle,
    'COMPLETED',
    v_payment.amount,
    v_payment.currency,
    coalesce(p_paid_at, now())
  )
  returning id into v_transaction_id;

  -- Activer l'abonnement si c'est le premier paiement
  if v_subscription.status in ('APPROVAL_PENDING', 'APPROVED') then
    update public.saas_paywall_subscriptions
    set 
      status = 'ACTIVE',
      start_date = coalesce(p_paid_at, now()),
      current_period_start = coalesce(p_paid_at, now()),
      current_period_end = (coalesce(p_paid_at, now()) + (v_plan.billing_cycle || ' months')::interval)::timestamptz,
      next_billing_date = (coalesce(p_paid_at, now()) + (v_plan.billing_cycle || ' months')::interval)::timestamptz,
      updated_at = now()
    where id = v_subscription.id;

    -- Historique
    insert into public.saas_paywall_subscription_history (
      subscription_id,
      event_type,
      new_tier,
      new_plan_id,
      actor_id,
      metadata
    ) values (
      v_subscription.id,
      'activated',
      v_plan.tier,
      v_plan.id,
      auth.uid(),
      jsonb_build_object('payment_method', v_payment.payment_method, 'payment_id', p_payment_id, 'transaction_id', v_transaction_id)
    );
  end if;

  -- Event outbox pour déclencher notifications/emails
  insert into public.commerce_outbox (event_type, dedupe_key, payload)
  values (
    'paywall.manual_payment.confirmed',
    'paywall.manual_payment.confirmed:' || p_payment_id::text,
    jsonb_build_object(
      'payment_id', p_payment_id,
      'subscription_id', v_subscription.id,
      'user_id', v_subscription.user_id,
      'amount', v_payment.amount,
      'currency', v_payment.currency,
      'payment_method', v_payment.payment_method,
      'confirmed_at', now()
    )
  )
  on conflict (dedupe_key) do nothing;

  return p_payment_id;
end;
$$;

grant execute on function public.saas_paywall_confirm_manual_payment(uuid, text, timestamptz, text) to authenticated;

-- ---------- FUNCTION: Rejeter un paiement manuel ----------
create or replace function public.saas_paywall_reject_manual_payment(
  p_payment_id uuid,
  p_reason text
)
returns uuid
language plpgsql
security definer
as $$
declare
  v_payment public.saas_paywall_manual_payments;
begin
  if not public.is_admin_user() then
    raise exception 'forbidden';
  end if;

  select * into v_payment from public.saas_paywall_manual_payments where id = p_payment_id;
  if not found then
    raise exception 'payment_not_found';
  end if;

  if v_payment.status != 'pending' then
    raise exception 'payment_already_processed';
  end if;

  update public.saas_paywall_manual_payments
  set 
    status = 'rejected',
    rejected_at = now(),
    rejected_by = auth.uid(),
    rejection_reason = p_reason,
    updated_at = now()
  where id = p_payment_id;

  return p_payment_id;
end;
$$;

grant execute on function public.saas_paywall_reject_manual_payment(uuid, text) to authenticated;

commit;

-- =============================================
-- FIN DE LA MIGRATION SAAS PAYWALL MULTI PAYMENT METHODS
-- =============================================
