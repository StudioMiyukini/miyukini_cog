'use client'

import { useState } from 'react'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardHeader, CardBody } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { Avatar } from '@/components/atoms/avatar'
import { useCategories } from '@/contexts/CategoriesContext'

/**
 * BackOfficeScreen - Dashboard d'administration
 * Utilise AdminLayout avec menu latéral
 * 
 * @layer screens (portable vers Android/iOS)
 * @contract ScreenContract { render, props, navigation }
 */

export interface BackOfficeScreenProps {
  onNavigate?: (path: string) => void
}

// Mock data
const mockUsers = [
  { id: '1', name: 'Jean Dupont', email: 'jean@exemple.com', status: 'Actif', role: 'Admin', date: '2026-01-08' },
  { id: '2', name: 'Marie Martin', email: 'marie@exemple.com', status: 'En attente', role: 'User', date: '2026-01-07' },
  { id: '3', name: 'Pierre Durand', email: 'pierre@exemple.com', status: 'Actif', role: 'User', date: '2026-01-06' },
  { id: '4', name: 'Sophie Bernard', email: 'sophie@exemple.com', status: 'Inactif', role: 'User', date: '2026-01-05' },
]

const mockMeetings = [
  { id: '1', title: 'Revue de sprint', date: '10 Jan', time: '09:00-10:00', category: 'Dev', variant: 'primary' as const },
  { id: '2', title: 'Point client', date: '10 Jan', time: '14:00-15:00', category: 'Business', variant: 'warning' as const },
  { id: '3', title: 'Design review', date: '11 Jan', time: '11:00-12:00', category: 'Design', variant: 'info' as const },
  { id: '4', title: 'Formation équipe', date: '12 Jan', time: '10:00-11:30', category: 'RH', variant: 'success' as const },
]

const recentActivity = [
  { action: 'Nouvel utilisateur inscrit', user: 'Marie M.', time: 'Il y a 5 min', icon: 'icon-[tabler--user-plus]' },
  { action: 'Contenu publié', user: 'Admin', time: 'Il y a 15 min', icon: 'icon-[tabler--article]' },
  { action: 'Paramètres modifiés', user: 'Jean D.', time: 'Il y a 1h', icon: 'icon-[tabler--settings]' },
]

