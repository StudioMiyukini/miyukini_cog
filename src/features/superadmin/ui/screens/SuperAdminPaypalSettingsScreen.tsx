'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { useRequireSuperAdmin } from '@/features/backoffice/ui/hooks/useRequireSuperAdmin'
import { useAuth } from '@/contexts/AuthContext'
import { getSupabaseClient } from '@/lib/supabase/client'

const GLOBAL_WORKSPACE_ID = '00000000-0000-0000-0000-000000000000'

type PaypalEnvironment = 'sandbox' | 'live'

type PaypalProviderConfigRow = {
  id: string
  workspace_id: string
  is_active: boolean
  environment: PaypalEnvironment
  client_id: string
  merchant_id: string | null
  webhook_id: string | null
  brand_name: string | null
  return_url: string | null
  cancel_url: string | null
  last_test_at: string | null
  last_test_status: string | null
  last_test_error: string | null
  updated_at: string
}

type FormState = {
  is_active: boolean
  environment: PaypalEnvironment
  client_id: string
  client_secret: string // jamais rechargé
  merchant_id: string
  webhook_id: string
  brand_name: string
  return_url: string
  cancel_url: string
}

const DEFAULT_FORM: FormState = {
  is_active: false,
  environment: 'sandbox',
  client_id: '',
  client_secret: '',
  merchant_id: '',
  webhook_id: '',
  brand_name: '',
  return_url: '',
  cancel_url: '',
}

