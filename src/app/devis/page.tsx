'use client'

import dynamic from 'next/dynamic'

const QuoteRequestScreen = dynamic(
  () => import('@/features/commerce-devis/ui/screens/QuoteRequestScreen').then((m) => ({ default: m.QuoteRequestScreen })),
  { ssr: false }
)

export default function DevisPage() {
  return <QuoteRequestScreen />
}

