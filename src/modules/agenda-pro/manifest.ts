export const agendaProModuleManifest = {
  id: 'agenda-pro',
  name: 'Agenda Pro',
  version: '0.1.0',

  capabilitiesUsed: ['auth', 'agenda', 'communication', 'notifications', 'documents', 'billing'],

  permissions: {
    requiredRoles: ['admin', 'planner', 'manager'],
  },

  navigation: {
    group: 'backoffice',
    entries: [
      { label: 'Agenda', href: '/admin/agenda', icon: 'tabler--calendar' },
      { label: 'Agendas', href: '/admin/agenda/calendars', icon: 'tabler--calendar-cog' },
      { label: 'Ressources', href: '/admin/agenda/resources', icon: 'tabler--users' },
      { label: 'Réservations', href: '/admin/agenda/bookings', icon: 'tabler--clipboard-list' },
      { label: 'Paramètres', href: '/admin/agenda/settings', icon: 'tabler--settings' },
    ],
  },

  events: {
    published: ['module.agenda-pro.booking.validated'],
    subscribed: [
      'agenda.slot.requested',
      'agenda.slot.confirmed',
      'agenda.slot.cancelled',
      'agenda.slot.rescheduled',
      'payment.invoice.paid',
    ],
  },
} as const

