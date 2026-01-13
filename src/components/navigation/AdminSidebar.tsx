'use client'

import { useState, useEffect } from 'react'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { useAuth } from '@/contexts/AuthContext'

/**
 * AdminSidebar - Menu latéral du Back-Office
 * Inspiré du template FlyonUI Dashboard Free
 * 
 * @layer organisms
 * @responsive Desktop: sidebar fixe, Mobile: drawer overlay
 */

export interface AdminSidebarProps {
  isOpen: boolean
  onClose: () => void
}

// Menu sections configuration
const menuSections = [
  {
    title: null, // Pas de titre pour la première section
    items: [
      {
        id: 'dashboard',
        label: 'Dashboard',
        icon: 'icon-[tabler--dashboard]',
        href: '/admin',
        children: [
          { label: 'Aperçu', href: '/admin' },
          { label: 'Analytiques', href: '/admin/analytics', badge: 'Bientôt' },
        ]
      },
    ]
  },
  {
    title: 'Gestion',
    items: [
      {
        id: 'users',
        label: 'Utilisateurs',
        icon: 'icon-[tabler--users]',
        href: '/admin/users',
        children: [
          { label: 'Liste', href: '/admin/users' },
          { label: 'Rôles', href: '/admin/users/roles', badge: 'Bientôt' },
        ]
      },
      {
        id: 'categories',
        label: 'Catégories',
        icon: 'icon-[tabler--category]',
        href: '/admin/categories',
      },
      {
        id: 'budget',
        label: 'Budget',
        icon: 'icon-[tabler--wallet]',
        href: '/admin/budget',
      },
      {
        id: 'quotes',
        label: 'Devis',
        icon: 'icon-[tabler--file-invoice]',
        href: '/admin/quotes',
      },
      {
        id: 'content',
        label: 'Contenu',
        icon: 'icon-[tabler--article]',
        href: '/admin/content',
      },
    ]
  },
  {
    title: 'Configuration',
    items: [
      {
        id: 'settings',
        label: 'Paramètres',
        icon: 'icon-[tabler--settings]',
        href: '/admin/settings',
        children: [
          { label: 'Général', href: '/admin/settings' },
          { label: 'Notifications', href: '/admin/settings/notifications' },
          { label: 'Sécurité', href: '/admin/settings/security', badge: 'Bientôt' },
        ]
      },
      {
        id: 'modules',
        label: 'Modules',
        icon: 'icon-[tabler--puzzle]',
        href: '/admin/modules',
      },
      {
        id: 'appearance',
        label: 'Apparence',
        icon: 'icon-[tabler--palette]',
        href: '/admin/appearance',
        children: [
          { label: 'Branding', href: '/admin/appearance/branding' },
          { label: 'Homepage', href: '/admin/appearance/homepage' },
          { label: 'Thèmes', href: '/admin/appearance', badge: 'Bientôt' },
        ],
      },
    ]
  },
  {
    title: 'Support',
    items: [
      {
        id: 'help',
        label: 'Centre d\'aide',
        icon: 'icon-[tabler--help]',
        href: '/admin/help',
      },
      {
        id: 'docs',
        label: 'Documentation',
        icon: 'icon-[tabler--book]',
        href: '/admin/docs',
      },
    ]
  },
]

interface MenuItemProps {
  item: {
    id: string
    label: string
    icon: string
    href: string
    children?: { label: string; href: string; badge?: string }[]
  }
  pathname: string
}

function MenuItem({ item, pathname }: MenuItemProps) {
  const [isExpanded, setIsExpanded] = useState(
    item.children?.some(child => pathname === child.href) || pathname === item.href
  )
  const hasChildren = item.children && item.children.length > 0
  const isActive = pathname === item.href || item.children?.some(child => pathname === child.href)

  if (!hasChildren) {
    return (
      <li>
        <Link
          href={item.href}
          className={`flex items-center gap-3 px-3 py-2 rounded-lg transition-colors ${
            isActive 
              ? 'bg-primary/10 text-primary font-medium' 
              : 'text-base-content hover:bg-base-200'
          }`}
        >
          <span className={`${item.icon} size-5`} />
          <span>{item.label}</span>
        </Link>
      </li>
    )
  }

  return (
    <li className="accordion-item">
      <button
        className={`flex items-center gap-3 w-full px-3 py-2 rounded-lg transition-colors text-left ${
          isActive 
            ? 'bg-primary/10 text-primary font-medium' 
            : 'text-base-content hover:bg-base-200'
        }`}
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <span className={`${item.icon} size-5`} />
        <span className="flex-1">{item.label}</span>
        <span className={`icon-[tabler--chevron-right] size-4 transition-transform duration-200 ${isExpanded ? 'rotate-90' : ''}`} />
      </button>
      
      {isExpanded && (
        <ul className="mt-1 ml-4 pl-4 border-l border-base-300 space-y-1">
          {item.children?.map((child) => (
            <li key={child.href}>
              <Link
                href={child.href}
                className={`flex items-center justify-between px-3 py-1.5 rounded-lg text-sm transition-colors ${
                  pathname === child.href
                    ? 'text-primary font-medium'
                    : 'text-base-content/70 hover:text-base-content hover:bg-base-200'
                }`}
              >
                <span>{child.label}</span>
                {child.badge && (
                  <span className="badge badge-primary badge-soft badge-xs">{child.badge}</span>
                )}
              </Link>
            </li>
          ))}
        </ul>
      )}
    </li>
  )
}

