export type AgendaId = string
export type SlotId = string
export type ResourceId = string
export type WorkspaceId = string
export type UserId = string

export type SlotStatus = 'draft' | 'pending' | 'confirmed' | 'paid' | 'cancelled'

export type Agenda = {
  id: AgendaId
  moduleId: string
  name: string
  description: string | null
  workspaceId: WorkspaceId
  createdBy: UserId
  defaultSlotDuration: number | null
  slotBufferBefore: number | null
  slotBufferAfter: number | null
  maxParticipantsPerSlot: number | null
  allowOverbooking: boolean | null
  timezone: string | null
  isPublic: boolean | null
  createdAt: string
  updatedAt: string
}

export type Slot = {
  id: SlotId
  agendaId: AgendaId
  resourceId: ResourceId | null
  startAt: string
  endAt: string
  status: SlotStatus
  capacity: number | null
  participantsCount: number | null
  price: number | null
  currency: string | null
  paymentLink: string | null
  tags: string[] | null
  metadata: Record<string, unknown> | null
  createdAt: string
  updatedAt: string
}

export type SlotParticipant = {
  slotId: SlotId
  userId: UserId
  role: string | null
  status: string | null
  confirmedAt: string | null
}

export type SlotResource = {
  slotId: SlotId
  resourceId: ResourceId
  resourceType: string | null
  assignedAt: string | null
}

export type SlotEvent = {
  slotId: SlotId
  eventType: string
  actorId: UserId | null
  payload: Record<string, unknown> | null
  createdAt: string
}

export type GetSlotsFilters = {
  agendaIds?: AgendaId[]
  resourceIds?: ResourceId[]
  statuses?: SlotStatus[]
  tags?: string[]
  workspaceId?: WorkspaceId
  startRange?: string
  endRange?: string
  limit?: number
}

export type CreateAgendaInput = {
  moduleId: string
  name: string
  description?: string | null
  workspaceId: WorkspaceId
  defaultSlotDuration?: number | null
  slotBufferBefore?: number | null
  slotBufferAfter?: number | null
  maxParticipantsPerSlot?: number | null
  allowOverbooking?: boolean | null
  timezone?: string | null
  isPublic?: boolean | null
}

export type UpdateAgendaInput = Partial<Omit<CreateAgendaInput, 'workspaceId' | 'moduleId'>> & {
  name?: string
}

export type CreateSlotInput = {
  agendaId: AgendaId
  resourceId?: ResourceId | null
  startAt: string
  endAt: string
  status?: SlotStatus
  capacity?: number | null
  price?: number | null
  currency?: string | null
  paymentLink?: string | null
  tags?: string[] | null
  metadata?: Record<string, unknown> | null
}

export type UpdateSlotInput = Partial<Omit<CreateSlotInput, 'agendaId'>> & {
  status?: SlotStatus
}

