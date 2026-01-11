'use client'

import { useState, useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { BottomNav } from '@/components/navigation/BottomNav'
import { Card, CardHeader, CardBody } from '@/components/atoms/card'
import { Avatar } from '@/components/atoms/avatar'
import { Badge } from '@/components/atoms/badge'
import { useAuth } from '@/contexts/AuthContext'

/**
 * ProfileScreen - Écran de profil utilisateur
 * 
 * @layer screens (portable vers Android/iOS)
 * @contract ScreenContract { render, props, navigation }
 * @dependencies FlyonUI, Card, Avatar, Badge, Supabase Auth
 */

export interface ProfileScreenProps {
  onNavigate?: (path: string) => void
}

export function ProfileScreen({ onNavigate }: ProfileScreenProps) {
  const router = useRouter()
  const { user, profile, isAuthenticated, isLoading, signOut, refreshProfile } = useAuth()
  const [activeTab, setActiveTab] = useState<'activity' | 'settings'>('activity')

  // Rediriger si non connecté
  useEffect(() => {
    if (!isLoading && !isAuthenticated) {
      router.push('/signin')
    }
  }, [isAuthenticated, isLoading, router])

  const handleSignOut = async () => {
    await signOut()
    router.push('/')
  }

  const handleNavigate = (path: string) => {
    if (onNavigate) {
      onNavigate(path)
    } else {
      router.push(path)
    }
  }

  // Loading state
  if (isLoading) {
    return (
      <AppShellScreen>
        <ContentStack>
          <div className="flex items-center justify-center min-h-[50vh]">
            <span className="loading loading-spinner loading-lg text-primary" />
          </div>
        </ContentStack>
        <BottomNav />
      </AppShellScreen>
    )
  }

  // Non connecté
  if (!isAuthenticated || !user) {
    return null // La redirection est gérée par useEffect
  }

  // Données du profil
  const displayName = profile?.display_name || profile?.first_name || user.email?.split('@')[0] || 'Utilisateur'
  const email = profile?.email || user.email || ''
  const avatarUrl = profile?.avatar_url || undefined
  const role = profile?.role || 'user'
  const tier = profile?.tier || 'free'
  const joinedAt = profile?.created_at || user.created_at

  // Badges de rôle
  const getRoleBadge = () => {
    switch (role) {
      case 'super_admin':
        return <Badge variant="error" style="soft">Super Admin</Badge>
      case 'admin':
        return <Badge variant="warning" style="soft">Admin</Badge>
      default:
        return <Badge variant="primary" style="soft">Utilisateur</Badge>
    }
  }

  // Badges de tier
  const getTierBadge = () => {
    switch (tier) {
      case 'enterprise':
        return <Badge variant="secondary" style="soft">Enterprise</Badge>
      case 'pro':
        return <Badge variant="success" style="soft">Pro</Badge>
      case 'starter':
        return <Badge variant="info" style="soft">Starter</Badge>
      default:
        return <Badge variant="neutral" style="soft">Gratuit</Badge>
    }
  }

  return (
    <AppShellScreen>
      <ContentStack>
        {/* Profile Header Card */}
        <Card className="overflow-visible">
          <CardBody className="pt-8">
            {/* Avatar & Basic Info */}
            <div className="flex flex-col items-center text-center -mt-16">
              <Avatar 
                src={avatarUrl} 
                size="xl" 
                className="ring-4 ring-base-100 shadow-lg"
              />
              <h1 className="text-2xl font-bold text-base-content mt-4">{displayName}</h1>
              <p className="text-base-content/60">{email}</p>
              <div className="flex gap-2 mt-2">
                {getRoleBadge()}
                {getTierBadge()}
              </div>
            </div>

            {/* Bio */}
            {profile?.metadata && typeof profile.metadata === 'object' && 'bio' in profile.metadata && (
              <p className="text-center text-base-content/80 mt-4 max-w-md mx-auto">
                {String(profile.metadata.bio)}
              </p>
            )}

            {/* Status */}
            <div className="flex justify-center gap-8 mt-6 pt-6 border-t border-base-content/10">
              <div className="text-center">
                <div className={`w-3 h-3 rounded-full mx-auto mb-1 ${profile?.email_verified ? 'bg-success' : 'bg-warning'}`} />
                <div className="text-sm text-base-content/60">
                  Email {profile?.email_verified ? 'vérifié' : 'non vérifié'}
                </div>
              </div>
              <div className="text-center">
                <div className={`w-3 h-3 rounded-full mx-auto mb-1 ${profile?.onboarding_completed ? 'bg-success' : 'bg-warning'}`} />
                <div className="text-sm text-base-content/60">
                  Onboarding {profile?.onboarding_completed ? 'complété' : 'en cours'}
                </div>
              </div>
            </div>

            {/* Actions */}
            <div className="flex gap-3 mt-6 justify-center">
              <button 
                className="btn btn-primary btn-soft"
                onClick={() => handleNavigate('/profile/edit')}
              >
                <span className="icon-[tabler--edit] size-5" />
                Modifier le profil
              </button>
              <button className="btn btn-outline">
                <span className="icon-[tabler--share] size-5" />
                Partager
              </button>
            </div>
          </CardBody>
        </Card>

        {/* Tabs */}
        <div className="tabs tabs-boxed bg-base-200 p-1 rounded-box">
          <button 
            className={`tab flex-1 ${activeTab === 'activity' ? 'tab-active' : ''}`}
            onClick={() => setActiveTab('activity')}
          >
            <span className="icon-[tabler--activity] size-5 mr-2" />
            Activité
          </button>
          <button 
            className={`tab flex-1 ${activeTab === 'settings' ? 'tab-active' : ''}`}
            onClick={() => setActiveTab('settings')}
          >
            <span className="icon-[tabler--settings] size-5 mr-2" />
            Paramètres
          </button>
        </div>

        {/* Tab Content */}
        {activeTab === 'activity' ? (
          <Card>
            <CardHeader>Activité récente</CardHeader>
            <CardBody>
              <div className="text-center py-8 text-base-content/60">
                <span className="icon-[tabler--activity] size-12 opacity-30 block mx-auto mb-2" />
                <p>Aucune activité récente</p>
              </div>
            </CardBody>
          </Card>
        ) : (
          <Card>
            <CardBody className="space-y-2">
              {/* Settings Menu */}
              <button 
                className="btn btn-ghost btn-block justify-start"
                onClick={() => handleNavigate('/profile/edit')}
              >
                <span className="icon-[tabler--user] size-5" />
                Informations du compte
                <span className="icon-[tabler--chevron-right] size-5 ml-auto" />
              </button>
              <button 
                className="btn btn-ghost btn-block justify-start"
                onClick={() => handleNavigate('/settings/notifications')}
              >
                <span className="icon-[tabler--bell] size-5" />
                Notifications
                <span className="icon-[tabler--chevron-right] size-5 ml-auto" />
              </button>
              <button 
                className="btn btn-ghost btn-block justify-start"
                onClick={() => handleNavigate('/settings/privacy')}
              >
                <span className="icon-[tabler--lock] size-5" />
                Confidentialité
                <span className="icon-[tabler--chevron-right] size-5 ml-auto" />
              </button>
              <button 
                className="btn btn-ghost btn-block justify-start"
                onClick={() => handleNavigate('/settings/appearance')}
              >
                <span className="icon-[tabler--palette] size-5" />
                Apparence
                <span className="icon-[tabler--chevron-right] size-5 ml-auto" />
              </button>
              
              {/* Admin link */}
              {(role === 'admin' || role === 'super_admin') && (
                <>
                  <div className="divider" />
                  <button 
                    className="btn btn-ghost btn-block justify-start text-warning"
                    onClick={() => handleNavigate('/admin')}
                  >
                    <span className="icon-[tabler--dashboard] size-5" />
                    Administration
                    <span className="icon-[tabler--chevron-right] size-5 ml-auto" />
                  </button>
                </>
              )}
              
              <div className="divider" />
              
              <button 
                className="btn btn-ghost btn-block justify-start text-error"
                onClick={handleSignOut}
              >
                <span className="icon-[tabler--logout] size-5" />
                Déconnexion
              </button>
            </CardBody>
          </Card>
        )}

        {/* Member Since */}
        <p className="text-center text-sm text-base-content/50">
          Membre depuis {new Date(joinedAt).toLocaleDateString('fr-FR', { month: 'long', year: 'numeric' })}
        </p>
      </ContentStack>
      <BottomNav />
    </AppShellScreen>
  )
}
