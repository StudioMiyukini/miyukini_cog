'use client'

import { useMemo, useState } from 'react'
import { Modal } from '@/components/molecules/modal'

export type IconPickerModalProps = {
  open: boolean
  title?: string
  value?: string
  onClose: () => void
  onSelect: (iconClass: string) => void
}

type IconOption = { label: string; iconClass: string; tags?: string[] }

const ICON_OPTIONS: IconOption[] = [
  { label: 'Accueil', iconClass: 'icon-[tabler--home]', tags: ['home'] },
  { label: 'Dashboard', iconClass: 'icon-[tabler--layout-dashboard]', tags: ['dashboard'] },
  { label: 'Explorer', iconClass: 'icon-[tabler--compass]', tags: ['explore'] },
  { label: 'Recherche', iconClass: 'icon-[tabler--search]', tags: ['search'] },
  { label: 'Favoris', iconClass: 'icon-[tabler--heart]', tags: ['heart', 'fav'] },
  { label: 'Messages', iconClass: 'icon-[tabler--message]', tags: ['chat', 'message'] },
  { label: 'Notifications', iconClass: 'icon-[tabler--bell]', tags: ['notif'] },
  { label: 'Paramètres', iconClass: 'icon-[tabler--settings]', tags: ['settings'] },
  { label: 'Aide', iconClass: 'icon-[tabler--help]', tags: ['help'] },
  { label: 'Utilisateur', iconClass: 'icon-[tabler--user]', tags: ['user', 'profile'] },
  { label: 'Utilisateurs', iconClass: 'icon-[tabler--users]', tags: ['users'] },
  { label: 'Admin', iconClass: 'icon-[tabler--shield-lock]', tags: ['admin', 'security'] },
  { label: 'Sécurité', iconClass: 'icon-[tabler--lock]', tags: ['lock'] },
  { label: 'Calendrier', iconClass: 'icon-[tabler--calendar]', tags: ['calendar'] },
  { label: 'Horloge', iconClass: 'icon-[tabler--clock]', tags: ['time'] },
  { label: 'Réservation', iconClass: 'icon-[tabler--calendar-event]', tags: ['booking', 'event'] },
  { label: 'Agenda', iconClass: 'icon-[tabler--calendar-time]', tags: ['agenda'] },
  { label: 'Boutique', iconClass: 'icon-[tabler--shopping-bag]', tags: ['shop'] },
  { label: 'Panier', iconClass: 'icon-[tabler--shopping-cart]', tags: ['cart'] },
  { label: 'Paiement', iconClass: 'icon-[tabler--credit-card]', tags: ['payment'] },
  { label: 'Devis', iconClass: 'icon-[tabler--file-invoice]', tags: ['quote', 'invoice'] },
  { label: 'Document', iconClass: 'icon-[tabler--file-text]', tags: ['doc'] },
  { label: 'Dossier', iconClass: 'icon-[tabler--folder]', tags: ['folder'] },
  { label: 'Photos', iconClass: 'icon-[tabler--photo]', tags: ['gallery', 'image'] },
  { label: 'Caméra', iconClass: 'icon-[tabler--camera]', tags: ['camera'] },
  { label: 'Localisation', iconClass: 'icon-[tabler--map-pin]', tags: ['map', 'location'] },
  { label: 'Navigation', iconClass: 'icon-[tabler--navigation]', tags: ['nav'] },
  { label: 'Statistiques', iconClass: 'icon-[tabler--chart-bar]', tags: ['stats', 'analytics'] },
  { label: 'Graphique', iconClass: 'icon-[tabler--chart-line]', tags: ['chart'] },
  { label: 'Monnaie', iconClass: 'icon-[tabler--currency-euro]', tags: ['money', 'euro'] },
  { label: 'Budget', iconClass: 'icon-[tabler--calculator]', tags: ['budget', 'calc'] },
  { label: 'Email', iconClass: 'icon-[tabler--mail]', tags: ['email'] },
  { label: 'Téléphone', iconClass: 'icon-[tabler--phone]', tags: ['phone'] },
  { label: 'Lien', iconClass: 'icon-[tabler--link]', tags: ['link'] },
  { label: 'Partager', iconClass: 'icon-[tabler--share]', tags: ['share'] },
  { label: 'Téléchargement', iconClass: 'icon-[tabler--download]', tags: ['download'] },
  { label: 'Upload', iconClass: 'icon-[tabler--upload]', tags: ['upload'] },
  { label: 'Plus', iconClass: 'icon-[tabler--plus]', tags: ['add'] },
  { label: 'Moins', iconClass: 'icon-[tabler--minus]', tags: ['remove'] },
  { label: 'Editer', iconClass: 'icon-[tabler--pencil]', tags: ['edit'] },
  { label: 'Valider', iconClass: 'icon-[tabler--check]', tags: ['check', 'ok'] },
  { label: 'Fermer', iconClass: 'icon-[tabler--x]', tags: ['close'] },
  { label: 'Supprimer', iconClass: 'icon-[tabler--trash]', tags: ['delete', 'trash'] },
  { label: 'Alerte', iconClass: 'icon-[tabler--alert-triangle]', tags: ['alert', 'warning'] },
  { label: 'Info', iconClass: 'icon-[tabler--info-circle]', tags: ['info'] },
  { label: 'Succès', iconClass: 'icon-[tabler--circle-check]', tags: ['success'] },
  { label: 'Erreur', iconClass: 'icon-[tabler--circle-x]', tags: ['error'] },
  { label: 'Cadenas', iconClass: 'icon-[tabler--shield-check]', tags: ['shield'] },
]

