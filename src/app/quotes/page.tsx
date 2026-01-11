'use client'

import dynamic from 'next/dynamic'

const MyQuotesScreen = dynamic(
  () => import('@/features/commerce-devis/ui/screens/MyQuotesScreen').then((m) => ({ default: m.MyQuotesScreen })),
  { ssr: false }
)

export default function QuotesPage() {
  return <MyQuotesScreen />
}

