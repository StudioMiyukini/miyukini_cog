'use client'

import dynamic from 'next/dynamic'

// Import dynamique pour éviter les erreurs d'hydratation
const BackOfficeScreen = dynamic(
  () => import('@/features/backoffice/ui/screens/BackOfficeScreen').then(mod => ({ default: mod.BackOfficeScreen })),
  { ssr: false }
)

/**
 * Page du dashboard d'administration
 * @route /admin
 */
export default function AdminPage() {
  return <BackOfficeScreen />
}
