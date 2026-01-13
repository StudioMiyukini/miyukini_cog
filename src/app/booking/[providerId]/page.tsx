'use client'

import dynamic from 'next/dynamic'

const BookingProviderPublicScreen = dynamic(
  () => import('@/modules/booking/ui/screens/BookingProviderPublicScreen').then((m) => ({ default: m.BookingProviderPublicScreen })),
  { ssr: false }
)

export default function BookingProviderPage() {
  return <BookingProviderPublicScreen />
}

