'use client'

import dynamic from 'next/dynamic'

const AdminDocsScreen = dynamic(
  () => import('@/features/backoffice/ui/screens/AdminDocsScreen').then((m) => ({ default: m.AdminDocsScreen })),
  { ssr: false }
)

export default function AdminDocsPage() {
  return <AdminDocsScreen />
}

