'use client'

import dynamic from 'next/dynamic'

const BudgetAdminScreen = dynamic(
  () => import('@/features/budget/ui/screens/BudgetAdminScreen').then((mod) => ({ default: mod.BudgetAdminScreen })),
  { ssr: false }
)

export default function AdminBudgetPage() {
  return <BudgetAdminScreen />
}
