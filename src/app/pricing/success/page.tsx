'use client'

import { useEffect, useState } from 'react'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { Card, CardBody } from '@/components/atoms/card'
import { useRouter, useSearchParams } from 'next/navigation'

export default function PricingSuccessPage() {
  const router = useRouter()
  const searchParams = useSearchParams()
  const [loading, setLoading] = useState(true)
  const paymentMethod = searchParams.get('method')

  useEffect(() => {
    // Attendre un peu pour que PayPal mette à jour le statut (si PayPal)
    const timer = setTimeout(() => {
      setLoading(false)
    }, paymentMethod === 'manual' ? 0 : 2000)

    return () => clearTimeout(timer)
  }, [paymentMethod])

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
            ) : paymentMethod === 'manual' ? (
              <>
                <div className="mb-6">
                  <span className="icon-[tabler--clock] size-16 text-warning mx-auto" />
                </div>
                <h2 className="text-2xl font-bold mb-2">Demande d'abonnement créée</h2>
                <p className="text-base-content/60 mb-6">
                  Votre demande d'abonnement avec paiement par virement bancaire a été créée. Un administrateur
                  confirmera votre paiement une fois le virement reçu. Vous recevrez une notification par email
                  une fois votre abonnement activé.
                </p>
                <div className="bg-base-200 p-4 rounded-lg mb-6 text-left">
                  <p className="text-sm font-semibold mb-2">Prochaines étapes :</p>
                  <ol className="text-sm text-base-content/70 space-y-1 list-decimal list-inside">
                    <li>Effectuez le virement bancaire selon les instructions qui vous seront envoyées</li>
                    <li>Attendez la confirmation par un administrateur</li>
                    <li>Vous recevrez un email de confirmation une fois le paiement validé</li>
                  </ol>
                </div>
                <div className="flex gap-4 justify-center">
                  <button className="btn btn-primary" onClick={() => router.push('/subscription')}>
                    Voir ma demande
                  </button>
                  <button className="btn btn-outline" onClick={() => router.push('/')}>
                    Retour à l'accueil
                  </button>
                </div>
              </>
            ) : (
              <>
                <div className="mb-6">
                  <span className="icon-[tabler--circle-check] size-16 text-success mx-auto" />
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