export function AdminSidebar({ isOpen, onClose }: AdminSidebarProps) {
  const pathname = usePathname()
  const { profile } = useAuth()
  const isSuperAdmin = profile?.role === 'super_admin'

  // Fermer le sidebar mobile lors des changements de route
  useEffect(() => {
    onClose()
  }, [pathname]) // eslint-disable-line react-hooks/exhaustive-deps

  // Bloquer/débloquer le scroll du body quand le drawer mobile est ouvert
  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = 'hidden'
    } else {
      document.body.style.overflow = ''
    }
    
    // Cleanup au démontage
    return () => {
      document.body.style.overflow = ''
    }
  }, [isOpen])

  const sidebarContent = (
    <div className="flex h-full flex-col bg-base-100">
      {/* Header avec profil */}
      <div className="border-b border-base-300 p-4">
        <div className="flex items-center gap-3">
          <div className="avatar avatar-placeholder">
            <div className="bg-primary text-primary-content rounded-full size-12 flex items-center justify-center">
              <span className="icon-[tabler--user] size-6" />
            </div>
          </div>
          <div className="flex-1 min-w-0">
            <h3 className="font-semibold text-base-content truncate">Admin User</h3>
            <p className="text-sm text-base-content/60 truncate">admin@miyukini.com</p>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto p-3">
        {menuSections.map((section, sectionIndex) => (
          <div key={sectionIndex} className={sectionIndex > 0 ? 'mt-4' : ''}>
            {section.title && (
              <h4 className="px-3 py-2 text-xs font-semibold uppercase text-base-content/50 tracking-wider">
                {section.title}
              </h4>
            )}
            <ul className="space-y-1">
              {section.items.map((item) => (
                <MenuItem key={item.id} item={item} pathname={pathname} />
              ))}
            </ul>
          </div>
        ))}

        {/* Super Admin (visible uniquement super_admin) */}
        {isSuperAdmin && (
          <div className="mt-4">
            <h4 className="px-3 py-2 text-xs font-semibold uppercase text-base-content/50 tracking-wider">
              Super Admin
            </h4>
            <ul className="space-y-1">
              <MenuItem
                item={{
                  id: 'superadmin-integrations',
                  label: 'Intégrations',
                  icon: 'icon-[tabler--plug]',
                  href: '/admin/superadmin/integrations/smtp',
                  children: [
                    { label: 'SMTP', href: '/admin/superadmin/integrations/smtp' },
                    { label: 'PayPal', href: '/admin/superadmin/integrations/paypal' },
                  ],
                }}
                pathname={pathname}
              />
              <MenuItem
                item={{
                  id: 'superadmin-paywall',
                  label: 'Abonnements',
                  icon: 'icon-[tabler--credit-card]',
                  href: '/admin/superadmin/paywall/plans',
                  children: [
                    { label: 'Plans', href: '/admin/superadmin/paywall/plans' },
                    { label: 'Paiements manuels', href: '/admin/superadmin/paywall/manual-payments' },
                  ],
                }}
                pathname={pathname}
              />
            </ul>
          </div>
        )}
      </nav>

      {/* Footer avec workspace */}
      <div className="border-t border-base-300 p-3">
        <div className="flex items-center gap-3 px-3 py-2 bg-base-200 rounded-lg">
          <div className="bg-primary text-primary-content rounded-lg size-9 flex items-center justify-center">
            <span className="icon-[tabler--brand-tabler] size-5" />
          </div>
          <div className="flex-1 min-w-0">
            <p className="font-semibold text-sm text-base-content truncate">Miyukini</p>
            <p className="text-xs text-base-content/60">Workspace</p>
          </div>
          <span className="icon-[tabler--chevron-down] size-4 text-base-content/50" />
        </div>
      </div>
    </div>
  )

  return (
    <>
      {/* Desktop Sidebar - sous le header principal (top-16 = 64px) */}
      <aside className="hidden lg:flex lg:flex-col lg:w-72 lg:fixed lg:top-16 lg:bottom-0 lg:z-30 border-r border-base-300 bg-base-100">
        {sidebarContent}
      </aside>

      {/* Mobile Drawer - overlay */}
      {isOpen && (
        <>
          {/* Backdrop */}
          <div 
            className="fixed inset-0 bg-black/50 z-40 lg:hidden"
            onClick={onClose}
          />
          
          {/* Drawer - pleine hauteur sur mobile */}
          <aside className="fixed inset-y-0 left-0 w-72 z-50 lg:hidden transform transition-transform duration-300">
            {/* Close button */}
            <button
              className="absolute top-4 right-4 btn btn-ghost btn-circle btn-sm z-10"
              onClick={onClose}
            >
              <span className="icon-[tabler--x] size-5" />
            </button>
            {sidebarContent}
          </aside>
        </>
      )}
    </>
  )
}
