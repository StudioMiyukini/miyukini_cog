'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { useRequireSuperAdmin } from '@/features/backoffice/ui/hooks/useRequireSuperAdmin'
import { useAuth } from '@/contexts/AuthContext'
import { getSupabaseClient } from '@/lib/supabase/client'

type PlanStatus = 'draft' | 'active' | 'archived'
type PlanRow = {
  id: string
  name: string
  description: string | null
  price_monthly: number
  price_yearly: number | null
  currency: string
  billing_cycle: string
  tier: string
  features: any
  paypal_plan_id: string | null
  status: PlanStatus
  sort_order: number
  is_featured: boolean
  created_at: string
  updated_at: string
}

export function SuperAdminPaywallPlansScreen() {
  const supabase = useMemo(() => getSupabaseClient() as any, [])
  const { user } = useAuth()
  const { isReady, authLoading } = useRequireSuperAdmin()

  const [loading, setLoading] = useState(true)
  const [creating, setCreating] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [status, setStatus] = useState<string | null>(null)
  const [plans, setPlans] = useState<PlanRow[]>([])

  const load = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const { data, error: loadError } = await supabase
        .from('saas_paywall_plans')
        .select('*')
        .order('sort_order', { ascending: true })

      if (loadError) throw new Error(loadError.message)
      setPlans((data ?? []) as PlanRow[])
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e)
      setError(msg)
    } finally {
      setLoading(false)
    }
  }, [supabase])

  useEffect(() => {
    if (!authLoading && isReady) void load()
  }, [authLoading, isReady, load])

  const createPayPalPlan = useCallback(
    async (planId: string) => {
      if (!user?.id) {
        console.error('[PaywallPlans] Auth required')
        setError('auth_required')
        return
      }

      console.log('[PaywallPlans] Creating PayPal plan for plan_id:', planId)
      setCreating(true)
      setError(null)
      setStatus(null)

      try {
        const { data: session, error: sessionError } = await supabase.auth.getSession()
        if (sessionError) {
          console.error('[PaywallPlans] Session error:', sessionError)
          throw new Error('session_error')
        }
        if (!session?.session?.access_token) {
          console.error('[PaywallPlans] No access token')
          throw new Error('no_session')
        }

        const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL
        if (!supabaseUrl) {
          console.error('[PaywallPlans] Supabase URL not configured')
          throw new Error('supabase_url_not_configured')
        }

        console.log('[PaywallPlans] Calling Edge Function:', `${supabaseUrl}/functions/v1/paywall-create-plan`)
        
        const response = await fetch(`${supabaseUrl}/functions/v1/paywall-create-plan`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${session.session.access_token}`,
          },
          body: JSON.stringify({ plan_id: planId }),
        })

        console.log('[PaywallPlans] Response status:', response.status)
        const result = await response.json()
        console.log('[PaywallPlans] Response data:', result)
        
        if (!response.ok) {
          console.error('[PaywallPlans] Error response:', result)
          throw new Error(result.error || result.details || 'creation_failed')
        }

        console.log('[PaywallPlans] Plan created successfully:', result.paypal_plan_id)
        setStatus(`Plan PayPal créé avec succès: ${result.paypal_plan_id}`)
        await load()
      } catch (e) {
        const msg = e instanceof Error ? e.message : String(e)
        console.error('[PaywallPlans] Exception:', msg, e)
        setError(msg)
      } finally {
        setCreating(false)
      }
    },
    [user, supabase, load]
  )

  const toggleStatus = useCallback(
    async (planId: string, currentStatus: PlanStatus) => {
      if (!user?.id) return

      const newStatus: PlanStatus = currentStatus === 'active' ? 'archived' : 'active'

      try {
        const { error: updateError } = await supabase
          .from('saas_paywall_plans')
          .update({ status: newStatus })
          .eq('id', planId)

        if (updateError) throw new Error(updateError.message)
        await load()
        setStatus(`Plan ${newStatus === 'active' ? 'activé' : 'archivé'}`)
      } catch (e) {
        const msg = e instanceof Error ? e.message : String(e)
        setError(msg)
      }
    },
    [user, supabase, load]
  )

  if (authLoading || !isReady) {
    return (
      <AdminLayout>
        <div className="p-6">
          <p>Chargement...</p>
        </div>
      </AdminLayout>
    )
  }

  return (
    <AdminLayout>
      <div className="p-6 space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold">Plans d'abonnement</h1>
            <p className="text-base-content/60 mt-1">Gérer les plans d'abonnement PayPal</p>
          </div>
        </div>

        {error && (
          <div className="alert alert-error">
            <span className="icon-[tabler--alert-circle] size-5" />
            <span>{error}</span>
          </div>
        )}

        {status && (
          <div className="alert alert-success">
            <span className="icon-[tabler--check] size-5" />
            <span>{status}</span>
          </div>
        )}

        {loading ? (
          <div className="text-center py-8">
            <p>Chargement des plans...</p>
          </div>
        ) : (
          <div className="grid gap-4">
            {plans.map((plan) => (
              <Card key={plan.id}>
                <CardHeader>
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <h3 className="text-lg font-semibold">{plan.name}</h3>
                      <Badge variant={plan.status === 'active' ? 'success' : 'secondary'}>
                        {plan.status}
                      </Badge>
                      {plan.is_featured && (
                        <Badge variant="primary">Featured</Badge>
                      )}
                    </div>
                    <div className="flex items-center gap-2">
                      {!plan.paypal_plan_id && (
                        <button
                          className="btn btn-primary btn-sm"
                          onClick={() => createPayPalPlan(plan.id)}
                          disabled={creating}
                        >
                          {creating ? 'Création...' : 'Créer plan PayPal'}
                        </button>
                      )}
                      {plan.paypal_plan_id && (
                        <button
                          className="btn btn-outline btn-sm"
                          onClick={() => toggleStatus(plan.id, plan.status)}
                        >
                          {plan.status === 'active' ? 'Archiver' : 'Activer'}
                        </button>
                      )}
                    </div>
                  </div>
                </CardHeader>
                <CardBody>
                  <div className="space-y-3">
                    <div>
                      <p className="text-sm text-base-content/60">Description</p>
                      <p className="text-base-content">{plan.description || 'Aucune description'}</p>
                    </div>
                    <div className="grid grid-cols-2 gap-4">
                      <div>
                        <p className="text-sm text-base-content/60">Prix mensuel</p>
                        <p className="text-lg font-semibold">
                          {plan.price_monthly} {plan.currency}
                        </p>
                      </div>
                      {plan.price_yearly && (
                        <div>
                          <p className="text-sm text-base-content/60">Prix annuel</p>
                          <p className="text-lg font-semibold">
                            {plan.price_yearly} {plan.currency}
                          </p>
                        </div>
                      )}
                    </div>
                    <div>
                      <p className="text-sm text-base-content/60">Tier associé</p>
                      <Badge variant="secondary" style="outline">{plan.tier}</Badge>
                    </div>
                    {plan.paypal_plan_id && (
                      <div>
                        <p className="text-sm text-base-content/60">Plan PayPal ID</p>
                        <code className="text-xs bg-base-200 px-2 py-1 rounded">
                          {plan.paypal_plan_id}
                        </code>
                      </div>
                    )}
                    {plan.features && typeof plan.features === 'object' && (
                      <div>
                        <p className="text-sm text-base-content/60 mb-2">Fonctionnalités</p>
                        <ul className="list-disc list-inside space-y-1">
                          {Array.isArray(plan.features.features) &&
                            plan.features.features.map((feature: string, idx: number) => (
                              <li key={idx} className="text-sm">
                                {feature}
                              </li>
                            ))}
                        </ul>
                      </div>
                    )}
                  </div>
                </CardBody>
              </Card>
            ))}
          </div>
        )}
      </div>
    </AdminLayout>
  )
}
