'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables } from '@/lib/supabase/database.types'
import { useRequireEnabledModule } from '../hooks/useRequireEnabledModule'
import { useBookingProvider } from '../hooks/useBookingProvider'

type BookingProvider = Tables<'booking_providers'>
type BookingProviderPhoto = Tables<'booking_provider_photos'>

type DayKey = 'mon' | 'tue' | 'wed' | 'thu' | 'fri' | 'sat' | 'sun'

const PHOTOS_BUCKET = 'booking-provider-photos'

const DAYS: Array<{ key: DayKey; label: string }> = [
  { key: 'mon', label: 'Lundi' },
  { key: 'tue', label: 'Mardi' },
  { key: 'wed', label: 'Mercredi' },
  { key: 'thu', label: 'Jeudi' },
  { key: 'fri', label: 'Vendredi' },
  { key: 'sat', label: 'Samedi' },
  { key: 'sun', label: 'Dimanche' },
]

function safeString(v: any) {
  return typeof v === 'string' ? v : ''
}

function getSocial(socialLinks: any, key: string) {
  const v = socialLinks?.[key]
  return typeof v === 'string' ? v : ''
}

function setSocial(socialLinks: any, key: string, value: string) {
  const next = { ...(socialLinks ?? {}) }
  if (!value.trim()) delete next[key]
  else next[key] = value.trim()
  return next
}

function readOpening(openingHours: any): Record<DayKey, { closed: boolean; open: string; close: string }> {
  const map = openingHours ?? {}
  const read = (k: DayKey) => {
    const v = map?.[k] ?? null
    if (v?.closed === true) return { closed: true, open: '09:00', close: '19:00' }
    return {
      closed: false,
      open: safeString(v?.open) || '09:00',
      close: safeString(v?.close) || '19:00',
    }
  }
  return {
    mon: read('mon'),
    tue: read('tue'),
    wed: read('wed'),
    thu: read('thu'),
    fri: read('fri'),
    sat: read('sat'),
    sun: read('sun'),
  }
}

function toOpeningHoursJson(state: Record<DayKey, { closed: boolean; open: string; close: string }>) {
  const out: any = {}
  for (const k of Object.keys(state) as DayKey[]) {
    const d = state[k]
    out[k] = d.closed ? { closed: true } : { open: d.open, close: d.close }
  }
  return out
}

