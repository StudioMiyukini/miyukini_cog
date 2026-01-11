# Miyukini Framework - Guide des Migrations

## Contexte

Ce dossier contient les migrations SQL nécessaires pour initialiser une base de données Supabase compatible avec le Miyukini Framework.

## Portée / Scope

- **Module concerné** : Compte Utilisateur
- **Tables créées** : `profiles`, `user_consents`
- **Fonctionnalités** : Authentification, Profils, Rôles, Tiers, Consentements RGPD

---

## Structure des Migrations

### 1. Migration Initiale

**Fichier** : `Miyukini Framework - Migration Initiale.sql`

Cette migration crée le module Compte Utilisateur :

| Partie | Description |
|--------|-------------|
| 1 | Types ENUM (`user_role`, `user_tier`, `consent_type`) |
| 2 | Table `profiles` avec index |
| 3 | Table `user_consents` avec index |
| 4 | Politiques RLS pour `profiles` |
| 5 | Politiques RLS pour `user_consents` |
| 6 | Fonctions et Triggers automatiques |
| 7 | Compte Super Admin par défaut |

### 2. Migration Categories

**Fichier** : `Miyukini Framework - Migration Categories.sql`

Cette migration crée le module Catégories de Navigation :

| Partie | Description |
|--------|-------------|
| 1 | Table `categories` (8 catégories globales) |
| 2 | Table `user_category_preferences` |
| 3 | Politiques RLS |
| 4 | Triggers automatiques |
| 5 | Données initiales (8 catégories par défaut) |

---

## Tables Créées

### `public.profiles`

Extension de `auth.users` avec informations de profil.

| Colonne | Type | Description |
|---------|------|-------------|
| `id` | UUID | Clé primaire, référence `auth.users` |
| `email` | TEXT | Email de l'utilisateur |
| `phone` | TEXT | Téléphone |
| `first_name` | TEXT | Prénom |
| `last_name` | TEXT | Nom |
| `display_name` | TEXT | Nom d'affichage |
| `avatar_url` | TEXT | URL de l'avatar |
| `role` | user_role | Rôle (user/admin/super_admin) |
| `tier` | user_tier | Niveau (free/starter/pro/enterprise) |
| `metadata` | JSONB | Données supplémentaires |
| `email_verified` | BOOLEAN | Email vérifié |
| `phone_verified` | BOOLEAN | Téléphone vérifié |
| `onboarding_completed` | BOOLEAN | Onboarding terminé |
| `created_at` | TIMESTAMPTZ | Date de création |
| `updated_at` | TIMESTAMPTZ | Dernière mise à jour |
| `deleted_at` | TIMESTAMPTZ | Soft delete |

### `public.user_consents`

Gestion des consentements RGPD.

| Colonne | Type | Description |
|---------|------|-------------|
| `id` | UUID | Clé primaire |
| `user_id` | UUID | Référence utilisateur |
| `consent_type` | consent_type | Type de consentement |
| `granted` | BOOLEAN | Consentement accordé |
| `granted_at` | TIMESTAMPTZ | Date d'accord |
| `revoked_at` | TIMESTAMPTZ | Date de révocation |
| `ip_address` | INET | Adresse IP |
| `user_agent` | TEXT | Agent utilisateur |
| `created_at` | TIMESTAMPTZ | Date de création |
| `updated_at` | TIMESTAMPTZ | Dernière mise à jour |

### `public.categories`

Catégories de navigation globales.

| Colonne | Type | Description |
|---------|------|-------------|
| `id` | TEXT | Clé primaire (ex: cat_1) |
| `name` | TEXT | Nom affiché |
| `icon_class` | TEXT | Classe CSS de l'icône |
| `path` | TEXT | Route de navigation |
| `is_default` | BOOLEAN | Activé par défaut |
| `sort_order` | INTEGER | Ordre d'affichage |
| `created_at` | TIMESTAMPTZ | Date de création |
| `updated_at` | TIMESTAMPTZ | Dernière mise à jour |

### `public.user_category_preferences`

Préférences de catégories par utilisateur.

| Colonne | Type | Description |
|---------|------|-------------|
| `id` | UUID | Clé primaire |
| `user_id` | UUID | Référence utilisateur |
| `category_id` | TEXT | Référence catégorie |
| `enabled` | BOOLEAN | Catégorie activée |
| `custom_order` | INTEGER | Ordre personnalisé |
| `created_at` | TIMESTAMPTZ | Date de création |
| `updated_at` | TIMESTAMPTZ | Dernière mise à jour |

---

## Types ENUM

### `user_role`
```sql
'user' | 'admin' | 'super_admin'
```

### `user_tier`
```sql
'free' | 'starter' | 'pro' | 'enterprise'
```

### `consent_type`
```sql
'marketing' | 'analytics' | 'service' | 'newsletter' | 'third_party'
```

---

## Politiques RLS (Row Level Security)

### Profiles

