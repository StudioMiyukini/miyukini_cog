export { agendaCapabilityManifest } from './manifest'

export type { AgendaPort } from './ports/AgendaPort'

export type {
  Agenda,
  Slot,
  SlotParticipant,
  SlotResource,
  SlotEvent,
  SlotStatus,
  GetSlotsFilters,
  CreateAgendaInput,
  UpdateAgendaInput,
  CreateSlotInput,
  UpdateSlotInput,
} from './domain/types'

export { useAgenda } from './hooks/useAgenda'
export { useAgendaEvents } from './hooks/useAgendaEvents'

