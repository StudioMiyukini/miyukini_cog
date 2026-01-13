'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { useParams, useSearchParams } from 'next/navigation'
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

export function BookingProviderPlanningScreen() {
  const { isLoading: modulesLoading, enabled } = useRequireEnabledModule('booking', '/')
  const supabase = useMemo(() => getSupabaseClient(), [])
  const { isAuthenticated, user } = useAuth()
  const params = useParams<{ providerId: string }>()
  const search = useSearchParams()

  const providerId = (params?.providerId as string) ?? ''
  const preselectedServiceId = search?.get('service') ?? ''

  const [loading, setLoading] = useState(true)
  const [bookingError, setBookingError] = useState<string | null>(null)
  const [bookingSuccess, setBookingSuccess] = useState<string | null>(null)

  const [provider, setProvider] = useState<BookingProvider | null>(null)
  const [services, setServices] = useState<BookingService[]>([])
  const [slots, setSlots] = useState<Slot[]>([])

  const [serviceId, setServiceId] = useState<string>('')
  const [customerEmail, setCustomerEmail] = useState('')
  const [customerPhone, setCustomerPhone] = useState('')
  const [viewMode, setViewMode] = useState<'week' | 'day'>('week')
  const [anchorDate, setAnchorDate] = useState<Date>(new Date())

  const selectedService = useMemo(() => services.find((s) => s.id === serviceId) ?? null, [serviceId, services])

  const loadProvider = useCallback(async () => {
    if (!providerId) return
    const { data, error } = await supabase.from('booking_providers').select('*').eq('id', providerId).single()
    if (error) throw error
    setProvider((data ?? null) as any)
  }, [providerId, supabase])

  const loadServices = useCallback(async () => {
    if (!providerId) return
    const { data, error } = await supabase
      .from('booking_services')
      .select('*')
      .eq('provider_id', providerId)
      .eq('is_active', true)
      .order('created_at', { ascending: false })
    if (error) throw error
    const list = (data ?? []) as BookingService[]
    setServices(list)

    // init selection (querystring > current > first)
    const qs = preselectedServiceId
    if (qs && list.some((s) => s.id === qs)) {
      setServiceId(qs)
      return
    }
    if (!serviceId) {
      const first = list?.[0]?.id
      if (first) setServiceId(first)
    }
  }, [preselectedServiceId, providerId, serviceId, supabase])

  const loadSlots = useCallback(async () => {
    if (!providerId || !serviceId) {
      setSlots([])
      return
    }
    setLoading(true)
    try {
      // Tolérance: on démarre 1h avant "maintenant" pour éviter les effets de fuseau/latence
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
      setSlots((data ?? []) as Slot[])
    } finally {
      setLoading(false)
    }
  }, [providerId, serviceId, supabase])

  useEffect(() => {
    if (modulesLoading) return
    if (!enabled) return
    setLoading(true)
    Promise.all([loadProvider(), loadServices()])
      .catch((e) => console.error(e))
      .finally(() => setLoading(false))
  }, [enabled, loadProvider, loadServices, modulesLoading])

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
      setBookingSuccess('Réservation créée.')
      console.log('Booking créé:', data)
      await loadSlots()
    } catch (e: any) {
      setBookingError(e?.message ?? 'Impossible de réserver ce créneau.')
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
        <div className="flex flex-wrap items-start justify-between gap-3">
          <div className="space-y-1 min-w-0">
          <div className="text-sm text-base-content/60">
            <Link className="link" href={`/booking/${providerId}`}>
              {provider?.display_name ?? 'Prestataire'}
            </Link>{' '}
            <span className="text-base-content/40">/</span> Planning
          </div>
          <h1 className="text-2xl font-bold text-base-content">Choisir un créneau</h1>
          <p className="text-base-content/60 text-sm truncate">
            {selectedService ? `${selectedService.name} (${selectedService.duration_minutes} min)` : '—'}
          </p>
          </div>

          <Link href={`/booking/${providerId}`} className="btn btn-ghost btn-sm shrink-0">
            <span className="icon-[tabler--arrow-left] size-4" />
            Retour fiche prestataire
          </Link>
        </div>

        <Card>
          <CardHeader className="flex flex-wrap items-center justify-between gap-2 py-3">
            <div className="flex items-center gap-2">
              <span className="font-semibold text-base-content">Planning</span>
              <Badge variant="neutral" style="soft" size="sm">
                {slots.length}
              </Badge>
              {loading && <span className="loading loading-spinner loading-sm text-primary" />}
            </div>

            <div className="flex flex-wrap items-center gap-1.5">
              <div className="flex items-center gap-1">
                <button
                  className="btn btn-ghost btn-sm"
                  onClick={() => {
                    const stepDays = viewMode === 'week' ? 7 : 1
                    setAnchorDate((d) => new Date(d.getTime() - stepDays * 24 * 60 * 60 * 1000))
                  }}
                  aria-label="Précédent"
                >
                  <span className="icon-[tabler--chevron-left] size-4" />
                </button>
                <span className="text-sm font-semibold text-base-content/80">
                  {anchorDate.toLocaleDateString('fr-FR', { weekday: 'short', day: '2-digit', month: 'short' })}
                </span>
                <button
                  className="btn btn-ghost btn-sm"
                  onClick={() => {
                    const stepDays = viewMode === 'week' ? 7 : 1
                    setAnchorDate((d) => new Date(d.getTime() + stepDays * 24 * 60 * 60 * 1000))
                  }}
                  aria-label="Suivant"
                >
                  <span className="icon-[tabler--chevron-right] size-4" />
                </button>
              </div>

              <button className="btn btn-ghost btn-sm" onClick={() => setAnchorDate(new Date())}>
                Aujourd’hui
              </button>

              <div className="flex gap-1 text-xs font-semibold uppercase">
                <button
                  className={`btn btn-sm btn-outline ${viewMode === 'week' ? 'btn-primary' : 'btn-ghost'}`}
                  onClick={() => setViewMode('week')}
                >
                  Semaine
                </button>
                <button
                  className={`btn btn-sm btn-outline ${viewMode === 'day' ? 'btn-primary' : 'btn-ghost'}`}
                  onClick={() => setViewMode('day')}
                >
                  Jour
                </button>
              </div>
            </div>
          </CardHeader>
          <CardBody className="p-0">
            {!isAuthenticated && (
              <div className="p-3 border-b border-base-200">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                  <div>
                    <label className="label py-1">
                      <span className="label-text text-xs">Email</span>
                    </label>
                    <input
                      className="input input-bordered w-full"
                      value={customerEmail}
                      onChange={(e) => setCustomerEmail(e.target.value)}
                      placeholder="obligatoire pour réserver"
                    />
                  </div>
                  <div>
                    <label className="label py-1">
                      <span className="label-text text-xs">Téléphone (optionnel)</span>
                    </label>
                    <input
                      className="input input-bordered w-full"
                      value={customerPhone}
                      onChange={(e) => setCustomerPhone(e.target.value)}
                    />
                  </div>
                </div>
              </div>
            )}

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
              <div className="p-4">
                <CalendarGrid
                  mode={viewMode}
                  anchorDate={anchorDate}
                  slots={slots.map((s) => ({ ...s, startAt: s.start_at, endAt: s.end_at }))}
                  renderSlot={(s) => {
                    const slot = s as unknown as Slot & { startAt: string; endAt: string }
                    const isReserved = (slot.participants_count ?? 0) > 0
                    const isPast = new Date(slot.start_at).getTime() < Date.now()
                    const remaining = Math.max(0, (slot.capacity ?? 0) - (slot.participants_count ?? 0))
                    const canBook = !isPast && remaining > 0
                    return (
                      <div className={['rounded-lg border border-base-200 p-2', isReserved ? 'bg-blue-500/70' : 'bg-green-500/70'].join(' ')}>
                        <div className="text-[11px] font-semibold text-base-content">
                          {new Date(slot.start_at).toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' })}
                        </div>
                        <div className="mt-1 flex items-center justify-between gap-2">
                          <Badge variant={isReserved ? 'info' : 'success'} style="soft" size="xs">
                            {isReserved ? 'Réservé' : 'Libre'}
                          </Badge>
                          <button
                            className="btn btn-xs btn-primary"
                            disabled={loading || !canBook || (!isAuthenticated && !customerEmail.trim())}
                            onClick={(e) => {
                              e.preventDefault()
                              e.stopPropagation()
                              void book(slot.id)
                            }}
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

