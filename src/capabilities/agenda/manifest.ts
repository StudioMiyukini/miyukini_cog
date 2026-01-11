export const agendaCapabilityManifest = {
  id: 'agenda',
  name: 'Agenda',
  version: '0.1.0',

  exports: {
    ports: ['AgendaPort'],
    hooks: ['useAgenda', 'useAgendaEvents'],
  },

  dataContracts: {
    tables: ['agendas', 'slots', 'slot_participants', 'slot_resources', 'slot_events'],
  },

  policies: {
    agendas: ['select:workspace_or_role', 'write:admin_planner_scheduler'],
    slots: ['select:workspace_or_participant', 'write:owner_or_admin', 'delete:draft_or_admin'],
    slot_participants: ['select:participant_or_admin', 'write:scheduler_or_invited'],
  },

  eventsEmitted: [
    'agenda.slot.requested',
    'agenda.slot.confirmed',
    'agenda.slot.cancelled',
    'agenda.slot.rescheduled',
    'agenda.slot.paid',
    'agenda.capacity.threshold',
    'agenda.schedule.updated',
    'agenda.slot.document.generated',
  ],

  eventsConsumed: [
    'payment.invoice.paid',
    'documents.file.uploaded',
    'notifications.preference.changed',
  ],
} as const

