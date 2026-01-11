'use client'

import dynamic from 'next/dynamic'

const SuperAdminPaypalSettingsScreen = dynamic(
  () =>
    import('@/features/superadmin/ui/screens/SuperAdminPaypalSettingsScreen').then((m) => ({
      default: m.SuperAdminPaypalSettingsScreen,
    })),
  { ssr: false },
)

export default function SuperAdminPaypalSettingsPage() {
  return <SuperAdminPaypalSettingsScreen />
}

