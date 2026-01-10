'use client'

import dynamic from 'next/dynamic'

const CategoryScreen = dynamic(
  () => import('@/features/category/ui/screens/CategoryScreen').then(mod => ({ default: mod.CategoryScreen })),
  { ssr: false }
)

export default function RecherchePage() {
  return (
    <CategoryScreen
      id="cat_6"
      name="Recherche"
      iconClass="icon-[tabler--search]"
      description="Trouvez rapidement ce que vous cherchez."
    />
  )
}
