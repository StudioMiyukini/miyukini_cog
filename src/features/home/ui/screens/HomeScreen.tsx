'use client'

import { useCallback, useEffect, useMemo, useState } from 'react'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { BottomNav } from '@/components/navigation/BottomNav'
import { Accordion } from '@/components/molecules/accordion'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables } from '@/lib/supabase/database.types'

/**
 * HomeScreen - Page d'accueil
 * 
 * @layer screens (portable vers Android/iOS)
 * @contract ScreenContract { render, props, navigation }
 */

export interface HomeScreenProps {
  onNavigate?: (path: string) => void
}

type HomepageRow = Tables<'homepage_content'>

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
}

// Onboarding steps
const onboardingSteps = [
  {
    id: 1,
    title: 'Bienvenue sur Miyukini',
    description: 'Découvrez un framework modulaire conçu pour créer des applications web et mobiles rapidement.',
    icon: 'icon-[tabler--sparkles]',
  },
  {
    id: 2,
    title: 'Navigation intuitive',
    description: 'Utilisez la barre de navigation pour accéder à toutes les fonctionnalités en un clic.',
    icon: 'icon-[tabler--layout-navbar]',
  },
  {
    id: 3,
    title: 'Prêt pour le mobile',
    description: 'Chaque écran est pensé pour être portable vers Android et iOS nativement.',
    icon: 'icon-[tabler--device-mobile]',
  },
]

// FAQ items
const faqItems = [
  {
    title: 'Comment démarrer avec Miyukini ?',
    content: 'Créez un compte, explorez les modules depuis la page d\'accueil, et personnalisez votre profil dans les paramètres.',
    defaultOpen: true,
  },
  {
    title: 'Quelles sont les fonctionnalités principales ?',
    content: 'Miyukini offre un système d\'authentification, un back-office complet, des composants UI réutilisables et une architecture portable.',
  },
  {
    title: 'L\'application fonctionne-t-elle sur mobile ?',
    content: 'Oui ! Miyukini est conçu responsive et portable vers les applications natives Android et iOS grâce à son architecture "Screens".',
  },
]

// Quick Actions
const quickActions = [
  { id: 'content', label: 'Contenu', icon: 'icon-[tabler--article]', path: '/mockcontent', description: 'Explorez le contenu' },
  { id: 'profile', label: 'Profil', icon: 'icon-[tabler--user]', path: '/profile', description: 'Gérez votre compte' },
  { id: 'admin', label: 'Admin', icon: 'icon-[tabler--dashboard]', path: '/admin', description: 'Tableau de bord' },
]

// Stats
const stats = [
  { value: '8+', label: 'Années d\'expérience' },
  { value: '150+', label: 'Composants UI' },
  { value: '4.9', label: 'Note moyenne' },
  { value: '10K+', label: 'Utilisateurs satisfaits' },
]

function OnboardingModal({ 
  onComplete,
  onClose,
  steps,
}: { 
  onComplete: () => void
  onClose: () => void
  steps: Array<{ id: number; title: string; description: string; icon: string }>
}) {
  const [currentStep, setCurrentStep] = useState(0)
  const step = steps[currentStep]

  const handleNext = () => {
    if (currentStep < steps.length - 1) {
      setCurrentStep(currentStep + 1)
    } else {
      onComplete()
    }
  }

  // Fermer en cliquant sur le backdrop
  const handleBackdropClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      onClose()
    }
  }

  return (
    <div 
      className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
      onClick={handleBackdropClick}
    >
      <div className="card bg-base-100 shadow-2xl w-full max-w-md">
        <div className="card-body text-center">
          {/* Close button */}
          <button 
            className="btn btn-ghost btn-circle btn-sm absolute top-2 right-2"
            onClick={onClose}
          >
            <span className="icon-[tabler--x] size-5" />
          </button>

          {/* Progress */}
          <div className="flex justify-center gap-2 mb-6">
            {steps.map((_, index) => (
              <div
                key={index}
                className={`h-1.5 rounded-full transition-all duration-300 ${
                  index <= currentStep 
                    ? 'w-8 bg-primary' 
                    : 'w-2 bg-base-300'
                }`}
              />
            ))}
          </div>

          {/* Icon */}
          <div className="mx-auto mb-4">
            <div className="bg-primary/20 text-primary rounded-full size-20 flex items-center justify-center">
              <span className={`${step.icon} size-10`} />
            </div>
          </div>

          {/* Content */}
          <h2 className="text-2xl font-bold text-base-content">{step.title}</h2>
          <p className="text-base-content/70 mt-2">{step.description}</p>

          {/* Actions */}
          <div className="flex gap-3 mt-8">
            <button className="btn btn-ghost flex-1" onClick={onClose}>
              Passer
            </button>
            <button className="btn btn-primary flex-1" onClick={handleNext}>
              {currentStep < onboardingSteps.length - 1 ? (
                <>
                  Suivant
                  <span className="icon-[tabler--arrow-right] size-5" />
                </>
              ) : (
                <>
                  Commencer
                  <span className="icon-[tabler--check] size-5" />
                </>
              )}
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}

