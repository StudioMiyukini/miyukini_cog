Miyukini Framework - Architecture Globale et Dépendances

## Contexte

La robustesse d’un écosystème Miyukini repose sur la cohérence entre ses couches : framework, frontend, backend et infrastructure. Sans document central qui décrit “qui appelle quoi / qui dépend de quoi”, on risque des doublons (double authentification, services redondants) ou des points de rupture lors de l’intégration de nouveaux modules (UI Kit, super admin, back office, page d’accueil).

Ce document propose une cartographie des couches, des services critiques et des dépendances transverse afin de faciliter les décisions d’architecture, d’aligner les équipes frontend/backend/infra, et de sécuriser la gouvernance des modules.

## Portée / Scope

- Décrire le schéma en couches (Infra > Backend > Framework > Frontend) et les responsabilités de chacune.
- Lister les services API, tâches (jobs, cron), webhooks et dépendances (auth, stockage, notifications) par couche.
- Faire apparaître les flux de dépendances principaux pour éviter les redondances (ex. deux systèmes d’auth).
- Fournir une vue sur les contrats critiques (auth, comptes, super admin, back office, UI Kit).
- Mettre en évidence les points d’entrée pour les modules (landing page, super admin, back office) et les dépendances transverses (thème, hooks, contextes).


## 1. Schéma en couches

```
Infra / Plateforme (K8s, Supabase, Storage, Webhooks)
   ↓
Backend (API REST, tâches, auth, services métier)
   ↓
Framework (communs, UI Kit, contextes, thèmes)
   ↓
Frontend (apps web / pages : home, back office, super admin)
```

- **Infra :** héberge les bases (Supabase), les buckets, les tâches planifiées (cron, workers) et les webhooks entrants/sortants.
- **Backend :** expose les APIs protégées, gère l’auth (tokens JWT, session Supabase), orchestre les tâches (ex : import/export, notifications), et publie des événements (webhooks).
- **Framework :** regroupe les primitives partagées (UI Kit atomique, hooks `useActiveTheme`, `useAuth`, contextes `RoleSimulation`, layout responsives) utilisables par n’importe quel frontend.
- **Frontend :** assemble les applications (page d’accueil, back office, super admin) en combinant layout, UI Kit, données via API et thèmes dynamiques.


## 2. Services et dépendances par couche

### 2.1 Infra & plateforme

- **Supabase** : base utilisateurs/roles, stockage de fichiers, realtime (pubs/sub). Expose les tables `users`, `accounts`, `sessions`.
- **Buckets (Storage)** : centralisé pour les assets (avatars, exports). Utilisation via SDK Supabase + helpers `getStorageClient`.
- **Webhooks** : endpoints dédiés (ex : `/webhook/notification`, `/webhook/order`) lors d’événements backend.
- **Jobs / cron** : export CSV, purge sessions, notifications planifiées ; orchestrés via tâches backend (ex : `src/services/tasks/`), déclenchées depuis infra (scheduler Firebase/Cloudflare Workers).

### 2.2 Backend

- **Auth service** : centralise les mécanismes `login`, `logout`, `refresh token`, `mfa`, `role switch`. Utilise Supabase Auth et expose des endpoints (`/api/auth/*`).
- **Compte utilisateur** : contrôle les rôles (super admin, admin, gestion, visitor, etc.), les profils, la MFA, les paramètres (with `supabase`).
- **Back office / Super admin services** : endpoints `admin/*`, `superadmin/*` offrant gestion des catégories, audit logs, métriques. Influencent la visibilité des menus dans `BottomNav`.
- **UI Kit & Framework Utility service** : fournit les tokens (via API ou config partagée) pour les thèmes dynamiques, mais se contente généralement de fichiers `.ts`.
- **API métier** : `sections`, `gestion`, `balance`, `notifications`. Chaque module consomme des routes documentées (API contract).
- **Webhooks** : tasks backend reçoivent/émettent des webhooks (ex : notifications push). Ils dépendent du module `services/webhooks`.

### 2.3 Framework

