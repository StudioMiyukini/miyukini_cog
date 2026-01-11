'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { useRequireSuperAdmin } from '@/features/backoffice/ui/hooks/useRequireSuperAdmin'
import { useAuth } from '@/contexts/AuthContext'
import { getSupabaseClient } from '@/lib/supabase/client'

const GLOBAL_WORKSPACE_ID = '00000000-0000-0000-0000-000000000000'

type EmailProviderType = 'smtp' | 'gmail_oauth2' | 'gmail_smtp'

type EmailProviderConfigRow = {
  id: string
  workspace_id: string
  provider: EmailProviderType
  is_active: boolean
  smtp_host: string | null
  smtp_port: number | null
  smtp_username: string | null
  smtp_secure: boolean
  from_email: string | null
  from_name: string | null
  reply_to: string | null
  updated_at: string
  last_test_at: string | null
  last_test_status: string | null
  last_test_error: string | null
}

type FormState = {
  is_active: boolean
  smtp_host: string
  smtp_port: string
  smtp_username: string
  smtp_password: string
  smtp_secure: boolean
  from_email: string
  from_name: string
  reply_to: string
}

const DEFAULT_FORM: FormState = {
  is_active: true,
  smtp_host: '',
  smtp_port: '587',
  smtp_username: '',
  smtp_password: '',
  smtp_secure: false,
  from_email: '',
  from_name: '',
  reply_to: '',
}

