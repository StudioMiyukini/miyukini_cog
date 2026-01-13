'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { useAuth } from '@/contexts/AuthContext'
import { getSupabaseClient } from '@/lib/supabase/client'
import { useRouter } from 'next/navigation'

type SubscriptionRow = {
  id: string
  user_id: string
  plan_id: string
  paypal_subscription_id: string
  status: string
  start_date: string | null
  end_date: string | null
  current_period_start: string | null
  current_period_end: string | null
  next_billing_date: string | null
  cancel_at_period_end: boolean
  cancelled_at: string | null
  created_at: string
  plan: {
    id: string
    name: string
    price_monthly: number
    currency: string
  }
}

type TransactionRow = {
  id: string
  subscription_id: string
  paypal_transaction_id: string
  billing_cycle: number
  status: string
  amount: number
  currency: string
  transaction_time: string
  created_at: string
}

export function UserSubscriptionScreen() {
  const supabase = useMemo(() => getSupabaseClient() as any, [])
  const { user } = useAuth()
  const router = useRouter()

  const [loading, setLoading] = useState(true)
  const [cancelling, setCancelling] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [subscription, setSubscription] = useState<SubscriptionRow | null>(null)
  const [transactions, setTransactions] = useState<TransactionRow[]>([])

  const load = useCallback(async () => {
    if (!user?.id) {
      router.push('/signin?redirect=/subscription')
      return
    }

    setLoading(true)
    setError(null)
    try {
      // Charger l'abonnement actif
      const { data: subData, error: subError } = await supabase
        .from('saas_paywall_subscriptions')
        .select(
          `
          *,
          plan:saas_paywall_plans(id, name, price_monthly, currency),
          manual_payments:saas_paywall_manual_payments(id, status, amount, due_date, confirmed_at, rejected_at, rejection_reason)
        `
        )
        .eq('user_id', user.id)
        .in('status', ['ACTIVE', 'APPROVED', 'APPROVAL_PENDING', 'SUSPENDED', 'CANCELLED'])
        .order('created_at', { ascending: false })
        .limit(1)
        .maybeSingle()

      if (subError) throw new Error(subError.message)
      setSubscription((subData ?? null) as SubscriptionRow | null)

      // Charger les transactions si abonnement actif
      if (subData) {
        const { data: transData, error: transError } = await supabase
          .from('saas_paywall_subscription_transactions')
          .select('*')
          .eq('subscription_id', subData.id)
          .order('transaction_time', { ascending: false })
          .limit(20)

        if (transError) console.error('Error loading transactions:', transError)
        setTransactions((transData ?? []) as TransactionRow[])
      }
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e)
      setError(msg)
    } finally {
      setLoading(false)
    }
  }, [supabase, user, router])

  useEffect(() => {
    void load()
  }, [load])

  const cancelSubscription = useCallback(async () => {
    if (!subscription || !user?.id) return

    if (!confirm('Êtes-vous sûr de vouloir annuler votre abonnement ?')) {
      return
    }

    setCancelling(true)
    setError(null)

    try {
      const { data: session } = await supabase.auth.getSession()
      if (!session?.session?.access_token) throw new Error('no_session')

      const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL
      if (!supabaseUrl) throw new Error('supabase_url_not_configured')

      // Appeler l'Edge Function pour annuler (à créer si nécessaire)
      // Pour l'instant, on met juste à jour le statut localement
      const { error: updateError } = await supabase
        .from('saas_paywall_subscriptions')
        .update({
          cancel_at_period_end: true,
          cancelled_at: new Date().toISOString(),
        })
        .eq('id', subscription.id)

      if (updateError) throw new Error(updateError.message)

      await load()
      alert('Votre abonnement sera annulé à la fin de la période de facturation en cours.')
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e)
      setError(msg)
    } finally {
      setCancelling(false)
    }
  }, [subscription, user, supabase, load])

  if (loading) {
    return (
      <AppShellScreen>
        <div className="flex items-center justify-center min-h-[60vh]">
          <span className="loading loading-spinner loading-lg text-primary" />
        </div>
      </AppShellScreen>
    )
  }

  if (!subscription) {
    return (
      <AppShellScreen>
        <div className="max-w-2xl mx-auto px-4 py-8 sm:px-6 lg:px-8">
          <Card>
            <CardBody className="text-center py-12">
              <div className="mb-6">
                <span className="icon-[tabler--info-circle] size-16 text-base-content/40 mx-auto" />
              </div>
              <h2 className="text-2xl font-bold mb-2">Aucun abonnement actif</h2>
              <p className="text-base-content/60 mb-6">
                Vous n'avez pas d'abonnement actif. Choisissez un plan pour commencer.
              </p>
              <button className="btn btn-primary" onClick={() => router.push('/pricing')}>Voir les plans</button>
            </CardBody>
          </Card>
        </div>
      </AppShellScreen>
    )
  }

  const statusColors: Record<string, 'success' | 'info' | 'warning' | 'error' | 'secondary'> = {
    ACTIVE: 'success',
    APPROVED: 'info',
    APPROVAL_PENDING: 'warning',
    SUSPENDED: 'error',
    CANCELLED: 'secondary',
  }

  const statusLabels: Record<string, string> = {
    ACTIVE: 'Actif',
    APPROVED: 'Approuvé',
    APPROVAL_PENDING: 'En attente',
    SUSPENDED: 'Suspendu',
    CANCELLED: 'Annulé',
  }

  return (
    <AppShellScreen>
      <div className="max-w-4xl mx-auto px-4 py-8 sm:px-6 lg:px-8 space-y-6">
        <div>
          <h1 className="text-3xl font-bold mb-2">Mon abonnement</h1>
          <p className="text-base-content/60">Gérez votre abonnement et consultez l'historique</p>
        </div>

        {error && (
          <div className="alert alert-error">
            <span className="icon-[tabler--alert-circle] size-5" />
            <span>{error}</span>
          </div>
        )}

        {/* Détails de l'abonnement */}
        <Card>
          <CardHeader>
            <div className="flex items-center justify-between">
              <div>
                <h2 className="text-xl font-bold">
                  {subscription.plan?.name || 'Plan inconnu'}
                </h2>
                <Badge variant={statusColors[subscription.status] || 'secondary'} className="mt-2">
                  {statusLabels[subscription.status] || subscription.status}
                </Badge>
                {(subscription as any).payment_method && (subscription as any).payment_method !== 'paypal' && (
                  <Badge variant="info" className="mt-2 ml-2">
                    Paiement: {(subscription as any).payment_method === 'bank_transfer' ? 'Virement bancaire' : (subscription as any).payment_method}
                  </Badge>
                )}
              </div>
              <div className="text-right">
                <p className="text-2xl font-bold">
                  {subscription.plan?.price_monthly} {subscription.plan?.currency}
                </p>
                <p className="text-sm text-base-content/60">par mois</p>
              </div>
            </div>
          </CardHeader>
          <CardBody>
            {/* Avertissement si paiement manuel en attente */}
            {(subscription as any).payment_method && (subscription as any).payment_method !== 'paypal' && subscription.status === 'APPROVAL_PENDING' && (
              <div className="alert alert-warning mb-4">
                <span className="icon-[tabler--clock] size-5" />
                <div className="flex-1">
                  <h3 className="font-semibold">Paiement en attente de confirmation</h3>
                  <p className="text-sm">
                    Votre demande d'abonnement avec paiement par virement bancaire est en attente de confirmation par un administrateur.
                    {((subscription as any).manual_payments?.[0] as any)?.due_date && (
                      <> Date d'échéance : {new Date(((subscription as any).manual_payments?.[0] as any)?.due_date).toLocaleDateString('fr-FR')}</>
                    )}
                  </p>
                </div>
              </div>
            )}
            <div className="grid gap-4 md:grid-cols-2">
              {subscription.start_date && (
                <div>
                  <p className="text-sm text-base-content/60">Date de début</p>
                  <p className="font-semibold">
                    {new Date(subscription.start_date).toLocaleDateString('fr-FR')}
                  </p>
                </div>
              )}
              {subscription.next_billing_date && (
                <div>
                  <p className="text-sm text-base-content/60">Prochain paiement</p>
                  <p className="font-semibold">
                    {new Date(subscription.next_billing_date).toLocaleDateString('fr-FR')}
                  </p>
                </div>
              )}
              {subscription.current_period_end && (
                <div>
                  <p className="text-sm text-base-content/60">Fin de période</p>
                  <p className="font-semibold">
                    {new Date(subscription.current_period_end).toLocaleDateString('fr-FR')}
                  </p>
                </div>
              )}
              {subscription.cancel_at_period_end && (
                <div>
                  <p className="text-sm text-base-content/60">Statut</p>
                  <p className="font-semibold text-warning">Annulation programmée</p>
                </div>
              )}
            </div>

            {subscription.status === 'ACTIVE' && !subscription.cancel_at_period_end && (
              <div className="mt-6 pt-6 border-t border-base-300">
                <button className="btn btn-outline" onClick={cancelSubscription} disabled={cancelling}>
                  {cancelling ? 'Annulation...' : 'Annuler l\'abonnement'}
                </button>
              </div>
            )}
          </CardBody>
        </Card>

        {/* Historique des transactions */}
        {transactions.length > 0 && (
          <Card>
            <CardHeader>
              <h2 className="text-xl font-bold">Historique des paiements</h2>
            </CardHeader>
            <CardBody>
              <div className="overflow-x-auto">
                <table className="table w-full">
                  <thead>
                    <tr>
                      <th>Date</th>
                      <th>Cycle</th>
                      <th>Montant</th>
                      <th>Statut</th>
                    </tr>
                  </thead>
                  <tbody>
                    {transactions.map((transaction) => (
                      <tr key={transaction.id}>
                        <td>
                          {new Date(transaction.transaction_time).toLocaleDateString('fr-FR')}
                        </td>
                        <td>#{transaction.billing_cycle}</td>
                        <td>
                          {transaction.amount} {transaction.currency}
                        </td>
                        <td>
                          <Badge
                            variant={
                              (transaction.status === 'COMPLETED'
                                ? 'success'
                                : transaction.status === 'DECLINED'
                                  ? 'error'
                                  : 'warning') as 'success' | 'error' | 'warning'
                            }
                          >
                            {transaction.status}
                          </Badge>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </CardBody>
          </Card>
        )}
      </div>
    </AppShellScreen>
  )
}
