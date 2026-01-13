'use client'

import { useEffect } from 'react'
import { useRouter } from 'next/navigation'

/**
 * Redirection depuis l'ancienne route /admin/superadmin/pricing
 * vers la nouvelle route /admin/superadmin/paywall/plans
 */
export default function PricingRedirectPage() {
  const router = useRouter()

  useEffect(() => {
    router.replace('/admin/superadmin/paywall/plans')
  }, [router])

  return (
    <div className="min-h-screen bg-base-200 flex items-center justify-center">
      <span className="loading loading-spinner loading-lg text-primary" />
    </div>
  )
}
