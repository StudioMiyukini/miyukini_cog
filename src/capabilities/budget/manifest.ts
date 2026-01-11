'use client'

import type { BudgetEntry, BudgetCategory, BudgetPlan } from './domain/types'

export const budgetCapabilityManifest = {
  id: 'budget',
  name: 'Budget',
  description: 'Centralise les entrées recettes/dépenses, plans et rapports budgétaires.',
  dataContracts: ['public.budget_entries', 'public.budget_categories', 'public.budget_plans', 'public.budget_plan_lines'],
  exports: {
    ports: ['BudgetEntriesPort'],
  },
  policies: ['budget_entries_admin', 'budget_categories_admin'],
  events: {
    emitted: ['budget.entry.created', 'budget.entry.validated', 'budget.plan.updated'],
    consumed: ['billing.invoice.paid', 'billing.refund.created'],
  },
} as const

export type BudgetCapabilityManifest = typeof budgetCapabilityManifest
