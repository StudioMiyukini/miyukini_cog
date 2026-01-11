import { getSupabaseClient } from '@/lib/supabase/client'
import { appEventBus } from '@/framework/events'
import type { AgendaPort } from '../../ports/AgendaPort'
import type {
  Agenda,
  AgendaId,
  CreateAgendaInput,
  CreateSlotInput,
  GetSlotsFilters,
  Slot,
  SlotId,
  SlotParticipant,
  SlotStatus,
  UpdateAgendaInput,
  UpdateSlotInput,
} from '../../domain/types'

type AnyClient = {
  from: (table: string) => any
  auth: { getUser: () => Promise<{ data: { user: { id: string } | null } }> }
}

function mapAgendaRow(row: any): Agenda {
  return {
    id: row.id,
    moduleId: row.module_id,
    name: row.name,
    description: row.description ?? null,
    workspaceId: row.workspace_id,
    createdBy: row.created_by,
    defaultSlotDuration: row.default_slot_duration ?? null,
    slotBufferBefore: row.slot_buffer_before ?? null,
    slotBufferAfter: row.slot_buffer_after ?? null,
    maxParticipantsPerSlot: row.max_participants_per_slot ?? null,
    allowOverbooking: row.allow_overbooking ?? null,
    timezone: row.timezone ?? null,
    isPublic: row.is_public ?? null,
    createdAt: row.created_at,
    updatedAt: row.updated_at,
  }
}

function mapSlotRow(row: any): Slot {
  return {
    id: row.id,
    agendaId: row.agenda_id,
    resourceId: row.resource_id ?? null,
    startAt: row.start_at,
    endAt: row.end_at,
    status: row.status,
    capacity: row.capacity ?? null,
    participantsCount: row.participants_count ?? null,
    price: row.price ?? null,
    currency: row.currency ?? null,
    paymentLink: row.payment_link ?? null,
    tags: row.tags ?? null,
    metadata: row.metadata ?? null,
    createdAt: row.created_at,
    updatedAt: row.updated_at,
  }
}

