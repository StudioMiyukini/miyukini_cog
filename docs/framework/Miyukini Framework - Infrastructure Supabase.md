# Miyukini Framework - Infrastructure Supabase

## Contexte

- Supabase est la couche backend privilégiée du Miyukini Framework : authentification, stockage, fonctions Edge, RLS, storage, notifications. Ce document détaille la structure des tables, les policies, les Migrations, les fonctions et les connexions nécessaires pour déployer tous les modules (booking, professional, account, superadmin, metrics).
- Il sert de guide pour les développeurs / agents IA qui doivent installer, migrer ou vérifier une instance Supabase pour n’importe quel SaaS dérivé du framework.

## Architecture données (tables & relations)

- **auth.users** : Supabase Auth, contient `id`, `email`, `role`, `created_at`, `app_metadata`.
- **profiles** (Account module) :
  - `user_id UUID REFERENCES auth.users`, `first_name`, `last_name`, `display_name`, `avatar_url`, `metadata JSONB`.
  - `status ENUM('active','onboarding','suspended')`, `tier_id`.
  - RLS `profiles_select_self`, `profiles_update_self`.
- **consents** :
  - `id`, `user_id`, `type ENUM('marketing','service','analytics')`, `granted BOOLEAN`, `granted_at`, `source TEXT`.
  - Trigger `log_consent_changes` écrit dans `admin_logs`.
- **auth_sessions** :
  - `id`, `user_id`, `refresh_token`, `expires_at`, `ip_address`, `device_info`.
  - Utilisée pour monitorer les refresh tokens et détecter les revocations.
- **admin_logs** :
  - `actor_id`, `action`, `details JSONB`, `created_at`, lié aux activités SuperAdmin.
  - Reçoit les insertions depuis les triggers (clé, consentement, rotation, RGPD).
- **superadmin_config** (optionnelle) :
  - `scope`, `payload JSONB`, `role_access TEXT[]`, `updated_by`, `updated_at`.
  - Utile uniquement pour piloter la configuration du panel (bottom menus, permissions). Peut rester vide si inutilisée.

## RLS & policies critiques

- `profiles_select_self`: `auth.uid() = user_id`.
- `profiles_update_self`: `auth.uid() = user_id`.
- `consents_manage`: `auth.uid() = user_id`.
- `superadmin_config_super_admin`: `auth.role() = 'super_admin'` (si le panel est déployé).
- Toutes les tables sensibles logguent dans `admin_logs` via triggers pour audit.

## Edge Functions & API

- `functions/send-welcome-email`: payload `user_id`, utilise SendGrid (texte + branding).
- `functions/auth-refresh`: rafraîchit le JWT et logge l’événement dans `auth_sessions`.
- `functions/rgpd-delete-user`: purge `profiles`, `consents`, `auth_sessions`, `admin_logs` pour `user_id`.
- Setup : seules les fonctions nécessaires à la gestion du compte (auth refresh, onboarding, RGPD) sont déployées ici.

## Migrations clés

- `001_create_profiles.sql`
- `002_create_account_related_sequences.sql`
- `003_create_consents.sql`
- `004_next_available_slot.sql` reste optionnelle (module booking) — peut être ignorée si on cible uniquement l’account module.
- `101_account_tables.sql` (agenda pro, vacations, invoices) est déjà présent sous `supabase/migrations` pour JéRDV pro.
- Appliquer les migrations depuis `supabase/migrations` (ex. `supabase db push` ou `supabase migration apply`).

## Services & connexions

- **TanStack Query** via `supabaseClient`: `SUPABASE_URL` + `SUPABASE_ANON_KEY`.
- **Supabase service_role key** : réservée aux appels backend (Edge functions, rotation de clés); stockée dans `KeysManager`.
- **Edge Functions** : utilisent `service_role` pour les actions RGPD/auth refresh.
- **SendGrid / SMTP** : stockées en secrets pour onboarding et notifications.
- **Storage** : buckets `avatars`, `documents` avec RLS `bucket_auth_user` (le module de compte gère uniquement `avatars`).

## Observabilité & logs

- `admin_logs` collecte toutes les actions sensibles (modif profil, rotation clé, RGPD).
- `pg_stat_activity` peut être monitoré à la demande, mais le focus reste la visibilité des sessions et logs utilisateur.
- Les métriques (latence, erreurs) sont exposées via `/superadmin/metrics` si le panel est déployé; sinon, les logs suffisent.

## RGPD & sécurité

- `consents`: `granted_at`, `source`, trigger `log_consent_changes`.
- Suppression via `rgpd-delete-user`, les actions loguées.
- Rotation des clés (service_role, SendGrid) via `KeysManager`; jamais de clé hardcodée.
- Secrets stockés dans Supabase (Vault) avec RLS `protected_templates`.

## Scripts d’installation (destinés aux agents IA)

```bash
# 1. Installer Supabase CLI
npm install -g supabase@latest

# 2. Initialiser projet
cd /workspace/Miyukini
supabase init

# 3. Copier uniquement les migrations account
cp -r supabase/migrations/101_account supabase/migrations/

# 4. Appliquer migrations
supabase db reset --apply-migrations

# 5. Déployer Edge Functions minimales
supabase functions deploy send-welcome-email
supabase functions deploy auth-refresh
supabase functions deploy rgpd-delete-user

# 6. Créer policies critiques
supabase db query "CREATE POLICY IF NOT EXISTS profiles_select_self ON profiles FOR SELECT USING (auth.uid() = user_id);"
supabase db query "CREATE POLICY IF NOT EXISTS consents_manage ON consents FOR INSERT USING (auth.uid() = user_id);"

# 7. Charger données initiales (roles account)
psql $SUPABASE_DB_URL -f supabase/data/seed_account_roles.sql

# 8. Tester le login + refresh
npm run test:account-integration
```

> Ce script minimal instaure uniquement l’ossature Supabase Account. Il présuppose l’existence des variables `SUPABASE_URL`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `SENDGRID_API_KEY` injectées dans l’environnement sécurisé.
