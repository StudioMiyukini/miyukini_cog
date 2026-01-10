'use client'

import dynamic from 'next/dynamic'

const CategoryScreen = dynamic(
  () => import('@/features/category/ui/screens/CategoryScreen').then(mod => ({ default: mod.CategoryScreen })),
  { ssr: false }
)

export default function NotificationsPage() {
  return (
    <CategoryScreen
      id="cat_5"
      name="Notifications"
      iconClass="icon-[tabler--bell]"
      description="Restez informé de toutes les nouveautés et alertes."
    />
  )
}
