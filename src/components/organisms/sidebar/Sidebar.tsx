'use client'

import { useId, useState } from 'react'
import { Avatar } from '@/components/atoms/avatar'
import { Badge, BadgeVariant } from '@/components/atoms/badge'

export interface SidebarMenuItem {
  id: string
  icon?: string
  label: string
  href?: string
  badge?: {
    label: string
    variant?: BadgeVariant
  }
  children?: SidebarMenuItem[]
  active?: boolean
}

export interface SidebarSection {
  label?: string
  items: SidebarMenuItem[]
}

export interface SidebarProfile {
  name: string
  email?: string
  avatar?: string
  socialLinks?: { icon: string; href: string; label: string }[]
}

export interface SidebarProps {
  profile?: SidebarProfile
  sections: SidebarSection[]
  footer?: React.ReactNode
  className?: string
  onNavigate?: (href: string) => void
}

/**
 * Sidebar - Navigation latérale FlyonUI avec accordion
 * 
 * @layer organisms
 * @tokens theme.colors.*, theme.spacing.*
 * @dependencies FlyonUI, Avatar, Badge, Accordion
 * 
 * @example
 * ```tsx
 * <Sidebar 
 *   profile={{ name: 'John Doe', email: 'john@example.com' }}
 *   sections={[
 *     { label: 'Main', items: [{ id: 'dashboard', icon: 'icon-[tabler--dashboard]', label: 'Dashboard' }] }
 *   ]}
 * />
 * ```
 */
export function Sidebar({ profile, sections, footer, className = '', onNavigate }: SidebarProps) {
  const baseId = useId()
  const [openItems, setOpenItems] = useState<Set<string>>(new Set())

  const toggleItem = (id: string) => {
    setOpenItems((prev) => {
      const next = new Set(prev)
      if (next.has(id)) {
        next.delete(id)
      } else {
        next.add(id)
      }
      return next
    })
  }

  const renderMenuItem = (item: SidebarMenuItem, level = 0): React.ReactNode => {
    const hasChildren = item.children && item.children.length > 0
    const isOpen = openItems.has(item.id)
    const itemId = `${baseId}-${item.id}`
    const collapseId = `${itemId}-collapse`

    if (hasChildren) {
      return (
        <li key={item.id} className={`accordion-item ${item.active ? 'active' : ''}`} id={itemId}>
          <button
            className="accordion-toggle accordion-item-active:bg-neutral/10 inline-flex w-full items-center p-2 text-start text-sm font-normal"
            aria-controls={collapseId}
            aria-expanded={isOpen}
            onClick={() => toggleItem(item.id)}
          >
            {item.icon && <span className={`${item.icon} size-4.5`} />}
            <span className="grow">{item.label}</span>
            {item.badge && (
              <Badge variant={item.badge.variant || 'primary'} size="sm" style="soft" className="me-2">
                {item.badge.label}
              </Badge>
            )}
            <span className={`icon-[tabler--chevron-right] size-4.5 shrink-0 transition-transform duration-300 ${isOpen ? 'rotate-90' : ''}`} />
          </button>
          <div
            id={collapseId}
            className={`accordion-content mt-1 w-full overflow-hidden transition-[height] duration-300 ${!isOpen ? 'hidden' : 'block'}`}
            aria-labelledby={itemId}
            role="region"
          >
            <ul className="space-y-1">
              {item.children!.map((child) => renderMenuItem(child, level + 1))}
            </ul>
          </div>
        </li>
      )
    }

    return (
      <li key={item.id}>
        <a
          href={item.href || '#'}
          className={`inline-flex w-full items-center px-2 ${item.active ? 'menu-active' : ''}`}
          onClick={(e) => {
            if (onNavigate && item.href) {
              e.preventDefault()
              onNavigate(item.href)
            }
          }}
        >
          {level === 0 && item.icon && <span className={`${item.icon} size-4.5`} />}
          <span className="grow">{item.label}</span>
          {item.badge && (
            <Badge variant={item.badge.variant || 'primary'} size="sm" style="soft">
              {item.badge.label}
            </Badge>
          )}
        </a>
      </li>
    )
  }

  return (
    <aside
      id="layout-sidebar"
      className={`overlay overlay-open:translate-x-0 drawer drawer-start sm:w-75 inset-y-0 start-0 hidden h-full [--auto-close:lg] lg:z-50 lg:block lg:translate-x-0 lg:shadow-none ${className}`}
      aria-label="Sidebar"
      tabIndex={-1}
    >
      <div className="drawer-body border-base-content/20 h-full border-e p-0">
        <div className="flex h-full max-h-full flex-col">
          {/* Close button (mobile) */}
          <button
            type="button"
            className="btn btn-text btn-circle btn-sm absolute end-3 top-3 lg:hidden"
            aria-label="Fermer"
            data-overlay="#layout-sidebar"
          >
            <span className="icon-[tabler--x] size-4.5" />
          </button>

          {/* Profile section */}
          {profile && (
            <div className="text-base-content border-base-content/20 flex flex-col items-center gap-4 border-b px-4 py-6">
              <Avatar src={profile.avatar} size="xl" />
              <div className="text-center">
                <h3 className="text-base-content text-lg font-semibold">{profile.name}</h3>
                {profile.email && <p className="text-base-content/80">{profile.email}</p>}
              </div>
              {profile.socialLinks && (
                <div className="flex gap-3">
                  {profile.socialLinks.map((link, index) => (
                    <a key={index} href={link.href} className="link size-4.5" aria-label={link.label}>
                      <span className={`${link.icon} size-4.5`} />
                    </a>
                  ))}
                </div>
              )}
            </div>
          )}

          {/* Navigation */}
          <div className="h-full overflow-y-auto">
            <ul className="accordion menu menu-sm gap-1 p-3">
              {sections.map((section, sectionIndex) => (
                <li key={sectionIndex}>
                  {section.label && (
                    <span className="text-base-content/50 before:bg-base-content/20 mt-2 p-2 text-xs uppercase before:absolute before:-start-3 before:top-1/2 before:h-0.5 before:w-2.5">
                      {section.label}
                    </span>
                  )}
                  <ul className="space-y-1">
                    {section.items.map((item) => renderMenuItem(item))}
                  </ul>
                </li>
              ))}
            </ul>
          </div>

          {/* Footer */}
          {footer && <div className="p-2">{footer}</div>}
        </div>
      </div>
    </aside>
  )
}
