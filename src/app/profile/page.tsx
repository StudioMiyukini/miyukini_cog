'use client'

import dynamic from 'next/dynamic'

// Import dynamique pour éviter les erreurs d'hydratation
const ProfileScreen = dynamic(
  () => import('@/features/profile/ui/screens/ProfileScreen').then(mod => ({ default: mod.ProfileScreen })),
  { ssr: false }
)

/**
 * Page de profil utilisateur
 * @route /profile
 */
export default function ProfilePage() {
  return <ProfileScreen />
}
