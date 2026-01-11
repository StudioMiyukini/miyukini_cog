'use client'

import { useEffect, useMemo, useState } from 'react'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { getSupabaseClient } from '@/lib/supabase/client'
import { useAuth } from '@/contexts/AuthContext'
import { commerceDevisModuleManifest } from '@/modules/commerce-devis/manifest'

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
  options?: { label: string; value: string }[]
  hideWhenAuthenticated?: boolean
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
  description: 'Décris ton besoin. Un humain te répondra avec un devis (pas de paiement en ligne).',
  fields: [
    { id: 'title', label: 'Titre', type: 'text', binding: 'title', required: true, placeholder: 'Ex: Création d’un site vitrine' },
    { id: 'description', label: 'Description', type: 'textarea', binding: 'description', placeholder: 'Contexte, contraintes, délais, budget approximatif…' },
    { id: 'category', label: 'Catégorie', type: 'text', binding: 'category', placeholder: 'Ex: Site web, Branding, Booking…' },
    { id: 'tags', label: 'Tags (séparés par virgule)', type: 'text', binding: 'tags', placeholder: 'Ex: urgent, festival, v1' },
    {
      id: 'contactEmail',
      label: 'Email',
      type: 'email',
      binding: 'contactEmail',
      required: true,
      placeholder: 'nom@exemple.fr',
      hideWhenAuthenticated: true,
    },
    { id: 'contactPhone', label: 'Téléphone (optionnel)', type: 'tel', binding: 'contactPhone', placeholder: '06…' },
    {
      id: 'preferredChannel',
      label: 'Canal préféré',
      type: 'select',
      binding: 'preferredChannel',
      options: [
        { label: 'Email', value: 'email' },
        { label: 'Téléphone', value: 'phone' },
      ],
    },
  ],
}

function bindingKey(binding: Binding) {
  return binding
}

