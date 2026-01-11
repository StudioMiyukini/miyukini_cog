import { useEffect, useMemo } from 'react'
import { useRouter } from 'next/navigation'
import { useAuth } from '@/contexts/AuthContext'

/**
 * useRequireAdmin - garde d'accès client-side pour le back-office.
 * - Non connecté -> redirige vers /signin
 * - Connecté mais non admin -> redirige vers /
 */
export function useRequireAdmin() {
  const router = useRouter()
  const { isAuthenticated, isLoading: authLoading, profile } = useAuth()

  const isAdmin = useMemo(
    () => profile?.role === 'admin' || profile?.role === 'super_admin',
    [profile?.role]
  )

  useEffect(() => {
    if (authLoading) return
    if (!isAuthenticated) {
      router.push('/signin')
      return
    }
    if (isAuthenticated && !isAdmin) {
      router.push('/')
    }
  }, [authLoading, isAuthenticated, isAdmin, router])

  return {
    authLoading,
    isAuthenticated,
    isAdmin,
    /**
     * Ready quand on a fini le chargement auth et que l'utilisateur est admin.
     * (Sinon, la redirection prend le relais.)
     */
    isReady: !authLoading && isAuthenticated && isAdmin,
  }
}

