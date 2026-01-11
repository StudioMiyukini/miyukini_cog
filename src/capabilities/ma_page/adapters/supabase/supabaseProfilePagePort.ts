import { getSupabaseClient } from '@/lib/supabase/client'
import { appEventBus } from '@/framework/events'
import type { ProfilePagePort } from '../../ports/ProfilePagePort'
import type {
  ProfileBlock,
  ProfilePage,
  ProfilePageVisibility,
  UpsertProfileBlockInput,
  UpsertProfilePageInput,
} from '../../domain/types'

type AnyClient = {
  from: (table: string) => any
  auth: { getUser: () => Promise<{ data: { user: { id: string } | null } }> }
}

function mapPage(row: any): ProfilePage {
  return {
    id: row.id,
    userId: row.user_id,
    profileTypeId: row.profile_type_id,
    slug: row.slug ?? null,
    visibility: (row.visibility ?? 'private') as ProfilePageVisibility,
    title: row.title ?? null,
    summary: row.summary ?? null,
    tags: row.tags ?? null,
    meta: row.meta ?? null,
    publishedAt: row.published_at ?? null,
    createdAt: row.created_at,
    updatedAt: row.updated_at,
  }
}

function mapBlock(row: any): ProfileBlock {
  return {
    id: row.id,
    pageId: row.page_id,
    blockType: row.block_type,
    position: row.position,
    isEnabled: row.is_enabled ?? true,
    config: row.config ?? null,
    dataRef: row.data_ref ?? null,
    createdAt: row.created_at,
    updatedAt: row.updated_at,
  }
}

export function createSupabaseProfilePagePort(): ProfilePagePort {
  const supabase = getSupabaseClient() as unknown as AnyClient

  async function getCurrentUserId(): Promise<string> {
    const res = await supabase.auth.getUser()
    const userId = res.data.user?.id
    if (!userId) throw new Error('Utilisateur non authentifié')
    return userId
  }

  return {
    async getMyPage() {
      const userId = await getCurrentUserId()
      return await this.getPageByUserId(userId)
    },

    async getPageByUserId(userId) {
      const { data, error } = await supabase.from('profile_pages').select('*').eq('user_id', userId).maybeSingle()
      if (error) throw error
      return data ? mapPage(data) : null
    },

    async getPageBySlug(slug) {
      const { data, error } = await supabase.from('profile_pages').select('*').eq('slug', slug).maybeSingle()
      if (error) throw error
      return data ? mapPage(data) : null
    },

    async upsertPage(input: UpsertProfilePageInput) {
      // Upsert par user_id (contrainte unique recommandée côté DB)
      const payload = {
        user_id: input.userId,
        profile_type_id: input.profileTypeId,
        slug: input.slug ?? null,
        visibility: input.visibility ?? 'private',
        title: input.title ?? null,
        summary: input.summary ?? null,
        tags: input.tags ?? null,
        meta: input.meta ?? null,
        updated_at: new Date().toISOString(),
      }

      const { data, error } = await supabase.from('profile_pages').upsert(payload).select('*').single()
      if (error) throw error

      appEventBus.publish({
        type: 'profile.page.updated',
        payload: { pageId: data.id, userId: data.user_id },
        occurredAt: new Date().toISOString(),
      })

      return mapPage(data)
    },

    async setVisibility({ pageId, visibility }) {
      const { data, error } = await supabase
        .from('profile_pages')
        .update({ visibility, updated_at: new Date().toISOString() })
        .eq('id', pageId)
        .select('*')
        .single()
      if (error) throw error
      return mapPage(data)
    },

    async publish(pageId) {
      const now = new Date().toISOString()
      const { data, error } = await supabase
        .from('profile_pages')
        .update({ published_at: now, visibility: 'public', updated_at: now })
        .eq('id', pageId)
        .select('*')
        .single()
      if (error) throw error

      appEventBus.publish({
        type: 'profile.page.published',
        payload: { pageId: data.id, userId: data.user_id },
        occurredAt: now,
      })

      return mapPage(data)
    },

    async unpublish(pageId) {
      const now = new Date().toISOString()
      const { data, error } = await supabase
        .from('profile_pages')
        .update({ published_at: null, visibility: 'private', updated_at: now })
        .eq('id', pageId)
        .select('*')
        .single()
      if (error) throw error

      appEventBus.publish({
        type: 'profile.page.unpublished',
        payload: { pageId: data.id, userId: data.user_id },
        occurredAt: now,
      })

      return mapPage(data)
    },

    async listBlocks(pageId) {
      const { data, error } = await supabase
        .from('profile_blocks')
        .select('*')
        .eq('page_id', pageId)
        .order('position', { ascending: true })
      if (error) throw error
      return (data ?? []).map(mapBlock)
    },

    async upsertBlock(input: UpsertProfileBlockInput) {
      const payload = {
        id: crypto.randomUUID(),
        page_id: input.pageId,
        block_type: input.blockType,
        position: input.position,
        is_enabled: input.isEnabled ?? true,
        config: input.config ?? null,
        data_ref: input.dataRef ?? null,
        updated_at: new Date().toISOString(),
      }

      // si tu veux un vrai upsert, ajoute une contrainte unique (page_id, position) ou (page_id, id).
      const { data, error } = await supabase.from('profile_blocks').insert(payload).select('*').single()
      if (error) throw error

      appEventBus.publish({
        type: 'profile.block.updated',
        payload: { blockId: data.id, pageId: data.page_id },
        occurredAt: new Date().toISOString(),
      })

      return mapBlock(data)
    },

    async deleteBlock(id) {
      const { error } = await supabase.from('profile_blocks').delete().eq('id', id)
      if (error) throw error
    },

    async reorderBlocks(pageId, orderedBlockIds) {
      // MVP simple : update en boucle (ok volume faible). Optimisable via RPC.
      let pos = 0
      for (const blockId of orderedBlockIds) {
        const { error } = await supabase
          .from('profile_blocks')
          .update({ position: pos++, updated_at: new Date().toISOString() })
          .eq('id', blockId)
          .eq('page_id', pageId)
        if (error) throw error
      }

      appEventBus.publish({
        type: 'profile.page.updated',
        payload: { pageId },
        occurredAt: new Date().toISOString(),
      })
    },
  }
}