export function ProviderProfileScreen() {
  const { isLoading: modulesLoading, enabled } = useRequireEnabledModule('booking', '/')
  const supabase = useMemo(() => getSupabaseClient(), [])
  const { isAuthenticated, mustDenyProviderBackOffice, ensureProvider } = useBookingProvider()

  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [success, setSuccess] = useState<string | null>(null)

  const [provider, setProvider] = useState<BookingProvider | null>(null)
  const [photos, setPhotos] = useState<BookingProviderPhoto[]>([])

  // form
  const [displayName, setDisplayName] = useState('')
  const [description, setDescription] = useState('')
  const [contactEmail, setContactEmail] = useState('')
  const [phone, setPhone] = useState('')
  const [website, setWebsite] = useState('')

  const [address1, setAddress1] = useState('')
  const [address2, setAddress2] = useState('')
  const [postalCode, setPostalCode] = useState('')
  const [city, setCity] = useState('')
  const [country, setCountry] = useState('FR')

  const [socialLinks, setSocialLinks] = useState<any>({})
  const [opening, setOpening] = useState<Record<DayKey, { closed: boolean; open: string; close: string }>>(readOpening({}))

  const [newPhotoAlt, setNewPhotoAlt] = useState('')
  const [newPhotoFile, setNewPhotoFile] = useState<File | null>(null)

  const getPhotoUrl = useCallback(
    (p: BookingProviderPhoto) => {
      if (p.url) return p.url
      if (p.path) {
        return supabase.storage.from(PHOTOS_BUCKET).getPublicUrl(p.path).data.publicUrl
      }
      return null
    },
    [supabase]
  )

  const load = useCallback(async () => {
    if (!enabled || !isAuthenticated || mustDenyProviderBackOffice) return
    setLoading(true)
    setError(null)
    setSuccess(null)
    try {
      const prov = await ensureProvider()
      if (!prov) return
      const { data, error: e } = await supabase.from('booking_providers').select('*').eq('id', prov.id).single()
      if (e) throw e
      const row = (data ?? null) as BookingProvider | null
      setProvider(row)

      setDisplayName(row?.display_name ?? '')
      setDescription((row as any)?.description ?? '')
      setContactEmail((row as any)?.contact_email ?? '')
      setPhone((row as any)?.phone ?? '')
      setWebsite((row as any)?.website ?? '')
      setAddress1((row as any)?.address_line1 ?? '')
      setAddress2((row as any)?.address_line2 ?? '')
      setPostalCode((row as any)?.postal_code ?? '')
      setCity((row as any)?.city ?? '')
      setCountry((row as any)?.country ?? 'FR')
      setSocialLinks((row as any)?.social_links ?? {})
      setOpening(readOpening((row as any)?.opening_hours))

      const { data: photosData } = await supabase
        .from('booking_provider_photos')
        .select('*')
        .eq('provider_id', prov.id)
        .order('sort_order', { ascending: true })
      setPhotos((photosData ?? []) as BookingProviderPhoto[])
    } catch (err: any) {
      setError(err?.message ?? 'Impossible de charger le profil prestataire.')
    } finally {
      setLoading(false)
    }
  }, [enabled, ensureProvider, isAuthenticated, mustDenyProviderBackOffice, supabase])

  useEffect(() => {
    if (modulesLoading) return
    void load()
  }, [load, modulesLoading])

  const save = async () => {
    if (!provider) return
    setSaving(true)
    setError(null)
    setSuccess(null)
    try {
      const payload: any = {
        display_name: displayName.trim() || null,
        description: description.trim() || null,
        contact_email: contactEmail.trim() || null,
        phone: phone.trim() || null,
        website: website.trim() || null,
        address_line1: address1.trim() || null,
        address_line2: address2.trim() || null,
        postal_code: postalCode.trim() || null,
        city: city.trim() || null,
        country: country.trim() || 'FR',
        social_links: socialLinks ?? {},
        opening_hours: toOpeningHoursJson(opening),
      }
      const { data, error: e } = await supabase.from('booking_providers').update(payload).eq('id', provider.id).select('*').single()
      if (e) throw e
      setProvider((data ?? null) as any)
      setSuccess('Profil mis à jour.')
    } catch (err: any) {
      setError(err?.message ?? 'Impossible d’enregistrer.')
    } finally {
      setSaving(false)
    }
  }

  const addPhoto = async () => {
    if (!provider) return
    const file = newPhotoFile
    if (!file) return

    const allowed = ['image/jpeg', 'image/png', 'image/webp']
    if (!allowed.includes(file.type)) {
      setError('Format non supporté. Formats acceptés : JPG, PNG, WEBP.')
      return
    }
    if (file.size > 10 * 1024 * 1024) {
      setError('Fichier trop lourd (max 10 Mo).')
      return
    }
    setSaving(true)
    setError(null)
    setSuccess(null)
    try {
      const maxSort = photos.reduce((m, p) => Math.max(m, p.sort_order ?? 0), 0)

      const ext = (file.name.split('.').pop() || 'jpg').toLowerCase()
      const objectPath = `${provider.id}/${crypto.randomUUID()}.${ext}`
      const { error: upErr } = await supabase.storage.from(PHOTOS_BUCKET).upload(objectPath, file, {
        contentType: file.type,
        cacheControl: '3600',
        upsert: false,
      })
      if (upErr) throw upErr

      const publicUrl = supabase.storage.from(PHOTOS_BUCKET).getPublicUrl(objectPath).data.publicUrl
      const { data, error: e } = await supabase
        .from('booking_provider_photos')
        .insert({
          provider_id: provider.id,
          path: objectPath,
          url: publicUrl,
          alt: newPhotoAlt.trim() || null,
          sort_order: maxSort + 1,
          is_public: true,
        })
        .select('*')
        .single()
      if (e) throw e
      setPhotos((prev) => [...prev, data as any])
      setNewPhotoAlt('')
      setNewPhotoFile(null)
      setSuccess('Photo ajoutée.')
    } catch (err: any) {
      setError(err?.message ?? 'Impossible d’ajouter la photo.')
    } finally {
      setSaving(false)
    }
  }

  const deletePhoto = async (photoId: string) => {
    if (!provider) return
    setSaving(true)
    setError(null)
    setSuccess(null)
    try {
      const photo = photos.find((p) => p.id === photoId) ?? null
      if (photo?.path) {
        // Best-effort: si déjà supprimée, on continue
        await supabase.storage.from(PHOTOS_BUCKET).remove([photo.path])
      }
      const { error: e } = await supabase.from('booking_provider_photos').delete().eq('id', photoId).eq('provider_id', provider.id)
      if (e) throw e
      setPhotos((prev) => prev.filter((p) => p.id !== photoId))
      setSuccess('Photo supprimée.')
    } catch (err: any) {
      setError(err?.message ?? 'Impossible de supprimer la photo.')
    } finally {
      setSaving(false)
    }
  }

  const setPhotoPublic = async (photoId: string, isPublic: boolean) => {
    if (!provider) return
    setSaving(true)
    setError(null)
    setSuccess(null)
    try {
      const { data, error: e } = await supabase
        .from('booking_provider_photos')
        .update({ is_public: isPublic })
        .eq('id', photoId)
        .eq('provider_id', provider.id)
        .select('*')
      if (e) throw e
      const updated = (data ?? []) as BookingProviderPhoto[]
      setPhotos((prev) =>
        prev.map((p) => updated.find((u) => u.id === p.id) ?? p).sort((a, b) => (a.sort_order ?? 0) - (b.sort_order ?? 0))
      )
      setSuccess(isPublic ? 'Photo publiée.' : 'Photo masquée.')
    } catch (err: any) {
      setError(err?.message ?? 'Impossible de modifier la visibilité.')
    } finally {
      setSaving(false)
    }
  }

  const movePhoto = async (photoId: string, direction: 'up' | 'down') => {
    if (!provider) return
    const sorted = [...photos].sort((a, b) => (a.sort_order ?? 0) - (b.sort_order ?? 0))
    const idx = sorted.findIndex((p) => p.id === photoId)
    const swapWith = direction === 'up' ? idx - 1 : idx + 1
    if (idx < 0 || swapWith < 0 || swapWith >= sorted.length) return

    const a = sorted[idx]
    const b = sorted[swapWith]
    setSaving(true)
    setError(null)
    setSuccess(null)
    try {
      // swap sort_order
      const { error: e1 } = await supabase
        .from('booking_provider_photos')
        .update({ sort_order: b.sort_order })
        .eq('id', a.id)
        .eq('provider_id', provider.id)
      if (e1) throw e1
      const { error: e2 } = await supabase
        .from('booking_provider_photos')
        .update({ sort_order: a.sort_order })
        .eq('id', b.id)
        .eq('provider_id', provider.id)
      if (e2) throw e2

      setPhotos((prev) =>
        prev
          .map((p) => {
            if (p.id === a.id) return { ...p, sort_order: b.sort_order }
            if (p.id === b.id) return { ...p, sort_order: a.sort_order }
            return p
          })
          .sort((x, y) => (x.sort_order ?? 0) - (y.sort_order ?? 0))
      )
      setSuccess('Ordre mis à jour.')
    } catch (err: any) {
      setError(err?.message ?? 'Impossible de réordonner.')
    } finally {
      setSaving(false)
    }
  }

  if (modulesLoading) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }
  if (!enabled) return null

  if (!isAuthenticated) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <div className="text-base-content/70">Connecte-toi pour accéder à ton profil prestataire.</div>
      </div>
    )
  }

  if (mustDenyProviderBackOffice) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <div className="text-base-content/70">Cet espace est réservé aux prestataires.</div>
      </div>
    )
  }

  return (
    <AppShellScreen>
      <ContentStack>
        <div className="flex items-start justify-between gap-3">
          <div className="space-y-1 min-w-0">
            <div className="text-sm text-base-content/60">
              <Link className="link" href="/pro/booking">
                Back-office prestataire
              </Link>{' '}
              <span className="text-base-content/40">/</span> Profil pro
            </div>
            <h1 className="text-2xl font-bold text-base-content">Profil pro</h1>
            <p className="text-base-content/60 text-sm">
              Ces informations alimentent ta fiche publique <span className="font-semibold">/booking</span>.
            </p>
          </div>
          {provider?.id && (
            <Link href={`/booking/${provider.id}`} className="btn btn-ghost btn-sm shrink-0">
              <span className="icon-[tabler--external-link] size-4" />
              Voir la fiche publique
            </Link>
          )}
        </div>

        {(error || success) && (
          <div className="space-y-2">
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
            <span className="font-semibold text-base-content">Identité & présentation</span>
            {loading ? <span className="loading loading-spinner loading-sm text-primary" /> : null}
          </CardHeader>
          <CardBody className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              <div>
                <label className="label">
                  <span className="label-text">Nom d’enseigne</span>
                </label>
                <input className="input input-bordered w-full" value={displayName} onChange={(e) => setDisplayName(e.target.value)} />
              </div>
              <div className="flex items-end">
                <div className="text-sm text-base-content/60">
                  Astuce : utilise le même nom que sur tes réseaux.
                </div>
              </div>
            </div>

            <div>
              <label className="label">
                <span className="label-text">Description</span>
              </label>
              <textarea className="textarea textarea-bordered w-full min-h-28" value={description} onChange={(e) => setDescription(e.target.value)} />
            </div>
          </CardBody>
        </Card>

        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Adresse</span>
            <Badge variant="neutral" style="soft" size="sm">
              Public
            </Badge>
          </CardHeader>
          <CardBody className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              <div>
                <label className="label">
                  <span className="label-text">Adresse (ligne 1)</span>
                </label>
                <input className="input input-bordered w-full" value={address1} onChange={(e) => setAddress1(e.target.value)} />
              </div>
              <div>
                <label className="label">
                  <span className="label-text">Adresse (ligne 2)</span>
                </label>
                <input className="input input-bordered w-full" value={address2} onChange={(e) => setAddress2(e.target.value)} />
              </div>
              <div>
                <label className="label">
                  <span className="label-text">Code postal</span>
                </label>
                <input className="input input-bordered w-full" value={postalCode} onChange={(e) => setPostalCode(e.target.value)} />
              </div>
              <div>
                <label className="label">
                  <span className="label-text">Ville</span>
                </label>
                <input className="input input-bordered w-full" value={city} onChange={(e) => setCity(e.target.value)} />
              </div>
              <div className="md:col-span-2">
                <label className="label">
                  <span className="label-text">Pays</span>
                </label>
                <input className="input input-bordered w-full" value={country} onChange={(e) => setCountry(e.target.value)} />
              </div>
            </div>
          </CardBody>
        </Card>

        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Contact</span>
            <Badge variant="neutral" style="soft" size="sm">
              Public
            </Badge>
          </CardHeader>
          <CardBody className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
              <div>
                <label className="label">
                  <span className="label-text">Email de contact</span>
                </label>
                <input className="input input-bordered w-full" value={contactEmail} onChange={(e) => setContactEmail(e.target.value)} />
              </div>
              <div>
                <label className="label">
                  <span className="label-text">Téléphone</span>
                </label>
                <input className="input input-bordered w-full" value={phone} onChange={(e) => setPhone(e.target.value)} />
              </div>
              <div>
                <label className="label">
                  <span className="label-text">Site web</span>
                </label>
                <input className="input input-bordered w-full" value={website} onChange={(e) => setWebsite(e.target.value)} placeholder="https://…" />
              </div>
            </div>
          </CardBody>
        </Card>

        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Réseaux sociaux</span>
            <Badge variant="neutral" style="soft" size="sm">
              Public
            </Badge>
          </CardHeader>
          <CardBody className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              <div>
                <label className="label">
                  <span className="label-text">Instagram</span>
                </label>
                <input
                  className="input input-bordered w-full"
                  value={getSocial(socialLinks, 'instagram')}
                  onChange={(e) => setSocialLinks((prev: any) => setSocial(prev, 'instagram', e.target.value))}
                  placeholder="https://instagram.com/…"
                />
              </div>
              <div>
                <label className="label">
                  <span className="label-text">Facebook</span>
                </label>
                <input
                  className="input input-bordered w-full"
                  value={getSocial(socialLinks, 'facebook')}
                  onChange={(e) => setSocialLinks((prev: any) => setSocial(prev, 'facebook', e.target.value))}
                  placeholder="https://facebook.com/…"
                />
              </div>
              <div>
                <label className="label">
                  <span className="label-text">TikTok</span>
                </label>
                <input
                  className="input input-bordered w-full"
                  value={getSocial(socialLinks, 'tiktok')}
                  onChange={(e) => setSocialLinks((prev: any) => setSocial(prev, 'tiktok', e.target.value))}
                  placeholder="https://tiktok.com/@…"
                />
              </div>
              <div>
                <label className="label">
                  <span className="label-text">Google (fiche / maps)</span>
                </label>
                <input
                  className="input input-bordered w-full"
                  value={getSocial(socialLinks, 'google')}
                  onChange={(e) => setSocialLinks((prev: any) => setSocial(prev, 'google', e.target.value))}
                  placeholder="https://…"
                />
              </div>
            </div>
          </CardBody>
        </Card>

        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Horaires</span>
            <Badge variant="neutral" style="soft" size="sm">
              Public
            </Badge>
          </CardHeader>
          <CardBody className="space-y-3">
            <div className="text-sm text-base-content/60">
              Format utilisé sur la fiche publique : <span className="font-semibold">Europe/Paris</span>.
            </div>
            <div className="space-y-2">
              {DAYS.map((d) => {
                const v = opening[d.key]
                return (
                  <div key={d.key} className="grid grid-cols-1 md:grid-cols-[160px_1fr_1fr_140px] gap-2 items-center rounded-xl border border-base-300 bg-base-100 p-3">
                    <div className="font-medium text-base-content">{d.label}</div>
                    <div>
                      <label className="label py-0">
                        <span className="label-text text-xs">Ouverture</span>
                      </label>
                      <input
                        type="time"
                        className="input input-bordered w-full"
                        value={v.open}
                        disabled={v.closed}
                        onChange={(e) => setOpening((prev) => ({ ...prev, [d.key]: { ...prev[d.key], open: e.target.value } }))}
                      />
                    </div>
                    <div>
                      <label className="label py-0">
                        <span className="label-text text-xs">Fermeture</span>
                      </label>
                      <input
                        type="time"
                        className="input input-bordered w-full"
                        value={v.close}
                        disabled={v.closed}
                        onChange={(e) => setOpening((prev) => ({ ...prev, [d.key]: { ...prev[d.key], close: e.target.value } }))}
                      />
                    </div>
                    <div className="flex items-center gap-2 justify-start md:justify-end">
                      <input
                        type="checkbox"
                        className="toggle toggle-primary"
                        checked={v.closed}
                        onChange={(e) => setOpening((prev) => ({ ...prev, [d.key]: { ...prev[d.key], closed: e.target.checked } }))}
                      />
                      <span className="text-sm text-base-content/70">Fermé</span>
                    </div>
                  </div>
                )
              })}
            </div>
          </CardBody>
        </Card>

        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Photos</span>
            <Badge variant="neutral" style="soft" size="sm">
              Public
            </Badge>
          </CardHeader>
          <CardBody className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-[1fr_240px] gap-3">
              <div>
                <label className="label">
                  <span className="label-text">Fichier image</span>
                </label>
                <input
                  type="file"
                  accept="image/png,image/jpeg,image/webp"
                  className="file-input file-input-bordered w-full"
                  onChange={(e) => setNewPhotoFile(e.target.files?.[0] ?? null)}
                />
                <div className="mt-1 text-xs text-base-content/50">Formats : JPG/PNG/WEBP • max 10 Mo</div>
              </div>
              <div>
                <label className="label">
                  <span className="label-text">Texte alternatif</span>
                </label>
                <input className="input input-bordered w-full" value={newPhotoAlt} onChange={(e) => setNewPhotoAlt(e.target.value)} placeholder="Ex: intérieur du salon" />
              </div>
            </div>
            <button className="btn btn-primary btn-sm" onClick={() => void addPhoto()} disabled={saving || !newPhotoFile}>
              Ajouter la photo
            </button>

            {photos.length === 0 ? (
              <div className="text-base-content/60">Aucune photo pour le moment.</div>
            ) : (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                {photos.map((p) => (
                  <div key={p.id} className="rounded-2xl border border-base-300 bg-base-100 p-3">
                    <div className="flex items-start gap-3">
                      <div className="h-20 w-28 rounded-xl bg-base-200 overflow-hidden border border-base-300 shrink-0">
                        {getPhotoUrl(p) ? (
                          // eslint-disable-next-line @next/next/no-img-element
                          <img src={getPhotoUrl(p) ?? ''} alt={p.alt ?? 'Photo'} className="h-full w-full object-cover" />
                        ) : (
                          <div className="h-full w-full flex items-center justify-center text-base-content/50 text-sm">Photo</div>
                        )}
                      </div>
                      <div className="min-w-0 flex-1">
                        <div className="text-sm text-base-content/70 truncate">{p.path ?? p.url ?? '—'}</div>
                        {p.alt && <div className="text-xs text-base-content/50 truncate">{p.alt}</div>}
                        <div className="mt-2 flex flex-wrap items-center justify-between gap-2">
                          <div className="flex items-center gap-2">
                            <Badge variant={p.is_public ? 'success' : 'neutral'} style="soft" size="sm">
                              {p.is_public ? 'Publié' : 'Masqué'}
                            </Badge>
                            <button
                              className="btn btn-ghost btn-xs"
                              onClick={() => void setPhotoPublic(p.id, !p.is_public)}
                              disabled={saving}
                            >
                              {p.is_public ? 'Masquer' : 'Publier'}
                            </button>
                          </div>

                          <div className="flex items-center gap-1">
                            <button className="btn btn-ghost btn-xs" onClick={() => void movePhoto(p.id, 'up')} disabled={saving}>
                              ↑
                            </button>
                            <button className="btn btn-ghost btn-xs" onClick={() => void movePhoto(p.id, 'down')} disabled={saving}>
                              ↓
                            </button>
                            <button className="btn btn-ghost btn-xs text-error" onClick={() => void deletePhoto(p.id)} disabled={saving}>
                              Supprimer
                            </button>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}

            <div className="text-sm text-base-content/60">
              Les photos sont stockées dans Supabase Storage (bucket <span className="font-semibold">booking-provider-photos</span>).
            </div>
          </CardBody>
        </Card>

        <div className="flex flex-col sm:flex-row gap-2 justify-end">
          <Link href="/pro/booking" className="btn btn-ghost">
            Retour
          </Link>
          <button className="btn btn-primary" onClick={() => void save()} disabled={saving || loading}>
            {saving ? 'Enregistrement…' : 'Enregistrer'}
          </button>
        </div>
      </ContentStack>
    </AppShellScreen>
  )
}

