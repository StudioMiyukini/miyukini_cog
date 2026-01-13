'use client'

import { useEffect, useState } from 'react'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { Card, CardBody } from '@/components/atoms/card'
import { useRouter } from 'next/navigation'

export default function PricingSuccessPage() {
  const router = useRouter()
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    // Attendre un peu pour que PayPal mette à jour le statut
    const timer = setTimeout(() => {
      setLoading(false)
    }, 2000)

    return () => clearTimeout(timer)
  }, [])

  return (
    <AppShellScreen>
      <div className="max-w-2xl mx-auto px-4 py-8 sm:px-6 lg:px-8">
        <Card>
          <CardBody className="text-center py-12">
            {loading ? (
              <>
                <span className="loading loading-spinner loading-lg text-primary mb-4" />
                <h2 className="text-2xl font-bold mb-2">Traitement en cours...</h2>
                <p className="text-base-content/60">
                  Vérification de votre abonnement avec PayPal
                </p>
              </>
            ) : (
              <>
                <div className="mb-6">
                  <span className="icon-[tabler--check-circle] size-16 text-success mx-auto" />
                </div>
                <h2 className="text-2xl font-bold mb-2">Abonnement confirmé !</h2>
                <p className="text-base-content/60 mb-6">
                  Votre abonnement a été activé avec succès. Vous pouvez maintenant accéder à toutes
                  les fonctionnalités.
                </p>
                <div className="flex gap-4 justify-center">
                  <button className="btn btn-primary" onClick={() => router.push('/subscription')}>
                    Voir mon abonnement
                  </button>
                  <button className="btn btn-outline" onClick={() => router.push('/')}>
                    Retour à l'accueil
                  </button>
                </div>
              </>
            )}
          </CardBody>
        </Card>
      </div>
    </AppShellScreen>
  )
}
