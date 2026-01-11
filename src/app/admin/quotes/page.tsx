'use client'

import dynamic from 'next/dynamic'

const AdminQuotesScreen = dynamic(
  () => import('@/features/commerce-devis/ui/screens/AdminQuotesScreen').then((m) => ({ default: m.AdminQuotesScreen })),
  { ssr: false }
)

export default function AdminQuotesPage() {
  return <AdminQuotesScreen />
}

