-- Miyukini Framework - Migration Agenda
-- Objectif : tables + RLS pour la capability Agenda (source de vérité des créneaux)
-- Date : 2026-01-10 (Europe/Paris)

begin;

-- 0) Enum status slot
do $$
begin
  if not exists (select 1 from pg_type where typname = 'agenda_slot_status') then
    create type public.agenda_slot_status as enum (
      'draft',
      'pending',
      'confirmed',
      'paid',
      'cancelled'
    );
  end if;
end $$;

-- 1) Agendas
create table if not exists public.agendas (
  id uuid primary key default gen_random_uuid(),
  module_id text not null,
  name text not null,
  description text null,

  -- Optionnel (future multi-workspace). Laisser null pour le MVP.
  workspace_id text null,

  created_by uuid not null references public.profiles(id) on delete cascade,

  default_slot_duration int null,
  slot_buffer_before int null,
  slot_buffer_after int null,
  max_participants_per_slot int null,
  allow_overbooking boolean null default false,
  timezone text null,
  is_public boolean not null default false,

  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index if not exists agendas_created_by_idx on public.agendas(created_by);
create index if not exists agendas_module_id_idx on public.agendas(module_id);

comment on table public.agendas is 'Capability Agenda - calendriers.';

-- 2) Slots (créneaux)
create table if not exists public.slots (
  id uuid primary key default gen_random_uuid(),
  agenda_id uuid not null references public.agendas(id) on delete cascade,
  resource_id text null,
  start_at timestamptz not null,
  end_at timestamptz not null,
  status public.agenda_slot_status not null default 'draft',

  capacity int null,
  participants_count int not null default 0,

  price numeric null,
  currency text null,
  payment_link text null,

  tags text[] null,
  metadata jsonb null,

  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),

  constraint slots_time_range_check check (end_at > start_at)
);

create index if not exists slots_agenda_id_idx on public.slots(agenda_id);
create index if not exists slots_start_at_idx on public.slots(start_at);
create index if not exists slots_status_idx on public.slots(status);
create index if not exists slots_tags_gin_idx on public.slots using gin (tags);

comment on table public.slots is 'Capability Agenda - créneaux.';

-- 3) Ressources liées à un slot (optionnel)
create table if not exists public.slot_resources (
  slot_id uuid not null references public.slots(id) on delete cascade,
  resource_id text not null,
  resource_type text null,
  assigned_at timestamptz not null default now(),
  primary key (slot_id, resource_id)
);

-- 4) Participants
create table if not exists public.slot_participants (
  id uuid primary key default gen_random_uuid(),
  slot_id uuid not null references public.slots(id) on delete cascade,
  user_id uuid not null references public.profiles(id) on delete cascade,
  role text null,
  status text null,
  confirmed_at timestamptz null,
  created_at timestamptz not null default now(),
  unique (slot_id, user_id)
);

create index if not exists slot_participants_slot_id_idx on public.slot_participants(slot_id);
create index if not exists slot_participants_user_id_idx on public.slot_participants(user_id);

-- 5) Event log
create table if not exists public.slot_events (
  id uuid primary key default gen_random_uuid(),
  slot_id uuid not null references public.slots(id) on delete cascade,
  event_type text not null,
  actor_id uuid null references public.profiles(id) on delete set null,
  payload jsonb null,
  created_at timestamptz not null default now()
);

create index if not exists slot_events_slot_id_idx on public.slot_events(slot_id);
create index if not exists slot_events_event_type_idx on public.slot_events(event_type);

-- 6) updated_at triggers (fonction public.set_updated_at existe déjà)
drop trigger if exists agendas_set_updated_at on public.agendas;
create trigger agendas_set_updated_at
before update on public.agendas
for each row execute function public.set_updated_at();

drop trigger if exists slots_set_updated_at on public.slots;
create trigger slots_set_updated_at
before update on public.slots
for each row execute function public.set_updated_at();

-- 7) Trigger participants_count (simple)
create or replace function public.agenda_handle_participants_count()
returns trigger
language plpgsql
security definer
set search_path = public
as $$
begin
  if (tg_op = 'INSERT') then
    update public.slots set participants_count = participants_count + 1 where id = new.slot_id;
    return new;
  elsif (tg_op = 'DELETE') then
    update public.slots set participants_count = greatest(participants_count - 1, 0) where id = old.slot_id;
    return old;
  end if;
  return null;
end;
$$;

drop trigger if exists slot_participants_count_insert on public.slot_participants;
create trigger slot_participants_count_insert
after insert on public.slot_participants
for each row execute function public.agenda_handle_participants_count();

drop trigger if exists slot_participants_count_delete on public.slot_participants;
create trigger slot_participants_count_delete
after delete on public.slot_participants
for each row execute function public.agenda_handle_participants_count();

