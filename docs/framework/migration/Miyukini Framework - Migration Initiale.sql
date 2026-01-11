-- =============================================
-- MIYUKINI FRAMEWORK - MIGRATION INITIALE
-- Module: Compte Utilisateur
-- Version: 1.0.0
-- Date: 2026-01-10
-- =============================================
-- 
-- Cette migration crée toutes les tables, types, 
-- politiques RLS et triggers nécessaires pour le 
-- module de compte utilisateur du framework Miyukini.
--
-- À exécuter sur une instance Supabase vierge.
-- =============================================

-- =============================================
-- PARTIE 1: TYPES ENUM
-- =============================================

-- Enum pour les rôles utilisateur
CREATE TYPE public.user_role AS ENUM ('user', 'admin', 'super_admin');

-- Enum pour les tiers/niveaux d'abonnement
CREATE TYPE public.user_tier AS ENUM ('free', 'starter', 'pro', 'enterprise');

-- Enum pour les types de consentement RGPD
CREATE TYPE public.consent_type AS ENUM (
  'marketing',
  'analytics', 
  'service',
  'newsletter',
  'third_party'
);

-- =============================================
-- PARTIE 2: TABLE PROFILES
-- =============================================

-- Table profiles (extension de auth.users)
-- Stocke les informations de profil étendues
CREATE TABLE public.profiles (
  id UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,
  email TEXT,
  phone TEXT,
  first_name TEXT,
  last_name TEXT,
  display_name TEXT,
  avatar_url TEXT,
  role public.user_role DEFAULT 'user'::public.user_role NOT NULL,
  tier public.user_tier DEFAULT 'free'::public.user_tier NOT NULL,
  metadata JSONB DEFAULT '{}'::jsonb,
  email_verified BOOLEAN DEFAULT false,
  phone_verified BOOLEAN DEFAULT false,
  onboarding_completed BOOLEAN DEFAULT false,
  created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
  deleted_at TIMESTAMPTZ
);

-- Index pour optimiser les recherches
CREATE INDEX idx_profiles_email ON public.profiles(email);
CREATE INDEX idx_profiles_role ON public.profiles(role);
CREATE INDEX idx_profiles_tier ON public.profiles(tier);

-- Commentaire descriptif
COMMENT ON TABLE public.profiles IS 'Profils utilisateur étendus - Module Compte Utilisateur Miyukini';

-- =============================================
-- PARTIE 3: TABLE USER_CONSENTS (RGPD)
-- =============================================

-- Table user_consents pour la conformité RGPD
CREATE TABLE public.user_consents (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
  consent_type public.consent_type NOT NULL,
  granted BOOLEAN NOT NULL DEFAULT false,
  granted_at TIMESTAMPTZ,
  revoked_at TIMESTAMPTZ,
  ip_address INET,
  user_agent TEXT,
  created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
  
  -- Contrainte: un utilisateur ne peut avoir qu'un consentement par type
  UNIQUE(user_id, consent_type)
);

-- Index pour optimiser les requêtes
CREATE INDEX idx_user_consents_user_id ON public.user_consents(user_id);
CREATE INDEX idx_user_consents_type ON public.user_consents(consent_type);

-- Commentaire descriptif
COMMENT ON TABLE public.user_consents IS 'Consentements RGPD utilisateur - Module Compte Utilisateur Miyukini';

-- =============================================
-- PARTIE 4: RLS POLICIES - PROFILES
-- =============================================

-- Mini-fonctions utilitaires pour sécuriser les policies sans recursion
CREATE OR REPLACE FUNCTION public.get_current_profile_role()
RETURNS public.user_role
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = public
AS $$
  SELECT COALESCE(
    (SELECT p.role FROM public.profiles p WHERE p.id = auth.uid()),
    'user'::public.user_role
  );
$$;

CREATE OR REPLACE FUNCTION public.is_admin_user()
RETURNS boolean
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = public
AS $$
  SELECT public.get_current_profile_role() IN ('admin'::public.user_role, 'super_admin'::public.user_role);
$$;

CREATE OR REPLACE FUNCTION public.is_super_admin()
RETURNS boolean
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = public
AS $$
  SELECT public.get_current_profile_role() = 'super_admin'::public.user_role;
$$;

-- Activer Row Level Security sur profiles
ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;

-- Policy: Les utilisateurs peuvent voir leur propre profil
CREATE POLICY profiles_select_own ON public.profiles
  FOR SELECT
  USING (auth.uid() = id);

-- Policy: Les utilisateurs peuvent mettre à jour leur propre profil
CREATE POLICY profiles_update_own ON public.profiles
  FOR UPDATE
  USING (auth.uid() = id)
  WITH CHECK (auth.uid() = id);

-- Policy: Les admins peuvent voir tous les profils
CREATE POLICY profiles_select_admin ON public.profiles
  FOR SELECT
  USING (
    public.is_admin_user()
  );

