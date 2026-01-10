# spec_auth_account_module__cursor

## Contexte
- Ce module publie un socle utilisateur réutilisable par tout SaaS construit sur Supabase : authentification, profil, sessions, rôles, consentements.
- Il complète le Framework Modulaires en fournissant un contrat clair entre frontend (Atomic UI) et backend (Supabase, Edge Functions, Webhooks).
- Le document sert de référence pour les équipes produit/tech qui doivent déployer ce module dans de nouveaux projets SaaS sans réinventer la roue.

## Objectifs & Scope
- **Vision** : proposer un compte utilisateur sécurisé et évolutif, prêt à être branché sur n’importe quel frontend React + backend Supabase, avec état utilisateur, session, consentements et rôles.
- **Objectifs**
  1. Normaliser la structure Supabase (tables, RLS, fonctions) pour que chaque nouveau SaaS récupère un module plug-and-play.
  2. Documenter les endpoints, les événements et les scripts utiles (Edge Functions, triggers, migrations).
  3. Prévoir les surfaces communes à toute app : onboarding, gestion de profil, MFA, consentement RGPD, autorisations.
- **Hors scope** : gestion métier spécifique (payments, booking, inventory) reste dans les modules métiers propres à chaque SaaS.

## Architecture et composants
- **Backend Supabase**
  - Tables principales : `users`, `profiles`, `auth_sessions`, `consents`.
  - RLS déclenchées via `auth.uid()` pour limiter les lectures/écritures.
  - Edge Functions : `send-welcome-email`, `reissue-session`, `rgpd-delete-user`.
- **Frontend**
  - Atomes : `Input`, `Button`, `ToastStack`, `ModalDialog`.
  - Organismes : `AccountShell` (header/nav), `ProfileEditor`, `ConfirmDialog`.
  - Hooks : `useActiveTheme`, `useCurrentUser`, `useAuthRedirect`.
- **Services**
  - `AuthService` (TanStack Query + Supabase client) : login, signup, refresh, invite.
  - `AccountService` : récupère profil, update metadata, liste sessions.
  - `ConsentService` : enregistre consentements, expose historique.

## Modèles & données
- Tables Supabase :
  1. `users` (réplique de `auth.users`) : `id UUID PRIMARY KEY`, `email`, `phone`, `role`, `tier`, `created_at`, `deleted_at`.
  2. `profiles` : `user_id`, `first_name`, `last_name`, `display_name`, `avatar_url`, `metadata JSONB`, `updated_at`.
  3. `auth_sessions` : `id`, `user_id`, `refresh_token`, `expires_at`, `ip_address`, `device_info`.
  4. `consents` : `id`, `user_id`, `type ENUM('marketing','service','analytics')`, `granted BOOLEAN`, `granted_at`.
- Zod schemas (front/backend) : `profileSchema`, `sessionSchema`, `consentSchema`.
- Enum TypeScript : `AccountRole = 'user' | 'admin' | 'super_admin'`.

## Logique métier
- **Onboarding** : l’utilisateur s’inscrit (Supabase Auth), on crée automatiquement son profil (trigger `auth.users INSERT`), on envoie un email via Edge Function.
- **Session** : `AuthService` gère login, refresh, logout. Chaque session créee écrit dans `auth_sessions`, les tokens refreshs sont stockés/rotated automatiquement.
- **Consentements** : lors de l’inscription ou de l’accès aux préférences, le frontend enregistre `consents` ; les logs sont consultables par l’admin via UI.
- **Rôles** : RLS assure que seuls `super_admin` peuvent manipuler profils d’autres rôles, `admin` gère les invitations, `user` n’accède qu’à son propre profil.
- **Passwordless + MFA** : option emails magic link, validation SMS. Supabase OTP + `auth.factor` (si activé).

## Endpoints & scripts (Supabase)
- GraphQL / REST (via Edge Functions) :
  - `POST /auth/login` (email+password) → `supabase.auth.signInWithPassword`.
  - `POST /auth/magic-link` → `supabase.auth.signInWithOtp`.
  - `GET /account/profile` (JWT) → retourne `profiles`.
  - `POST /account/profile` → mise à jour `profiles`, `metadata`.
  - `POST /account/consents` → upsert dans `consents`.
  - `POST /admin/sessions/revoke` → supprime `auth_sessions`.
  - `POST /rgpd/delete-user` (Edge Function) → purge tables.
- SQL / migrations :
  - Création `consents` table + trigger `log_consent_changes`.
  - RLS policies `profiles_select_own`, `profiles_update_own`, `consents_manage`.
  - Functions `send_welcome_email(user_id UUID)`.

## UX & Accessibilité
- Composer des écrans mobile-first (landing, login, dashboard) avec `ContentStack` + `ActionTray`.
- Écrans à thèmes : `AccountShell` (header sticky) + `BottomNavigation` adaptatif.
- Feedbacks : `ToastStack` pour erreurs API, `ModalDialog` pour confirmer suppression de compte ou réinitialisation.
- Indicateurs de statut (badge `StatusBadge`) : email verified, tier, contexte RGPD.

## Sécurité & RGPD
- Rôles Supabase + RLS :
  - `profiles` : `SELECT`/`UPDATE` limité à `auth.uid()` + `admin`.
  - `consents` : `INSERT`/`UPDATE` utilisateur uniquement, `admin` peut lire.
  - `auth_sessions` : `user` peut lister ses sessions, `admin` peut révoquer.
- Logging RGPD : chaque consentement enregistré avec `granted_at`; anonymisation via `rgpd-delete-user`.
- Secrets rotation : tokens Supabase séparés par environnement, `service_role` utilisé uniquement dans Edge Functions.

## Observabilité & tests
- Logs centralisés via `admin_logs` pour chaque action critique (login, consentement, session revoke).
- Tests unitaires : `profileSchema`, `AuthService`, Edge Functions (mock Supabase).
- Tests d’intégration : parcours login → profil → consentement → suppression (via Supabase localstack ou test env).

## Recommandations de réutilisation
- Ce module peut s’intégrer à tout SaaS en clonant les migrations SQL + Edge Functions + services TS.
- Externaliser `AuthService` dans `core/services` pour partager avec Booking/Professional modules.
- Documenter chaque paramètre (capabilities, scopes) via `doc/framework/spec_auth_account_module__cursor.md`.

