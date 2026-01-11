'use client'

import dynamic from 'next/dynamic'

const AdminHomepageEditorScreen = dynamic(
  () =>
    import('@/features/backoffice/ui/screens/AdminHomepageEditorScreen').then((m) => ({
      default: m.AdminHomepageEditorScreen,
    })),
  { ssr: false }
)

export default function AdminHomepageEditorPage() {
  return <AdminHomepageEditorScreen />
}

