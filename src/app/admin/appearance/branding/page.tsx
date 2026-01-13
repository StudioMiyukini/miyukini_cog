'use client'

import dynamic from 'next/dynamic'

const SuperAdminBrandingScreen = dynamic(
  () =>
    import('@/features/superadmin/ui/screens/SuperAdminBrandingScreen').then((m) => ({
      default: m.SuperAdminBrandingScreen,
    })),
  { ssr: false }
)

export default function AdminBrandingPage() {
  return <SuperAdminBrandingScreen />
}
