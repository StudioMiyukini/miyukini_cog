'use client'

import type {
  BudgetEntry,
  BudgetCategory,
  BudgetPlan,
  BudgetPlanLine,
} from '../domain/types'

export type BudgetEntryFilters = {
  workspaceId?: string
  moduleId?: string
  type?: BudgetEntry['type']
  status?: BudgetEntry['status']
  periodStart?: string
  periodEnd?: string
}

export interface BudgetEntriesPort {
  listEntries(filters?: BudgetEntryFilters): Promise<BudgetEntry[]>
  createEntry(input: Partial<BudgetEntry> & { workspace_id: string; module_id: string; amount_gross: string; amount_net: string }): Promise<BudgetEntry | null>
  updateEntry(id: string, patch: Partial<BudgetEntry>): Promise<BudgetEntry | null>
  deleteEntry(id: string): Promise<boolean>
  listCategories(workspaceId?: string): Promise<BudgetCategory[]>
  createCategory(input: Partial<BudgetCategory> & { workspace_id: string; name: string; type: BudgetCategory['type'] }): Promise<BudgetCategory | null>
  listPlans(workspaceId?: string): Promise<BudgetPlan[]>
  createPlan(input: Partial<BudgetPlan> & { workspace_id: string; period_type: BudgetPlan['period_type']; start_date: string; end_date: string }): Promise<BudgetPlan | null>
  listPlanLines(planId: string): Promise<BudgetPlanLine[]>
  createPlanLine(input: Partial<BudgetPlanLine> & { plan_id: string; period_start: string; amount_planned: string }): Promise<BudgetPlanLine | null>
}
