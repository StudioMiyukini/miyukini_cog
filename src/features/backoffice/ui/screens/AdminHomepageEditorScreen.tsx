'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import Link from 'next/link'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { IconPickerModal } from '@/components/molecules/icon-picker'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables } from '@/lib/supabase/database.types'
import { useAuth } from '@/contexts/AuthContext'
import { useRequireAdmin } from '../hooks/useRequireAdmin'

type HomepageRow = Tables<'homepage_content'>

type SectionKey = 'hero' | 'stats' | 'quickActions' | 'faq' | 'onboarding'

const SECTION_ORDER: SectionKey[] = ['hero', 'stats', 'quickActions', 'faq', 'onboarding']

const SECTION_META: Record<SectionKey, { title: string; description: string }> = {
  hero: { title: 'Hero', description: 'Bloc principal avec titre, CTA et badge' },
  stats: { title: 'Stats', description: 'Valeurs clés (XP, composants, communauté)' },
  quickActions: { title: 'Quick actions', description: 'Cartes d’accès rapides aux modules' },
  faq: { title: 'FAQ', description: 'Bloc de questions fréquentes' },
  onboarding: { title: 'Onboarding', description: 'Étapes d’accueil / tutoriel' },
}

type HomepageConfig = {
  hero?: {
    badgeText?: string
    title?: { line1?: string; line2?: string; highlight?: boolean }
    subtitle?: string
    primaryCta?: { label?: string; href?: string }
    secondaryCta?: { label?: string; href?: string }
  }
  sectionsEnabled?: { stats?: boolean; quickActions?: boolean; faq?: boolean; onboarding?: boolean }
  stats?: Array<{ value?: string; label?: string }>
  quickActions?: Array<{ id?: string; label?: string; icon?: string; path?: string; description?: string }>
  faq?: Array<{ title?: string; content?: string; defaultOpen?: boolean }>
  onboardingSteps?: Array<{ id?: number; title?: string; description?: string; icon?: string }>
  sectionOrder?: SectionKey[]
}

function safeJsonStringify(value: unknown) {
  try {
    return JSON.stringify(value ?? {}, null, 2)
  } catch {
    return '{\n  \n}'
  }
}

