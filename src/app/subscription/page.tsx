'use client'

import dynamic from 'next/dynamic'

const UserSubscriptionScreen = dynamic(
  () =>
    import('@/features/paywall/ui/screens/UserSubscriptionScreen').then((m) => ({
      default: m.UserSubscriptionScreen,
    })),
  { ssr: false },
)

export default function SubscriptionPage() {
  return <UserSubscriptionScreen />
}
