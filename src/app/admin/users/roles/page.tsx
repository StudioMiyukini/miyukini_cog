'use client'

import dynamic from 'next/dynamic'

const AdminUserRolesScreen = dynamic(
  () =>
    import('@/features/backoffice/ui/screens/AdminUserRolesScreen').then((m) => ({ default: m.AdminUserRolesScreen })),
  { ssr: false }
)

export default function AdminUserRolesPage() {
  return <AdminUserRolesScreen />
}

