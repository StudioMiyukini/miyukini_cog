'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { getSupabaseClient } from '@/lib/supabase/client'
import { useAuth } from '@/contexts/AuthContext'
import { useRouter } from 'next/navigation'

type ManualPaymentRow = {
  id: string
  subscription_id: string
  payment_method: string
  amount: number
  currency: string
  reference: string | null
  status: string
  due_date: string
  paid_at: string | null
  confirmed_at: string | null
  confirmed_by: string | null
  rejected_at: string | null
  rejected_by: string | null
  rejection_reason: string | null
  notes: string | null
  billing_cycle: number
  created_at: string
  subscription: {
    id: string
    user_id: string
    plan_id: string
    status: string
    user: {
      id: string
      email: string
      display_name: string | null
      first_name: string | null
      last_name: string | null
    } | null
    plan: {
      id: string
      name: string
    } | null
  } | null
}

export function SuperAdminManualPaymentsScreen() {
  const supabase = useMemo(() => getSupabaseClient() as any, [])
  const { profile } = useAuth()
  const router = useRouter()

  const [loading, setLoading] = useState(true)
  const [processing, setProcessing] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [payments, setPayments] = useState<ManualPaymentRow[]>([])

  const [confirmForm, setConfirmForm] = useState<Record<string, { reference: string; paid_at: string; notes: string }>>({})
  const [rejectForm, setRejectForm] = useState<Record<string, { reason: string }>>({})

  const load = useCallback(async () => {
    if (profile?.role !== 'super_admin') {
      router.push('/admin')
      return
    }

    setLoading(true)
    setError(null)
    try {
      const { data, error: loadError } = await supabase
        .from('saas_paywall_manual_payments')
        .select(
          `
          *,
          subscription:saas_paywall_subscriptions(
            id,
            user_id,
            plan_id,
            status,
            user:profiles(id, email, display_name, first_name, last_name),
            plan:saas_paywall_plans(id, name)
          )
        `
        )
        .order('created_at', { ascending: false })
        .limit(100)

      if (loadError) throw new Error(loadError.message)
      setPayments((data ?? []) as ManualPaymentRow[])
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e)
      setError(msg)
    } finally {
      setLoading(false)
    }
  }, [supabase, profile, router])

  useEffect(() => {
    void load()
  }, [load])

  const confirmPayment = useCallback(
    async (paymentId: string) => {
      if (!profile?.id) return

      const form = confirmForm[paymentId] || { reference: '', paid_at: '', notes: '' }
      setProcessing(paymentId)
      setError(null)

      try {
        const paidAt = form.paid_at ? new Date(form.paid_at).toISOString() : null

        const { error: rpcError } = await supabase.rpc('saas_paywall_confirm_manual_payment', {
          p_payment_id: paymentId,
          p_reference: form.reference || null,
          p_paid_at: paidAt,
          p_notes: form.notes || null,
        })

        if (rpcError) throw new Error(rpcError.message)

        await load()
        setConfirmForm((prev) => {
          const next = { ...prev }
          delete next[paymentId]
          return next
        })
      } catch (e) {
        const msg = e instanceof Error ? e.message : String(e)
        setError(msg)
      } finally {
        setProcessing(null)
      }
    },
    [supabase, profile, confirmForm, load]
  )

  const rejectPayment = useCallback(
    async (paymentId: string) => {
      if (!profile?.id) return

      const form = rejectForm[paymentId] || { reason: '' }
      if (!form.reason.trim()) {
        setError('Veuillez indiquer une raison de rejet')
        return
      }

      setProcessing(paymentId)
      setError(null)

      try {
        const { error: rpcError } = await supabase.rpc('saas_paywall_reject_manual_payment', {
          p_payment_id: paymentId,
          p_reason: form.reason,
        })

        if (rpcError) throw new Error(rpcError.message)

        await load()
        setRejectForm((prev) => {
          const next = { ...prev }
          delete next[paymentId]
          return next
        })
      } catch (e) {
        const msg = e instanceof Error ? e.message : String(e)
        setError(msg)
      } finally {
        setProcessing(null)
      }
    },
    [supabase, profile, rejectForm, load]
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

  if (profile?.role !== 'super_admin') {
    return (
      <AppShellScreen>
        <div className="text-center py-12">
          <p className="text-base-content/60">Accès réservé aux super administrateurs.</p>
        </div>
      </AppShellScreen>
    )
  }

  const statusColors: Record<string, 'success' | 'warning' | 'error' | 'secondary'> = {
    pending: 'warning',
    confirmed: 'success',
    rejected: 'error',
    cancelled: 'secondary',
  }

  const statusLabels: Record<string, string> = {
    pending: 'En attente',
    confirmed: 'Confirmé',
    rejected: 'Rejeté',
    cancelled: 'Annulé',
  }

  const paymentMethodLabels: Record<string, string> = {
    bank_transfer: 'Virement bancaire',
    cash: 'Espèces',
    check: 'Chèque',
    card_remote: 'Carte à distance',
    other: 'Autre',
  }

  const pendingPayments = payments.filter((p) => p.status === 'pending')
  const otherPayments = payments.filter((p) => p.status !== 'pending')

  return (
    <AppShellScreen>
      <div className="max-w-7xl mx-auto px-4 py-8 sm:px-6 lg:px-8 space-y-6">
        <div>
          <h1 className="text-3xl font-bold mb-2">Paiements manuels</h1>
          <p className="text-base-content/60">Gérer les paiements par virement bancaire et autres méthodes manuelles</p>
        </div>

        {error && (
          <div className="alert alert-error">
            <span className="icon-[tabler--alert-circle] size-5" />
            <span>{error}</span>
          </div>
        )}

        {/* Paiements en attente */}
        {pendingPayments.length > 0 && (
          <Card>
            <CardHeader>
              <h2 className="text-xl font-bold">Paiements en attente ({pendingPayments.length})</h2>
            </CardHeader>
            <CardBody>
              <div className="overflow-x-auto">
                <table className="table w-full">
                  <thead>
                    <tr>
                      <th>Utilisateur</th>
                      <th>Plan</th>
                      <th>Méthode</th>
                      <th>Montant</th>
                      <th>Échéance</th>
                      <th>Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {pendingPayments.map((payment) => {
                      const user = payment.subscription?.user
                      const plan = payment.subscription?.plan
                      const userName = user?.display_name || `${user?.first_name || ''} ${user?.last_name || ''}`.trim() || user?.email || 'N/A'
                      const form = confirmForm[payment.id] || { reference: '', paid_at: '', notes: '' }
                      const rejectFormData = rejectForm[payment.id] || { reason: '' }

                      return (
                        <tr key={payment.id}>
                          <td>
                            <div>
                              <div className="font-medium">{userName}</div>
                              <div className="text-sm text-base-content/60">{user?.email}</div>
                            </div>
                          </td>
                          <td>{plan?.name || 'N/A'}</td>
                          <td>{paymentMethodLabels[payment.payment_method] || payment.payment_method}</td>
                          <td>
                            {payment.amount} {payment.currency}
                          </td>
                          <td>{new Date(payment.due_date).toLocaleDateString('fr-FR')}</td>
                          <td>
                            <div className="flex flex-col gap-2">
                              {/* Formulaire de confirmation */}
                              <div className="flex gap-2">
                                <input
                                  type="text"
                                  placeholder="Référence"
                                  className="input input-sm input-bordered w-32"
                                  value={form.reference}
                                  onChange={(e) =>
                                    setConfirmForm((prev) => ({
                                      ...prev,
                                      [payment.id]: { ...form, reference: e.target.value },
                                    }))
                                  }
                                />
                                <input
                                  type="date"
                                  className="input input-sm input-bordered w-36"
                                  value={form.paid_at}
                                  onChange={(e) =>
                                    setConfirmForm((prev) => ({
                                      ...prev,
                                      [payment.id]: { ...form, paid_at: e.target.value },
                                    }))
                                  }
                                />
                                <button
                                  className="btn btn-sm btn-success"
                                  onClick={() => confirmPayment(payment.id)}
                                  disabled={processing === payment.id}
                                >
                                  {processing === payment.id ? '...' : 'Confirmer'}
                                </button>
                              </div>
                              {/* Formulaire de rejet */}
                              <div className="flex gap-2">
                                <input
                                  type="text"
                                  placeholder="Raison du rejet"
                                  className="input input-sm input-bordered flex-1"
                                  value={rejectFormData.reason}
                                  onChange={(e) =>
                                    setRejectForm((prev) => ({
                                      ...prev,
                                      [payment.id]: { reason: e.target.value },
                                    }))
                                  }
                                />
                                <button
                                  className="btn btn-sm btn-error"
                                  onClick={() => rejectPayment(payment.id)}
                                  disabled={processing === payment.id || !rejectFormData.reason.trim()}
                                >
                                  Rejeter
                                </button>
                              </div>
                            </div>
                          </td>
                        </tr>
                      )
                    })}
                  </tbody>
                </table>
              </div>
            </CardBody>
          </Card>
        )}

        {/* Autres paiements */}
        {otherPayments.length > 0 && (
          <Card>
            <CardHeader>
              <h2 className="text-xl font-bold">Historique ({otherPayments.length})</h2>
            </CardHeader>
            <CardBody>
              <div className="overflow-x-auto">
                <table className="table w-full">
                  <thead>
                    <tr>
                      <th>Utilisateur</th>
                      <th>Plan</th>
                      <th>Méthode</th>
                      <th>Montant</th>
                      <th>Statut</th>
                      <th>Date</th>
                    </tr>
                  </thead>
                  <tbody>
                    {otherPayments.map((payment) => {
                      const user = payment.subscription?.user
                      const plan = payment.subscription?.plan
                      const userName = user?.display_name || `${user?.first_name || ''} ${user?.last_name || ''}`.trim() || user?.email || 'N/A'

                      return (
                        <tr key={payment.id}>
                          <td>
                            <div>
                              <div className="font-medium">{userName}</div>
                              <div className="text-sm text-base-content/60">{user?.email}</div>
                            </div>
                          </td>
                          <td>{plan?.name || 'N/A'}</td>
                          <td>{paymentMethodLabels[payment.payment_method] || payment.payment_method}</td>
                          <td>
                            {payment.amount} {payment.currency}
                          </td>
                          <td>
                            <Badge variant={statusColors[payment.status] || 'secondary'}>
                              {statusLabels[payment.status] || payment.status}
                            </Badge>
                          </td>
                          <td>
                            {payment.confirmed_at
                              ? new Date(payment.confirmed_at).toLocaleDateString('fr-FR')
                              : payment.rejected_at
                                ? new Date(payment.rejected_at).toLocaleDateString('fr-FR')
                                : new Date(payment.created_at).toLocaleDateString('fr-FR')}
                          </td>
                        </tr>
                      )
                    })}
                  </tbody>
                </table>
              </div>
            </CardBody>
          </Card>
        )}

        {payments.length === 0 && (
          <Card>
            <CardBody className="text-center py-12">
              <p className="text-base-content/60">Aucun paiement manuel pour le moment.</p>
            </CardBody>
          </Card>
        )}
      </div>
    </AppShellScreen>
  )
}
