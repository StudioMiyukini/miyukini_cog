'use client'

import { ReactNode } from 'react'
import { useRequireSubscription } from '@/hooks/useRequireSubscription'
import { Card, CardBody } from '@/components/atoms/card'
import Link from 'next/link'

type SubscriptionGateProps = {
  children: ReactNode
  /**
   * Message personnalisé affiché si pas d'abonnement
   */
  message?: string
  /**
   * Chemin de redirection vers le paywall
   * @default '/pricing'
   */
  paywallPath?: string
  /**
   * Afficher un loader pendant la vérification
   */
  showLoader?: boolean
}

/**
 * Composant qui bloque l'accès si l'utilisateur n'a pas d'abonnement actif
 * Affiche un message avec lien vers le paywall
 */
export function SubscriptionGate({
  children,
  message = 'Un abonnement actif est requis pour accéder à cette fonctionnalité.',
  paywallPath = '/pricing',
  showLoader = true,
}: SubscriptionGateProps) {
  const { isLoading, hasSubscription } = useRequireSubscription({
    redirectToPaywall: false, // On gère l'affichage nous-mêmes
  })

  if (isLoading && showLoader) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }

  if (!hasSubscription) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center p-4">
        <Card className="max-w-md w-full">
          <CardBody className="text-center py-12">
            <div className="mb-6">
              <span className="icon-[tabler--lock] size-16 text-warning mx-auto" />
            </div>
            <h2 className="text-2xl font-bold mb-2">Abonnement requis</h2>
            <p className="text-base-content/60 mb-6">{message}</p>
            <div className="flex flex-col sm:flex-row gap-3 justify-center">
              <Link href={paywallPath} className="btn btn-primary">
                Voir les plans
              </Link>
              <Link href="/subscription" className="btn btn-outline">
                Mon abonnement
              </Link>
            </div>
          </CardBody>
        </Card>
      </div>
    )
  }

  return <>{children}</>
}
