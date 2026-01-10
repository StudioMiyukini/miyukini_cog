# Miyukini Framework - Overview

## Contexte général
- Le Miyukini Framework est une architecture modulaire front + back pensée pour des SaaS ultra-personnalisés (booking, professional tools, accounts, SuperAdmin). Il s’appuie sur une UX atomic design, un backend Supabase (PostgreSQL + Edge Functions) et un noyau partagé (`core/`) qui orchestre modules, événements, services et UI adaptative.
- Toutes les autres documentations sont des couches spécialisées du Framework ; ce document les rassemble et les contextualise autour des dépendances, de l’infrastructure et des pratiques à suivre.

## 1. Architecture & modules

### 1.1 Pattern modulaire
- Modules (`features/<module>`) respectent `ModuleContract` exposé depuis `core/modules` : routes, permissions, hooks `initialize`/`cleanup`. Le `ModuleRegistry` centralise les modules (`booking`, `professional`, `account`, `superadmin`, etc.).
- Tous les modules s’appuient sur `core/events` (EventBus + EventTypes) pour communiquer sans violer l’isolation.

### 1.2 Modules documentés
- **Account** : table `profiles`, `consents`, `auth_sessions`, endpoints `/auth/login`, `/account/profile`, triggers RGPD. Utilise le UI Kit (atoms, ActionTray, AppShellScreen) pour header/body/bottom.
- **SuperAdmin** : cockpit WordPress-style, configure clés/Intégrations/layout, gère metrics via `/superadmin/metrics`. Log et sécurité décrits dans `spec_superadmin_panel__cursor`.
- **UI Layout** : pattern triptyque (header/body/bottom) avec bottom à 5 zones/sous-menus dynamiques (SuperAdmin configure les boutons via API `/ui/bottom-submenu`).
- **Supabase infra** : tables et RLS ciblées assurant l’ossature account, triggers logs, scripts d’installation pour agents IA minimalistes.

## 2. UI Kit atomic & thème

- Tous les composants visuels consomment `useActiveTheme`, `colorToRgba`, `getModalOverlayStyle`. Interdiction totale de couleurs/inline hardcodées. Les atoms `Button`, `Card`, `Input`, `StatusBadge` s’assemblent en `ActionTray`, `BottomNavigation`, `ModuleToolbar`.
- Layout `AppShellScreen` + `ContentStack` assurent header/body/bottom. Les transitions et hover sont gérées via helpers (`ActionTray` interactions) et `colorToRgba` pour gérer opacités (body 40 %, cards 5-10 %).
- Le bottom menu mobile se transforme sur desktop en AdaptivePanel, et chaque bouton ouvre un sous-menu glissant (jusqu’à 6 actions) alimenté dynamiquement par SuperAdmin.

## 3. Backend Supabase & services

- Le backend Supabase repose sur :
  1. Tables : `auth.users`, `profiles`, `consents`, `auth_sessions`, `admin_logs`, `superadmin_config`.
  2. Policies : `profiles_select_self`, `consents_manage`, `superadmin_config_super_admin`, `metrics_read`.
  3. Edge Functions : `send-welcome-email`, `auth-refresh`, `rgpd-delete-user`.
- Services partagés (`core/services` dans repo) orchestrent DatabaseService (offline), NotificationService, GamificationService, BookingService et peuvent être prolongés par le panel SuperAdmin (`MaintenanceRunner`, `LayoutConfigurator`, `MetricsDashboard`).

## 4. Sécurité / RGPD

- Rôles : `user`, `professional`, `admin`, `super_admin`. RGPD via table `consents` + function `rgpd-delete-user`. Logs centralisés dans `admin_logs`. Rotation de clés surveillée par `KeysManager`.
- Pas de "backdoor". Le panel SuperAdmin s’authentifie via Supabase (login + MFA), lit `/superadmin/metrics` et ne repose que sur des comptes valides.

## 5. Observabilité & métriques

- `MetricsDashboard` expose latence, taux d’erreur, uptime ; les métriques sont envoyées via `admin_logs` et `superadmin` endpoints. Les logs sont reliés aux actions (consentements, rotations) et accessibles via `/superadmin/logs`.

## 6. Installation & dépendances

- **Pré-requis** :
  - Node.js 18+, npm/yarn.
  - Supabase CLI (suivant doc `spec_supabase_infrastructure__cursor`).
  - Git, pnpm optionnel.
- **Dépendances front** :
  - React 18, TypeScript strict.
  - TanStack Query + Zustand.
  - Dexie (IndexedDB) pour offline.
- **Dépendances backend** :
  - Supabase (Postgres + Edge Functions).
  - SendGrid ou tout service SMTP (disponible via Secrets).
  - Supabase Storage buckets (`avatars`, `documents`, `exports`).

### 6.1 Script d’installation de base (account only)

Reprendre le script `spec_supabase_infrastructure__cursor` (Supabase CLI, migrations `101_account`, Edge functions `send-welcome-email`, `auth-refresh`, `rgpd-delete-user`, policies, seeds, tests). Les variables `SUPABASE_URL`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `SENDGRID_API_KEY` doivent être injectées avant l’exécution.

## 7. Extensions & maintenance

- **SuperAdmin Layout** configure bottom menu, sous-menus, panels via `LayoutConfigurator` et broadcast WebSocket.
- **Migrations** : `001_create_profiles.sql`, `002_create_account_related_sequences.sql`, `003_create_consents.sql`. Ajouter d’autres migrations par module métier.
- **Testing** : unitaires (Zod schemas, services), intégration (Supabase auth + refresh + consents), e2e (Playwright parcours header/body/bottom).

## 8. Tri des docs

- L’ossature décrite ici s'appuie sur :
  - `spec_supabase_infrastructure__cursor` (tables, RLS, scripts).
  - `spec_superadmin_panel__cursor` (panel WordPress-style, sécurité, métriques).
  - `spec_ui_triptych_layout__cursor` (layout header/body/bottom, menu 5 zones).
  - `Compte Utilisateur` (auth, profils, RGPD, Edge functions).
  - `Framework Modulaire — Implémentation TS` (modules, event bus, structure, UI Kit).

## 9. FAQ & prochaines étapes

- Q : Où documenter un nouveau module métier ?  
  R : Ajouter `features/<module>/api`, `domain`, `data`, `ui`. Lister ses contrats dans `ModuleRegistry`.
- Q : Comment intégrer SuperAdmin ?  
  R : Créer `features/superadmin` avec services de configuration, hooks (`useSuperAdminConfig`), API, UI (AppShellScreen triptyque) et conférences vers `superadmin_config`.
- Q : Quels tests run ?  
  R : `npm run lint`, `npm run test`, tests Supabase (script `test:account-integration`), Playwright sur layout en header-body-bottom.

> Ce document synthétique peut servir de README principal pour le Miyukini Framework ; chaque section pointe vers des docs spécialisées pour aller plus loin.

