'use client'

import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { Card, CardBody } from '@/components/atoms/card'
import { useAuth } from '@/contexts/AuthContext'
import { useRequireEnabledModule } from '../hooks/useRequireEnabledModule'
import { SubscriptionGate } from '@/components/paywall/SubscriptionGate'
import { useRequireSubscription } from '@/hooks/useRequireSubscription'
import Link from 'next/link'

export function ProviderBookingDashboardScreen() {
  const { isLoading, enabled } = useRequireEnabledModule('booking', '/')
  const { isAuthenticated, profile } = useAuth()
  const { hasSubscription, subscription, isLoading: isLoadingSubscription } = useRequireSubscription({
    redirectToPaywall: false,
  })
  const mustDenyProviderBackOffice = profile?.role === 'admin' || profile?.role === 'super_admin'
  const displayName =
    profile?.display_name ??
    `${profile?.first_name ?? ''} ${profile?.last_name ?? ''}`.trim() ??
    null

  if (isLoading || isLoadingSubscription) {
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
    <SubscriptionGate
      message="Un abonnement actif est requis pour accéder à l'espace prestataire. Abonnez-vous pour commencer à gérer vos prestations."
      paywallPath="/pricing"
    >
      <AppShellScreen>
        <ContentStack>
          <div className="space-y-2">
            <h1 className="text-2xl font-bold text-base-content">Back-office prestataire</h1>
            <p className="text-base-content/60">
              {displayName ? (
                <>
                  Bonjour <span className="font-semibold text-base-content">{displayName}</span>. Ici tu gères tes prestations,
                  tes créneaux et tes réservations.
                </>
              ) : (
                <>Ici tu gères tes prestations, tes créneaux et tes réservations.</>
              )}
            </p>
          </div>

          {/* Avertissement si abonnement suspendu ou en attente */}
          {subscription && subscription.status !== 'ACTIVE' && (
            <div className="alert alert-warning">
              <span className="icon-[tabler--alert-triangle] size-5" />
              <div className="flex-1">
                <h3 className="font-semibold">
                  Abonnement {subscription.status === 'APPROVAL_PENDING' ? 'en attente' : subscription.status === 'SUSPENDED' ? 'suspendu' : 'non actif'}
                </h3>
                <p className="text-sm">
                  {subscription.status === 'APPROVAL_PENDING' && "Votre abonnement est en attente d'approbation PayPal."}
                  {subscription.status === 'SUSPENDED' && 'Votre abonnement est suspendu. Veuillez vérifier votre moyen de paiement.'}
                  {subscription.status === 'APPROVED' && 'Votre abonnement est approuvé et sera activé prochainement.'}
                </p>
              </div>
              <Link href="/subscription" className="btn btn-sm btn-outline">
                Voir mon abonnement
              </Link>
            </div>
          )}

          <Card>
          <CardBody className="space-y-5">
            <div className="space-y-2">
              <div className="text-sm font-semibold text-base-content">Démarrage rapide (recommandé)</div>
              <ol className="list-decimal pl-5 text-sm text-base-content/70 space-y-1">
                <li>
                  Renseigne ton <span className="font-medium">profil pro</span> (nom d’enseigne, adresse, horaires, photos).
                </li>
                <li>Crée tes <span className="font-medium">prestations</span> (durée, prix, actif/inactif).</li>
                <li>Définis une <span className="font-medium">semaine type</span> (tes horaires habituels).</li>
                <li>Ajoute tes <span className="font-medium">vacances</span> / indisponibilités.</li>
                <li>Génère / vérifie tes créneaux dans le <span className="font-medium">planning</span>.</li>
                <li>Surveille l’onglet <span className="font-medium">réservations</span> et réponds aux demandes.</li>
              </ol>
            </div>

            <div className="grid grid-cols-1 gap-3 lg:grid-cols-2">
              <div className="rounded-2xl border border-base-300 bg-base-100 p-4 lg:col-span-2">
                <div className="flex items-center justify-between gap-3">
                  <div className="min-w-0">
                    <div className="font-semibold text-base-content">Profil pro</div>
                    <div className="text-sm text-base-content/60">Ta fiche visible côté client (adresse, horaires, photos, réseaux).</div>
                  </div>
                  <Link href="/pro/booking/profile" className="btn btn-primary btn-sm shrink-0">
                    Ouvrir
                  </Link>
                </div>
                <ul className="mt-3 list-disc pl-5 text-sm text-base-content/70 space-y-1">
                  <li>Renseigne ton nom d’enseigne, une description et tes coordonnées.</li>
                  <li>Ajoute une adresse et des horaires d’ouverture.</li>
                  <li>Ajoute quelques photos pour rendre ta fiche plus attractive.</li>
                </ul>
              </div>

              <div className="rounded-2xl border border-base-300 bg-base-100 p-4">
                <div className="flex items-center justify-between gap-3">
                  <div className="min-w-0">
                    <div className="font-semibold text-base-content">Prestations</div>
                    <div className="text-sm text-base-content/60">Ce que tes clients peuvent réserver.</div>
                  </div>
                  <Link href="/pro/booking/services" className="btn btn-primary btn-sm shrink-0">
                    Ouvrir
                  </Link>
                </div>
                <ul className="mt-3 list-disc pl-5 text-sm text-base-content/70 space-y-1">
                  <li>Crée une prestation (ex: “Massage thaï”).</li>
                  <li>Active/désactive une prestation sans la supprimer.</li>
                  <li>Les prestations inactives ne sont plus proposées aux clients.</li>
                </ul>
              </div>

              <div className="rounded-2xl border border-base-300 bg-base-100 p-4">
                <div className="flex items-center justify-between gap-3">
                  <div className="min-w-0">
                    <div className="font-semibold text-base-content">Planning</div>
                    <div className="text-sm text-base-content/60">Tes créneaux visibles côté client.</div>
                  </div>
                  <Link href="/pro/booking/planning" className="btn btn-primary btn-sm shrink-0">
                    Ouvrir
                  </Link>
                </div>
                <ul className="mt-3 list-disc pl-5 text-sm text-base-content/70 space-y-1">
                  <li>
                    <span className="font-medium">Vert</span> = libre, <span className="font-medium">bleu</span> = réservé.
                  </li>
                  <li>Clique sur un créneau pour l’éditer (capacité, durée, prestations autorisées).</li>
                  <li>
                    Si un client a réservé, tu peux <span className="font-medium">annuler la réservation</span> pour
                    libérer le créneau.
                  </li>
                  <li>Seuls les créneaux “publiés” sont visibles côté client.</li>
                </ul>
              </div>

              <div className="rounded-2xl border border-base-300 bg-base-100 p-4">
                <div className="flex items-center justify-between gap-3">
                  <div className="min-w-0">
                    <div className="font-semibold text-base-content">Vacances</div>
                    <div className="text-sm text-base-content/60">Bloquer des jours/heures d’indisponibilité.</div>
                  </div>
                  <Link href="/pro/booking/time-off" className="btn btn-ghost btn-sm shrink-0">
                    Ouvrir
                  </Link>
                </div>
                <ul className="mt-3 list-disc pl-5 text-sm text-base-content/70 space-y-1">
                  <li>Ajoute une période d’absence (vacances, rendez-vous, fermeture).</li>
                  <li>Astuce : ajoute tes absences avant de générer en masse tes créneaux.</li>
                </ul>
              </div>

              <div className="rounded-2xl border border-base-300 bg-base-100 p-4">
                <div className="flex items-center justify-between gap-3">
                  <div className="min-w-0">
                    <div className="font-semibold text-base-content">Semaine type</div>
                    <div className="text-sm text-base-content/60">Générer des créneaux récurrents rapidement.</div>
                  </div>
                  <Link href="/pro/booking/week-templates" className="btn btn-ghost btn-sm shrink-0">
                    Ouvrir
                  </Link>
                </div>
                <ul className="mt-3 list-disc pl-5 text-sm text-base-content/70 space-y-1">
                  <li>Définis des blocs horaires (ex: lun 09:00–12:00, 14:00–18:00).</li>
                  <li>Génère ensuite tes créneaux sur une période (ex: 30 jours).</li>
                  <li>Les heures sont en fuseau <span className="font-medium">Europe/Paris</span>.</li>
                </ul>
              </div>

              <div className="rounded-2xl border border-base-300 bg-base-100 p-4 lg:col-span-2">
                <div className="flex items-center justify-between gap-3">
                  <div className="min-w-0">
                    <div className="font-semibold text-base-content">Réservations</div>
                    <div className="text-sm text-base-content/60">Voir qui a réservé, gérer les demandes.</div>
                  </div>
                  <Link href="/pro/booking/bookings" className="btn btn-ghost btn-sm shrink-0">
                    Ouvrir
                  </Link>
                </div>
                <ul className="mt-3 list-disc pl-5 text-sm text-base-content/70 space-y-1">
                  <li>Retrouve les réservations “en attente” et “confirmées”.</li>
                  <li>Accède aux coordonnées du client (nom, email, téléphone si renseigné).</li>
                  <li>En cas d’annulation, libère le créneau depuis l’écran Planning.</li>
                </ul>
              </div>
            </div>

            <div className="rounded-2xl border border-base-300 bg-base-200/40 p-4">
              <div className="text-sm font-semibold text-base-content">Bon à savoir</div>
              <ul className="mt-2 list-disc pl-5 text-sm text-base-content/70 space-y-1">
                <li>
                  Si tes créneaux n’apparaissent pas côté client, vérifie qu’ils sont <span className="font-medium">publiés</span>{' '}
                  et qu’ils ont au moins une prestation autorisée.
                </li>
                <li>Les accès sont filtrés automatiquement (sécurité/RLS) : tu ne vois que tes données.</li>
              </ul>
            </div>
          </CardBody>
        </Card>
      </ContentStack>
    </AppShellScreen>
    </SubscriptionGate>
  )
}

