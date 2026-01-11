'use client'

import { useCallback, useMemo, useState } from 'react'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables } from '@/lib/supabase/database.types'
import { useAuth } from '@/contexts/AuthContext'

type BookingProvider = Tables<'booking_providers'>

export function useBookingProvider() {
  const { isAuthenticated, profile, user } = useAuth()
  const supabase = useMemo(() => getSupabaseClient(), [])

  const [provider, setProvider] = useState<BookingProvider | null>(null)
  const [loading, setLoading] = useState(false)

  // Le Super Admin doit pouvoir accéder à l’agenda prestataire.
  // On conserve le refus pour le rôle "admin" (back-office général), mais pas pour "super_admin".
  const mustDenyProviderBackOffice = profile?.role === 'admin'

  const ensureProvider = useCallback(async () => {
    if (!isAuthenticated || !user?.id) return null
    if (mustDenyProviderBackOffice) return null

    setLoading(true)
    try {
      const { data: existing, error } = await supabase
        .from('booking_providers')
        .select('*')
        .eq('id', user.id)
        .maybeSingle()

      if (error) throw error
      if (existing) {
        setProvider(existing)
        return existing
      }

      const payload: BookingProvider = {
        id: user.id,
        display_name: profile?.display_name ?? profile?.email ?? null,
        timezone: 'Europe/Paris',
        tags: null,
        is_active: true,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      }

      const { data: created, error: insertError } = await supabase
        .from('booking_providers')
        .insert(payload)
        .select('*')
        .single()
      if (insertError) throw insertError
      setProvider(created)
      return created
    } finally {
      setLoading(false)
    }
  }, [isAuthenticated, mustDenyProviderBackOffice, profile?.display_name, profile?.email, supabase, user?.id])

  return {
    provider,
    loading,
    ensureProvider,
    isAuthenticated,
    mustDenyProviderBackOffice,
  }
}

