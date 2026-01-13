'use client'

import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { Card, CardBody } from '@/components/atoms/card'
import { useRouter } from 'next/navigation'

export default function PricingCancelPage() {
  const router = useRouter()

  return (
    <AppShellScreen>
      <div className="max-w-2xl mx-auto px-4 py-8 sm:px-6 lg:px-8">
        <Card>
          <CardBody className="text-center py-12">
            <div className="mb-6">
              <span className="icon-[tabler--x-circle] size-16 text-error mx-auto" />
            </div>
            <h2 className="text-2xl font-bold mb-2">Abonnement annulé</h2>
            <p className="text-base-content/60 mb-6">
              Vous avez annulé le processus d'abonnement. Aucun paiement n'a été effectué.
            </p>
            <div className="flex gap-4 justify-center">
              <button className="btn btn-primary" onClick={() => router.push('/pricing')}>
                Réessayer
              </button>
              <button className="btn btn-outline" onClick={() => router.push('/')}>
                Retour à l'accueil
              </button>
            </div>
          </CardBody>
        </Card>
      </div>
    </AppShellScreen>
  )
}