export function QuoteRequestScreen() {
  const supabase = useMemo(() => getSupabaseClient(), [])
  const { user, isAuthenticated } = useAuth()

  const [loading, setLoading] = useState(false)
  const [doneId, setDoneId] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  const [schema, setSchema] = useState<FormSchema>(defaultSchema)
  const [values, setValues] = useState<Record<string, string | boolean>>(() => {
    const initial: Record<string, string | boolean> = {}
    for (const f of defaultSchema.fields) {
      initial[bindingKey(f.binding)] = f.type === 'checkbox' ? false : ''
    }
    initial.preferredChannel = 'email'
    return initial
  })

  useEffect(() => {
    const loadSchema = async () => {
      const { data, error: loadError } = await (supabase as any)
        .from('module_form_configs')
        .select('schema_json,is_active')
        .eq('workspace_id', GLOBAL_WORKSPACE_ID)
        .eq('module_id', commerceDevisModuleManifest.id)
        .eq('form_key', FORM_KEY)
        .maybeSingle()
      if (loadError) return
      if (!data?.is_active) return

      const s = data?.schema_json as FormSchema | null
      if (!s || s.version !== 1 || !Array.isArray(s.fields)) return

      setSchema(s)
      setValues((prev) => {
        const next: Record<string, string | boolean> = { ...prev }
        for (const f of s.fields) {
          const k = bindingKey(f.binding)
          if (!(k in next)) next[k] = f.type === 'checkbox' ? false : ''
        }
        if (!('preferredChannel' in next)) next.preferredChannel = 'email'
        return next
      })
    }
    void loadSchema()
  }, [supabase])

  const setValue = (binding: Binding, v: string | boolean) => {
    setValues((s) => ({ ...s, [bindingKey(binding)]: v }))
  }

  const submit = async () => {
    setLoading(true)
    setError(null)
    setDoneId(null)
    try {
      const getStr = (b: Binding) => String(values[bindingKey(b)] ?? '')

      const title = getStr('title').trim()
      if (!title) throw new Error('Le titre est requis.')

      const preferredChannel = (getStr('preferredChannel') || 'email') as 'email' | 'phone'

      const tagsRaw = getStr('tags')
      const tags = tagsRaw
        .split(',')
        .map((t) => t.trim())
        .filter(Boolean)

      const contactEmail = isAuthenticated ? user?.email ?? '' : getStr('contactEmail')
      const contactEmailRequired = schema.fields.some((f) => f.binding === 'contactEmail' && f.required)
      if (!isAuthenticated && contactEmailRequired && !contactEmail.trim()) throw new Error("L'email est requis.")

      const payload: any = {
        title,
        description: getStr('description').trim() || null,
        tags: tags.length ? tags : null,
        category: getStr('category').trim() || null,
        preferred_contact_channel: preferredChannel,
        contact_email: contactEmail.trim() || null,
        contact_phone: getStr('contactPhone').trim() || null,
        requested_by: isAuthenticated ? user?.id : null,
        status: 'submitted',
        meta: {
          source: 'web_form',
          custom_fields: Object.fromEntries(
            schema.fields
              .filter((f) => f.binding.startsWith('custom:'))
              .map((f) => [f.binding.slice('custom:'.length), values[bindingKey(f.binding)] ?? null]),
          ),
        },
      }

      const { data, error: insertError } = await supabase
        .from('commerce_quote_requests')
        .insert(payload)
        .select('id')
        .single()
      if (insertError) throw insertError
      setDoneId(data.id)
      setValues(() => {
        const next: Record<string, string | boolean> = {}
        for (const f of schema.fields) {
          next[bindingKey(f.binding)] = f.type === 'checkbox' ? false : ''
        }
        next.preferredChannel = 'email'
        return next
      })
    } catch (e: any) {
      setError(e?.message ?? 'Erreur lors de l’envoi.')
    } finally {
      setLoading(false)
    }
  }

  return (
    <AppShellScreen>
      <main className="min-h-screen bg-base-100 px-4 sm:px-6 lg:px-8 py-8">
        <div className="max-w-2xl mx-auto">
          <div className="flex items-start justify-between gap-4">
            <div>
              <h1 className="text-3xl font-bold text-base-content">{schema.title ?? 'Demander un devis'}</h1>
              <p className="text-base-content/60 mt-1">
                {schema.description ?? 'Décris ton besoin. Un humain te répondra avec un devis (pas de paiement en ligne).'}
              </p>
            </div>
            <Badge variant="neutral" style="soft">
              Commerce
            </Badge>
          </div>

          <Card className="mt-6">
            <CardHeader className="flex items-center justify-between">
              <span className="font-semibold">Formulaire</span>
              {loading && <span className="loading loading-spinner loading-sm text-primary" />}
            </CardHeader>
            <CardBody className="space-y-4">
              {doneId ? (
                <div className="alert alert-success">
                  <span className="icon-[tabler--circle-check] size-5" />
                  <div>
                    <div className="font-semibold">Demande envoyée</div>
                    <div className="text-sm opacity-80">Référence: {doneId}</div>
                  </div>
                </div>
              ) : null}
              {error ? (
                <div className="alert alert-error">
                  <span className="icon-[tabler--alert-triangle] size-5" />
                  <span>{error}</span>
                </div>
              ) : null}

              {schema.fields
                .filter((f) => !(isAuthenticated && f.hideWhenAuthenticated))
                .map((f) => {
                  const v = values[bindingKey(f.binding)]
                  const requiredMark = f.required ? <span className="text-error"> *</span> : null

                  if (f.type === 'textarea') {
                    return (
                      <div key={f.id}>
                        <label className="label">
                          <span className="label-text">
                            {f.label}
                            {requiredMark}
                          </span>
                        </label>
                        <textarea
                          className="textarea textarea-bordered w-full min-h-[140px]"
                          value={String(v ?? '')}
                          onChange={(e) => setValue(f.binding, e.target.value)}
                          placeholder={f.placeholder ?? ''}
                        />
                        {f.help ? <p className="mt-1 text-xs text-base-content/60">{f.help}</p> : null}
                      </div>
                    )
                  }

                  if (f.type === 'select') {
                    return (
                      <div key={f.id}>
                        <label className="label">
                          <span className="label-text">
                            {f.label}
                            {requiredMark}
                          </span>
                        </label>
                        <select
                          className="select select-bordered w-full"
                          value={String(v ?? '')}
                          onChange={(e) => setValue(f.binding, e.target.value)}
                        >
                          {(f.options ?? []).map((o) => (
                            <option key={o.value} value={o.value}>
                              {o.label}
                            </option>
                          ))}
                        </select>
                        {f.help ? <p className="mt-1 text-xs text-base-content/60">{f.help}</p> : null}
                      </div>
                    )
                  }

                  if (f.type === 'checkbox') {
                    return (
                      <div key={f.id}>
                        <label className="label cursor-pointer justify-start gap-3">
                          <input
                            type="checkbox"
                            className="checkbox checkbox-primary"
                            checked={Boolean(v)}
                            onChange={(e) => setValue(f.binding, e.target.checked)}
                          />
                          <span className="label-text">
                            {f.label}
                            {requiredMark}
                          </span>
                        </label>
                        {f.help ? <p className="mt-1 text-xs text-base-content/60">{f.help}</p> : null}
                      </div>
                    )
                  }

                  const inputType = f.type === 'text' ? 'text' : f.type
                  return (
                    <div key={f.id}>
                      <label className="label">
                        <span className="label-text">
                          {f.label}
                          {requiredMark}
                        </span>
                      </label>
                      <input
                        className="input input-bordered w-full"
                        type={inputType}
                        value={String(v ?? '')}
                        onChange={(e) => setValue(f.binding, e.target.value)}
                        placeholder={f.placeholder ?? ''}
                      />
                      {f.help ? <p className="mt-1 text-xs text-base-content/60">{f.help}</p> : null}
                    </div>
                  )
                })}

              {isAuthenticated ? (
                <div className="text-sm text-base-content/60">
                  Connecté en tant que <span className="font-medium">{user?.email}</span> — l’email de contact sera celui-ci.
                </div>
              ) : null}

              <div className="pt-2 flex justify-end">
                <button className="btn btn-primary" onClick={() => void submit()} disabled={loading || !String(values.title ?? '').trim()}>
                  Envoyer ma demande
                  <span className="icon-[tabler--arrow-right] size-5" />
                </button>
              </div>
            </CardBody>
          </Card>
        </div>
      </main>
    </AppShellScreen>
  )
}

