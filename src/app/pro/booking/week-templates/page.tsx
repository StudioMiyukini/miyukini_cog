'use client'

import dynamic from 'next/dynamic'

const ProviderWeekTemplatesScreen = dynamic(
  () =>
    import('@/modules/booking/ui/screens/ProviderWeekTemplatesScreen').then((m) => ({
      default: m.ProviderWeekTemplatesScreen,
    })),
  { ssr: false }
)

export default function ProviderWeekTemplatesPage() {
  return <ProviderWeekTemplatesScreen />
}

