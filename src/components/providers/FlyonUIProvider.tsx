'use client'

import { useEffect, useCallback } from 'react'
import { usePathname } from 'next/navigation'

/**
 * FlyonUIProvider - Initialise FlyonUI JS pour les composants interactifs
 * 
 * @description
 * Ce composant charge et initialise FlyonUI JavaScript côté client.
 * Réinitialise à chaque changement de route pour éviter les overlays fantômes.
 */
export function FlyonUIProvider({ children }: { children: React.ReactNode }) {
  const pathname = usePathname()

  // Nettoyer uniquement les backdrops d'overlay (pas le header/bottom nav!)
  const cleanupOverlays = useCallback(() => {
    if (typeof window === 'undefined') return

    // Cibler UNIQUEMENT les vrais backdrops FlyonUI
    const flyonuiBackdrops = document.querySelectorAll(
      '.hs-overlay-backdrop, [data-hs-overlay-backdrop]'
    )
    flyonuiBackdrops.forEach(el => el.remove())
    
    // Fermer les overlays FlyonUI
    document.querySelectorAll('.hs-overlay.open, .hs-overlay.opened').forEach(el => {
      el.classList.remove('open', 'opened')
    })
    
    // Fermer les modals/drawers
    document.querySelectorAll('.modal.modal-open, .drawer.drawer-open').forEach(el => {
      el.classList.remove('modal-open', 'drawer-open')
    })
    
    // Restaurer le body (uniquement les classes liées aux overlays)
    document.body.style.overflow = ''
    document.body.style.paddingRight = ''
    document.body.classList.remove(
      'overflow-hidden', 
      'modal-open', 
      'hs-overlay-body-open'
    )
    
    // Restaurer le html
    document.documentElement.style.overflow = ''
  }, [])

  // Initialiser FlyonUI
  const initFlyonUI = useCallback(async () => {
    try {
      await import('flyonui/flyonui')
      
      if (typeof window !== 'undefined' && (window as any).HSStaticMethods) {
        ;(window as any).HSStaticMethods.autoInit()
      }
    } catch (error) {
      // Silencieux en prod
    }
  }, [])

  // Nettoyer au changement de route
  useEffect(() => {
    cleanupOverlays()
    
    // Petit délai pour s'assurer que le DOM est prêt
    const timer = setTimeout(() => {
      initFlyonUI()
    }, 50)

    return () => {
      clearTimeout(timer)
      cleanupOverlays()
    }
  }, [pathname, cleanupOverlays, initFlyonUI])

  // Nettoyer au démontage du composant
  useEffect(() => {
    return () => {
      cleanupOverlays()
    }
  }, [cleanupOverlays])

  // Écouter l'événement popstate (bouton retour)
  useEffect(() => {
    const handlePopState = () => {
      cleanupOverlays()
    }

    window.addEventListener('popstate', handlePopState)
    return () => {
      window.removeEventListener('popstate', handlePopState)
    }
  }, [cleanupOverlays])

  return <>{children}</>
}
