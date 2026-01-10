'use client'

import dynamic from 'next/dynamic'

// Import dynamique pour éviter les erreurs d'hydratation
const SignUpScreen = dynamic(
  () => import('@/features/auth/ui/screens/SignUpScreen').then(mod => ({ default: mod.SignUpScreen })),
  { ssr: false }
)

/**
 * Page d'inscription
 * @route /signup
 */
export default function SignUpPage() {
  return <SignUpScreen />
}
