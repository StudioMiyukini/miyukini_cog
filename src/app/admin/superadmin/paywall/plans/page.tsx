'use client'

import dynamic from 'next/dynamic'

const SuperAdminPaywallPlansScreen = dynamic(
  () =>
    import('@/features/superadmin/ui/screens/SuperAdminPaywallPlansScreen').then((m) => ({
      default: m.SuperAdminPaywallPlansScreen,
    })),
  { ssr: false },
)

export default function SuperAdminPaywallPlansPage() {
  return <SuperAdminPaywallPlansScreen />
}
