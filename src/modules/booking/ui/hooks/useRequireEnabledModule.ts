'use client'

import { useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { useModules } from '@/contexts/ModulesContext'

export function useRequireEnabledModule(moduleId: string, redirectTo: string = '/') {
  const router = useRouter()
  const { isLoading, isEnabled } = useModules()

  const enabled = isEnabled(moduleId)

  useEffect(() => {
    if (isLoading) return
    if (!enabled) router.push(redirectTo)
  }, [enabled, isLoading, redirectTo, router])

  return { isLoading, enabled, isReady: !isLoading && enabled }
}

