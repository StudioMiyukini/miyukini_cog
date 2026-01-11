'use client'

import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { useRequireAdmin } from '../hooks/useRequireAdmin'

export interface AdminPlaceholderScreenProps {
  title: string
  description?: string
}

export function AdminPlaceholderScreen({ title, description }: AdminPlaceholderScreenProps) {
  const { isReady, authLoading } = useRequireAdmin()

  if (authLoading || !isReady) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }

  return (
    <AdminLayout title={title}>
      <h1 className="text-2xl font-bold text-base-content">{title}</h1>
      {description && <p className="text-base-content/60 mt-1">{description}</p>}

      <Card className="mt-6">
        <CardHeader>
          <span className="font-semibold">Bientôt</span>
        </CardHeader>
        <CardBody>
          <p className="text-base-content/70">
            Cette page est prête côté routing et layout. La fonctionnalité sera implémentée dans un prochain sprint.
          </p>
        </CardBody>
      </Card>
    </AdminLayout>
  )
}

