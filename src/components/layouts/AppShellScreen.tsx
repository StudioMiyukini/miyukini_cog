'use client'

import { Header } from '@/components/navigation/Header'

/**
 * AppShellScreen - Layout triptyque (Header / Body / BottomNav)
 * 
 * @layer layouts
 * @structure header (sticky top) + body (scrollable) + bottomnav (fixed bottom)
 */

export interface AppShellScreenProps {
  children: React.ReactNode
  headerTitle?: string
  showHeader?: boolean
}

export function AppShellScreen({ 
  children, 
  headerTitle = 'Miyukini',
  showHeader = true 
}: AppShellScreenProps) {
  return (
    <div className="app-shell min-h-screen bg-base-100">
      {/* Header sticky */}
      {showHeader && <Header title={headerTitle} />}
      
      {/* Body content */}
      <div className="app-shell__content">
        {children}
      </div>
    </div>
  )
}
