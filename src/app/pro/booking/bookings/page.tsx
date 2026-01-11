'use client'

import dynamic from 'next/dynamic'

const ProviderBookingsScreen = dynamic(
  () => import('@/modules/booking/ui/screens/ProviderBookingsScreen').then((m) => ({ default: m.ProviderBookingsScreen })),
  { ssr: false }
)

export default function ProviderBookingsPage() {
  return <ProviderBookingsScreen />
}

