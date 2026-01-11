'use client'

import dynamic from 'next/dynamic'

const BookingClientScreen = dynamic(
  () => import('@/modules/booking/ui/screens/BookingClientScreen').then((m) => ({ default: m.BookingClientScreen })),
  { ssr: false }
)

export default function BookingPage() {
  return <BookingClientScreen />
}

