'use client'

import { useEffect, useState } from 'react'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { Avatar } from '@/components/atoms/avatar'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Profile } from '@/lib/supabase/database.types'
import { useRequireAdmin } from '../hooks/useRequireAdmin'

export function AdminUsersScreen() {
  const { isReady, authLoading } = useRequireAdmin()
  const [users, setUsers] = useState<Profile[]>([])
  const [loading, setLoading] = useState(true)
  const supabase = getSupabaseClient()

  useEffect(() => {
    const load = async () => {
      if (!isReady) return
      setLoading(true)
      try {
        const { data, error } = await supabase
          .from('profiles')
          .select('*')
          .order('created_at', { ascending: false })
          .limit(200)

        if (error) {
          console.error('Erreur chargement utilisateurs:', error)
          return
        }
        setUsers(data || [])
      } finally {
        setLoading(false)
      }
    }
    load()
  }, [isReady, supabase])

  if (authLoading || !isReady) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }

  return (
    <AdminLayout title="Utilisateurs">
      <div className="flex items-start justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold text-base-content">Utilisateurs</h1>
          <p className="text-base-content/60 mt-1">Liste des profils (protégé par RLS).</p>
        </div>
        <button className="btn btn-primary btn-sm">
          <span className="icon-[tabler--user-plus] size-4" />
          Nouvel utilisateur
        </button>
      </div>

      <Card className="mt-6">
        <CardHeader className="flex items-center justify-between">
          <span className="font-semibold text-base-content">Liste</span>
          <Badge variant="neutral" style="soft" size="sm">
            {users.length} total
          </Badge>
        </CardHeader>
        <CardBody className="p-0">
          {loading ? (
            <div className="p-6 flex items-center justify-center">
              <span className="loading loading-spinner text-primary" />
            </div>
          ) : users.length === 0 ? (
            <div className="p-6 text-base-content/60">Aucun utilisateur.</div>
          ) : (
            <div className="overflow-x-auto">
              <table className="table table-zebra">
                <thead>
                  <tr>
                    <th>Utilisateur</th>
                    <th>Email</th>
                    <th>Rôle</th>
                    <th>Tier</th>
                    <th>Statut</th>
                  </tr>
                </thead>
                <tbody>
                  {users.map((u) => {
                    const name =
                      u.display_name ||
                      [u.first_name, u.last_name].filter(Boolean).join(' ') ||
                      u.email?.split('@')[0] ||
                      'Utilisateur'
                    const initials = name
                      .split(' ')
                      .filter(Boolean)
                      .slice(0, 2)
                      .map((p) => p[0]?.toUpperCase())
                      .join('')
                    const status = u.deleted_at
                      ? 'Supprimé'
                      : u.email_verified
                        ? 'Actif'
                        : 'En attente'
                    return (
                      <tr key={u.id}>
                        <td>
                          <div className="flex items-center gap-3">
                            <Avatar
                              size="sm"
                              src={u.avatar_url || undefined}
                              alt={name}
                              placeholder={
                                initials ? (
                                  <span className="text-xs font-semibold">{initials}</span>
                                ) : undefined
                              }
                            />
                            <div className="min-w-0">
                              <div className="font-medium text-base-content truncate">{name}</div>
                              <div className="text-xs text-base-content/60 truncate">{u.id}</div>
                            </div>
                          </div>
                        </td>
                        <td className="text-base-content">{u.email || '-'}</td>
                        <td>
                          <Badge variant="neutral" style="soft" size="sm">
                            {u.role}
                          </Badge>
                        </td>
                        <td>
                          <Badge variant="neutral" style="soft" size="sm">
                            {u.tier}
                          </Badge>
                        </td>
                        <td>
                          <Badge
                            variant={
                              status === 'Actif'
                                ? 'success'
                                : status === 'En attente'
                                  ? 'warning'
                                  : 'error'
                            }
                            size="sm"
                          >
                            {status}
                          </Badge>
                        </td>
                      </tr>
                    )
                  })}
                </tbody>
              </table>
            </div>
          )}
        </CardBody>
      </Card>
    </AdminLayout>
  )
}

