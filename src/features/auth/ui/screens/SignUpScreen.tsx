'use client'

import { useState, useEffect } from 'react'
import Link from 'next/link'
import { useRouter } from 'next/navigation'
import { useAuth } from '@/contexts/AuthContext'

/**
 * SignUpScreen - Écran d'inscription
 * 
 * @layer screens (portable vers Android/iOS)
 * @contract ScreenContract { render, props, navigation }
 * @dependencies FlyonUI, Supabase Auth
 */

export interface SignUpScreenProps {
  onSignUp?: (data: { name: string; email: string; password: string }) => Promise<void>
  onNavigateToSignIn?: () => void
}

export function SignUpScreen({ onSignUp, onNavigateToSignIn }: SignUpScreenProps) {
  const router = useRouter()
  const { signUp, isAuthenticated, isLoading: authLoading } = useAuth()
  
  const [firstName, setFirstName] = useState('')
  const [lastName, setLastName] = useState('')
  const [phone, setPhone] = useState('')
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [confirmPassword, setConfirmPassword] = useState('')
  const [showPassword, setShowPassword] = useState(false)
  const [acceptTerms, setAcceptTerms] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [success, setSuccess] = useState(false)

  // Rediriger si déjà connecté
  useEffect(() => {
    if (isAuthenticated && !authLoading) {
      router.push('/')
    }
  }, [isAuthenticated, authLoading, router])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    // Validation
    if (!firstName.trim() || !lastName.trim() || !phone.trim()) {
      setError('Prénom, nom et téléphone sont obligatoires.')
      return
    }

    if (password !== confirmPassword) {
      setError('Les mots de passe ne correspondent pas.')
      return
    }

    if (password.length < 8) {
      setError('Le mot de passe doit contenir au moins 8 caractères.')
      return
    }

    if (!acceptTerms) {
      setError('Vous devez accepter les conditions d\'utilisation.')
      return
    }

    setIsLoading(true)

    try {
      if (onSignUp) {
        await onSignUp({ name: `${firstName} ${lastName}`.trim(), email, password })
      } else {
        // Inscription via Supabase
        const { error: authError } = await signUp(email, password, {
          firstName: firstName.trim(),
          lastName: lastName.trim(),
          phone: phone.trim(),
        })
        
        if (authError) {
          if (authError.message.includes('already registered')) {
            setError('Cet email est déjà utilisé.')
          } else {
            setError(authError.message)
          }
          return
        }
        
        // Afficher le message de succès
        setSuccess(true)
      }
    } catch (err) {
      setError('Une erreur est survenue. Veuillez réessayer.')
    } finally {
      setIsLoading(false)
    }
  }

  // Écran de succès
  if (success) {
    return (
      <div className="min-h-screen bg-base-200 flex items-center justify-center p-4">
        <div className="card bg-base-100 shadow-xl w-full max-w-md">
          <div className="card-body text-center">
            <div className="avatar avatar-placeholder mx-auto mb-4">
              <div className="bg-success text-success-content rounded-full size-16 flex items-center justify-center">
                <span className="icon-[tabler--check] size-8" />
              </div>
            </div>
            <h1 className="text-2xl font-bold text-base-content">Compte créé !</h1>
            <p className="text-base-content/60 mt-2">
              Votre compte a été créé avec succès. Vous pouvez maintenant vous connecter.
            </p>
            <Link href="/signin" className="btn btn-primary mt-6">
              <span className="icon-[tabler--login] size-5" />
              Se connecter
            </Link>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-base-200 flex items-center justify-center p-4">
      <div className="card bg-base-100 shadow-xl w-full max-w-md">
        <div className="card-body">
          {/* Header */}
          <div className="text-center mb-6">
            <div className="avatar avatar-placeholder mx-auto mb-4">
              <div className="bg-primary text-primary-content rounded-full size-16 flex items-center justify-center">
                <span className="icon-[tabler--user-plus] size-8" />
              </div>
            </div>
            <h1 className="text-2xl font-bold text-base-content">Créer un compte</h1>
            <p className="text-base-content/60 mt-2">
              Rejoignez-nous et commencez votre aventure !
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
            {/* Identité */}
            <div className="form-control">
              <label className="label">
                <span className="label-text">Identité</span>
              </label>
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-2">
                <div className="input input-bordered flex items-center gap-2">
                  <span className="icon-[tabler--user] size-5 text-base-content/50" />
                  <input
                    type="text"
                    placeholder="Prénom"
                    className="grow bg-transparent border-none focus:outline-none"
                    value={firstName}
                    onChange={(e) => setFirstName(e.target.value)}
                    required
                    autoComplete="given-name"
                  />
                </div>
                <div className="input input-bordered flex items-center gap-2">
                  <span className="icon-[tabler--id] size-5 text-base-content/50" />
                  <input
                    type="text"
                    placeholder="Nom"
                    className="grow bg-transparent border-none focus:outline-none"
                    value={lastName}
                    onChange={(e) => setLastName(e.target.value)}
                    required
                    autoComplete="family-name"
                  />
                </div>
              </div>
            </div>

            {/* Téléphone */}
            <div className="form-control">
              <label className="label" htmlFor="phone">
                <span className="label-text">Téléphone</span>
              </label>
              <div className="input input-bordered flex items-center gap-2">
                <span className="icon-[tabler--phone] size-5 text-base-content/50" />
                <input
                  type="tel"
                  id="phone"
                  placeholder="06 12 34 56 78"
                  className="grow bg-transparent border-none focus:outline-none"
                  value={phone}
                  onChange={(e) => setPhone(e.target.value)}
                  required
                  autoComplete="tel"
                />
              </div>
            </div>

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
                  placeholder="Min. 8 caractères"
                  className="grow bg-transparent border-none focus:outline-none"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  required
                  minLength={8}
                  autoComplete="new-password"
                />
                <button
                  type="button"
                  className="btn btn-ghost btn-circle btn-xs"
                  onClick={() => setShowPassword(!showPassword)}
                >
                  <span className={`${showPassword ? 'icon-[tabler--eye-off]' : 'icon-[tabler--eye]'} size-5`} />
                </button>
              </div>
            </div>

            {/* Confirm Password */}
            <div className="form-control">
              <label className="label" htmlFor="confirmPassword">
                <span className="label-text">Confirmer le mot de passe</span>
              </label>
              <div className="input input-bordered flex items-center gap-2">
                <span className="icon-[tabler--lock-check] size-5 text-base-content/50" />
                <input
                  type={showPassword ? 'text' : 'password'}
                  id="confirmPassword"
                  placeholder="Confirmez votre mot de passe"
                  className="grow bg-transparent border-none focus:outline-none"
                  value={confirmPassword}
                  onChange={(e) => setConfirmPassword(e.target.value)}
                  required
                  autoComplete="new-password"
                />
              </div>
            </div>

            {/* Terms */}
            <label className="label cursor-pointer justify-start gap-3">
              <input
                type="checkbox"
                className="checkbox checkbox-primary checkbox-sm"
                checked={acceptTerms}
                onChange={(e) => setAcceptTerms(e.target.checked)}
                required
              />
              <span className="label-text">
                J'accepte les{' '}
                <Link href="/terms" className="link link-primary">conditions d'utilisation</Link>
                {' '}et la{' '}
                <Link href="/privacy" className="link link-primary">politique de confidentialité</Link>
              </span>
            </label>

            {/* Submit */}
            <button
              type="submit"
              className={`btn btn-primary btn-block ${isLoading ? 'loading' : ''}`}
              disabled={isLoading}
            >
              {isLoading ? (
                <>
                  <span className="loading loading-spinner loading-sm" />
                  Création...
                </>
              ) : (
                <>
                  <span className="icon-[tabler--user-plus] size-5" />
                  Créer mon compte
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

          {/* Sign In Link */}
          <p className="text-center text-base-content/60 mt-6">
            Déjà un compte ?{' '}
            {onNavigateToSignIn ? (
              <button onClick={onNavigateToSignIn} className="link link-primary font-medium">
                Se connecter
              </button>
            ) : (
              <Link href="/signin" className="link link-primary font-medium">
                Se connecter
              </Link>
            )}
          </p>
        </div>
      </div>
    </div>
  )
}
