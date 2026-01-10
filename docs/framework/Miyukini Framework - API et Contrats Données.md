Miyukini Framework - API et Contrats Données

## Contexte

Pour aligner le frontend et le backend (layout, auth, comptes, back office, analytics), il faut un catalogue partagé des endpoints incontournables et des contrats de données (`user`, `role`, `layout`). Ce guide documente les API essentielles, leurs schémas, les règles de permissions et la synchronisation avec le layout (header/body/bottom) côté client.

## Portée / Scope

- Lister les endpoints critiques (auth, comptes, back office, analytics) avec leurs verbes & payloads.
- Définir les schémas de données associés (user, layout, category, role).
- Spécifier les droits requis (visitor, gestion, admin, super admin).
- Indiquer les connexions au layout (header/body/bottom) et aux contexts front.


## 1. API Auth & comptes

- `POST /api/auth/login` → `{ email, password, mfaCode? }` → `{ token, user, roles }`.
- `POST /api/auth/logout` → `{}`.
- `POST /api/auth/refresh` → `{ refreshToken }` → `{ token }`.
- `POST /api/auth/role-switch` → `{ role }` → `{ token, currentRole }`.
- `GET /api/auth/session` → `{ token, user }`.
- `GET /api/accounts/me` → `{ user }`.
- `PATCH /api/accounts/me` → `{ profile updates }`.
- `GET /api/accounts/:id/roles` → `{ roles }`.

**Données** : `{ user: { id, email, user_type, role, theme, settings } }`.

**Permissions** : login autorisé pour tous ; role-switch limité à `user_type >= admin`.


## 2. API Layout & navigation

- `GET /api/layout/header` → `{ title, actions, showRoleSimulation }`.
- `GET /api/layout/bottom` → `{ items: [{ label, icon, route, rolesAllowed }] }`.
- `GET /api/layout/body-sections` → `{ sections: [{ id, type, layout, dataSources }] }`.

Front synchronise :
- `header` (RoleSimulation également) via `LayoutModeContext`.
- `bottom` via `BottomNav` / `GestionBottomNav` (filtré avec `rolesAllowed`).
- `body` sections via `section.id` et contexts `Edition`, `ActiveCategory`.


## 3. API Back office & super admin

- `GET /api/admin/categories` → liste avec `{ id, label, roleRequired }`.
- `POST /api/admin/categories/:id/actions` → `{ action, payload }`.
- `GET /api/admin/audit` → logs (critique pour super admin).
- `GET /api/superadmin/metrics` → `{ visitors, conversions, errors }`.
- `POST /api/superadmin/users/:id/impersonate` → immatricule session.

**Permissions** : `roleRequired` (admin/super admin). Requiert token dans header ; backend valide via `user_type`.


## 4. API Analytics & notifications

- `GET /api/homepage` → `Hero`, `modules`, `featuredSections`.
- `GET /api/sections` → données (cards, lists, stats).
- `GET /api/notifications` → liste filtrée par user/role.
- `POST /api/notifications/mark-read`.

- `POST /webhook/notification` → reçoit push, déclenche `notifications`.
- `POST /webhook/payment` → updates `balance`, signale `alert`.

La data contract pour `Notification` : `{ id, type, message, read, created_at }`.


## 5. Schémas de données clés

- **User** : `{ id, email, user_type, role, status, theme, settings, created_at }`.
- **Category** : `{ id, label, roleRequired, active, sections: [] }`.
- **Layout section** : `{ id, type ('card', 'table', 'form'), dataSource, rolesAllowed }`.
- **Role** : `{ id, name, permissions: [], defaultLayout }`.


## 6. Synchronisation front/back

- Tous les layouts invoices `AuthContext` + `LayoutModeContext`. Backend expose les structs (header/bottom/body).
- `BottomNav` filtre `items` selon `rolesAllowed`; backend pre-calc `allowedBottomNavItems`.
- `useActiveTheme` consomme `user.theme`.


## 7. Observabilité & tests API

- Documenter chaque endpoint dans `docs/specifications/api/`.
- Tests backend : contrats d’API (Swagger/openapi).
- Tests front : mocks (MSW) pour vérifier `BottomNav/header/body`.


> Un guide API + contrats garantit que le backend ne diverge pas des attentes front (layouts, roles) et évite de dupliquer les logiques auth/navigation. Documenter chaque endpoint avec schémas dans `docs/specifications/` est recommandé.