export function BackOfficeScreen({ onNavigate }: BackOfficeScreenProps) {
  const [activeTab, setActiveTab] = useState<'overview' | 'users' | 'categories'>('overview')
  const { categories, enabledCategories, toggleCategory, resetCategories } = useCategories()

  return (
    <AdminLayout>
      {/* Page Header */}
      <div className="mb-6">
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
          <div>
            <h1 className="text-2xl font-bold text-base-content">Dashboard</h1>
            <p className="text-base-content/60 mt-1">Bienvenue dans le back-office Miyukini</p>
          </div>
          <button className="btn btn-primary">
            <span className="icon-[tabler--plus] size-5" />
            Nouveau
          </button>
        </div>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-4 mb-6">
        {/* Stat Card 1 */}
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
                <p className="text-2xl font-bold text-base-content">1,234</p>
                <p className="text-xs text-success flex items-center gap-1">
                  <span className="icon-[tabler--arrow-up] size-3" />
                  12.5%
                </p>
              </div>
            </div>
          </CardBody>
        </Card>

        {/* Stat Card 2 */}
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

        {/* Stat Card 3 */}
        <Card className="bg-base-100">
          <CardBody className="p-5">
            <div className="flex items-center gap-4">
              <div className="avatar avatar-placeholder">
                <div className="bg-accent/20 text-accent rounded-xl size-12 flex items-center justify-center">
                  <span className="icon-[tabler--eye] size-6" />
                </div>
              </div>
              <div>
                <p className="text-base-content/60 text-sm">Visites</p>
                <p className="text-2xl font-bold text-base-content">45.2K</p>
                <p className="text-xs text-error flex items-center gap-1">
                  <span className="icon-[tabler--arrow-down] size-3" />
                  3.1%
                </p>
              </div>
            </div>
          </CardBody>
        </Card>

        {/* Stat Card 4 */}
        <Card className="bg-base-100">
          <CardBody className="p-5">
            <div className="flex items-center gap-4">
              <div className="avatar avatar-placeholder">
                <div className="bg-success/20 text-success rounded-xl size-12 flex items-center justify-center">
                  <span className="icon-[tabler--chart-bar] size-6" />
                </div>
              </div>
              <div>
                <p className="text-base-content/60 text-sm">Conversion</p>
                <p className="text-2xl font-bold text-base-content">4.8%</p>
                <p className="text-xs text-success flex items-center gap-1">
                  <span className="icon-[tabler--arrow-up] size-3" />
                  1.2%
                </p>
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
            {/* Meeting Schedules */}
            <Card className="bg-base-100">
              <CardHeader action={
                <button className="btn btn-ghost btn-sm">Voir tout</button>
              }>
                <h3 className="text-lg font-semibold">Réunions planifiées</h3>
              </CardHeader>
              <CardBody>
                <ul className="space-y-4">
                  {mockMeetings.map((meeting) => (
                    <li key={meeting.id} className="flex items-center gap-4">
                      <Avatar size="sm" />
                      <div className="flex-1 min-w-0">
                        <p className="font-medium text-base-content truncate">{meeting.title}</p>
                        <p className="text-sm text-base-content/50 flex items-center gap-1">
                          <span className="icon-[tabler--calendar] size-4" />
                          {meeting.date} | {meeting.time}
                        </p>
                      </div>
                      <Badge variant={meeting.variant} size="sm">{meeting.category}</Badge>
                    </li>
                  ))}
                </ul>
              </CardBody>
            </Card>

            {/* Users Table */}
            <Card className="bg-base-100">
              <CardHeader action={
                <button className="btn btn-ghost btn-sm">Gérer</button>
              }>
                <h3 className="text-lg font-semibold">Utilisateurs récents</h3>
              </CardHeader>
              <CardBody className="p-0">
                <div className="overflow-x-auto">
                  <table className="table">
                    <thead>
                      <tr>
                        <th>Nom</th>
                        <th>Email</th>
                        <th>Statut</th>
                        <th>Rôle</th>
                        <th></th>
                      </tr>
                    </thead>
                    <tbody>
                      {mockUsers.map((user) => (
                        <tr key={user.id}>
                          <td className="font-medium">{user.name}</td>
                          <td className="text-base-content/70">{user.email}</td>
                          <td>
                            <Badge 
                              variant={user.status === 'Actif' ? 'success' : user.status === 'En attente' ? 'warning' : 'error'}
                              size="xs"
                            >
                              {user.status}
                            </Badge>
                          </td>
                          <td>{user.role}</td>
                          <td>
                            <button className="btn btn-ghost btn-circle btn-sm">
                              <span className="icon-[tabler--dots-vertical] size-4" />
                            </button>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </CardBody>
            </Card>
          </div>

          {/* Right Column */}
          <div className="space-y-6">
            {/* Recent Activity */}
            <Card className="bg-base-100">
              <CardHeader>
                <h3 className="text-lg font-semibold">Activité récente</h3>
              </CardHeader>
              <CardBody>
                <ul className="space-y-4">
                  {recentActivity.map((item, index) => (
                    <li key={index} className="flex items-start gap-3">
                      <div className="avatar avatar-placeholder">
                        <div className="bg-base-200 rounded-lg size-8 flex items-center justify-center">
                          <span className={`${item.icon} size-4 text-base-content/70`} />
                        </div>
                      </div>
                      <div className="flex-1 min-w-0">
                        <p className="text-sm text-base-content">{item.action}</p>
                        <p className="text-xs text-base-content/50">{item.user} • {item.time}</p>
                      </div>
                    </li>
                  ))}
                </ul>
              </CardBody>
            </Card>

            {/* Quick Stats */}
            <Card className="bg-gradient-to-br from-primary/10 to-secondary/10 border-none">
              <CardBody>
                <h3 className="font-semibold text-base-content mb-4">Progression mensuelle</h3>
                <div className="space-y-3">
                  <div>
                    <div className="flex justify-between text-sm mb-1">
                      <span className="text-base-content/70">Objectif utilisateurs</span>
                      <span className="font-medium">78%</span>
                    </div>
                    <progress className="progress progress-primary h-2" value="78" max="100"></progress>
                  </div>
                  <div>
                    <div className="flex justify-between text-sm mb-1">
                      <span className="text-base-content/70">Contenu créé</span>
                      <span className="font-medium">45%</span>
                    </div>
                    <progress className="progress progress-secondary h-2" value="45" max="100"></progress>
                  </div>
                  <div>
                    <div className="flex justify-between text-sm mb-1">
                      <span className="text-base-content/70">Engagement</span>
                      <span className="font-medium">92%</span>
                    </div>
                    <progress className="progress progress-accent h-2" value="92" max="100"></progress>
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
            <button className="btn btn-primary btn-sm">
              <span className="icon-[tabler--plus] size-4" />
              Ajouter
            </button>
          }>
            <h3 className="text-lg font-semibold">Gestion des utilisateurs</h3>
          </CardHeader>
          <CardBody className="p-0">
            <div className="overflow-x-auto">
              <table className="table">
                <thead>
                  <tr>
                    <th>Nom</th>
                    <th>Email</th>
                    <th>Statut</th>
                    <th>Rôle</th>
                    <th>Inscription</th>
                    <th>Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {mockUsers.map((user) => (
                    <tr key={user.id}>
                      <td className="font-medium">{user.name}</td>
                      <td className="text-base-content/70">{user.email}</td>
                      <td>
                        <Badge 
                          variant={user.status === 'Actif' ? 'success' : user.status === 'En attente' ? 'warning' : 'error'}
                          size="xs"
                        >
                          {user.status}
                        </Badge>
                      </td>
                      <td>{user.role}</td>
                      <td className="text-base-content/70">{user.date}</td>
                      <td>
                        <div className="flex gap-1">
                          <button className="btn btn-ghost btn-circle btn-sm">
                            <span className="icon-[tabler--pencil] size-4" />
                          </button>
                          <button className="btn btn-ghost btn-circle btn-sm text-error">
                            <span className="icon-[tabler--trash] size-4" />
                          </button>
                        </div>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
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
