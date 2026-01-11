'use client'

import dynamic from 'next/dynamic'

const ProviderPlanningScreen = dynamic(
  () => import('@/modules/booking/ui/screens/ProviderPlanningScreen').then((m) => ({ default: m.ProviderPlanningScreen })),
  { ssr: false }
)

export default function ProviderPlanningPage() {
  return <ProviderPlanningScreen />
}

