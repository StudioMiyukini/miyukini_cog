'use client'

import dynamic from 'next/dynamic'

const PaywallScreen = dynamic(
  () => import('@/features/paywall/ui/screens/PaywallScreen').then((m) => ({ default: m.PaywallScreen })),
  { ssr: false },
)

export default function PricingPage() {
  return <PaywallScreen />
}
