'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables } from '@/lib/supabase/database.types'
import { useRequireAdmin } from '@/features/backoffice/ui/hooks/useRequireAdmin'

type QuoteRequest = Tables<'commerce_quote_requests'>
type Quote = Tables<'commerce_quotes'>
type Invoice = Tables<'commerce_quote_invoices'>

export function AdminQuotesScreen() {
  const { isReady, authLoading } = useRequireAdmin()
  const supabase = useMemo(() => getSupabaseClient(), [])

  const [loading, setLoading] = useState(true)
  const [requests, setRequests] = useState<QuoteRequest[]>([])
  const [quotes, setQuotes] = useState<Quote[]>([])
  const [invoices, setInvoices] = useState<Invoice[]>([])
  const [error, setError] = useState<string | null>(null)
  const [paymentForm, setPaymentForm] = useState<Record<string, { method: string; reference: string; amount: string; notes: string }>>({})

  const load = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const [{ data: rData, error: rErr }, { data: qData }, { data: iData }] = await Promise.all([
        supabase.from('commerce_quote_requests').select('*').order('created_at', { ascending: false }).limit(100),
        supabase.from('commerce_quotes').select('*').order('created_at', { ascending: false }).limit(100),
        supabase.from('commerce_quote_invoices').select('*').order('created_at', { ascending: false }).limit(100),
      ])
      if (rErr) throw rErr
      setRequests((rData ?? []) as QuoteRequest[])
      setQuotes((qData ?? []) as Quote[])
      setInvoices((iData ?? []) as Invoice[])
    } catch (e: any) {
      setError(e?.message ?? 'Erreur chargement.')
    } finally {
      setLoading(false)
    }
  }, [supabase])

  useEffect(() => {
    if (authLoading) return
    if (!isReady) return
    void load()
  }, [authLoading, isReady, load])

  const createQuote = async (requestId: string) => {
    setLoading(true)
    try {
      const { data, error: rpcErr } = await supabase.rpc('commerce_create_quote_from_request', { p_request_id: requestId })
      if (rpcErr) throw rpcErr
      await load()
      return data
    } finally {
      setLoading(false)
    }
  }

  const sendQuote = async (quoteId: string) => {
    setLoading(true)
    try {
      const { error: rpcErr } = await supabase.rpc('commerce_send_quote', { p_quote_id: quoteId })
      if (rpcErr) throw rpcErr
      await load()
    } finally {
      setLoading(false)
    }
  }

  const createInvoice = async (quoteId: string) => {
    setLoading(true)
    try {
      const { error: rpcErr } = await supabase.rpc('commerce_create_invoice_from_quote', { p_quote_id: quoteId })
      if (rpcErr) throw rpcErr
      await load()
    } finally {
      setLoading(false)
    }
  }

  const confirmPayment = async (invoiceId: string) => {
    const f = paymentForm[invoiceId] ?? { method: 'bank_transfer', reference: '', amount: '', notes: '' }
    setLoading(true)
    try {
      const amount = f.amount ? Number(f.amount) : null
      const { error: rpcErr } = await supabase.rpc('commerce_confirm_payment', {
        p_invoice_id: invoiceId,
        p_method: f.method,
        p_reference: f.reference || null,
        p_amount: amount,
        p_notes: f.notes || null,
      })
      if (rpcErr) throw rpcErr
      await load()
    } finally {
      setLoading(false)
    }
  }

  if (authLoading || !isReady) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }

  const requestsById = new Map(requests.map((r) => [r.id, r]))

  return (
    <AdminLayout title="Devis">
      <div className="flex items-start justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold text-base-content">Commerce par devis</h1>
          <p className="text-base-content/60 mt-1">Inbox → Devis → Facture → Paiement confirmé (manuel).</p>
        </div>
        <div className="flex gap-2">
          <button className="btn btn-ghost btn-sm" onClick={() => void load()} disabled={loading}>
            <span className="icon-[tabler--refresh] size-4" />
            Rafraîchir
          </button>
          <Link href="/admin/quotes/form" className="btn btn-outline btn-sm">
            <span className="icon-[tabler--forms] size-4" />
            Éditer formulaire
          </Link>
          <Link href="/devis" className="btn btn-primary btn-sm">
            Voir formulaire public
            <span className="icon-[tabler--external-link] size-4" />
          </Link>
        </div>
      </div>

      {error ? (
        <div className="alert alert-error mt-6">
          <span className="icon-[tabler--alert-triangle] size-5" />
          <span>{error}</span>
        </div>
      ) : null}

      <div className="mt-6 grid grid-cols-1 xl:grid-cols-3 gap-6">
        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold">Demandes</span>
            <Badge variant="neutral" style="soft" size="sm">
              {requests.length}
            </Badge>
          </CardHeader>
          <CardBody className="space-y-3">
            {loading ? (
              <div className="py-6 text-center">
                <span className="loading loading-spinner text-primary" />
              </div>
            ) : requests.length === 0 ? (
              <div className="text-base-content/60">Aucune demande.</div>
            ) : (
              requests.slice(0, 20).map((r) => (
                <div key={r.id} className="rounded-xl border border-base-300 p-3">
                  <div className="flex items-start justify-between gap-2">
                    <div>
                      <div className="font-medium">{r.title}</div>
                      <div className="text-xs text-base-content/60">{r.status}</div>
                      <div className="text-xs text-base-content/50 mt-1">{r.contact_email ?? '—'}</div>
                    </div>
                    <button className="btn btn-outline btn-xs" disabled={loading} onClick={() => void createQuote(r.id)}>
                      Créer devis
                    </button>
                  </div>
                </div>
              ))
            )}
          </CardBody>
        </Card>

        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold">Devis</span>
            <Badge variant="neutral" style="soft" size="sm">
              {quotes.length}
            </Badge>
          </CardHeader>
          <CardBody className="space-y-3">
            {loading ? (
              <div className="py-6 text-center">
                <span className="loading loading-spinner text-primary" />
              </div>
            ) : quotes.length === 0 ? (
              <div className="text-base-content/60">Aucun devis.</div>
            ) : (
              quotes.slice(0, 20).map((q) => (
                <div key={q.id} className="rounded-xl border border-base-300 p-3">
                  <div className="flex items-start justify-between gap-2">
                    <div className="min-w-0">
                      <div className="font-medium">{q.quote_number}</div>
                      <div className="text-xs text-base-content/60">
                        {q.status} • req {q.quote_request_id.slice(0, 6)}…
                      </div>
                      <div className="text-xs text-base-content/50 truncate">
                        {requestsById.get(q.quote_request_id)?.title ?? '—'}
                      </div>
                    </div>
                    <div className="flex flex-col items-end gap-1">
                      <Link href={`/quotes/${q.id}`} className="btn btn-ghost btn-xs">
                        Voir client
                      </Link>
                      {q.status === 'draft' ? (
                        <button className="btn btn-primary btn-xs" disabled={loading} onClick={() => void sendQuote(q.id)}>
                          Envoyer
                        </button>
                      ) : null}
                      {q.status === 'accepted' ? (
                        <button className="btn btn-outline btn-xs" disabled={loading} onClick={() => void createInvoice(q.id)}>
                          Générer facture
                        </button>
                      ) : null}
                    </div>
                  </div>
                </div>
              ))
            )}
          </CardBody>
        </Card>

        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold">Factures</span>
            <Badge variant="neutral" style="soft" size="sm">
              {invoices.length}
            </Badge>
          </CardHeader>
          <CardBody className="space-y-3">
            {loading ? (
              <div className="py-6 text-center">
                <span className="loading loading-spinner text-primary" />
              </div>
            ) : invoices.length === 0 ? (
              <div className="text-base-content/60">Aucune facture.</div>
            ) : (
              invoices.slice(0, 20).map((inv) => {
                const f = paymentForm[inv.id] ?? { method: 'bank_transfer', reference: '', amount: '', notes: '' }
                return (
                  <div key={inv.id} className="rounded-xl border border-base-300 p-3">
                    <div className="flex items-start justify-between gap-2">
                      <div>
                        <div className="font-medium">{inv.invoice_number}</div>
                        <div className="text-xs text-base-content/60">{inv.status}</div>
                        <div className="text-xs text-base-content/50">
                          {inv.total_gross !== null
                            ? Number(inv.total_gross).toLocaleString('fr-FR', { style: 'currency', currency: inv.currency ?? 'EUR' })
                            : '—'}
                        </div>
                      </div>
                    </div>

                    {inv.status !== 'paid_confirmed' ? (
                      <div className="mt-3 space-y-2">
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                          <select
                            className="select select-bordered select-sm"
                            value={f.method}
                            onChange={(e) => setPaymentForm((p) => ({ ...p, [inv.id]: { ...f, method: e.target.value } }))}
                          >
                            <option value="bank_transfer">Virement</option>
                            <option value="cash">Espèces</option>
                            <option value="check">Chèque</option>
                            <option value="card_remote">CB à distance</option>
                            <option value="other">Autre</option>
                          </select>
                          <input
                            className="input input-bordered input-sm"
                            value={f.reference}
                            onChange={(e) => setPaymentForm((p) => ({ ...p, [inv.id]: { ...f, reference: e.target.value } }))}
                            placeholder="Référence"
                          />
                          <input
                            className="input input-bordered input-sm"
                            value={f.amount}
                            onChange={(e) => setPaymentForm((p) => ({ ...p, [inv.id]: { ...f, amount: e.target.value } }))}
                            placeholder="Montant (optionnel)"
                            type="number"
                          />
                          <input
                            className="input input-bordered input-sm"
                            value={f.notes}
                            onChange={(e) => setPaymentForm((p) => ({ ...p, [inv.id]: { ...f, notes: e.target.value } }))}
                            placeholder="Notes"
                          />
                        </div>
                        <div className="flex justify-end">
                          <button className="btn btn-primary btn-sm" disabled={loading} onClick={() => void confirmPayment(inv.id)}>
                            Confirmer paiement
                          </button>
                        </div>
                      </div>
                    ) : (
                      <div className="mt-2 text-xs text-success">Paiement confirmé → budget alimenté (idempotent).</div>
                    )}
                  </div>
                )
              })
            )}
          </CardBody>
        </Card>
      </div>
    </AdminLayout>
  )
}

