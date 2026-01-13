'use client'

import dynamic from 'next/dynamic'

const ProfileEditScreen = dynamic(
  () => import('@/features/profile/ui/screens/ProfileEditScreen').then((m) => ({ default: m.ProfileEditScreen })),
  { ssr: false },
)

export default function ProfileEditPage() {
  return <ProfileEditScreen />
}

