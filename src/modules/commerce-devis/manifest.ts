'use client'

export const commerceDevisModuleManifest = {
  id: 'commerce-devis',
  name: 'Commerce par Devis',
  version: '0.1.0',

  capabilitiesUsed: ['auth', 'documents', 'emailing', 'budget'],

  navigation: {
    group: 'modules',
    entries: [],
  },

  events: {
    published: [
      'commerce.quote_request.submitted',
      'commerce.quote.created',
      'commerce.quote.sent',
      'commerce.quote.accepted',
      'commerce.quote.rejected',
      'commerce.invoice.issued',
      'commerce.payment.confirmed',
    ],
    subscribed: ['emailing.notification.sent', 'emailing.notification.failed'],
  },
} as const