export function SuperAdminPaypalSettingsScreen() {
  const supabase = useMemo(() => getSupabaseClient() as any, [])
  const { user } = useAuth()
  const { isReady, authLoading } = useRequireSuperAdmin()

  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const [testing, setTesting] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [status, setStatus] = useState<string | null>(null)
  const [testResult, setTestResult] = useState<string | null>(null)

  const [rowId, setRowId] = useState<string | null>(null)
  const [serverCfg, setServerCfg] = useState<PaypalProviderConfigRow | null>(null)
  const [form, setForm] = useState<FormState>(DEFAULT_FORM)
  const [showSecret, setShowSecret] = useState(false)

  const load = useCallback(async () => {
    setLoading(true)
    setError(null)
    setStatus(null)
    setTestResult(null)
    try {
      const { data, error: loadError } = await supabase
        .from('paypal_provider_configs')
        .select(
          'id,workspace_id,is_active,environment,client_id,merchant_id,webhook_id,brand_name,return_url,cancel_url,last_test_at,last_test_status,last_test_error,updated_at',
        )
        .eq('workspace_id', GLOBAL_WORKSPACE_ID)
        .maybeSingle()

      if (loadError) throw new Error(loadError.message)

      const cfg = (data ?? null) as PaypalProviderConfigRow | null
      setRowId(cfg?.id ?? null)
      setServerCfg(cfg)
      setForm({
        is_active: cfg?.is_active ?? false,
        environment: cfg?.environment ?? 'sandbox',
        client_id: cfg?.client_id ?? '',
        client_secret: '', // jamais rechargé
        merchant_id: cfg?.merchant_id ?? '',
        webhook_id: cfg?.webhook_id ?? '',
        brand_name: cfg?.brand_name ?? '',
        return_url: cfg?.return_url ?? '',
        cancel_url: cfg?.cancel_url ?? '',
      })
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e)
      setError(msg)
    } finally {
      setLoading(false)
    }
  }, [supabase])

  useEffect(() => {
    if (!authLoading && isReady) void load()
  }, [authLoading, isReady, load])

  const canSave = useMemo(() => {
    if (!form.client_id.trim()) return false
    // à la création: secret obligatoire; à l'update: optionnel
    if (!rowId && !form.client_secret.trim()) return false
    return true
  }, [form.client_id, form.client_secret, rowId])

  const save = useCallback(async () => {
    setSaving(true)
    setError(null)
    setStatus(null)
    try {
      if (!user?.id) throw new Error('auth_required')
      if (!canSave) throw new Error('form_invalid')

      const patch: Record<string, unknown> = {
        workspace_id: GLOBAL_WORKSPACE_ID,
        is_active: form.is_active,
        environment: form.environment,
        client_id: form.client_id.trim(),
        merchant_id: form.merchant_id.trim() || null,
        webhook_id: form.webhook_id.trim() || null,
        brand_name: form.brand_name.trim() || null,
        return_url: form.return_url.trim() || null,
        cancel_url: form.cancel_url.trim() || null,
        updated_by: user.id,
      }

      if (form.client_secret.trim()) patch.client_secret = form.client_secret

      if (!rowId) {
        const { error: insertError } = await supabase.from('paypal_provider_configs').insert(patch)
        if (insertError) throw new Error(insertError.message)
      } else {
        const { error: updateError } = await supabase.from('paypal_provider_configs').update(patch).eq('id', rowId)
        if (updateError) throw new Error(updateError.message)
      }

      setStatus('Configuration PayPal enregistrée.')
      setForm((f) => ({ ...f, client_secret: '' }))
      await load()
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e)
      setError(msg)
    } finally {
      setSaving(false)
    }
  }, [canSave, form, load, rowId, supabase, user?.id])

  const test = useCallback(async () => {
    setTesting(true)
    setError(null)
    setTestResult(null)
    try {
      const { data, error: fnError } = await supabase.functions.invoke('paypal-test', {
        body: {},
      })
      if (fnError) throw new Error(fnError.message)
      if (!data?.ok) throw new Error(data?.error ?? 'paypal_test_failed')
      setTestResult(`Connexion PayPal OK (${data.environment}).`)
      await load()
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e)
      setError(msg)
    } finally {
      setTesting(false)
    }
  }, [load, supabase.functions])

  if (authLoading || !isReady) {
    return (
      <AdminLayout title="Super Admin • PayPal">
        <div className="min-h-screen flex items-center justify-center">
          <span className="loading loading-spinner text-primary" />
        </div>
      </AdminLayout>
    )
  }

  return (
    <AdminLayout title="Super Admin • PayPal">
      <div className="flex flex-col gap-6">
        <div className="flex items-start justify-between gap-4">
          <div>
            <h1 className="text-2xl font-bold text-base-content">PayPal</h1>
            <p className="text-sm text-base-content/70">Configuration provider paiement (sandbox/live). Accès Super Admin.</p>
          </div>
          <Badge variant="error" style="soft">
            Super Admin
          </Badge>
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

        {testResult && (
          <div className="alert alert-success">
            <span className="icon-[tabler--shield-check] size-5" />
            <span>{testResult}</span>
          </div>
        )}

        <Card className="bg-base-100">
          <CardHeader className="flex items-center justify-between">
            <div>
              <h2 className="text-lg font-semibold">Provider PayPal</h2>
              <p className="text-sm text-base-content/60">Client ID/Secret, environnement, URLs, webhook.</p>
            </div>
            <div className="flex items-center gap-2">
              {rowId ? (
                <Badge variant="success" style="soft">
                  Configurée
                </Badge>
              ) : (
                <Badge variant="warning" style="soft">
                  Non configurée
                </Badge>
              )}
              <button className="btn btn-ghost btn-sm" onClick={() => void load()} disabled={loading || saving || testing}>
                <span className="icon-[tabler--refresh] size-4" />
                Recharger
              </button>
            </div>
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
                <div className="md:col-span-2 flex items-center justify-between gap-3">
                  <label className="label cursor-pointer justify-start gap-3">
                    <input
                      type="checkbox"
                      className="toggle toggle-primary"
                      checked={form.is_active}
                      onChange={(e) => setForm((f) => ({ ...f, is_active: e.target.checked }))}
                    />
                    <span className="label-text">Activer PayPal</span>
                  </label>
                  <div className="text-xs text-base-content/60">
                    {serverCfg?.last_test_status ? (
                      <span>
                        Dernier test: {serverCfg.last_test_status} {serverCfg.last_test_at ? `(${new Date(serverCfg.last_test_at).toLocaleString('fr-FR')})` : ''}
                      </span>
                    ) : (
                      <span>Dernier test: —</span>
                    )}
                  </div>
                </div>

                <div>
                  <label className="label">
                    <span className="label-text font-medium">Environnement</span>
                  </label>
                  <select
                    className="select select-bordered w-full"
                    value={form.environment}
                    onChange={(e) => setForm((f) => ({ ...f, environment: e.target.value as PaypalEnvironment }))}
                  >
                    <option value="sandbox">Sandbox</option>
                    <option value="live">Live</option>
                  </select>
                </div>

                <div>
                  <label className="label">
                    <span className="label-text font-medium">Brand name (optionnel)</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    placeholder="Nom affiché PayPal"
                    value={form.brand_name}
                    onChange={(e) => setForm((f) => ({ ...f, brand_name: e.target.value }))}
                  />
                </div>

                <div className="md:col-span-2">
                  <label className="label">
                    <span className="label-text font-medium">Client ID</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    value={form.client_id}
                    onChange={(e) => setForm((f) => ({ ...f, client_id: e.target.value }))}
                  />
                </div>

                <div className="md:col-span-2">
                  <label className="label">
                    <span className="label-text font-medium">Client Secret</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    type={showSecret ? 'text' : 'password'}
                    placeholder={rowId ? '•••••••• (laisser vide pour ne pas changer)' : 'secret'}
                    value={form.client_secret}
                    onChange={(e) => setForm((f) => ({ ...f, client_secret: e.target.value }))}
                  />
                  <label className="label cursor-pointer justify-start gap-2">
                    <input
                      type="checkbox"
                      className="checkbox checkbox-sm"
                      checked={showSecret}
                      onChange={(e) => setShowSecret(e.target.checked)}
                    />
                    <span className="label-text text-sm">Afficher</span>
                  </label>
                </div>

                <div>
                  <label className="label">
                    <span className="label-text font-medium">Merchant ID (optionnel)</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    value={form.merchant_id}
                    onChange={(e) => setForm((f) => ({ ...f, merchant_id: e.target.value }))}
                  />
                </div>

                <div>
                  <label className="label">
                    <span className="label-text font-medium">Webhook ID (optionnel)</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    value={form.webhook_id}
                    onChange={(e) => setForm((f) => ({ ...f, webhook_id: e.target.value }))}
                  />
                </div>

                <div className="md:col-span-2">
                  <label className="label">
                    <span className="label-text font-medium">Return URL (optionnel)</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    placeholder="https://…/paypal/return"
                    value={form.return_url}
                    onChange={(e) => setForm((f) => ({ ...f, return_url: e.target.value }))}
                  />
                </div>

                <div className="md:col-span-2">
                  <label className="label">
                    <span className="label-text font-medium">Cancel URL (optionnel)</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    placeholder="https://…/paypal/cancel"
                    value={form.cancel_url}
                    onChange={(e) => setForm((f) => ({ ...f, cancel_url: e.target.value }))}
                  />
                </div>

                <div className="md:col-span-2 flex items-center justify-between gap-3 mt-2">
                  <div className="text-xs text-base-content/60">
                    Secret jamais rechargé. Les tests passent par l’Edge Function `paypal-test`.
                  </div>
                  <div className="flex gap-2">
                    <button
                      type="button"
                      className="btn btn-outline"
                      onClick={() => void test()}
                      disabled={!rowId || testing || saving}
                      title="Teste l'obtention d'un token PayPal"
                    >
                      {testing ? (
                        <>
                          <span className="loading loading-spinner loading-xs" />
                          Test…
                        </>
                      ) : (
                        <>
                          <span className="icon-[tabler--activity] size-5" />
                          Tester
                        </>
                      )}
                    </button>
                    <button className="btn btn-primary" disabled={!canSave || saving}>
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
                </div>
              </form>
            )}
          </CardBody>
        </Card>
      </div>
    </AdminLayout>
  )
}

