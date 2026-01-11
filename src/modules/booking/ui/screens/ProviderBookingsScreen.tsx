'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables } from '@/lib/supabase/database.types'
import { CalendarGrid } from '@/components/molecules/calendar-grid'
import { Modal } from '@/components/molecules/modal'
import { useRequireEnabledModule } from '../hooks/useRequireEnabledModule'
import { useBookingProvider } from '../hooks/useBookingProvider'

type BookingBooking = Tables<'booking_bookings'>
type BookingService = Tables<'booking_services'>
type Slot = Tables<'slots'>

export function ProviderBookingsScreen() {
  const { isLoading: modulesLoading, enabled } = useRequireEnabledModule('booking', '/')
  const supabase = useMemo(() => getSupabaseClient(), [])
  const { isAuthenticated, mustDenyProviderBackOffice, ensureProvider } = useBookingProvider()

  const [loading, setLoading] = useState(true)
  const [bookings, setBookings] = useState<BookingBooking[]>([])
  const [servicesById, setServicesById] = useState<Record<string, BookingService>>({})
  const [slotsById, setSlotsById] = useState<Record<string, Slot>>({})
  const [cancelReasonById, setCancelReasonById] = useState<Record<string, string>>({})
  const [rescheduleOpen, setRescheduleOpen] = useState(false)
  const [rescheduleBooking, setRescheduleBooking] = useState<BookingBooking | null>(null)
  const [rescheduleSlots, setRescheduleSlots] = useState<Slot[]>([])
  const [rescheduleViewMode, setRescheduleViewMode] = useState<'week' | 'day'>('week')
  const [rescheduleAnchorDate, setRescheduleAnchorDate] = useState<Date>(new Date())

  const load = useCallback(async () => {
    if (!enabled || !isAuthenticated || mustDenyProviderBackOffice) return
    setLoading(true)
    try {
      const prov = await ensureProvider()
      if (!prov) return
      const { data, error } = await supabase
        .from('booking_bookings')
        .select('*')
        .eq('provider_id', prov.id)
        .order('created_at', { ascending: false })
        .limit(200)
      if (error) throw error
      const list = data ?? []
      setBookings(list)

      // hydrate services
      const serviceIds = Array.from(new Set(list.map((b: BookingBooking) => b.service_id).filter(Boolean)))
      if (serviceIds.length) {
        const { data: sData, error: sErr } = await supabase.from('booking_services').select('*').in('id', serviceIds)
        if (sErr) throw sErr
        const map: Record<string, BookingService> = {}
        for (const s of (sData ?? []) as BookingService[]) map[s.id] = s
        setServicesById(map)
      } else {
        setServicesById({})
      }

      // hydrate slots for display
      const slotIds = Array.from(new Set(list.map((b: BookingBooking) => b.slot_id).filter(Boolean)))
      const uuidSlotIds = slotIds
        .map((x) => (typeof x === 'string' && /^[0-9a-fA-F-]{36}$/.test(x) ? x : null))
        .filter(Boolean) as string[]
      if (uuidSlotIds.length) {
        const { data: slData, error: slErr } = await supabase.from('slots').select('*').in('id', uuidSlotIds)
        if (slErr) throw slErr
        const map: Record<string, Slot> = {}
        for (const s of (slData ?? []) as Slot[]) map[s.id] = s
        setSlotsById(map)
      } else {
        setSlotsById({})
      }
    } finally {
      setLoading(false)
    }
  }, [enabled, ensureProvider, isAuthenticated, mustDenyProviderBackOffice, supabase])

  const openReschedule = async (b: BookingBooking) => {
    setRescheduleBooking(b)
    setRescheduleAnchorDate(new Date())
    setRescheduleViewMode('week')
    setRescheduleOpen(true)
    setLoading(true)
    try {
      const now = new Date()
      const end = new Date()
      end.setDate(end.getDate() + 21)
      const { data, error } = await supabase.rpc('booking_list_available_slots', {
        p_provider_id: b.provider_id,
        p_service_id: b.service_id,
        p_range_start: now.toISOString(),
        p_range_end: end.toISOString(),
        p_quantity: b.quantity,
      })
      if (error) throw error
      const list = ((data ?? []) as Slot[]).filter((s) => s.id !== (b.slot_id ?? ''))
      setRescheduleSlots(list)
    } finally {
      setLoading(false)
    }
  }

  const doReschedule = async (newSlotId: string) => {
    if (!rescheduleBooking) return
    setLoading(true)
    try {
      const { error } = await supabase.rpc('booking_reschedule_booking', {
        booking_id: rescheduleBooking.id,
        new_slot_id: newSlotId,
      })
      if (error) throw error
      setRescheduleOpen(false)
      setRescheduleBooking(null)
      setRescheduleSlots([])
      await load()
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    if (modulesLoading) return
    void load()
  }, [load, modulesLoading])

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
        <div className="text-base-content/70">Connecte-toi pour gérer tes réservations.</div>
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
      <ContentStack>
        <div className="flex items-start justify-between gap-4">
          <div>
            <h1 className="text-2xl font-bold text-base-content">Réservations</h1>
            <p className="text-base-content/60 mt-1">
              Liste des réservations (filtrées par RLS).
            </p>
          </div>
          <div className="flex gap-2">
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

        <Card className="mt-6">
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Liste</span>
            <div className="flex items-center gap-2">
              {loading && <span className="loading loading-spinner loading-sm text-primary" />}
              <Badge variant="neutral" style="soft" size="sm">
                {bookings.length}
              </Badge>
            </div>
          </CardHeader>
          <CardBody className="p-0">
            {loading ? (
              <div className="p-6 flex items-center justify-center">
                <span className="loading loading-spinner text-primary" />
              </div>
            ) : bookings.length === 0 ? (
              <div className="p-6 text-base-content/60">Aucune réservation pour l’instant.</div>
            ) : (
              <div className="overflow-x-auto">
                <table className="table table-zebra">
                  <thead>
                    <tr>
                      <th>Statut</th>
                      <th>Service</th>
                      <th>Client</th>
                      <th>Créneau</th>
                      <th>Quantité</th>
                      <th>Créé le</th>
                      <th>Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {bookings.map((b) => (
                      <tr key={b.id}>
                        <td>
                          <Badge
                            variant={
                              b.status === 'confirmed'
                                ? 'success'
                                : b.status === 'requested'
                                  ? 'warning'
                                  : b.status.startsWith('cancelled')
                                    ? 'error'
                                    : 'neutral'
                            }
                            size="sm"
                          >
                            {b.status}
                          </Badge>
                        </td>
                        <td className="text-base-content/70">
                          {servicesById[b.service_id]?.name ?? b.service_id}
                          {servicesById[b.service_id]?.requires_approval ? (
                            <span className="ml-2 badge badge-warning badge-soft badge-xs">Approval</span>
                          ) : null}
                        </td>
                        <td className="text-base-content/70">
                          {b.customer_email || b.customer_id || '-'}
                        </td>
                        <td className="text-base-content/70">
                          {b.slot_id && slotsById[b.slot_id]
                            ? new Date(slotsById[b.slot_id].start_at).toLocaleString('fr-FR')
                            : b.slot_id || '—'}
                        </td>
                        <td className="text-base-content/70">{b.quantity}</td>
                        <td className="text-base-content/60">
                          {new Date(b.created_at).toLocaleString('fr-FR')}
                        </td>
                        <td className="text-right">
                          <details className="dropdown dropdown-end">
                            <summary className="btn btn-ghost btn-xs">Gérer</summary>
                            <div className="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-80">
                              {b.status === 'requested' ? (
                                <button
                                  className="btn btn-primary btn-sm"
                                  disabled={loading}
                                  onClick={async () => {
                                    setLoading(true)
                                    try {
                                      const { error } = await supabase.rpc('booking_confirm_booking', { booking_id: b.id })
                                      if (error) throw error
                                      await load()
                                    } finally {
                                      setLoading(false)
                                    }
                                  }}
                                >
                                  Confirmer
                                </button>
                              ) : null}

                              <div className="mt-2">
                                <label className="label">
                                  <span className="label-text">Motif d’annulation (optionnel)</span>
                                </label>
                                <input
                                  className="input input-bordered input-sm w-full"
                                  value={cancelReasonById[b.id] ?? ''}
                                  onChange={(e) =>
                                    setCancelReasonById((prev) => ({ ...prev, [b.id]: e.target.value }))
                                  }
                                />
                              </div>

                              <button
                                className="btn btn-ghost btn-sm text-error mt-2"
                                disabled={loading}
                                onClick={async () => {
                                  setLoading(true)
                                  try {
                                    const { error } = await supabase.rpc('booking_cancel_booking', {
                                      booking_id: b.id,
                                      reason: cancelReasonById[b.id] ?? null,
                                    })
                                    if (error) throw error
                                    await load()
                                  } finally {
                                    setLoading(false)
                                  }
                                }}
                              >
                                Annuler
                              </button>

                              <button
                                className="btn btn-ghost btn-sm mt-2"
                                disabled={loading}
                                onClick={() => void openReschedule(b)}
                              >
                                Replanifier…
                              </button>
                            </div>
                          </details>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </CardBody>
        </Card>

        <Modal
          open={rescheduleOpen}
          title="Replanifier"
          onClose={() => {
            setRescheduleOpen(false)
            setRescheduleBooking(null)
            setRescheduleSlots([])
          }}
          footer={
            <div className="flex items-center justify-between">
              <div className="text-xs text-base-content/60">
                {rescheduleBooking ? `Booking ${rescheduleBooking.id}` : ''}
              </div>
              <button className="btn btn-ghost btn-sm" onClick={() => setRescheduleOpen(false)}>
                Fermer
              </button>
            </div>
          }
        >
          {!rescheduleBooking ? (
            <div className="text-base-content/60">Aucune réservation sélectionnée.</div>
          ) : (
            <div className="space-y-3">
              <div className="flex items-center justify-between gap-3">
                <div className="text-sm text-base-content/70">
                  Service: <span className="font-semibold">{servicesById[rescheduleBooking.service_id]?.name ?? rescheduleBooking.service_id}</span>
                </div>
                <div className="join">
                  <button
                    className={`btn btn-sm join-item ${rescheduleViewMode === 'week' ? 'btn-primary' : 'btn-ghost'}`}
                    onClick={() => setRescheduleViewMode('week')}
                  >
                    Semaine
                  </button>
                  <button
                    className={`btn btn-sm join-item ${rescheduleViewMode === 'day' ? 'btn-primary' : 'btn-ghost'}`}
                    onClick={() => setRescheduleViewMode('day')}
                  >
                    Jour
                  </button>
                </div>
              </div>

              <div className="flex items-center justify-between gap-2">
                <button
                  className="btn btn-ghost btn-sm"
                  onClick={() => setRescheduleAnchorDate((d) => new Date(d.getTime() - 7 * 24 * 60 * 60 * 1000))}
                >
                  <span className="icon-[tabler--chevron-left] size-4" />
                </button>
                <button className="btn btn-ghost btn-sm" onClick={() => setRescheduleAnchorDate(new Date())}>
                  Aujourd’hui
                </button>
                <button
                  className="btn btn-ghost btn-sm"
                  onClick={() => setRescheduleAnchorDate((d) => new Date(d.getTime() + 7 * 24 * 60 * 60 * 1000))}
                >
                  <span className="icon-[tabler--chevron-right] size-4" />
                </button>
              </div>

              <CalendarGrid
                mode={rescheduleViewMode}
                anchorDate={rescheduleAnchorDate}
                slots={rescheduleSlots.map((s) => ({ ...s, startAt: s.start_at, endAt: s.end_at }))}
                renderSlot={(s) => {
                  const slot = s as unknown as Slot & { startAt: string; endAt: string }
                  const cap = slot.capacity ?? 1
                  const remaining = cap - slot.participants_count
                  return (
                    <div className="rounded-lg border border-base-300 bg-base-100 p-2 hover:bg-base-200 transition-colors">
                      <div className="flex items-center justify-between">
                        <div className="text-xs font-semibold text-base-content">
                          {new Date(slot.start_at).toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' })}
                        </div>
                        <span className="text-[11px] text-base-content/60">
                          {remaining}/{cap}
                        </span>
                      </div>
                      <div className="mt-2">
                        <button className="btn btn-primary btn-xs" onClick={() => void doReschedule(slot.id)} disabled={loading}>
                          Choisir
                        </button>
                      </div>
                    </div>
                  )
                }}
              />
            </div>
          )}
        </Modal>
      </ContentStack>
    </AppShellScreen>
  )
}

