'use client'

import { useState, useEffect, useRef, useMemo } from 'react'
import Link from 'next/link'
import { useAuth } from '@/contexts/AuthContext'
import { getSupabaseClient } from '@/lib/supabase/client'
import type { Tables } from '@/lib/supabase/database.types'

type AppBranding = Tables<'app_branding'>
const BRANDING_BUCKET = 'app-branding'

/**
 * Header - Navigation supérieure sticky
 * Inspiré du template FlyonUI Landing Page
 * 
 * @layer organisms
 * @responsive Desktop: nav complète, Mobile: burger menu
 * @dependencies Supabase Auth
 */

export interface HeaderProps {
  title?: string
}

export function Header({ title = 'Miyukini' }: HeaderProps) {
  const supabase = useMemo(() => getSupabaseClient(), [])
  const [isOpen, setIsOpen] = useState(false)
  const [dropdownOpen, setDropdownOpen] = useState(false)
  const dropdownRef = useRef<HTMLDivElement | null>(null)
  const { user, profile, isAuthenticated, isLoading, signOut } = useAuth()
  const [branding, setBranding] = useState<AppBranding | null>(null)

  // Charger le branding (titre + logo)
  useEffect(() => {
    const loadBranding = async () => {
      const { data } = await supabase.from('app_branding').select('*').eq('id', 'main').single()
      if (data) setBranding(data as AppBranding)
    }
    void loadBranding()
  }, [supabase])

  const displayTitle = useMemo(() => branding?.app_title ?? title, [branding, title])
  const logoUrl = useMemo(() => {
    if (!branding) return null
    if (branding.logo_url) return branding.logo_url
    if (branding.logo_path) return supabase.storage.from(BRANDING_BUCKET).getPublicUrl(branding.logo_path).data.publicUrl
    return null
  }, [branding, supabase])

  const handleSignOut = async () => {
    await signOut()
    setDropdownOpen(false)
  }

  useEffect(() => {
    if (!dropdownOpen) {
      return
    }

    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setDropdownOpen(false)
      }
    }

    document.addEventListener('mousedown', handleClickOutside)
    return () => {
      document.removeEventListener('mousedown', handleClickOutside)
    }
  }, [dropdownOpen])

  // Déterminer le nom à afficher
  const displayName = profile?.display_name || profile?.first_name || user?.email?.split('@')[0] || 'Utilisateur'
  const avatarUrl = profile?.avatar_url
  const isAdmin = profile?.role === 'admin' || profile?.role === 'super_admin'

  return (
    <header className="fixed top-0 left-0 right-0 z-50 border-b border-base-content/10 bg-base-100/90 backdrop-blur-sm shadow-sm">
      <nav className="navbar mx-auto max-w-screen-xl px-4 sm:px-6 lg:px-8 py-2 lg:py-0">
        <div className="w-full lg:flex lg:items-center lg:gap-2">
          {/* Logo & Mobile toggle */}
          <div className="navbar-start items-center justify-between max-lg:w-full">
            <Link href="/" className="flex items-center gap-3 text-xl font-semibold text-base-content">
              {logoUrl ? (
                // eslint-disable-next-line @next/next/no-img-element
                <img src={logoUrl} alt={displayTitle} className="size-9 rounded-lg object-contain" />
              ) : (
                <span className="text-primary">
                  <div className="bg-primary text-primary-content rounded-lg size-9 flex items-center justify-center">
                    <span className="icon-[tabler--brand-tabler] size-5" />
                  </div>
                </span>
              )}
              {displayTitle}
            </Link>
            <div className="flex items-center gap-4 lg:hidden">
              {isAuthenticated ? (
                <Link href="/profile" className="btn btn-ghost btn-circle btn-sm">
                  <span className="icon-[tabler--settings] size-5" />
                </Link>
              ) : (
                <Link href="/signin" className="btn btn-primary btn-sm">
                  Connexion
                </Link>
              )}
              <button 
                type="button" 
                className="btn btn-outline btn-secondary btn-square btn-sm"
                onClick={() => setIsOpen(!isOpen)}
              >
                <span className={`${isOpen ? 'icon-[tabler--x]' : 'icon-[tabler--menu-2]'} size-5`} />
              </button>
            </div>
          </div>

          {/* Navigation links */}
          <div className={`lg:navbar-center grow font-medium lg:flex ${isOpen ? 'block' : 'hidden'}`}>
            <div className="flex gap-6 text-base text-base-content max-lg:mt-3 max-lg:flex-col lg:items-center">
              <Link href="/" className="hover:text-primary transition-colors">
                Accueil
              </Link>
              <Link href="/mockcontent" className="hover:text-primary transition-colors">
                Contenu
              </Link>
              {isAdmin && (
                <Link href="/admin" className="hover:text-primary transition-colors">
                  Admin
                </Link>
              )}
              {isAuthenticated && (
                <Link href="/profile" className="hover:text-primary transition-colors lg:hidden">
                  Profil
                </Link>
              )}
            </div>
          </div>

          {/* Desktop CTA - Authentifié ou non */}
          <div className="navbar-end max-lg:hidden">
            {isAuthenticated ? (
              <div
                ref={dropdownRef}
                className={`dropdown dropdown-end relative ${dropdownOpen ? 'dropdown-open' : ''}`}
              >
                <button
                  tabIndex={0}
                  className="btn btn-ghost gap-2"
                  onClick={() => setDropdownOpen(prev => !prev)}
                >
                  {avatarUrl ? (
                    <div className="avatar">
                      <div className="w-8 rounded-full">
                        <img src={avatarUrl} alt={displayName} />
                      </div>
                    </div>
                  ) : (
                    <div className="avatar avatar-placeholder">
                      <div className="bg-primary text-primary-content rounded-full size-8 flex items-center justify-center text-sm">
                        {displayName.charAt(0).toUpperCase()}
                      </div>
                    </div>
                  )}
                  <span className="max-w-24 truncate">{displayName}</span>
                  <span className="icon-[tabler--chevron-down] size-4" />
                </button>
                {dropdownOpen && (
                  <ul
                    tabIndex={0}
                    className="dropdown-content menu rounded-box absolute right-0 top-full mt-2 w-56 bg-base-100/90 border border-base-content/10 shadow-lg"
                  >
                  <li className="dropdown-header">
                    <span className="block text-sm font-medium">{displayName}</span>
                    <span className="block text-xs text-base-content/60">{user?.email}</span>
                  </li>
                  <li className="dropdown-divider" role="separator"></li>
                  <li>
                    <Link href="/profile" className="dropdown-item">
                      <span className="icon-[tabler--user] size-5" />
                      Mon profil
                    </Link>
                  </li>
                  <li>
                    <Link href="/profile" className="dropdown-item">
                      <span className="icon-[tabler--settings] size-5" />
                      Paramètres
                    </Link>
                  </li>
                  {isAdmin && (
                    <>
                      <li className="dropdown-divider" role="separator"></li>
                      <li>
                        <Link href="/admin" className="dropdown-item">
                          <span className="icon-[tabler--dashboard] size-5" />
                          Administration
                        </Link>
                      </li>
                    </>
                  )}
                  <li className="dropdown-divider" role="separator"></li>
                  <li>
                    <button onClick={handleSignOut} className="dropdown-item text-error">
                      <span className="icon-[tabler--logout] size-5" />
                      Déconnexion
                    </button>
                  </li>
                </ul>
                )}
              </div>
            ) : (
              <div className="flex gap-2">
                <Link href="/signin" className="btn btn-ghost">
                  Connexion
                </Link>
                <Link href="/signup" className="btn btn-primary">
                  <span className="icon-[tabler--user-plus] size-5" />
                  S'inscrire
                </Link>
              </div>
            )}
          </div>
        </div>
      </nav>
    </header>
  )
}
