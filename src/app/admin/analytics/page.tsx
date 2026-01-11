'use client'

import dynamic from 'next/dynamic'

const AdminAnalyticsScreen = dynamic(
  () =>
    import('@/features/backoffice/ui/screens/AdminAnalyticsScreen').then((m) => ({ default: m.AdminAnalyticsScreen })),
  { ssr: false }
)

export default function AdminAnalyticsPage() {
  return <AdminAnalyticsScreen />
}

