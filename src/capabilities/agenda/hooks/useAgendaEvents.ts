'use client'

import { useEffect } from 'react'
import { appEventBus } from '@/framework/events'
import type { AgendaEventType } from '../domain/events'

export function useAgendaEvents(
  type: AgendaEventType,
  handler: (event: { type: AgendaEventType; payload: any; occurredAt: string }) => void
) {
  useEffect(() => {
    return appEventBus.subscribe(type, handler as any)
  }, [type, handler])
}

