'use client'

import { useEffect, useId } from 'react'

// Types pour l'Accordion
export interface AccordionItem {
  id?: string
  title: string
  content: string | React.ReactNode
  defaultOpen?: boolean
}

export interface AccordionProps {
  items: AccordionItem[]
  className?: string
  /** Permet d'ouvrir plusieurs items à la fois */
  allowMultiple?: boolean
}

/**
 * Accordion - Composant FlyonUI intégré
 * 
 * @layer molecules
 * @tokens theme.colors.neutral, theme.colors.base-content
 * @dependencies FlyonUI JS
 * 
 * @example
 * ```tsx
 * <Accordion items={[
 *   { title: 'Question 1', content: 'Réponse 1', defaultOpen: true },
 *   { title: 'Question 2', content: 'Réponse 2' },
 * ]} />
 * ```
 */
export function Accordion({ items, className = '', allowMultiple = false }: AccordionProps) {
  const baseId = useId()

  // Initialiser FlyonUI JS au montage du composant
  useEffect(() => {
    const initFlyonUI = async () => {
      try {
        // FlyonUI s'initialise automatiquement via le plugin
        // mais on peut forcer le refresh si nécessaire
        if (typeof window !== 'undefined' && (window as any).HSAccordion) {
          ;(window as any).HSAccordion.autoInit()
        }
      } catch (error) {
        console.warn('FlyonUI accordion init:', error)
      }
    }
    initFlyonUI()
  }, [items])

  return (
    <div 
      className={`accordion divide-neutral/20 divide-y ${className}`}
      data-accordion={allowMultiple ? 'multiple' : 'single'}
    >
      {items.map((item, index) => {
        const itemId = item.id || `${baseId}-item-${index}`
        const collapseId = `${itemId}-collapse`
        const isActive = item.defaultOpen

        return (
          <div 
            key={itemId}
            className={`accordion-item ${isActive ? 'active' : ''}`}
            id={itemId}
          >
            <button
              className="accordion-toggle inline-flex items-center gap-x-4 text-start w-full py-4 px-5"
              aria-controls={collapseId}
              aria-expanded={isActive}
            >
              <span className="icon-[tabler--plus] accordion-item-active:hidden text-base-content size-4.5 block shrink-0" />
              <span className="icon-[tabler--minus] accordion-item-active:block text-base-content size-4.5 hidden shrink-0" />
              <span className="text-base-content font-medium">{item.title}</span>
            </button>
            <div
              id={collapseId}
              className={`accordion-content w-full overflow-hidden transition-[height] duration-300 ${!isActive ? 'hidden' : ''}`}
              aria-labelledby={itemId}
              role="region"
            >
              <div className="px-5 pb-4">
                {typeof item.content === 'string' ? (
                  <p className="text-base-content/80 font-normal">{item.content}</p>
                ) : (
                  item.content
                )}
              </div>
            </div>
          </div>
        )
      })}
    </div>
  )
}
