'use client'

import dynamic from 'next/dynamic'

const AdminSettingsNotificationsScreen = dynamic(
  () =>
    import('@/features/backoffice/ui/screens/AdminSettingsNotificationsScreen').then((m) => ({
      default: m.AdminSettingsNotificationsScreen,
    })),
  { ssr: false }
)

export default function AdminSettingsNotificationsPage() {
  return <AdminSettingsNotificationsScreen />
}

