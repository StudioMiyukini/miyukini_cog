'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables } from '@/lib/supabase/database.types'
import { useAuth } from '@/contexts/AuthContext'

type AppBranding = Tables<'app_branding'>

const BRANDING_BUCKET = 'app-branding'

export function SuperAdminBrandingScreen() {
  const supabase = useMemo(() => getSupabaseClient(), [])
  const { user } = useAuth()

  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [success, setSuccess] = useState<string | null>(null)

  const [branding, setBranding] = useState<AppBranding | null>(null)
  const [appTitle, setAppTitle] = useState('')
  const [logoFile, setLogoFile] = useState<File | null>(null)
  const [logoUrl, setLogoUrl] = useState('')

  const getLogoUrl = useCallback(
    (b: AppBranding | null) => {
      if (!b) return null
      if (b.logo_url) return b.logo_url
      if (b.logo_path) return supabase.storage.from(BRANDING_BUCKET).getPublicUrl(b.logo_path).data.publicUrl
      return null
    },
    [supabase]
  )

  const load = useCallback(async () => {
    setLoading(true)
    setError(null)
    setSuccess(null)
    try {
      const { data, error: e } = await supabase.from('app_branding').select('*').eq('id', 'main').single()
      if (e) throw e
      const row = (data ?? null) as AppBranding | null
      setBranding(row)
      setAppTitle(row?.app_title ?? 'Miyukini')
      setLogoUrl(getLogoUrl(row) ?? '')
    } catch (err: any) {
      setError(err?.message ?? 'Impossible de charger le branding.')
    } finally {
      setLoading(false)
    }
  }, [getLogoUrl, supabase])

  useEffect(() => {
    void load()
  }, [load])

  const save = async () => {
    if (!user) return
    setSaving(true)
    setError(null)
    setSuccess(null)
    try {
      let finalLogoPath = branding?.logo_path ?? null
      let finalLogoUrl = branding?.logo_url ?? null

      // Upload nouveau logo si fichier sélectionné
      if (logoFile) {
        const allowed = ['image/png', 'image/jpeg', 'image/svg+xml', 'image/webp']
        if (!allowed.includes(logoFile.type)) {
          setError('Format non supporté. Formats acceptés : PNG, JPG, SVG, WEBP.')
          return
        }
        if (logoFile.size > 2 * 1024 * 1024) {
          setError('Fichier trop lourd (max 2 Mo).')
          return
        }

        const ext = logoFile.name.split('.').pop()?.toLowerCase() || 'png'
        const objectPath = `logo.${ext}`

        // Supprimer l'ancien logo si existe
        if (branding?.logo_path) {
          await supabase.storage.from(BRANDING_BUCKET).remove([branding.logo_path])
        }

        const { error: upErr } = await supabase.storage.from(BRANDING_BUCKET).upload(objectPath, logoFile, {
          contentType: logoFile.type,
          cacheControl: '3600',
          upsert: true,
        })
        if (upErr) throw upErr

        finalLogoPath = objectPath
        finalLogoUrl = supabase.storage.from(BRANDING_BUCKET).getPublicUrl(objectPath).data.publicUrl
      }

      const payload: any = {
        app_title: appTitle.trim() || 'Miyukini',
        logo_path: finalLogoPath,
        logo_url: finalLogoUrl,
        updated_by: user.id,
      }

      const { data, error: e } = await supabase
        .from('app_branding')
        .upsert({ id: 'main', ...payload })
        .select('*')
        .single()
      if (e) throw e
      setBranding((data ?? null) as any)
      setLogoUrl(getLogoUrl(data as any) ?? '')
      setLogoFile(null)
      setSuccess('Branding mis à jour. Rafraîchis la page pour voir les changements.')
    } catch (err: any) {
      setError(err?.message ?? 'Impossible d’enregistrer.')
    } finally {
      setSaving(false)
    }
  }

  return (
    <AdminLayout title="Branding">
      <div>
        <h1 className="text-2xl font-bold text-base-content">Branding</h1>
        <p className="text-base-content/60 mt-1">Titre et logo de l’application (affichés dans le header).</p>
      </div>

      {(error || success) && (
        <div className="mt-4 space-y-2">
          {error && (
            <div className="alert alert-error">
              <span className="icon-[tabler--alert-circle] size-5" />
              <span>{error}</span>
            </div>
          )}
          {success && (
            <div className="alert alert-success">
              <span className="icon-[tabler--check] size-5" />
              <span>{success}</span>
            </div>
          )}
        </div>
      )}

      <Card>
        <CardHeader className="flex items-center justify-between">
          <span className="font-semibold text-base-content">Configuration</span>
          {loading ? <span className="loading loading-spinner loading-sm text-primary" /> : null}
        </CardHeader>
        <CardBody className="space-y-4">
          <div>
            <label className="label">
              <span className="label-text">Titre de l’application</span>
            </label>
            <input
              className="input input-bordered w-full"
              value={appTitle}
              onChange={(e) => setAppTitle(e.target.value)}
              placeholder="Miyukini"
            />
            <div className="mt-1 text-xs text-base-content/60">Affiché dans le header à côté du logo.</div>
          </div>

          <div>
            <label className="label">
              <span className="label-text">Logo</span>
            </label>
            <div className="space-y-3">
              {logoUrl && (
                <div className="rounded-xl border border-base-300 bg-base-100 p-4 flex items-center gap-4">
                  <div className="size-16 rounded-lg bg-base-200 overflow-hidden border border-base-300 shrink-0">
                    {/* eslint-disable-next-line @next/next/no-img-element */}
                    <img src={logoUrl} alt="Logo actuel" className="h-full w-full object-contain" />
                  </div>
                  <div className="min-w-0 flex-1">
                    <div className="text-sm font-semibold text-base-content">Logo actuel</div>
                    <div className="text-xs text-base-content/60 truncate">{branding?.logo_path ?? branding?.logo_url ?? '—'}</div>
                  </div>
                </div>
              )}

              <div>
                <input
                  type="file"
                  accept="image/png,image/jpeg,image/svg+xml,image/webp"
                  className="file-input file-input-bordered w-full"
                  onChange={(e) => setLogoFile(e.target.files?.[0] ?? null)}
                />
                <div className="mt-1 text-xs text-base-content/50">Formats : PNG/JPG/SVG/WEBP • max 2 Mo</div>
              </div>
            </div>
          </div>

          <div className="flex flex-col sm:flex-row gap-2 justify-end pt-2">
            <button className="btn btn-ghost" onClick={() => void load()} disabled={saving || loading}>
              Annuler
            </button>
            <button className="btn btn-primary" onClick={() => void save()} disabled={saving || loading || !appTitle.trim()}>
              {saving ? 'Enregistrement…' : 'Enregistrer'}
            </button>
          </div>
        </CardBody>
      </Card>
    </AdminLayout>
  )
}
