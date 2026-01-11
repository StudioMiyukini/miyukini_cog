'use client'

import dynamic from 'next/dynamic'

const AdminCategoriesScreen = dynamic(
  () =>
    import('@/features/backoffice/ui/screens/AdminCategoriesScreen').then((m) => ({ default: m.AdminCategoriesScreen })),
  { ssr: false }
)

export default function AdminCategoriesPage() {
  return <AdminCategoriesScreen />
}

