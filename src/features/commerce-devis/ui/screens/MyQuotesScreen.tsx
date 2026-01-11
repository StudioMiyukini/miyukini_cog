'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { getSupabaseClient } from '@/lib/supabase/client'
import { useAuth } from '@/contexts/AuthContext'
import type { Tables } from '@/lib/supabase/database.types'

type Quote = Tables<'commerce_quotes'>

export function MyQuotesScreen() {
  const supabase = useMemo(() => getSupabaseClient(), [])
  const { isAuthenticated, isLoading: authLoading } = useAuth()

  const [loading, setLoading] = useState(true)
  const [quotes, setQuotes] = useState<Quote[]>([])

  const load = useCallback(async () => {
    setLoading(true)
    try {
      const { data, error } = await supabase.from('commerce_quotes').select('*').order('created_at', { ascending: false }).limit(50)
      if (error) throw error
      setQuotes((data ?? []) as Quote[])
    } finally {
      setLoading(false)
    }
  }, [supabase])

  useEffect(() => {
    if (authLoading) return
    if (!isAuthenticated) {
      setLoading(false)
      setQuotes([])
      return
    }
    void load()
  }, [authLoading, isAuthenticated, load])

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
              <h1 className="text-3xl font-bold text-base-content">Mes devis</h1>
              <p className="text-base-content/60 mt-1">Retrouve tes devis, statuts et factures associées.</p>
            </div>
            <Link href="/devis" className="btn btn-primary btn-sm">
              Demander un devis
              <span className="icon-[tabler--plus] size-4" />
            </Link>
          </div>

          <Card className="mt-6">
            <CardHeader className="flex items-center justify-between">
              <span className="font-semibold">Liste</span>
              <div className="flex items-center gap-2">
                {loading && <span className="loading loading-spinner loading-sm text-primary" />}
                <Badge variant="neutral" style="soft" size="sm">
                  {quotes.length}
                </Badge>
              </div>
            </CardHeader>
            <CardBody className="space-y-3">
              {!isAuthenticated ? (
                <div className="text-base-content/60">Connecte-toi pour voir tes devis.</div>
              ) : loading ? (
                <div className="py-6 text-center">
                  <span className="loading loading-spinner text-primary" />
                </div>
              ) : quotes.length === 0 ? (
                <div className="text-base-content/60">Aucun devis pour l’instant.</div>
              ) : (
                quotes.map((q) => (
                  <Link
                    href={`/quotes/${q.id}`}
                    key={q.id}
                    className="block rounded-2xl border border-base-300 bg-base-100 hover:bg-base-200/40 transition-colors p-4"
                  >
                    <div className="flex items-center justify-between gap-3">
                      <div>
                        <div className="font-semibold text-base-content">{q.quote_number}</div>
                        <div className="text-sm text-base-content/60">
                          Statut: <span className="font-medium">{q.status}</span>
                        </div>
                      </div>
                      <div className="text-right">
                        <div className="text-sm font-semibold">
                          {q.total_gross !== null
                            ? Number(q.total_gross).toLocaleString('fr-FR', { style: 'currency', currency: q.currency ?? 'EUR' })
                            : '—'}
                        </div>
                        <div className="text-xs text-base-content/60">
                          {new Date(q.created_at).toLocaleDateString('fr-FR')}
                        </div>
                      </div>
                    </div>
                  </Link>
                ))
              )}
            </CardBody>
          </Card>
        </div>
      </main>
    </AppShellScreen>
  )
}

