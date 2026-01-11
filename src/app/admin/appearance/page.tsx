'use client'

import dynamic from 'next/dynamic'

const AdminAppearanceScreen = dynamic(
  () =>
    import('@/features/backoffice/ui/screens/AdminAppearanceScreen').then((m) => ({ default: m.AdminAppearanceScreen })),
  { ssr: false }
)

export default function AdminAppearancePage() {
  return <AdminAppearanceScreen />
}

