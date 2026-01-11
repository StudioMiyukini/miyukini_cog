'use client'

import { useMemo } from 'react'
import type { AgendaPort } from '../ports/AgendaPort'
import { createSupabaseAgendaPort } from '../adapters/supabase/supabaseAgendaPort'

/**
 * Hook d'accès à la capability Agenda.
 * IMPORTANT : la capability n'expose pas d'écran; seuls les modules l'utilisent.
 */
export function useAgenda(): AgendaPort {
  return useMemo(() => createSupabaseAgendaPort(), [])
}

