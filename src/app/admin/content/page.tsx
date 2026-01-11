'use client'

import dynamic from 'next/dynamic'

const AdminContentScreen = dynamic(
  () => import('@/features/backoffice/ui/screens/AdminContentScreen').then((m) => ({ default: m.AdminContentScreen })),
  { ssr: false }
)

export default function AdminContentPage() {
  return <AdminContentScreen />
}

