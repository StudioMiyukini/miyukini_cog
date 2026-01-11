'use client'

import dynamic from 'next/dynamic'

const AdminModulesScreen = dynamic(
  () => import('@/features/backoffice/ui/screens/AdminModulesScreen').then((m) => ({ default: m.AdminModulesScreen })),
  { ssr: false }
)

export default function AdminModulesPage() {
  return <AdminModulesScreen />
}

