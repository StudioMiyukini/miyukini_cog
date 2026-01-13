'use client'

import dynamic from 'next/dynamic'

const ProviderProfileScreen = dynamic(
  () => import('@/modules/booking/ui/screens/ProviderProfileScreen').then((m) => ({ default: m.ProviderProfileScreen })),
  { ssr: false }
)

export default function ProviderProfilePage() {
  return <ProviderProfileScreen />
}

