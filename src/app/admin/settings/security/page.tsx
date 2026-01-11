'use client'

import dynamic from 'next/dynamic'

const AdminSettingsSecurityScreen = dynamic(
  () =>
    import('@/features/backoffice/ui/screens/AdminSettingsSecurityScreen').then((m) => ({
      default: m.AdminSettingsSecurityScreen,
    })),
  { ssr: false }
)

export default function AdminSettingsSecurityPage() {
  return <AdminSettingsSecurityScreen />
}

