'use client'

/**
 * HeroModule - Section héro de la page d'accueil
 * 
 * @layer sections
 * @responsive Adaptatif mobile/desktop
 */

export interface HeroModuleProps {
  title?: string
  subtitle?: string
  ctaLabel?: string
  onCtaClick?: () => void
}

export function HeroModule({ 
  title = 'Bienvenue sur Miyukini',
  subtitle = 'Une interface modulaire pensée pour être portable sur mobile natif.',
  ctaLabel = 'Commencer',
  onCtaClick
}: HeroModuleProps) {
  return (
    <section className="card bg-base-200 border border-base-300">
      <div className="card-body">
        <h1 className="text-xl md:text-2xl font-bold text-base-content">
          {title}
        </h1>
        <p className="text-base-content/70">
          {subtitle}
        </p>
        <div className="card-actions mt-2">
          <button 
            className="btn btn-primary"
            onClick={onCtaClick}
          >
            {ctaLabel}
          </button>
        </div>
      </div>
    </section>
  )
}
