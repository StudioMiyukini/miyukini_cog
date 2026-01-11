'use client'

export type BudgetEntryType = 'income' | 'expense' | 'refund' | 'transfer'
export type BudgetEntryStatus = 'draft' | 'validated' | 'reconciled' | 'archived'
export type BudgetPeriodType = 'month' | 'quarter' | 'year'
export type BudgetCategoryType = 'income' | 'expense' | 'both'

export interface BudgetCategory {
  id: string
  workspace_id: string
  name: string
  type: BudgetCategoryType
  parent_id?: string | null
  icon?: string | null
  color?: string | null
  is_active: boolean
  account_code?: string | null
  created_at: string
  updated_at: string
}

export interface BudgetEntry {
  id: string
  workspace_id: string
  module_id: string
  context_id?: string | null
  type: BudgetEntryType
  status: BudgetEntryStatus
  entry_date: string
  description?: string | null
  amount_net: string
  amount_tax: string | null
  amount_gross: string
  currency: string
  fx_rate: string | null
  category_id?: string | null
  cost_center_id?: string | null
  tags?: string[] | null
  notes?: string | null
  receipt_path?: string | null
  invoice_id?: string | null
  payment_id?: string | null
  dedupe_key?: string | null
  created_by?: string | null
  updated_by?: string | null
  created_at: string
  updated_at: string
}

export interface BudgetPlan {
  id: string
  workspace_id: string
  module_id?: string | null
  context_id?: string | null
  period_type: BudgetPeriodType
  start_date: string
  end_date: string
  currency: string
  created_by?: string | null
  created_at: string
  updated_at: string
}

export interface BudgetPlanLine {
  id: string
  plan_id: string
  category_id?: string | null
  period_start: string
  amount_planned: string
  created_at: string
  updated_at: string
}

export interface BudgetEntryAudit {
  id: string
  entry_id: string
  action: string
  actor_id?: string | null
  payload?: Record<string, unknown> | null
  created_at: string
}

