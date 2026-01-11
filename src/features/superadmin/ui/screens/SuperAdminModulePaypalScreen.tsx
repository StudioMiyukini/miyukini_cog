'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { useParams } from 'next/navigation'
import Link from 'next/link'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { useAuth } from '@/contexts/AuthContext'
import { useRequireSuperAdmin } from '@/features/backoffice/ui/hooks/useRequireSuperAdmin'
import { getSupabaseClient } from '@/lib/supabase/client'

const GLOBAL_WORKSPACE_ID = '00000000-0000-0000-0000-000000000000'

type PaypalMode = 'orders' | 'subscriptions'
type PaypalIntent = 'CAPTURE' | 'AUTHORIZE'

type Row = {
  id: string
  module_id: string
  is_enabled: boolean
  mode: PaypalMode
  intent: PaypalIntent
  currency: string
  success_url: string | null
  cancel_url: string | null
  enable_refunds: boolean
  enable_webhooks: boolean
  enable_disputes: boolean
  enable_payouts: boolean
  settings_json: any
  updated_at: string
}

type Form = Omit<
  Row,
  | 'id'
  | 'module_id'
  | 'updated_at'
> & { settings_json: string; success_url: string; cancel_url: string }

const defaultForm = (): Form => ({
  is_enabled: false,
  mode: 'orders',
  intent: 'CAPTURE',
  currency: 'EUR',
  success_url: '',
  cancel_url: '',
  enable_refunds: true,
  enable_webhooks: false,
  enable_disputes: false,
  enable_payouts: false,
  settings_json: '{}',
})

