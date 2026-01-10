'use client'

import { useRouter, usePathname } from 'next/navigation'
import { useCategories } from '@/contexts/CategoriesContext'

/**
 * BottomNav - Navigation inférieure avec catégories dynamiques
 * Affiche uniquement les catégories activées depuis le BackOffice
 * 
 * @layer organisms
 */

export function BottomNav() {
  const router = useRouter()
  const pathname = usePathname()
  const { enabledCategories } = useCategories()

  // Si aucune catégorie activée, ne rien afficher
  if (enabledCategories.length === 0) {
    return null
  }

  return (
    <nav 
      className="fixed bottom-0 left-0 right-0 z-50 bg-base-100/95 backdrop-blur-sm border-t border-base-300" 
      role="navigation" 
      aria-label="Navigation principale"
    >
      <div className="flex items-center justify-around py-2 pb-[calc(0.5rem+env(safe-area-inset-bottom,0px))]">
        {enabledCategories.map((category) => {
          const isActive = pathname === category.path || 
            (category.path !== '/' && pathname.startsWith(category.path))
          
          return (
            <button
              key={category.id}
              className={`relative flex flex-col items-center justify-center min-w-[64px] min-h-[48px] px-2 py-1 rounded-lg transition-colors
                ${isActive 
                  ? 'text-primary font-medium' 
                  : 'text-base-content/60 hover:text-base-content hover:bg-base-200'
                }`}
              onClick={() => router.push(category.path)}
              aria-current={isActive ? 'page' : undefined}
            >
              {/* Indicateur actif */}
              {isActive && (
                <div className="absolute top-0 left-1/2 -translate-x-1/2 w-8 h-0.5 bg-primary rounded-full" />
              )}
              <span className={`${category.iconClass} size-6`} />
              <span className="text-xs mt-1 truncate max-w-[60px]">{category.name}</span>
            </button>
          )
        })}
      </div>
    </nav>
  )
}
