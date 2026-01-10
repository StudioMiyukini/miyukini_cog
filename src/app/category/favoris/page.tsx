'use client'

import dynamic from 'next/dynamic'

const CategoryScreen = dynamic(
  () => import('@/features/category/ui/screens/CategoryScreen').then(mod => ({ default: mod.CategoryScreen })),
  { ssr: false }
)

export default function FavorisPage() {
  return (
    <CategoryScreen
      id="cat_3"
      name="Favoris"
      iconClass="icon-[tabler--heart]"
      description="Retrouvez tous vos éléments favoris en un seul endroit."
    />
  )
}
