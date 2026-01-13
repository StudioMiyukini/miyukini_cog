'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { useAuth } from '@/contexts/AuthContext'
import { getSupabaseClient } from '@/lib/supabase/client'
import { useRouter } from 'next/navigation'

type SubscriptionStatus = 'ACTIVE' | 'APPROVED' | 'APPROVAL_PENDING' | 'SUSPENDED' | 'CANCELLED' | 'EXPIRED' | null

type SubscriptionData = {
  id: string
  status: SubscriptionStatus
  plan_id: string
  plan_name: string | null
  current_period_end: string | null
  next_billing_date: string | null
}

type UseRequireSubscriptionOptions = {
  /**
   * Rediriger vers le paywall si pas d'abonnement
   * @default true
   */
  redirectToPaywall?: boolean
  /**
   * Chemin de redirection personnalisé
   * @default '/pricing'
   */
  redirectPath?: string
  /**
   * Statuts considérés comme valides (abonnement actif)
   * @default ['ACTIVE', 'APPROVED']
   */
  validStatuses?: SubscriptionStatus[]
}

/**
 * Hook pour vérifier qu'un utilisateur a un abonnement actif
 * Utilisé pour protéger les routes prestataires
 */
export function useRequireSubscription(options: UseRequireSubscriptionOptions = {}) {
  const {
    redirectToPaywall = true,
    redirectPath = '/pricing',
    validStatuses = ['ACTIVE', 'APPROVED'],
  } = options

  const supabase = useMemo(() => getSupabaseClient() as any, [])
  const { user, isAuthenticated } = useAuth()
  const router = useRouter()

  const [isLoading, setIsLoading] = useState(true)
  const [hasSubscription, setHasSubscription] = useState(false)
  const [subscription, setSubscription] = useState<SubscriptionData | null>(null)
  const [error, setError] = useState<string | null>(null)

  const checkSubscription = useCallback(async () => {
    if (!isAuthenticated || !user?.id) {
      setIsLoading(false)
      setHasSubscription(false)
      if (redirectToPaywall) {
        router.push(`/signin?redirect=${encodeURIComponent(redirectPath)}`)
      }
      return
    }

    setIsLoading(true)
    setError(null)

    try {
      // Charger l'abonnement actif
      const { data: subData, error: subError } = await supabase
        .from('saas_paywall_subscriptions')
        .select(
          `
          id,
          status,
          plan_id,
          current_period_end,
          next_billing_date,
          plan:saas_paywall_plans(id, name)
        `
        )
        .eq('user_id', user.id)
        .in('status', ['ACTIVE', 'APPROVED', 'APPROVAL_PENDING', 'SUSPENDED', 'CANCELLED'])
        .order('created_at', { ascending: false })
        .limit(1)
        .maybeSingle()

      if (subError) {
        console.error('[useRequireSubscription] Error loading subscription:', subError)
        setError(subError.message)
        setHasSubscription(false)
        setSubscription(null)
        setIsLoading(false)
        return
      }

      if (!subData) {
        // Pas d'abonnement
        setHasSubscription(false)
        setSubscription(null)
        setIsLoading(false)
        if (redirectToPaywall) {
          router.push(redirectPath)
        }
        return
      }

      const subscriptionStatus = subData.status as SubscriptionStatus
      const isValid = validStatuses.includes(subscriptionStatus)

      setSubscription({
        id: subData.id,
        status: subscriptionStatus,
        plan_id: subData.plan_id,
        plan_name: (subData.plan as any)?.name || null,
        current_period_end: subData.current_period_end,
        next_billing_date: subData.next_billing_date,
      })

      setHasSubscription(isValid)

      if (!isValid && redirectToPaywall) {
        // Abonnement invalide (suspendu, annulé, etc.)
        router.push(redirectPath)
      }
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e)
      console.error('[useRequireSubscription] Error:', msg)
      setError(msg)
      setHasSubscription(false)
      setSubscription(null)
    } finally {
      setIsLoading(false)
    }
  }, [supabase, user, isAuthenticated, router, redirectToPaywall, redirectPath, validStatuses])

  useEffect(() => {
    void checkSubscription()
  }, [checkSubscription])

  return {
    isLoading,
    hasSubscription,
    subscription,
    error,
    refresh: checkSubscription,
  }
}
