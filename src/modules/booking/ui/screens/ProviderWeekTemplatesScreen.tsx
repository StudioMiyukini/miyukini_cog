'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables } from '@/lib/supabase/database.types'
import { useRequireEnabledModule } from '../hooks/useRequireEnabledModule'
import { useBookingProvider } from '../hooks/useBookingProvider'
import { SubscriptionGate } from '@/components/paywall/SubscriptionGate'

type BookingWeekTemplate = Tables<'booking_week_templates'>
type BookingService = Tables<'booking_services'>

type Weekday = 'monday' | 'tuesday' | 'wednesday' | 'thursday' | 'friday' | 'saturday' | 'sunday'

const weekdays: { key: Weekday; label: string }[] = [
  { key: 'monday', label: 'Lundi' },
  { key: 'tuesday', label: 'Mardi' },
  { key: 'wednesday', label: 'Mercredi' },
  { key: 'thursday', label: 'Jeudi' },
  { key: 'friday', label: 'Vendredi' },
  { key: 'saturday', label: 'Samedi' },
  { key: 'sunday', label: 'Dimanche' },
]

type RuleBlock = {
  start: string // "09:00"
  end: string // "12:00"
  slot: number // minutes
  capacity: number
  service_ids: string[]
}

type TemplateRules = Partial<Record<Weekday, RuleBlock[]>>

function safeJsonParse<T>(value: unknown, fallback: T): T {
  try {
    if (typeof value === 'object' && value) return value as T
    if (typeof value === 'string') return JSON.parse(value) as T
    return fallback
  } catch {
    return fallback
  }
}

function normalizeTime(input: string, fallback: string) {
  const raw = (input ?? '').trim()
  if (!raw) return fallback
  // "9" => "09:00"
  if (/^\d{1,2}$/.test(raw)) {
    const h = Math.max(0, Math.min(23, Number(raw)))
    return `${String(h).padStart(2, '0')}:00`
  }
  // "9:5" => "09:05"
  const m = raw.match(/^(\d{1,2}):(\d{1,2})$/)
  if (m) {
    const h = Math.max(0, Math.min(23, Number(m[1])))
    const min = Math.max(0, Math.min(59, Number(m[2])))
    return `${String(h).padStart(2, '0')}:${String(min).padStart(2, '0')}`
  }
  // Already "HH:MM"
  if (/^\d{2}:\d{2}$/.test(raw)) return raw
  return fallback
}

