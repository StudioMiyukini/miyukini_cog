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
  UserId,
} from '../domain/types'

export type AgendaPort = {
  // Agendas
  listAgendas(input?: { workspaceId?: string; moduleId?: string }): Promise<Agenda[]>
  getAgendaById(id: AgendaId): Promise<Agenda | null>
  createAgenda(input: CreateAgendaInput): Promise<Agenda>
  updateAgenda(id: AgendaId, patch: UpdateAgendaInput): Promise<Agenda>
  deleteAgenda(id: AgendaId): Promise<void>

  // Slots
  getSlots(filters: GetSlotsFilters): Promise<Slot[]>
  getSlotById(id: SlotId): Promise<Slot | null>
  createSlot(input: CreateSlotInput): Promise<Slot>
  updateSlot(id: SlotId, patch: UpdateSlotInput): Promise<Slot>
  setSlotStatus(input: {
    slotId: SlotId
    nextStatus: SlotStatus
    actorId: UserId
    reason?: string
    meta?: Record<string, unknown>
  }): Promise<Slot>

  // Participants
  addParticipant(input: {
    slotId: SlotId
    userId: UserId
    role?: string | null
    status?: string | null
  }): Promise<SlotParticipant>
}

