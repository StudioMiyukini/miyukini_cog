Miyukini Framework - UI Kit Catalogue

## Contexte

Le UI Kit Miyukini repose sur l’Atomic Design et le thème dynamique compilés dans `Miyukini Framework - Atomic Design & Theme Dynamique.md`. Pour permettre à chaque module (front, back office, super admin, page d’accueil) de réutiliser les composants existants sans recréer de primitives, il faut une carte explicite des composants disponibles par couche, leurs tokens, leurs dépendances, et leurs démonstrations (stories / exemples).

Ce catalogue sert de référence pour choisir un composant, comprendre ses tokens, trouver sa story/démo et savoir dans quelle couche il vit (`atoms`, `molecules`, `organisms`, `templates`). Il mentionne aussi les variations spécifiques (forms, modals, navigation) et leurs règles de thèmes/JS.

## Portée / Scope

- Indexer les composants par couche atomique (`atoms/ui`, `atoms/icons`, `molecules/forms`, `organisms/modals`, `templates`, etc.).
- Documenter les tokens utilisés (couleurs, spacing, typographie) pour chaque composant.
- Pointer vers les stories/démos ou exemples (Storybook, MDX, docs internes).
- Donner des recommandations de réutilisation (ex : back office utilise `GestionCard`, front `HeroSection`).


## 1. Atoms

- **`atoms/ui/Button`** : `primary`, `outline`, `ghost`. Tokens : `theme.colors.section.card.background`, `theme.colors.text.primary`, `theme.spacing.medium`. Stories : `stories/ui/button.stories.tsx`.
- **`atoms/ui/Input` / `Textarea` / `Select`** : utilisent `colorToRgba` et `theme.colors.section.border` pour fond et bordure, animations `onFocus` avec `theme.colors.accent`. Stories : `stories/ui/forms.stories.tsx`.
- **`atoms/ui/IconButton`** : `Icon` + label, `touch-target` 44px. Tokens : `theme.spacing.small`, `theme.colors.section.card.foreground`. Story : `stories/ui/icon-button.stories.tsx`.
- **`atoms/ui/Badge`, `Tag`, `Label`** : typographie `theme.fonts.body`, `theme.colors.text.secondary`, `colorToRgba` for background. Demo : `docs/ui/badges.md`.
- **`atoms/icons/*`** : collection d’icônes (Heroicons + custom). Chaque icône documentée en `stories/icons.stories.tsx`.


## 2. Molecules

- **`molecules/accordion/Accordion`** : Accordéon FlyonUI avec icônes +/-, animations, support multi-items. Props: `items`, `allowMultiple`. Dépendances : FlyonUI JS, `@iconify-json/tabler`. Demo : `stories/accordion/accordion.stories.tsx`.
- **`molecules/forms/FormField`** : combine label, helper text, input, erreurs; utilise `textPrimaryStyle`, `textSecondaryStyle`. Stories : `stories/forms/form-field.stories.tsx`.
- **`molecules/cards/InfoCard`** : `title`, `description`, `actions`. Tokens : `cardStyle`, `theme.spacing.large`. Demo : `stories/cards/info-card.stories.tsx`.
- **`molecules/forms/SearchBar`** : Input + icon + placeholder, utilise `colorToRgba` pour l’ombre. Demo `stories/forms/search-bar.stories.tsx`.
- **`molecules/lists/UserRow`** : alignement, avatar, badges. Story : `stories/lists/user-row.stories.tsx`.


## 3. Organisms

- **`organisms/modals/BaseModal`** : overlay `getModalOverlayStyle(theme, 0.5)`, `Card` container. Handles `header/body/bottom`. Demo: `stories/modals/modal.stories.tsx`.
- **`organisms/sections/EventCollection`** : assemble titres, badges, listes. Tokens : `spacing`, `textPrimaryStyle`. Story: `stories/sections/event-collection.stories.tsx`.
- **`organisms/sections/HeroModule`** : section hero avec titre, sous-titre, CTA. Tokens : `theme.colors.text.primary`, `theme.spacing.large`. Demo: `stories/sections/hero-module.stories.tsx`.
- **`organisms/sections/StatsGrid`** : grille de statistiques responsive. Tokens : `theme.colors.section.card`, `theme.spacing.medium`. Demo: `stories/sections/stats-grid.stories.tsx`.
- **`organisms/navigation/Header`** : navigation supérieure fixe, glassmorphism. Dépendances : `useActiveTheme`, `useIsMobile`, `useAuth`. Voir `Miyukini Framework - UI Header et BottomNav.md`. Demo: `stories/navigation/header.stories.tsx`.
- **`organisms/navigation/BottomNav`** : navigation inférieure fixe, 8 catégories par défaut, indicateur actif animé. Dépendances : `useActiveTheme`, `useIsMobile`, `useAuth`. Touch-target 48px (WCAG). Voir `Miyukini Framework - UI Header et BottomNav.md`. Demo: `stories/navigation/bottom-nav.stories.tsx`.
- **`organisms/layouts/GestionLayout`** : utilise `atoms`/`molecules`, gère colonnes, sidebars. Variation mobile vs desktop. Story: `stories/layouts/gestion-layout.stories.tsx`.


