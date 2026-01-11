'use client'

import dynamic from 'next/dynamic'

const ProviderTimeOffScreen = dynamic(
  () => import('@/modules/booking/ui/screens/ProviderTimeOffScreen').then((m) => ({ default: m.ProviderTimeOffScreen })),
  { ssr: false }
)

export default function ProviderTimeOffPage() {
  return <ProviderTimeOffScreen />
}

