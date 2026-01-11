'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import type { Tables } from '@/lib/supabase/database.types'
import { useAuth } from '@/contexts/AuthContext'
import { useRequireEnabledModule } from '../hooks/useRequireEnabledModule'
import { useBookingProvider } from '../hooks/useBookingProvider'
import { getSupabaseClient } from '@/lib/supabase/client'

type BookingProvider = Tables<'booking_providers'>
type BookingService = Tables<'booking_services'>

export function ProviderServicesScreen() {
  const { isLoading: modulesLoading, enabled } = useRequireEnabledModule('booking', '/')
  const { isAuthenticated, profile } = useAuth()
  const supabase = useMemo(() => getSupabaseClient(), [])

  const [loading, setLoading] = useState(true)
  const { provider, ensureProvider, mustDenyProviderBackOffice } = useBookingProvider()
  const [services, setServices] = useState<BookingService[]>([])

  const [name, setName] = useState('')
  const [durationMinutes, setDurationMinutes] = useState<number>(30)
  const [requiresApproval, setRequiresApproval] = useState(false)

  const load = useCallback(async () => {
    if (!enabled || !isAuthenticated || mustDenyProviderBackOffice) return
    setLoading(true)
    try {
      const prov = await ensureProvider()
      if (!prov) return

      const { data, error } = await supabase
        .from('booking_services')
        .select('*')
        .eq('provider_id', prov.id)
        .order('created_at', { ascending: false })
      if (error) throw error
      setServices(data ?? [])
    } finally {
      setLoading(false)
    }
  }, [enabled, ensureProvider, isAuthenticated, mustDenyProviderBackOffice, supabase])

  useEffect(() => {
    if (modulesLoading) return
    void load()
  }, [load, modulesLoading])

  const createService = async () => {
    if (!provider) return
    if (!name.trim()) return

    const payload = {
      provider_id: provider.id,
      name: name.trim(),
      description: null,
      duration_minutes: durationMinutes,
      price_hint: null,
      currency: null,
      requires_approval: requiresApproval,
      cancellation_policy: null,
      default_capacity: 1,
      tags: null,
      is_active: true,
    }

    setLoading(true)
    try {
      const { error } = await supabase.from('booking_services').insert(payload)
      if (error) throw error
      setName('')
      setDurationMinutes(30)
      setRequiresApproval(false)
      await load()
    } finally {
      setLoading(false)
    }
  }

  const toggleActive = async (serviceId: string, next: boolean) => {
    setLoading(true)
    try {
      const { error } = await supabase.from('booking_services').update({ is_active: next }).eq('id', serviceId)
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
        <div className="text-base-content/70">Connecte-toi pour gérer tes prestations.</div>
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
            <h1 className="text-2xl font-bold text-base-content">Prestations</h1>
            <p className="text-base-content/60 mt-1">
              Catalogue de prestations (CRUD) — filtré par RLS.
            </p>
          </div>
          <div className="flex gap-2">
            <Link href="/pro/booking" className="btn btn-ghost btn-sm">
              <span className="icon-[tabler--arrow-left] size-4" />
              Retour
            </Link>
          </div>
        </div>

        <Card className="mt-6">
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Nouvelle prestation</span>
            {loading && <span className="loading loading-spinner loading-sm text-primary" />}
          </CardHeader>
          <CardBody className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
              <div className="md:col-span-2">
                <label className="label">
                  <span className="label-text">Nom</span>
                </label>
                <input
                  className="input input-bordered w-full"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="Ex: Coupe, Barbe, Coloration..."
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
            </div>

            <label className="label cursor-pointer justify-start gap-3">
              <input
                type="checkbox"
                className="toggle toggle-primary"
                checked={requiresApproval}
                onChange={(e) => setRequiresApproval(e.target.checked)}
              />
              <span className="label-text">Nécessite une validation (approval)</span>
            </label>

            <button className="btn btn-primary btn-sm" onClick={createService} disabled={loading || !name.trim()}>
              <span className="icon-[tabler--plus] size-4" />
              Ajouter
            </button>
          </CardBody>
        </Card>

        <Card className="mt-6">
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Mes prestations</span>
            <Badge variant="neutral" style="soft" size="sm">
              {services.length}
            </Badge>
          </CardHeader>
          <CardBody className="p-0">
            {loading ? (
              <div className="p-6 flex items-center justify-center">
                <span className="loading loading-spinner text-primary" />
              </div>
            ) : services.length === 0 ? (
              <div className="p-6 text-base-content/60">Aucune prestation pour l’instant.</div>
            ) : (
              <div className="overflow-x-auto">
                <table className="table table-zebra">
                  <thead>
                    <tr>
                      <th>Nom</th>
                      <th>Durée</th>
                      <th>Approval</th>
                      <th>Statut</th>
                    </tr>
                  </thead>
                  <tbody>
                    {services.map((s) => (
                      <tr key={s.id}>
                        <td className="text-base-content font-medium">{s.name}</td>
                        <td className="text-base-content/70">{s.duration_minutes} min</td>
                        <td className="text-base-content/70">{s.requires_approval ? 'Oui' : 'Non'}</td>
                        <td>
                          <label className="label cursor-pointer justify-start gap-3">
                            <input
                              type="checkbox"
                              className="toggle toggle-primary"
                              checked={s.is_active}
                              onChange={(e) => toggleActive(s.id, e.target.checked)}
                              disabled={loading}
                            />
                            <span className="label-text">{s.is_active ? 'Active' : 'Inactive'}</span>
                          </label>
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
  )
}

