'use client'

import { useState } from 'react'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { BottomNav } from '@/components/navigation/BottomNav'
import { Card, CardHeader, CardBody } from '@/components/atoms/card'
import { Avatar } from '@/components/atoms/avatar'
import { Badge } from '@/components/atoms/badge'

/**
 * ProfileScreen - Écran de profil utilisateur
 * 
 * @layer screens (portable vers Android/iOS)
 * @contract ScreenContract { render, props, navigation }
 * @dependencies FlyonUI, Card, Avatar, Badge
 */

export interface UserProfile {
  id: string
  name: string
  email: string
  avatar?: string
  role: string
  bio?: string
  joinedAt: string
  stats: {
    posts: number
    followers: number
    following: number
  }
}

export interface ProfileScreenProps {
  user?: UserProfile
  isCurrentUser?: boolean
  onEditProfile?: () => void
  onLogout?: () => void
  onNavigate?: (path: string) => void
}

// Mock user data
const mockUser: UserProfile = {
  id: '1',
  name: 'Jean Dupont',
  email: 'jean.dupont@exemple.com',
  role: 'Utilisateur Premium',
  bio: 'Passionné de technologie et développement. Amateur de café et de bons livres.',
  joinedAt: '2024-01-15',
  stats: {
    posts: 42,
    followers: 1234,
    following: 567
  }
}

export function ProfileScreen({ 
  user = mockUser, 
  isCurrentUser = true,
  onEditProfile,
  onLogout,
  onNavigate 
}: ProfileScreenProps) {
  const [activeTab, setActiveTab] = useState<'activity' | 'settings'>('activity')

  return (
    <AppShellScreen>
      <ContentStack>
        {/* Profile Header Card */}
        <Card className="overflow-visible">
          <CardBody className="pt-8">
            {/* Avatar & Basic Info */}
            <div className="flex flex-col items-center text-center -mt-16">
              <Avatar 
                src={user.avatar} 
                size="xl" 
                className="ring-4 ring-base-100 shadow-lg"
              />
              <h1 className="text-2xl font-bold text-base-content mt-4">{user.name}</h1>
              <p className="text-base-content/60">{user.email}</p>
              <Badge variant="primary" style="soft" className="mt-2">
                {user.role}
              </Badge>
            </div>

            {/* Bio */}
            {user.bio && (
              <p className="text-center text-base-content/80 mt-4 max-w-md mx-auto">
                {user.bio}
              </p>
            )}

            {/* Stats */}
            <div className="flex justify-center gap-8 mt-6 pt-6 border-t border-base-content/10">
              <div className="text-center">
                <div className="text-2xl font-bold text-base-content">{user.stats.posts}</div>
                <div className="text-sm text-base-content/60">Publications</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-base-content">{user.stats.followers.toLocaleString()}</div>
                <div className="text-sm text-base-content/60">Abonnés</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-base-content">{user.stats.following}</div>
                <div className="text-sm text-base-content/60">Abonnements</div>
              </div>
            </div>

            {/* Actions */}
            {isCurrentUser && (
              <div className="flex gap-3 mt-6 justify-center">
                <button 
                  className="btn btn-primary btn-soft"
                  onClick={onEditProfile}
                >
                  <span className="icon-[tabler--edit] size-5" />
                  Modifier le profil
                </button>
                <button className="btn btn-outline">
                  <span className="icon-[tabler--share] size-5" />
                  Partager
                </button>
              </div>
            )}
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
              <ul className="space-y-4">
                {[1, 2, 3].map((i) => (
                  <li key={i} className="flex items-start gap-3 pb-4 border-b border-base-content/10 last:border-0">
                    <div className="avatar avatar-placeholder">
                      <div className="bg-primary/20 text-primary rounded-full size-10 flex items-center justify-center">
                        <span className="icon-[tabler--message] size-5" />
                      </div>
                    </div>
                    <div className="flex-1">
                      <p className="text-base-content">
                        <span className="font-medium">Nouveau commentaire</span> sur votre publication
                      </p>
                      <p className="text-sm text-base-content/60">Il y a {i} heure{i > 1 ? 's' : ''}</p>
                    </div>
                  </li>
                ))}
              </ul>
            </CardBody>
          </Card>
        ) : (
          <Card>
            <CardBody className="space-y-2">
              {/* Settings Menu */}
              <button 
                className="btn btn-ghost btn-block justify-start"
                onClick={() => onNavigate?.('/settings/account')}
              >
                <span className="icon-[tabler--user] size-5" />
                Informations du compte
                <span className="icon-[tabler--chevron-right] size-5 ml-auto" />
              </button>
              <button 
                className="btn btn-ghost btn-block justify-start"
                onClick={() => onNavigate?.('/settings/notifications')}
              >
                <span className="icon-[tabler--bell] size-5" />
                Notifications
                <span className="icon-[tabler--chevron-right] size-5 ml-auto" />
              </button>
              <button 
                className="btn btn-ghost btn-block justify-start"
                onClick={() => onNavigate?.('/settings/privacy')}
              >
                <span className="icon-[tabler--lock] size-5" />
                Confidentialité
                <span className="icon-[tabler--chevron-right] size-5 ml-auto" />
              </button>
              <button 
                className="btn btn-ghost btn-block justify-start"
                onClick={() => onNavigate?.('/settings/appearance')}
              >
                <span className="icon-[tabler--palette] size-5" />
                Apparence
                <span className="icon-[tabler--chevron-right] size-5 ml-auto" />
              </button>
              
              <div className="divider" />
              
              <button 
                className="btn btn-ghost btn-block justify-start text-error"
                onClick={onLogout}
              >
                <span className="icon-[tabler--logout] size-5" />
                Déconnexion
              </button>
            </CardBody>
          </Card>
        )}

        {/* Member Since */}
        <p className="text-center text-sm text-base-content/50">
          Membre depuis {new Date(user.joinedAt).toLocaleDateString('fr-FR', { month: 'long', year: 'numeric' })}
        </p>
      </ContentStack>
      <BottomNav />
    </AppShellScreen>
  )
}
