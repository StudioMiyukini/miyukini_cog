'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { useParams, useSearchParams } from 'next/navigation'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { BottomNav } from '@/components/navigation/BottomNav'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables } from '@/lib/supabase/database.types'
import { useRequireEnabledModule } from '../hooks/useRequireEnabledModule'
import { useAuth } from '@/contexts/AuthContext'

type BookingProvider = Tables<'booking_providers'>
type BookingService = Tables<'booking_services'>
type BookingProviderPhoto = Tables<'booking_provider_photos'>
type BookingProviderReview = Tables<'booking_provider_reviews'>

type ProviderRating = { avg: number; count: number }

type TabKey = 'rdv' | 'avis' | 'apropos'
const PHOTOS_BUCKET = 'booking-provider-photos'

function formatAddress(p: BookingProvider | null) {
  if (!p) return null
  const parts = [p.address_line1, p.address_line2].filter(Boolean).join(', ')
  const city = [p.postal_code, p.city].filter(Boolean).join(' ')
  const line = [parts, city].filter(Boolean).join(' • ')
  return line || null
}

function stars(avg: number) {
  const rounded = Math.round(avg * 2) / 2
  const full = Math.floor(rounded)
  const half = rounded - full >= 0.5
  const empty = Math.max(0, 5 - full - (half ? 1 : 0))
  return `${'★'.repeat(full)}${half ? '☆' : ''}${'✩'.repeat(empty)}`
}

function normalize(text: string) {
  return text
    .toLowerCase()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .trim()
}

function readOpeningHours(openingHours: any) {
  // Format recommandé (JSON) :
  // { mon:{open:"08:00",close:"19:00"}, tue:{closed:true}, ... }
  const map = openingHours ?? {}
  const keys = [
    { key: 'mon', label: 'Lundi', fallback: ['lundi', 'monday'] },
    { key: 'tue', label: 'Mardi', fallback: ['mardi', 'tuesday'] },
    { key: 'wed', label: 'Mercredi', fallback: ['mercredi', 'wednesday'] },
    { key: 'thu', label: 'Jeudi', fallback: ['jeudi', 'thursday'] },
    { key: 'fri', label: 'Vendredi', fallback: ['vendredi', 'friday'] },
    { key: 'sat', label: 'Samedi', fallback: ['samedi', 'saturday'] },
    { key: 'sun', label: 'Dimanche', fallback: ['dimanche', 'sunday'] },
  ]
  const get = (k: string, fb: string[]) => map?.[k] ?? fb.map((x) => map?.[x]).find(Boolean) ?? null
  return keys.map(({ key, label, fallback }) => {
    const v = get(key, fallback)
    if (!v) return { label, text: 'Non renseigné' }
    if (v.closed === true) return { label, text: 'Fermé' }
    const open = v.open ?? v.start ?? null
    const close = v.close ?? v.end ?? null
    if (!open || !close) return { label, text: 'Non renseigné' }
    return { label, text: `${open} - ${close}` }
  })
}

