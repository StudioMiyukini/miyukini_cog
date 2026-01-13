'use client'

import dynamic from 'next/dynamic'

const BookingProviderPlanningScreen = dynamic(
  () =>
    import('@/modules/booking/ui/screens/BookingProviderPlanningScreen').then((m) => ({
      default: m.BookingProviderPlanningScreen,
    })),
  { ssr: false }
)

export default function BookingProviderPlanningPage() {
  return <BookingProviderPlanningScreen />
}

