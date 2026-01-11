-- Miyukini Framework - Migration Booking
-- Objectif : tables + RLS pour le module Booking (Reservation)
-- Date : 2026-01-10 (Europe/Paris)

begin;

-- 0) Enums (sans IF NOT EXISTS natif fiable)
do $$
begin
  if not exists (select 1 from pg_type where typname = 'booking_booking_status') then
    create type public.booking_booking_status as enum (
      'requested',
      'confirmed',
      'cancelled_by_client',
      'cancelled_by_provider',
      'no_show',
      'completed'
    );
  end if;

  if not exists (select 1 from pg_type where typname = 'booking_time_off_mode') then
    create type public.booking_time_off_mode as enum (
      'block_slots',
      'cancel_bookings',
      'request_reschedule'
    );
  end if;
end $$;

-- 1) Prestataires
create table if not exists public.booking_providers (
  id uuid primary key references public.profiles(id) on delete cascade,
  display_name text,
  timezone text,
  tags text[] null,
  is_active boolean not null default true,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

comment on table public.booking_providers is 'Prestataires Booking (un prestataire = un profil).';

-- 2) Prestations
create table if not exists public.booking_services (
  id uuid primary key default gen_random_uuid(),
  provider_id uuid not null references public.booking_providers(id) on delete cascade,
  name text not null,
  description text null,
  duration_minutes int not null,
  price_hint numeric null,
  currency text null,
  requires_approval boolean not null default false,
  cancellation_policy jsonb null,
  default_capacity int not null default 1,
  tags text[] null,
  is_active boolean not null default true,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index if not exists booking_services_provider_id_idx on public.booking_services(provider_id);

-- 3) Réservations (bookings)
create table if not exists public.booking_bookings (
  id uuid primary key default gen_random_uuid(),
  provider_id uuid not null references public.booking_providers(id) on delete cascade,
  service_id uuid not null references public.booking_services(id) on delete restrict,
  agenda_id text null,
  slot_id text null,
  customer_id uuid null references public.profiles(id) on delete set null,
  customer_email text null,
  customer_phone text null,
  status public.booking_booking_status not null default 'requested',
  quantity int not null default 1,
  notes_customer text null,
  notes_internal text null,
  cancel_reason text null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  constraint booking_bookings_customer_check check (customer_id is not null or customer_email is not null)
);

create index if not exists booking_bookings_provider_id_idx on public.booking_bookings(provider_id);
create index if not exists booking_bookings_customer_id_idx on public.booking_bookings(customer_id);
create index if not exists booking_bookings_slot_id_idx on public.booking_bookings(slot_id);

-- 4) Prestations disponibles par créneau (slot ↔ service)
create table if not exists public.booking_slot_services (
  id uuid primary key default gen_random_uuid(),
  provider_id uuid not null references public.booking_providers(id) on delete cascade,
  slot_id text not null,
  service_id uuid not null references public.booking_services(id) on delete cascade,
  created_at timestamptz not null default now(),
  unique(provider_id, slot_id, service_id)
);

create index if not exists booking_slot_services_provider_id_idx on public.booking_slot_services(provider_id);
create index if not exists booking_slot_services_slot_id_idx on public.booking_slot_services(slot_id);

-- 5) Semaine type
create table if not exists public.booking_week_templates (
  id uuid primary key default gen_random_uuid(),
  provider_id uuid not null references public.booking_providers(id) on delete cascade,
  name text not null,
  timezone text null,
  rules jsonb not null default '{}'::jsonb,
  is_active boolean not null default true,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index if not exists booking_week_templates_provider_id_idx on public.booking_week_templates(provider_id);

-- 6) Vacances / indisponibilités
create table if not exists public.booking_time_off (
  id uuid primary key default gen_random_uuid(),
  provider_id uuid not null references public.booking_providers(id) on delete cascade,
  start_at timestamptz not null,
  end_at timestamptz not null,
  mode public.booking_time_off_mode not null default 'block_slots',
  reason text null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  constraint booking_time_off_range_check check (end_at > start_at)
);

create index if not exists booking_time_off_provider_id_idx on public.booking_time_off(provider_id);
create index if not exists booking_time_off_range_idx on public.booking_time_off(start_at, end_at);

-- 7) updated_at triggers (fonction public.set_updated_at existe déjà)
drop trigger if exists booking_providers_set_updated_at on public.booking_providers;
create trigger booking_providers_set_updated_at
before update on public.booking_providers
for each row execute function public.set_updated_at();

drop trigger if exists booking_services_set_updated_at on public.booking_services;
create trigger booking_services_set_updated_at
before update on public.booking_services
for each row execute function public.set_updated_at();

drop trigger if exists booking_bookings_set_updated_at on public.booking_bookings;
create trigger booking_bookings_set_updated_at
before update on public.booking_bookings
for each row execute function public.set_updated_at();

drop trigger if exists booking_week_templates_set_updated_at on public.booking_week_templates;
create trigger booking_week_templates_set_updated_at
before update on public.booking_week_templates
for each row execute function public.set_updated_at();