## 4. Templates

- **`templates/AppShellScreen`** : wrapper triptyque (header/body/bottom), gère les paddings de compensation (pt-64px, pb-80px). Voir `Miyukini Framework - UI Header et BottomNav.md`. Story: `stories/layouts/app-shell.stories.tsx`.
- **`templates/ContentStack`** : empileur de contenu vertical, spacing automatique. Tokens : `theme.spacing.large`. Story: `stories/layouts/content-stack.stories.tsx`.
- **`templates/Layout`** : wrapper auth, `Toaster`, `header/body/bottom`. Tokens : `theme.spacing.page`, `theme.colors.section.background`.
- **`templates/HomepageModule`** : hero + CTA, depends on `UI Kit modules`.
- **`templates/BackOfficeGrid`** : colonnes 15/70/15, `ResizablePanelGroup`. Stories: `stories/templates/backoffice-grid.stories.tsx`.


## 5. Stories & démos

- Préférer Storybook/MDX centralisé (`stories/**/*.stories.tsx`) pour tester visuellement chaque composant.
- Documenter les composants les plus utilisés dans `docs/ui/` (ex : `docs/ui/cards.md`, `docs/ui/forms.md`).
- Chaque entrée de catalogue liste :
  - Couche & chemin (`src/components/atoms/ui/Button.tsx`).
  - Tokens / styles (`theme.colors`, `colorToRgba`, spacing, typography).
  - Hooks requis (`useActiveTheme`, `useIsMobile`, `useAuth`).
  - Story / demo (fichier ou sandbox).

### Documents de référence détaillés

| Document | Composants couverts |
|----------|---------------------|
| `Miyukini Framework - UI Header et BottomNav.md` | Header, BottomNav, AppShellScreen |
| `Miyukini Framework - Atomic Design & Theme Dynamique.md` | Tokens, thèmes, règles de styling |
| `Miyukini Framework - Layouts Responsives.md` | Layouts, breakpoints, triptyque header/body/bottom |
| `Miyukini Framework - Catégories et Thèmes.md` | 8 catégories, thèmes graphiques (Standard, Dark, Oasis) |


## 6. Réutilisation par module

- **Front (Homepage)** : privilégier `templates/HomepageModule`, `molecules/cards/InfoCard`, `atoms/ui/Button`.
- **Back office** : `organisms/layouts/GestionLayout`, `organisms/navigation/GestionBottomNav`, `molecules/forms/SearchBar`.
- **Super admin** : `templates/Layout`, `organisms/modals/BaseModal`, `atoms/ui/Input`, `molecules/forms/FormField`.
- **UI Kit** : tout module doit référencer cette doc pour éviter la duplication (pas de couleurs en dur, tokens centralisés).


## 7. Tokens & theming

- `theme.colors.section.*`, `theme.colors.text.*`, `theme.spacing.*`, `theme.borders.*`.
- Helpers : `colorToRgba`, `getModalOverlayStyle`, `textPrimaryStyle`, `textSecondaryStyle`.
- Les stories doivent utiliser ces tokens pour illustrer les versions mobile/desktop, hover, disabled.


## 8. Gouvernance & contribution

- Mettre à jour ce catalogue lorsque de nouveaux composants sont ajoutés (`atoms` → `pages`).
- Ajouter une story ou exemple minimal lors de la création d’un komponant.
- Valider que chaque composant respecte les règles d’interdiction (pas de couleurs hardcodées, hooks de thème) et la hiérarchie atomique.

> Ce catalogue sert de pont entre les développeurs front/back, le framework (UI Kit) et les cron hooks ; il garantit réutilisabilité, cohérence de thème et documentation visuelle (stories/démos).
