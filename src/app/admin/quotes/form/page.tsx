'use client'

import dynamic from 'next/dynamic'

const AdminQuoteRequestFormBuilderScreen = dynamic(
  () =>
    import('@/features/commerce-devis/ui/screens/AdminQuoteRequestFormBuilderScreen').then((m) => ({
      default: m.AdminQuoteRequestFormBuilderScreen,
    })),
  { ssr: false },
)

export default function AdminQuoteRequestFormBuilderPage() {
  return <AdminQuoteRequestFormBuilderScreen />
}

