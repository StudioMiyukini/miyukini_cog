'use client'

import Link from 'next/link'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { BottomNav } from '@/components/navigation/BottomNav'
import { Card, CardBody } from '@/components/atoms/card'
import { useRequireEnabledModule } from '../hooks/useRequireEnabledModule'

export function BookingLandingScreen() {
  const { isLoading, enabled } = useRequireEnabledModule('booking', '/')

  if (isLoading) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }

  if (!enabled) return null

  return (
    <AppShellScreen>
      <ContentStack>
        <div className="space-y-2">
          <h1 className="text-2xl font-bold text-base-content">Réservation</h1>
          <p className="text-base-content/60">
            Module Booking activé. Prochaine étape : pages prestataire (Ma Page) + disponibilités (Agenda).
          </p>
        </div>

        <Card>
          <CardBody className="space-y-4">
            <div className="flex items-start gap-3">
              <span className="icon-[tabler--calendar-event] size-6 text-primary" />
              <div>
                <h2 className="font-semibold text-base-content">Réserver un créneau</h2>
                <p className="text-sm text-base-content/60">
                  La recherche prestataire/prestation + la sélection de créneaux sera branchée sur la capability Agenda.
                </p>
              </div>
            </div>

            <div className="flex flex-col sm:flex-row gap-2">
              <Link href="/pro/booking" className="btn btn-primary btn-sm">
                Espace prestataire
              </Link>
              <Link href="/" className="btn btn-ghost btn-sm">
                Retour
              </Link>
            </div>
          </CardBody>
        </Card>
      </ContentStack>
      <BottomNav />
    </AppShellScreen>
  )
}