export function SuperAdminSmtpSettingsScreen() {
  const supabase = useMemo(() => getSupabaseClient() as any, [])
  const { user } = useAuth()
  const { isReady, authLoading } = useRequireSuperAdmin()

  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [status, setStatus] = useState<string | null>(null)

  const [rowId, setRowId] = useState<string | null>(null)
  const [form, setForm] = useState<FormState>(DEFAULT_FORM)
  const [showPassword, setShowPassword] = useState(false)

  const [testTo, setTestTo] = useState('')
  const [testSubject, setTestSubject] = useState('Test SMTP (Miyukini)')
  const [testText, setTestText] = useState('Ceci est un email de test envoyé depuis le Back Office Super Admin.')
  const [testing, setTesting] = useState(false)
  const [testResult, setTestResult] = useState<string | null>(null)

  const load = useCallback(async () => {
    setLoading(true)
    setError(null)
    setStatus(null)
    try {
      const { data, error: loadError } = await supabase
        .from('email_provider_configs')
        .select(
          'id,workspace_id,provider,is_active,smtp_host,smtp_port,smtp_username,smtp_secure,from_email,from_name,reply_to,updated_at,last_test_at,last_test_status,last_test_error',
        )
        .eq('workspace_id', GLOBAL_WORKSPACE_ID)
        .eq('provider', 'smtp')
        .maybeSingle()

      if (loadError) {
        throw new Error(loadError.message)
      }

      const cfg = (data ?? null) as EmailProviderConfigRow | null
      setRowId(cfg?.id ?? null)
      setForm({
        is_active: cfg?.is_active ?? true,
        smtp_host: cfg?.smtp_host ?? '',
        smtp_port: cfg?.smtp_port ? String(cfg.smtp_port) : '587',
        smtp_username: cfg?.smtp_username ?? '',
        smtp_password: '', // jamais pré-rempli (évite d’exposer le secret)
        smtp_secure: cfg?.smtp_secure ?? false,
        from_email: cfg?.from_email ?? '',
        from_name: cfg?.from_name ?? '',
        reply_to: cfg?.reply_to ?? '',
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
    if (!form.smtp_host.trim()) return false
    if (!form.from_email.trim()) return false
    const port = Number(form.smtp_port)
    if (!Number.isFinite(port) || port <= 0 || port >= 65536) return false
    return true
  }, [form.smtp_host, form.from_email, form.smtp_port])

  const save = useCallback(async () => {
    setSaving(true)
    setError(null)
    setStatus(null)
    try {
      if (!user?.id) throw new Error('auth_required')
      if (!canSave) throw new Error('form_invalid')

      const port = Number(form.smtp_port)

      if (!rowId) {
        // Création: si password vide, on refuse (sinon config inutilisable)
        if (!form.smtp_password.trim()) throw new Error('smtp_password_required')

        const { error: insertError } = await supabase.from('email_provider_configs').insert({
          workspace_id: GLOBAL_WORKSPACE_ID,
          provider: 'smtp',
          is_active: form.is_active,
          smtp_host: form.smtp_host.trim(),
          smtp_port: port,
          smtp_username: form.smtp_username.trim() || null,
          smtp_password: form.smtp_password,
          smtp_secure: form.smtp_secure,
          from_email: form.from_email.trim(),
          from_name: form.from_name.trim() || null,
          reply_to: form.reply_to.trim() || null,
          updated_by: user.id,
        })

        if (insertError) throw new Error(insertError.message)
      } else {
        const patch: Record<string, unknown> = {
          is_active: form.is_active,
          smtp_host: form.smtp_host.trim(),
          smtp_port: port,
          smtp_username: form.smtp_username.trim() || null,
          smtp_secure: form.smtp_secure,
          from_email: form.from_email.trim(),
          from_name: form.from_name.trim() || null,
          reply_to: form.reply_to.trim() || null,
          updated_by: user.id,
        }

        // Ne remplace le mot de passe que si l’input est renseigné
        if (form.smtp_password.trim()) patch.smtp_password = form.smtp_password

        const { error: updateError } = await supabase
          .from('email_provider_configs')
          .update(patch)
          .eq('id', rowId)

        if (updateError) throw new Error(updateError.message)
      }

      setStatus('Configuration enregistrée.')
      setForm((f) => ({ ...f, smtp_password: '' }))
      await load()
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e)
      setError(msg)
    } finally {
      setSaving(false)
    }
  }, [canSave, form, load, rowId, supabase, user?.id])

  const sendTestEmail = useCallback(async () => {
    setTesting(true)
    setError(null)
    setTestResult(null)
    try {
      const to = testTo.trim()
      if (!to) throw new Error('test_to_required')

      const { data, error: fnError } = await supabase.functions.invoke('emailing-smtp-test', {
        body: {
          to,
          subject: testSubject.trim() || 'Test SMTP (Miyukini)',
          text: testText.trim() || 'Email de test.',
        },
      })

      if (fnError) throw new Error(fnError.message)
      if (!data?.ok) throw new Error(data?.error ?? 'smtp_test_failed')

      setTestResult(`Email envoyé à ${to} (${data.sent_at ?? 'ok'}).`)
      await load()
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e)
      setError(msg)
    } finally {
      setTesting(false)
    }
  }, [load, supabase.functions, testSubject, testText, testTo])

  if (authLoading || !isReady) {
    return (
      <AdminLayout title="Super Admin • SMTP">
        <div className="min-h-screen flex items-center justify-center">
          <span className="loading loading-spinner text-primary" />
        </div>
      </AdminLayout>
    )
  }

  return (
    <AdminLayout title="Super Admin • SMTP">
      <div className="flex flex-col gap-6">
        <div className="flex items-start justify-between gap-4">
          <div>
            <h1 className="text-2xl font-bold text-base-content">SMTP (Emailing)</h1>
            <p className="text-sm text-base-content/70">
              Configuration globale utilisée par les Edge Functions (transactionnel). Accès réservé Super Admin.
            </p>
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

        <Card className="bg-base-100">
          <CardHeader className="flex items-center justify-between">
            <div>
              <h2 className="text-lg font-semibold">Provider SMTP</h2>
              <p className="text-sm text-base-content/60">Host, port, identifiants, from/reply-to.</p>
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
              <button className="btn btn-ghost btn-sm" onClick={() => void load()} disabled={loading || saving}>
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
                <div className="md:col-span-2">
                  <label className="label cursor-pointer justify-start gap-3">
                    <input
                      type="checkbox"
                      className="toggle toggle-primary"
                      checked={form.is_active}
                      onChange={(e) => setForm((f) => ({ ...f, is_active: e.target.checked }))}
                    />
                    <span className="label-text">Activer l’envoi SMTP</span>
                  </label>
                </div>

                <div>
                  <label className="label">
                    <span className="label-text font-medium">Hôte SMTP</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    placeholder="smtp.exemple.com"
                    value={form.smtp_host}
                    onChange={(e) => setForm((f) => ({ ...f, smtp_host: e.target.value }))}
                  />
                </div>

                <div>
                  <label className="label">
                    <span className="label-text font-medium">Port</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    inputMode="numeric"
                    placeholder="587"
                    value={form.smtp_port}
                    onChange={(e) => setForm((f) => ({ ...f, smtp_port: e.target.value }))}
                  />
                  <p className="mt-1 text-xs text-base-content/60">
                    587 (STARTTLS) recommandé. 465 si SMTPS (secure).
                  </p>
                </div>

                <div>
                  <label className="label">
                    <span className="label-text font-medium">Utilisateur</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    placeholder="login"
                    value={form.smtp_username}
                    onChange={(e) => setForm((f) => ({ ...f, smtp_username: e.target.value }))}
                  />
                </div>

                <div>
                  <label className="label">
                    <span className="label-text font-medium">Mot de passe</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    type={showPassword ? 'text' : 'password'}
                    placeholder={rowId ? '•••••••• (laisser vide pour ne pas changer)' : 'mot de passe'}
                    value={form.smtp_password}
                    onChange={(e) => setForm((f) => ({ ...f, smtp_password: e.target.value }))}
                  />
                  <label className="label cursor-pointer justify-start gap-2">
                    <input
                      type="checkbox"
                      className="checkbox checkbox-sm"
                      checked={showPassword}
                      onChange={(e) => setShowPassword(e.target.checked)}
                    />
                    <span className="label-text text-sm">Afficher</span>
                  </label>
                </div>

                <div className="md:col-span-2">
                  <label className="label cursor-pointer justify-start gap-3">
                    <input
                      type="checkbox"
                      className="checkbox checkbox-primary"
                      checked={form.smtp_secure}
                      onChange={(e) => setForm((f) => ({ ...f, smtp_secure: e.target.checked }))}
                    />
                    <span className="label-text">Connexion sécurisée (SMTPS 465)</span>
                  </label>
                </div>

                <div>
                  <label className="label">
                    <span className="label-text font-medium">From email</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    placeholder="devis@domaine.fr"
                    value={form.from_email}
                    onChange={(e) => setForm((f) => ({ ...f, from_email: e.target.value }))}
                  />
                </div>

                <div>
                  <label className="label">
                    <span className="label-text font-medium">From name</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    placeholder="Miyukini"
                    value={form.from_name}
                    onChange={(e) => setForm((f) => ({ ...f, from_name: e.target.value }))}
                  />
                </div>

                <div className="md:col-span-2">
                  <label className="label">
                    <span className="label-text font-medium">Reply-to</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    placeholder="support@domaine.fr"
                    value={form.reply_to}
                    onChange={(e) => setForm((f) => ({ ...f, reply_to: e.target.value }))}
                  />
                </div>

                <div className="md:col-span-2 flex items-center justify-between gap-3 mt-2">
                  <div className="text-xs text-base-content/60">
                    {rowId ? (
                      <span>Une partie des champs sensibles (mot de passe) n’est jamais rechargée.</span>
                    ) : (
                      <span>Crée la configuration SMTP globale du framework.</span>
                    )}
                  </div>
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
              </form>
            )}
          </CardBody>
        </Card>

        <Card className="bg-base-100">
          <CardHeader className="flex items-center justify-between">
            <div>
              <h2 className="text-lg font-semibold">Tester la configuration</h2>
              <p className="text-sm text-base-content/60">Envoie un email via l’Edge Function (super_admin uniquement).</p>
            </div>
            <Badge variant="neutral" style="soft">
              emailing-smtp-test
            </Badge>
          </CardHeader>
          <CardBody className="space-y-4">
            {testResult && (
              <div className="alert alert-success">
                <span className="icon-[tabler--check] size-5" />
                <span>{testResult}</span>
              </div>
            )}

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="md:col-span-2">
                <label className="label">
                  <span className="label-text font-medium">Destinataire</span>
                </label>
                <input
                  className="input input-bordered w-full"
                  placeholder="destinataire@domaine.fr"
                  value={testTo}
                  onChange={(e) => setTestTo(e.target.value)}
                />
              </div>

              <div className="md:col-span-2">
                <label className="label">
                  <span className="label-text font-medium">Sujet</span>
                </label>
                <input
                  className="input input-bordered w-full"
                  value={testSubject}
                  onChange={(e) => setTestSubject(e.target.value)}
                />
              </div>

              <div className="md:col-span-2">
                <label className="label">
                  <span className="label-text font-medium">Message</span>
                </label>
                <textarea
                  className="textarea textarea-bordered w-full min-h-28"
                  value={testText}
                  onChange={(e) => setTestText(e.target.value)}
                />
              </div>
            </div>

            <div className="flex items-center justify-between gap-3">
              <div className="text-xs text-base-content/60">
                Le statut du dernier test est enregistré dans `email_provider_configs.last_test_*`.
              </div>
              <button className="btn btn-primary" onClick={() => void sendTestEmail()} disabled={testing}>
                {testing ? (
                  <>
                    <span className="loading loading-spinner loading-xs" />
                    Envoi…
                  </>
                ) : (
                  <>
                    <span className="icon-[tabler--send] size-5" />
                    Envoyer un email de test
                  </>
                )}
              </button>
            </div>
          </CardBody>
        </Card>
      </div>
    </AdminLayout>
  )
}

