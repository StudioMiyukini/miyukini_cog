Miyukini Framework - Layouts Responsives

## Contexte

Le système de layout Catakana sépare la structure de chaque écran en trois zones distinctes : le **header** (top de page), le **body** (zone principale) et le **bottom** (navigation fixe ou flottante). Cette partition garantit cohérence, accessibilité mobile/desktop et isolation des rôles (auth, navigation, contenu). Ce document reprend ces bons usages et les articule avec la nomenclature d’Atomic Design et du theming dynamique déjà imposés dans le workspace Miyukini.

## Portée / Scope

- Décrire les responsabilités de chaque zone (`header`, `body`, `bottom`) afin de pouvoir reproduire ces layouts dans d’autres projets.
- Formaliser les comportements responsives clés (breakpoint 800 px, colonnes, bottom nav).
- Lister les composants, hooks et contexts nécessaires (`Layout`, `GestionLayout`, `BottomNav`, `useIsMobile`, `useActiveTheme`, etc.).
- Fournir un guide de duplication (sécurité, accessibilité, thèmes) en évitant toute référence propre à Catakana (comme `FloorPlan`).


## Parcours utilisateur

- L’utilisateur arrive sur `/dashboard` (ou toute route protégée) ; le `header` gère l’auth, la simulation de rôle, les actions globales et les toasts (`Toaster`).
- Le `body` affiche les pages via un `Outlet` ou un composant spécifique (ex. gestion, rapports); il s’adapte aux breakpoints `<800 px` (mobile) et `>=800 px` (desktop).
- Le `bottom` (navigation fixe ou flottante) reste visible selon le rôle sélectionné et permet de switcher entre catégories sans perturber le `body`.


## Points d’accès (Frontend / Backend)

- **Frontend**
  - `src/components/Layout.tsx` : wrapper d’authentification, gestion du `RoleSimulationButton`, rendu du `header/body/bottom`, `Toaster` et `BottomNav`.
  - `src/components/layouts/GestionLayout.tsx` : body responsive, menus exclusifs, catégories, bottom nav flottant (`GestionBottomNav`).
  - `src/components/navigation/GestionBottomNav.tsx` : nav mobile flottante avec `framer-motion`, `motion.button`, grille 4x2 et `aria-label`.
  - `src/components/navigation/BottomNav.tsx` : nav standard par rôle.
  - `src/hooks/use-mobile.tsx` : breakpoint unique à 800 px.
  - `src/components/hooks/useAdminConfig.ts` + `useActiveTheme` : appliquent les tokens de thème.
- **Backend**
  - Layout calculé côté client ; les sections/catégories dérivent de `gestionCategoriesConfig.ts` ou `sections.ts` mais pas de routes serveur dédiées.


## Modèles & données

- `Layout` s’appuie sur `AuthContext`, `RoleSimulationContext` et `EditionContext` pour déterminer la visibilité du header, la navigation et les toasts.
- `GestionLayout` conserve `activeCategory`, `activeSection` et les flags `showCategoryXMenu` pour garantir l’exclusivité des menus et piloter l’affichage du body.
- `useIsMobile` encapsule `window.matchMedia('(max-width:799px)')` et fournit un booléen stable ; il peut être remplacé par un hook équivalent si le projet cible un breakpoint différent.


## 1. Structure header / body / bottom

- **Header** : éléments globaux non spécifiques (logo, titre, filtres principaux, simulation de rôle, notifications). Doit conserver un espacement fixe (`top`) pour éviter que le body le recouvre sur mobile. Peut tirer ses couleurs de `useActiveTheme` et `colorToRgba`.
- **Body** : `main` `flex-1` contenant `Outlet` ou page intégrée, avec `px`/`py` constants, `max-width` optionnel et `padding-top` pour prendre en compte un header fixe. Sur desktop, il devient `display: flex` (trois colonnes par exemple); sur mobile, il devient `flex-col` (sections empilées). Le body regroupe les composants métiers (cartes, listes) et synchronise `useActiveEdition`, `useLayoutMode`.
- **Bottom** : `BottomNav` ou `GestionBottomNav` avec `motion.nav`, `motion.button`, `aria-label`, `touch-target` (min `44px`). Position `left:0; right:0; bottom:0`, `z-50`, ou flottant pour la gestion. Les boutons respectent `colorToRgba` + tokens et appliquent `whileHover`/`onMouseEnter` / `onMouseLeave`.


## 2. Modes mobile / desktop

- Breakpoint unique : `<800 px` = mobile (pile vertical, bottom nav visible), `>=800 px` = desktop (colonnes, sidebars, bottom nav réduit).
- `useIsMobile` contrôle les `[@media(max-width:799px)]` et `[@media(min-width:800px)]` (classes et JS).
- Mobile : colonnes uniques, bottom nav pleine largeur, boutons plus grands, nav flottante de `GestionBottomNav`.
- Desktop : layout multi-colonnes (ex. `70%` centre, `15%` latérales), sidebars `AdminCategoryXMenu` visibles, bottom nav réduit ou caché dans un bouton contextuel.


