'use client'

import { getSupabaseClient } from '@/lib/supabase/client'
import type {
  BudgetCategory,
  BudgetEntry,
  BudgetPlan,
  BudgetPlanLine,
} from '../../domain/types'
import type { BudgetEntriesPort, BudgetEntryFilters } from '../../ports/BudgetEntriesPort'

const supabase = getSupabaseClient()

function applyFilters(query: ReturnType<typeof supabase.from>, filters?: BudgetEntryFilters) {
  const builder = query
  if (!filters) return builder
  if (filters.workspaceId) builder.eq('workspace_id', filters.workspaceId)
  if (filters.moduleId) builder.eq('module_id', filters.moduleId)
  if (filters.type) builder.eq('type', filters.type)
  if (filters.status) builder.eq('status', filters.status)
  if (filters.periodStart) builder.gte('entry_date', filters.periodStart)
  if (filters.periodEnd) builder.lte('entry_date', filters.periodEnd)
  return builder
}

export function createSupabaseBudgetEntriesPort(): BudgetEntriesPort {
  return {
    async listEntries(filters) {
      const builder = applyFilters(supabase.from('budget_entries').select('*'), filters)
      const { data } = await builder.order('entry_date', { ascending: false }).limit(200)
      return (data ?? []) as BudgetEntry[]
    },
    async createEntry(input) {
      const { data, error } = await supabase.from('budget_entries').insert(input).select('*').single()
      if (error) {
        console.error('budget create error', error)
        return null
      }
      return data as BudgetEntry
    },
    async updateEntry(id, patch) {
      const { data, error } = await supabase.from('budget_entries').update(patch).eq('id', id).select('*').single()
      if (error) {
        console.error('budget update error', error)
        return null
      }
      return data as BudgetEntry
    },
    async deleteEntry(id) {
      const { error } = await supabase.from('budget_entries').delete().eq('id', id)
      return !error
    },
    async listCategories(workspaceId) {
      const builder = workspaceId ? supabase.from('budget_categories').select('*').eq('workspace_id', workspaceId) : supabase.from('budget_categories').select('*')
      const { data } = await builder.order('name')
      return (data ?? []) as BudgetCategory[]
    },
    async createCategory(input) {
      const { data, error } = await supabase.from('budget_categories').insert(input).select('*').single()
      if (error) {
        console.error('budget category create error', error)
        return null
      }
      return data as BudgetCategory
    },
    async listPlans(workspaceId) {
      const builder = workspaceId ? supabase.from('budget_plans').select('*').eq('workspace_id', workspaceId) : supabase.from('budget_plans').select('*')
      const { data } = await builder.order('start_date', { ascending: false })
      return (data ?? []) as BudgetPlan[]
    },
    async createPlan(input) {
      const { data, error } = await supabase.from('budget_plans').insert(input).select('*').single()
      if (error) {
        console.error('budget plan create error', error)
        return null
      }
      return data as BudgetPlan
    },
    async listPlanLines(planId) {
      const { data } = await supabase.from('budget_plan_lines').select('*').eq('plan_id', planId)
      return (data ?? []) as BudgetPlanLine[]
    },
    async createPlanLine(input) {
      const { data, error } = await supabase.from('budget_plan_lines').insert(input).select('*').single()
      if (error) {
        console.error('budget plan line create error', error)
        return null
      }
      return data as BudgetPlanLine
    },
  }
}
