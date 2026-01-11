'use client'

import { useEffect, useMemo, useState } from 'react'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { BudgetCategory, BudgetEntry } from '@/capabilities/budget/domain/types'

type Summary = {
  totalIncome: number
  totalExpense: number
  net: number
}

export function BudgetOverviewScreen() {
  const supabase = useMemo(() => getSupabaseClient(), [])
  const [entries, setEntries] = useState<BudgetEntry[]>([])
  const [categories, setCategories] = useState<BudgetCategory[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    let cancelled = false
    const load = async () => {
      setLoading(true)
      try {
        const [{ data: entriesData }, { data: categoriesData }] = await Promise.all([
          supabase.from('budget_entries').select('*').order('entry_date', { ascending: false }).limit(200),
          supabase.from('budget_categories').select('*').order('name'),
        ])
        if (!cancelled) {
          setEntries((entriesData ?? []) as BudgetEntry[])
          setCategories((categoriesData ?? []) as BudgetCategory[])
        }
      } finally {
        if (!cancelled) setLoading(false)
      }
    }
    void load()
    return () => {
      cancelled = true
    }
  }, [supabase])

  const summary = useMemo<Summary>(() => {
    const income = entries
      .filter((entry) => entry.type === 'income')
      .reduce((acc, entry) => acc + Number(entry.amount_gross || 0), 0)
    const expense = entries
      .filter((entry) => entry.type === 'expense' || entry.type === 'refund')
      .reduce((acc, entry) => acc + Number(entry.amount_gross || 0), 0)
    return { totalIncome: income, totalExpense: expense, net: income - expense }
  }, [entries])

  return (
    <AppShellScreen>
      <main className="min-h-screen bg-base-100 px-4 sm:px-6 lg:px-8 py-8">
        <div className="flex items-center justify-between flex-wrap gap-4">
          <div>
            <h1 className="text-3xl font-bold text-base-content">Budget</h1>
            <p className="text-base-content/60">Vue rapide des recettes, dépenses et catégories.</p>
          </div>
        </div>

        <div className="mt-6 grid grid-cols-1 md:grid-cols-3 gap-4">
          <Card>
            <CardHeader>
              <span className="font-semibold">Recettes</span>
            </CardHeader>
            <CardBody>
              <div className="text-3xl font-bold text-success">{summary.totalIncome.toLocaleString('fr-FR', { style: 'currency', currency: 'EUR' })}</div>
              <p className="text-sm text-base-content/60">Montant brut enregistré</p>
            </CardBody>
          </Card>
          <Card>
            <CardHeader>
              <span className="font-semibold">Dépenses</span>
            </CardHeader>
            <CardBody>
              <div className="text-3xl font-bold text-error">{summary.totalExpense.toLocaleString('fr-FR', { style: 'currency', currency: 'EUR' })}</div>
              <p className="text-sm text-base-content/60">Dépenses</p>
            </CardBody>
          </Card>
          <Card>
            <CardHeader>
              <span className="font-semibold">Net</span>
            </CardHeader>
            <CardBody>
              <div className="text-3xl font-bold text-base-content">{summary.net.toLocaleString('fr-FR', { style: 'currency', currency: 'EUR' })}</div>
              <p className="text-sm text-base-content/60">Solde</p>
            </CardBody>
          </Card>
        </div>

        <div className="mt-8 grid grid-cols-1 lg:grid-cols-2 gap-6">
          <Card>
            <CardHeader>
              <span className="font-semibold">Entrées récentes</span>
            </CardHeader>
            <CardBody className="space-y-3 divide-y">
              {loading ? (
                <div className="py-6 text-center">
                  <span className="loading loading-spinner text-primary" />
                </div>
              ) : entries.length === 0 ? (
                <div className="py-6 text-base-content/60">Aucune entrée.</div>
              ) : (
                entries.slice(0, 6).map((entry) => (
                  <div key={entry.id} className="flex items-center justify-between py-2">
                    <div>
                      <div className="text-sm font-semibold text-base-content">{entry.description ?? 'Sans description'}</div>
                      <div className="text-xs text-base-content/60">
                        {entry.entry_date} • {entry.type}
                      </div>
                    </div>
                    <div className="text-right">
                      <span className="text-sm font-semibold text-base-content">
                        {Number(entry.amount_gross).toLocaleString('fr-FR', { style: 'currency', currency: entry.currency ?? 'EUR' })}
                      </span>
                      <div className="text-[11px] text-base-content/50">{entry.status}</div>
                    </div>
                  </div>
                ))
              )}
            </CardBody>
          </Card>
          <Card>
            <CardHeader>
              <span className="font-semibold">Catégories</span>
            </CardHeader>
            <CardBody className="space-y-2">
              {categories.length === 0 ? (
                <div className="py-6 text-base-content/60">Aucune catégorie.</div>
              ) : (
                categories.map((category) => (
                  <div key={category.id} className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <span className={`${category.icon ?? 'icon-[tabler--tag]'} text-primary size-5`} />
                      <div>
                        <div className="text-sm font-medium">{category.name}</div>
                        <div className="text-[11px] text-base-content/60">{category.type}</div>
                      </div>
                    </div>
                    <span className={`badge badge-${category.type === 'income' ? 'success' : 'warning'} badge-soft badge-sm`}>
                      {category.is_active ? 'actif' : 'archivé'}
                    </span>
                  </div>
                ))
              )}
            </CardBody>
          </Card>
        </div>
      </main>
    </AppShellScreen>
  )
}
