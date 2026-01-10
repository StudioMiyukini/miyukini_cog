'use client'

import dynamic from 'next/dynamic'

// Import dynamique pour éviter les erreurs d'hydratation
const ContentScreen = dynamic(
  () => import('@/features/content/ui/screens/ContentScreen').then(mod => ({ default: mod.ContentScreen })),
  { ssr: false }
)

/**
 * Page de contenu mock
 * @route /mockcontent
 */
export default function MockContentPage() {
  return <ContentScreen />
}
