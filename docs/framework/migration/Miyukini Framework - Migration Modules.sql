-- Miyukini Framework - Migration Modules
-- Objectif : activer/désactiver des modules côté DB (piloté masteradmin/super_admin)
-- Date : 2026-01-10 (Europe/Paris)

begin;

-- 1) Table de pilotage d'activation des modules
create table if not exists public.framework_modules (
  module_id text primary key,
  enabled boolean not null default false,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

comment on table public.framework_modules is
  'Catalogue d''activation des modules Miyukini. Source de vérité: enabled (piloté par super_admin).';

comment on column public.framework_modules.module_id is
  'Identifiant stable du module (ex: booking, agenda-pro).';

comment on column public.framework_modules.enabled is
  'Active/désactive le module dans l''application.';

-- 2) Trigger updated_at
create or replace function public.set_updated_at()
returns trigger
language plpgsql
as $$
begin
  new.updated_at = now();
  return new;
end;
$$;

drop trigger if exists framework_modules_set_updated_at on public.framework_modules;
create trigger framework_modules_set_updated_at
before update on public.framework_modules
for each row execute function public.set_updated_at();

-- 3) RLS
alter table public.framework_modules enable row level security;

-- Lecture : autorisée à tous (y compris anon) car non sensible
drop policy if exists framework_modules_select_all on public.framework_modules;
create policy framework_modules_select_all
on public.framework_modules
for select
using (true);

-- Écriture : super_admin uniquement (helper défini dans la migration initiale)
drop policy if exists framework_modules_insert_super_admin on public.framework_modules;
create policy framework_modules_insert_super_admin
on public.framework_modules
for insert
with check (public.is_super_admin());

drop policy if exists framework_modules_update_super_admin on public.framework_modules;
create policy framework_modules_update_super_admin
on public.framework_modules
for update
using (public.is_super_admin())
with check (public.is_super_admin());

drop policy if exists framework_modules_delete_super_admin on public.framework_modules;
create policy framework_modules_delete_super_admin
on public.framework_modules
for delete
using (public.is_super_admin());

-- 4) Seeds (modules connus) - désactivés par défaut
insert into public.framework_modules (module_id, enabled)
values
  ('booking', false),
  ('agenda-pro', false)
on conflict (module_id) do nothing;

commit;

