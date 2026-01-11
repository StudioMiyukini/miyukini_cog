export const bookingModuleManifest = {
  id: 'booking',
  name: 'Booking (Réservation)',
  version: '0.1.0',

  capabilitiesUsed: ['auth', 'agenda', 'ma_page', 'emailing', 'documents'],

  // Le module expose des routes/screens quand il est activé (hors scope ici).
  navigation: {
    group: 'modules',
    entries: [],
  },

  events: {
    published: [
      'booking.booking.requested',
      'booking.booking.confirmed',
      'booking.booking.cancelled',
      'booking.booking.rescheduled',
    ],
    subscribed: ['agenda.slot.confirmed', 'agenda.slot.cancelled'],
  },
} as const

