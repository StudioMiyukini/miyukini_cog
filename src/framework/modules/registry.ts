import type { ModuleManifest } from './ModuleManifest'
import { agendaProModuleManifest } from '@/modules/agenda-pro'
import { bookingModuleManifest } from '@/modules/booking'
import { commerceDevisModuleManifest } from '@/modules/commerce-devis'

/**
 * Registry central (léger) des modules connus du framework.
 * IMPORTANT : l'activation/désactivation est gérée côté DB (pilotée super_admin).
 * Ici, on expose uniquement le catalogue "installable".
 */
const allModules: ModuleManifest[] = [agendaProModuleManifest, bookingModuleManifest, commerceDevisModuleManifest]

export function getAllModules(): ModuleManifest[] {
  return allModules
}

