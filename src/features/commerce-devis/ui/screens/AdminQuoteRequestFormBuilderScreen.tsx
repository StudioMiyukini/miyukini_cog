'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { useRequireAdmin } from '@/features/backoffice/ui/hooks/useRequireAdmin'
import { getSupabaseClient } from '@/lib/supabase/client'
import { commerceDevisModuleManifest } from '@/modules/commerce-devis/manifest'
import { useAuth } from '@/contexts/AuthContext'

const GLOBAL_WORKSPACE_ID = '00000000-0000-0000-0000-000000000000'
const FORM_KEY = 'quote_request'

type FieldType = 'text' | 'textarea' | 'email' | 'tel' | 'select' | 'checkbox'

type Binding =
  | 'title'
  | 'description'
  | 'tags'
  | 'category'
  | 'preferredChannel'
  | 'contactEmail'
  | 'contactPhone'
  | `custom:${string}`

type FormField = {
  id: string
  label: string
  type: FieldType
  binding: Binding
  required?: boolean
  placeholder?: string
  help?: string
  options?: { label: string; value: string }[] // select
  hideWhenAuthenticated?: boolean // ex: contactEmail
}

type FormSchema = {
  version: 1
  title?: string
  description?: string
  fields: FormField[]
}

const defaultSchema: FormSchema = {
  version: 1,
  title: 'Demander un devis',
  description: 'Décris ton besoin. Un humain te répondra avec un devis.',
  fields: [
    { id: crypto.randomUUID(), label: 'Titre', type: 'text', binding: 'title', required: true },
    { id: crypto.randomUUID(), label: 'Description', type: 'textarea', binding: 'description' },
    { id: crypto.randomUUID(), label: 'Tags', type: 'text', binding: 'tags', placeholder: 'ex: urgent, VIP' },
    { id: crypto.randomUUID(), label: 'Catégorie', type: 'text', binding: 'category' },
    {
      id: crypto.randomUUID(),
      label: 'Canal préféré',
      type: 'select',
      binding: 'preferredChannel',
      options: [
        { label: 'Email', value: 'email' },
        { label: 'Téléphone', value: 'phone' },
      ],
    },
    {
      id: crypto.randomUUID(),
      label: 'Email de contact',
      type: 'email',
      binding: 'contactEmail',
      required: true,
      hideWhenAuthenticated: true,
    },
    { id: crypto.randomUUID(), label: 'Téléphone', type: 'tel', binding: 'contactPhone' },
  ],
}

function safeJsonParse(input: string) {
  try {
    return { ok: true as const, value: JSON.parse(input) }
  } catch (e) {
    return { ok: false as const, error: e instanceof Error ? e.message : String(e) }
  }
}

