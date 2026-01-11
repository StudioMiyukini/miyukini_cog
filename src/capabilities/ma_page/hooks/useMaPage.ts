'use client'

import { useMemo } from 'react'
import type { ProfileTypePort } from '../ports/ProfileTypePort'
import type { ProfilePagePort } from '../ports/ProfilePagePort'
import type { BlockRegistryPort } from '../ports/BlockRegistryPort'
import type { BlockDataPort } from '../ports/BlockDataPort'
import { createSupabaseProfileTypePort } from '../adapters/supabase/supabaseProfileTypePort'
import { createSupabaseProfilePagePort } from '../adapters/supabase/supabaseProfilePagePort'
import { maPageBlockRegistry } from '../adapters/inMemory/inMemoryBlockRegistry'
import { maPageBlockDataPort } from '../adapters/inMemory/inMemoryBlockDataPort'

export type MaPageCapability = {
  profileTypes: ProfileTypePort
  profilePages: ProfilePagePort
  blockRegistry: BlockRegistryPort
  blockData: BlockDataPort
}

/**
 * Hook d'accès à la capability Ma Page.
 * IMPORTANT : aucune UI n'est exposée directement par la capability.
 * Les modules (ex: Booking) consomment ces ports et rendent les blocs.
 */
export function useMaPage(): MaPageCapability {
  return useMemo(
    () => ({
      profileTypes: createSupabaseProfileTypePort(),
      profilePages: createSupabaseProfilePagePort(),
      blockRegistry: maPageBlockRegistry,
      blockData: maPageBlockDataPort,
    }),
    []
  )
}

