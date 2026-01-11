'use client'

import dynamic from 'next/dynamic'

const AdminUsersScreen = dynamic(
  () => import('@/features/backoffice/ui/screens/AdminUsersScreen').then((m) => ({ default: m.AdminUsersScreen })),
  { ssr: false }
)

export default function AdminUsersPage() {
  return <AdminUsersScreen />
}

