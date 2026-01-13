'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { BottomNav } from '@/components/navigation/BottomNav'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables } from '@/lib/supabase/database.types'
import { useRequireEnabledModule } from '../hooks/useRequireEnabledModule'

type BookingProvider = Tables<'booking_providers'>
type BookingProviderPhoto = Tables<'booking_provider_photos'>
type BookingProviderReview = Tables<'booking_provider_reviews'>

type ProviderRating = { avg: number; count: number }
const PHOTOS_BUCKET = 'booking-provider-photos'

function normalize(text: string) {
  return text
    .toLowerCase()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .trim()
}

export function BookingProvidersDirectoryScreen() {
  const { isLoading: modulesLoading, enabled } = useRequireEnabledModule('booking', '/')
  const supabase = useMemo(() => getSupabaseClient(), [])

  const [loading, setLoading] = useState(true)
  const [providers, setProviders] = useState<BookingProvider[]>([])
  const [query, setQuery] = useState('')
  const [cityQuery, setCityQuery] = useState('')
  const [thumbByProvider, setThumbByProvider] = useState<Record<string, BookingProviderPhoto | null>>({})
  const [ratingByProvider, setRatingByProvider] = useState<Record<string, ProviderRating>>({})

  const getPhotoUrl = useCallback(
    (p: BookingProviderPhoto | null) => {
      if (!p) return null
      if (p.url) return p.url
      if (p.path) return supabase.storage.from(PHOTOS_BUCKET).getPublicUrl(p.path).data.publicUrl
      return null
    },
    [supabase]
  )

  const formatAddress = useCallback((p: BookingProvider) => {
    const parts = [p.address_line1, p.address_line2].filter(Boolean).join(', ')
    const city = [p.postal_code, p.city].filter(Boolean).join(' ')
    const line = [parts, city].filter(Boolean).join(' • ')
    return line || null
  }, [])

  const stars = useCallback((avg: number) => {
    const rounded = Math.round(avg * 2) / 2
    const full = Math.floor(rounded)
    const half = rounded - full >= 0.5
    const empty = Math.max(0, 5 - full - (half ? 1 : 0))
    return `${'★'.repeat(full)}${half ? '☆' : ''}${'✩'.repeat(empty)}`
  }, [])

  const loadProviders = useCallback(async () => {
    setLoading(true)
    try {
      const { data, error } = await supabase
        .from('booking_providers')
        .select('*')
        .eq('is_active', true)
        .order('created_at', { ascending: false })
      if (error) throw error
      const list = (data ?? []) as BookingProvider[]
      setProviders(list)

      const ids = list.map((p) => p.id)
      if (ids.length === 0) {
        setThumbByProvider({})
        setRatingByProvider({})
        return
      }

      // Thumbnails (1ère photo publique)
      const { data: photosData } = await supabase
        .from('booking_provider_photos')
        .select('provider_id,url,path,alt,sort_order,is_public,created_at,updated_at,id')
        .in('provider_id', ids)
        .eq('is_public', true)
        .order('sort_order', { ascending: true })
      const photos = (photosData ?? []) as BookingProviderPhoto[]
      const byProvider: Record<string, BookingProviderPhoto | null> = {}
      for (const p of ids) byProvider[p] = null
      for (const ph of photos) {
        if (!byProvider[ph.provider_id]) byProvider[ph.provider_id] = ph
      }
      setThumbByProvider(byProvider)

      // Ratings (avis publiés)
      const { data: reviewsData } = await supabase
        .from('booking_provider_reviews')
        .select('provider_id,rating')
        .in('provider_id', ids)
        .eq('is_published', true)
      const reviews = (reviewsData ?? []) as Pick<BookingProviderReview, 'provider_id' | 'rating'>[]
      const ratingMap: Record<string, { sum: number; count: number }> = {}
      for (const p of ids) ratingMap[p] = { sum: 0, count: 0 }
      for (const r of reviews) {
        ratingMap[r.provider_id] ??= { sum: 0, count: 0 }
        ratingMap[r.provider_id].sum += r.rating
        ratingMap[r.provider_id].count += 1
      }
      const out: Record<string, ProviderRating> = {}
      for (const [pid, v] of Object.entries(ratingMap)) {
        out[pid] = { avg: v.count ? v.sum / v.count : 0, count: v.count }
      }
      setRatingByProvider(out)
    } finally {
      setLoading(false)
    }
  }, [supabase])

  useEffect(() => {
    if (modulesLoading) return
    if (!enabled) return
    void loadProviders()
  }, [enabled, loadProviders, modulesLoading])

  const filtered = useMemo(() => {
    const q = normalize(query)
    const cq = normalize(cityQuery)
    const base = providers.filter((p) => {
      if (!cq) return true
      return normalize(p.city ?? '').includes(cq) || normalize(p.postal_code ?? '').includes(cq)
    })
    if (!q) return base
    return base.filter((p) => {
      const name = normalize(p.display_name ?? '')
      const tags = normalize((p.tags ?? []).join(' '))
      const city = normalize(`${p.postal_code ?? ''} ${p.city ?? ''}`)
      return name.includes(q) || tags.includes(q) || city.includes(q)
    })
  }, [cityQuery, providers, query])

  if (modulesLoading) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }
  if (!enabled) return null

  return (
    <AppShellScreen>
      <ContentStack>
        <div className="space-y-2">
          <h1 className="text-2xl font-bold text-base-content">Réserver</h1>
          <p className="text-base-content/60">
            Recherche un prestataire, ouvre sa page publique, puis choisis un créneau dans son planning.
          </p>
        </div>

        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold text-base-content">Prestataires</span>
            {loading && <span className="loading loading-spinner loading-sm text-primary" />}
          </CardHeader>
          <CardBody className="space-y-3">
            <div className="grid grid-cols-1 md:grid-cols-4 gap-3">
              <div className="md:col-span-2">
                <label className="label">
                  <span className="label-text">Métier / prestation</span>
                </label>
                <input
                  className="input input-bordered w-full"
                  value={query}
                  onChange={(e) => setQuery(e.target.value)}
                  placeholder="Nom, tag… (ex: massage, coiffure)"
                />
              </div>
              <div>
                <label className="label">
                  <span className="label-text">Ville</span>
                </label>
                <input
                  className="input input-bordered w-full"
                  value={cityQuery}
                  onChange={(e) => setCityQuery(e.target.value)}
                  placeholder="Paris, 75011…"
                />
              </div>
              <div className="flex items-end">
                <button className="btn btn-ghost w-full" onClick={() => void loadProviders()} disabled={loading}>
                  <span className="icon-[tabler--refresh] size-4" />
                  Rafraîchir
                </button>
              </div>
            </div>

            <div className="text-sm text-base-content/60">
              Résultats : <span className="font-semibold text-base-content">{filtered.length}</span>
            </div>

            {filtered.length === 0 ? (
              <div className="text-base-content/60">Aucun prestataire trouvé.</div>
            ) : (
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
                {/* Liste */}
                <div className="space-y-3">
                  {filtered.map((p) => {
                    const addr = formatAddress(p)
                    const thumb = thumbByProvider[p.id]
                    const rating = ratingByProvider[p.id] ?? { avg: 0, count: 0 }
                    const thumbUrl = getPhotoUrl(thumb)
                    return (
                      <Link
                        key={p.id}
                        href={`/booking/${p.id}`}
                        className="rounded-2xl border border-base-300 bg-base-100 hover:bg-base-200/40 transition-colors overflow-hidden"
                      >
                        <div className="grid grid-cols-[120px_1fr] gap-4 p-4">
                          <div className="aspect-[4/3] rounded-xl bg-base-200 overflow-hidden border border-base-300">
                            {thumbUrl ? (
                              // eslint-disable-next-line @next/next/no-img-element
                              <img src={thumbUrl} alt={thumb?.alt ?? p.display_name ?? 'Prestataire'} className="h-full w-full object-cover" />
                            ) : (
                              <div className="h-full w-full flex items-center justify-center text-base-content/40 text-sm">
                                Photo
                              </div>
                            )}
                          </div>
                          <div className="min-w-0">
                            <div className="flex items-start justify-between gap-3">
                              <div className="min-w-0">
                                <div className="font-semibold text-base-content truncate">
                                  {p.display_name ?? 'Prestataire'}
                                </div>
                                {addr && (
                                  <div className="mt-0.5 text-sm text-base-content/60 truncate">
                                    <span className="icon-[tabler--map-pin] size-4 inline-block align-[-2px] mr-1" />
                                    {addr}
                                  </div>
                                )}
                              </div>
                              <span className="icon-[tabler--chevron-right] size-5 text-base-content/50 shrink-0" />
                            </div>

                            <div className="mt-2 flex flex-wrap items-center gap-2 text-sm text-base-content/70">
                              {rating.count > 0 ? (
                                <>
                                  <span className="text-base-content">{stars(rating.avg)}</span>
                                  <span className="font-semibold text-base-content">{rating.avg.toFixed(1)}</span>
                                  <span className="text-base-content/60">({rating.count} avis)</span>
                                </>
                              ) : (
                                <span className="text-base-content/50">Pas encore d’avis</span>
                              )}
                            </div>

                            {!!(p.tags?.length ?? 0) && (
                              <div className="mt-3 flex flex-wrap gap-2">
                                {(p.tags ?? []).slice(0, 6).map((t) => (
                                  <Badge key={t} variant="neutral" style="soft" size="sm">
                                    {t}
                                  </Badge>
                                ))}
                              </div>
                            )}
                          </div>
                        </div>
                      </Link>
                    )
                  })}
                </div>

                {/* Colonne “carte” (placeholder UX Planity) */}
                <div className="hidden lg:block">
                  <div className="sticky top-24 rounded-2xl border border-base-300 bg-base-100 overflow-hidden">
                    <div className="p-4 border-b border-base-300">
                      <div className="font-semibold text-base-content">Carte</div>
                      <div className="text-sm text-base-content/60">Vue carte (à brancher plus tard)</div>
                    </div>
                    <div className="h-[520px] bg-base-200 flex items-center justify-center text-base-content/50">
                      Map
                    </div>
                  </div>
                </div>
              </div>
            )}

            <div className="text-sm text-base-content/60">
              Prestataire ? <Link className="link" href="/pro/booking">Accéder à l’espace prestataire</Link>
            </div>
          </CardBody>
        </Card>
      </ContentStack>
      <BottomNav />
    </AppShellScreen>
  )
}

