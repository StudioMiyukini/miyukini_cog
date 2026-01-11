'use client'

import dynamic from 'next/dynamic'

const SuperAdminSmtpSettingsScreen = dynamic(
  () =>
    import('@/features/superadmin/ui/screens/SuperAdminSmtpSettingsScreen').then((m) => ({
      default: m.SuperAdminSmtpSettingsScreen,
    })),
  { ssr: false },
)

export default function SuperAdminSmtpSettingsPage() {
  return <SuperAdminSmtpSettingsScreen />
}