-- 8) RLS (aligné Booking : prestataire (owner) + super_admin, admin exclu)
alter table public.agendas enable row level security;
alter table public.slots enable row level security;
alter table public.slot_resources enable row level security;
alter table public.slot_participants enable row level security;
alter table public.slot_events enable row level security;

-- agendas
drop policy if exists agendas_select_public_or_owner_or_super_admin on public.agendas;
create policy agendas_select_public_or_owner_or_super_admin
on public.agendas for select
using (is_public = true or created_by = auth.uid() or public.is_super_admin());

drop policy if exists agendas_write_owner_or_super_admin on public.agendas;
create policy agendas_write_owner_or_super_admin
on public.agendas for all
using (created_by = auth.uid() or public.is_super_admin())
with check (created_by = auth.uid() or public.is_super_admin());

-- slots: select si agenda public ou owner ou participant ou super_admin
drop policy if exists slots_select_accessible on public.slots;
create policy slots_select_accessible
on public.slots for select
using (
  exists (
    select 1 from public.agendas a
    where a.id = slots.agenda_id
      and (a.is_public = true or a.created_by = auth.uid() or public.is_super_admin())
  )
  or exists (
    select 1 from public.slot_participants sp
    where sp.slot_id = slots.id and sp.user_id = auth.uid()
  )
);

drop policy if exists slots_write_owner_or_super_admin on public.slots;
create policy slots_write_owner_or_super_admin
on public.slots for all
using (
  exists (
    select 1 from public.agendas a
    where a.id = slots.agenda_id
      and (a.created_by = auth.uid() or public.is_super_admin())
  )
)
with check (
  exists (
    select 1 from public.agendas a
    where a.id = slots.agenda_id
      and (a.created_by = auth.uid() or public.is_super_admin())
  )
);

-- slot_resources
drop policy if exists slot_resources_select_owner_or_super_admin on public.slot_resources;
create policy slot_resources_select_owner_or_super_admin
on public.slot_resources for select
using (
  exists (
    select 1 from public.slots s
    join public.agendas a on a.id = s.agenda_id
    where s.id = slot_resources.slot_id
      and (a.created_by = auth.uid() or public.is_super_admin())
  )
);

drop policy if exists slot_resources_write_owner_or_super_admin on public.slot_resources;
create policy slot_resources_write_owner_or_super_admin
on public.slot_resources for all
using (
  exists (
    select 1 from public.slots s
    join public.agendas a on a.id = s.agenda_id
    where s.id = slot_resources.slot_id
      and (a.created_by = auth.uid() or public.is_super_admin())
  )
)
with check (
  exists (
    select 1 from public.slots s
    join public.agendas a on a.id = s.agenda_id
    where s.id = slot_resources.slot_id
      and (a.created_by = auth.uid() or public.is_super_admin())
  )
);

-- slot_participants
drop policy if exists slot_participants_select_participant_owner_or_super_admin on public.slot_participants;
create policy slot_participants_select_participant_owner_or_super_admin
on public.slot_participants for select
using (
  user_id = auth.uid()
  or exists (
    select 1 from public.slots s
    join public.agendas a on a.id = s.agenda_id
    where s.id = slot_participants.slot_id
      and (a.created_by = auth.uid() or public.is_super_admin())
  )
);

drop policy if exists slot_participants_write_owner_or_self_or_super_admin on public.slot_participants;
create policy slot_participants_write_owner_or_self_or_super_admin
on public.slot_participants for all
using (
  user_id = auth.uid()
  or exists (
    select 1 from public.slots s
    join public.agendas a on a.id = s.agenda_id
    where s.id = slot_participants.slot_id
      and (a.created_by = auth.uid() or public.is_super_admin())
  )
)
with check (
  user_id = auth.uid()
  or exists (
    select 1 from public.slots s
    join public.agendas a on a.id = s.agenda_id
    where s.id = slot_participants.slot_id
      and (a.created_by = auth.uid() or public.is_super_admin())
  )
);

-- slot_events (logs)
drop policy if exists slot_events_select_owner_or_super_admin on public.slot_events;
create policy slot_events_select_owner_or_super_admin
on public.slot_events for select
using (
  exists (
    select 1 from public.slots s
    join public.agendas a on a.id = s.agenda_id
    where s.id = slot_events.slot_id
      and (a.created_by = auth.uid() or public.is_super_admin())
  )
);

drop policy if exists slot_events_insert_owner_or_super_admin on public.slot_events;
create policy slot_events_insert_owner_or_super_admin
on public.slot_events for insert
with check (
  exists (
    select 1 from public.slots s
    join public.agendas a on a.id = s.agenda_id
    where s.id = slot_events.slot_id
      and (a.created_by = auth.uid() or public.is_super_admin())
  )
);

commit;

