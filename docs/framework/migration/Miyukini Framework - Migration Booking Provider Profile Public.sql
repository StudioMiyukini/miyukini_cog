-- Miyukini Framework - Migration Booking Provider Profile Public
-- Objectif : enrichir le profil public prestataire (adresse, horaires, contact, reseaux, photos, avis)
-- Date : 2026-01-12 (Europe/Paris)

begin;

-- 1) booking_providers : champs "fiche"
alter table public.booking_providers
  add column if not exists description text,
  add column if not exists phone text,
  add column if not exists website text,
  add column if not exists contact_email text,
  add column if not exists address_line1 text,
  add column if not exists address_line2 text,
  add column if not exists postal_code text,
  add column if not exists city text,
  add column if not exists country text default 'FR',
  add column if not exists lat numeric,
  add column if not exists lng numeric,
  add column if not exists opening_hours jsonb not null default '{}'::jsonb,
  add column if not exists social_links jsonb not null default '{}'::jsonb;

comment on column public.booking_providers.opening_hours is
  'JSON horaires. Exemple: {\"mon\":{\"open\":\"08:00\",\"close\":\"19:00\"},\"sun\":{\"closed\":true}}';

comment on column public.booking_providers.social_links is
  'JSON liens reseaux. Exemple: {\"instagram\":\"https://...\",\"facebook\":\"https://...\"}';

-- 2) booking_services : categories de prestations
alter table public.booking_services
  add column if not exists category text,
  add column if not exists category_sort integer;

-- 3) Photos prestataire
create table if not exists public.booking_provider_photos (
  id uuid primary key default gen_random_uuid(),
  provider_id uuid not null references public.booking_providers(id) on delete cascade,
  path text,
  url text,
  alt text,
  sort_order integer not null default 0,
  is_public boolean not null default true,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create index if not exists booking_provider_photos_provider_id_idx on public.booking_provider_photos(provider_id);
create index if not exists booking_provider_photos_public_sort_idx on public.booking_provider_photos(provider_id, is_public, sort_order);

-- 4) Avis prestataire (lecture publique des avis publies)
create table if not exists public.booking_provider_reviews (
  id uuid primary key default gen_random_uuid(),
  provider_id uuid not null references public.booking_providers(id) on delete cascade,
  booking_id uuid null references public.booking_bookings(id) on delete set null,
  author_name text,
  rating integer not null check (rating between 1 and 5),
  comment text,
  is_published boolean not null default false,
  created_at timestamptz not null default now()
);

create index if not exists booking_provider_reviews_provider_id_idx on public.booking_provider_reviews(provider_id);
create index if not exists booking_provider_reviews_published_idx on public.booking_provider_reviews(provider_id, is_published, created_at desc);

-- 5) updated_at trigger (photos)
create or replace function public.set_updated_at()
returns trigger
language plpgsql
as $$
begin
  new.updated_at = now();
  return new;
end;
$$;

drop trigger if exists set_booking_provider_photos_updated_at on public.booking_provider_photos;
create trigger set_booking_provider_photos_updated_at
before update on public.booking_provider_photos
for each row
execute function public.set_updated_at();

-- 6) RLS (public read)
alter table public.booking_provider_photos enable row level security;
alter table public.booking_provider_reviews enable row level security;

drop policy if exists "Public read provider photos" on public.booking_provider_photos;
create policy "Public read provider photos"
on public.booking_provider_photos
for select
to public
using (
  is_public = true
  and exists (
    select 1 from public.booking_providers p
    where p.id = booking_provider_photos.provider_id
      and p.is_active = true
  )
);

drop policy if exists "Provider manage own photos" on public.booking_provider_photos;
create policy "Provider manage own photos"
on public.booking_provider_photos
for all
to authenticated
using (auth.uid() = provider_id)
with check (auth.uid() = provider_id);

drop policy if exists "Public read provider reviews" on public.booking_provider_reviews;
create policy "Public read provider reviews"
on public.booking_provider_reviews
for select
to public
using (
  is_published = true
  and exists (
    select 1 from public.booking_providers p
    where p.id = booking_provider_reviews.provider_id
      and p.is_active = true
  )
);

commit;

