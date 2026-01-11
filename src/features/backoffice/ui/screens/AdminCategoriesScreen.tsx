'use client'

import { useMemo, useState } from 'react'
import { AdminLayout } from '@/components/layouts/AdminLayout'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { IconPickerModal } from '@/components/molecules/icon-picker'
import { useCategories } from '@/contexts/CategoriesContext'
import { useRequireAdmin } from '../hooks/useRequireAdmin'

export function AdminCategoriesScreen() {
  const { isReady, authLoading } = useRequireAdmin()
  const { categories, enabledCategories, toggleCategory, updateCategoryDefinition, resetCategories, isLoading } =
    useCategories()

  const PAGE_OPTIONS: Array<{ label: string; value: string }> = [
    { label: 'Accueil (/) ', value: '/' },
    { label: 'Contenu (mock) (/mockcontent)', value: '/mockcontent' },
    { label: 'Booking (client) (/booking)', value: '/booking' },
    { label: 'Profil (/profile)', value: '/profile' },
    { label: 'Admin (/admin)', value: '/admin' },
    { label: 'Catégorie - Explorer', value: '/category/explorer' },
    { label: 'Catégorie - Favoris', value: '/category/favoris' },
    { label: 'Catégorie - Messages', value: '/category/messages' },
    { label: 'Catégorie - Notifications', value: '/category/notifications' },
    { label: 'Catégorie - Recherche', value: '/category/recherche' },
    { label: 'Catégorie - Paramètres', value: '/category/parametres' },
    { label: 'Catégorie - Aide', value: '/category/aide' },
    { label: 'Prestataire Booking (/pro/booking)', value: '/pro/booking' },
  ]
  const knownPaths = new Set(PAGE_OPTIONS.map((o) => o.value))
  const [iconPickerOpen, setIconPickerOpen] = useState(false)
  const [iconPickerCategoryId, setIconPickerCategoryId] = useState<string | null>(null)

  const iconPickerValue = useMemo(() => {
    if (!iconPickerCategoryId) return undefined
    return categories.find((c) => c.id === iconPickerCategoryId)?.iconClass
  }, [categories, iconPickerCategoryId])

  if (authLoading || !isReady) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center">
        <span className="loading loading-spinner loading-lg text-primary" />
      </div>
    )
  }

  return (
    <AdminLayout title="Catégories">
      <div className="flex items-start justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold text-base-content">Catégories</h1>
          <p className="text-base-content/60 mt-1">
            Gestion des catégories de navigation (persistées en base via Supabase).
          </p>
        </div>
        <button className="btn btn-ghost btn-sm" onClick={resetCategories}>
          <span className="icon-[tabler--refresh] size-4" />
          Réinitialiser
        </button>
      </div>

      <div className="mt-6 grid grid-cols-1 lg:grid-cols-3 gap-6">
        <Card>
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold">Aperçu</span>
          </CardHeader>
          <CardBody>
            <div className="flex flex-wrap gap-2">
              <Badge variant="neutral" style="soft" size="sm">
                {categories.length} catégories
              </Badge>
              <Badge variant="neutral" style="soft" size="sm">
                {enabledCategories.length} actives
              </Badge>
            </div>
            <p className="text-sm text-base-content/60 mt-3">
              Par défaut, seule « Accueil » est activée. Les préférences sont par utilisateur.
            </p>
          </CardBody>
        </Card>

        <Card className="lg:col-span-2">
          <CardHeader className="flex items-center justify-between">
            <span className="font-semibold">Activation</span>
            {isLoading && <span className="loading loading-spinner loading-sm text-primary" />}
          </CardHeader>
          <CardBody className="p-0">
            <div className="overflow-x-auto">
              <table className="table table-zebra">
                <thead>
                  <tr>
                    <th>Catégorie</th>
                    <th>Page affichée</th>
                    <th>Afficher</th>
                    <th>Dans ma nav</th>
                  </tr>
                </thead>
                <tbody>
                  {categories.map((cat) => (
                    <tr key={cat.id}>
                      <td className="text-base-content">
                        <div className="flex flex-col gap-2">
                          <div className="flex items-center gap-2">
                            <span className={`${cat.iconClass} size-5 text-primary`} />
                            <span className="text-xs text-base-content/50">Icône</span>
                          </div>
                          <div className="flex items-center gap-2">
                            <button
                              className="btn btn-outline btn-sm"
                              disabled={isLoading}
                              onClick={() => {
                                setIconPickerCategoryId(cat.id)
                                setIconPickerOpen(true)
                              }}
                            >
                              Choisir…
                            </button>
                            <input
                              className="input input-bordered input-sm w-full max-w-xs"
                              value={cat.iconClass}
                              disabled={isLoading}
                              onChange={(e) => void updateCategoryDefinition(cat.id, { iconClass: e.target.value })}
                              placeholder="icon-[tabler--home]"
                            />
                          </div>

                          <div className="mt-1">
                            <div className="text-xs text-base-content/50">Nom</div>
                            <input
                              className="input input-bordered input-sm w-full max-w-xs"
                              value={cat.name}
                              disabled={isLoading}
                              onChange={(e) => void updateCategoryDefinition(cat.id, { name: e.target.value })}
                              placeholder="Accueil"
                            />
                          </div>
                        </div>
                      </td>
                      <td>
                        <select
                          className="select select-bordered select-sm w-full max-w-xs"
                          value={cat.path}
                          disabled={isLoading}
                          onChange={(e) => void updateCategoryDefinition(cat.id, { path: e.target.value })}
                        >
                          {!knownPaths.has(cat.path) && (
                            <option value={cat.path}>Actuel (custom): {cat.path}</option>
                          )}
                          {PAGE_OPTIONS.map((opt) => (
                            <option key={opt.value} value={opt.value}>
                              {opt.label}
                            </option>
                          ))}
                        </select>
                        <div className="mt-1 text-xs text-base-content/50">{cat.path}</div>
                      </td>
                      <td>
                        <input
                          type="checkbox"
                          className="toggle toggle-primary"
                          checked={cat.visible}
                          onChange={() => void updateCategoryDefinition(cat.id, { visible: !cat.visible })}
                          disabled={isLoading}
                        />
                      </td>
                      <td>
                        <input
                          type="checkbox"
                          className="toggle toggle-primary"
                          checked={cat.enabled}
                          onChange={() => toggleCategory(cat.id)}
                          disabled={isLoading}
                        />
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </CardBody>
        </Card>
      </div>

      <IconPickerModal
        open={iconPickerOpen}
        value={iconPickerValue}
        onClose={() => {
          setIconPickerOpen(false)
          setIconPickerCategoryId(null)
        }}
        onSelect={(iconClass) => {
          if (!iconPickerCategoryId) return
          void updateCategoryDefinition(iconPickerCategoryId, { iconClass })
          setIconPickerOpen(false)
          setIconPickerCategoryId(null)
        }}
      />
    </AdminLayout>
  )
}