-- Policy: Les super_admin peuvent tout modifier
CREATE POLICY profiles_all_super_admin ON public.profiles
  FOR ALL
  USING (
    public.is_super_admin()
  );

-- Policy: Insertion autorisée pour son propre profil
CREATE POLICY profiles_insert_service ON public.profiles
  FOR INSERT
  WITH CHECK (auth.uid() = id);

-- =============================================
-- PARTIE 5: RLS POLICIES - USER_CONSENTS
-- =============================================

-- Activer Row Level Security sur user_consents
ALTER TABLE public.user_consents ENABLE ROW LEVEL SECURITY;

-- Policy: Les utilisateurs peuvent voir leurs propres consentements
CREATE POLICY consents_select_own ON public.user_consents
  FOR SELECT
  USING (auth.uid() = user_id);

-- Policy: Les utilisateurs peuvent insérer leurs propres consentements
CREATE POLICY consents_insert_own ON public.user_consents
  FOR INSERT
  WITH CHECK (auth.uid() = user_id);

-- Policy: Les utilisateurs peuvent mettre à jour leurs propres consentements
CREATE POLICY consents_update_own ON public.user_consents
  FOR UPDATE
  USING (auth.uid() = user_id)
  WITH CHECK (auth.uid() = user_id);

-- Policy: Les admins peuvent lire tous les consentements (audit)
CREATE POLICY consents_select_admin ON public.user_consents
  FOR SELECT
  USING (
    EXISTS (
      SELECT 1 FROM public.profiles p 
      WHERE p.id = auth.uid() 
      AND p.role IN ('admin', 'super_admin')
    )
  );

-- =============================================
-- PARTIE 6: FONCTIONS ET TRIGGERS
-- =============================================

-- Fonction: Mise à jour automatique de updated_at
CREATE OR REPLACE FUNCTION public.handle_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Trigger updated_at sur profiles
CREATE TRIGGER profiles_updated_at
  BEFORE UPDATE ON public.profiles
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

-- Trigger updated_at sur user_consents
CREATE TRIGGER consents_updated_at
  BEFORE UPDATE ON public.user_consents
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_updated_at();

-- Fonction: Création automatique du profil à l'inscription
-- Ce trigger est déclenché quand un nouvel utilisateur s'inscrit
CREATE OR REPLACE FUNCTION public.handle_new_user()
RETURNS TRIGGER AS $$
BEGIN
  INSERT INTO public.profiles (id, email, display_name, email_verified)
  VALUES (
    NEW.id,
    NEW.email,
    COALESCE(NEW.raw_user_meta_data->>'display_name', split_part(NEW.email, '@', 1)),
    CASE WHEN NEW.email_confirmed_at IS NOT NULL THEN true ELSE false END
  );
  RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Trigger sur auth.users pour créer automatiquement le profil
CREATE TRIGGER on_auth_user_created
  AFTER INSERT ON auth.users
  FOR EACH ROW
  EXECUTE FUNCTION public.handle_new_user();

-- =============================================
-- PARTIE 7: COMPTE SUPER ADMIN PAR DÉFAUT
-- =============================================
-- 
-- ⚠️ IMPORTANT: Cette section crée un compte super admin
-- par défaut pour l'administration du framework.
-- 
-- Identifiants:
--   Email: miyukini@gmail.com
--   Mot de passe: 070287
--
-- ⚠️ SÉCURITÉ: Changer le mot de passe après le premier
-- déploiement en production !
-- =============================================

-- Création du compte super admin dans auth.users
-- Note: Le trigger on_auth_user_created créera automatiquement
-- l'entrée correspondante dans public.profiles
INSERT INTO auth.users (
  id,
  instance_id,
  email,
  encrypted_password,
  email_confirmed_at,
  raw_app_meta_data,
  raw_user_meta_data,
  aud,
  role,
  created_at,
  updated_at,
  confirmation_token,
  email_change,
  email_change_token_new,
  recovery_token
) VALUES (
  gen_random_uuid(),
  '00000000-0000-0000-0000-000000000000',
  'miyukini@gmail.com',
  crypt('070287', gen_salt('bf')),
  now(),
  '{"provider": "email", "providers": ["email"]}'::jsonb,
  '{"display_name": "Super Admin Miyukini"}'::jsonb,
  'authenticated',
  'authenticated',
  now(),
  now(),
  '',
  '',
  '',
  ''
)
ON CONFLICT (email) DO NOTHING;

-- Mise à jour du profil en super_admin
-- (exécuté après le trigger qui crée le profil)
UPDATE public.profiles
SET 
  role = 'super_admin',
  first_name = 'Super',
  last_name = 'Admin',
  display_name = 'Super Admin Miyukini',
  onboarding_completed = true
WHERE email = 'miyukini@gmail.com';

-- =============================================
-- FIN DE LA MIGRATION INITIALE
-- =============================================
