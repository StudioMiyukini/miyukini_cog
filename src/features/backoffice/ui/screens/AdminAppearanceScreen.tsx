'use client'

import Link from 'next/link'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'

export function AdminAppearanceScreen() {
  return (
    <AdminLayout title="Apparence">
      <div>
        <h1 className="text-2xl font-bold text-base-content">Apparence</h1>
        <p className="text-base-content/60 mt-1">Branding, thèmes, et édition des pages publiques.</p>
      </div>

      <div className="mt-6 grid grid-cols-1 md:grid-cols-2 gap-6">
        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold">Branding</span>
            <span className="badge badge-success badge-soft badge-sm">Actif</span>
          </CardHeader>
          <CardBody>
            <p className="text-sm text-base-content/60">
              Titre et logo de l’application (affichés dans le header).
            </p>
            <div className="mt-4">
              <Link href="/admin/appearance/branding" className="btn btn-primary btn-sm">
                Ouvrir
                <span className="icon-[tabler--arrow-right] size-4" />
              </Link>
            </div>
          </CardBody>
        </Card>

        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold">Homepage</span>
            <span className="badge badge-success badge-soft badge-sm">Actif</span>
          </CardHeader>
          <CardBody>
            <p className="text-sm text-base-content/60">
              Édite le contenu de la page d’accueil `/` (hero, sections, onboarding, FAQ).
            </p>
            <div className="mt-4">
              <Link href="/admin/appearance/homepage" className="btn btn-primary btn-sm">
                Ouvrir l’éditeur
                <span className="icon-[tabler--arrow-right] size-4" />
              </Link>
            </div>
          </CardBody>
        </Card>

        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold">Thèmes</span>
            <span className="badge badge-warning badge-soft badge-sm">Bientôt</span>
          </CardHeader>
          <CardBody>
            <p className="text-sm text-base-content/60">
              Palette, typographies, modes light/dark, design system.
            </p>
          </CardBody>
        </Card>
      </div>
    </AdminLayout>
  )
}

