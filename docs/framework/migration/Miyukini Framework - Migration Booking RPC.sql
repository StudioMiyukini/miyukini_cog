-- Miyukini Framework - Migration Booking RPC
-- Objectif : RPC robustes pour génération de slots + réservation (capacity/time_off) + politiques publiques providers
-- Date : 2026-01-10 (Europe/Paris)

begin;

-- 1) Rendre la liste des prestataires visible publiquement (pour la recherche client)
drop policy if exists booking_providers_select_owner_or_super_admin on public.booking_providers;
create policy booking_providers_select_public_or_owner_or_super_admin
on public.booking_providers for select
using (is_active = true or id = auth.uid() or public.is_super_admin());

-- 2) Unicité des slots (anti-doublons à la génération)
create unique index if not exists slots_unique_range_idx on public.slots(agenda_id, start_at, end_at);

-- 3) Guard helper : ignore vacances en mode block_slots
create or replace function public.booking_is_blocked_by_time_off(
  p_provider_id uuid,
  p_start_at timestamptz,
  p_end_at timestamptz
)
returns boolean
language sql
stable
security definer
set search_path = public
as $$
  select exists (
    select 1
    from public.booking_time_off t
    where t.provider_id = p_provider_id
      and t.mode = 'block_slots'::public.booking_time_off_mode
      and tstzrange(t.start_at, t.end_at, '[)') && tstzrange(p_start_at, p_end_at, '[)')
  );
$$;

-- 4) Maintenir slots.participants_count via booking_bookings (MVP Booking)
create or replace function public.booking_handle_slots_participants_count()
returns trigger
language plpgsql
security definer
set search_path = public
as $$
declare
  v_slot_id uuid;
  v_delta int;
begin
  -- slot_id est stocké en text côté booking_bookings (compat). On ignore si null/non-uuid.
  if (tg_op = 'INSERT') then
    if new.slot_id is null then return new; end if;
    begin
      v_slot_id := new.slot_id::uuid;
    exception when others then
      return new;
    end;
    v_delta := coalesce(new.quantity, 1);
    update public.slots set participants_count = participants_count + v_delta where id = v_slot_id;
    return new;
  elsif (tg_op = 'DELETE') then
    if old.slot_id is null then return old; end if;
    begin
      v_slot_id := old.slot_id::uuid;
    exception when others then
      return old;
    end;
    v_delta := coalesce(old.quantity, 1);
    update public.slots set participants_count = greatest(participants_count - v_delta, 0) where id = v_slot_id;
    return old;
  elsif (tg_op = 'UPDATE') then
    -- si slot_id change ou quantity change : ajuster
    if (new.slot_id is distinct from old.slot_id) or (new.quantity is distinct from old.quantity) then
      -- remove old
      if old.slot_id is not null then
        begin
          v_slot_id := old.slot_id::uuid;
          v_delta := coalesce(old.quantity, 1);
          update public.slots set participants_count = greatest(participants_count - v_delta, 0) where id = v_slot_id;
        exception when others then
          -- ignore cast errors
        end;
      end if;
      -- add new
      if new.slot_id is not null then
        begin
          v_slot_id := new.slot_id::uuid;
          v_delta := coalesce(new.quantity, 1);
          update public.slots set participants_count = participants_count + v_delta where id = v_slot_id;
        exception when others then
          -- ignore cast errors
        end;
      end if;
    end if;
    return new;
  end if;
  return null;
end;
$$;

drop trigger if exists booking_bookings_slots_participants_insert on public.booking_bookings;
create trigger booking_bookings_slots_participants_insert
after insert on public.booking_bookings
for each row execute function public.booking_handle_slots_participants_count();

drop trigger if exists booking_bookings_slots_participants_delete on public.booking_bookings;
create trigger booking_bookings_slots_participants_delete
after delete on public.booking_bookings
for each row execute function public.booking_handle_slots_participants_count();

drop trigger if exists booking_bookings_slots_participants_update on public.booking_bookings;
create trigger booking_bookings_slots_participants_update
after update of slot_id, quantity on public.booking_bookings
for each row execute function public.booking_handle_slots_participants_count();

-- 5) RPC : générer des slots depuis booking_week_templates.rules
create or replace function public.booking_generate_slots(
  provider_id uuid,
  template_id uuid,
  range_start date,
  range_end date
)
returns void
language plpgsql
security definer
set search_path = public
as $$
declare
  v_agenda_id uuid;
  v_rules jsonb;
  v_date date;
  v_dow int;
  v_day_key text;
  v_block jsonb;
  v_start_time text;
  v_end_time text;
  v_slot_minutes int;
  v_capacity int;
  v_service_ids text[];
  v_hour int;
  v_min int;
  v_end_hour int;
  v_end_min int;
  v_cur timestamptz;
  v_end timestamptz;
  v_slot_end timestamptz;
  v_slot_id uuid;
  v_tz text;
