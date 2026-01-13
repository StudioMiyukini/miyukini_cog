'use client'

import { createContext, useContext, useState, useEffect, ReactNode, useCallback, useRef } from 'react'
import { User, Session, AuthError, type AuthChangeEvent } from '@supabase/supabase-js'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Profile } from '@/lib/supabase/database.types'

/**
 * AuthContext - Gestion de l'authentification Supabase
 * 
 * @layer contexts
 * @dependencies Supabase Auth
 */

interface AuthContextType {
  user: User | null
  profile: Profile | null
  session: Session | null
  isLoading: boolean
  isAuthenticated: boolean
  signIn: (email: string, password: string) => Promise<{ error: AuthError | null }>
  signUp: (
    email: string,
    password: string,
    profileData?: { firstName: string; lastName: string; phone: string }
  ) => Promise<{ error: AuthError | null }>
  signOut: () => Promise<void>
  refreshProfile: () => Promise<void>
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

function getSupabase() {
  // Singleton partagé sur toute l'app
  return getSupabaseClient() as any
}

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null)
  const [profile, setProfile] = useState<Profile | null>(null)
  const [session, setSession] = useState<Session | null>(null)
  const [isLoading, setIsLoading] = useState(true)
  const initialized = useRef(false)

  // Charger le profil utilisateur
  const fetchProfile = useCallback(async (userId: string): Promise<Profile | null> => {
    try {
      const supabase = getSupabase()
      const { data, error } = await supabase
        .from('profiles')
        .select('*')
        .eq('id', userId)
        .single()

      if (error) {
        console.error('Erreur lors du chargement du profil:', error)
        return null
      }
      return data as Profile
    } catch (err) {
      console.error('Erreur fetchProfile:', err)
      return null
    }
  }, [])

  // Rafraîchir le profil
  const refreshProfile = useCallback(async () => {
    if (user) {
      const newProfile = await fetchProfile(user.id)
      if (newProfile) {
        setProfile(newProfile)
      }
    }
  }, [user, fetchProfile])

  // Initialiser l'auth une seule fois
  useEffect(() => {
    if (initialized.current) return
    initialized.current = true

    const supabase = getSupabase()

    // Récupérer la session actuelle
    const initAuth = async () => {
      try {
        const { data: { session: currentSession } } = await supabase.auth.getSession()
        
        setSession(currentSession)
        setUser(currentSession?.user ?? null)

        if (currentSession?.user) {
          const userProfile = await fetchProfile(currentSession.user.id)
          setProfile(userProfile)
        }
      } catch (error) {
        console.error('Erreur initialisation auth:', error)
      } finally {
        setIsLoading(false)
      }
    }

    initAuth()

    // Écouter les changements d'auth
    const { data: { subscription } } = supabase.auth.onAuthStateChange(
      async (_event: AuthChangeEvent, newSession: Session | null) => {
        setSession(newSession)
        setUser(newSession?.user ?? null)

        if (newSession?.user) {
          // Petit délai pour le trigger handle_new_user
          await new Promise(resolve => setTimeout(resolve, 200))
          const userProfile = await fetchProfile(newSession.user.id)
          setProfile(userProfile)
        } else {
          setProfile(null)
        }

        setIsLoading(false)
      }
    )

    return () => {
      subscription.unsubscribe()
    }
  }, [fetchProfile])

  // Connexion
  const signIn = useCallback(async (email: string, password: string) => {
    const supabase = getSupabase()
    setIsLoading(true)
    try {
      const { error } = await supabase.auth.signInWithPassword({
        email,
        password,
      })
      return { error }
    } finally {
      setIsLoading(false)
    }
  }, [])

  // Inscription
  const signUp = useCallback(async (email: string, password: string, profileData?: { firstName: string; lastName: string; phone: string }) => {
    const supabase = getSupabase()
    setIsLoading(true)
    try {
      const displayName =
        profileData?.firstName && profileData?.lastName
          ? `${profileData.firstName} ${profileData.lastName}`.trim()
          : undefined
      const { error } = await supabase.auth.signUp({
        email,
        password,
        options: {
          data: {
            display_name: displayName || email.split('@')[0],
            first_name: profileData?.firstName ?? null,
            last_name: profileData?.lastName ?? null,
            phone: profileData?.phone ?? null,
          },
        },
      })
      return { error }
    } finally {
      setIsLoading(false)
    }
  }, [])

  // Déconnexion
  const signOut = useCallback(async () => {
    const supabase = getSupabase()
    setIsLoading(true)
    try {
      await supabase.auth.signOut()
      setUser(null)
      setProfile(null)
      setSession(null)
    } finally {
      setIsLoading(false)
    }
  }, [])

  return (
    <AuthContext.Provider
      value={{
        user,
        profile,
        session,
        isLoading,
        isAuthenticated: !!user,
        signIn,
        signUp,
        signOut,
        refreshProfile,
      }}
    >
      {children}
    </AuthContext.Provider>
  )
}

export function useAuth() {
  const context = useContext(AuthContext)
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider')
  }
  return context
}