export function HomeScreen({ onNavigate }: HomeScreenProps) {
  const [showOnboarding, setShowOnboarding] = useState(false)
  const supabase = useMemo(() => getSupabaseClient(), [])
  const [dbConfig, setDbConfig] = useState<HomepageConfig | null>(null)

  const navigate = useCallback(
    (path: string) => {
      if (onNavigate) return onNavigate(path)
      if (typeof window !== 'undefined') window.location.href = path
    },
    [onNavigate]
  )

  useEffect(() => {
    let cancelled = false
    ;(async () => {
      try {
        const { data, error } = await supabase.from('homepage_content').select('*').eq('id', 'home').maybeSingle()
        if (error) throw error
        const row = data as HomepageRow | null
        const cfg = (row?.config ?? null) as unknown as HomepageConfig | null
        if (!cancelled) setDbConfig(cfg)
      } catch {
        if (!cancelled) setDbConfig(null)
      }
    })()
    return () => {
      cancelled = true
    }
  }, [supabase])

  const hero = dbConfig?.hero
  const enabled = dbConfig?.sectionsEnabled ?? {}
  const pageStats =
    (dbConfig?.stats ?? []).map((s) => ({ value: s.value ?? '', label: s.label ?? '' })) || []
  const pageQuickActions =
    (dbConfig?.quickActions ?? []).map((a) => ({
      id: a.id ?? '',
      label: a.label ?? '',
      icon: a.icon ?? 'icon-[tabler--sparkles]',
      path: a.path ?? '/',
      description: a.description ?? '',
    })) || []
  const pageFaqItems =
    (dbConfig?.faq ?? []).map((f) => ({
      title: f.title ?? '',
      content: f.content ?? '',
      defaultOpen: !!f.defaultOpen,
    })) || []
  const pageOnboardingSteps =
    (dbConfig?.onboardingSteps ?? []).map((s, idx) => ({
      id: typeof s.id === 'number' ? s.id : idx + 1,
      title: s.title ?? '—',
      description: s.description ?? '',
      icon: s.icon ?? 'icon-[tabler--sparkles]',
    })) || []

  const handleOnboardingComplete = () => {
    setShowOnboarding(false)
  }

  return (
    <>
      {showOnboarding && (
        <OnboardingModal 
          onComplete={handleOnboardingComplete} 
          onClose={() => setShowOnboarding(false)}
          steps={pageOnboardingSteps.length ? pageOnboardingSteps : onboardingSteps}
        />
      )}
      
      <AppShellScreen>
        <main className="bg-base-100">
          {/* Hero Section */}
          <section className="pt-24 pb-12 sm:pt-32 sm:pb-16 lg:pb-24">
            <div className="mx-auto max-w-screen-xl px-4 sm:px-6 lg:px-8">
              <div className="flex flex-col items-center gap-6 text-center">
                {/* Badge */}
                <div className="bg-base-200 border border-base-content/10 w-fit rounded-full px-4 py-1.5">
                  <span className="text-sm text-base-content/80">
                    {hero?.badgeText ?? '🚀 Framework modulaire depuis 2016'}
                  </span>
                </div>

                {/* Title */}
                <h1 className="text-3xl sm:text-4xl lg:text-5xl font-bold text-base-content leading-tight max-w-3xl">
                  {hero?.title?.line1 ?? 'Créez. Déployez.'}
                  <br />
                  <span className="text-primary">{hero?.title?.line2 ?? 'Impressionnez.'}</span>
                </h1>

                {/* Subtitle */}
                <p className="text-base-content/70 text-lg max-w-2xl">
                  {hero?.subtitle ??
                    'Miyukini est un framework modulaire pensé pour créer des applications web et mobiles modernes, performantes et portables.'}
                </p>

                {/* CTA */}
                <div className="flex gap-4 flex-wrap justify-center">
                  <button 
                    className="btn btn-primary btn-lg"
                    onClick={() => navigate(hero?.primaryCta?.href ?? '/mockcontent')}
                  >
                    {hero?.primaryCta?.label ?? 'Explorer les fonctionnalités'}
                    <span className="icon-[tabler--arrow-right] size-5" />
                  </button>
                  <button 
                    className="btn btn-outline btn-lg"
                    onClick={() => navigate(hero?.secondaryCta?.href ?? '/admin')}
                  >
                    {hero?.secondaryCta?.label ?? 'Voir le dashboard'}
                  </button>
                </div>
              </div>
            </div>
          </section>

          {/* Stats Section */}
          {enabled.stats !== false && (
            <section className="py-12 bg-base-200">
            <div className="mx-auto max-w-screen-xl px-4 sm:px-6 lg:px-8">
              <div className="grid grid-cols-2 lg:grid-cols-4 gap-6">
                {(pageStats.length ? pageStats : stats).map((stat) => (
                  <div key={stat.label} className="text-center">
                    <p className="text-3xl lg:text-4xl font-bold text-primary">{stat.value}</p>
                    <p className="text-base-content/70 mt-1">{stat.label}</p>
                  </div>
                ))}
              </div>
            </div>
          </section>
          )}

          {/* Services / Quick Actions */}
          {enabled.quickActions !== false && (
            <section className="py-12 sm:py-16 lg:py-24">
            <div className="mx-auto max-w-screen-xl px-4 sm:px-6 lg:px-8">
              {/* Section Header */}
              <div className="text-center mb-12">
                <h2 className="text-2xl sm:text-3xl font-semibold text-base-content mb-4">
                  Tout ce dont vous avez besoin
                </h2>
                <p className="text-base-content/70 text-lg max-w-2xl mx-auto">
                  Des composants UI aux écrans complets, Miyukini vous accompagne dans la création de votre application.
                </p>
              </div>

              {/* Cards Grid */}
              <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                {(pageQuickActions.length ? pageQuickActions : quickActions).map((action) => (
                  <button
                    key={action.id}
                    className="card bg-base-100 border border-base-300 hover:border-primary transition-colors text-left"
                    onClick={() => navigate(action.path)}
                  >
                    <div className="card-body">
                      <div className="bg-primary/10 text-primary rounded-lg size-12 flex items-center justify-center mb-4">
                        <span className={`${action.icon ?? 'icon-[tabler--sparkles]'} size-6`} />
                      </div>
                      <h3 className="card-title text-lg text-base-content">{action.label}</h3>
                      <p className="text-base-content/70">{action.description}</p>
                      <div className="card-actions mt-4">
                        <span className="text-primary font-medium flex items-center gap-2">
                          Découvrir
                          <span className="icon-[tabler--arrow-right] size-4" />
                        </span>
                      </div>
                    </div>
                  </button>
                ))}
              </div>
            </div>
          </section>
          )}

          {/* Activity Section */}
          <section className="py-12 bg-base-200">
            <div className="mx-auto max-w-screen-xl px-4 sm:px-6 lg:px-8">
              <div className="flex items-center justify-between mb-6">
                <h2 className="text-xl font-semibold text-base-content">Activité récente</h2>
                <span className="badge badge-info">3 nouvelles</span>
              </div>
              <div className="card bg-base-100 border border-base-300">
                <div className="card-body p-0 divide-y divide-base-200">
                  {[
                    { text: 'Nouveau contenu disponible', time: 'Il y a 5 min', icon: 'icon-[tabler--article]' },
                    { text: 'Mise à jour du système', time: 'Il y a 1h', icon: 'icon-[tabler--refresh]' },
                    { text: 'Nouvelle fonctionnalité ajoutée', time: 'Il y a 2h', icon: 'icon-[tabler--sparkles]' },
                  ].map((item, index) => (
                    <div key={index} className="flex items-center gap-4 p-4 hover:bg-base-200 transition-colors cursor-pointer">
                      <div className="bg-primary/10 text-primary rounded-lg size-10 flex items-center justify-center">
                        <span className={`${item.icon} size-5`} />
                      </div>
                      <div className="flex-1">
                        <p className="font-medium text-base-content">{item.text}</p>
                        <p className="text-sm text-base-content/60">{item.time}</p>
                      </div>
                      <span className="icon-[tabler--chevron-right] size-5 text-base-content/30" />
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </section>

          {/* FAQ Section */}
          {enabled.faq !== false && (
            <section className="py-12 sm:py-16 lg:py-24">
            <div className="mx-auto max-w-screen-xl px-4 sm:px-6 lg:px-8">
              <div className="text-center mb-12">
                <h2 className="text-2xl sm:text-3xl font-semibold text-base-content mb-4">
                  Questions fréquentes
                </h2>
                <p className="text-base-content/70 text-lg">
                  Tout ce que vous devez savoir pour bien démarrer
                </p>
              </div>
              <div className="max-w-2xl mx-auto">
                <Accordion items={pageFaqItems.length ? pageFaqItems : faqItems} />
              </div>
            </div>
          </section>
          )}

          {/* CTA Section */}
          <section className="py-12 sm:py-16">
            <div className="mx-auto max-w-screen-xl px-4 sm:px-6 lg:px-8">
              <div className="bg-gradient-to-r from-primary/10 to-secondary/10 rounded-2xl p-8 lg:p-12 text-center">
                <h2 className="text-2xl sm:text-3xl font-bold text-base-content mb-4">
                  Prêt à commencer ?
                </h2>
                <p className="text-base-content/70 mb-6 max-w-xl mx-auto">
                  Rejoignez des milliers de développeurs qui utilisent Miyukini pour créer des applications exceptionnelles.
                </p>
                <div className="flex gap-4 justify-center flex-wrap">
                  <button 
                    className="btn btn-primary"
                    onClick={() => navigate('/signup')}
                  >
                    Créer un compte
                    <span className="icon-[tabler--arrow-right] size-5" />
                  </button>
                  <button 
                    className="btn btn-ghost"
                    onClick={() => setShowOnboarding(true)}
                  >
                    <span className="icon-[tabler--help] size-5" />
                    Voir le tutoriel
                  </button>
                </div>
              </div>
            </div>
          </section>

          {/* Footer spacer for BottomNav */}
          <div className="h-20" />
        </main>
        <BottomNav />
      </AppShellScreen>
    </>
  )
}