| Policy | Action | Description |
|--------|--------|-------------|
| `profiles_select_own` | SELECT | Voir son propre profil |
| `profiles_update_own` | UPDATE | Modifier son propre profil |
| `profiles_select_admin` | SELECT | Admins voient tous les profils |
| `profiles_all_super_admin` | ALL | Super admins ont tous les droits |
| `profiles_insert_service` | INSERT | Insertion de son propre profil |

### Fonctions helpers RLS

| Fonction | Description |
|----------|-------------|
| `public.get_current_profile_role()` | Détermine le rôle (`user`, `admin`, `super_admin`) du profil associé à `auth.uid()` sans déclencher les policies (SECURITY DEFINER). |
| `public.is_admin_user()` | Retourne `true` si le rôle actuel est `admin` ou `super_admin`; utilisé dans les policies d'audit. |
| `public.is_super_admin()` | Retourne `true` uniquement pour le super admin; permet d'accorder tous les droits sans récursivité. |

### User Consents

| Policy | Action | Description |
|--------|--------|-------------|
| `consents_select_own` | SELECT | Voir ses propres consentements |
| `consents_insert_own` | INSERT | Créer ses consentements |
| `consents_update_own` | UPDATE | Modifier ses consentements |
| `consents_select_admin` | SELECT | Admins peuvent auditer |

### Categories

| Policy | Action | Description |
|--------|--------|-------------|
| `categories_select_all` | SELECT | Tout le monde peut lire |
| `categories_admin_all` | ALL | Admins peuvent tout modifier |

### User Category Preferences

| Policy | Action | Description |
|--------|--------|-------------|
| `prefs_select_own` | SELECT | Voir ses propres préférences |
| `prefs_insert_own` | INSERT | Créer ses préférences |
| `prefs_update_own` | UPDATE | Modifier ses préférences |
| `prefs_delete_own` | DELETE | Supprimer ses préférences |

---

## Triggers Automatiques

### `on_auth_user_created`

**Déclencheur** : Après insertion dans `auth.users`

**Action** : Crée automatiquement un enregistrement dans `public.profiles` avec :
- `id` : ID de l'utilisateur
- `email` : Email de l'utilisateur
- `display_name` : Extrait des metadata ou partie avant @ de l'email
- `email_verified` : Basé sur `email_confirmed_at`

### `profiles_updated_at` / `consents_updated_at`

**Déclencheur** : Avant UPDATE

**Action** : Met à jour automatiquement le champ `updated_at`

---

## Installation

### Via Supabase Dashboard

1. Aller dans **SQL Editor**
2. Créer une nouvelle requête
3. Copier le contenu de `Miyukini Framework - Migration Initiale.sql`
4. Exécuter

### Via Supabase CLI

```bash
supabase db push
```

### Via MCP (Cursor)

Utiliser l'outil `apply_migration` du serveur MCP Supabase.

---

## Vérification

Après l'installation, vérifier :

```sql
-- Tables créées
SELECT table_name FROM information_schema.tables 
WHERE table_schema = 'public';

-- Types créés
SELECT typname FROM pg_type 
WHERE typnamespace = 'public'::regnamespace;

-- RLS activé
SELECT tablename, rowsecurity FROM pg_tables 
WHERE schemaname = 'public';
```

---

## Rollback

Pour supprimer toutes les tables et types (⚠️ DANGER) :

```sql
-- Supprimer les triggers d'abord
DROP TRIGGER IF EXISTS on_auth_user_created ON auth.users;
DROP TRIGGER IF EXISTS profiles_updated_at ON public.profiles;
DROP TRIGGER IF EXISTS consents_updated_at ON public.user_consents;

-- Supprimer les fonctions
DROP FUNCTION IF EXISTS public.handle_new_user();
DROP FUNCTION IF EXISTS public.handle_updated_at();

-- Supprimer les tables
DROP TABLE IF EXISTS public.user_consents;
DROP TABLE IF EXISTS public.profiles;

-- Supprimer les types
DROP TYPE IF EXISTS public.consent_type;
DROP TYPE IF EXISTS public.user_tier;
DROP TYPE IF EXISTS public.user_role;
```

---

## Compte Super Admin par Défaut

La migration crée automatiquement un compte super administrateur pour l'accès initial au back-office.

### Identifiants

| Champ | Valeur |
|-------|--------|
| **Email** | `miyukini@gmail.com` |
| **Mot de passe** | `070287` |
| **Rôle** | `super_admin` |
| **Tier** | `free` |

### ⚠️ Avertissement de Sécurité

> **IMPORTANT** : Ces identifiants sont publics dans la documentation.
> 
> **En production** :
> 1. Connectez-vous avec ces identifiants
> 2. Changez immédiatement le mot de passe
> 3. Ou supprimez ce compte et créez-en un nouveau

### Caractéristiques

- Email vérifié automatiquement
- Onboarding marqué comme complété
- Accès complet au back-office (`/admin`)
- Droits de gestion sur tous les utilisateurs

---

## Versions

| Version | Date | Description |
|---------|------|-------------|
| 1.0.0 | 2026-01-10 | Migration initiale - Module Compte Utilisateur + Super Admin |
| 1.1.0 | 2026-01-10 | Migration catégories - Module Navigation + Préférences |
