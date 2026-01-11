'use client'

import dynamic from 'next/dynamic'

const ProviderBookingDashboardScreen = dynamic(
  () =>
    import('@/modules/booking/ui/screens/ProviderBookingDashboardScreen').then((m) => ({
      default: m.ProviderBookingDashboardScreen,
    })),
  { ssr: false }
)

export default function ProviderBookingPage() {
  return <ProviderBookingDashboardScreen />
}

