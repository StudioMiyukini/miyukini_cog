-- Miyukini Framework - Migration Booking Workflow
-- Objectif :
-- - filtrer les créneaux côté client par prestations autorisées (sans exposer booking_slot_services)
-- - ajouter workflow approval (confirm), annulation, replanification
-- - renforcer booking_create_booking (service doit être autorisé sur slot)
-- Date : 2026-01-10 (Europe/Paris)

begin;

-- 1) RPC: lister les créneaux disponibles (public) pour un prestataire + prestation
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
    and sl.status <> 'cancelled'::public.agenda_slot_status
    and sl.start_at >= p_range_start
    and sl.start_at <= p_range_end
    and not public.booking_is_blocked_by_time_off(p_provider_id, sl.start_at, sl.end_at)
    and (sl.participants_count + greatest(p_quantity, 1)) <= coalesce(sl.capacity, 1)
  order by sl.start_at asc;
$$;

revoke all on function public.booking_list_available_slots(uuid, uuid, timestamptz, timestamptz, int) from public;
grant execute on function public.booking_list_available_slots(uuid, uuid, timestamptz, timestamptz, int) to anon, authenticated;

-- 2) Renforcer booking_create_booking: service doit être autorisé sur slot
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

  select * into v_service
  from public.booking_services s
  where s.id = service_id and s.provider_id = provider_id and s.is_active = true;
  if not found then
    raise exception 'service not found or inactive';
  end if;

  -- slot must belong to provider booking agenda
  select sl.* into v_slot
  from public.slots sl
  join public.agendas a on a.id = sl.agenda_id
  where sl.id = slot_id
    and a.module_id = 'booking'
    and a.created_by = provider_id
    and sl.status <> 'cancelled'::public.agenda_slot_status;
  if not found then
    raise exception 'slot not found';
  end if;

  -- service must be allowed on this slot
  if not exists (
    select 1 from public.booking_slot_services bss
    where bss.provider_id = provider_id
      and bss.service_id = service_id
      and bss.slot_id = slot_id::text
  ) then
    raise exception 'service not allowed on this slot';
  end if;

  if public.booking_is_blocked_by_time_off(provider_id, v_slot.start_at, v_slot.end_at) then
    raise exception 'slot blocked by time off';
  end if;

  v_capacity := coalesce(v_slot.capacity, 1);
  if v_slot.participants_count + quantity > v_capacity then
    raise exception 'slot capacity exceeded';
  end if;

  v_customer_id := auth.uid();
  if v_customer_id is null then
    if customer_email is null or length(trim(customer_email)) = 0 then
      raise exception 'customer_email required for anonymous booking';
    end if;
  end if;

  v_next_status :=
    case when v_service.requires_approval
      then 'requested'::public.booking_booking_status
      else 'confirmed'::public.booking_booking_status
    end;

  insert into public.booking_bookings (
    provider_id, service_id, agenda_id, slot_id, customer_id, customer_email, customer_phone, status, quantity
  ) values (
    provider_id, service_id, v_slot.agenda_id::text, slot_id::text, v_customer_id, customer_email, customer_phone, v_next_status, quantity
  )
  returning id into v_booking_id;

  return v_booking_id;
end;
$$;

-- 3) RPC: confirmer une réservation (workflow approval)
create or replace function public.booking_confirm_booking(booking_id uuid)
returns uuid
language plpgsql
security definer
set search_path = public
as $$
declare
  v_booking public.booking_bookings%rowtype;
begin
  select * into v_booking from public.booking_bookings b where b.id = booking_id;
  if not found then raise exception 'booking not found'; end if;

  if not (auth.uid() = v_booking.provider_id or public.is_super_admin()) then
    raise exception 'not allowed';
  end if;

  if v_booking.status <> 'requested'::public.booking_booking_status then
    raise exception 'booking not in requested status';
  end if;

  update public.booking_bookings
  set status = 'confirmed'::public.booking_booking_status
  where id = booking_id;

  return booking_id;
end;
$$;

revoke all on function public.booking_confirm_booking(uuid) from public;
grant execute on function public.booking_confirm_booking(uuid) to authenticated;

-- 4) RPC: annuler une réservation (prestataire ou client connecté)
create or replace function public.booking_cancel_booking(booking_id uuid, reason text)
returns uuid
language plpgsql
security definer
set search_path = public
as $$
declare
  v_booking public.booking_bookings%rowtype;
  v_new_status public.booking_booking_status;
begin
  select * into v_booking from public.booking_bookings b where b.id = booking_id;
  if not found then raise exception 'booking not found'; end if;

  if auth.uid() is null then
    raise exception 'authentication required';
  end if;

  if auth.uid() = v_booking.provider_id or public.is_super_admin() then
    v_new_status := 'cancelled_by_provider'::public.booking_booking_status;
  elsif v_booking.customer_id is not null and auth.uid() = v_booking.customer_id then
    v_new_status := 'cancelled_by_client'::public.booking_booking_status;
  else
    raise exception 'not allowed';
  end if;

  update public.booking_bookings
  set status = v_new_status,
      cancel_reason = nullif(trim(reason), '')
  where id = booking_id;

  return booking_id;
end;
$$;

revoke all on function public.booking_cancel_booking(uuid, text) from public;
grant execute on function public.booking_cancel_booking(uuid, text) to authenticated;

-- 5) RPC: replanifier une réservation (prestataire ou client connecté)
create or replace function public.booking_reschedule_booking(booking_id uuid, new_slot_id uuid)
returns uuid
language plpgsql
security definer
set search_path = public
as $$
declare
  v_booking public.booking_bookings%rowtype;
  v_slot public.slots%rowtype;
  v_capacity int;
begin
  select * into v_booking from public.booking_bookings b where b.id = booking_id;
  if not found then raise exception 'booking not found'; end if;

  if auth.uid() is null then
    raise exception 'authentication required';
  end if;

  if not (auth.uid() = v_booking.provider_id or public.is_super_admin() or (v_booking.customer_id is not null and auth.uid() = v_booking.customer_id)) then
    raise exception 'not allowed';
  end if;

  -- slot must belong to provider booking agenda
  select sl.* into v_slot
  from public.slots sl
  join public.agendas a on a.id = sl.agenda_id
  where sl.id = new_slot_id
    and a.module_id = 'booking'
    and a.created_by = v_booking.provider_id
    and sl.status <> 'cancelled'::public.agenda_slot_status;
  if not found then
    raise exception 'slot not found';
  end if;

  -- service must be allowed on this slot
  if not exists (
    select 1 from public.booking_slot_services bss
    where bss.provider_id = v_booking.provider_id
      and bss.service_id = v_booking.service_id
      and bss.slot_id = new_slot_id::text
  ) then
    raise exception 'service not allowed on this slot';
  end if;

  if public.booking_is_blocked_by_time_off(v_booking.provider_id, v_slot.start_at, v_slot.end_at) then
    raise exception 'slot blocked by time off';
  end if;

  v_capacity := coalesce(v_slot.capacity, 1);
  if v_slot.participants_count + v_booking.quantity > v_capacity then
    raise exception 'slot capacity exceeded';
  end if;

  update public.booking_bookings
  set slot_id = new_slot_id::text,
      agenda_id = v_slot.agenda_id::text
  where id = booking_id;

  return booking_id;
end;
$$;

revoke all on function public.booking_reschedule_booking(uuid, uuid) from public;
grant execute on function public.booking_reschedule_booking(uuid, uuid) to authenticated;

commit;

