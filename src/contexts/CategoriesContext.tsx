'use client'

import { createContext, useContext, useState, useEffect, useCallback, ReactNode } from 'react'
import { getSupabaseClient } from '@/lib/supabase/client'
import { useAuth } from './AuthContext'
import type { 
  Category as DBCategory, 
  UserCategoryPreference 
} from '@/lib/supabase/database.types'

/**
 * CategoriesContext - Gestion des 8 catégories de navigation
 * Persisté en base de données Supabase (avec fallback localStorage pour les non-connectés)
 * 
 * @layer contexts
 * @dependencies Supabase
 */

export interface Category {
  id: string
  name: string
  iconClass: string
  path: string
  visible: boolean
  enabled: boolean
  order: number
}

// 8 catégories par défaut - seule "Accueil" est activée
const DEFAULT_CATEGORIES: Category[] = [
  { id: 'cat_1', name: 'Accueil', iconClass: 'icon-[tabler--home]', path: '/', visible: true, enabled: true, order: 1 },
  { id: 'cat_2', name: 'Explorer', iconClass: 'icon-[tabler--compass]', path: '/category/explorer', visible: true, enabled: false, order: 2 },
  { id: 'cat_3', name: 'Favoris', iconClass: 'icon-[tabler--heart]', path: '/category/favoris', visible: true, enabled: false, order: 3 },
  { id: 'cat_4', name: 'Messages', iconClass: 'icon-[tabler--message]', path: '/category/messages', visible: true, enabled: false, order: 4 },
  { id: 'cat_5', name: 'Notifications', iconClass: 'icon-[tabler--bell]', path: '/category/notifications', visible: true, enabled: false, order: 5 },
  { id: 'cat_6', name: 'Recherche', iconClass: 'icon-[tabler--search]', path: '/category/recherche', visible: true, enabled: false, order: 6 },
  { id: 'cat_7', name: 'Paramètres', iconClass: 'icon-[tabler--settings]', path: '/category/parametres', visible: true, enabled: false, order: 7 },
  { id: 'cat_8', name: 'Aide', iconClass: 'icon-[tabler--help]', path: '/category/aide', visible: true, enabled: false, order: 8 },
]

const STORAGE_KEY = 'miyukini_categories'

interface CategoriesContextType {
  categories: Category[]
  enabledCategories: Category[]
  isLoading: boolean
  toggleCategory: (id: string) => void
  updateCategory: (id: string, updates: Partial<Category>) => void
  updateCategoryDefinition: (
    id: string,
    updates: { name?: string; iconClass?: string; path?: string; visible?: boolean }
  ) => Promise<void>
  resetCategories: () => void
  refreshCategories: () => Promise<void>
}

const CategoriesContext = createContext<CategoriesContextType | undefined>(undefined)

