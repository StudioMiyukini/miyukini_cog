'use client'

import dynamic from 'next/dynamic'

const CategoryScreen = dynamic(
  () => import('@/features/category/ui/screens/CategoryScreen').then(mod => ({ default: mod.CategoryScreen })),
  { ssr: false }
)

export default function ParametresPage() {
  return (
    <CategoryScreen
      id="cat_7"
      name="Paramètres"
      iconClass="icon-[tabler--settings]"
      description="Personnalisez votre expérience et vos préférences."
    />
  )
}
