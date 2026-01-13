'use client'

import dynamic from 'next/dynamic'

const SuperAdminManualPaymentsScreen = dynamic(
  () =>
    import('@/features/superadmin/ui/screens/SuperAdminManualPaymentsScreen').then((m) => ({
      default: m.SuperAdminManualPaymentsScreen,
    })),
  { ssr: false }
)

export default function SuperAdminManualPaymentsPage() {
  return <SuperAdminManualPaymentsScreen />
}
