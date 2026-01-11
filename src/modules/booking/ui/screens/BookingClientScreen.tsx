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
      const now = new Date()
      const end = new Date()
      end.setDate(end.getDate() + 21)

      const { data, error } = await supabase.rpc('booking_list_available_slots', {
        p_provider_id: providerId,
        p_service_id: serviceId,
        p_range_start: now.toISOString(),
        p_range_end: end.toISOString(),
        p_quantity: 1,
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
      console.log('Booking créé:', data)
      await loadSlots()
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
                          <button
                            className="btn btn-primary btn-xs"
                            onClick={() => book(slot.id)}
                            disabled={loading || (!isAuthenticated && !customerEmail) || !providerId || !serviceId}
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