export function AdminHomepageEditorScreen() {
  const { isReady, authLoading } = useRequireAdmin()
  const { profile } = useAuth()
  const supabase = useMemo(() => getSupabaseClient(), [])

  const [loading, setLoading] = useState(true)
  const [row, setRow] = useState<HomepageRow | null>(null)
  const [isPublished, setIsPublished] = useState(true)
  const [config, setConfig] = useState<HomepageConfig>({})
  const [rawJson, setRawJson] = useState('')
  const [rawJsonError, setRawJsonError] = useState<string | null>(null)
  const [activeTab, setActiveTab] = useState<'hero' | 'sections' | 'raw'>('hero')

  const [iconPickerOpen, setIconPickerOpen] = useState(false)
  const [iconPickerTarget, setIconPickerTarget] = useState<
    | { kind: 'quickAction'; index: number }
    | { kind: 'onboarding'; index: number }
    | null
  >(null)

  const load = useCallback(async () => {
    setLoading(true)
    try {
      const { data, error } = await supabase.from('homepage_content').select('*').eq('id', 'home').maybeSingle()
      if (error) throw error
      const r = data ?? null
      setRow(r)
      setIsPublished(r?.is_published ?? true)
      const cfg = (r?.config ?? {}) as unknown as HomepageConfig
      setConfig(cfg)
      setRawJson(safeJsonStringify(cfg))
      setRawJsonError(null)
    } finally {
      setLoading(false)
    }
  }, [supabase])

  useEffect(() => {
    if (!authLoading && isReady) void load()
  }, [authLoading, isReady, load])

  const setConfigDeep = (patch: Partial<HomepageConfig>) => {
    setConfig((prev) => {
      const next: HomepageConfig = {
        ...prev,
        ...patch,
        hero: patch.hero ? { ...prev.hero, ...patch.hero } : prev.hero,
        sectionsEnabled: patch.sectionsEnabled
          ? { ...(prev.sectionsEnabled ?? {}), ...patch.sectionsEnabled }
          : prev.sectionsEnabled,
        stats: patch.stats ?? prev.stats,
        quickActions: patch.quickActions ?? prev.quickActions,
        faq: patch.faq ?? prev.faq,
        onboardingSteps: patch.onboardingSteps ?? prev.onboardingSteps,
        sectionOrder: patch.sectionOrder ?? prev.sectionOrder,
      }

      setRawJson(safeJsonStringify(next))
      return next
    })
  }

  const save = async () => {
    setLoading(true)
    try {
      const payload = {
        id: 'home',
        is_published: isPublished,
        config: config as any,
        updated_by: profile?.id ?? null,
        updated_at: new Date().toISOString(),
      }
      const { error } = await supabase.from('homepage_content').upsert(payload, { onConflict: 'id' })
      if (error) throw error
      await load()
    } finally {
      setLoading(false)
    }
  }

  const applyRawJson = () => {
    try {
      const parsed = JSON.parse(rawJson) as HomepageConfig
      setConfig(parsed ?? {})
      setRawJsonError(null)
    } catch (e: any) {
      setRawJsonError(e?.message ?? 'JSON invalide')
    }
  }

  const sectionsEnabled = config.sectionsEnabled ?? {}
  const stats = config.stats ?? []
  const quickActions = useMemo(() => config.quickActions ?? [], [config.quickActions])
  const faq = config.faq ?? []
  const onboarding = useMemo(() => config.onboardingSteps ?? [], [config.onboardingSteps])

  const sectionOrder = config.sectionOrder ?? SECTION_ORDER
  const sectionEnabled = (section: SectionKey) => (sectionsEnabled as Record<SectionKey, boolean | undefined>)[section] ?? true
  const setSectionOrder = (order: SectionKey[]) => setConfigDeep({ sectionOrder: order })
  const moveSection = (section: SectionKey, direction: 'up' | 'down') => {
    const currentIndex = sectionOrder.indexOf(section)
    const targetIndex = direction === 'up' ? currentIndex - 1 : currentIndex + 1
    if (targetIndex < 0 || targetIndex >= sectionOrder.length) return
    const next = [...sectionOrder]
    next[currentIndex] = next[targetIndex]
    next[targetIndex] = section
    setSectionOrder(next)
  }
  const toggleSectionVisibility = (section: SectionKey) => {
    setConfigDeep({
      sectionsEnabled: {
        ...(sectionsEnabled as Record<SectionKey, boolean | undefined>),
        [section]: !sectionEnabled(section),
      },
    })
  }

  const iconPickerValue = useMemo(() => {
    if (!iconPickerTarget) return undefined
    if (iconPickerTarget.kind === 'quickAction') return quickActions[iconPickerTarget.index]?.icon
    return onboarding[iconPickerTarget.index]?.icon
  }, [iconPickerTarget, onboarding, quickActions])

  if (authLoading || !isReady) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }

  return (
    <AdminLayout title="Homepage (éditeur)">
      <div className="flex items-start justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold text-base-content">Homepage (éditeur)</h1>
          <p className="text-base-content/60 mt-1">Personnalise la page d’accueil `/` (contenu stocké en base).</p>
        </div>
        <div className="flex gap-2">
          <Link href="/" className="btn btn-ghost btn-sm">
            <span className="icon-[tabler--external-link] size-4" />
            Voir `/`
          </Link>
          <button className="btn btn-ghost btn-sm" onClick={() => void load()} disabled={loading}>
            <span className="icon-[tabler--refresh] size-4" />
            Rafraîchir
          </button>
          <button className="btn btn-primary btn-sm" onClick={() => void save()} disabled={loading}>
            {loading ? <span className="loading loading-spinner loading-sm" /> : <span className="icon-[tabler--device-floppy] size-4" />}
            Enregistrer
          </button>
        </div>
      </div>

      <div className="mt-6 flex flex-wrap items-center gap-2">
        <Badge variant="neutral" style="soft" size="sm">
          id: home
        </Badge>
        <label className="flex items-center gap-2 text-sm">
          <input
            type="checkbox"
            className="toggle toggle-primary"
            checked={isPublished}
            onChange={() => setIsPublished((v) => !v)}
          />
          Publié (visible publiquement)
        </label>
      </div>

      <Card className="mt-6">
        <CardHeader className="flex items-center justify-between">
          <span className="font-semibold">Section builder (style Elementor)</span>
          <span className="text-xs text-base-content/60">WordPress inspo</span>
        </CardHeader>
        <CardBody className="space-y-3">
          {sectionOrder.map((section, index) => (
            <div
              key={section}
              className="flex items-center justify-between gap-3 rounded-2xl border border-base-300 bg-base-100 px-3 py-2"
            >
              <div className="min-w-0">
                <div className="font-medium">{SECTION_META[section].title}</div>
                <p className="text-xs text-base-content/60 truncate">{SECTION_META[section].description}</p>
              </div>
              <div className="flex flex-col gap-1 text-xs text-base-content/70">
                <button
                  className="btn btn-ghost btn-xs"
                  onClick={() => moveSection(section, 'up')}
                  disabled={index === 0}
                >
                  ↑
                </button>
                <button
                  className="btn btn-ghost btn-xs"
                  onClick={() => moveSection(section, 'down')}
                  disabled={index === sectionOrder.length - 1}
                >
                  ↓
                </button>
              </div>
              <label className="flex items-center gap-2 text-xs">
                <input
                  type="checkbox"
                  className="checkbox checkbox-sm checkbox-primary"
                  checked={sectionEnabled(section)}
                  onChange={() => toggleSectionVisibility(section)}
                />
                Visible
              </label>
              <button
                className="btn btn-link btn-xs text-primary"
                onClick={() => setActiveTab(section === 'hero' ? 'hero' : 'sections')}
              >
                Éditer
              </button>
            </div>
          ))}
          <button className="btn btn-ghost btn-sm w-full justify-center" onClick={() => setSectionOrder(SECTION_ORDER)}>
            Restituer l’ordre Elementor
          </button>
          <p className="text-xs text-base-content/50">
            Inspiré de WordPress/Elementor : chaque section est un bloc flexible. Réorganise, masque, et clique sur “Éditer” pour la personnaliser.
          </p>
        </CardBody>
      </Card>

      <div className="mt-6 join">
        <button className={`btn btn-sm join-item ${activeTab === 'hero' ? 'btn-primary' : 'btn-ghost'}`} onClick={() => setActiveTab('hero')}>
          Hero
        </button>
        <button className={`btn btn-sm join-item ${activeTab === 'sections' ? 'btn-primary' : 'btn-ghost'}`} onClick={() => setActiveTab('sections')}>
          Sections
        </button>
        <button className={`btn btn-sm join-item ${activeTab === 'raw' ? 'btn-primary' : 'btn-ghost'}`} onClick={() => setActiveTab('raw')}>
          JSON (avancé)
        </button>
      </div>

      {activeTab === 'hero' ? (
        <div className="mt-6 grid grid-cols-1 lg:grid-cols-2 gap-6">
          <Card>
            <CardHeader className="flex items-center justify-between">
              <span className="font-semibold">Hero</span>
              {loading && <span className="loading loading-spinner loading-sm text-primary" />}
            </CardHeader>
            <CardBody className="space-y-4">
              <div>
                <label className="label">
                  <span className="label-text">Badge</span>
                </label>
                <input
                  className="input input-bordered w-full"
                  value={config.hero?.badgeText ?? ''}
                  onChange={(e) =>
                    setConfigDeep({ hero: { ...(config.hero ?? {}), badgeText: e.target.value } })
                  }
                  placeholder="Framework modulaire depuis 2016"
                />
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                <div>
                  <label className="label">
                    <span className="label-text">Titre ligne 1</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    value={config.hero?.title?.line1 ?? ''}
                    onChange={(e) =>
                      setConfigDeep({
                        hero: {
                          ...(config.hero ?? {}),
                          title: { ...(config.hero?.title ?? {}), line1: e.target.value },
                        },
                      })
                    }
                  />
                </div>
                <div>
                  <label className="label">
                    <span className="label-text">Titre ligne 2</span>
                  </label>
                  <input
                    className="input input-bordered w-full"
                    value={config.hero?.title?.line2 ?? ''}
                    onChange={(e) =>
                      setConfigDeep({
                        hero: {
                          ...(config.hero ?? {}),
                          title: { ...(config.hero?.title ?? {}), line2: e.target.value },
                        },
                      })
                    }
                  />
                </div>
              </div>

              <div>
                <label className="label">
                  <span className="label-text">Sous-titre</span>
                </label>
                <textarea
                  className="textarea textarea-bordered w-full min-h-[96px]"
                  value={config.hero?.subtitle ?? ''}
                  onChange={(e) =>
                    setConfigDeep({ hero: { ...(config.hero ?? {}), subtitle: e.target.value } })
                  }
                />
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
                <div className="space-y-2">
                  <div className="font-medium text-sm text-base-content">CTA primaire</div>
                  <input
                    className="input input-bordered w-full"
                    value={config.hero?.primaryCta?.label ?? ''}
                    placeholder="Label"
                    onChange={(e) =>
                      setConfigDeep({
                        hero: { ...(config.hero ?? {}), primaryCta: { ...(config.hero?.primaryCta ?? {}), label: e.target.value } },
                      })
                    }
                  />
                  <input
                    className="input input-bordered w-full"
                    value={config.hero?.primaryCta?.href ?? ''}
                    placeholder="/mockcontent"
                    onChange={(e) =>
                      setConfigDeep({
                        hero: { ...(config.hero ?? {}), primaryCta: { ...(config.hero?.primaryCta ?? {}), href: e.target.value } },
                      })
                    }
                  />
                </div>
                <div className="space-y-2">
                  <div className="font-medium text-sm text-base-content">CTA secondaire</div>
                  <input
                    className="input input-bordered w-full"
                    value={config.hero?.secondaryCta?.label ?? ''}
                    placeholder="Label"
                    onChange={(e) =>
                      setConfigDeep({
                        hero: { ...(config.hero ?? {}), secondaryCta: { ...(config.hero?.secondaryCta ?? {}), label: e.target.value } },
                      })
                    }
                  />
                  <input
                    className="input input-bordered w-full"
                    value={config.hero?.secondaryCta?.href ?? ''}
                    placeholder="/admin"
                    onChange={(e) =>
                      setConfigDeep({
                        hero: { ...(config.hero ?? {}), secondaryCta: { ...(config.hero?.secondaryCta ?? {}), href: e.target.value } },
                      })
                    }
                  />
                </div>
              </div>
            </CardBody>
          </Card>

          <Card>
            <CardHeader className="flex items-center justify-between">
              <span className="font-semibold">Aperçu (Hero)</span>
              <Badge variant="neutral" style="soft" size="sm">
                {isPublished ? 'Publié' : 'Brouillon'}
              </Badge>
            </CardHeader>
            <CardBody>
              <div className="rounded-2xl border border-base-300 bg-base-100 p-6 text-center">
                <div className="inline-flex bg-base-200 border border-base-content/10 w-fit rounded-full px-4 py-1.5">
                  <span className="text-sm text-base-content/80">
                    {config.hero?.badgeText ?? '—'}
                  </span>
                </div>
                <h2 className="mt-5 text-2xl sm:text-3xl font-bold text-base-content">
                  {config.hero?.title?.line1 ?? '—'}
                  <br />
                  <span className="text-primary">{config.hero?.title?.line2 ?? '—'}</span>
                </h2>
                <p className="mt-4 text-base-content/70">{config.hero?.subtitle ?? '—'}</p>
                <div className="mt-6 flex flex-wrap justify-center gap-3">
                  <div className="btn btn-primary btn-sm">{config.hero?.primaryCta?.label ?? 'CTA'}</div>
                  <div className="btn btn-outline btn-sm">{config.hero?.secondaryCta?.label ?? 'CTA'}</div>
                </div>
              </div>
            </CardBody>
          </Card>
        </div>
      ) : null}

      {activeTab === 'sections' ? (
        <div className="mt-6 space-y-6">
          <Card>
            <CardHeader className="flex items-center justify-between">
              <span className="font-semibold">Sections activées</span>
            </CardHeader>
            <CardBody className="grid grid-cols-1 md:grid-cols-2 gap-3">
              {(['stats', 'quickActions', 'faq', 'onboarding'] as const).map((k) => (
                <label key={k} className="flex items-center gap-2 text-sm">
                  <input
                    type="checkbox"
                    className="toggle toggle-primary"
                    checked={(sectionsEnabled as any)[k] ?? true}
                    onChange={() =>
                      setConfigDeep({
                        sectionsEnabled: { ...(sectionsEnabled as any), [k]: !((sectionsEnabled as any)[k] ?? true) },
                      })
                    }
                  />
                  {k}
                </label>
              ))}
            </CardBody>
          </Card>

          <Card>
            <CardHeader className="flex items-center justify-between">
              <span className="font-semibold">Stats</span>
              <button
                className="btn btn-ghost btn-sm"
                onClick={() => setConfigDeep({ stats: [...stats, { value: '1', label: 'Nouvelle stat' }] })}
              >
                <span className="icon-[tabler--plus] size-4" />
                Ajouter
              </button>
            </CardHeader>
            <CardBody className="space-y-3">
              {stats.length === 0 ? (
                <div className="text-sm text-base-content/60">Aucune stat.</div>
              ) : (
                stats.map((s, i) => (
                  <div key={i} className="grid grid-cols-1 md:grid-cols-3 gap-2 items-center">
                    <input
                      className="input input-bordered input-sm"
                      value={s.value ?? ''}
                      placeholder="8+"
                      onChange={(e) => {
                        const next = [...stats]
                        next[i] = { ...next[i], value: e.target.value }
                        setConfigDeep({ stats: next })
                      }}
                    />
                    <input
                      className="input input-bordered input-sm md:col-span-2"
                      value={s.label ?? ''}
                      placeholder="Années d'expérience"
                      onChange={(e) => {
                        const next = [...stats]
                        next[i] = { ...next[i], label: e.target.value }
                        setConfigDeep({ stats: next })
                      }}
                    />
                    <div className="md:col-span-3 flex justify-end">
                      <button
                        className="btn btn-ghost btn-xs text-error"
                        onClick={() => {
                          const next = stats.filter((_, idx) => idx !== i)
                          setConfigDeep({ stats: next })
                        }}
                      >
                        Supprimer
                      </button>
                    </div>
                  </div>
                ))
              )}
            </CardBody>
          </Card>

          <Card>
            <CardHeader className="flex items-center justify-between">
              <span className="font-semibold">Quick actions</span>
              <button
                className="btn btn-ghost btn-sm"
                onClick={() =>
                  setConfigDeep({
                    quickActions: [
                      ...quickActions,
                      { id: `action_${quickActions.length + 1}`, label: 'Nouvelle action', icon: 'icon-[tabler--sparkles]', path: '/', description: '' },
                    ],
                  })
                }
              >
                <span className="icon-[tabler--plus] size-4" />
                Ajouter
              </button>
            </CardHeader>
            <CardBody className="space-y-4">
              {quickActions.length === 0 ? (
                <div className="text-sm text-base-content/60">Aucune action.</div>
              ) : (
                quickActions.map((a, i) => (
                  <div key={i} className="rounded-xl border border-base-300 p-3 space-y-2">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <span className={`${a.icon ?? 'icon-[tabler--sparkles]'} size-5 text-primary`} />
                        <span className="font-medium text-base-content">{a.label ?? '—'}</span>
                      </div>
                      <button
                        className="btn btn-ghost btn-xs text-error"
                        onClick={() => setConfigDeep({ quickActions: quickActions.filter((_, idx) => idx !== i) })}
                      >
                        Supprimer
                      </button>
                    </div>
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-2">
                      <input
                        className="input input-bordered input-sm"
                        value={a.id ?? ''}
                        placeholder="id"
                        onChange={(e) => {
                          const next = [...quickActions]
                          next[i] = { ...next[i], id: e.target.value }
                          setConfigDeep({ quickActions: next })
                        }}
                      />
                      <input
                        className="input input-bordered input-sm"
                        value={a.label ?? ''}
                        placeholder="Label"
                        onChange={(e) => {
                          const next = [...quickActions]
                          next[i] = { ...next[i], label: e.target.value }
                          setConfigDeep({ quickActions: next })
                        }}
                      />
                      <div className="flex items-center gap-2">
                        <button
                          className="btn btn-outline btn-sm"
                          onClick={() => {
                            setIconPickerTarget({ kind: 'quickAction', index: i })
                            setIconPickerOpen(true)
                          }}
                        >
                          Icône…
                        </button>
                        <input
                          className="input input-bordered input-sm w-full"
                          value={a.icon ?? ''}
                          placeholder="icon-[tabler--home]"
                          onChange={(e) => {
                            const next = [...quickActions]
                            next[i] = { ...next[i], icon: e.target.value }
                            setConfigDeep({ quickActions: next })
                          }}
                        />
                      </div>
                      <input
                        className="input input-bordered input-sm md:col-span-2"
                        value={a.path ?? ''}
                        placeholder="/"
                        onChange={(e) => {
                          const next = [...quickActions]
                          next[i] = { ...next[i], path: e.target.value }
                          setConfigDeep({ quickActions: next })
                        }}
                      />
                      <input
                        className="input input-bordered input-sm md:col-span-3"
                        value={a.description ?? ''}
                        placeholder="Description"
                        onChange={(e) => {
                          const next = [...quickActions]
                          next[i] = { ...next[i], description: e.target.value }
                          setConfigDeep({ quickActions: next })
                        }}
                      />
                    </div>
                  </div>
                ))
              )}
            </CardBody>
          </Card>

          <Card>
            <CardHeader className="flex items-center justify-between">
              <span className="font-semibold">FAQ</span>
              <button
                className="btn btn-ghost btn-sm"
                onClick={() => setConfigDeep({ faq: [...faq, { title: 'Nouvelle question', content: '', defaultOpen: false }] })}
              >
                <span className="icon-[tabler--plus] size-4" />
                Ajouter
              </button>
            </CardHeader>
            <CardBody className="space-y-3">
              {faq.length === 0 ? (
                <div className="text-sm text-base-content/60">Aucune entrée.</div>
              ) : (
                faq.map((f, i) => (
                  <div key={i} className="rounded-xl border border-base-300 p-3 space-y-2">
                    <div className="flex items-center justify-between">
                      <div className="font-medium">{f.title ?? '—'}</div>
                      <button className="btn btn-ghost btn-xs text-error" onClick={() => setConfigDeep({ faq: faq.filter((_, idx) => idx !== i) })}>
                        Supprimer
                      </button>
                    </div>
                    <input
                      className="input input-bordered input-sm w-full"
                      value={f.title ?? ''}
                      onChange={(e) => {
                        const next = [...faq]
                        next[i] = { ...next[i], title: e.target.value }
                        setConfigDeep({ faq: next })
                      }}
                    />
                    <textarea
                      className="textarea textarea-bordered w-full min-h-[90px]"
                      value={f.content ?? ''}
                      onChange={(e) => {
                        const next = [...faq]
                        next[i] = { ...next[i], content: e.target.value }
                        setConfigDeep({ faq: next })
                      }}
                    />
                    <label className="flex items-center gap-2 text-sm">
                      <input
                        type="checkbox"
                        className="checkbox checkbox-primary"
                        checked={!!f.defaultOpen}
                        onChange={() => {
                          const next = [...faq]
                          next[i] = { ...next[i], defaultOpen: !f.defaultOpen }
                          setConfigDeep({ faq: next })
                        }}
                      />
                      Ouvert par défaut
                    </label>
                  </div>
                ))
              )}
            </CardBody>
          </Card>

          <Card>
            <CardHeader className="flex items-center justify-between">
              <span className="font-semibold">Onboarding</span>
              <button
                className="btn btn-ghost btn-sm"
                onClick={() => setConfigDeep({ onboardingSteps: [...onboarding, { id: onboarding.length + 1, title: 'Nouveau step', description: '', icon: 'icon-[tabler--sparkles]' }] })}
              >
                <span className="icon-[tabler--plus] size-4" />
                Ajouter
              </button>
            </CardHeader>
            <CardBody className="space-y-3">
              {onboarding.length === 0 ? (
                <div className="text-sm text-base-content/60">Aucun step.</div>
              ) : (
                onboarding.map((s, i) => (
                  <div key={i} className="rounded-xl border border-base-300 p-3 space-y-2">
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <span className={`${s.icon ?? 'icon-[tabler--sparkles]'} size-5 text-primary`} />
                        <span className="font-medium">{s.title ?? '—'}</span>
                      </div>
                      <button className="btn btn-ghost btn-xs text-error" onClick={() => setConfigDeep({ onboardingSteps: onboarding.filter((_, idx) => idx !== i) })}>
                        Supprimer
                      </button>
                    </div>
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-2">
                      <input
                        className="input input-bordered input-sm"
                        value={String(s.id ?? '')}
                        placeholder="id"
                        onChange={(e) => {
                          const next = [...onboarding]
                          next[i] = { ...next[i], id: Number(e.target.value) || 0 }
                          setConfigDeep({ onboardingSteps: next })
                        }}
                      />
                      <input
                        className="input input-bordered input-sm md:col-span-2"
                        value={s.title ?? ''}
                        placeholder="Titre"
                        onChange={(e) => {
                          const next = [...onboarding]
                          next[i] = { ...next[i], title: e.target.value }
                          setConfigDeep({ onboardingSteps: next })
                        }}
                      />
                      <div className="flex items-center gap-2 md:col-span-3">
                        <button
                          className="btn btn-outline btn-sm"
                          onClick={() => {
                            setIconPickerTarget({ kind: 'onboarding', index: i })
                            setIconPickerOpen(true)
                          }}
                        >
                          Icône…
                        </button>
                        <input
                          className="input input-bordered input-sm w-full"
                          value={s.icon ?? ''}
                          placeholder="icon-[tabler--home]"
                          onChange={(e) => {
                            const next = [...onboarding]
                            next[i] = { ...next[i], icon: e.target.value }
                            setConfigDeep({ onboardingSteps: next })
                          }}
                        />
                      </div>
                      <textarea
                        className="textarea textarea-bordered w-full min-h-[84px] md:col-span-3"
                        value={s.description ?? ''}
                        placeholder="Description"
                        onChange={(e) => {
                          const next = [...onboarding]
                          next[i] = { ...next[i], description: e.target.value }
                          setConfigDeep({ onboardingSteps: next })
                        }}
                      />
                    </div>
                  </div>
                ))
              )}
            </CardBody>
          </Card>

          <IconPickerModal
            open={iconPickerOpen}
            value={iconPickerValue}
            onClose={() => {
              setIconPickerOpen(false)
              setIconPickerTarget(null)
            }}
            onSelect={(iconClass) => {
              if (!iconPickerTarget) return
              if (iconPickerTarget.kind === 'quickAction') {
                const next = [...quickActions]
                next[iconPickerTarget.index] = { ...next[iconPickerTarget.index], icon: iconClass }
                setConfigDeep({ quickActions: next })
              } else {
                const next = [...onboarding]
                next[iconPickerTarget.index] = { ...next[iconPickerTarget.index], icon: iconClass }
                setConfigDeep({ onboardingSteps: next })
              }
              setIconPickerOpen(false)
              setIconPickerTarget(null)
            }}
          />
        </div>
      ) : null}

      {activeTab === 'raw' ? (
        <div className="mt-6">
          <Card>
            <CardHeader className="flex items-center justify-between">
              <span className="font-semibold">JSON</span>
              <div className="flex gap-2">
                <button className="btn btn-ghost btn-sm" onClick={applyRawJson}>
                  Appliquer
                </button>
              </div>
            </CardHeader>
            <CardBody className="space-y-3">
              {rawJsonError ? (
                <div className="alert alert-error">
                  <span className="icon-[tabler--alert-triangle] size-5" />
                  <span>{rawJsonError}</span>
                </div>
              ) : null}
              <textarea
                className="textarea textarea-bordered w-full min-h-[420px] font-mono text-xs"
                value={rawJson}
                onChange={(e) => setRawJson(e.target.value)}
              />
              <div className="text-xs text-base-content/60">
                Conseil : utilise “Appliquer” puis “Enregistrer”.
              </div>
            </CardBody>
          </Card>
        </div>
      ) : null}

      {/* Debug row info */}
      {row ? (
        <div className="mt-6 text-xs text-base-content/50">
          Dernière mise à jour: {new Date(row.updated_at).toLocaleString('fr-FR')}
        </div>
      ) : null}
    </AdminLayout>
  )
}

