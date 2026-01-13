'use client'

import { useEffect, useMemo, useState } from 'react'
import { useRouter } from 'next/navigation'
import { AppShellScreen } from '@/components/layouts/AppShellScreen'
import { ContentStack } from '@/components/layouts/ContentStack'
import { BottomNav } from '@/components/navigation/BottomNav'
import { Card, CardBody, CardHeader } from '@/components/atoms/card'
import { Badge } from '@/components/atoms/badge'
import { useAuth } from '@/contexts/AuthContext'
import { getSupabaseClient } from '@/lib/supabase/client'

export function ProfileEditScreen() {
  const router = useRouter()
  const { user, profile, isAuthenticated, isLoading, refreshProfile } = useAuth()
  const supabase = useMemo(() => getSupabaseClient(), [])

  const [firstName, setFirstName] = useState('')
  const [lastName, setLastName] = useState('')
  const [phone, setPhone] = useState('')
  const [saving, setSaving] = useState(false)
  const [saved, setSaved] = useState(false)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (!isLoading && !isAuthenticated) {
      router.push('/signin')
    }
  }, [isAuthenticated, isLoading, router])

  useEffect(() => {
    setFirstName(profile?.first_name ?? '')
    setLastName(profile?.last_name ?? '')
    setPhone(profile?.phone ?? '')
  }, [profile?.first_name, profile?.last_name, profile?.phone])

  const save = async () => {
    if (!user) return
    setSaved(false)
    setError(null)

    const fn = firstName.trim()
    const ln = lastName.trim()
    const ph = phone.trim()

    if (!fn || !ln || !ph) {
      setError('Prénom, nom et téléphone sont obligatoires.')
      return
    }

    setSaving(true)
    try {
      const displayName = `${fn} ${ln}`.trim()
      const { error: upErr } = await supabase
        .from('profiles')
        .update({
          first_name: fn,
          last_name: ln,
          phone: ph,
          display_name: displayName,
        })
        .eq('id', user.id)
      if (upErr) throw upErr
      await refreshProfile()
      setSaved(true)
      setTimeout(() => setSaved(false), 1500)
    } catch (e: any) {
      setError(e?.message ?? 'Impossible de sauvegarder le profil.')
    } finally {
      setSaving(false)
    }
  }

  if (isLoading) {
    return (
      <AppShellScreen headerTitle="Profil">
        <ContentStack>
          <div className="flex items-center justify-center min-h-[50vh]">
            <span className="loading loading-spinner loading-lg text-primary" />
          </div>
        </ContentStack>
        <BottomNav />
      </AppShellScreen>
    )
  }

  if (!isAuthenticated || !user) return null

  return (
    <AppShellScreen headerTitle="Modifier le profil">
      <ContentStack>
        <Card>
          <CardHeader className="flex items-center justify-between">
            <span>Informations du compte</span>
            {saved && (
              <Badge variant="success" style="soft" size="sm">
                Enregistré
              </Badge>
            )}
          </CardHeader>
          <CardBody className="space-y-4">
            {error && (
              <div className="alert alert-error">
                <span className="icon-[tabler--alert-circle] size-5" />
                <span>{error}</span>
              </div>
            )}

            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              <div className="form-control">
                <label className="label">
                  <span className="label-text">Prénom</span>
                </label>
                <input className="input input-bordered w-full" value={firstName} onChange={(e) => setFirstName(e.target.value)} />
              </div>
              <div className="form-control">
                <label className="label">
                  <span className="label-text">Nom</span>
                </label>
                <input className="input input-bordered w-full" value={lastName} onChange={(e) => setLastName(e.target.value)} />
              </div>
            </div>

            <div className="form-control">
              <label className="label">
                <span className="label-text">Téléphone</span>
              </label>
              <input className="input input-bordered w-full" value={phone} onChange={(e) => setPhone(e.target.value)} autoComplete="tel" />
            </div>

            <div className="flex items-center justify-between gap-3">
              <button className="btn btn-ghost" onClick={() => router.push('/profile')} disabled={saving}>
                Retour
              </button>
              <button className="btn btn-primary" onClick={() => void save()} disabled={saving}>
                {saving ? 'Enregistrement…' : 'Enregistrer'}
              </button>
            </div>
          </CardBody>
        </Card>
      </ContentStack>
      <BottomNav />
    </AppShellScreen>
  )
}