## 3. Composants, hooks et patterns

- `Layout`, `GestionLayout`, `BottomNav`, `GestionBottomNav`, `useIsMobile`, `useLayoutMode`, `useActiveTheme`, `colorToRgba`, `getModalOverlayStyle`.
- Les composants thématiques doivent consommer uniquement les tokens (`theme.colors.section`, `theme.colors.text`) et exécuter `colorToRgba`/`getModalOverlayStyle` pour fonds et overlays.
- Les boutons de navigation appliquent `onMouseEnter`/`onMouseLeave` pour augmenter l’opacité et utilisent `colorToRgba(theme, 0.1 → 0.2)`.


## 4. Logique métier détaillée

- **Header** : gère la redirection `loading`/`session`, appelle `LayoutModeContext`, reste stylé par le thème, expose `role` et `edition` (actions globales).
- **Body** : distinct pour mobile/desktop. Mobile = colonne unique avec sections empilées, sidebars cachées. Desktop = `flex` 3 colonnes, sidebars persistantes, panels redimensionnables (`ResizablePanelGroup`), sections synchronisées avec `activeCategory`.
- **Bottom** : visible selon les rôles (`bottomNavRoles`), utilise `motion` pour les états hover/tap, `aria-label` et `touch-target`. Peut être remplacé par un composant équivalent dans un autre projet.


## 5. Besoins UI / UX

- Cohérence via les tokens (`theme.colors.section`, `theme.spacing`, `theme.borders`). Aucun code couleur (`#...`) ou style inline fixe.
- Breakpoint central `800 px` ; utiliser les classes `[@media(max-width:799px)]` / `[@media(min-width:800px)]`.
- Bottom nav : boutons avec `aria-label`, `focus ring`, `min-h`/`min-w` 44px. Mobile = grille 4x2, desktop = nav réduite.
- Header et body doivent toujours offrir un titre visible et un `role` clair pour l’utilisateur (isolé par `RoleSimulationButton`).


## 6. Flux technique complet

- `App.tsx` enveloppe `Layout` dans `RequireAuth` et `LayoutModeProvider`, permettant de réutiliser la structure.
- Chaque page insérée dans le `body` via `Outlet` ; `bottom` et sections synchronisés par `useIsMobile`.
- Les hooks et contexts (role, édition, thème) pilotent la navigation, les menus et les toasts sans import de données serveur supplémentaires.


## 7. Gestion des erreurs

- `Layout` intercepte `loading`, `session`, `user`. Sans session, redirection (`/login`), sinon `Outlet`.
- Absence de rôle/permissions = navigation limitée (`getAccessibleGestionCategories`).


## 8. Sécurité

- `header` et `bottom` s’affichent uniquement si `Auth` et `RoleSimulation` fournissent les données exigées (évite l’accès non autorisé).
- `GestionLayout` filtre les catégories via `getAccessibleGestionCategories(user?.user_type)` et ne propose que les actions sécurisées.


## 9. Performances & optimisation

- Le layout reste léger : il ne rend que des wrappers avec classes utilitaires. `React.lazy` + `Suspense` dans `App.tsx` limite le bundle initial des pages.
- `framer-motion` reste simple (`initial`, `animate`), ce qui conserve la fluidité.


## 10. Dépendances & réutilisabilité

- Prévoir un hook `useIsMobile` ou équivalent, un provider de thème, des routes protégées (`RequireAuth`), un bottom nav aware des rôles.
- Utiliser `colorToRgba`, `getModalOverlayStyle`, `theme.colors` garantit la conformité quand on duplique.


## 11. Tests

- Unitaires : vérifier l’affichage de chaque zone (`header`, `body`, `bottom`) sur mobile et desktop (mock `window.innerWidth`), et que `BottomNav` dépend des rôles.
- QA : changer la largeur locale, observer les classes `[@media(max-width:799px)]`, tester les interactions du bottom nav (hover, motion).


## 12. Livrables & monitoring

- Document : `docs/framework/Miyukini Framework - Layouts Responsives.md`.
- Focaliser la surveillance sur `src/components/Layout.tsx`, `src/components/layouts/GestionLayout.tsx`, `src/components/navigation/GestionBottomNav.tsx`, `src/components/navigation/BottomNav.tsx`, `src/hooks/use-mobile.tsx`.
- Garder le focus sur header/body/bottom responsive ; exclure `FloorPlan` et autres spécificités Catakana.


## 13. Critères d’acceptation (DoD)

- [ ] Chaque zone (`header`, `body`, `bottom`) a une fonction claire, ses dépendances et son comportement responsive.
- [ ] Les breakpoints mobile/desktop (`<800px` / `>=800px`) et les hooks requis (`useIsMobile`, `useActiveTheme`) sont documentés.
