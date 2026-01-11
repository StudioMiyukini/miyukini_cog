'use client'

import dynamic from 'next/dynamic'

const AdminSettingsScreen = dynamic(
  () => import('@/features/backoffice/ui/screens/AdminSettingsScreen').then((m) => ({ default: m.AdminSettingsScreen })),
  { ssr: false }
)

export default function AdminSettingsPage() {
  return <AdminSettingsScreen />
}

