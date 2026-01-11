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

export function ProviderPlanningScreen() {
  const { isLoading: modulesLoading, enabled } = useRequireEnabledModule('booking', '/')
  const supabase = useMemo(() => getSupabaseClient(), [])
  const { isAuthenticated, mustDenyProviderBackOffice, ensureProvider } = useBookingProvider()

  const [loading, setLoading] = useState(true)
  const [agenda, setAgenda] = useState<Agenda | null>(null)
  const [slots, setSlots] = useState<Slot[]>([])
  const [services, setServices] = useState<BookingService[]>([])
  const [slotServices, setSlotServices] = useState<Record<string, string[]>>({})

  const [startAt, setStartAt] = useState('')
  const [durationMinutes, setDurationMinutes] = useState<number>(30)
  const [capacity, setCapacity] = useState<number>(1)
  const [selectedServiceIds, setSelectedServiceIds] = useState<string[]>([])
  const [viewMode, setViewMode] = useState<'week' | 'day'>('week')
  const [anchorDate, setAnchorDate] = useState<Date>(new Date())
  const [editOpen, setEditOpen] = useState(false)
  const [editSlotId, setEditSlotId] = useState<string | null>(null)
  const [editCapacity, setEditCapacity] = useState<number>(1)
  const [editStatus, setEditStatus] = useState<'confirmed' | 'draft' | 'cancelled'>('confirmed')
  const [editServiceIds, setEditServiceIds] = useState<string[]>([])

  const ensureBookingAgenda = useCallback(
    async (providerId: string) => {
      const { data: existing, error } = await supabase
        .from('agendas')
        .select('*')
        .eq('module_id', 'booking')
        .eq('created_by', providerId)
        .maybeSingle()
      if (error) throw error
      if (existing) return existing

      const payload = {
        module_id: 'booking',
        name: 'Agenda Booking',
        description: 'Agenda prestataire (Booking)',
        created_by: providerId,
        timezone: 'Europe/Paris',
        is_public: false,
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
    setEditServiceIds(slotServices[slotId] ?? [])
    setEditOpen(true)
  }

  const saveEditor = async () => {
    if (!editSlotId) return
    setLoading(true)
    try {
      const { error } = await supabase
        .from('slots')
        .update({ capacity: editCapacity, status: editStatus })
        .eq('id', editSlotId)
      if (error) throw error
      await updateAllowedServices(editSlotId, editServiceIds)
      setEditOpen(false)
      setEditSlotId(null)
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
      <ContentStack>
        <div className="flex items-start justify-between gap-4">
          <div>
            <h1 className="text-2xl font-bold text-base-content">Planning</h1>
            <p className="text-base-content/60 mt-1">
              Créneaux Agenda + prestations disponibles par créneau (Booking).
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
            <span className="font-semibold text-base-content">Calendrier</span>
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
              <div className="join">
                <button className={`btn btn-sm join-item ${viewMode === 'week' ? 'btn-primary' : 'btn-ghost'}`} onClick={() => setViewMode('week')}>
                  Semaine
                </button>
                <button className={`btn btn-sm join-item ${viewMode === 'day' ? 'btn-primary' : 'btn-ghost'}`} onClick={() => setViewMode('day')}>
                  Jour
                </button>
              </div>
            </div>
          </CardHeader>
          <CardBody>
            <CalendarGrid
              mode={viewMode}
              anchorDate={anchorDate}
              slots={slots.map((s) => ({ ...s, startAt: s.start_at, endAt: s.end_at }))}
              renderSlot={(s) => {
                const slot = s as unknown as Slot & { startAt: string; endAt: string }
                const cap = slot.capacity ?? 1
                const remaining = cap - slot.participants_count
                const allowed = slotServices[slot.id] ?? []
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
                    <div className="text-[11px] text-base-content/60 mt-1 truncate">
                      {allowed.length
                        ? allowed.map((id) => serviceLabel.get(id) ?? id).join(', ')
                        : '—'}
                    </div>
                    <div className="mt-2">
                      <Badge variant="neutral" style="soft" size="xs">
                        {slot.status}
                      </Badge>
                    </div>
                  </div>
                )
              }}
              onSlotClick={(s) => {
                const slot = s as unknown as Slot & { startAt: string; endAt: string }
                openEditor(slot.id)
              }}
            />
          </CardBody>
        </Card>

        <Modal
          open={editOpen}
          title="Éditer le créneau"
          onClose={() => {
            setEditOpen(false)
            setEditSlotId(null)
          }}
          footer={
            <div className="flex items-center justify-end gap-2">
              <button className="btn btn-ghost btn-sm" onClick={() => setEditOpen(false)} disabled={loading}>
                Fermer
              </button>
              <button className="btn btn-primary btn-sm" onClick={() => void saveEditor()} disabled={loading || !editSlotId}>
                Enregistrer
              </button>
            </div>
          }
        >
          {!editSlotId ? (
            <div className="text-base-content/60">Aucun créneau sélectionné.</div>
          ) : (
            <div className="space-y-4">
              <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
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
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                <div>
                  <label className="label">
                    <span className="label-text">Statut</span>
                  </label>
                  <select
                    className="select select-bordered w-full"
                    value={editStatus}
                    onChange={(e) => setEditStatus(e.target.value as any)}
                  >
                    <option value="confirmed">Publié (confirmed)</option>
                    <option value="draft">Masqué (draft)</option>
                    <option value="cancelled">Annulé (cancelled)</option>
                  </select>
                </div>
                <div className="text-sm text-base-content/60 flex items-center">
                  “Publié” = visible côté client.
                </div>
              </div>

              <div>
                <div className="flex items-center justify-between">
                  <div className="font-medium text-base-content">Prestations autorisées</div>
                  <Badge variant="neutral" style="soft" size="sm">
                    {editServiceIds.length}
                  </Badge>
                </div>
                <div className="mt-2 grid grid-cols-1 sm:grid-cols-2 gap-2">
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
                      return (
                        <tr key={s.id}>
                          <td className="text-base-content">
                            {new Date(s.start_at).toLocaleString('fr-FR')}
                          </td>
                          <td className="text-base-content/70">
                            {Math.round((new Date(s.end_at).getTime() - new Date(s.start_at).getTime()) / 60000)} min
                          </td>
                          <td className="text-base-content/70">{s.capacity ?? '-'}</td>
                          <td className="text-base-content/70">{s.participants_count}</td>
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
      </ContentStack>
    </AppShellScreen>
  )
}