export function ProviderWeekTemplatesScreen() {
  const { isLoading: modulesLoading, enabled } = useRequireEnabledModule('booking', '/')
  const supabase = useMemo(() => getSupabaseClient(), [])
  const { isAuthenticated, mustDenyProviderBackOffice, ensureProvider } = useBookingProvider()

  const [loading, setLoading] = useState(true)
  const [templates, setTemplates] = useState<BookingWeekTemplate[]>([])
  const [services, setServices] = useState<BookingService[]>([])

  const [name, setName] = useState('Semaine type')
  const [selectedTemplateId, setSelectedTemplateId] = useState<string | null>(null)

  // Rule builder
  const [day, setDay] = useState<Weekday>('monday')
  const [start, setStart] = useState('09:00')
  const [end, setEnd] = useState('12:00')
  const [slot, setSlot] = useState<number>(30)
  const [capacity, setCapacity] = useState<number>(1)
  const [serviceIds, setServiceIds] = useState<string[]>([])

  // Generation
  const [rangeStart, setRangeStart] = useState<string>('')
  const [rangeEnd, setRangeEnd] = useState<string>('')

  const load = useCallback(async () => {
    if (!enabled || !isAuthenticated || mustDenyProviderBackOffice) return
    setLoading(true)
    try {
      const prov = await ensureProvider()
      if (!prov) return

      const { data: servicesData, error: servicesError } = await supabase
        .from('booking_services')
        .select('*')
        .eq('provider_id', prov.id)
        .order('created_at', { ascending: false })
      if (servicesError) throw servicesError
      setServices(servicesData ?? [])

      const { data, error } = await supabase
        .from('booking_week_templates')
        .select('*')
        .eq('provider_id', prov.id)
        .order('created_at', { ascending: false })
      if (error) throw error
      setTemplates(data ?? [])
      setSelectedTemplateId((prev) => prev ?? (data?.[0]?.id ?? null))
    } finally {
      setLoading(false)
    }
  }, [enabled, ensureProvider, isAuthenticated, mustDenyProviderBackOffice, supabase])

  useEffect(() => {
    if (modulesLoading) return
    void load()
  }, [load, modulesLoading])

  const selectedTemplate = useMemo(
    () => templates.find((t) => t.id === selectedTemplateId) ?? null,
    [selectedTemplateId, templates]
  )

  const rules: TemplateRules = useMemo(() => safeJsonParse<TemplateRules>(selectedTemplate?.rules, {}), [selectedTemplate?.rules])

  const createTemplate = async () => {
    setLoading(true)
    try {
      const prov = await ensureProvider()
      if (!prov) return
      const payload = {
        provider_id: prov.id,
        name: name.trim() || 'Semaine type',
        timezone: 'Europe/Paris',
        rules: {},
        is_active: true,
      }
      const { data, error } = await supabase.from('booking_week_templates').insert(payload).select('*').single()
      if (error) throw error
      setTemplates((prev) => [data, ...prev])
      setSelectedTemplateId(data.id)
    } finally {
      setLoading(false)
    }
  }

  const saveRules = async (templateId: string, nextRules: TemplateRules) => {
    setLoading(true)
    try {
      const { error } = await supabase.from('booking_week_templates').update({ rules: nextRules }).eq('id', templateId)
      if (error) throw error
      await load()
    } finally {
      setLoading(false)
    }
  }

  const addBlock = async () => {
    if (!selectedTemplate) return
    if (serviceIds.length === 0) return

    const next: TemplateRules = { ...rules }
    const list = Array.from(next[day] ?? [])
    list.push({
      start: normalizeTime(start, '09:00'),
      end: normalizeTime(end, '10:00'),
      slot,
      capacity,
      service_ids: serviceIds,
    })
    next[day] = list
    await saveRules(selectedTemplate.id, next)
  }

  const deleteBlock = async (weekday: Weekday, index: number) => {
    if (!selectedTemplate) return
    const next: TemplateRules = { ...rules }
    const list = Array.from(next[weekday] ?? [])
    list.splice(index, 1)
    next[weekday] = list
    await saveRules(selectedTemplate.id, next)
  }

  const generateSlots = async () => {
    if (!selectedTemplate) return
    if (!rangeStart || !rangeEnd) return
    setLoading(true)
    try {
      const prov = await ensureProvider()
      if (!prov) return

      // RPC (sera ajoutée dans la migration) : booking_generate_slots(provider_id, template_id, range_start, range_end)
      const { error } = await supabase.rpc('booking_generate_slots', {
        provider_id: prov.id,
        template_id: selectedTemplate.id,
        range_start: rangeStart,
        range_end: rangeEnd,
      })
      if (error) throw error
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

  if (!isAuthenticated) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <div className="text-base-content/70">Connecte-toi pour gérer tes semaines types.</div>
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
    <SubscriptionGate
      message="Un abonnement actif est requis pour gérer vos semaines types."
      paywallPath="/pricing"
    >
      <AppShellScreen>
      <ContentStack>
        <div className="flex items-start justify-between gap-4">
          <div>
            <h1 className="text-2xl font-bold text-base-content">Semaine type</h1>
            <p className="text-base-content/60 mt-1">
              Définis des règles hebdo puis génère des créneaux en masse (en respectant les vacances).
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

        <div className="mt-6 grid grid-cols-1 lg:grid-cols-3 gap-6">
          <Card>
            <CardHeader className="flex items-center justify-between">
              <span className="font-semibold text-base-content">Templates</span>
              {loading && <span className="loading loading-spinner loading-sm text-primary" />}
            </CardHeader>
            <CardBody className="space-y-3">
              <label className="label">
                <span className="label-text">Nouveau template</span>
              </label>
              <input className="input input-bordered w-full" value={name} onChange={(e) => setName(e.target.value)} />
              <button className="btn btn-primary btn-sm" onClick={createTemplate} disabled={loading}>
                <span className="icon-[tabler--plus] size-4" />
                Créer
              </button>

              <div className="divider" />

              <div className="space-y-2">
                {templates.length === 0 ? (
                  <div className="text-sm text-base-content/60">Aucun template.</div>
                ) : (
                  templates.map((t) => (
                    <button
                      key={t.id}
                      className={`btn btn-sm w-full justify-start ${t.id === selectedTemplateId ? 'btn-primary' : 'btn-ghost'}`}
                      onClick={() => setSelectedTemplateId(t.id)}
                    >
                      {t.name}
                    </button>
                  ))
                )}
              </div>
            </CardBody>
          </Card>

          <Card className="lg:col-span-2">
            <CardHeader className="flex items-center justify-between">
              <span className="font-semibold text-base-content">Règles</span>
              <Badge variant="neutral" style="soft" size="sm">
                {Object.values(rules).reduce((acc, v) => acc + (v?.length ?? 0), 0)}
              </Badge>
            </CardHeader>
            <CardBody className="space-y-4">
              {!selectedTemplate ? (
                <div className="text-base-content/60">Sélectionne un template.</div>
              ) : (
                <>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                    <div>
                      <label className="label">
                        <span className="label-text">Jour</span>
                      </label>
                      <select className="select select-bordered w-full" value={day} onChange={(e) => setDay(e.target.value as Weekday)}>
                        {weekdays.map((d) => (
                          <option key={d.key} value={d.key}>
                            {d.label}
                          </option>
                        ))}
                      </select>
                    </div>
                    <div className="grid grid-cols-2 gap-3">
                      <div>
                        <label className="label">
                          <span className="label-text">Début</span>
                        </label>
                        <input type="time" className="input input-bordered w-full" value={start} onChange={(e) => setStart(e.target.value)} />
                      </div>
                      <div>
                        <label className="label">
                          <span className="label-text">Fin</span>
                        </label>
                        <input type="time" className="input input-bordered w-full" value={end} onChange={(e) => setEnd(e.target.value)} />
                      </div>
                    </div>
                  </div>

                  <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
                    <div>
                      <label className="label">
                        <span className="label-text">Créneau (min)</span>
                      </label>
                      <input type="number" className="input input-bordered w-full" value={slot} min={5} step={5} onChange={(e) => setSlot(Number(e.target.value))} />
                    </div>
                    <div>
                      <label className="label">
                        <span className="label-text">Capacité</span>
                      </label>
                      <input type="number" className="input input-bordered w-full" value={capacity} min={1} step={1} onChange={(e) => setCapacity(Number(e.target.value))} />
                    </div>
                    <div>
                      <label className="label">
                        <span className="label-text">Prestations</span>
                      </label>
                      <div className="max-h-32 overflow-auto border border-base-300 rounded-lg p-2 bg-base-100">
                        {services.filter((s) => s.is_active).length === 0 ? (
                          <div className="text-sm text-base-content/60">
                            Ajoute des prestations dans <Link className="link" href="/pro/booking/services">Prestations</Link>.
                          </div>
                        ) : (
                          services
                            .filter((s) => s.is_active)
                            .map((s) => (
                              <label key={s.id} className="label cursor-pointer justify-start gap-3 py-1">
                                <input
                                  type="checkbox"
                                  className="checkbox checkbox-xs checkbox-primary"
                                  checked={serviceIds.includes(s.id)}
                                  onChange={(e) => {
                                    setServiceIds((prev) =>
                                      e.target.checked ? Array.from(new Set([...prev, s.id])) : prev.filter((x) => x !== s.id)
                                    )
                                  }}
                                />
                                <span className="label-text text-xs">{s.name}</span>
                              </label>
                            ))
                        )}
                      </div>
                    </div>
                  </div>

                  <button className="btn btn-primary btn-sm" onClick={addBlock} disabled={loading || serviceIds.length === 0}>
                    <span className="icon-[tabler--plus] size-4" />
                    Ajouter ce bloc
                  </button>

                  <div className="divider" />

                  <div className="space-y-3">
                    {weekdays.map((d) => {
                      const blocks = rules[d.key] ?? []
                      return (
                        <div key={d.key} className="border border-base-300 rounded-lg p-3">
                          <div className="flex items-center justify-between">
                            <div className="font-semibold text-base-content">{d.label}</div>
                            <Badge variant="neutral" style="soft" size="sm">
                              {blocks.length}
                            </Badge>
                          </div>
                          {blocks.length === 0 ? (
                            <div className="text-sm text-base-content/60 mt-2">Aucune règle.</div>
                          ) : (
                            <div className="mt-2 space-y-2">
                              {blocks.map((b, idx) => (
                                <div key={idx} className="flex items-start justify-between gap-3 bg-base-200 rounded-lg p-2">
                                  <div className="text-sm text-base-content/80">
                                    <div>
                                      {b.start}–{b.end} • slot {b.slot}min • cap {b.capacity}
                                    </div>
                                    <div className="text-xs text-base-content/60">
                                      prestations: {b.service_ids.length}
                                    </div>
                                  </div>
                                  <button className="btn btn-ghost btn-xs text-error" disabled={loading} onClick={() => deleteBlock(d.key, idx)}>
                                    Supprimer
                                  </button>
                                </div>
                              ))}
                            </div>
                          )}
                        </div>
                      )
                    })}
                  </div>

                  <div className="divider" />

                  <div className="space-y-3">
                    <div className="font-semibold text-base-content">Générer des créneaux</div>
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                      <div>
                        <label className="label">
                          <span className="label-text">Du</span>
                        </label>
                        <input type="date" className="input input-bordered w-full" value={rangeStart} onChange={(e) => setRangeStart(e.target.value)} />
                      </div>
                      <div>
                        <label className="label">
                          <span className="label-text">Au</span>
                        </label>
                        <input type="date" className="input input-bordered w-full" value={rangeEnd} onChange={(e) => setRangeEnd(e.target.value)} />
                      </div>
                    </div>

                    <button className="btn btn-primary btn-sm" onClick={generateSlots} disabled={loading || !rangeStart || !rangeEnd}>
                      <span className="icon-[tabler--calendar-plus] size-4" />
                      Générer
                    </button>
                    <p className="text-xs text-base-content/60">
                      La génération est faite côté serveur et ignore les périodes de vacances (mode block_slots).
                    </p>
                  </div>
                </>
              )}
            </CardBody>
          </Card>
        </div>
      </ContentStack>
    </AppShellScreen>
    </SubscriptionGate>
  )
}

