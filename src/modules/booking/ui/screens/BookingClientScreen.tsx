'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { BottomNav } from '@/components/navigation/BottomNav'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { CalendarGrid } from '@/components/molecules/calendar-grid'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables } from '@/lib/supabase/database.types'
import { useRequireEnabledModule } from '../hooks/useRequireEnabledModule'
import { useAuth } from '@/contexts/AuthContext'

type BookingProvider = Tables<'booking_providers'>
type BookingService = Tables<'booking_services'>
type Slot = Tables<'slots'>

export function BookingClientScreen() {
  const { isLoading: modulesLoading, enabled } = useRequireEnabledModule('booking', '/')
  const supabase = useMemo(() => getSupabaseClient(), [])
  const { isAuthenticated, user } = useAuth()

  const [loading, setLoading] = useState(true)
  const [bookingError, setBookingError] = useState<string | null>(null)
  const [bookingSuccess, setBookingSuccess] = useState<string | null>(null)
  const [providers, setProviders] = useState<BookingProvider[]>([])
  const [services, setServices] = useState<BookingService[]>([])
  const [slots, setSlots] = useState<Slot[]>([])

  const [providerId, setProviderId] = useState<string>('')
  const [serviceId, setServiceId] = useState<string>('')
  const [customerEmail, setCustomerEmail] = useState('')
  const [customerPhone, setCustomerPhone] = useState('')
  const [viewMode, setViewMode] = useState<'week' | 'day'>('week')
  const [anchorDate, setAnchorDate] = useState<Date>(new Date())

  const loadProviders = useCallback(async () => {
    setLoading(true)
    try {
      const { data, error } = await supabase.from('booking_providers').select('*').eq('is_active', true).order('created_at', { ascending: false })
      if (error) throw error
      setProviders(data ?? [])
      const first = data?.[0]?.id
      if (first && !providerId) setProviderId(first)
    } finally {
      setLoading(false)
    }
  }, [providerId, supabase])

  const loadServices = useCallback(async () => {
    if (!providerId) return
    setLoading(true)
    try {
      const { data, error } = await supabase
        .from('booking_services')
        .select('*')
        .eq('provider_id', providerId)
        .eq('is_active', true)
        .order('created_at', { ascending: false })
      if (error) throw error
      setServices(data ?? [])
      const first = data?.[0]?.id
      if (first && !serviceId) setServiceId(first)
    } finally {
      setLoading(false)
    }
  }, [providerId, serviceId, supabase])

  const loadSlots = useCallback(async () => {
    if (!providerId || !serviceId) {
      setSlots([])
      return
    }
    setLoading(true)
    try {
      // Tolérance: on démarre 1h avant "maintenant" pour éviter les effets de fuseau/latence
      // (sinon un créneau tout juste créé peut disparaître si l'horloge client est décalée).
      const now = new Date()
      const rangeStart = new Date(now.getTime() - 60 * 60 * 1000)
      const end = new Date()
      end.setDate(end.getDate() + 21)

      const { data, error } = await supabase.rpc('booking_list_slots_public', {
        p_provider_id: providerId,
        p_service_id: serviceId,
        p_range_start: rangeStart.toISOString(),
        p_range_end: end.toISOString(),
      })
      if (error) throw error
      console.info('[Booking] booking_list_available_slots', {
        providerId,
        serviceId,
        rangeStart: rangeStart.toISOString(),
        rangeEnd: end.toISOString(),
        count: (data as any[])?.length ?? 0,
      })
      setSlots((data ?? []) as Slot[])
    } finally {
      setLoading(false)
    }
  }, [providerId, serviceId, supabase])

  useEffect(() => {
    if (modulesLoading) return
    if (!enabled) return
    void loadProviders()
  }, [enabled, loadProviders, modulesLoading])

  useEffect(() => {
    if (!enabled) return
    void loadServices()
  }, [enabled, loadServices, providerId])

  useEffect(() => {
    if (!enabled) return
    void loadSlots()
  }, [enabled, loadSlots, providerId, serviceId])

  const book = async (slotId: string) => {
    if (!providerId || !serviceId) return
    setLoading(true)
    setBookingError(null)
    setBookingSuccess(null)
    try {
      const { data, error } = await supabase.rpc('booking_create_booking', {
        provider_id: providerId,
        service_id: serviceId,
        slot_id: slotId,
        quantity: 1,
        customer_email: isAuthenticated ? null : customerEmail,
        customer_phone: customerPhone || null,
      })
      if (error) throw error

      // Option: data = booking_id
      setBookingSuccess('Réservation créée. Le créneau peut disparaître s’il est complet.')
      console.log('Booking créé:', data)
      await loadSlots()
    } catch (e: any) {
      const msg = e?.message ?? 'Impossible de réserver ce créneau.'
      setBookingError(msg)
    } finally {
      setLoading(false)
    }
  }

  if (modulesLoading) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }
  if (!enabled) return null

  return (
    <AppShellScreen>
      <ContentStack>
        <div className="space-y-2">
          <h1 className="text-2xl font-bold text-base-content">Réserver</h1>
          <p className="text-base-content/60">
            Choisis un prestataire, une prestation, puis réserve un créneau.
          </p>
        </div>

        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Recherche</span>
            {loading && <span className="loading loading-spinner loading-sm text-primary" />}
          </CardHeader>
          <CardBody className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              <div>
                <label className="label">
                  <span className="label-text">Prestataire</span>
                </label>
                <select className="select select-bordered w-full" value={providerId} onChange={(e) => setProviderId(e.target.value)}>
                  <option value="">—</option>
                  {providers.map((p) => (
                    <option key={p.id} value={p.id}>
                      {p.display_name || p.id}
                    </option>
                  ))}
                </select>
              </div>

              <div>
                <label className="label">
                  <span className="label-text">Prestation</span>
                </label>
                <select className="select select-bordered w-full" value={serviceId} onChange={(e) => setServiceId(e.target.value)}>
                  <option value="">—</option>
                  {services.map((s) => (
                    <option key={s.id} value={s.id}>
                      {s.name} ({s.duration_minutes} min)
                    </option>
                  ))}
                </select>
              </div>
            </div>

            {!isAuthenticated && (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                <div>
                  <label className="label">
                    <span className="label-text">Email</span>
                  </label>
                  <input className="input input-bordered w-full" value={customerEmail} onChange={(e) => setCustomerEmail(e.target.value)} />
                </div>
                <div>
                  <label className="label">
                    <span className="label-text">Téléphone (optionnel)</span>
                  </label>
                  <input className="input input-bordered w-full" value={customerPhone} onChange={(e) => setCustomerPhone(e.target.value)} />
                </div>
              </div>
            )}

            <div className="text-sm text-base-content/60">
              Prestataire ? <Link className="link" href="/pro/booking">Espace prestataire</Link>
            </div>
          </CardBody>
        </Card>

        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Créneaux disponibles</span>
            <Badge variant="neutral" style="soft" size="sm">
              {slots.length}
            </Badge>
          </CardHeader>
          <CardBody className="p-0">
            {(bookingError || bookingSuccess) && (
              <div className="p-4">
                {bookingError && (
                  <div className="alert alert-error">
                    <span className="icon-[tabler--alert-circle] size-5" />
                    <span>{bookingError}</span>
                  </div>
                )}
                {bookingSuccess && (
                  <div className="alert alert-success">
                    <span className="icon-[tabler--check] size-5" />
                    <span>{bookingSuccess}</span>
                  </div>
                )}
              </div>
            )}
            {loading ? (
              <div className="p-6 flex items-center justify-center">
                <span className="loading loading-spinner text-primary" />
              </div>
            ) : slots.length === 0 ? (
              <div className="p-6 text-base-content/60">Aucun créneau trouvé.</div>
            ) : (
              <div className="p-4 space-y-3">
                <div className="flex items-center justify-between gap-3">
                  <div className="flex items-center gap-2">
                    <button
                      className="btn btn-ghost btn-sm"
                      onClick={() => setAnchorDate((d) => new Date(d.getTime() - 7 * 24 * 60 * 60 * 1000))}
                    >
                      <span className="icon-[tabler--chevron-left] size-4" />
                    </button>
                    <button className="btn btn-ghost btn-sm" onClick={() => setAnchorDate(new Date())}>
                      Aujourd’hui
                    </button>
                    <button
                      className="btn btn-ghost btn-sm"
                      onClick={() => setAnchorDate((d) => new Date(d.getTime() + 7 * 24 * 60 * 60 * 1000))}
                    >
                      <span className="icon-[tabler--chevron-right] size-4" />
                    </button>
                  </div>
                  <div className="join">
                    <button
                      className={`btn btn-sm join-item ${viewMode === 'week' ? 'btn-primary' : 'btn-ghost'}`}
                      onClick={() => setViewMode('week')}
                    >
                      Semaine
                    </button>
                    <button
                      className={`btn btn-sm join-item ${viewMode === 'day' ? 'btn-primary' : 'btn-ghost'}`}
                      onClick={() => setViewMode('day')}
                    >
                      Jour
                    </button>
                  </div>
                </div>

                <CalendarGrid
                  mode={viewMode}
                  anchorDate={anchorDate}
                  slots={slots.map((s) => ({ ...s, startAt: s.start_at, endAt: s.end_at }))}
                  renderSlot={(s) => {
                    const slot = s as unknown as Slot & { startAt: string; endAt: string }
                    const cap = slot.capacity ?? 1
                    const remaining = cap - slot.participants_count
                    const isReserved = slot.participants_count > 0
                    const isPast = new Date(slot.start_at).getTime() < Date.now()
                    return (
                      <div
                        className={[
                          'rounded-lg border border-base-300 p-2 transition-colors',
                          // Fond coloré (opacité 60%) : Libre=vert, Occupé=bleu
                          isReserved ? 'bg-blue-500/60 hover:bg-blue-500/70' : 'bg-green-500/60 hover:bg-green-500/70',
                        ].join(' ')}
                      >
                        <div className="flex items-center justify-between">
                          <div className="text-xs font-semibold text-base-content">
                            {new Date(slot.start_at).toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' })}
                          </div>
                        </div>
                        <div className="mt-2 flex items-center gap-2">
                          <Badge
                            variant={isReserved ? 'info' : 'success'}
                            style="soft"
                            size="xs"
                          >
                            {isReserved ? 'Réservé' : 'Libre'}
                          </Badge>
                        </div>
                        <div className="mt-2">
                          <button
                            className="btn btn-primary btn-xs"
                            onClick={() => book(slot.id)}
                            disabled={isPast || remaining <= 0 || loading || (!isAuthenticated && !customerEmail) || !providerId || !serviceId}
                          >
                            Réserver
                          </button>
                        </div>
                      </div>
                    )
                  }}
                />
              </div>
            )}
          </CardBody>
        </Card>
      </ContentStack>
      <BottomNav />
    </AppShellScreen>
  )
}

