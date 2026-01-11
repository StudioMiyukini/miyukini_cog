'use client'

import { useState, useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardHeader, CardBody } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { Avatar } from '@/components/atoms/avatar'
import { useCategories } from '@/contexts/CategoriesContext'
import { useAuth } from '@/contexts/AuthContext'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Profile } from '@/lib/supabase/database.types'

/**
 * BackOfficeScreen - Dashboard d'administration
 * Protégé par RLS : seuls les admins peuvent accéder aux données
 * 
 * @layer screens (portable vers Android/iOS)
 * @contract ScreenContract { render, props, navigation }
 * @rls profiles_select_admin - Admins peuvent voir tous les profils
 */

export interface BackOfficeScreenProps {
  onNavigate?: (path: string) => void
}

export function BackOfficeScreen({ onNavigate }: BackOfficeScreenProps) {
  const router = useRouter()
  const { user, profile, isAuthenticated, isLoading: authLoading } = useAuth()
  const [activeTab, setActiveTab] = useState<'overview' | 'users' | 'categories'>('overview')
  const { categories, enabledCategories, toggleCategory, resetCategories } = useCategories()
  
  // État pour les données admin
  const [users, setUsers] = useState<Profile[]>([])
  const [usersLoading, setUsersLoading] = useState(true)
  const [stats, setStats] = useState({
    totalUsers: 0,
    activeUsers: 0,
    pendingUsers: 0,
  })

  const supabase = getSupabaseClient()

  // Vérifier si l'utilisateur est admin
  const isAdmin = profile?.role === 'admin' || profile?.role === 'super_admin'

  // Rediriger si non admin
  useEffect(() => {
    if (!authLoading && !isAuthenticated) {
      router.push('/signin')
    } else if (!authLoading && isAuthenticated && !isAdmin) {
      router.push('/')
    }
  }, [authLoading, isAuthenticated, isAdmin, router])

  // Charger les utilisateurs (RLS filtre automatiquement)
  useEffect(() => {
    const loadUsers = async () => {
      if (!isAdmin) return
      
      setUsersLoading(true)
      try {
        const { data, error } = await supabase
          .from('profiles')
          .select('*')
          .order('created_at', { ascending: false })
          .limit(50)

        if (error) {
          console.error('Erreur chargement utilisateurs:', error)
          return
        }

        setUsers(data || [])
        
        // Calculer les stats
        const total = data?.length || 0
        const active = data?.filter((u: Profile) => u.email_verified && !u.deleted_at).length || 0
        const pending = data?.filter((u: Profile) => !u.email_verified && !u.deleted_at).length || 0
        
        setStats({
          totalUsers: total,
          activeUsers: active,
          pendingUsers: pending,
        })
      } catch (err) {
        console.error('Erreur loadUsers:', err)
      } finally {
        setUsersLoading(false)
      }
    }

    if (isAdmin) {
      loadUsers()
    }
  }, [isAdmin, supabase])

  // Loading state
  if (authLoading || (!isAuthenticated && !authLoading)) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }

  // Non admin - ne devrait pas arriver grâce à la redirection
  if (!isAdmin) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <Card className="max-w-md">
          <CardBody className="text-center py-8">
            <div className="avatar avatar-placeholder mx-auto mb-4">
              <div className="bg-error/20 text-error rounded-full size-16 flex items-center justify-center">
                <span className="icon-[tabler--lock] size-8" />
              </div>
            </div>
            <h1 className="text-xl font-bold text-base-content">Accès refusé</h1>
            <p className="text-base-content/60 mt-2">
              Vous n'avez pas les permissions nécessaires pour accéder au back-office.
            </p>
            <button 
              className="btn btn-primary mt-4"
              onClick={() => router.push('/')}
            >
              Retour à l'accueil
            </button>
          </CardBody>
        </Card>
      </div>
    )
  }

  // Fonction pour obtenir le badge de statut
  const getStatusBadge = (user: Profile) => {
    if (user.deleted_at) {
      return <Badge variant="error" size="xs">Supprimé</Badge>
    }
    if (!user.email_verified) {
      return <Badge variant="warning" size="xs">En attente</Badge>
    }
    return <Badge variant="success" size="xs">Actif</Badge>
  }

  // Fonction pour obtenir le badge de rôle
  const getRoleBadge = (role: string) => {
    switch (role) {
      case 'super_admin':
        return <Badge variant="error" size="xs">Super Admin</Badge>
      case 'admin':
        return <Badge variant="warning" size="xs">Admin</Badge>
      default:
        return <Badge variant="info" size="xs">User</Badge>
    }
  }

  return (
    <AdminLayout>
      {/* Page Header */}
      <div className="mb-6">
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
          <div>
            <h1 className="text-2xl font-bold text-base-content">Dashboard</h1>
            <p className="text-base-content/60 mt-1">
              Bienvenue, {profile?.display_name || 'Admin'} 
              <span className="ml-2">{getRoleBadge(profile?.role || 'user')}</span>
            </p>
          </div>
          <button className="btn btn-primary">
            <span className="icon-[tabler--plus] size-5" />
            Nouveau
          </button>
        </div>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-4 mb-6">
        {/* Stat Card 1 - Total Users */}
        <Card className="bg-base-100">
          <CardBody className="p-5">
            <div className="flex items-center gap-4">
              <div className="avatar avatar-placeholder">
                <div className="bg-primary/20 text-primary rounded-xl size-12 flex items-center justify-center">
                  <span className="icon-[tabler--users] size-6" />
                </div>
              </div>
              <div>
                <p className="text-base-content/60 text-sm">Utilisateurs</p>
                <p className="text-2xl font-bold text-base-content">
                  {usersLoading ? '...' : stats.totalUsers}
                </p>
                <p className="text-xs text-base-content/50">total</p>
              </div>
            </div>
          </CardBody>
        </Card>

        {/* Stat Card 2 - Active Users */}
        <Card className="bg-base-100">
          <CardBody className="p-5">
            <div className="flex items-center gap-4">
              <div className="avatar avatar-placeholder">
                <div className="bg-success/20 text-success rounded-xl size-12 flex items-center justify-center">
                  <span className="icon-[tabler--user-check] size-6" />
                </div>
              </div>
              <div>
                <p className="text-base-content/60 text-sm">Actifs</p>
                <p className="text-2xl font-bold text-base-content">
                  {usersLoading ? '...' : stats.activeUsers}
                </p>
                <p className="text-xs text-success">vérifiés</p>
              </div>
            </div>
          </CardBody>
        </Card>

        {/* Stat Card 3 - Pending */}
        <Card className="bg-base-100">
          <CardBody className="p-5">
            <div className="flex items-center gap-4">
              <div className="avatar avatar-placeholder">
                <div className="bg-warning/20 text-warning rounded-xl size-12 flex items-center justify-center">
                  <span className="icon-[tabler--user-question] size-6" />
                </div>
              </div>
              <div>
                <p className="text-base-content/60 text-sm">En attente</p>
                <p className="text-2xl font-bold text-base-content">
                  {usersLoading ? '...' : stats.pendingUsers}
                </p>
                <p className="text-xs text-warning">non vérifiés</p>
              </div>
            </div>
          </CardBody>
        </Card>

        {/* Stat Card 4 - Categories */}
        <Card className="bg-base-100">
          <CardBody className="p-5">
            <div className="flex items-center gap-4">
              <div className="avatar avatar-placeholder">
                <div className="bg-secondary/20 text-secondary rounded-xl size-12 flex items-center justify-center">
                  <span className="icon-[tabler--category] size-6" />
                </div>
              </div>
              <div>
                <p className="text-base-content/60 text-sm">Catégories</p>
                <p className="text-2xl font-bold text-base-content">{enabledCategories.length}/8</p>
                <p className="text-xs text-base-content/50">actives</p>
              </div>
            </div>
          </CardBody>
        </Card>
      </div>

      {/* Tabs */}
      <div className="tabs tabs-boxed bg-base-100 p-1 rounded-xl mb-6 inline-flex">
        <button 
          className={`tab ${activeTab === 'overview' ? 'tab-active' : ''}`}
          onClick={() => setActiveTab('overview')}
        >
          <span className="icon-[tabler--chart-pie] size-4 mr-2" />
          Aperçu
        </button>
        <button 
          className={`tab ${activeTab === 'users' ? 'tab-active' : ''}`}
          onClick={() => setActiveTab('users')}
        >
          <span className="icon-[tabler--users] size-4 mr-2" />
          Utilisateurs
        </button>
        <button 
          className={`tab ${activeTab === 'categories' ? 'tab-active' : ''}`}
          onClick={() => setActiveTab('categories')}
        >
          <span className="icon-[tabler--category] size-4 mr-2" />
          Catégories
        </button>
      </div>

      {/* Tab Content */}
      {activeTab === 'overview' && (
        <div className="grid lg:grid-cols-3 gap-6">
          {/* Left Column */}
          <div className="lg:col-span-2 space-y-6">
            {/* Users Table */}
            <Card className="bg-base-100">
              <CardHeader action={
                <button 
                  className="btn btn-ghost btn-sm"
                  onClick={() => setActiveTab('users')}
                >
                  Voir tout
                </button>
              }>
                <h3 className="text-lg font-semibold">Utilisateurs récents</h3>
              </CardHeader>
              <CardBody className="p-0">
                {usersLoading ? (
                  <div className="flex items-center justify-center py-8">
                    <span className="loading loading-spinner loading-md" />
                  </div>
                ) : users.length === 0 ? (
                  <div className="text-center py-8 text-base-content/60">
                    Aucun utilisateur trouvé
                  </div>
                ) : (
                  <div className="overflow-x-auto">
                    <table className="table">
                      <thead>
                        <tr>
                          <th>Nom</th>
                          <th>Email</th>
                          <th>Statut</th>
                          <th>Rôle</th>
                        </tr>
                      </thead>
                      <tbody>
                        {users.slice(0, 5).map((user) => (
                          <tr key={user.id}>
                            <td className="font-medium">
                              {user.display_name || user.first_name || 'N/A'}
                            </td>
                            <td className="text-base-content/70">{user.email}</td>
                            <td>{getStatusBadge(user)}</td>
                            <td>{getRoleBadge(user.role)}</td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                )}
              </CardBody>
            </Card>
          </div>

          {/* Right Column */}
          <div className="space-y-6">
            {/* Quick Info */}
            <Card className="bg-base-100">
              <CardHeader>
                <h3 className="text-lg font-semibold">Informations</h3>
              </CardHeader>
              <CardBody>
                <ul className="space-y-3">
                  <li className="flex items-center justify-between text-sm">
                    <span className="text-base-content/60">Rôle actuel</span>
                    <span>{getRoleBadge(profile?.role || 'user')}</span>
                  </li>
                  <li className="flex items-center justify-between text-sm">
                    <span className="text-base-content/60">Tier</span>
                    <Badge variant="info" size="xs">{profile?.tier || 'free'}</Badge>
                  </li>
                  <li className="flex items-center justify-between text-sm">
                    <span className="text-base-content/60">Catégories actives</span>
                    <span className="font-medium">{enabledCategories.length}</span>
                  </li>
                </ul>
              </CardBody>
            </Card>

            {/* RLS Info */}
            <Card className="bg-info/10 border border-info/20">
              <CardBody>
                <div className="flex items-start gap-3">
                  <span className="icon-[tabler--shield-check] size-6 text-info flex-shrink-0 mt-0.5" />
                  <div>
                    <h4 className="font-medium text-info">Protection RLS active</h4>
                    <p className="text-sm text-info/80 mt-1">
                      Les données sont protégées par Row Level Security. 
                      Seuls les administrateurs peuvent voir tous les profils.
                    </p>
                  </div>
                </div>
              </CardBody>
            </Card>
          </div>
        </div>
      )}

      {activeTab === 'users' && (
        <Card className="bg-base-100">
          <CardHeader action={
            <div className="flex gap-2">
              <button className="btn btn-ghost btn-sm">
                <span className="icon-[tabler--download] size-4" />
                Exporter
              </button>
              <button className="btn btn-primary btn-sm">
                <span className="icon-[tabler--plus] size-4" />
                Ajouter
              </button>
            </div>
          }>
            <h3 className="text-lg font-semibold">Gestion des utilisateurs</h3>
          </CardHeader>
          <CardBody className="p-0">
            {usersLoading ? (
              <div className="flex items-center justify-center py-8">
                <span className="loading loading-spinner loading-md" />
              </div>
            ) : users.length === 0 ? (
              <div className="text-center py-8 text-base-content/60">
                Aucun utilisateur trouvé
              </div>
            ) : (
              <div className="overflow-x-auto">
                <table className="table">
                  <thead>
                    <tr>
                      <th>Nom</th>
                      <th>Email</th>
                      <th>Statut</th>
                      <th>Rôle</th>
                      <th>Tier</th>
                      <th>Inscription</th>
                      <th>Actions</th>
                    </tr>
                  </thead>
                  <tbody>
                    {users.map((user) => (
                      <tr key={user.id}>
                        <td className="font-medium">
                          {user.display_name || user.first_name || 'N/A'}
                        </td>
                        <td className="text-base-content/70">{user.email}</td>
                        <td>{getStatusBadge(user)}</td>
                        <td>{getRoleBadge(user.role)}</td>
                        <td>
                          <Badge variant="neutral" size="xs">{user.tier}</Badge>
                        </td>
                        <td className="text-base-content/70">
                          {new Date(user.created_at).toLocaleDateString('fr-FR')}
                        </td>
                        <td>
                          <div className="flex gap-1">
                            <button className="btn btn-ghost btn-circle btn-sm" title="Modifier">
                              <span className="icon-[tabler--pencil] size-4" />
                            </button>
                            {user.id !== profile?.id && (
                              <button className="btn btn-ghost btn-circle btn-sm text-error" title="Supprimer">
                                <span className="icon-[tabler--trash] size-4" />
                              </button>
                            )}
                          </div>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </CardBody>
        </Card>
      )}

      {activeTab === 'categories' && (
        <Card className="bg-base-100">
          <CardHeader action={
            <button 
              className="btn btn-ghost btn-sm text-error"
              onClick={resetCategories}
            >
              <span className="icon-[tabler--refresh] size-4" />
              Réinitialiser
            </button>
          }>
            <div>
              <h3 className="text-lg font-semibold">Catégories de navigation</h3>
              <p className="text-sm text-base-content/60">Activez les catégories à afficher dans le BottomNav</p>
            </div>
          </CardHeader>
          <CardBody>
            <div className="grid sm:grid-cols-2 gap-3">
              {categories.map((cat) => (
                <div 
                  key={cat.id}
                  className={`flex items-center justify-between p-4 rounded-xl transition-colors ${
                    cat.enabled ? 'bg-primary/5 border-2 border-primary/20' : 'bg-base-200 border-2 border-transparent'
                  }`}
                >
                  <div className="flex items-center gap-3">
                    <div className={`rounded-xl size-10 flex items-center justify-center ${
                      cat.enabled ? 'bg-primary/20 text-primary' : 'bg-base-300 text-base-content/40'
                    }`}>
                      <span className={`${cat.iconClass} size-5`} />
                    </div>
                    <div>
                      <p className={`font-medium ${cat.enabled ? 'text-base-content' : 'text-base-content/60'}`}>
                        {cat.name}
                      </p>
                      <p className="text-xs text-base-content/50">{cat.path}</p>
                    </div>
                  </div>
                  <input
                    type="checkbox"
                    className="toggle toggle-primary toggle-sm"
                    checked={cat.enabled}
                    onChange={() => toggleCategory(cat.id)}
                  />
                </div>
              ))}
            </div>
            
            {/* Info */}
            <div className="mt-6 p-4 rounded-xl bg-info/10 border border-info/20">
              <p className="text-sm text-info flex items-center gap-2">
                <span className="icon-[tabler--info-circle] size-5" />
                <span><strong>{enabledCategories.length}</strong> catégorie(s) active(s) dans le BottomNav</span>
              </p>
            </div>

            {/* Preview */}
            <div className="mt-4 p-4 rounded-xl bg-base-200">
              <p className="text-sm font-medium text-base-content mb-3">Aperçu du BottomNav :</p>
              <div className="flex items-center justify-around py-3 bg-base-100 rounded-xl border border-base-300">
                {enabledCategories.length > 0 ? (
                  enabledCategories.slice(0, 5).map((cat) => (
                    <div key={cat.id} className="flex flex-col items-center text-primary">
                      <span className={`${cat.iconClass} size-5`} />
                      <span className="text-xs mt-1">{cat.name}</span>
                    </div>
                  ))
                ) : (
                  <p className="text-sm text-base-content/50 py-2">Aucune catégorie activée</p>
                )}
              </div>
              {enabledCategories.length > 5 && (
                <p className="text-xs text-base-content/50 mt-2 text-center">
                  +{enabledCategories.length - 5} autres catégories
                </p>
              )}
            </div>
          </CardBody>
        </Card>
      )}
    </AdminLayout>
  )
}
