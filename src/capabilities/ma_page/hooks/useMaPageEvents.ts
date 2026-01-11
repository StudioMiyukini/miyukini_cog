'use client'

import { useEffect } from 'react'
import { appEventBus } from '@/framework/events'
import type { MaPageEventType } from '../domain/events'

export function useMaPageEvents(
  type: MaPageEventType,
  handler: (event: { type: MaPageEventType; payload: any; occurredAt: string }) => void
) {
  useEffect(() => {
    return appEventBus.subscribe(type, handler as any)
  }, [type, handler])
}

