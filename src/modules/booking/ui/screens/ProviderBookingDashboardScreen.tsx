'use client'

import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { Card, CardBody } from '@/components/atoms/card'
import { useAuth } from '@/contexts/AuthContext'
import { useRequireEnabledModule } from '../hooks/useRequireEnabledModule'
import Link from 'next/link'

export function ProviderBookingDashboardScreen() {
  const { isLoading, enabled } = useRequireEnabledModule('booking', '/')
  const { isAuthenticated, profile } = useAuth()
  const mustDenyProviderBackOffice = profile?.role === 'admin' || profile?.role === 'super_admin'

  if (isLoading) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }

  if (!enabled) return null

  if (!isAuthenticated) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <div className="text-base-content/70">Connecte-toi pour accéder à l’espace prestataire.</div>
      </div>
    )
  }

  if (mustDenyProviderBackOffice) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <div className="text-base-content/70">
          L’espace prestataire est réservé aux prestataires (pas aux admins plateforme).
        </div>
      </div>
    )
  }

  return (
    <AppShellScreen>
      <ContentStack>
        <div className="space-y-2">
          <h1 className="text-2xl font-bold text-base-content">Back-office prestataire</h1>
          <p className="text-base-content/60">
            Ici : agenda des réservations + gestion créneaux + prestations (filtré par RLS).
          </p>
        </div>

        <Card>
          <CardBody className="space-y-2">
            <p className="text-sm text-base-content/60">
              Prochaine implémentation :
            </p>
            <ul className="list-disc pl-5 text-sm text-base-content/70 space-y-1">
              <li>CRUD prestations (`booking_services`) + disponibilité par créneau (`booking_slot_services`).</li>
              <li>Planning prestataire (slots Agenda) + création en masse (semaine type/vacances).</li>
              <li>Agenda des réservations (lecture/gestion `booking_bookings`).</li>
            </ul>
            <div className="pt-2">
              <div className="flex flex-col sm:flex-row gap-2">
                <Link href="/pro/booking/services" className="btn btn-primary btn-sm">
                  Prestations
                </Link>
                <Link href="/pro/booking/planning" className="btn btn-ghost btn-sm">
                  Planning
                </Link>
                <Link href="/pro/booking/time-off" className="btn btn-ghost btn-sm">
                  Vacances
                </Link>
                <Link href="/pro/booking/week-templates" className="btn btn-ghost btn-sm">
                  Semaine type
                </Link>
                <Link href="/pro/booking/bookings" className="btn btn-ghost btn-sm">
                  Réservations
                </Link>
              </div>
            </div>
          </CardBody>
        </Card>
      </ContentStack>
    </AppShellScreen>
  )
}

