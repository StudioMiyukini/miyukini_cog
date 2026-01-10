Miyukini Framework - Back Office et Super Admin

## Contexte

Le back office et le super admin partagent l’infrastructure layout/theme mais diffèrent par les niveaux de visibilité, les actions critiques et les accès systèmes. Ce document formalise les composantes spécifiques (menus, dashboards, actions) et définit les invariants pour assurer une expérience cohérente tout en gardant la sécurité (contextes, permissions, contrôles).

## Portée / Scope

- Décrire les composants UI spécifiques (menus, dashboards, actions critiques) pour le back office et le super admin.
- Détail des niveaux de visibilité & permissions (navigation, actions, audit).
- Lier les contexts (role, edition, layout) et les hooks aux actions disponibles.
- Fournir des guidelines de composition UI (layouts, sections) pour ces roles.


## 1. Architectures des layouts

- **Back office** : repose sur `GestionLayout`, colonnes responsive (mobile empilé, desktop 3 colonnes), `GestionBottomNav`, contextes `useLayoutMode`, `useActiveTheme`, `RoleSimulation`.
- **Super admin** : utilise `Layout` + `BottomNav`, sections dashboards (metrics, audit logs) + `RoleSimulationButton`.
- Header/body/bottom consistent : `header` avec simulation de rôle/badges, `body` pour sections modulaires, `bottom` pour nav contextualisée.


## 2. Niveaux de visibilité

- **Back office (admin/gestion)** :
  - Navigation latérale/BottomNav montre catégories (`GestionBottomNav`).
  - Sections `AdminCategoryXMenu` : accessibles selon `activeCategory`.
  - Boutons critiques (ex : `créer une catégorie`, `archiver`) ne sont visibles que si `user_type` >= `gestion`.
  - Menus flottants exclusifs (ouvrir/fermer) via `showCategoryXMenu`.
- **Super admin** :
  - Menu complet dans `BottomNav` (audit, users, metrics).
  - Actions supplémentaires : `impersonate user`, `forcer reload`, `export logs`.
  - Dashboard `superadmin/metrics` avec cartes `InfoCard`, `charts`, etc.


## 3. Actions critiques & permissions

- **Back office** :
  - `POST /api/admin/categories/:id/actions` (state change) vérifie `user.user_type >= admin`.
  - `GestionBottomNav` change `activeCategory`, `showMenu`. Les `motion.button` suivent les tokens `buttonStyle`.
  - Actions `create`/`edit`/`delete` encapsulées dans molécules `FormField`, `Button`.
- **Super admin** :
  - Actions `impersonate`, `deploy`, `audit` dans `superadmin` nav.
  - `RoleSimulationButton` avec `role switch` (affiche roles disponibles).
  - `BottomNav` (desktop) / `GestionBottomNav` (mobile) s’adaptent aux `allowedBottomNavItems`.


## 4. Contextes & hooks

- `RoleSimulationContext` : switch role via backend, impacte nav et composants.
- `EditionContext` : mode edition (super admin) active certains boutons (ex : mode “Editer layout”).
- `useActiveTheme` + `colorToRgba` appliqués partout (back office, super admin dashboards).
- `useLayoutMode` : définit `mode: gestion | superadmin | visitor` et active le layout mobile/desktop.


## 5. Composants recommandés

- `organisms/layouts/GestionLayout` pour back office.
- `organisms/navigation/GestionBottomNav` (mobile) + `BottomNav` (desktop) pour navigation.
- `organisms/modals/BaseModal` pour confirmations critiques.
- `molecules/cards/InfoCard` et `organisms/sections/EventCollection` pour dashboards.
- `atoms/ui/Button`, `molecules/forms/FormField` pour actions/modals.


## 6. Tests & QA

- Vérifier les menus selon rôle (mock `getAccessibleGestionCategories`).
- Tester `RoleSimulationButton` + `bottom nav` sur mobile/desktop.
- Tester actions critiques (impersonate, logs export) dans super admin.


## 7. Observabilité

- Logs backend pour actions super admin.
- Audit pour les changements (super admin + back office) via `GET /api/admin/audit`.

> Ce guide assure que les back offices et super admin partagent les mêmes bases (layout/responsivité/thème) tout en respectant les différences de visibilité/actions.
