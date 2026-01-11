'use client'

import dynamic from 'next/dynamic'

const AdminHelpScreen = dynamic(
  () => import('@/features/backoffice/ui/screens/AdminHelpScreen').then((m) => ({ default: m.AdminHelpScreen })),
  { ssr: false }
)

export default function AdminHelpPage() {
  return <AdminHelpScreen />
}