export function CategoriesProvider({ children }: { children: ReactNode }) {
  const [categories, setCategories] = useState<Category[]>(DEFAULT_CATEGORIES)
  const [isLoading, setIsLoading] = useState(true)
  const { user, isAuthenticated, isLoading: authLoading } = useAuth()
  
  const supabase = getSupabaseClient()

  // Charger les catégories depuis Supabase
  const loadCategoriesFromDB = useCallback(async () => {
    if (!user) return null

    try {
      // Charger les catégories globales
      const { data: dbCategories, error: catError } = await supabase
        .from('categories')
        .select('*')
        .order('sort_order')

      if (catError) {
        console.error('Erreur chargement catégories:', catError)
        return null
      }

      // Charger les préférences utilisateur
      const { data: userPrefs, error: prefsError } = await supabase
        .from('user_category_preferences')
        .select('*')
        .eq('user_id', user.id)

      if (prefsError) {
        console.error('Erreur chargement préférences:', prefsError)
      }

      // Fusionner catégories et préférences
      const mergedCategories: Category[] = dbCategories.map((cat: DBCategory) => {
        const pref = userPrefs?.find((p: UserCategoryPreference) => p.category_id === cat.id)
        return {
          id: cat.id,
          name: cat.name,
          iconClass: cat.icon_class,
          path: cat.path,
          visible: (cat as any).is_visible ?? true,
          enabled: pref ? pref.enabled : (cat.is_default ?? false),
          order: pref?.custom_order ?? cat.sort_order,
        }
      })

      return mergedCategories
    } catch (error) {
      console.error('Erreur loadCategoriesFromDB:', error)
      return null
    }
  }, [user, supabase])

  // Sauvegarder une préférence dans Supabase
  const savePreferenceToDB = useCallback(async (categoryId: string, enabled: boolean, order?: number) => {
    if (!user) return false

    try {
      const { error } = await supabase
        .from('user_category_preferences')
        .upsert({
          user_id: user.id,
          category_id: categoryId,
          enabled,
          custom_order: order,
          updated_at: new Date().toISOString(),
        }, {
          onConflict: 'user_id,category_id',
        })

      if (error) {
        console.error('Erreur sauvegarde préférence:', error)
        return false
      }
      return true
    } catch (error) {
      console.error('Erreur savePreferenceToDB:', error)
      return false
    }
  }, [user, supabase])

  // Charger depuis localStorage (fallback pour non-connectés)
  const loadFromLocalStorage = useCallback(() => {
    if (typeof window === 'undefined') return DEFAULT_CATEGORIES

    const stored = localStorage.getItem(STORAGE_KEY)
    if (stored) {
      try {
        const parsed = JSON.parse(stored)
        return DEFAULT_CATEGORIES.map(defaultCat => {
          const storedCat = parsed.find((c: Category) => c.id === defaultCat.id)
          return storedCat ? { ...defaultCat, ...storedCat } : defaultCat
        })
      } catch {
        return DEFAULT_CATEGORIES
      }
    }
    return DEFAULT_CATEGORIES
  }, [])

  // Sauvegarder dans localStorage (fallback)
  const saveToLocalStorage = useCallback((cats: Category[]) => {
    if (typeof window !== 'undefined') {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(cats))
    }
  }, [])

  // Rafraîchir les catégories
  const refreshCategories = useCallback(async () => {
    setIsLoading(true)
    try {
      if (isAuthenticated && user) {
        const dbCategories = await loadCategoriesFromDB()
        if (dbCategories) {
          setCategories(dbCategories)
        }
      } else {
        setCategories(loadFromLocalStorage())
      }
    } finally {
      setIsLoading(false)
    }
  }, [isAuthenticated, user, loadCategoriesFromDB, loadFromLocalStorage])

  // Charger au mount et quand l'auth change
  useEffect(() => {
    if (!authLoading) {
      refreshCategories()
    }
  }, [authLoading, isAuthenticated, user?.id, refreshCategories])

  // Toggle catégorie
  const toggleCategory = useCallback(async (id: string) => {
    const category = categories.find(c => c.id === id)
    if (!category) return

    const newEnabled = !category.enabled

    // Mise à jour optimiste
    setCategories(prev => prev.map(cat =>
      cat.id === id ? { ...cat, enabled: newEnabled } : cat
    ))

    // Sauvegarder
    if (isAuthenticated && user) {
      const success = await savePreferenceToDB(id, newEnabled, category.order)
      if (!success) {
        // Rollback si erreur
        setCategories(prev => prev.map(cat =>
          cat.id === id ? { ...cat, enabled: !newEnabled } : cat
        ))
      }
    } else {
      // localStorage pour non-connectés
      const updated = categories.map(cat =>
        cat.id === id ? { ...cat, enabled: newEnabled } : cat
      )
      saveToLocalStorage(updated)
    }
  }, [categories, isAuthenticated, user, savePreferenceToDB, saveToLocalStorage])

  // Mettre à jour une catégorie
  const updateCategory = useCallback(async (id: string, updates: Partial<Category>) => {
    const category = categories.find(c => c.id === id)
    if (!category) return

    // Mise à jour optimiste
    setCategories(prev => prev.map(cat =>
      cat.id === id ? { ...cat, ...updates } : cat
    ))

    // Sauvegarder
    if (isAuthenticated && user) {
      await savePreferenceToDB(
        id,
        updates.enabled ?? category.enabled,
        updates.order ?? category.order
      )
    } else {
      const updated = categories.map(cat =>
        cat.id === id ? { ...cat, ...updates } : cat
      )
      saveToLocalStorage(updated)
    }
  }, [categories, isAuthenticated, user, savePreferenceToDB, saveToLocalStorage])

  // Mettre à jour la définition globale d'une catégorie (path + visibilité globale)
  const updateCategoryDefinition = useCallback(
    async (id: string, updates: { name?: string; iconClass?: string; path?: string; visible?: boolean }) => {
      const before = categories
      const next = categories.map((cat) => {
        if (cat.id !== id) return cat
        return {
          ...cat,
          ...(updates.name !== undefined ? { name: updates.name } : null),
          ...(updates.iconClass !== undefined ? { iconClass: updates.iconClass } : null),
          ...(updates.path !== undefined ? { path: updates.path } : null),
          ...(updates.visible !== undefined ? { visible: updates.visible } : null),
        }
      })

      // Mise à jour optimiste
      setCategories(next)

      if (isAuthenticated && user) {
        try {
          const payload: Record<string, any> = {}
          if (updates.name !== undefined) payload.name = updates.name
          if (updates.iconClass !== undefined) payload.icon_class = updates.iconClass
          if (updates.path !== undefined) payload.path = updates.path
          if (updates.visible !== undefined) payload.is_visible = updates.visible
          payload.updated_at = new Date().toISOString()

          const { error } = await supabase.from('categories').update(payload).eq('id', id)
          if (error) throw error
        } catch (error) {
          console.error('Erreur updateCategoryDefinition:', error)
          setCategories(before)
        }
      } else {
        // fallback localStorage (mode non connecté)
        saveToLocalStorage(next)
      }
    },
    [categories, isAuthenticated, saveToLocalStorage, supabase, user]
  )

  // Reset catégories
  const resetCategories = useCallback(async () => {
    if (isAuthenticated && user) {
      // Supprimer toutes les préférences utilisateur
      try {
        await supabase
          .from('user_category_preferences')
          .delete()
          .eq('user_id', user.id)
      } catch (error) {
        console.error('Erreur reset préférences:', error)
      }
    } else {
      localStorage.removeItem(STORAGE_KEY)
    }
    
    setCategories(DEFAULT_CATEGORIES)
  }, [isAuthenticated, user, supabase])

  const enabledCategories = categories
    .filter(cat => cat.visible && cat.enabled)
    .sort((a, b) => a.order - b.order)

  return (
    <CategoriesContext.Provider value={{
      categories,
      enabledCategories,
      isLoading,
      toggleCategory,
      updateCategory,
      updateCategoryDefinition,
      resetCategories,
      refreshCategories,
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