drop trigger if exists booking_time_off_set_updated_at on public.booking_time_off;
create trigger booking_time_off_set_updated_at
before update on public.booking_time_off
for each row execute function public.set_updated_at();

-- 8) RLS
alter table public.booking_providers enable row level security;
alter table public.booking_services enable row level security;
alter table public.booking_bookings enable row level security;
alter table public.booking_slot_services enable row level security;
alter table public.booking_week_templates enable row level security;
alter table public.booking_time_off enable row level security;

-- booking_providers
drop policy if exists booking_providers_select_owner_or_super_admin on public.booking_providers;
create policy booking_providers_select_owner_or_super_admin
on public.booking_providers for select
using (id = auth.uid() or public.is_super_admin());

drop policy if exists booking_providers_insert_owner_or_super_admin on public.booking_providers;
create policy booking_providers_insert_owner_or_super_admin
on public.booking_providers for insert
with check (id = auth.uid() or public.is_super_admin());

drop policy if exists booking_providers_update_owner_or_super_admin on public.booking_providers;
create policy booking_providers_update_owner_or_super_admin
on public.booking_providers for update
using (id = auth.uid() or public.is_super_admin())
with check (id = auth.uid() or public.is_super_admin());

drop policy if exists booking_providers_delete_super_admin on public.booking_providers;
create policy booking_providers_delete_super_admin
on public.booking_providers for delete
using (public.is_super_admin());

-- booking_services
drop policy if exists booking_services_select_public_or_owner_or_super_admin on public.booking_services;
create policy booking_services_select_public_or_owner_or_super_admin
on public.booking_services for select
using (is_active = true or provider_id = auth.uid() or public.is_super_admin());

drop policy if exists booking_services_write_owner_or_super_admin on public.booking_services;
create policy booking_services_write_owner_or_super_admin
on public.booking_services for all
using (provider_id = auth.uid() or public.is_super_admin())
with check (provider_id = auth.uid() or public.is_super_admin());

-- booking_slot_services
drop policy if exists booking_slot_services_select_owner_or_super_admin on public.booking_slot_services;
create policy booking_slot_services_select_owner_or_super_admin
on public.booking_slot_services for select
using (provider_id = auth.uid() or public.is_super_admin());

drop policy if exists booking_slot_services_write_owner_or_super_admin on public.booking_slot_services;
create policy booking_slot_services_write_owner_or_super_admin
on public.booking_slot_services for all
using (provider_id = auth.uid() or public.is_super_admin())
with check (provider_id = auth.uid() or public.is_super_admin());

-- booking_week_templates
drop policy if exists booking_week_templates_select_owner_or_super_admin on public.booking_week_templates;
create policy booking_week_templates_select_owner_or_super_admin
on public.booking_week_templates for select
using (provider_id = auth.uid() or public.is_super_admin());

drop policy if exists booking_week_templates_write_owner_or_super_admin on public.booking_week_templates;
create policy booking_week_templates_write_owner_or_super_admin
on public.booking_week_templates for all
using (provider_id = auth.uid() or public.is_super_admin())
with check (provider_id = auth.uid() or public.is_super_admin());

-- booking_time_off
drop policy if exists booking_time_off_select_owner_or_super_admin on public.booking_time_off;
create policy booking_time_off_select_owner_or_super_admin
on public.booking_time_off for select
using (provider_id = auth.uid() or public.is_super_admin());

drop policy if exists booking_time_off_write_owner_or_super_admin on public.booking_time_off;
create policy booking_time_off_write_owner_or_super_admin
on public.booking_time_off for all
using (provider_id = auth.uid() or public.is_super_admin())
with check (provider_id = auth.uid() or public.is_super_admin());

-- booking_bookings
drop policy if exists booking_bookings_select_owner_customer_or_super_admin on public.booking_bookings;
create policy booking_bookings_select_owner_customer_or_super_admin
on public.booking_bookings for select
using (
  provider_id = auth.uid()
  or customer_id = auth.uid()
  or public.is_super_admin()
);

-- Insert: autoriser connecté, et autoriser anon si customer_email fourni (MVP)
drop policy if exists booking_bookings_insert_auth_or_email on public.booking_bookings;
create policy booking_bookings_insert_auth_or_email
on public.booking_bookings for insert
with check (
  public.is_super_admin()
  or provider_id is not null
  and service_id is not null
  and quantity >= 1
  and (
    customer_id = auth.uid()
    or (auth.uid() is null and customer_id is null and customer_email is not null)
  )
);

drop policy if exists booking_bookings_update_owner_customer_or_super_admin on public.booking_bookings;
create policy booking_bookings_update_owner_customer_or_super_admin
on public.booking_bookings for update
using (
  provider_id = auth.uid()
  or customer_id = auth.uid()
  or public.is_super_admin()
)
with check (
  provider_id = auth.uid()
  or customer_id = auth.uid()
  or public.is_super_admin()
);

drop policy if exists booking_bookings_delete_super_admin on public.booking_bookings;
create policy booking_bookings_delete_super_admin
on public.booking_bookings for delete
using (public.is_super_admin());

commit;

