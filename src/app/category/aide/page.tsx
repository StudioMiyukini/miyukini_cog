'use client'

import dynamic from 'next/dynamic'

const CategoryScreen = dynamic(
  () => import('@/features/category/ui/screens/CategoryScreen').then(mod => ({ default: mod.CategoryScreen })),
  { ssr: false }
)

export default function AidePage() {
  return (
    <CategoryScreen
      id="cat_8"
      name="Aide"
      iconClass="icon-[tabler--help]"
      description="Besoin d'assistance ? Consultez notre centre d'aide."
    />
  )
}