export function createSupabaseAgendaPort(): AgendaPort {
  // Tant que `database.types.ts` n'inclut pas `agendas/slots/...`,
  // on utilise un client "untyped" (cast) pour éviter les erreurs TS.
  const supabase = getSupabaseClient() as unknown as AnyClient

  const port: AgendaPort = {
    async listAgendas(input) {
      let q = supabase.from('agendas').select('*')
      if (input?.workspaceId) q = q.eq('workspace_id', input.workspaceId)
      if (input?.moduleId) q = q.eq('module_id', input.moduleId)

      const { data, error } = await q.order('created_at', { ascending: false })
      if (error) throw error
      return (data ?? []).map(mapAgendaRow)
    },

    async getAgendaById(id) {
      const { data, error } = await supabase.from('agendas').select('*').eq('id', id).maybeSingle()
      if (error) throw error
      return data ? mapAgendaRow(data) : null
    },

    async createAgenda(input) {
      const userRes = await supabase.auth.getUser()
      const userId = userRes.data.user?.id
      if (!userId) throw new Error('Utilisateur non authentifié')

      const payload = {
        id: crypto.randomUUID(),
        module_id: input.moduleId,
        name: input.name,
        description: input.description ?? null,
        workspace_id: input.workspaceId,
        created_by: userId,
        default_slot_duration: input.defaultSlotDuration ?? null,
        slot_buffer_before: input.slotBufferBefore ?? null,
        slot_buffer_after: input.slotBufferAfter ?? null,
        max_participants_per_slot: input.maxParticipantsPerSlot ?? null,
        allow_overbooking: input.allowOverbooking ?? null,
        timezone: input.timezone ?? null,
        is_public: input.isPublic ?? null,
      }

      const { data, error } = await supabase.from('agendas').insert(payload).select('*').single()
      if (error) throw error

      appEventBus.publish({
        type: 'agenda.schedule.updated',
        payload: { agendaId: data.id, workspaceId: data.workspace_id },
        occurredAt: new Date().toISOString(),
      })

      return mapAgendaRow(data)
    },

    async updateAgenda(id, patch) {
      const payload: Record<string, unknown> = {}
      if (patch.name !== undefined) payload.name = patch.name
      if (patch.description !== undefined) payload.description = patch.description
      if (patch.defaultSlotDuration !== undefined) payload.default_slot_duration = patch.defaultSlotDuration
      if (patch.slotBufferBefore !== undefined) payload.slot_buffer_before = patch.slotBufferBefore
      if (patch.slotBufferAfter !== undefined) payload.slot_buffer_after = patch.slotBufferAfter
      if (patch.maxParticipantsPerSlot !== undefined)
        payload.max_participants_per_slot = patch.maxParticipantsPerSlot
      if (patch.allowOverbooking !== undefined) payload.allow_overbooking = patch.allowOverbooking
      if (patch.timezone !== undefined) payload.timezone = patch.timezone
      if (patch.isPublic !== undefined) payload.is_public = patch.isPublic
      payload.updated_at = new Date().toISOString()

      const { data, error } = await supabase.from('agendas').update(payload).eq('id', id).select('*').single()
      if (error) throw error

      appEventBus.publish({
        type: 'agenda.schedule.updated',
        payload: { agendaId: data.id, workspaceId: data.workspace_id },
        occurredAt: new Date().toISOString(),
      })

      return mapAgendaRow(data)
    },

    async deleteAgenda(id) {
      const { error } = await supabase.from('agendas').delete().eq('id', id)
      if (error) throw error
    },

    async getSlots(filters) {
      let q = supabase.from('slots').select('*')
      if (filters.workspaceId) {
        // par design, la table slots peut ne pas porter workspace_id (dérivé via agenda).
        // on garde le filtre au niveau module (RPC à prévoir). Ici on ne filtre pas.
      }
      if (filters.agendaIds?.length) q = q.in('agenda_id', filters.agendaIds)
      if (filters.resourceIds?.length) q = q.in('resource_id', filters.resourceIds)
      if (filters.statuses?.length) q = q.in('status', filters.statuses)
      if (filters.startRange) q = q.gte('start_at', filters.startRange)
      if (filters.endRange) q = q.lte('end_at', filters.endRange)
      if (filters.tags?.length) q = q.contains('tags', filters.tags)
      if (filters.limit) q = q.limit(filters.limit)

      const { data, error } = await q.order('start_at', { ascending: true })
      if (error) throw error
      return (data ?? []).map(mapSlotRow)
    },

    async getSlotById(id) {
      const { data, error } = await supabase.from('slots').select('*').eq('id', id).maybeSingle()
      if (error) throw error
      return data ? mapSlotRow(data) : null
    },

    async createSlot(input) {
      const payload = {
        id: crypto.randomUUID(),
        agenda_id: input.agendaId,
        resource_id: input.resourceId ?? null,
        start_at: input.startAt,
        end_at: input.endAt,
        status: input.status ?? ('draft' as SlotStatus),
        capacity: input.capacity ?? null,
        price: input.price ?? null,
        currency: input.currency ?? null,
        payment_link: input.paymentLink ?? null,
        tags: input.tags ?? null,
        metadata: input.metadata ?? null,
      }

      const { data, error } = await supabase.from('slots').insert(payload).select('*').single()
      if (error) throw error

      appEventBus.publish({
        type: 'agenda.slot.requested',
        payload: { slotId: data.id, agendaId: data.agenda_id },
        occurredAt: new Date().toISOString(),
      })

      return mapSlotRow(data)
    },

    async updateSlot(id, patch) {
      const payload: Record<string, unknown> = {}
      if (patch.resourceId !== undefined) payload.resource_id = patch.resourceId
      if (patch.startAt !== undefined) payload.start_at = patch.startAt
      if (patch.endAt !== undefined) payload.end_at = patch.endAt
      if (patch.status !== undefined) payload.status = patch.status
      if (patch.capacity !== undefined) payload.capacity = patch.capacity
      if (patch.price !== undefined) payload.price = patch.price
      if (patch.currency !== undefined) payload.currency = patch.currency
      if (patch.paymentLink !== undefined) payload.payment_link = patch.paymentLink
      if (patch.tags !== undefined) payload.tags = patch.tags
      if (patch.metadata !== undefined) payload.metadata = patch.metadata
      payload.updated_at = new Date().toISOString()

      const { data, error } = await supabase.from('slots').update(payload).eq('id', id).select('*').single()
      if (error) throw error
      return mapSlotRow(data)
    },

    async setSlotStatus({ slotId, nextStatus, actorId, reason, meta }) {
      const current = await port.getSlotById(slotId)
      if (!current) throw new Error('Slot introuvable')

      const { data, error } = await supabase
        .from('slots')
        .update({ status: nextStatus, updated_at: new Date().toISOString() })
        .eq('id', slotId)
        .select('*')
        .single()
      if (error) throw error

      const emittedEventType =
        nextStatus === 'confirmed'
          ? ('agenda.slot.confirmed' as const)
          : nextStatus === 'cancelled'
            ? ('agenda.slot.cancelled' as const)
            : nextStatus === 'paid'
              ? ('agenda.slot.paid' as const)
              : null

      // Persist event log (DB)
      {
        const { error: eventError } = await supabase.from('slot_events').insert({
          slot_id: slotId,
          event_type: emittedEventType ?? 'agenda.slot.status_changed',
          actor_id: actorId,
          payload: { previousStatus: current.status, nextStatus, reason, meta },
        })
        if (eventError) throw eventError
      }

      // Publish app event (in-memory) only for declared domain events
      if (emittedEventType) {
        appEventBus.publish({
          type: emittedEventType,
          payload: {
            slotId,
            agendaId: data.agenda_id,
            actorId,
            previousStatus: current.status,
            nextStatus,
            reason,
            meta,
          },
          occurredAt: new Date().toISOString(),
        })
      }

      return mapSlotRow(data)
    },

    async addParticipant({ slotId, userId, role, status }) {
      const payload = {
        slot_id: slotId,
        user_id: userId,
        role: role ?? null,
        status: status ?? 'invited',
        confirmed_at: null,
      }

      const { data, error } = await supabase
        .from('slot_participants')
        .insert(payload)
        .select('*')
        .single()
      if (error) throw error

      return {
        slotId: data.slot_id,
        userId: data.user_id,
        role: data.role ?? null,
        status: data.status ?? null,
        confirmedAt: data.confirmed_at ?? null,
      }
    },
  }

  return port
}

