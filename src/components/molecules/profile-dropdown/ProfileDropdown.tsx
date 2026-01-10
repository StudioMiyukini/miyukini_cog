'use client'

import { useId } from 'react'
import { Avatar } from '@/components/atoms/avatar'

export interface ProfileMenuItem {
  icon: string
  label: string
  href?: string
  onClick?: () => void
  divider?: boolean
  danger?: boolean
}

export interface ProfileDropdownProps {
  user: {
    name: string
    email?: string
    role?: string
    avatar?: string
  }
  menuItems: ProfileMenuItem[]
  className?: string
}

/**
 * ProfileDropdown - Menu dropdown profil FlyonUI
 * 
 * @layer molecules
 * @tokens theme.colors.*, theme.spacing.*
 * @dependencies FlyonUI, Avatar
 * 
 * @example
 * ```tsx
 * <ProfileDropdown 
 *   user={{ name: 'John Doe', email: 'john@example.com', avatar: '/avatar.png' }}
 *   menuItems={[
 *     { icon: 'icon-[tabler--user]', label: 'Mon compte', href: '/account' },
 *     { icon: 'icon-[tabler--settings]', label: 'Paramètres', href: '/settings' },
 *     { divider: true },
 *     { icon: 'icon-[tabler--logout]', label: 'Déconnexion', onClick: () => {}, danger: true }
 *   ]}
 * />
 * ```
 */
export function ProfileDropdown({ user, menuItems, className = '' }: ProfileDropdownProps) {
  const dropdownId = useId()

  return (
    <div className={`dropdown relative inline-flex [--offset:21] ${className}`}>
      <button
        id={dropdownId}
        type="button"
        className="dropdown-toggle avatar"
        aria-haspopup="menu"
        aria-expanded="false"
        aria-label="Menu profil"
      >
        <span className="rounded-field size-9.5">
          {user.avatar ? (
            <img src={user.avatar} alt={user.name} />
          ) : (
            <span className="icon-[tabler--user] size-6" />
          )}
        </span>
      </button>
      <ul
        className="dropdown-menu dropdown-open:opacity-100 max-w-75 hidden w-full space-y-0.5"
        role="menu"
        aria-orientation="vertical"
        aria-labelledby={dropdownId}
      >
        {/* Header */}
        <li className="dropdown-header pt-4.5 mb-1 gap-4 px-5 pb-3.5">
          <Avatar src={user.avatar} size="md" status="online" />
          <div>
            <h6 className="text-base-content mb-0.5 font-semibold">{user.name}</h6>
            <p className="text-base-content/80 font-medium">{user.role || user.email}</p>
          </div>
        </li>

        {/* Menu items */}
        {menuItems.map((item, index) =>
          item.divider ? (
            <li key={`divider-${index}`}>
              <hr className="border-base-content/20 -mx-2 my-1" />
            </li>
          ) : (
            <li key={`item-${index}`} className={item.danger ? 'dropdown-footer p-2 pt-1' : ''}>
              {item.danger ? (
                <a
                  className="btn btn-text btn-error btn-block h-11 justify-start px-3 font-normal"
                  href={item.href || '#'}
                  onClick={item.onClick}
                >
                  <span className={`${item.icon} size-5`} />
                  {item.label}
                </a>
              ) : (
                <a
                  className="dropdown-item px-3"
                  href={item.href || '#'}
                  onClick={item.onClick}
                >
                  <span className={`${item.icon} size-5`} />
                  {item.label}
                </a>
              )}
            </li>
          )
        )}
      </ul>
    </div>
  )
}
