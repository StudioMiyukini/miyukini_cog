'use client'

import { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react'
import { getSupabaseClient } from '@/lib/supabase/client'
import { getAllModules } from '@/framework/modules'
import type { ModuleManifest } from '@/framework/modules'
import { useAuth } from './AuthContext'

type EnabledModuleRow = {
  module_id: string
  enabled: boolean
  updated_at?: string
}

type ModulesContextValue = {
  isLoading: boolean
  allModules: readonly ModuleManifest[]
  enabledModuleIds: Set<string>
  isEnabled: (moduleId: string) => boolean
  refresh: () => Promise<void>
  /**
   * Persist en DB (réservé super_admin via RLS).
   */
  setEnabled: (moduleId: string, enabled: boolean) => Promise<void>
}

const ModulesContext = createContext<ModulesContextValue | null>(null)

export function ModulesProvider({ children }: { children: React.ReactNode }) {
  const supabase = getSupabaseClient()
  const { profile } = useAuth()
  const [isLoading, setIsLoading] = useState(true)
  const [enabledModuleIds, setEnabledModuleIds] = useState<Set<string>>(new Set())

  const allModules = useMemo(() => getAllModules(), [])

  const refresh = useCallback(async () => {
    setIsLoading(true)
    try {
      // Table attendue côté DB : `framework_modules` (module_id, enabled, updated_at)
      const { data, error } = await supabase.from('framework_modules').select('module_id,enabled,updated_at')
      if (error) {
        console.warn('[Modules] Impossible de charger framework_modules:', error)
        setEnabledModuleIds(new Set())
        return
      }
      const rows = (data ?? []) as EnabledModuleRow[]
      const next = new Set(rows.filter((r) => r.enabled).map((r) => r.module_id))
      setEnabledModuleIds(next)
    } finally {
      setIsLoading(false)
    }
  }, [supabase])

  useEffect(() => {
    refresh()
  }, [refresh])

  const isEnabled = useCallback((moduleId: string) => enabledModuleIds.has(moduleId), [enabledModuleIds])

  const setEnabled = useCallback(
    async (moduleId: string, enabled: boolean) => {
      // Garde UX : seul super_admin doit pouvoir (RLS fera foi de toute façon)
      if (profile?.role !== 'super_admin') {
        throw new Error('Action réservée au super_admin (masteradmin).')
      }

      const payload: EnabledModuleRow = {
        module_id: moduleId,
        enabled,
        updated_at: new Date().toISOString(),
      }

      const { error } = await supabase.from('framework_modules').upsert(payload, { onConflict: 'module_id' })
      if (error) throw error
      await refresh()
    },
    [profile?.role, refresh, supabase]
  )

  const value = useMemo<ModulesContextValue>(
    () => ({
      isLoading,
      allModules,
      enabledModuleIds,
      isEnabled,
      refresh,
      setEnabled,
    }),
    [allModules, enabledModuleIds, isEnabled, isLoading, refresh, setEnabled]
  )

  return <ModulesContext.Provider value={value}>{children}</ModulesContext.Provider>
}

export function useModules() {
  const ctx = useContext(ModulesContext)
  if (!ctx) throw new Error('useModules must be used within ModulesProvider')
  return ctx
}

