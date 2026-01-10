'use client'

import dynamic from 'next/dynamic'

const CategoryScreen = dynamic(
  () => import('@/features/category/ui/screens/CategoryScreen').then(mod => ({ default: mod.CategoryScreen })),
  { ssr: false }
)

export default function ExplorerPage() {
  return (
    <CategoryScreen
      id="cat_2"
      name="Explorer"
      iconClass="icon-[tabler--compass]"
      description="Découvrez du nouveau contenu et explorez les fonctionnalités."
    />
  )
}
