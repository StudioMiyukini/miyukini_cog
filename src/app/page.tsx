'use client'

import dynamic from 'next/dynamic'
import { useRouter } from 'next/navigation'

const HomeScreen = dynamic(
  () => import('@/features/home/ui/screens/HomeScreen').then(mod => ({ default: mod.HomeScreen })),
  { ssr: false }
)

/**
 * Page d'accueil avec onboarding
 * @route /
 */
export default function HomePage() {
  const router = useRouter()

  const handleNavigate = (path: string) => {
    router.push(path)
  }

  return <HomeScreen onNavigate={handleNavigate} />
}
