'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { useParams } from 'next/navigation'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { getSupabaseClient } from '@/lib/supabase/client'
import { useAuth } from '@/contexts/AuthContext'
import type { Tables } from '@/lib/supabase/database.types'

type Quote = Tables<'commerce_quotes'>
type QuoteItem = Tables<'commerce_quote_items'>
type Invoice = Tables<'commerce_quote_invoices'>

export function QuoteDetailsScreen() {
  const params = useParams<{ id: string }>()
  const quoteId = params?.id
  const supabase = useMemo(() => getSupabaseClient(), [])
  const { isAuthenticated, isLoading: authLoading } = useAuth()

  const [loading, setLoading] = useState(true)
  const [quote, setQuote] = useState<Quote | null>(null)
  const [items, setItems] = useState<QuoteItem[]>([])
  const [invoices, setInvoices] = useState<Invoice[]>([])
  const [error, setError] = useState<string | null>(null)
  const [rejectReason, setRejectReason] = useState('')

  const load = useCallback(async () => {
    if (!quoteId) return
    setLoading(true)
    setError(null)
    try {
      const [{ data: qData, error: qErr }, { data: iData }, { data: invData }] = await Promise.all([
        supabase.from('commerce_quotes').select('*').eq('id', quoteId).single(),
        supabase.from('commerce_quote_items').select('*').eq('quote_id', quoteId).order('position'),
        supabase.from('commerce_quote_invoices').select('*').eq('quote_id', quoteId).order('created_at', { ascending: false }),
      ])
      if (qErr) throw qErr
      setQuote(qData as Quote)
      setItems((iData ?? []) as QuoteItem[])
      setInvoices((invData ?? []) as Invoice[])
    } catch (e: any) {
      setError(e?.message ?? 'Impossible de charger le devis.')
    } finally {
      setLoading(false)
    }
  }, [quoteId, supabase])

  useEffect(() => {
    if (authLoading) return
    if (!isAuthenticated) return
    void load()
  }, [authLoading, isAuthenticated, load])

  const accept = async () => {
    if (!quoteId) return
    setLoading(true)
    try {
      const { error } = await supabase.rpc('commerce_accept_quote', { p_quote_id: quoteId })
      if (error) throw error
      await load()
    } catch (e: any) {
      setError(e?.message ?? 'Erreur acceptation.')
    } finally {
      setLoading(false)
    }
  }

  const reject = async () => {
    if (!quoteId) return
    setLoading(true)
    try {
      const { error } = await supabase.rpc('commerce_reject_quote', { p_quote_id: quoteId, p_reason: rejectReason || null })
      if (error) throw error
      setRejectReason('')
      await load()
    } catch (e: any) {
      setError(e?.message ?? 'Erreur refus.')
    } finally {
      setLoading(false)
    }
  }

  if (authLoading) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }

  return (
    <AppShellScreen>
      <main className="min-h-screen bg-base-100 px-4 sm:px-6 lg:px-8 py-8">
        <div className="max-w-3xl mx-auto">
          <div className="flex items-start justify-between gap-4">
            <div>
              <h1 className="text-3xl font-bold text-base-content">Détail devis</h1>
              <p className="text-base-content/60 mt-1">Lecture + décision humaine (accepter/refuser).</p>
            </div>
            <Link href="/quotes" className="btn btn-ghost btn-sm">
              <span className="icon-[tabler--arrow-left] size-4" />
              Retour
            </Link>
          </div>

          {error ? (
            <div className="alert alert-error mt-6">
              <span className="icon-[tabler--alert-triangle] size-5" />
              <span>{error}</span>
            </div>
          ) : null}

          {!isAuthenticated ? (
            <div className="mt-6 text-base-content/60">Connecte-toi pour accéder à ce devis.</div>
          ) : loading ? (
            <div className="mt-6 flex items-center justify-center">
              <span className="loading loading-spinner text-primary" />
            </div>
          ) : !quote ? (
            <div className="mt-6 text-base-content/60">Devis introuvable.</div>
          ) : (
            <>
              <Card className="mt-6">
                <CardHeader className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <span className="font-semibold">{quote.quote_number}</span>
                    <Badge variant="neutral" style="soft" size="sm">
                      {quote.status}
                    </Badge>
                  </div>
                  <div className="text-right">
                    <div className="font-semibold">
                      {quote.total_gross !== null
                        ? Number(quote.total_gross).toLocaleString('fr-FR', { style: 'currency', currency: quote.currency ?? 'EUR' })
                        : '—'}
                    </div>
                    <div className="text-xs text-base-content/60">Révision {quote.revision}</div>
                  </div>
                </CardHeader>
                <CardBody className="space-y-3">
                  <div className="text-sm text-base-content/70">
                    Valide jusqu’au:{' '}
                    <span className="font-medium">{quote.valid_until ? new Date(quote.valid_until).toLocaleDateString('fr-FR') : '—'}</span>
                  </div>

                  <div className="divider my-2" />

                  <div className="space-y-2">
                    {items.length === 0 ? (
                      <div className="text-base-content/60">Aucune ligne.</div>
                    ) : (
                      items.map((it) => (
                        <div key={it.id} className="flex items-start justify-between gap-3 rounded-xl border border-base-300 p-3">
                          <div>
                            <div className="font-medium">{it.label}</div>
                            {it.description ? <div className="text-sm text-base-content/60">{it.description}</div> : null}
                            <div className="text-xs text-base-content/50 mt-1">
                              Qté {it.quantity} • TVA {(Number(it.tax_rate) * 100).toFixed(0)}%
                            </div>
                          </div>
                          <div className="text-right font-semibold">
                            {Number(it.unit_price_net).toLocaleString('fr-FR', { style: 'currency', currency: quote.currency ?? 'EUR' })}
                          </div>
                        </div>
                      ))
                    )}
                  </div>

                  {(quote.status === 'sent' || quote.status === 'viewed') && (
                    <div className="pt-2 flex flex-col gap-3">
                      <div className="flex flex-wrap gap-2 justify-end">
                        <button className="btn btn-primary" disabled={loading} onClick={() => void accept()}>
                          J’accepte
                          <span className="icon-[tabler--check] size-5" />
                        </button>
                      </div>
                      <div className="rounded-xl border border-base-300 p-3">
                        <div className="text-sm font-medium">Refuser</div>
                        <textarea
                          className="textarea textarea-bordered w-full mt-2"
                          value={rejectReason}
                          onChange={(e) => setRejectReason(e.target.value)}
                          placeholder="Optionnel : raison du refus"
                        />
                        <div className="mt-2 flex justify-end">
                          <button className="btn btn-ghost text-error" disabled={loading} onClick={() => void reject()}>
                            Refuser le devis
                          </button>
                        </div>
                      </div>
                    </div>
                  )}
                </CardBody>
              </Card>

              <Card className="mt-6">
                <CardHeader className="flex items-center justify-between">
                  <span className="font-semibold">Factures</span>
                  <Badge variant="neutral" style="soft" size="sm">
                    {invoices.length}
                  </Badge>
                </CardHeader>
                <CardBody className="space-y-2">
                  {invoices.length === 0 ? (
                    <div className="text-base-content/60">Aucune facture.</div>
                  ) : (
                    invoices.map((inv) => (
                      <div key={inv.id} className="flex items-center justify-between rounded-xl border border-base-300 p-3">
                        <div>
                          <div className="font-medium">{inv.invoice_number}</div>
                          <div className="text-xs text-base-content/60">{inv.status}</div>
                        </div>
                        <div className="font-semibold">
                          {inv.total_gross !== null
                            ? Number(inv.total_gross).toLocaleString('fr-FR', { style: 'currency', currency: inv.currency ?? 'EUR' })
                            : '—'}
                        </div>
                      </div>
                    ))
                  )}
                </CardBody>
              </Card>
            </>
          )}
        </div>
      </main>
    </AppShellScreen>
  )
}

