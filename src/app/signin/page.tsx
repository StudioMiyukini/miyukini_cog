'use client'

import dynamic from 'next/dynamic'

// Import dynamique pour éviter les erreurs d'hydratation
const SignInScreen = dynamic(
  () => import('@/features/auth/ui/screens/SignInScreen').then(mod => ({ default: mod.SignInScreen })),
  { ssr: false }
)

/**
 * Page de connexion
 * @route /signin
 */
export default function SignInPage() {
  return <SignInScreen />
}