- **UI Kit atomic** : `atoms/`, `molecules/`, `organisms/`, `templates/`, `pages/`. Utilise `useActiveTheme`, `colorToRgba`, `getModalOverlayStyle`.
- **Contextes** : `AuthContext`, `RoleSimulationContext`, `LayoutModeContext`, `EditionContext`, `ThemeContext`.
- **Hooks** : `useActiveTheme`, `useAuth`, `useRoleSimulation`, `useIsMobile`, `useAdminConfig`.
- **Layouts** : `Layout`, `GestionLayout`, `Layouts responsives`.
- **Helper services** : `themeUtils`, `apiClient`, `supabaseClient`, `storageClient`, `notificationClient`.

### 2.4 Frontend

- **Page d’accueil** (`HomePage`) : combine hero, modules CTA, retours des services `homepageMetrics`.
- **Back office** : repose sur `GestionLayout`, `GestionBottomNav`, appels backend admin, UI Kit atomique.
- **Super admin** : utilise `Layout`, `BottomNav`, `RoleSimulation`, dashboards spéciaux, audit logs (API `superadmin/audit`).
- **Auth & comptes** : pages `Login`, `Reset`, `Account`, connectées via API auth.
- **UI Kit modules** : `components/ui/` (shadcn + custom) et `icons`.
- **Feature flags / contexts** : `LayoutMode`, `Edition`, `Theme`.


## 3. Flux critiques & dépendances

- **Auth unique** : seul le service backend gère les tokens. Tous les front doivent appeler `/api/auth` ou Supabase Auth client ; au frontend, `useAuth` s’appuie sur ce service.
- **Comptes / rôles** : `superadmin` et `admin` partagent le même modèle (`user_type`). Backend expose `getAccessibleGestionCategories` et `getAllowedBottomNavItems`.
- **UI Kit consumé par tous** : aucun frontend ne doit recréer ses propres primitives (la duplication du thème est interdite). Les components `atoms`, `molecules`, etc. doivent évoluer dans `src/components`.
- **Tâches** : jobs backend déclenchent des notifications, qui doivent appeler l’API `notification` backend ; frontend utilise `notificationClient` qui repose sur ces endpoints.
- **Webhooks** : backend réceptionne webhooks (ex. `payment`, `supabase` events) et notifie front via `realtime`/pubsub ou en écrivant dans la base.


## 4. Contrats recommandés

- **API Auth (REST)** : `POST /api/auth/login`, `POST /api/auth/refresh`, `POST /api/auth/role-switch`, `POST /api/auth/logout`.
- **Comptes** : `GET /api/accounts/me`, `PATCH /api/accounts/me`, `GET /api/accounts/:id/roles`.
- **Back office** : `GET /api/admin/categories`, `POST /api/admin/categories/:id/actions`, `GET /api/admin/audit`.
- **Super admin** : `GET /api/superadmin/metrics`, `POST /api/superadmin/users/:id/impersonate`.
- **UI / contenu** : `GET /api/homepage`, `GET /api/sections`, `GET /api/notifications`.
- **Webhooks** : `POST /webhook/notification`, `POST /webhook/payment` (voir infra).

Chaque contrat doit être documenté avec les schémas de données (`user`, `category`, `layout`) dans `docs/specifications/`.


## 5. Observabilité / monitoring

- Les logs infra/back indiquent les appels API critiques.
- Les tâches backend reportent les statuts (ex : email envoyé, export terminé).
- Le frontend utilise `loggingClient` pour tracer les erreurs d’UI Kit (ex : thèmes manquants).


## 6. Extension future

- Ajouter un module “UI Kit catalogue” (stories ou storybook) pour vérifier les composants.
- Prévoir un “schema de dépendances” en diagramme (PlantUML / Mermaid) dans `docs/architecture/`.
- Centraliser les secrets (Supabase, webhooks) dans un vault (évoqué ici mais géré par infra).

> Une architecture cohérente évite la duplication, sécurise les accès et facilite la réutilisation des modules (front/back, UI Kit, auth, back office, super admin).
