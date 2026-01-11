'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Badge } from '@/components/atoms/badge'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { useRequireAdmin } from '@/features/backoffice/ui/hooks/useRequireAdmin'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { BudgetEntry, BudgetCategory } from '@/capabilities/budget/domain/types'

const supabase = getSupabaseClient()

export function BudgetAdminScreen() {
  const { isReady, authLoading } = useRequireAdmin()
  const [entries, setEntries] = useState<BudgetEntry[]>([])
  const [categories, setCategories] = useState<BudgetCategory[]>([])
  const [loading, setLoading] = useState(true)
  const [form, setForm] = useState({ description: '', amount: '0', type: 'income', categoryId: '' })

  const load = useCallback(async () => {
    setLoading(true)
    try {
      const [{ data: entriesData }, { data: categoriesData }] = await Promise.all([
        supabase.from('budget_entries').select('*').order('entry_date', { ascending: false }).limit(100),
        supabase.from('budget_categories').select('*').order('name'),
      ])
      setEntries((entriesData ?? []) as BudgetEntry[])
      setCategories((categoriesData ?? []) as BudgetCategory[])
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    if (!authLoading && isReady) void load()
  }, [authLoading, isReady, load])

  if (authLoading || !isReady) {
    return (
      <AdminLayout title="Budget">
        <div className="min-h-screen flex items-center justify-center">
          <span className="loading loading-spinner text-primary" />
        </div>
      </AdminLayout>
    )
  }

  const total = entries.reduce((acc, entry) => acc + Number(entry.amount_gross ?? 0), 0)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    await supabase.from('budget_entries').insert({
      workspace_id: categories[0]?.workspace_id ?? '00000000-0000-0000-0000-000000000000',
      module_id: 'budget',
      entry_date: new Date().toISOString().split('T')[0],
      amount_gross: form.amount,
      amount_net: form.amount,
      currency: 'EUR',
      type: form.type,
      status: 'draft',
      category_id: form.categoryId || null,
      description: form.description,
    })
    setForm({ description: '', amount: '0', type: 'income', categoryId: '' })
    void load()
  }

  return (
    <AdminLayout title="Budget">
      <div className="flex flex-col gap-6">
        <Card>
          <CardHeader className="flex items-center justify-between">
            <div>
              <h2 className="text-xl font-semibold">Entrées</h2>
              <p className="text-sm text-base-content/70">Ajouter, filtrer, rapprocher.</p>
            </div>
            <Badge variant="neutral" style="soft">
              Total {total.toLocaleString('fr-FR', { style: 'currency', currency: 'EUR' })}
            </Badge>
          </CardHeader>
          <CardBody>
            <form className="grid grid-cols-1 md:grid-cols-4 gap-3 mb-4" onSubmit={handleSubmit}>
              <input
                className="input input-bordered"
                placeholder="Description"
                value={form.description}
                onChange={(e) => setForm((prev) => ({ ...prev, description: e.target.value }))}
              />
              <input
                className="input input-bordered"
                placeholder="Montant"
                type="number"
                value={form.amount}
                onChange={(e) => setForm((prev) => ({ ...prev, amount: e.target.value }))}
              />
              <select
                className="select select-bordered"
                value={form.type}
                onChange={(e) => setForm((prev) => ({ ...prev, type: e.target.value }))}
              >
                <option value="income">Recette</option>
                <option value="expense">Dépense</option>
                <option value="refund">Remboursement</option>
                <option value="transfer">Transfert</option>
              </select>
              <select
                className="select select-bordered"
                value={form.categoryId}
                onChange={(e) => setForm((prev) => ({ ...prev, categoryId: e.target.value }))}
              >
                <option value="">Catégorie</option>
                {categories.map((cat) => (
                  <option key={cat.id} value={cat.id}>
                    {cat.name}
                  </option>
                ))}
              </select>
              <div className="md:col-span-4 flex flex-wrap gap-2">
                <button className="btn btn-primary" type="submit">
                  Ajouter entrée
                </button>
                <button className="btn btn-ghost" type="button" onClick={() => void load()}>
                  Rafraîchir
                </button>
              </div>
            </form>
            {loading ? (
              <div className="py-6 text-center text-sm text-base-content/60">
                <span className="loading loading-spinner" />
              </div>
            ) : entries.length === 0 ? (
              <div className="text-sm text-base-content/60">Aucune entrée.</div>
            ) : (
              <div className="overflow-x-auto">
                <table className="table table-zebra w-full">
                  <thead>
                    <tr>
                      <th>Date</th>
                      <th>Description</th>
                      <th>Montant</th>
                      <th>Type</th>
                      <th>Statut</th>
                    </tr>
                  </thead>
                  <tbody>
                    {entries.slice(0, 12).map((entry) => (
                      <tr key={entry.id}>
                        <td>{entry.entry_date}</td>
                        <td>{entry.description ?? '—'}</td>
                        <td>{Number(entry.amount_gross).toLocaleString('fr-FR', { style: 'currency', currency: entry.currency })}</td>
                        <td>{entry.type}</td>
                        <td>{entry.status}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </CardBody>
        </Card>
        <Card>
          <CardHeader>
            <div>
              <h2 className="text-xl font-semibold">Catégories</h2>
              <p className="text-sm text-base-content/70">Organisation hiérarchique des postes budgétaires.</p>
            </div>
          </CardHeader>
          <CardBody>
            <div className="flex flex-wrap gap-2">
              {categories.map((category) => (
                <div key={category.id} className="badge badge-lg badge-outline">
                  <span className={`${category.icon ?? 'icon-[tabler--tag]'} size-4`} />
                  {category.name}
                </div>
              ))}
            </div>
          </CardBody>
        </Card>
      </div>
    </AdminLayout>
  )
}
