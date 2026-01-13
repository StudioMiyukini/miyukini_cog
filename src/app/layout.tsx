import { FC } from 'react'
import '@/components/layouts/globals.css'
import { FlyonUIProvider } from '@/components/providers/FlyonUIProvider'
import { CategoriesProvider } from '@/contexts/CategoriesContext'
import { AuthProvider } from '@/contexts/AuthContext'
import { ModulesProvider } from '@/contexts/ModulesContext'

export const metadata = {
  title: 'Miyukini Framework',
  description: 'Foundation pour le framework modulaire Miyukini',
}

// Forcer le rendu dynamique pour éviter les erreurs de pré-rendu avec Supabase
export const dynamic = 'force-dynamic'

const RootLayout: FC<{ children: React.ReactNode }> = ({ children }) => {
  return (
    <html lang="fr" suppressHydrationWarning data-theme="light">
      <head>
        <link rel="icon" href="/favicon.ico" />
      </head>
      <body suppressHydrationWarning>
        <AuthProvider>
          <ModulesProvider>
            <CategoriesProvider>
              <FlyonUIProvider>{children}</FlyonUIProvider>
            </CategoriesProvider>
          </ModulesProvider>
        </AuthProvider>
      </body>
    </html>
  )
}

export default RootLayout
