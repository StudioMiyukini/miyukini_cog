'use client'

import { useState } from 'react'
import Link from 'next/link'

/**
 * SignInScreen - Écran de connexion
 * 
 * @layer screens (portable vers Android/iOS)
 * @contract ScreenContract { render, props, navigation }
 * @dependencies FlyonUI
 * 
 * Portabilité native:
 * - Aucune dépendance Next.js directe (Link remplaçable)
 * - État géré localement
 * - Callbacks pour navigation
 */

export interface SignInScreenProps {
  onSignIn?: (email: string, password: string) => Promise<void>
  onNavigateToSignUp?: () => void
  onNavigateToForgotPassword?: () => void
}

export function SignInScreen({ 
  onSignIn, 
  onNavigateToSignUp,
  onNavigateToForgotPassword 
}: SignInScreenProps) {
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [showPassword, setShowPassword] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)
    setIsLoading(true)

    try {
      if (onSignIn) {
        await onSignIn(email, password)
      } else {
        // Mock sign in pour démo
        console.log('Sign in:', { email, password })
        await new Promise(resolve => setTimeout(resolve, 1000))
      }
    } catch (err) {
      setError('Identifiants incorrects. Veuillez réessayer.')
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className="min-h-screen bg-base-200 flex items-center justify-center p-4">
      <div className="card bg-base-100 shadow-xl w-full max-w-md">
        <div className="card-body">
          {/* Header */}
          <div className="text-center mb-6">
            <div className="avatar avatar-placeholder mx-auto mb-4">
              <div className="bg-primary text-primary-content rounded-full size-16 flex items-center justify-center">
                <span className="icon-[tabler--login] size-8" />
              </div>
            </div>
            <h1 className="text-2xl font-bold text-base-content">Connexion</h1>
            <p className="text-base-content/60 mt-2">
              Bienvenue ! Connectez-vous à votre compte.
            </p>
          </div>

          {/* Error Alert */}
          {error && (
            <div className="alert alert-error mb-4">
              <span className="icon-[tabler--alert-circle] size-5" />
              <span>{error}</span>
            </div>
          )}

          {/* Form */}
          <form onSubmit={handleSubmit} className="space-y-4">
            {/* Email */}
            <div className="form-control">
              <label className="label" htmlFor="email">
                <span className="label-text">Email</span>
              </label>
              <div className="input input-bordered flex items-center gap-2">
                <span className="icon-[tabler--mail] size-5 text-base-content/50" />
                <input
                  type="email"
                  id="email"
                  placeholder="vous@exemple.com"
                  className="grow bg-transparent border-none focus:outline-none"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  required
                  autoComplete="email"
                />
              </div>
            </div>

            {/* Password */}
            <div className="form-control">
              <label className="label" htmlFor="password">
                <span className="label-text">Mot de passe</span>
              </label>
              <div className="input input-bordered flex items-center gap-2">
                <span className="icon-[tabler--lock] size-5 text-base-content/50" />
                <input
                  type={showPassword ? 'text' : 'password'}
                  id="password"
                  placeholder="••••••••"
                  className="grow bg-transparent border-none focus:outline-none"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  required
                  autoComplete="current-password"
                />
                <button
                  type="button"
                  className="btn btn-ghost btn-circle btn-xs"
                  onClick={() => setShowPassword(!showPassword)}
                  aria-label={showPassword ? 'Masquer' : 'Afficher'}
                >
                  <span className={`${showPassword ? 'icon-[tabler--eye-off]' : 'icon-[tabler--eye]'} size-5`} />
                </button>
              </div>
            </div>

            {/* Remember & Forgot */}
            <div className="flex items-center justify-between">
              <label className="label cursor-pointer gap-2">
                <input type="checkbox" className="checkbox checkbox-primary checkbox-sm" />
                <span className="label-text">Se souvenir de moi</span>
              </label>
              {onNavigateToForgotPassword ? (
                <button
                  type="button"
                  onClick={onNavigateToForgotPassword}
                  className="link link-primary text-sm"
                >
                  Mot de passe oublié ?
                </button>
              ) : (
                <Link href="/forgot-password" className="link link-primary text-sm">
                  Mot de passe oublié ?
                </Link>
              )}
            </div>

            {/* Submit */}
            <button
              type="submit"
              className={`btn btn-primary btn-block ${isLoading ? 'loading' : ''}`}
              disabled={isLoading}
            >
              {isLoading ? (
                <>
                  <span className="loading loading-spinner loading-sm" />
                  Connexion...
                </>
              ) : (
                <>
                  <span className="icon-[tabler--login] size-5" />
                  Se connecter
                </>
              )}
            </button>
          </form>

          {/* Divider */}
          <div className="divider my-6">ou</div>

          {/* Social Login */}
          <div className="flex gap-3">
            <button className="btn btn-outline flex-1">
              <span className="icon-[tabler--brand-google] size-5" />
              Google
            </button>
            <button className="btn btn-outline flex-1">
              <span className="icon-[tabler--brand-github] size-5" />
              GitHub
            </button>
          </div>

          {/* Sign Up Link */}
          <p className="text-center text-base-content/60 mt-6">
            Pas encore de compte ?{' '}
            {onNavigateToSignUp ? (
              <button onClick={onNavigateToSignUp} className="link link-primary font-medium">
                Créer un compte
              </button>
            ) : (
              <Link href="/signup" className="link link-primary font-medium">
                Créer un compte
              </Link>
            )}
          </p>
        </div>
      </div>
    </div>
  )
}
