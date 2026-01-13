'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { useAuth } from '@/contexts/AuthContext'
import { getSupabaseClient } from '@/lib/supabase/client'
import { useRouter } from 'next/navigation'

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
}

type SubscriptionRow = {
  id: string
  user_id: string
  plan_id: string
  status: string
}

export function PaywallScreen() {
  const supabase = useMemo(() => getSupabaseClient() as any, [])
  const { user, profile } = useAuth()
  const router = useRouter()

  const [loading, setLoading] = useState(true)
  const [subscribing, setSubscribing] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [plans, setPlans] = useState<PlanRow[]>([])
  const [activeSubscription, setActiveSubscription] = useState<SubscriptionRow | null>(null)

  const load = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      // Charger les plans actifs
      const { data: plansData, error: plansError } = await supabase
        .from('saas_paywall_plans')
        .select('*')
        .eq('status', 'active')
        .not('paypal_plan_id', 'is', null)
        .order('sort_order', { ascending: true })

      if (plansError) throw new Error(plansError.message)
      setPlans((plansData ?? []) as PlanRow[])

      // Charger l'abonnement actif de l'utilisateur (si connecté)
      if (user?.id) {
        const { data: subData, error: subError } = await supabase
          .from('saas_paywall_subscriptions')
          .select('*')
          .eq('user_id', user.id)
          .in('status', ['ACTIVE', 'APPROVED', 'APPROVAL_PENDING'])
          .maybeSingle()

        if (subError) console.error('Error loading subscription:', subError)
        setActiveSubscription((subData ?? null) as SubscriptionRow | null)
      }
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e)
      setError(msg)
    } finally {
      setLoading(false)
    }
  }, [supabase, user])

  useEffect(() => {
    void load()
  }, [load])

  const subscribe = useCallback(
    async (planId: string) => {
      if (!user?.id) {
        console.log('[Paywall] User not authenticated, redirecting to signin')
        router.push('/signin?redirect=/pricing')
        return
      }

      console.log('[Paywall] Starting subscription for plan_id:', planId)
      setSubscribing(planId)
      setError(null)

      try {
        const { data: session, error: sessionError } = await supabase.auth.getSession()
        if (sessionError) {
          console.error('[Paywall] Session error:', sessionError)
          throw new Error('session_error')
        }
        if (!session?.session?.access_token) {
          console.error('[Paywall] No access token')
          throw new Error('no_session')
        }

        const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL
        if (!supabaseUrl) {
          console.error('[Paywall] Supabase URL not configured')
          throw new Error('supabase_url_not_configured')
        }

        const returnUrl = `${window.location.origin}/pricing/success`
        const cancelUrl = `${window.location.origin}/pricing/cancel`

        console.log('[Paywall] Calling Edge Function:', `${supabaseUrl}/functions/v1/paywall-create-subscription`)
        console.log('[Paywall] Request payload:', { plan_id: planId, return_url: returnUrl, cancel_url: cancelUrl })

        const response = await fetch(`${supabaseUrl}/functions/v1/paywall-create-subscription`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${session.session.access_token}`,
          },
          body: JSON.stringify({ plan_id: planId, return_url: returnUrl, cancel_url: cancelUrl }),
        })

        console.log('[Paywall] Response status:', response.status)
        const result = await response.json()
        console.log('[Paywall] Response data:', result)
        
        if (!response.ok) {
          console.error('[Paywall] Error response:', result)
          throw new Error(result.error || result.details || 'subscription_creation_failed')
        }

        // Rediriger vers PayPal
        if (result.approve_url) {
          console.log('[Paywall] Redirecting to PayPal:', result.approve_url)
          window.location.href = result.approve_url
        } else {
          console.error('[Paywall] No approve_url in response')
          throw new Error('approve_url_not_found')
        }
      } catch (e) {
        const msg = e instanceof Error ? e.message : String(e)
        console.error('[Paywall] Exception:', msg, e)
        setError(msg)
        setSubscribing(null)
      }
    },
    [user, supabase, router]
  )

  if (loading) {
    return (
      <AppShellScreen>
        <div className="flex items-center justify-center min-h-[60vh]">
          <span className="loading loading-spinner loading-lg text-primary" />
        </div>
      </AppShellScreen>
    )
  }

  return (
    <AppShellScreen>
      <div className="max-w-7xl mx-auto px-4 py-8 sm:px-6 lg:px-8">
        {/* Hero Section */}
        <div className="text-center mb-12">
          <h1 className="text-4xl font-bold mb-4">Choisissez votre abonnement</h1>
          <p className="text-lg text-base-content/60">
            Accédez à toutes les fonctionnalités du service pour les prestataires
          </p>
        </div>

        {error && (
          <div className="alert alert-error mb-6">
            <span className="icon-[tabler--alert-circle] size-5" />
            <span>{error}</span>
          </div>
        )}

        {activeSubscription && (
          <div className="alert alert-info mb-6">
            <span className="icon-[tabler--info-circle] size-5" />
            <span>
              Vous avez déjà un abonnement actif. Vous pouvez le gérer depuis{' '}
              <a href="/subscription" className="link">
                votre page d'abonnement
              </a>
              .
            </span>
          </div>
        )}

        {/* Plans */}
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {plans.map((plan) => {
            const isSubscribing = subscribing === plan.id
            const canSubscribe = !activeSubscription && plan.paypal_plan_id

            return (
              <Card key={plan.id} className={plan.is_featured ? 'ring-2 ring-primary' : ''}>
                <CardHeader>
                  <div className="flex items-center justify-between mb-2">
                    <h3 className="text-xl font-bold">{plan.name}</h3>
                    {plan.is_featured && <Badge variant="primary">Recommandé</Badge>}
                  </div>
                  <div className="mt-4">
                    <span className="text-4xl font-bold">{plan.price_monthly}</span>
                    <span className="text-base-content/60 ml-2">
                      {plan.currency} / {plan.billing_cycle === 'yearly' ? 'an' : 'mois'}
                    </span>
                  </div>
                </CardHeader>
                <CardBody>
                  {plan.description && (
                    <p className="text-base-content/70 mb-4">{plan.description}</p>
                  )}
                  {plan.features && typeof plan.features === 'object' && (
                    <ul className="space-y-2 mb-6">
                      {Array.isArray(plan.features.features) &&
                        plan.features.features.map((feature: string, idx: number) => (
                          <li key={idx} className="flex items-start gap-2">
                            <span className="icon-[tabler--check] size-5 text-success mt-0.5 flex-shrink-0" />
                            <span className="text-sm">{feature}</span>
                          </li>
                        ))}
                    </ul>
                  )}
                  <button
                    className={`w-full btn ${plan.is_featured ? 'btn-primary' : 'btn-outline'}`}
                    onClick={() => subscribe(plan.id)}
                    disabled={!canSubscribe || isSubscribing}
                  >
                    {isSubscribing
                      ? 'Redirection...'
                      : activeSubscription
                        ? 'Déjà abonné'
                        : !plan.paypal_plan_id
                          ? 'Indisponible'
                          : 'S\'abonner'}
                  </button>
                </CardBody>
              </Card>
            )
          })}
        </div>

        {plans.length === 0 && (
          <div className="text-center py-12">
            <p className="text-base-content/60">Aucun plan disponible pour le moment.</p>
          </div>
        )}
      </div>
    </AppShellScreen>
  )
}
