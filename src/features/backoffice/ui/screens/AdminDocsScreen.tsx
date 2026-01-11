'use client'

import Link from 'next/link'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { useRequireAdmin } from '../hooks/useRequireAdmin'

export function AdminDocsScreen() {
  const { isReady, authLoading } = useRequireAdmin()

  if (authLoading || !isReady) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }

  return (
    <AdminLayout title="Documentation">
      <h1 className="text-2xl font-bold text-base-content">Documentation</h1>
      <p className="text-base-content/60 mt-1">
        Accès rapide aux documents du framework (dossier <code>docs/</code>).
      </p>

      <div className="mt-6 grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card>
          <CardHeader>
            <span className="font-semibold">Framework</span>
          </CardHeader>
          <CardBody className="space-y-2">
            <Link className="link link-primary" href="/">
              Retour au site
            </Link>
            <p className="text-sm text-base-content/60">
              Les docs sont dans le repo. Cette page sert de hub back-office.
            </p>
          </CardBody>
        </Card>

        <Card>
          <CardHeader>
            <span className="font-semibold">Migrations</span>
          </CardHeader>
          <CardBody className="space-y-2">
            <p className="text-sm text-base-content/60">
              Voir <code>docs/framework/migration/</code> pour les scripts initiaux Supabase.
            </p>
            <Link className="link link-primary" href="/admin">
              Aller au dashboard
            </Link>
          </CardBody>
        </Card>
      </div>
    </AdminLayout>
  )
}