export function BookingProviderPublicScreen() {
  const { isLoading: modulesLoading, enabled } = useRequireEnabledModule('booking', '/')
  const supabase = useMemo(() => getSupabaseClient(), [])
  const { isAuthenticated, user } = useAuth()
  const params = useParams<{ providerId: string }>()
  const search = useSearchParams()

  const providerId = (params?.providerId as string) ?? ''
  const preselectedServiceId = search?.get('service') ?? ''

  const [loading, setLoading] = useState(true)
  const [provider, setProvider] = useState<BookingProvider | null>(null)
  const [services, setServices] = useState<BookingService[]>([])
  const [photos, setPhotos] = useState<BookingProviderPhoto[]>([])
  const [activePhoto, setActivePhoto] = useState(0)
  const [reviews, setReviews] = useState<BookingProviderReview[]>([])
  const [rating, setRating] = useState<ProviderRating>({ avg: 0, count: 0 })
  const [tab, setTab] = useState<TabKey>('rdv')

  const [serviceId, setServiceId] = useState<string>('')

  const loadProvider = useCallback(async () => {
    if (!providerId) return
    setLoading(true)
    try {
      const { data, error } = await supabase.from('booking_providers').select('*').eq('id', providerId).single()
      if (error) throw error
      setProvider((data ?? null) as any)
    } finally {
      setLoading(false)
    }
  }, [providerId, supabase])

  const loadPhotos = useCallback(async () => {
    if (!providerId) return
    const { data } = await supabase
      .from('booking_provider_photos')
      .select('*')
      .eq('provider_id', providerId)
      .eq('is_public', true)
      .order('sort_order', { ascending: true })
    setPhotos((data ?? []) as BookingProviderPhoto[])
    setActivePhoto(0)
  }, [providerId, supabase])

  const getPhotoUrl = useCallback(
    (p: BookingProviderPhoto | null | undefined) => {
      if (!p) return null
      if (p.url) return p.url
      if (p.path) return supabase.storage.from(PHOTOS_BUCKET).getPublicUrl(p.path).data.publicUrl
      return null
    },
    [supabase]
  )

  const loadReviews = useCallback(async () => {
    if (!providerId) return
    const { data } = await supabase
      .from('booking_provider_reviews')
      .select('*')
      .eq('provider_id', providerId)
      .eq('is_published', true)
      .order('created_at', { ascending: false })
      .limit(50)
    const list = (data ?? []) as BookingProviderReview[]
    setReviews(list)
    const sum = list.reduce((acc, r) => acc + (r.rating ?? 0), 0)
    setRating({ avg: list.length ? sum / list.length : 0, count: list.length })
  }, [providerId, supabase])

  const loadServices = useCallback(async () => {
    if (!providerId) return
    setLoading(true)
    try {
      const { data, error } = await supabase
        .from('booking_services')
        .select('*')
        .eq('provider_id', providerId)
        .eq('is_active', true)
        .order('created_at', { ascending: false })
      if (error) throw error
      const list = (data ?? []) as BookingService[]
      setServices(list)

      // init selection (querystring > current > first)
      const qs = preselectedServiceId
      if (qs && list.some((s) => s.id === qs)) {
        setServiceId(qs)
        return
      }
      if (!serviceId) {
        const first = list?.[0]?.id
        if (first) setServiceId(first)
      }
    } finally {
      setLoading(false)
    }
  }, [preselectedServiceId, providerId, serviceId, supabase])

  useEffect(() => {
    if (modulesLoading) return
    if (!enabled) return
    void loadProvider()
    void loadPhotos()
    void loadReviews()
  }, [enabled, loadPhotos, loadProvider, loadReviews, modulesLoading])

  useEffect(() => {
    if (!enabled) return
    void loadServices()
  }, [enabled, loadServices, providerId])

  // Si la liste de photos change (suppression / rechargement), on évite un index hors limite
  useEffect(() => {
    if (photos.length === 0) {
      if (activePhoto !== 0) setActivePhoto(0)
      return
    }
    if (activePhoto > photos.length - 1) setActivePhoto(0)
  }, [activePhoto, photos.length])

  const groupedServices = useMemo(() => {
    const groups = new Map<string, BookingService[]>()
    for (const s of services) {
      const key = (s.category ?? '').trim() || 'Prestations'
      groups.set(key, [...(groups.get(key) ?? []), s])
    }
    const entries = Array.from(groups.entries()).map(([category, items]) => ({
      category,
      items: items.sort((a, b) => {
        const as = a.category_sort ?? 9999
        const bs = b.category_sort ?? 9999
        if (as !== bs) return as - bs
        return normalize(a.name).localeCompare(normalize(b.name))
      }),
    }))
    // catégories: ordre alpha simple (MVP)
    return entries.sort((a, b) => normalize(a.category).localeCompare(normalize(b.category)))
  }, [services])

  if (modulesLoading) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }
  if (!enabled) return null

  const addr = formatAddress(provider)
  const opening = readOpeningHours((provider as any)?.opening_hours)
  const mapsUrl =
    provider?.lat != null && provider?.lng != null
      ? `https://www.google.com/maps?q=${provider.lat},${provider.lng}`
      : addr
        ? `https://www.google.com/maps/search/?api=1&query=${encodeURIComponent(addr)}`
        : null

  return (
    <AppShellScreen>
      <ContentStack>
        {/* Breadcrumb */}
        <div className="text-sm text-base-content/60">
          <Link className="link" href="/booking">
            Prestataires
          </Link>{' '}
          <span className="text-base-content/40">/</span> {provider?.display_name ?? 'Prestataire'}
        </div>

        {/* Hero (photos + infos) */}
        <div className="grid grid-cols-1 lg:grid-cols-[1.2fr_1fr] gap-4">
          <div className="rounded-3xl border border-base-300 bg-base-100 overflow-hidden">
            <div className="aspect-[16/9] bg-base-200">
              {getPhotoUrl(photos[activePhoto]) ? (
                // eslint-disable-next-line @next/next/no-img-element
                <img
                  src={getPhotoUrl(photos[activePhoto]) ?? ''}
                  alt={photos[activePhoto]?.alt ?? provider?.display_name ?? 'Prestataire'}
                  className="h-full w-full object-cover"
                />
              ) : (
                <div className="h-full w-full flex items-center justify-center text-base-content/50">Photos</div>
              )}
            </div>
            {photos.length > 1 && (
              <div className="p-3 border-t border-base-300">
                <div className="flex gap-2 overflow-x-auto">
                  {photos.slice(0, 12).map((ph, idx) => (
                    <button
                      key={ph.id}
                      className={[
                        'h-16 w-24 shrink-0 rounded-xl overflow-hidden border',
                        idx === activePhoto ? 'border-primary' : 'border-base-300',
                      ].join(' ')}
                      onClick={() => setActivePhoto(idx)}
                      aria-label="Voir la photo"
                    >
                      {getPhotoUrl(ph) ? (
                        // eslint-disable-next-line @next/next/no-img-element
                        <img src={getPhotoUrl(ph) ?? ''} alt={ph.alt ?? 'Photo'} className="h-full w-full object-cover" />
                      ) : (
                        <div className="h-full w-full bg-base-200" />
                      )}
                    </button>
                  ))}
                </div>
              </div>
            )}
          </div>

          <div className="rounded-3xl border border-base-300 bg-base-100 p-5 space-y-3">
            <div className="flex items-start justify-between gap-3">
              <div className="min-w-0">
                <h1 className="text-3xl font-bold text-base-content leading-tight truncate">
                  {provider?.display_name ?? 'Prestataire'}
                </h1>
                {addr && (
                  <div className="mt-1 text-sm text-base-content/60">
                    <span className="icon-[tabler--map-pin] size-4 inline-block align-[-2px] mr-1" />
                    {addr}
                  </div>
                )}
                <div className="mt-2 flex items-center gap-2 text-sm">
                  {rating.count > 0 ? (
                    <>
                      <span className="text-base-content">{stars(rating.avg)}</span>
                      <span className="font-semibold text-base-content">{rating.avg.toFixed(1)}</span>
                      <button className="link text-sm" onClick={() => setTab('avis')}>
                        ({rating.count} avis)
                      </button>
                    </>
                  ) : (
                    <span className="text-base-content/50">Pas encore d’avis</span>
                  )}
                </div>
              </div>
              <button
                className="btn btn-ghost btn-sm shrink-0"
                onClick={() => {
                  void loadProvider()
                  void loadPhotos()
                  void loadReviews()
                  void loadServices()
                }}
                disabled={loading}
              >
                <span className="icon-[tabler--refresh] size-4" />
                Rafraîchir
              </button>
            </div>

            {!!(provider?.tags?.length ?? 0) && (
              <div className="flex flex-wrap gap-2">
                {(provider?.tags ?? []).slice(0, 10).map((t) => (
                  <Badge key={t} variant="neutral" style="soft" size="sm">
                    {t}
                  </Badge>
                ))}
              </div>
            )}

            <div className="flex flex-col sm:flex-row gap-2 pt-1">
              <button className="btn btn-primary" onClick={() => setTab('rdv')}>
                Prendre RDV
              </button>
              {mapsUrl && (
                <a className="btn btn-ghost" href={mapsUrl} target="_blank" rel="noreferrer">
                  <span className="icon-[tabler--map] size-4" />
                  Itinéraire
                </a>
              )}
              <button className="btn btn-ghost" onClick={() => setTab('apropos')}>
                À propos
              </button>
            </div>
          </div>
        </div>

        {/* Tabs */}
        <div className="flex flex-wrap gap-2">
          <button className={`btn btn-sm ${tab === 'rdv' ? 'btn-primary' : 'btn-ghost'}`} onClick={() => setTab('rdv')}>
            Prendre RDV
          </button>
          <button className={`btn btn-sm ${tab === 'avis' ? 'btn-primary' : 'btn-ghost'}`} onClick={() => setTab('avis')}>
            Avis
          </button>
          <button className={`btn btn-sm ${tab === 'apropos' ? 'btn-primary' : 'btn-ghost'}`} onClick={() => setTab('apropos')}>
            À propos
          </button>
        </div>

        {tab === 'rdv' && (
          <div className="grid grid-cols-1 lg:grid-cols-[1.2fr_0.8fr] gap-4">
            {/* Choix prestation (le planning est sur un écran dédié) */}
            <div className="space-y-4">
              <Card>
                <CardHeader className="flex items-center justify-between">
                  <span className="font-semibold text-base-content">Choix de la prestation</span>
                  {loading && <span className="loading loading-spinner loading-sm text-primary" />}
                </CardHeader>
                <CardBody className="space-y-4">
                  <div className="text-sm text-base-content/60">
                    Choisis une prestation, puis consulte les disponibilités sur l’écran “Planning”.
                  </div>

                  {groupedServices.length === 0 ? (
                    <div className="text-base-content/60">Aucune prestation disponible.</div>
                  ) : (
                    <div className="space-y-3">
                      {groupedServices.map((g) => (
                        <details key={g.category} className="rounded-2xl border border-base-300 bg-base-100" open>
                          <summary className="cursor-pointer select-none px-4 py-3 font-semibold text-base-content flex items-center justify-between">
                            <span className="truncate">{g.category}</span>
                            <span className="icon-[tabler--chevron-down] size-4 text-base-content/50" />
                          </summary>
                          <div className="px-4 pb-4 space-y-2">
                            {g.items.map((s) => (
                              <div
                                key={s.id}
                                className="flex items-center justify-between gap-3 rounded-xl border border-base-200 bg-base-100 p-3"
                              >
                                <div className="min-w-0">
                                  <div className="font-medium text-base-content truncate">{s.name}</div>
                                  <div className="text-sm text-base-content/60">
                                    {s.duration_minutes} min{typeof s.price_hint === 'number' ? ` • ${s.price_hint} €` : ''}
                                  </div>
                                </div>
                                <div className="flex items-center gap-2">
                                  <button
                                    className={`btn btn-sm ${serviceId === s.id ? 'btn-primary' : 'btn-ghost'}`}
                                    onClick={() => setServiceId(s.id)}
                                  >
                                    Sélectionner
                                  </button>
                                  <Link
                                    className="btn btn-sm btn-primary"
                                    href={`/booking/${providerId}/planning?service=${encodeURIComponent(s.id)}`}
                                  >
                                    Voir planning
                                  </Link>
                                </div>
                              </div>
                            ))}
                          </div>
                        </details>
                      ))}
                    </div>
                  )}
                </CardBody>
              </Card>
            </div>

            {/* Sidebar info */}
            <div className="space-y-4">
              <Card>
                <CardHeader>
                  <span className="font-semibold text-base-content">Horaires d’ouverture</span>
                </CardHeader>
                <CardBody className="space-y-2">
                  {opening.map((d) => (
                    <div key={d.label} className="flex items-center justify-between gap-3 text-sm">
                      <span className="text-base-content/70">{d.label}</span>
                      <span className="font-medium text-base-content">{d.text}</span>
                    </div>
                  ))}
                </CardBody>
              </Card>

              <Card>
                <CardHeader>
                  <span className="font-semibold text-base-content">Coordonnées</span>
                </CardHeader>
                <CardBody className="space-y-2 text-sm">
                  {addr && (
                    <div className="text-base-content/70">
                      <span className="icon-[tabler--map-pin] size-4 inline-block align-[-2px] mr-1" />
                      {addr}
                    </div>
                  )}
                  {(provider as any)?.phone && (
                    <div className="text-base-content/70">
                      <span className="icon-[tabler--phone] size-4 inline-block align-[-2px] mr-1" />
                      {(provider as any).phone}
                    </div>
                  )}
                  {(provider as any)?.website && (
                    <a className="link" href={(provider as any).website} target="_blank" rel="noreferrer">
                      {(provider as any).website}
                    </a>
                  )}
                  {mapsUrl && (
                    <a className="btn btn-ghost btn-sm w-full" href={mapsUrl} target="_blank" rel="noreferrer">
                      <span className="icon-[tabler--map] size-4" />
                      Voir sur la carte
                    </a>
                  )}
                </CardBody>
              </Card>
            </div>
          </div>
        )}

        {tab === 'avis' && (
          <Card>
            <CardHeader className="flex items-center justify-between">
              <span className="font-semibold text-base-content">Avis</span>
              <div className="text-sm text-base-content/70">
                {rating.count > 0 ? (
                  <>
                    <span className="text-base-content">{stars(rating.avg)}</span> <span className="font-semibold">{rating.avg.toFixed(1)}</span>{' '}
                    <span className="text-base-content/60">({rating.count})</span>
                  </>
                ) : (
                  'Aucun avis'
                )}
              </div>
            </CardHeader>
            <CardBody className="space-y-3">
              {reviews.length === 0 ? (
                <div className="text-base-content/60">Aucun avis pour le moment.</div>
              ) : (
                reviews.map((r) => (
                  <div key={r.id} className="rounded-2xl border border-base-300 bg-base-100 p-4">
                    <div className="flex items-center justify-between gap-3">
                      <div className="font-semibold text-base-content truncate">{r.author_name ?? 'Client'}</div>
                      <div className="text-sm text-base-content/70">{'★'.repeat(r.rating)}{'✩'.repeat(Math.max(0, 5 - r.rating))}</div>
                    </div>
                    {r.comment && <div className="mt-2 text-sm text-base-content/70 whitespace-pre-wrap">{r.comment}</div>}
                    <div className="mt-2 text-xs text-base-content/50">
                      {new Date(r.created_at).toLocaleDateString('fr-FR', { year: 'numeric', month: 'long', day: '2-digit' })}
                    </div>
                  </div>
                ))
              )}
            </CardBody>
          </Card>
        )}

        {tab === 'apropos' && (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
            <Card>
              <CardHeader>
                <span className="font-semibold text-base-content">À propos</span>
              </CardHeader>
              <CardBody className="space-y-3">
                {(provider as any)?.description ? (
                  <div className="text-base-content/70 whitespace-pre-wrap">{(provider as any).description}</div>
                ) : (
                  <div className="text-base-content/60">Description non renseignée.</div>
                )}
                {(provider as any)?.phone && (
                  <div className="text-sm text-base-content/70">
                    <span className="icon-[tabler--phone] size-4 inline-block align-[-2px] mr-1" />
                    {(provider as any).phone}
                  </div>
                )}
                {(provider as any)?.website && (
                  <a className="link" href={(provider as any).website} target="_blank" rel="noreferrer">
                    {(provider as any).website}
                  </a>
                )}
              </CardBody>
            </Card>

            <Card>
              <CardHeader>
                <span className="font-semibold text-base-content">Horaires</span>
              </CardHeader>
              <CardBody className="space-y-2">
                {opening.map((d) => (
                  <div key={d.label} className="flex items-center justify-between gap-3 text-sm">
                    <span className="text-base-content/70">{d.label}</span>
                    <span className="font-medium text-base-content">{d.text}</span>
                  </div>
                ))}
              </CardBody>
            </Card>
          </div>
        )}
      </ContentStack>
      <BottomNav />
    </AppShellScreen>
  )
}

