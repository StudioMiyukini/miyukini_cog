'use client'

import { createContext, useContext, useState, useEffect, ReactNode } from 'react'

/**
 * CategoriesContext - Gestion des 8 catégories de navigation
 * Persisté en localStorage (remplaçable par Supabase/API)
 * 
 * @layer contexts
 */

export interface Category {
  id: string
  name: string
  iconClass: string
  path: string
  enabled: boolean
  order: number
}

// 8 catégories par défaut - seule "Accueil" est activée
const DEFAULT_CATEGORIES: Category[] = [
  { id: 'cat_1', name: 'Accueil', iconClass: 'icon-[tabler--home]', path: '/', enabled: true, order: 1 },
  { id: 'cat_2', name: 'Explorer', iconClass: 'icon-[tabler--compass]', path: '/category/explorer', enabled: false, order: 2 },
  { id: 'cat_3', name: 'Favoris', iconClass: 'icon-[tabler--heart]', path: '/category/favoris', enabled: false, order: 3 },
  { id: 'cat_4', name: 'Messages', iconClass: 'icon-[tabler--message]', path: '/category/messages', enabled: false, order: 4 },
  { id: 'cat_5', name: 'Notifications', iconClass: 'icon-[tabler--bell]', path: '/category/notifications', enabled: false, order: 5 },
  { id: 'cat_6', name: 'Recherche', iconClass: 'icon-[tabler--search]', path: '/category/recherche', enabled: false, order: 6 },
  { id: 'cat_7', name: 'Paramètres', iconClass: 'icon-[tabler--settings]', path: '/category/parametres', enabled: false, order: 7 },
  { id: 'cat_8', name: 'Aide', iconClass: 'icon-[tabler--help]', path: '/category/aide', enabled: false, order: 8 },
]

const STORAGE_KEY = 'miyukini_categories'

interface CategoriesContextType {
  categories: Category[]
  enabledCategories: Category[]
  toggleCategory: (id: string) => void
  updateCategory: (id: string, updates: Partial<Category>) => void
  resetCategories: () => void
}

const CategoriesContext = createContext<CategoriesContextType | undefined>(undefined)

export function CategoriesProvider({ children }: { children: ReactNode }) {
  const [categories, setCategories] = useState<Category[]>(DEFAULT_CATEGORIES)
  const [isLoaded, setIsLoaded] = useState(false)

  // Charger depuis localStorage au mount
  useEffect(() => {
    if (typeof window !== 'undefined') {
      const stored = localStorage.getItem(STORAGE_KEY)
      if (stored) {
        try {
          const parsed = JSON.parse(stored)
          // Fusionner avec les defaults pour gérer les nouvelles catégories
          const merged = DEFAULT_CATEGORIES.map(defaultCat => {
            const storedCat = parsed.find((c: Category) => c.id === defaultCat.id)
            return storedCat ? { ...defaultCat, ...storedCat } : defaultCat
          })
          setCategories(merged)
        } catch {
          setCategories(DEFAULT_CATEGORIES)
        }
      }
      setIsLoaded(true)
    }
  }, [])

  // Sauvegarder dans localStorage à chaque changement
  useEffect(() => {
    if (isLoaded && typeof window !== 'undefined') {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(categories))
    }
  }, [categories, isLoaded])

  const toggleCategory = (id: string) => {
    setCategories(prev => prev.map(cat =>
      cat.id === id ? { ...cat, enabled: !cat.enabled } : cat
    ))
  }

  const updateCategory = (id: string, updates: Partial<Category>) => {
    setCategories(prev => prev.map(cat =>
      cat.id === id ? { ...cat, ...updates } : cat
    ))
  }

  const resetCategories = () => {
    setCategories(DEFAULT_CATEGORIES)
    if (typeof window !== 'undefined') {
      localStorage.removeItem(STORAGE_KEY)
    }
  }

  const enabledCategories = categories
    .filter(cat => cat.enabled)
    .sort((a, b) => a.order - b.order)

  return (
    <CategoriesContext.Provider value={{
      categories,
      enabledCategories,
      toggleCategory,
      updateCategory,
      resetCategories,
    }}>
      {children}
    </CategoriesContext.Provider>
  )
}

export function useCategories() {
  const context = useContext(CategoriesContext)
  if (!context) {
    throw new Error('useCategories must be used within a CategoriesProvider')
  }
  return context
}
