'use client'

import dynamic from 'next/dynamic'

const BookingProvidersDirectoryScreen = dynamic(
  () =>
    import('@/modules/booking/ui/screens/BookingProvidersDirectoryScreen').then((m) => ({
      default: m.BookingProvidersDirectoryScreen,
    })),
  { ssr: false }
)

export default function BookingPage() {
  return <BookingProvidersDirectoryScreen />
}

