'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables, BookingTimeOffMode } from '@/lib/supabase/database.types'
import { useRequireEnabledModule } from '../hooks/useRequireEnabledModule'
import { useBookingProvider } from '../hooks/useBookingProvider'
import { SubscriptionGate } from '@/components/paywall/SubscriptionGate'

type BookingTimeOff = Tables<'booking_time_off'>

const modes: { value: BookingTimeOffMode; label: string; help: string }[] = [
  { value: 'block_slots', label: 'Bloquer les créneaux', help: 'Empêche la réservation sur cette période.' },
  { value: 'cancel_bookings', label: 'Annuler les réservations', help: 'Option future (workflow + emails).' },
  { value: 'request_reschedule', label: 'Demander un report', help: 'Option future (workflow + emails).' },
]

export function ProviderTimeOffScreen() {
  const { isLoading: modulesLoading, enabled } = useRequireEnabledModule('booking', '/')
  const supabase = useMemo(() => getSupabaseClient(), [])
  const { isAuthenticated, mustDenyProviderBackOffice, ensureProvider } = useBookingProvider()

  const [loading, setLoading] = useState(true)
  const [rows, setRows] = useState<BookingTimeOff[]>([])

  const [startAt, setStartAt] = useState('')
  const [endAt, setEndAt] = useState('')
  const [mode, setMode] = useState<BookingTimeOffMode>('block_slots')
  const [reason, setReason] = useState('')

  const load = useCallback(async () => {
    if (!enabled || !isAuthenticated || mustDenyProviderBackOffice) return
    setLoading(true)
    try {
      const prov = await ensureProvider()
      if (!prov) return
      const { data, error } = await supabase
        .from('booking_time_off')
        .select('*')
        .eq('provider_id', prov.id)
        .order('start_at', { ascending: true })
        .limit(200)
      if (error) throw error
      setRows(data ?? [])
    } finally {
      setLoading(false)
    }
  }, [enabled, ensureProvider, isAuthenticated, mustDenyProviderBackOffice, supabase])

  useEffect(() => {
    if (modulesLoading) return
    void load()
  }, [load, modulesLoading])

  const create = async () => {
    if (!startAt || !endAt) return
    setLoading(true)
    try {
      const prov = await ensureProvider()
      if (!prov) return
      const { error } = await supabase.from('booking_time_off').insert({
        provider_id: prov.id,
        start_at: new Date(startAt).toISOString(),
        end_at: new Date(endAt).toISOString(),
        mode,
        reason: reason.trim() ? reason.trim() : null,
      })
      if (error) throw error
      setStartAt('')
      setEndAt('')
      setMode('block_slots')
      setReason('')
      await load()
    } finally {
      setLoading(false)
    }
  }

  const remove = async (id: string) => {
    setLoading(true)
    try {
      const { error } = await supabase.from('booking_time_off').delete().eq('id', id)
      if (error) throw error
      await load()
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
        <div className="text-base-content/70">Connecte-toi pour gérer tes vacances.</div>
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
      message="Un abonnement actif est requis pour gérer vos vacances et indisponibilités."
      paywallPath="/pricing"
    >
      <AppShellScreen>
      <ContentStack>
        <div className="flex items-start justify-between gap-4">
          <div>
            <h1 className="text-2xl font-bold text-base-content">Vacances & indisponibilités</h1>
            <p className="text-base-content/60 mt-1">
              Déclare des périodes pour bloquer la réservation et la génération de créneaux.
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
            <span className="font-semibold text-base-content">Ajouter une période</span>
            {loading && <span className="loading loading-spinner loading-sm text-primary" />}
          </CardHeader>
          <CardBody className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
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
                  <span className="label-text">Fin</span>
                </label>
                <input
                  type="datetime-local"
                  className="input input-bordered w-full"
                  value={endAt}
                  onChange={(e) => setEndAt(e.target.value)}
                />
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              <div>
                <label className="label">
                  <span className="label-text">Mode</span>
                </label>
                <select className="select select-bordered w-full" value={mode} onChange={(e) => setMode(e.target.value as any)}>
                  {modes.map((m) => (
                    <option key={m.value} value={m.value}>
                      {m.label}
                    </option>
                  ))}
                </select>
                <p className="text-xs text-base-content/60 mt-1">{modes.find((m) => m.value === mode)?.help}</p>
              </div>
              <div>
                <label className="label">
                  <span className="label-text">Raison (optionnel)</span>
                </label>
                <input className="input input-bordered w-full" value={reason} onChange={(e) => setReason(e.target.value)} />
              </div>
            </div>

            <button className="btn btn-primary btn-sm" onClick={create} disabled={loading || !startAt || !endAt}>
              <span className="icon-[tabler--plus] size-4" />
              Ajouter
            </button>
          </CardBody>
        </Card>

        <Card className="mt-6">
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Mes périodes</span>
            <Badge variant="neutral" style="soft" size="sm">
              {rows.length}
            </Badge>
          </CardHeader>
          <CardBody className="p-0">
            {loading ? (
              <div className="p-6 flex items-center justify-center">
                <span className="loading loading-spinner text-primary" />
              </div>
            ) : rows.length === 0 ? (
              <div className="p-6 text-base-content/60">Aucune période.</div>
            ) : (
              <div className="overflow-x-auto">
                <table className="table table-zebra">
                  <thead>
                    <tr>
                      <th>Période</th>
                      <th>Mode</th>
                      <th>Raison</th>
                      <th />
                    </tr>
                  </thead>
                  <tbody>
                    {rows.map((r) => (
                      <tr key={r.id}>
                        <td className="text-base-content">
                          {new Date(r.start_at).toLocaleString('fr-FR')} → {new Date(r.end_at).toLocaleString('fr-FR')}
                        </td>
                        <td className="text-base-content/70">{r.mode}</td>
                        <td className="text-base-content/70">{r.reason || '—'}</td>
                        <td className="text-right">
                          <button className="btn btn-ghost btn-xs text-error" onClick={() => remove(r.id)} disabled={loading}>
                            Supprimer
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </CardBody>
        </Card>
      </ContentStack>
    </AppShellScreen>
    </SubscriptionGate>
  )
}

