'use client'

import dynamic from 'next/dynamic'

const ProviderServicesScreen = dynamic(
  () => import('@/modules/booking/ui/screens/ProviderServicesScreen').then((m) => ({ default: m.ProviderServicesScreen })),
  { ssr: false }
)

export default function ProviderServicesPage() {
  return <ProviderServicesScreen />
}

