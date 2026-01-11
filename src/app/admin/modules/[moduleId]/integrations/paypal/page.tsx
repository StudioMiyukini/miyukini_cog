'use client'

import dynamic from 'next/dynamic'

const SuperAdminModulePaypalScreen = dynamic(
  () =>
    import('@/features/superadmin/ui/screens/SuperAdminModulePaypalScreen').then((m) => ({
      default: m.SuperAdminModulePaypalScreen,
    })),
  { ssr: false },
)

export default function AdminModulePaypalSettingsPage() {
  return <SuperAdminModulePaypalScreen />
}

