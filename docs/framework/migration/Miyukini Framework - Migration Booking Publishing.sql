-- Miyukini Framework - Migration Booking Publishing
-- Objectif :
-- - notion "publié" des créneaux via slots.status='confirmed'
-- - ne montrer aux clients que les slots confirmés
-- - générer des slots confirmés par défaut (semaine type)
-- - migrer les anciens slots booking draft -> confirmed
-- Date : 2026-01-10 (Europe/Paris)

begin;

-- 1) booking_list_available_slots : slots confirmés uniquement
create or replace function public.booking_list_available_slots(
  p_provider_id uuid,
  p_service_id uuid,
  p_range_start timestamptz,
  p_range_end timestamptz,
  p_quantity int default 1
)
returns setof public.slots
language sql
stable
security definer
set search_path = public
as $$
  select sl.*
  from public.slots sl
  join public.agendas a on a.id = sl.agenda_id
  join public.booking_services bs on bs.id = p_service_id
  join public.booking_slot_services bss
    on bss.provider_id = p_provider_id
   and bss.service_id = p_service_id
   and bss.slot_id = sl.id::text
  where a.module_id = 'booking'
    and a.created_by = p_provider_id
    and a.is_public = true
    and bs.provider_id = p_provider_id
    and bs.is_active = true
    and sl.status = 'confirmed'::public.agenda_slot_status
    and sl.start_at >= p_range_start
    and sl.start_at <= p_range_end
    and not public.booking_is_blocked_by_time_off(p_provider_id, sl.start_at, sl.end_at)
    and (sl.participants_count + greatest(p_quantity, 1)) <= coalesce(sl.capacity, 1)
  order by sl.start_at asc;
$$;

-- 2) booking_generate_slots : slots confirmés par défaut (publiés)
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
    v_dow := extract(isodow from v_date)::int;
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
          if v_slot_end > v_end then exit; end if;

          if not public.booking_is_blocked_by_time_off(provider_id, v_cur, v_slot_end) then
            insert into public.slots (agenda_id, start_at, end_at, status, capacity, tags)
            values (v_agenda_id, v_cur, v_slot_end, 'confirmed'::public.agenda_slot_status, v_capacity, array['booking'])
            on conflict (agenda_id, start_at, end_at) do update
              set capacity = excluded.capacity,
                  status = excluded.status
            returning id into v_slot_id;

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

-- 3) Migrer les anciens slots booking "draft" -> "confirmed" (pour ne pas casser l'existant)
update public.slots sl
set status = 'confirmed'::public.agenda_slot_status
where sl.status = 'draft'::public.agenda_slot_status
  and exists (
    select 1 from public.agendas a
    where a.id = sl.agenda_id
      and a.module_id = 'booking'
  );

commit;

