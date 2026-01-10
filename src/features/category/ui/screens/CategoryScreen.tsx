'use client'

import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { BottomNav } from '@/components/navigation/BottomNav'
import { Card, CardBody } from '@/components/atoms/card'

/**
 * CategoryScreen - Écran générique pour les catégories
 * Template neutre réutilisable pour chaque catégorie
 * 
 * @layer screens (portable vers Android/iOS)
 * @contract ScreenContract { render, props, navigation }
 */

export interface CategoryScreenProps {
  id: string
  name: string
  iconClass: string
  description?: string
  onNavigate?: (path: string) => void
}

export function CategoryScreen({ 
  id,
  name, 
  iconClass,
  description = 'Cette section est en cours de développement.',
  onNavigate 
}: CategoryScreenProps) {
  return (
    <AppShellScreen headerTitle={name}>
      <ContentStack>
        {/* Header */}
        <div className="text-center pt-8 pb-4">
          <div className="mx-auto mb-4">
            <div className="bg-primary/10 text-primary rounded-2xl size-20 flex items-center justify-center mx-auto">
              <span className={`${iconClass} size-10`} />
            </div>
          </div>
          <h1 className="text-2xl font-bold text-base-content">{name}</h1>
          <p className="text-base-content/60 mt-2">{description}</p>
        </div>

        {/* Content placeholder */}
        <Card>
          <CardBody>
            <div className="flex flex-col items-center py-8 text-center">
              <div className="bg-base-200 rounded-full size-16 flex items-center justify-center mb-4">
                <span className="icon-[tabler--package] size-8 text-base-content/30" />
              </div>
              <h3 className="text-lg font-semibold text-base-content mb-2">
                Contenu à venir
              </h3>
              <p className="text-base-content/60 max-w-sm">
                Cette catégorie sera bientôt disponible avec du contenu personnalisé.
              </p>
            </div>
          </CardBody>
        </Card>

        {/* Quick info */}
        <div className="grid grid-cols-2 gap-4">
          <Card>
            <CardBody className="p-4 text-center">
              <span className="icon-[tabler--clock] size-6 text-primary mx-auto mb-2" />
              <p className="text-sm font-medium text-base-content">Bientôt disponible</p>
              <p className="text-xs text-base-content/50">En développement</p>
            </CardBody>
          </Card>
          <Card>
            <CardBody className="p-4 text-center">
              <span className="icon-[tabler--bell] size-6 text-primary mx-auto mb-2" />
              <p className="text-sm font-medium text-base-content">Notifications</p>
              <p className="text-xs text-base-content/50">Activez les alertes</p>
            </CardBody>
          </Card>
        </div>

        {/* Action */}
        <Card className="bg-gradient-to-r from-primary/5 to-secondary/5">
          <CardBody>
            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-medium text-base-content">Besoin d'aide ?</h3>
                <p className="text-sm text-base-content/60">Consultez notre documentation</p>
              </div>
              <button 
                className="btn btn-primary btn-sm"
                onClick={() => onNavigate?.('/admin')}
              >
                <span className="icon-[tabler--help] size-4" />
                Aide
              </button>
            </div>
          </CardBody>
        </Card>

        {/* Category ID for debugging */}
        <p className="text-xs text-base-content/30 text-center">
          ID: {id}
        </p>
      </ContentStack>
      <BottomNav />
    </AppShellScreen>
  )
}
