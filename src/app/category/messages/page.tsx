'use client'

import dynamic from 'next/dynamic'

const CategoryScreen = dynamic(
  () => import('@/features/category/ui/screens/CategoryScreen').then(mod => ({ default: mod.CategoryScreen })),
  { ssr: false }
)

export default function MessagesPage() {
  return (
    <CategoryScreen
      id="cat_4"
      name="Messages"
      iconClass="icon-[tabler--message]"
      description="Consultez et gérez vos conversations."
    />
  )
}
