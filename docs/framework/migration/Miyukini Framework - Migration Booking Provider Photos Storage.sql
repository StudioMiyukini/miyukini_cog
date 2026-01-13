-- Miyukini Framework - Migration Booking Provider Photos Storage
-- Objectif : bucket Supabase Storage + policies pour uploader/servir les photos prestataire
-- Date : 2026-01-12 (Europe/Paris)

begin;

-- 1) Bucket public pour les photos
insert into storage.buckets (id, name, public, file_size_limit, allowed_mime_types)
values (
  'booking-provider-photos',
  'booking-provider-photos',
  true,
  10485760,
  array['image/jpeg','image/png','image/webp']
)
on conflict (id) do update
  set public = excluded.public,
      file_size_limit = excluded.file_size_limit,
      allowed_mime_types = excluded.allowed_mime_types;

-- 2) RLS storage.objects : public read + provider write in own folder <providerId>/...
drop policy if exists "Public read booking provider photos" on storage.objects;
create policy "Public read booking provider photos"
on storage.objects
for select
to public
using (bucket_id = 'booking-provider-photos');

drop policy if exists "Provider manage own booking photos" on storage.objects;
create policy "Provider manage own booking photos"
on storage.objects
for all
to authenticated
using (
  bucket_id = 'booking-provider-photos'
  and auth.uid()::text = (storage.foldername(name))[1]
)
with check (
  bucket_id = 'booking-provider-photos'
  and auth.uid()::text = (storage.foldername(name))[1]
);

commit;

