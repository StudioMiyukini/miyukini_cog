'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { CalendarGrid } from '@/components/molecules/calendar-grid'
import { Modal } from '@/components/molecules/modal'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables } from '@/lib/supabase/database.types'
import { useRequireEnabledModule } from '../hooks/useRequireEnabledModule'
import { useBookingProvider } from '../hooks/useBookingProvider'

type Agenda = Tables<'agendas'>
type Slot = Tables<'slots'>
type BookingService = Tables<'booking_services'>
type BookingSlotService = Tables<'booking_slot_services'>
type BookingBooking = Tables<'booking_bookings'>

export function ProviderPlanningScreen() {
  const { isLoading: modulesLoading, enabled } = useRequireEnabledModule('booking', '/')
  const supabase = useMemo(() => getSupabaseClient(), [])
  const { isAuthenticated, mustDenyProviderBackOffice, ensureProvider } = useBookingProvider()

  const [loading, setLoading] = useState(true)
  const [agenda, setAgenda] = useState<Agenda | null>(null)
  const [slots, setSlots] = useState<Slot[]>([])
  const [services, setServices] = useState<BookingService[]>([])
  const [slotServices, setSlotServices] = useState<Record<string, string[]>>({})
  const [slotBookings, setSlotBookings] = useState<
    Record<
      string,
      Array<
        Pick<BookingBooking, 'id' | 'status' | 'quantity' | 'customer_id' | 'customer_email'> & {
          customer?: { first_name: string | null; last_name: string | null; display_name: string | null } | null
        }
      >
    >
  >({})

  const [startAt, setStartAt] = useState('')
  const [durationMinutes, setDurationMinutes] = useState<number>(30)
  const [capacity, setCapacity] = useState<number>(1)
  const [selectedServiceIds, setSelectedServiceIds] = useState<string[]>([])
  const [viewMode, setViewMode] = useState<'week' | '3days' | 'day'>('week')
  const [anchorDate, setAnchorDate] = useState<Date>(new Date())
  const [editOpen, setEditOpen] = useState(false)
  const [editSlotId, setEditSlotId] = useState<string | null>(null)
  const [editCapacity, setEditCapacity] = useState<number>(1)
  const [editStatus, setEditStatus] = useState<'confirmed' | 'draft' | 'cancelled'>('confirmed')
  const [editDurationMinutes, setEditDurationMinutes] = useState<number>(60)
  const [editServiceIds, setEditServiceIds] = useState<string[]>([])

  const ensureBookingAgenda = useCallback(
    async (providerId: string) => {
      // NOTE: on évite maybeSingle() ici car on peut avoir des doublons historiques
      // (PostgREST renvoie alors PGRST116). On prend l'agenda le plus récent.
      const { data: existingRows, error } = await supabase
        .from('agendas')
        .select('*')
        .eq('module_id', 'booking')
        .eq('created_by', providerId)
        // On privilégie un agenda déjà public (sinon l'agenda public ne voit aucun créneau)
        .order('is_public', { ascending: false })
        .order('created_at', { ascending: false })
        .limit(2)
      if (error) throw error
      if ((existingRows?.length ?? 0) > 1) {
        console.warn(
          '[Booking] Plusieurs agendas trouvés pour ce prestataire. Utilisation du plus récent.',
          { providerId, agendaIds: existingRows?.map((a: any) => a.id) }
        )
      }
      if (existingRows?.[0]) {
        const picked = existingRows[0] as any
        // MVP: l'agenda Booking doit être public pour exposer les créneaux au client
        if (picked.is_public !== true) {
          const { data: updated, error: upErr } = await supabase
            .from('agendas')
            .update({ is_public: true, updated_at: new Date().toISOString() })
            .eq('id', picked.id)
            .select('*')
            .single()
          if (upErr) throw upErr
          return updated as any
        }
        return picked
      }

      const payload = {
        module_id: 'booking',
        name: 'Agenda Booking',
        description: 'Agenda prestataire (Booking)',
        created_by: providerId,
        timezone: 'Europe/Paris',
        // MVP: agenda public (sinon aucun créneau ne remonte côté /booking)
        is_public: true,
        allow_overbooking: false,
      }
      const { data: created, error: insertError } = await supabase.from('agendas').insert(payload).select('*').single()
      if (insertError) throw insertError
      return created
    },
    [supabase]
  )

  const load = useCallback(async () => {
    if (!enabled || !isAuthenticated || mustDenyProviderBackOffice) return
    setLoading(true)
    try {
      const prov = await ensureProvider()
      if (!prov) return

      const a = await ensureBookingAgenda(prov.id)
      setAgenda(a)

      const { data: servicesData, error: servicesError } = await supabase
        .from('booking_services')
        .select('*')
        .eq('provider_id', prov.id)
        .order('created_at', { ascending: false })
      if (servicesError) throw servicesError
      setServices(servicesData ?? [])

      const rangeStart = new Date()
      rangeStart.setHours(0, 0, 0, 0)
      const rangeEnd = new Date(rangeStart)
      rangeEnd.setDate(rangeEnd.getDate() + 30)

      const { data: slotsData, error: slotsError } = await supabase
        .from('slots')
        .select('*')
        .eq('agenda_id', a.id)
        .gte('start_at', rangeStart.toISOString())
        .lte('start_at', rangeEnd.toISOString())
        .order('start_at', { ascending: true })
      if (slotsError) throw slotsError
      const slotsList: Slot[] = slotsData ?? []
      setSlots(slotsList)

      const slotIds = slotsList.map((s: Slot) => s.id)
      if (slotIds.length === 0) {
        setSlotServices({})
        setSlotBookings({})
        return
      }
      const { data: ssData, error: ssError } = await supabase
        .from('booking_slot_services')
        .select('*')
        .eq('provider_id', prov.id)
        .in('slot_id', slotIds)
      if (ssError) throw ssError

      const mapping: Record<string, string[]> = {}
      for (const row of (ssData ?? []) as BookingSlotService[]) {
        mapping[row.slot_id] ??= []
        mapping[row.slot_id].push(row.service_id)
      }
      setSlotServices(mapping)

      // Charger les bookings pour afficher "Réservé + Prénom Nom"
      const { data: bbData, error: bbError } = await supabase
        .from('booking_bookings')
        .select('id,status,quantity,customer_id,customer_email,slot_id, customer:profiles(first_name,last_name,display_name,phone)')
        .eq('provider_id', prov.id)
        .in('slot_id', slotIds)
        .in('status', ['requested', 'confirmed'])
      if (bbError) throw bbError
      const bySlot: Record<string, any[]> = {}
      for (const row of (bbData ?? []) as any[]) {
        const sid = row.slot_id as string
        bySlot[sid] ??= []
        bySlot[sid].push(row)
      }
      setSlotBookings(bySlot as any)
    } finally {
      setLoading(false)
    }
  }, [enabled, ensureBookingAgenda, ensureProvider, isAuthenticated, mustDenyProviderBackOffice, supabase])

  useEffect(() => {
    if (modulesLoading) return
    void load()
  }, [load, modulesLoading])

  const createSlot = async () => {
    if (!agenda) return
    if (!startAt) return
    if (selectedServiceIds.length === 0) return

    const start = new Date(startAt)
    const end = new Date(start)
    end.setMinutes(end.getMinutes() + durationMinutes)

    setLoading(true)
    try {
      const { data: created, error } = await supabase
        .from('slots')
        .insert({
          agenda_id: agenda.id,
          start_at: start.toISOString(),
          end_at: end.toISOString(),
          status: 'confirmed',
          capacity: capacity,
          tags: ['booking'],
        })
        .select('*')
        .single()
      if (error) throw error

      // relier prestations au créneau
      const prov = await ensureProvider()
      if (!prov) return
      const rows = selectedServiceIds.map((sid) => ({
        provider_id: prov.id,
        slot_id: created.id,
        service_id: sid,
      }))
      const { error: linkError } = await supabase.from('booking_slot_services').insert(rows)
      if (linkError) throw linkError

      setStartAt('')
      setDurationMinutes(30)
      setCapacity(1)
      setSelectedServiceIds([])
      await load()
    } finally {
      setLoading(false)
    }
  }

  const updateAllowedServices = async (slotId: string, nextServiceIds: string[]) => {
    const prov = await ensureProvider()
    if (!prov) return
    setLoading(true)
    try {
      const { error: delErr } = await supabase
        .from('booking_slot_services')
        .delete()
        .eq('provider_id', prov.id)
        .eq('slot_id', slotId)
      if (delErr) throw delErr

      if (nextServiceIds.length > 0) {
        const rows = nextServiceIds.map((sid) => ({ provider_id: prov.id, slot_id: slotId, service_id: sid }))
        const { error: insErr } = await supabase.from('booking_slot_services').insert(rows)
        if (insErr) throw insErr
      }

      await load()
    } finally {
      setLoading(false)
    }
  }

  const setSlotStatus = async (slotId: string, nextStatus: 'confirmed' | 'draft' | 'cancelled') => {
    setLoading(true)
    try {
      const { error } = await supabase.from('slots').update({ status: nextStatus }).eq('id', slotId)
      if (error) throw error
      await load()
    } finally {
      setLoading(false)
    }
  }

  const openEditor = (slotId: string) => {
    const slot = slots.find((s) => s.id === slotId)
    if (!slot) return
    setEditSlotId(slotId)
    setEditCapacity(slot.capacity ?? 1)
    setEditStatus(slot.status as any)
    setEditDurationMinutes(
      Math.max(5, Math.round((new Date(slot.end_at).getTime() - new Date(slot.start_at).getTime()) / 60000))
    )
    setEditServiceIds(slotServices[slotId] ?? [])
    setEditOpen(true)
  }

  const saveEditor = async () => {
    if (!editSlotId) return
    setLoading(true)
    try {
      const slot = slots.find((s) => s.id === editSlotId)
      if (!slot) return
      const start = new Date(slot.start_at)
      const nextEnd = new Date(start.getTime() + Math.max(5, editDurationMinutes) * 60000)
      const { error } = await supabase
        .from('slots')
        .update({ capacity: editCapacity, status: editStatus, end_at: nextEnd.toISOString() })
        .eq('id', editSlotId)
      if (error) throw error
      await updateAllowedServices(editSlotId, editServiceIds)
      setEditOpen(false)
      setEditSlotId(null)
    } finally {
      setLoading(false)
    }
  }

  const cancelEdit = () => {
    setEditOpen(false)
    setEditSlotId(null)
  }

  const deleteSlot = async () => {
    if (!editSlotId) return
    setLoading(true)
    try {
      const { error } = await supabase.from('slots').delete().eq('id', editSlotId)
      if (error) throw error
      await load()
      setEditOpen(false)
      setEditSlotId(null)
    } finally {
      setLoading(false)
    }
  }

  const cancelReservation = async () => {
    if (!editSlotId) return
    setLoading(true)
    try {
      const { data, error } = await supabase.rpc('booking_cancel_reservations_for_slot', { p_slot_id: editSlotId })
      if (error) throw error
      console.info('[Booking] Réservations annulées:', data)
      await load()
    } finally {
      setLoading(false)
    }
  }

  const serviceLabel = useMemo(() => {
    const map = new Map<string, string>()
    for (const s of services) map.set(s.id, s.name)
    return map
  }, [services])

  if (modulesLoading) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }

  if (!enabled) return null

  if (!isAuthenticated) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <div className="text-base-content/70">Connecte-toi pour gérer tes créneaux.</div>
      </div>
    )
  }

  if (mustDenyProviderBackOffice) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <div className="text-base-content/70">
          L’espace prestataire est réservé aux prestataires (pas aux admins plateforme).
        </div>
      </div>
    )
  }

  return (
    <AppShellScreen>
      <ContentStack className="max-w-none">
        <div className="mx-auto w-full max-w-none space-y-4 px-2 lg:px-6">
          <div className="flex flex-col gap-3 rounded-3xl border border-base-200 bg-base-100 p-4 shadow-lg md:flex-row md:items-center md:justify-between">
            <div>
              <h1 className="text-2xl font-bold text-base-content mb-1">Planning</h1>
              <p className="text-base-content/60 text-sm">
                Agenda Booking avec aperçu rapide de vos réservations (mobile friendly).
              </p>
            </div>
            <div className="flex flex-wrap gap-2">
              <button className="btn btn-ghost btn-sm" onClick={() => void load()} disabled={loading}>
                <span className="icon-[tabler--refresh] size-4" />
                Rafraîchir
              </button>
              <Link href="/pro/booking" className="btn btn-ghost btn-sm">
                <span className="icon-[tabler--arrow-left] size-4" />
                Retour
              </Link>
            </div>
          </div>

          <Card className="mx-auto w-full max-w-none shadow-xl">
            <CardHeader className="flex flex-col gap-2 md:flex-row md:items-center md:justify-between">
              <div className="flex items-center gap-3">
                <button
                  className="btn btn-ghost btn-xs"
                  onClick={() => {
                    const stepDays = viewMode === 'week' ? 7 : viewMode === '3days' ? 3 : 1
                    setAnchorDate((d) => new Date(d.getTime() - stepDays * 24 * 60 * 60 * 1000))
                  }}
                >
                  <span className="icon-[tabler--chevron-left] size-4" />
                </button>
                <span className="text-base-content/80 text-sm font-semibold">
                  {anchorDate.toLocaleDateString('fr-FR', { weekday: 'short', day: '2-digit', month: 'short' })}
                </span>
                <button className="btn btn-ghost btn-xs" onClick={() => setAnchorDate(new Date())}>
                  Aujourd’hui
                </button>
                <button
                  className="btn btn-ghost btn-xs"
                  onClick={() => {
                    const stepDays = viewMode === 'week' ? 7 : viewMode === '3days' ? 3 : 1
                    setAnchorDate((d) => new Date(d.getTime() + stepDays * 24 * 60 * 60 * 1000))
                  }}
                >
                  <span className="icon-[tabler--chevron-right] size-4" />
                </button>
              </div>
              <div className="flex gap-1 text-xs font-semibold uppercase">
                <button
                  className={`btn btn-sm btn-outline ${viewMode === 'week' ? 'btn-primary' : 'btn-ghost'}`}
                  onClick={() => setViewMode('week')}
                >
                  Semaine
                </button>
                <button
                  className={`btn btn-sm btn-outline ${viewMode === '3days' ? 'btn-primary' : 'btn-ghost'}`}
                  onClick={() => setViewMode('3days')}
                >
                  3 jours
                </button>
                <button
                  className={`btn btn-sm btn-outline ${viewMode === 'day' ? 'btn-primary' : 'btn-ghost'}`}
                  onClick={() => setViewMode('day')}
                >
                  Jour
                </button>
              </div>
            </CardHeader>
            <CardBody className="px-0">
              <div className="overflow-hidden rounded-2xl border border-base-200 bg-base-50 shadow-inner">
                <CalendarGrid
                  mode={viewMode}
                  anchorDate={anchorDate}
                  slots={slots.map((s) => ({ ...s, startAt: s.start_at, endAt: s.end_at }))}
                  renderSlot={(s) => {
                    const slot = s as unknown as Slot & { startAt: string; endAt: string }
                    const isReserved = slot.participants_count > 0
                    const booking = (slotBookings[slot.id]?.[0] as any) ?? null
                    const customerName = booking?.customer
                      ? `${booking.customer.first_name ?? ''} ${booking.customer.last_name ?? ''}`.trim() || booking.customer.display_name
                      : null
                    return (
                      <div
                        className={[
                          'rounded-lg border border-base-200 p-1.5 md:p-2 transition-colors',
                          isReserved ? 'bg-blue-500/70 hover:bg-blue-500/80' : 'bg-green-500/70 hover:bg-green-500/80',
                        ].join(' ')}
                      >
                        {/* Desktop: heure + badge. Mobile: on laisse la couleur faire le job */}
                        <div className="hidden md:flex items-center justify-between text-[11px] font-semibold text-base-content">
                          <span>
                            {new Date(slot.start_at).toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' })}
                          </span>
                        </div>

                        <div className="hidden md:flex mt-1 items-center gap-2 text-xs">
                          <Badge variant={isReserved ? 'info' : 'success'} style="soft" size="xs">
                            {isReserved ? 'Réservé' : 'Libre'}
                          </Badge>
                          {isReserved && customerName && (
                            <span className="text-xs text-base-content/80 truncate">{customerName}</span>
                          )}
                        </div>

                        <div className="md:hidden">
                          <span className="sr-only">{isReserved ? 'Réservé' : 'Libre'}</span>
                          {isReserved && customerName ? (
                            <span className="block truncate text-[11px] font-semibold text-base-content/90">{customerName}</span>
                          ) : (
                            <span className="block h-4" />
                          )}
                        </div>
                      </div>
                    )
                  }}
                  onSlotClick={(s) => {
                    const slot = s as unknown as Slot & { startAt: string; endAt: string }
                    openEditor(slot.id)
                  }}
                />
              </div>
            </CardBody>
          </Card>

        <Modal
          open={editOpen}
          title="Éditer le créneau"
          onClose={() => {
            cancelEdit()
          }}
          footer={
            <div className="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
              <div className="flex flex-wrap items-center gap-2">
                <button className="btn btn-error btn-sm" onClick={() => void deleteSlot()} disabled={loading || !editSlotId}>
                  Supprimer
                </button>
                {(() => {
                  const slot = slots.find((s) => s.id === editSlotId)
                  const occupied = (slot?.participants_count ?? 0) > 0
                  if (!occupied) return null
                  return (
                    <button
                      className="btn btn-warning btn-sm"
                      onClick={() => void cancelReservation()}
                      disabled={loading || !editSlotId}
                    >
                      Annuler réservation
                    </button>
                  )
                })()}
              </div>
              <div className="flex flex-wrap items-center justify-end gap-2">
                <button className="btn btn-ghost btn-sm" onClick={() => cancelEdit()} disabled={loading}>
                  Annuler
                </button>
                <button className="btn btn-primary btn-sm" onClick={() => void saveEditor()} disabled={loading || !editSlotId}>
                  Enregistrer
                </button>
              </div>
            </div>
          }
        >
          {!editSlotId ? (
            <div className="text-base-content/60">Aucun créneau sélectionné.</div>
          ) : (
            <div className="space-y-4">
              {(() => {
                const slot = slots.find((s) => s.id === editSlotId)
                const booking = slot ? (slotBookings[slot.id]?.[0] as any) : null
                if (!booking) return null
                const customerName = booking.customer
                  ? `${booking.customer.first_name ?? ''} ${booking.customer.last_name ?? ''}`.trim() || booking.customer.display_name
                  : booking.customer_email
                return (
                  <div className="space-y-1 border border-base-300 rounded-2xl bg-base-100 p-4">
                    <div className="text-xs text-base-content/60 uppercase tracking-[0.2em]">Réservation</div>
                    <div className="text-xl font-semibold text-base-content leading-snug">
                      {customerName ?? 'Nom inconnu'}
                    </div>
                    <div className="flex flex-wrap gap-2 text-xs text-base-content/70">
                      <Badge variant="info" style="soft" size="xs">
                        {booking.status === 'requested' ? 'En attente' : 'Confirmée'}
                      </Badge>
                      <span>{booking.quantity} personne{booking.quantity > 1 ? 's' : ''}</span>
                      {booking.customer_email && <span>📧 {booking.customer_email}</span>}
                      {booking.customer?.phone && <span>📞 {booking.customer.phone}</span>}
                    </div>
                  </div>
                )
              })()}

              <div className="space-y-3 border border-base-300 rounded-2xl bg-base-100 p-4">
                <div className="text-sm font-medium text-base-content">Prestations autorisées</div>
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-2">
                  {services
                    .filter((sv) => sv.is_active)
                    .map((sv) => (
                      <label key={sv.id} className="label cursor-pointer justify-start gap-3 py-1">
                        <input
                          type="checkbox"
                          className="checkbox checkbox-primary"
                          checked={editServiceIds.includes(sv.id)}
                          onChange={(e) => {
                            setEditServiceIds((prev) =>
                              e.target.checked ? Array.from(new Set([...prev, sv.id])) : prev.filter((x) => x !== sv.id)
                            )
                          }}
                        />
                        <span className="label-text">{sv.name}</span>
                      </label>
                    ))}
                </div>
              </div>

              <div className="grid grid-cols-1 gap-3">
                <div className="grid grid-cols-1 md:grid-cols-4 gap-3">
                  <div className="md:col-span-2">
                    <div className="text-sm font-medium text-base-content">Créneau</div>
                    <div className="text-sm text-base-content/70 mt-1">
                      {new Date(slots.find((s) => s.id === editSlotId)?.start_at ?? '').toLocaleString('fr-FR')}
                    </div>
                  </div>
                  <div>
                    <label className="label">
                      <span className="label-text">Capacité</span>
                    </label>
                    <input
                      type="number"
                      className="input input-bordered w-full"
                      min={1}
                      value={editCapacity}
                      onChange={(e) => setEditCapacity(Number(e.target.value))}
                    />
                  </div>
                  <div>
                    <label className="label">
                      <span className="label-text">Durée (min)</span>
                    </label>
                    <input
                      type="number"
                      className="input input-bordered w-full"
                      min={5}
                      step={5}
                      value={editDurationMinutes}
                      onChange={(e) => setEditDurationMinutes(Number(e.target.value))}
                    />
                  </div>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                  <div>
                    <label className="label">
                      <span className="label-text">Publication</span>
                    </label>
                    <select
                      className="select select-bordered w-full"
                      value={editStatus}
                      onChange={(e) => setEditStatus(e.target.value as any)}
                    >
                      <option value="confirmed">Publié</option>
                      <option value="draft">Masqué</option>
                      <option value="cancelled">Annulé</option>
                    </select>
                  </div>
                  <div className="text-sm text-base-content/60 flex items-center">
                    Visible côté client uniquement si “Publié”.
                  </div>
                </div>
              </div>
            </div>
          )}
        </Modal>

        <Card className="mt-6">
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Créer un créneau</span>
            {loading && <span className="loading loading-spinner loading-sm text-primary" />}
          </CardHeader>
          <CardBody className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
              <div>
                <label className="label">
                  <span className="label-text">Début</span>
                </label>
                <input
                  type="datetime-local"
                  className="input input-bordered w-full"
                  value={startAt}
                  onChange={(e) => setStartAt(e.target.value)}
                />
              </div>
              <div>
                <label className="label">
                  <span className="label-text">Durée (min)</span>
                </label>
                <input
                  type="number"
                  className="input input-bordered w-full"
                  value={durationMinutes}
                  min={5}
                  step={5}
                  onChange={(e) => setDurationMinutes(Number(e.target.value))}
                />
              </div>
              <div>
                <label className="label">
                  <span className="label-text">Capacité</span>
                </label>
                <input
                  type="number"
                  className="input input-bordered w-full"
                  value={capacity}
                  min={1}
                  step={1}
                  onChange={(e) => setCapacity(Number(e.target.value))}
                />
              </div>
            </div>

            <div>
              <div className="flex items-center justify-between">
                <span className="font-medium text-base-content">Prestations disponibles</span>
                <Badge variant="neutral" style="soft" size="sm">
                  {selectedServiceIds.length}
                </Badge>
              </div>
              {services.length === 0 ? (
                <div className="text-sm text-base-content/60 mt-2">
                  Ajoute d’abord des prestations dans <Link className="link" href="/pro/booking/services">Prestations</Link>.
                </div>
              ) : (
                <div className="mt-2 grid grid-cols-1 sm:grid-cols-2 gap-2">
                  {services
                    .filter((s) => s.is_active)
                    .map((s) => (
                      <label key={s.id} className="label cursor-pointer justify-start gap-3">
                        <input
                          type="checkbox"
                          className="checkbox checkbox-primary"
                          checked={selectedServiceIds.includes(s.id)}
                          onChange={(e) => {
                            setSelectedServiceIds((prev) =>
                              e.target.checked ? Array.from(new Set([...prev, s.id])) : prev.filter((x) => x !== s.id)
                            )
                          }}
                        />
                        <span className="label-text">{s.name}</span>
                      </label>
                    ))}
                </div>
              )}
            </div>

            <button
              className="btn btn-primary btn-sm"
              onClick={createSlot}
              disabled={loading || !agenda || !startAt || selectedServiceIds.length === 0}
            >
              <span className="icon-[tabler--plus] size-4" />
              Créer le créneau
            </button>
          </CardBody>
        </Card>

        <Card className="mt-6">
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Créneaux (30 jours)</span>
            <Badge variant="neutral" style="soft" size="sm">
              {slots.length}
            </Badge>
          </CardHeader>
          <CardBody className="p-0">
            {loading ? (
              <div className="p-6 flex items-center justify-center">
                <span className="loading loading-spinner text-primary" />
              </div>
            ) : slots.length === 0 ? (
              <div className="p-6 text-base-content/60">Aucun créneau.</div>
            ) : (
              <div className="overflow-x-auto">
                <table className="table table-zebra">
                  <thead>
                    <tr>
                      <th>Date</th>
                      <th>Durée</th>
                      <th>Capacité</th>
                      <th>Participants</th>
                      <th>Statut</th>
                      <th>Prestations</th>
                      <th>Publication</th>
                    </tr>
                  </thead>
                  <tbody>
                    {slots.map((s) => {
                      const allowed = slotServices[s.id] ?? []
                      const label = allowed.map((id) => serviceLabel.get(id) ?? id)
                      const isOccupied = s.participants_count > 0
                      return (
                        <tr key={s.id}>
                          <td className="text-base-content">
                            {new Date(s.start_at).toLocaleString('fr-FR')}
                          </td>
                          <td className="text-base-content/70">
                            {Math.round((new Date(s.end_at).getTime() - new Date(s.start_at).getTime()) / 60000)} min
                          </td>
                          <td className="text-base-content/70">{s.capacity ?? '-'}</td>
                          <td className="text-base-content/70">
                            <div className="flex items-center gap-2">
                              <span>{s.participants_count}</span>
                              <Badge
                                variant={isOccupied ? 'info' : 'success'}
                                style="soft"
                                size="xs"
                              >
                                {isOccupied ? 'Occupé' : 'Libre'}
                              </Badge>
                            </div>
                          </td>
                          <td>
                            <Badge
                              variant={s.status === 'confirmed' ? 'success' : s.status === 'pending' ? 'warning' : 'neutral'}
                              size="sm"
                            >
                              {s.status}
                            </Badge>
                          </td>
                          <td className="text-base-content/70">
                            <div className="space-y-2">
                              <div className="text-xs">{label.length ? label.join(', ') : '—'}</div>
                              <details>
                                <summary className="cursor-pointer text-xs link">Modifier</summary>
                                <div className="mt-2 grid grid-cols-1 sm:grid-cols-2 gap-2">
                                  {services
                                    .filter((sv) => sv.is_active)
                                    .map((sv) => {
                                      const checked = allowed.includes(sv.id)
                                      return (
                                        <label key={sv.id} className="label cursor-pointer justify-start gap-3 py-1">
                                          <input
                                            type="checkbox"
                                            className="checkbox checkbox-xs checkbox-primary"
                                            checked={checked}
                                            onChange={(e) => {
                                              const next = e.target.checked
                                                ? Array.from(new Set([...allowed, sv.id]))
                                                : allowed.filter((x) => x !== sv.id)
                                              void updateAllowedServices(s.id, next)
                                            }}
                                            disabled={loading}
                                          />
                                          <span className="label-text text-xs">{sv.name}</span>
                                        </label>
                                      )
                                    })}
                                </div>
                              </details>
                            </div>
                          </td>
                          <td className="text-right">
                            <div className="flex items-center justify-end gap-2">
                              <button
                                className={`btn btn-xs ${s.status === 'confirmed' ? 'btn-primary' : 'btn-ghost'}`}
                                disabled={loading}
                                onClick={() => void setSlotStatus(s.id, 'confirmed')}
                              >
                                Publier
                              </button>
                              <button
                                className={`btn btn-xs ${s.status === 'draft' ? 'btn-primary' : 'btn-ghost'}`}
                                disabled={loading}
                                onClick={() => void setSlotStatus(s.id, 'draft')}
                              >
                                Masquer
                              </button>
                              <button
                                className="btn btn-xs btn-ghost text-error"
                                disabled={loading}
                                onClick={() => void setSlotStatus(s.id, 'cancelled')}
                              >
                                Annuler
                              </button>
                            </div>
                          </td>
                        </tr>
                      )
                    })}
                  </tbody>
                </table>
              </div>
            )}
          </CardBody>
        </Card>
        </div>
      </ContentStack>
    </AppShellScreen>
  )
}

