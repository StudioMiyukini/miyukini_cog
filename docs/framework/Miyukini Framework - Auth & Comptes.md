Miyukini Framework - Auth & Comptes

## Contexte

L’authentification et la gestion des comptes sont le socle de tous les modules Miyukini (front, back office, super admin, page d’accueil). Il faut un guide unifié qui explicite les flows (login, rôle, MFA), les contexts partagés (`AuthContext`, `RoleSimulationContext`), les permissions critiques et les dépendances backend (Supabase, JWT, API auth). Ce document complète les règles UI en garantissant que toute couche respecte les mêmes accès et tokens.

## Portée / Scope

- Décrire tous les flows d’authentification/account (login, logout, session, MFA, role switch).
- Récapitulatif des contexts front (`AuthContext`, `RoleSimulationContext`, `EditionContext`) et de leurs responsabilités.
- Lister les permissions nécessaires pour chaque type d’utilisateur (visitor, gestion, admin, super admin).
- Connecter ces flows aux endpoints backend (`/api/auth`, `/api/accounts`).


## 1. Flows d’authentification

### 1.1 Login / Logout

- Front : `LoginPage` appelle `POST /api/auth/login` (email, password, optional MFA code). Le token (JWT) est stocké via `authClient` et partagé via `AuthContext`.
- Backend : endpoint valide les credentials via Supabase Auth, retourne le token + rôle (user_type).
- Logout : `POST /api/auth/logout` invalide la session côté backend et vide les contexts (token, role).

### 1.2 Session / Refresh

- `AuthContext` garde `isLoading`, `user`, `token`. Au montage, il appelle `GET /api/auth/session` ou `refresh token`.
- Si le token expire, `refresh` via `POST /api/auth/refresh`.
- Les erreurs redirigent vers `/login`.

### 1.3 MFA / Role switch

- MFA activée via `POST /api/auth/mfa/enroll`, `POST /api/auth/mfa/verify`.
- Role switch (`RoleSimulationContext`) demande `POST /api/auth/role-switch` (role cible). Le backend vérifie les permissions et renvoie un token scoped.
- UI (header) affiche `RoleSimulationButton` et restreint les actions aux rôles permis.


## 2. Contexts & hooks front

- **`AuthContext`** : expose `{ user, token, isLoading, login, logout, refresh }`.
- **`RoleSimulationContext`** : garde `{ currentRole, availableRoles, switchRole }`.
- **`EditionContext`** : gère les états d’édition (mode preview vs réalité).
- **`useAuth` hook** : wrapper pour consommer `AuthContext`.
- **Protected routes** utilisent `RequireAuth` (front) qui repose sur `AuthContext`.


## 3. Permissions et rôles

- **Visitor** : accès lecture-only aux écrans publics. Ne voit pas `BottomNav` admin.
- **Gestion** : accès aux layouts de gestion (`GestionLayout`), actions par catégorie via bottom nav.
- **Admin** : même que gestion + accès aux catégories critiques, monitoring de base.
- **Super Admin** : accès complet, logs, impersonation.
- Chaque endpoint (ex : `/api/admin/categories`) vérifie `user.user_type`.
- `getAccessibleGestionCategories(user?.user_type)` détermine les catégories visibles.

### Compte de test Super Admin

- login : `miyukini@gmail.com`
- mot de passe : `070287`
- Ce compte est préconfiguré en dur pour les tests sur l’ossature (dev/preview). Il est chargé de tous les rôles, logs et permissions.
 


## 4. Contrats backend relatifs à l’auth

- `POST /api/auth/login` → { token, user, roles }
- `POST /api/auth/logout`
- `POST /api/auth/refresh`
- `POST /api/auth/role-switch` → { token }
- `GET /api/auth/session`
- `GET /api/accounts/me`
- `PATCH /api/accounts/me`
- `GET /api/accounts/:id/roles`

Chaque appel utilise `Authorization: Bearer <token>`.


## 5. Sécurité UX

- Tokens stockés dans mémoire ou cookies httpOnly (éviter localStorage uniquement). `AuthContext` perd le token immédiatement après logout.
- `BottomNav`, `RoleSimulationButton`, `GestionBottomNav` consultent `RoleSimulationContext` pour n’afficher que les boutons autorisés.
- Sessions invalides redirigent vers `/login` sans animation.


## 6. Tests & QA

- Mock `useAuth` pour tester les pages selon rôle.
- Vérifier les redirections (session expirée → `/login`).
- Tester la visibilité du bottom nav et des onglets admin par rôle.


## 7. Observabilité

- Auth backend log les échecs login/MFA.
- Front envoie les erreurs auth dans `loggingClient`.

> Un guide partagé évite deux systèmes d’auth et s’assure que tous les rôles consomment les mêmes contexts.
