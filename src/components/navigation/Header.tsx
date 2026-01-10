'use client'

import { useState } from 'react'
import Link from 'next/link'

/**
 * Header - Navigation supérieure sticky
 * Inspiré du template FlyonUI Landing Page
 * 
 * @layer organisms
 * @responsive Desktop: nav complète, Mobile: burger menu
 */

export interface HeaderProps {
  title?: string
}

export function Header({ title = 'Miyukini' }: HeaderProps) {
  const [isOpen, setIsOpen] = useState(false)

  return (
    <header className="fixed top-0 left-0 right-0 z-50 border-b border-base-content/10 bg-base-100/95 backdrop-blur-sm">
      <nav className="navbar mx-auto max-w-screen-xl px-4 sm:px-6 lg:px-8">
        <div className="w-full lg:flex lg:items-center lg:gap-2">
          {/* Logo & Mobile toggle */}
          <div className="navbar-start items-center justify-between max-lg:w-full">
            <Link href="/" className="flex items-center gap-3 text-xl font-semibold text-base-content">
              <span className="text-primary">
                <div className="bg-primary text-primary-content rounded-lg size-9 flex items-center justify-center">
                  <span className="icon-[tabler--brand-tabler] size-5" />
                </div>
              </span>
              {title}
            </Link>
            <div className="flex items-center gap-4 lg:hidden">
              <Link href="/signin" className="btn btn-primary btn-sm">
                Connexion
              </Link>
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
            <div className="flex gap-6 text-base text-base-content max-lg:mt-4 max-lg:flex-col lg:items-center">
              <Link href="/" className="hover:text-primary transition-colors">
                Accueil
              </Link>
              <Link href="/mockcontent" className="hover:text-primary transition-colors">
                Contenu
              </Link>
              <Link href="/admin" className="hover:text-primary transition-colors">
                Admin
              </Link>
              <Link href="/profile" className="hover:text-primary transition-colors">
                Profil
              </Link>
            </div>
          </div>

          {/* Desktop CTA */}
          <div className="navbar-end max-lg:hidden">
            <Link href="/signin" className="btn btn-primary">
              <span className="icon-[tabler--login] size-5" />
              Connexion
            </Link>
          </div>
        </div>
      </nav>
    </header>
  )
}
