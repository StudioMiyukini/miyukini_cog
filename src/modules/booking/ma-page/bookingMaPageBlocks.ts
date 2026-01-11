import type { BlockTypeDefinition } from '@/capabilities/ma_page'

/**
 * Déclarations de blocs "connectés" pour le module Booking.
 * IMPORTANT : ce fichier ne crée aucune route. Il sert de déclaration (manifest/runtime).
 *
 * Le rendu UI de ces blocs se fait côté module Booking (écrans), pas dans la capability Ma Page.
 */
export const bookingMaPageBlockDefinitions: BlockTypeDefinition[] = [
  {
    blockType: 'agenda_booking',
    label: 'Réservation (Agenda)',
    description: 'Affiche les disponibilités et permet de réserver selon les accès (RLS).',
    canBePublic: true,
  },
]

