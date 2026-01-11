'use client'

import dynamic from 'next/dynamic'

const QuoteDetailsScreen = dynamic(
  () => import('@/features/commerce-devis/ui/screens/QuoteDetailsScreen').then((m) => ({ default: m.QuoteDetailsScreen })),
  { ssr: false }
)

export default function QuoteDetailsPage() {
  return <QuoteDetailsScreen />
}