export function IconPickerModal({ open, title, value, onClose, onSelect }: IconPickerModalProps) {
  const [q, setQ] = useState('')

  const filtered = useMemo(() => {
    const query = q.trim().toLowerCase()
    if (!query) return ICON_OPTIONS
    return ICON_OPTIONS.filter((o) => {
      const hay = `${o.label} ${o.iconClass} ${(o.tags ?? []).join(' ')}`.toLowerCase()
      return hay.includes(query)
    })
  }, [q])

  return (
    <Modal
      open={open}
      title={title ?? 'Choisir une icône'}
      onClose={onClose}
      footer={
        <div className="flex items-center justify-between gap-3">
          <div className="text-xs text-base-content/60">{filtered.length} icônes</div>
          <button className="btn btn-ghost btn-sm" onClick={onClose}>
            Fermer
          </button>
        </div>
      }
    >
      <div className="flex items-center gap-2">
        <input
          className="input input-bordered w-full"
          placeholder="Rechercher (ex: home, calendar, user...)"
          value={q}
          onChange={(e) => setQ(e.target.value)}
        />
        {value ? (
          <div className="flex items-center gap-2 px-3 py-2 rounded-xl border border-base-300 bg-base-200/40">
            <span className={`${value} size-5 text-primary`} />
            <span className="text-xs text-base-content/60">actuelle</span>
          </div>
        ) : null}
      </div>

      <div className="mt-4 grid grid-cols-3 sm:grid-cols-5 md:grid-cols-7 gap-2">
        {filtered.map((o) => {
          const active = value === o.iconClass
          return (
            <button
              key={o.iconClass}
              type="button"
              className={`group rounded-xl border p-2 flex flex-col items-center justify-center gap-2 transition-colors
                ${active ? 'border-primary bg-primary/10' : 'border-base-300 bg-base-100 hover:bg-base-200/60'}
              `}
              onClick={() => onSelect(o.iconClass)}
              title={`${o.label} — ${o.iconClass}`}
            >
              <span className={`${o.iconClass} size-6 text-primary`} />
              <span className="text-[10px] text-base-content/60 truncate w-full">{o.label}</span>
            </button>
          )
        })}
      </div>

      <div className="mt-4 text-xs text-base-content/50">
        Astuce : tu peux aussi coller une valeur custom au format <code className="px-1">icon-[tabler--home]</code>.
      </div>
    </Modal>
  )
}

