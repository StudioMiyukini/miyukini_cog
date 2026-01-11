import type { AgendaId, SlotId, SlotStatus, UserId } from './types'

export type AgendaEventType =
  | 'agenda.slot.requested'
  | 'agenda.slot.confirmed'
  | 'agenda.slot.cancelled'
  | 'agenda.slot.rescheduled'
  | 'agenda.slot.paid'
  | 'agenda.capacity.threshold'
  | 'agenda.schedule.updated'
  | 'agenda.slot.document.generated'

export type AgendaSlotRequestedPayload = {
  slotId: SlotId
  agendaId: AgendaId
  requestedBy: UserId
  context?: Record<string, unknown>
  meta?: Record<string, unknown>
}

export type AgendaSlotStatusChangedPayload = {
  slotId: SlotId
  agendaId: AgendaId
  actorId: UserId
  previousStatus: SlotStatus
  nextStatus: SlotStatus
  reason?: string
  meta?: Record<string, unknown>
}

