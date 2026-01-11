'use client'

import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { useModules } from '@/contexts/ModulesContext'
import Link from 'next/link'
import { useRequireSuperAdmin } from '../hooks/useRequireSuperAdmin'

export function AdminModulesScreen() {
  const { isReady, authLoading } = useRequireSuperAdmin()
  const { allModules, isEnabled, isLoading, setEnabled } = useModules()

  if (authLoading || !isReady) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }

  return (
    <AdminLayout title="Modules">
      <div>
        <h1 className="text-2xl font-bold text-base-content">Modules</h1>
        <p className="text-base-content/60 mt-1">
          Activation/désactivation des modules par le <strong>masteradmin</strong>.
        </p>
      </div>

      <Card className="mt-6">
        <CardHeader className="flex items-center justify-between">
          <span className="font-semibold text-base-content">Catalogue</span>
          <div className="flex items-center gap-2">
            {isLoading && <span className="loading loading-spinner loading-sm text-primary" />}
            <Badge variant="neutral" style="soft" size="sm">
              {allModules.length} modules
            </Badge>
          </div>
        </CardHeader>
        <CardBody className="p-0">
          <div className="overflow-x-auto">
            <table className="table table-zebra">
              <thead>
                <tr>
                  <th>Module</th>
                  <th>ID</th>
                  <th>Version</th>
                  <th>Actif</th>
                </tr>
              </thead>
              <tbody>
                {allModules.map((m) => (
                  <tr key={m.id}>
                    <td className="text-base-content font-medium">{m.name}</td>
                    <td className="text-base-content/60">{m.id}</td>
                    <td className="text-base-content/60">{m.version}</td>
                    <td>
                      <div className="flex items-center gap-3">
                        <input
                          type="checkbox"
                          className="toggle toggle-primary"
                          checked={isEnabled(m.id)}
                          disabled={isLoading}
                          onChange={(e) => setEnabled(m.id, e.target.checked)}
                        />
                        <Link href={`/admin/modules/${encodeURIComponent(m.id)}/integrations/paypal`} className="btn btn-ghost btn-xs">
                          PayPal
                        </Link>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </CardBody>
      </Card>
    </AdminLayout>
  )
}