export function SuperAdminModulePaypalScreen() {
  const { isReady, authLoading } = useRequireSuperAdmin()
  const { user } = useAuth()
  const supabase = useMemo(() => getSupabaseClient() as any, [])
  const params = useParams() as { moduleId?: string }
  const moduleId = decodeURIComponent(params?.moduleId ?? '')

  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [status, setStatus] = useState<string | null>(null)

  const [rowId, setRowId] = useState<string | null>(null)
  const [form, setForm] = useState<Form>(defaultForm())

  const load = useCallback(async () => {
    if (!moduleId) return
    setLoading(true)
    setError(null)
    setStatus(null)
    try {
      const { data, error: loadError } = await supabase
        .from('module_paypal_configs')
        .select(
          'id,module_id,is_enabled,mode,intent,currency,success_url,cancel_url,enable_refunds,enable_webhooks,enable_disputes,enable_payouts,settings_json,updated_at',
        )
        .eq('workspace_id', GLOBAL_WORKSPACE_ID)
        .eq('module_id', moduleId)
        .maybeSingle()

      if (loadError) throw new Error(loadError.message)

      const r = (data ?? null) as Row | null
      setRowId(r?.id ?? null)
      setForm({
        is_enabled: r?.is_enabled ?? false,
        mode: (r?.mode ?? 'orders') as PaypalMode,
        intent: (r?.intent ?? 'CAPTURE') as PaypalIntent,
        currency: r?.currency ?? 'EUR',
        success_url: r?.success_url ?? '',
        cancel_url: r?.cancel_url ?? '',
        enable_refunds: r?.enable_refunds ?? true,
        enable_webhooks: r?.enable_webhooks ?? false,
        enable_disputes: r?.enable_disputes ?? false,
        enable_payouts: r?.enable_payouts ?? false,
        settings_json: JSON.stringify(r?.settings_json ?? {}, null, 2),
      })
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e))
    } finally {
      setLoading(false)
    }
  }, [moduleId, supabase])

  useEffect(() => {
    if (!authLoading && isReady) void load()
  }, [authLoading, isReady, load])

  const save = useCallback(async () => {
    setSaving(true)
    setError(null)
    setStatus(null)
    try {
      if (!user?.id) throw new Error('auth_required')
      if (!moduleId) throw new Error('module_required')

      let settingsJson: any = {}
      try {
        settingsJson = form.settings_json.trim() ? JSON.parse(form.settings_json) : {}
      } catch {
        throw new Error('settings_json_invalid')
      }

      const patch = {
        workspace_id: GLOBAL_WORKSPACE_ID,
        module_id: moduleId,
        is_enabled: form.is_enabled,
        mode: form.mode,
        intent: form.intent,
        currency: form.currency.trim() || 'EUR',
        success_url: (form.success_url ?? '').trim() || null,
        cancel_url: (form.cancel_url ?? '').trim() || null,
        enable_refunds: form.enable_refunds,
        enable_webhooks: form.enable_webhooks,
        enable_disputes: form.enable_disputes,
        enable_payouts: form.enable_payouts,
        settings_json: settingsJson,
        updated_by: user.id,
      }

      if (!rowId) {
        const { error: insertError } = await supabase.from('module_paypal_configs').insert(patch)
        if (insertError) throw new Error(insertError.message)
      } else {
        const { error: updateError } = await supabase.from('module_paypal_configs').update(patch).eq('id', rowId)
        if (updateError) throw new Error(updateError.message)
      }

      setStatus('Configuration PayPal du module enregistrée.')
      await load()
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e))
    } finally {
      setSaving(false)
    }
  }, [form, load, moduleId, rowId, supabase, user?.id])

  if (authLoading || !isReady) {
    return (
      <AdminLayout title="Super Admin • PayPal (module)">
        <div className="min-h-screen flex items-center justify-center">
          <span className="loading loading-spinner text-primary" />
        </div>
      </AdminLayout>
    )
  }

  return (
    <AdminLayout title={`PayPal • ${moduleId || 'module'}`}>
      <div className="flex flex-col gap-6">
        <div className="flex items-start justify-between gap-4">
          <div>
            <h1 className="text-2xl font-bold text-base-content">PayPal • Configuration module</h1>
            <p className="text-sm text-base-content/70">
              Module: <span className="font-mono">{moduleId}</span> — “toutes les possibilités” via toggles + JSON.
            </p>
          </div>
          <div className="flex items-center gap-2">
            <Badge variant="error" style="soft">
              Super Admin
            </Badge>
            <Link href="/admin/modules" className="btn btn-ghost btn-sm">
              <span className="icon-[tabler--arrow-left] size-4" />
              Modules
            </Link>
          </div>
        </div>

        {error && (
          <div className="alert alert-error">
            <span className="icon-[tabler--alert-triangle] size-5" />
            <span className="font-medium">Erreur</span>
            <span className="ml-2">{error}</span>
          </div>
        )}
        {status && (
          <div className="alert alert-success">
            <span className="icon-[tabler--check] size-5" />
            <span>{status}</span>
          </div>
        )}

        <Card className="bg-base-100">
          <CardHeader className="flex items-center justify-between">
            <div>
              <h2 className="text-lg font-semibold">Settings PayPal</h2>
              <p className="text-sm text-base-content/60">Orders, subscriptions, intent, webhooks, refunds, disputes, payouts.</p>
            </div>
            {rowId ? (
              <Badge variant="success" style="soft">
                Configurée
              </Badge>
            ) : (
              <Badge variant="warning" style="soft">
                Non configurée
              </Badge>
            )}
          </CardHeader>
          <CardBody>
            {loading ? (
              <div className="flex items-center justify-center py-10">
                <span className="loading loading-spinner text-primary" />
              </div>
            ) : (
              <form
                className="grid grid-cols-1 md:grid-cols-2 gap-4"
                onSubmit={(e) => {
                  e.preventDefault()
                  void save()
                }}
              >
                <div className="md:col-span-2">
                  <label className="label cursor-pointer justify-start gap-3">
                    <input
                      type="checkbox"
                      className="toggle toggle-primary"
                      checked={form.is_enabled}
                      onChange={(e) => setForm((f) => ({ ...f, is_enabled: e.target.checked }))}
                    />
                    <span className="label-text">Activer PayPal pour ce module</span>
                  </label>
                </div>

                <div>
                  <label className="label">
                    <span className="label-text font-medium">Mode</span>
                  </label>
                  <select
                    className="select select-bordered w-full"
                    value={form.mode}
                    onChange={(e) => setForm((f) => ({ ...f, mode: e.target.value as PaypalMode }))}
                  >
                    <option value="orders">Orders (one-shot)</option>
                    <option value="subscriptions">Subscriptions (abonnements)</option>
                  </select>
                </div>

                <div>
                  <label className="label">
                    <span className="label-text font-medium">Intent</span>
                  </label>
                  <select
                    className="select select-bordered w-full"
                    value={form.intent}
                    onChange={(e) => setForm((f) => ({ ...f, intent: e.target.value as PaypalIntent }))}
                  >
                    <option value="CAPTURE">CAPTURE</option>
                    <option value="AUTHORIZE">AUTHORIZE</option>
                  </select>
                </div>

                <div>
                  <label className="label">
                    <span className="label-text font-medium">Devise</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    value={form.currency}
                    onChange={(e) => setForm((f) => ({ ...f, currency: e.target.value }))}
                  />
                </div>

                <div className="md:col-span-2">
                  <label className="label">
                    <span className="label-text font-medium">Success URL (optionnel)</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    value={form.success_url}
                    onChange={(e) => setForm((f) => ({ ...f, success_url: e.target.value }))}
                    placeholder="https://…/paypal/success"
                  />
                </div>

                <div className="md:col-span-2">
                  <label className="label">
                    <span className="label-text font-medium">Cancel URL (optionnel)</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    value={form.cancel_url}
                    onChange={(e) => setForm((f) => ({ ...f, cancel_url: e.target.value }))}
                    placeholder="https://…/paypal/cancel"
                  />
                </div>

                <div className="md:col-span-2 grid grid-cols-1 sm:grid-cols-2 gap-3">
                  <label className="label cursor-pointer justify-start gap-3">
                    <input
                      type="checkbox"
                      className="checkbox checkbox-primary"
                      checked={form.enable_refunds}
                      onChange={(e) => setForm((f) => ({ ...f, enable_refunds: e.target.checked }))}
                    />
                    <span className="label-text">Remboursements</span>
                  </label>
                  <label className="label cursor-pointer justify-start gap-3">
                    <input
                      type="checkbox"
                      className="checkbox checkbox-primary"
                      checked={form.enable_webhooks}
                      onChange={(e) => setForm((f) => ({ ...f, enable_webhooks: e.target.checked }))}
                    />
                    <span className="label-text">Webhooks</span>
                  </label>
                  <label className="label cursor-pointer justify-start gap-3">
                    <input
                      type="checkbox"
                      className="checkbox checkbox-primary"
                      checked={form.enable_disputes}
                      onChange={(e) => setForm((f) => ({ ...f, enable_disputes: e.target.checked }))}
                    />
                    <span className="label-text">Litiges (disputes)</span>
                  </label>
                  <label className="label cursor-pointer justify-start gap-3">
                    <input
                      type="checkbox"
                      className="checkbox checkbox-primary"
                      checked={form.enable_payouts}
                      onChange={(e) => setForm((f) => ({ ...f, enable_payouts: e.target.checked }))}
                    />
                    <span className="label-text">Payouts (reverse payouts)</span>
                  </label>
                </div>

                <div className="md:col-span-2">
                  <label className="label">
                    <span className="label-text font-medium">settings_json</span>
                  </label>
                  <textarea
                    className="textarea textarea-bordered w-full min-h-48 font-mono text-sm"
                    value={form.settings_json}
                    onChange={(e) => setForm((f) => ({ ...f, settings_json: e.target.value }))}
                  />
                  <p className="mt-1 text-xs text-base-content/60">JSON libre: landing_page, shipping_preference, subscription_plan_ids, etc.</p>
                </div>

                <div className="md:col-span-2 flex items-center justify-end gap-2">
                  <button className="btn btn-primary" disabled={saving}>
                    {saving ? (
                      <>
                        <span className="loading loading-spinner loading-xs" />
                        Enregistrement…
                      </>
                    ) : (
                      <>
                        <span className="icon-[tabler--device-floppy] size-5" />
                        Enregistrer
                      </>
                    )}
                  </button>
                </div>
              </form>
            )}
          </CardBody>
        </Card>
      </div>
    </AdminLayout>
  )
}

