'use client'

import dynamic from 'next/dynamic'

const BudgetOverviewScreen = dynamic(
  () => import('@/features/budget/ui/screens/BudgetOverviewScreen').then((mod) => ({ default: mod.BudgetOverviewScreen })),
  { ssr: false }
)

export default function BudgetPage() {
  return <BudgetOverviewScreen />
}