begin
  if range_end < range_start then
    raise exception 'range_end must be >= range_start';
  end if;

  -- check module enabled
  if not exists (select 1 from public.framework_modules fm where fm.module_id='booking' and fm.enabled=true) then
    raise exception 'booking module disabled';
  end if;

  select a.id, coalesce(a.timezone, 'Europe/Paris')
    into v_agenda_id, v_tz
  from public.agendas a
  where a.module_id = 'booking'
    and a.created_by = provider_id
  limit 1;

  if v_agenda_id is null then
    insert into public.agendas (module_id, name, description, created_by, timezone, is_public)
    values ('booking', 'Agenda Booking', 'Agenda prestataire (Booking)', provider_id, 'Europe/Paris', true)
    returning id, timezone into v_agenda_id, v_tz;
  end if;

  select t.rules into v_rules
  from public.booking_week_templates t
  where t.id = template_id and t.provider_id = provider_id;

  if v_rules is null then
    raise exception 'template not found';
  end if;

  v_date := range_start;
  while v_date <= range_end loop
    v_dow := extract(isodow from v_date)::int; -- 1=lun .. 7=dim
    v_day_key := case v_dow
      when 1 then 'monday'
      when 2 then 'tuesday'
      when 3 then 'wednesday'
      when 4 then 'thursday'
      when 5 then 'friday'
      when 6 then 'saturday'
      when 7 then 'sunday'
    end;

    if v_rules ? v_day_key then
      for v_block in select * from jsonb_array_elements(coalesce(v_rules->v_day_key, '[]'::jsonb))
      loop
        v_start_time := coalesce(v_block->>'start', '09:00');
        v_end_time := coalesce(v_block->>'end', '10:00');
        v_slot_minutes := greatest((v_block->>'slot')::int, 5);
        v_capacity := greatest((v_block->>'capacity')::int, 1);
        v_service_ids := array(select jsonb_array_elements_text(coalesce(v_block->'service_ids', '[]'::jsonb)));

        v_hour := split_part(v_start_time, ':', 1)::int;
        v_min := split_part(v_start_time, ':', 2)::int;
        v_end_hour := split_part(v_end_time, ':', 1)::int;
        v_end_min := split_part(v_end_time, ':', 2)::int;

        v_cur := make_timestamptz(extract(year from v_date)::int, extract(month from v_date)::int, extract(day from v_date)::int, v_hour, v_min, 0, v_tz);
        v_end := make_timestamptz(extract(year from v_date)::int, extract(month from v_date)::int, extract(day from v_date)::int, v_end_hour, v_end_min, 0, v_tz);

        while v_cur < v_end loop
          v_slot_end := v_cur + make_interval(mins => v_slot_minutes);
          if v_slot_end > v_end then
            exit;
          end if;

          if not public.booking_is_blocked_by_time_off(provider_id, v_cur, v_slot_end) then
            -- create slot (ignore duplicates)
            insert into public.slots (agenda_id, start_at, end_at, status, capacity, tags)
            values (v_agenda_id, v_cur, v_slot_end, 'draft'::public.agenda_slot_status, v_capacity, array['booking'])
            on conflict (agenda_id, start_at, end_at) do update
              set capacity = excluded.capacity
            returning id into v_slot_id;

            -- link services
            if array_length(v_service_ids, 1) is not null then
              insert into public.booking_slot_services (provider_id, slot_id, service_id)
              select provider_id, v_slot_id::text, sid::uuid
              from unnest(v_service_ids) sid
              on conflict do nothing;
            end if;
          end if;

          v_cur := v_slot_end;
        end loop;
      end loop;
    end if;

    v_date := v_date + 1;
  end loop;
end;
$$;

-- 6) RPC : créer une réservation avec contrôle capacité + vacances
create or replace function public.booking_create_booking(
  provider_id uuid,
  service_id uuid,
  slot_id uuid,
  quantity int,
  customer_email text,
  customer_phone text
)
returns uuid
language plpgsql
security definer
set search_path = public
as $$
declare
  v_slot public.slots%rowtype;
  v_service public.booking_services%rowtype;
  v_capacity int;
  v_next_status public.booking_booking_status;
  v_booking_id uuid;
  v_customer_id uuid;
begin
  if not exists (select 1 from public.framework_modules fm where fm.module_id='booking' and fm.enabled=true) then
    raise exception 'booking module disabled';
  end if;

  if quantity is null or quantity < 1 then
    raise exception 'quantity must be >= 1';
  end if;

  select * into v_service from public.booking_services s where s.id = service_id and s.provider_id = provider_id and s.is_active = true;
  if not found then
    raise exception 'service not found or inactive';
  end if;

  select * into v_slot from public.slots sl
  join public.agendas a on a.id = sl.agenda_id
  where sl.id = slot_id
    and a.module_id = 'booking'
    and a.created_by = provider_id
    and sl.status <> 'cancelled'::public.agenda_slot_status;
  if not found then
    raise exception 'slot not found';
  end if;

  if public.booking_is_blocked_by_time_off(provider_id, v_slot.start_at, v_slot.end_at) then
    raise exception 'slot blocked by time off';
  end if;

  -- capacity check
  v_capacity := coalesce(v_slot.capacity, 1);
  if v_slot.participants_count + quantity > v_capacity then
    raise exception 'slot capacity exceeded';
  end if;

  -- resolve customer (optional)
  v_customer_id := auth.uid();
  if v_customer_id is null then
    if customer_email is null or length(trim(customer_email)) = 0 then
      raise exception 'customer_email required for anonymous booking';
    end if;
  end if;

  v_next_status := case when v_service.requires_approval then 'requested'::public.booking_booking_status else 'confirmed'::public.booking_booking_status end;

  insert into public.booking_bookings (
    provider_id, service_id, agenda_id, slot_id, customer_id, customer_email, customer_phone, status, quantity
  ) values (
    provider_id, service_id, v_slot.agenda_id::text, slot_id::text, v_customer_id, customer_email, customer_phone, v_next_status, quantity
  )
  returning id into v_booking_id;

  return v_booking_id;
end;
$$;

-- Permissions RPC
revoke all on function public.booking_generate_slots(uuid, uuid, date, date) from public;
grant execute on function public.booking_generate_slots(uuid, uuid, date, date) to authenticated;

revoke all on function public.booking_create_booking(uuid, uuid, uuid, int, text, text) from public;
grant execute on function public.booking_create_booking(uuid, uuid, uuid, int, text, text) to anon, authenticated;

commit;

