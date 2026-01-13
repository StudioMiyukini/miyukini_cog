import { createBrowserClient } from '@supabase/ssr'
import type { Database } from './database.types'

/**
 * Client Supabase côté navigateur (client-side)
 * À utiliser dans les composants React côté client
 * 
 * ⚠️ Ne pas appeler directement cette fonction. Utilisez getSupabaseClient() à la place.
 */
export function createClient() {
  const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL
  const supabaseAnonKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY

  if (!supabaseUrl || !supabaseAnonKey) {
    // Ne pas lancer d'erreur, retourner null et laisser getSupabaseClient gérer
    // Cela évite les erreurs si cette fonction est appelée directement
    return null as any
  }

  return createBrowserClient<Database>(supabaseUrl, supabaseAnonKey)
}

// Export singleton pour usage simple
let browserClient: ReturnType<typeof createBrowserClient> | null = null

// Client mock pour quand Supabase n'est pas configuré
function createMockClient() {
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

export function getSupabaseClient() {
  // Vérifier que les variables d'environnement sont disponibles
  if (!process.env.NEXT_PUBLIC_SUPABASE_URL || !process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY) {
    // Retourner un client mock qui ne fera rien (build time ou runtime sans config)
    if (typeof window !== 'undefined') {
      // Seulement logger côté client pour éviter les logs serveur
      console.warn(
        '⚠️ Supabase environment variables not configured. Using mock client. Please set NEXT_PUBLIC_SUPABASE_URL and NEXT_PUBLIC_SUPABASE_ANON_KEY'
      )
    }
    return createMockClient()
  }
  
  if (!browserClient) {
    browserClient = createClient()
    // Si createClient retourne null (variables manquantes), utiliser le mock
    if (!browserClient) {
      return createMockClient()
    }
  }
  return browserClient
}
