import { getSupabaseClient } from '@/lib/supabase/client'
import { appEventBus } from '@/framework/events'
import type { ProfileTypePort } from '../../ports/ProfileTypePort'
import type {
  CreateProfileTypeInput,
  CreateProfileTypeRuleInput,
  ProfileType,
  ProfileTypeRule,
  UpdateProfileTypeInput,
  UpdateProfileTypeRuleInput,
} from '../../domain/types'

type AnyClient = {
  from: (table: string) => any
}

function mapProfileType(row: any): ProfileType {
  return {
    id: row.id,
    code: row.code,
    name: row.name,
    description: row.description ?? null,
    defaultVisibility: row.default_visibility ?? 'private',
    allowedRoles: row.allowed_roles ?? null,
    defaultBlocks: row.default_blocks ?? null,
    createdBy: row.created_by ?? null,
    createdAt: row.created_at,
    updatedAt: row.updated_at,
  }
}

function mapProfileTypeRule(row: any): ProfileTypeRule {
  return {
    id: row.id,
    profileTypeId: row.profile_type_id,
    priority: row.priority,
    matchRole: row.match_role ?? null,
    matchTagsAll: row.match_tags_all ?? null,
    matchTagsAny: row.match_tags_any ?? null,
    matchContextType: row.match_context_type ?? null,
    matchContextId: row.match_context_id ?? null,
    isActive: row.is_active ?? true,
  }
}

export function createSupabaseProfileTypePort(): ProfileTypePort {
  const supabase = getSupabaseClient() as unknown as AnyClient

  return {
    async listProfileTypes() {
      const { data, error } = await supabase.from('profile_types').select('*').order('created_at', { ascending: false })
      if (error) throw error
      return (data ?? []).map(mapProfileType)
    },

    async getProfileTypeById(id) {
      const { data, error } = await supabase.from('profile_types').select('*').eq('id', id).maybeSingle()
      if (error) throw error
      return data ? mapProfileType(data) : null
    },

    async getProfileTypeByCode(code) {
      const { data, error } = await supabase.from('profile_types').select('*').eq('code', code).maybeSingle()
      if (error) throw error
      return data ? mapProfileType(data) : null
    },

    async createProfileType(input: CreateProfileTypeInput) {
      const payload = {
        id: crypto.randomUUID(),
        code: input.code,
        name: input.name,
        description: input.description ?? null,
        default_visibility: input.defaultVisibility ?? 'private',
        allowed_roles: input.allowedRoles ?? null,
        default_blocks: input.defaultBlocks ?? null,
      }
      const { data, error } = await supabase.from('profile_types').insert(payload).select('*').single()
      if (error) throw error

      appEventBus.publish({
        type: 'profile.type.created',
        payload: { profileTypeId: data.id },
        occurredAt: new Date().toISOString(),
      })

      return mapProfileType(data)
    },

    async updateProfileType(id, patch: UpdateProfileTypeInput) {
      const payload: Record<string, unknown> = {}
      if (patch.name !== undefined) payload.name = patch.name
      if (patch.description !== undefined) payload.description = patch.description
      if (patch.defaultVisibility !== undefined) payload.default_visibility = patch.defaultVisibility
      if (patch.allowedRoles !== undefined) payload.allowed_roles = patch.allowedRoles
      if (patch.defaultBlocks !== undefined) payload.default_blocks = patch.defaultBlocks
      payload.updated_at = new Date().toISOString()

      const { data, error } = await supabase.from('profile_types').update(payload).eq('id', id).select('*').single()
      if (error) throw error

      appEventBus.publish({
        type: 'profile.type.updated',
        payload: { profileTypeId: data.id },
        occurredAt: new Date().toISOString(),
      })

      return mapProfileType(data)
    },

    async deleteProfileType(id) {
      const { error } = await supabase.from('profile_types').delete().eq('id', id)
      if (error) throw error
    },

    async listRules(profileTypeId) {
      const { data, error } = await supabase
        .from('profile_type_rules')
        .select('*')
        .eq('profile_type_id', profileTypeId)
        .order('priority', { ascending: true })
      if (error) throw error
      return (data ?? []).map(mapProfileTypeRule)
    },

    async createRule(input: CreateProfileTypeRuleInput) {
      const payload = {
        id: crypto.randomUUID(),
        profile_type_id: input.profileTypeId,
        priority: input.priority,
        match_role: input.matchRole ?? null,
        match_tags_all: input.matchTagsAll ?? null,
        match_tags_any: input.matchTagsAny ?? null,
        match_context_type: input.matchContextType ?? null,
        match_context_id: input.matchContextId ?? null,
        is_active: input.isActive ?? true,
      }
      const { data, error } = await supabase.from('profile_type_rules').insert(payload).select('*').single()
      if (error) throw error
      return mapProfileTypeRule(data)
    },

    async updateRule(id, patch: UpdateProfileTypeRuleInput) {
      const payload: Record<string, unknown> = {}
      if (patch.priority !== undefined) payload.priority = patch.priority
      if (patch.matchRole !== undefined) payload.match_role = patch.matchRole
      if (patch.matchTagsAll !== undefined) payload.match_tags_all = patch.matchTagsAll
      if (patch.matchTagsAny !== undefined) payload.match_tags_any = patch.matchTagsAny
      if (patch.matchContextType !== undefined) payload.match_context_type = patch.matchContextType
      if (patch.matchContextId !== undefined) payload.match_context_id = patch.matchContextId
      if (patch.isActive !== undefined) payload.is_active = patch.isActive

      const { data, error } = await supabase.from('profile_type_rules').update(payload).eq('id', id).select('*').single()
      if (error) throw error
      return mapProfileTypeRule(data)
    },

    async deleteRule(id) {
      const { error } = await supabase.from('profile_type_rules').delete().eq('id', id)
      if (error) throw error
    },
  }
}

