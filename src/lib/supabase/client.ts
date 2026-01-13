import { createBrowserClient } from '@supabase/ssr'
import type { Database } from './database.types'

/**
 * Client Supabase côté navigateur (client-side)
 * À utiliser dans les composants React côté client
 */
export function createClient() {
  const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL
  const supabaseAnonKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY

  if (!supabaseUrl || !supabaseAnonKey) {
    // Ne pas lancer d'erreur ici, laisser getSupabaseClient gérer
    throw new Error(
      'Missing Supabase environment variables. Please set NEXT_PUBLIC_SUPABASE_URL and NEXT_PUBLIC_SUPABASE_ANON_KEY'
    )
  }

  return createBrowserClient<Database>(supabaseUrl, supabaseAnonKey)
}

// Export singleton pour usage simple
let browserClient: ReturnType<typeof createBrowserClient> | null = null

export function getSupabaseClient() {
  // Vérifier que les variables d'environnement sont disponibles
  if (!process.env.NEXT_PUBLIC_SUPABASE_URL || !process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY) {
    // Retourner un client mock qui ne fera rien (build time ou runtime sans config)
    console.warn(
      '⚠️ Supabase environment variables not configured. Using mock client. Please set NEXT_PUBLIC_SUPABASE_URL and NEXT_PUBLIC_SUPABASE_ANON_KEY'
    )
    return {
      from: () => ({
        select: () => Promise.resolve({ data: null, error: { message: 'Supabase not configured', code: 'PGRST116' } }),
        insert: () => Promise.resolve({ data: null, error: { message: 'Supabase not configured', code: 'PGRST116' } }),
        update: () => Promise.resolve({ data: null, error: { message: 'Supabase not configured', code: 'PGRST116' } }),
        delete: () => Promise.resolve({ data: null, error: { message: 'Supabase not configured', code: 'PGRST116' } }),
        upsert: () => Promise.resolve({ data: null, error: { message: 'Supabase not configured', code: 'PGRST116' } }),
      }),
      auth: {
        getUser: () => Promise.resolve({ data: { user: null }, error: { message: 'Supabase not configured' } }),
        getSession: () => Promise.resolve({ data: { session: null }, error: { message: 'Supabase not configured' } }),
        signInWithPassword: () => Promise.resolve({ data: { user: null, session: null }, error: { message: 'Supabase not configured' } }),
        signOut: () => Promise.resolve({ error: null }),
        onAuthStateChange: () => ({ data: { subscription: null }, error: null }),
      },
    } as any
  }
  
  if (!browserClient) {
    try {
      browserClient = createClient()
    } catch (error) {
      console.error('Failed to create Supabase client:', error)
      // Retourner le mock en cas d'erreur
      return getSupabaseClient() // Récursion mais avec les variables manquantes, retournera le mock
    }
  }
  return browserClient
}
