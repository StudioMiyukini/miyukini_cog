import { useEffect, useMemo } from 'react'
import { useRouter } from 'next/navigation'
import { useAuth } from '@/contexts/AuthContext'

/**
 * useRequireSuperAdmin - garde d'accès client-side réservé au masteradmin (super_admin).
 */
export function useRequireSuperAdmin() {
  const router = useRouter()
  const { isAuthenticated, isLoading: authLoading, profile } = useAuth()

  const isSuperAdmin = useMemo(() => profile?.role === 'super_admin', [profile?.role])

  useEffect(() => {
    if (authLoading) return
    if (!isAuthenticated) {
      router.push('/signin')
      return
    }
    if (isAuthenticated && !isSuperAdmin) {
      router.push('/admin')
    }
  }, [authLoading, isAuthenticated, isSuperAdmin, router])

  return {
    authLoading,
    isAuthenticated,
    isSuperAdmin,
    isReady: !authLoading && isAuthenticated && isSuperAdmin,
  }
}

