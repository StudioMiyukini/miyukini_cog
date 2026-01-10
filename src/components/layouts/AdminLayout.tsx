'use client'

import { useState } from 'react'
import Link from 'next/link'
import { AdminSidebar } from '@/components/navigation/AdminSidebar'
import { Header } from '@/components/navigation/Header'

/**
 * AdminLayout - Layout principal du Back-Office
 * Inspiré du template FlyonUI Dashboard Free
 * 
 * Structure: Header global + Sidebar gauche + Header admin + Contenu principal
 * 
 * @layer layouts
 * @responsive Desktop: sidebar + content, Mobile: drawer + full content
 */

export interface AdminLayoutProps {
  children: React.ReactNode
  title?: string
}

export function AdminLayout({ children, title = 'Dashboard' }: AdminLayoutProps) {
  const [sidebarOpen, setSidebarOpen] = useState(false)

  return (
    <div className="min-h-screen bg-base-200">
      {/* Header principal global */}
      <Header />

      {/* Sidebar - décalé sous le header */}
      <AdminSidebar 
        isOpen={sidebarOpen} 
        onClose={() => setSidebarOpen(false)} 
      />

      {/* Main Content Area - décalé sous le header */}
      <div className="lg:pl-72 flex flex-col min-h-screen pt-16">
        {/* Header admin secondaire */}
        <header className="sticky top-16 z-20 bg-base-100/95 backdrop-blur-sm border-b border-base-300">
          <div className="px-4 sm:px-6 lg:px-8">
            <div className="flex h-16 items-center justify-between">
              {/* Left side - Menu toggle & Search */}
              <div className="flex items-center gap-4">
                {/* Mobile menu button */}
                <button
                  type="button"
                  className="btn btn-ghost btn-square btn-sm lg:hidden"
                  onClick={() => setSidebarOpen(true)}
                >
                  <span className="icon-[tabler--menu-2] size-5" />
                </button>

                {/* Search */}
                <div className="hidden sm:flex items-center">
                  <div className="relative">
                    <span className="icon-[tabler--search] absolute left-3 top-1/2 -translate-y-1/2 size-4 text-base-content/50" />
                    <input
                      type="search"
                      placeholder="Rechercher..."
                      className="input input-sm pl-9 w-64 bg-base-200 border-none focus:bg-base-100"
                    />
                  </div>
                </div>
              </div>

              {/* Right side - Actions */}
              <div className="flex items-center gap-3">
                {/* Notifications */}
                <button className="btn btn-ghost btn-circle btn-sm relative">
                  <span className="icon-[tabler--bell] size-5" />
                  <span className="absolute top-1 right-1 size-2 bg-error rounded-full" />
                </button>

                {/* Quick Actions */}
                <div className="dropdown dropdown-end">
                  <button className="btn btn-ghost btn-circle btn-sm">
                    <span className="icon-[tabler--plus] size-5" />
                  </button>
                  <ul className="dropdown-menu dropdown-open:opacity-100 hidden min-w-48">
                    <li><a className="dropdown-item" href="#"><span className="icon-[tabler--user-plus] size-4" />Nouvel utilisateur</a></li>
                    <li><a className="dropdown-item" href="#"><span className="icon-[tabler--article] size-4" />Nouveau contenu</a></li>
                    <li><a className="dropdown-item" href="#"><span className="icon-[tabler--category] size-4" />Nouvelle catégorie</a></li>
                  </ul>
                </div>

                {/* Divider */}
                <div className="hidden sm:block w-px h-6 bg-base-300" />

                {/* Profile Dropdown */}
                <div className="dropdown dropdown-end">
                  <button className="btn btn-ghost btn-circle avatar">
                    <div className="bg-primary text-primary-content rounded-full size-8 flex items-center justify-center">
                      <span className="icon-[tabler--user] size-4" />
                    </div>
                  </button>
                  <ul className="dropdown-menu dropdown-open:opacity-100 hidden min-w-52">
                    <li className="dropdown-header gap-3 px-4 py-3">
                      <div className="avatar avatar-placeholder">
                        <div className="bg-primary text-primary-content rounded-full size-10 flex items-center justify-center">
                          <span className="icon-[tabler--user] size-5" />
                        </div>
                      </div>
                      <div>
                        <h6 className="font-semibold text-base-content">Admin User</h6>
                        <p className="text-xs text-base-content/60">SuperAdmin</p>
                      </div>
                    </li>
                    <li><hr className="border-base-300 my-1" /></li>
                    <li>
                      <Link href="/profile" className="dropdown-item">
                        <span className="icon-[tabler--user] size-4" />
                        Mon profil
                      </Link>
                    </li>
                    <li>
                      <Link href="/admin/settings" className="dropdown-item">
                        <span className="icon-[tabler--settings] size-4" />
                        Paramètres
                      </Link>
                    </li>
                    <li><hr className="border-base-300 my-1" /></li>
                    <li>
                      <Link href="/" className="dropdown-item">
                        <span className="icon-[tabler--home] size-4" />
                        Retour au site
                      </Link>
                    </li>
                    <li>
                      <a href="#" className="dropdown-item text-error">
                        <span className="icon-[tabler--logout] size-4" />
                        Déconnexion
                      </a>
                    </li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        </header>

        {/* Page Content */}
        <main className="flex-1 p-4 sm:p-6 lg:p-8">
          <div className="mx-auto max-w-7xl">
            {children}
          </div>
        </main>

        {/* Footer */}
        <footer className="border-t border-base-300 bg-base-100 px-4 sm:px-6 lg:px-8 py-4">
          <div className="mx-auto max-w-7xl flex flex-col sm:flex-row items-center justify-between gap-2 text-sm text-base-content/60">
            <p>© 2026 Miyukini Framework</p>
            <div className="flex items-center gap-4">
              <Link href="/admin/docs" className="hover:text-primary transition-colors">Documentation</Link>
              <Link href="/admin/help" className="hover:text-primary transition-colors">Support</Link>
            </div>
          </div>
        </footer>
      </div>
    </div>
  )
}
