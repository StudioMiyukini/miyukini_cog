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
  // Créer un objet query builder mock qui supporte les chaînes de méthodes
  const createQueryBuilder = () => {
    const builder = {
      select: () => builder,
      insert: () => builder,
      update: () => builder,
      delete: () => builder,
      upsert: () => builder,
      eq: () => builder,
      in: () => builder,
      not: () => builder,
      order: () => builder,
      limit: () => builder,
      maybeSingle: () => Promise.resolve({ data: null, error: null }),
      single: () => Promise.resolve({ data: null, error: null }),
    }
    return builder
  }

  return {
    from: () => createQueryBuilder(),
    auth: {
      getUser: () => Promise.resolve({ data: { user: null }, error: null }),
      getSession: () => Promise.resolve({ data: { session: null }, error: null }),
      signInWithPassword: () => Promise.resolve({ data: { user: null, session: null }, error: { message: 'Supabase not configured' } }),
      signUp: () => Promise.resolve({ data: { user: null, session: null }, error: { message: 'Supabase not configured' } }),
      signOut: () => Promise.resolve({ error: null }),
      onAuthStateChange: (_callback: any) => {
        // Retourner un objet avec la structure correcte pour éviter les erreurs
        return {
          data: {
            subscription: {
              unsubscribe: () => {},
            },
          },
          error: null,
        }
      },
    },
    rpc: () => Promise.resolve({ data: null, error: { message: 'Supabase not configured' } }),
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