export function AdminQuoteRequestFormBuilderScreen() {
  const supabase = useMemo(() => getSupabaseClient() as any, [])
  const { user } = useAuth()
  const { isReady, authLoading } = useRequireAdmin()

  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [status, setStatus] = useState<string | null>(null)

  const [rowId, setRowId] = useState<string | null>(null)
  const [schema, setSchema] = useState<FormSchema>(defaultSchema)
  const [jsonEditor, setJsonEditor] = useState<string>(JSON.stringify(defaultSchema, null, 2))
  const [mode, setMode] = useState<'builder' | 'json'>('builder')

  const load = useCallback(async () => {
    setLoading(true)
    setError(null)
    setStatus(null)
    try {
      const { data, error: loadError } = await supabase
        .from('module_form_configs')
        .select('id,is_active,schema_json,updated_at')
        .eq('workspace_id', GLOBAL_WORKSPACE_ID)
        .eq('module_id', commerceDevisModuleManifest.id)
        .eq('form_key', FORM_KEY)
        .maybeSingle()

      if (loadError) throw new Error(loadError.message)
      const schemaJson = (data?.schema_json ?? null) as FormSchema | null
      const next = schemaJson?.fields?.length ? schemaJson : defaultSchema
      setRowId(data?.id ?? null)
      setSchema(next)
      setJsonEditor(JSON.stringify(next, null, 2))
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e))
    } finally {
      setLoading(false)
    }
  }, [supabase])

  useEffect(() => {
    if (!authLoading && isReady) void load()
  }, [authLoading, isReady, load])

  const addField = () => {
    const field: FormField = {
      id: crypto.randomUUID(),
      label: 'Nouveau champ',
      type: 'text',
      binding: `custom:${crypto.randomUUID().slice(0, 8)}`,
    }
    setSchema((s) => ({ ...s, fields: [...s.fields, field] }))
  }

  const moveField = (idx: number, dir: -1 | 1) => {
    setSchema((s) => {
      const next = [...s.fields]
      const j = idx + dir
      if (j < 0 || j >= next.length) return s
      ;[next[idx], next[j]] = [next[j], next[idx]]
      return { ...s, fields: next }
    })
  }

  const deleteField = (idx: number) => {
    setSchema((s) => ({ ...s, fields: s.fields.filter((_, i) => i !== idx) }))
  }

  const updateField = (idx: number, patch: Partial<FormField>) => {
    setSchema((s) => ({
      ...s,
      fields: s.fields.map((f, i) => (i === idx ? { ...f, ...patch } : f)),
    }))
  }

  const syncJsonFromBuilder = () => setJsonEditor(JSON.stringify(schema, null, 2))
  const syncBuilderFromJson = () => {
    const parsed = safeJsonParse(jsonEditor)
    if (!parsed.ok) throw new Error(parsed.error)
    const v = parsed.value as FormSchema
    if (!v || v.version !== 1 || !Array.isArray(v.fields)) throw new Error('schema_invalid')
    setSchema(v)
  }

  const save = useCallback(async () => {
    setSaving(true)
    setError(null)
    setStatus(null)
    try {
      if (!user?.id) throw new Error('auth_required')

      if (mode === 'json') {
        syncBuilderFromJson()
      } else {
        syncJsonFromBuilder()
      }

      // On prend la version la plus récente (builder)
      const toSave = mode === 'json' ? (safeJsonParse(jsonEditor).value as FormSchema) : schema

      const patch = {
        workspace_id: GLOBAL_WORKSPACE_ID,
        module_id: commerceDevisModuleManifest.id,
        form_key: FORM_KEY,
        is_active: true,
        schema_json: toSave,
        updated_by: user.id,
      }

      if (!rowId) {
        const { error: insertError } = await supabase.from('module_form_configs').insert(patch)
        if (insertError) throw new Error(insertError.message)
      } else {
        const { error: updateError } = await supabase.from('module_form_configs').update(patch).eq('id', rowId)
        if (updateError) throw new Error(updateError.message)
      }

      setStatus('Formulaire enregistré.')
      await load()
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e))
    } finally {
      setSaving(false)
    }
  }, [jsonEditor, load, mode, rowId, schema, supabase, user?.id])

  if (authLoading || !isReady) {
    return (
      <AdminLayout title="Devis • Formulaire">
        <div className="min-h-screen flex items-center justify-center">
          <span className="loading loading-spinner text-primary" />
        </div>
      </AdminLayout>
    )
  }

  return (
    <AdminLayout title="Devis • Formulaire">
      <div className="flex flex-col gap-6">
        <div className="flex items-start justify-between gap-4">
          <div>
            <h1 className="text-2xl font-bold text-base-content">Éditeur de formulaire • Demande de devis</h1>
            <p className="text-sm text-base-content/70">Configure le formulaire public du module Commerce par Devis.</p>
          </div>
          <div className="flex items-center gap-2">
            <Badge variant="neutral" style="soft">
              {commerceDevisModuleManifest.id}
            </Badge>
            <Link href="/admin/quotes" className="btn btn-ghost btn-sm">
              <span className="icon-[tabler--arrow-left] size-4" />
              Retour Devis
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
              <h2 className="text-lg font-semibold">Configuration</h2>
              <p className="text-sm text-base-content/60">Mode builder + fallback JSON.</p>
            </div>
            <div className="flex items-center gap-2">
              <div className="tabs tabs-boxed bg-base-200 p-1 rounded-xl">
                <button
                  className={`tab ${mode === 'builder' ? 'tab-active' : ''}`}
                  onClick={() => {
                    syncJsonFromBuilder()
                    setMode('builder')
                  }}
                >
                  Builder
                </button>
                <button
                  className={`tab ${mode === 'json' ? 'tab-active' : ''}`}
                  onClick={() => {
                    syncJsonFromBuilder()
                    setMode('json')
                  }}
                >
                  JSON
                </button>
              </div>
              <button className="btn btn-ghost btn-sm" onClick={() => void load()} disabled={loading || saving}>
                <span className="icon-[tabler--refresh] size-4" />
                Recharger
              </button>
              <button className="btn btn-primary btn-sm" onClick={() => void save()} disabled={saving}>
                {saving ? (
                  <>
                    <span className="loading loading-spinner loading-xs" />
                    Enregistrement…
                  </>
                ) : (
                  <>
                    <span className="icon-[tabler--device-floppy] size-4" />
                    Enregistrer
                  </>
                )}
              </button>
            </div>
          </CardHeader>
          <CardBody>
            {loading ? (
              <div className="flex items-center justify-center py-10">
                <span className="loading loading-spinner text-primary" />
              </div>
            ) : mode === 'json' ? (
              <div className="space-y-3">
                <textarea
                  className="textarea textarea-bordered w-full min-h-96 font-mono text-sm"
                  value={jsonEditor}
                  onChange={(e) => setJsonEditor(e.target.value)}
                />
                <p className="text-xs text-base-content/60">
                  Schema: `{`{ version: 1, fields: [...] }`}`. Les champs custom sont stockés dans `meta.custom_fields`.
                </p>
              </div>
            ) : (
              <div className="space-y-4">
                <div className="flex items-center justify-between gap-2">
                  <div className="text-sm text-base-content/70">
                    <strong>{schema.fields.length}</strong> champ(s)
                  </div>
                  <button className="btn btn-outline btn-sm" onClick={addField}>
                    <span className="icon-[tabler--plus] size-4" />
                    Ajouter un champ
                  </button>
                </div>

                <div className="space-y-3">
                  {schema.fields.map((f, idx) => (
                    <div key={f.id} className="p-4 rounded-xl border border-base-300 bg-base-200/40">
                      <div className="flex items-start justify-between gap-3">
                        <div className="flex-1 grid grid-cols-1 md:grid-cols-2 gap-3">
                          <div>
                            <label className="label">
                              <span className="label-text font-medium">Label</span>
                            </label>
                            <input
                              className="input input-bordered w-full"
                              value={f.label}
                              onChange={(e) => updateField(idx, { label: e.target.value })}
                            />
                          </div>
                          <div>
                            <label className="label">
                              <span className="label-text font-medium">Type</span>
                            </label>
                            <select
                              className="select select-bordered w-full"
                              value={f.type}
                              onChange={(e) => updateField(idx, { type: e.target.value as FieldType })}
                            >
                              <option value="text">Texte</option>
                              <option value="textarea">Texte long</option>
                              <option value="email">Email</option>
                              <option value="tel">Téléphone</option>
                              <option value="select">Select</option>
                              <option value="checkbox">Checkbox</option>
                            </select>
                          </div>

                          <div>
                            <label className="label">
                              <span className="label-text font-medium">Binding</span>
                            </label>
                            <input
                              className="input input-bordered w-full font-mono"
                              value={f.binding}
                              onChange={(e) => updateField(idx, { binding: e.target.value as Binding })}
                              placeholder="title | description | custom:xxx"
                            />
                            <p className="text-xs text-base-content/60 mt-1">
                              Ex: `title`, `contactEmail`, `custom:budget`, etc.
                            </p>
                          </div>

                          <div>
                            <label className="label">
                              <span className="label-text font-medium">Placeholder</span>
                            </label>
                            <input
                              className="input input-bordered w-full"
                              value={f.placeholder ?? ''}
                              onChange={(e) => updateField(idx, { placeholder: e.target.value })}
                            />
                          </div>

                          <div className="md:col-span-2 flex items-center gap-6">
                            <label className="label cursor-pointer justify-start gap-3">
                              <input
                                type="checkbox"
                                className="checkbox checkbox-primary"
                                checked={Boolean(f.required)}
                                onChange={(e) => updateField(idx, { required: e.target.checked })}
                              />
                              <span className="label-text">Requis</span>
                            </label>
                            <label className="label cursor-pointer justify-start gap-3">
                              <input
                                type="checkbox"
                                className="checkbox checkbox-primary"
                                checked={Boolean(f.hideWhenAuthenticated)}
                                onChange={(e) => updateField(idx, { hideWhenAuthenticated: e.target.checked })}
                              />
                              <span className="label-text">Masquer si connecté</span>
                            </label>
                          </div>

                          {f.type === 'select' && (
                            <div className="md:col-span-2">
                              <label className="label">
                                <span className="label-text font-medium">Options (1 par ligne: label=value)</span>
                              </label>
                              <textarea
                                className="textarea textarea-bordered w-full min-h-24 font-mono text-sm"
                                value={(f.options ?? [])
                                  .map((o) => `${o.label}=${o.value}`)
                                  .join('\n')}
                                onChange={(e) => {
                                  const opts = e.target.value
                                    .split('\n')
                                    .map((line) => line.trim())
                                    .filter(Boolean)
                                    .map((line) => {
                                      const [label, value] = line.split('=')
                                      return { label: (label ?? '').trim(), value: (value ?? '').trim() }
                                    })
                                    .filter((o) => o.label && o.value)
                                  updateField(idx, { options: opts })
                                }}
                              />
                            </div>
                          )}
                        </div>

                        <div className="flex flex-col gap-2">
                          <button className="btn btn-ghost btn-sm" onClick={() => moveField(idx, -1)} disabled={idx === 0}>
                            <span className="icon-[tabler--arrow-up] size-4" />
                          </button>
                          <button
                            className="btn btn-ghost btn-sm"
                            onClick={() => moveField(idx, 1)}
                            disabled={idx === schema.fields.length - 1}
                          >
                            <span className="icon-[tabler--arrow-down] size-4" />
                          </button>
                          <button className="btn btn-ghost btn-sm text-error" onClick={() => deleteField(idx)}>
                            <span className="icon-[tabler--trash] size-4" />
                          </button>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </CardBody>
        </Card>
      </div>
    </AdminLayout>
  )
}

