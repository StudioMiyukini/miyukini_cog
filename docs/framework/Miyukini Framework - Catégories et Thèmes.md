Miyukini Framework - Catégories et Thèmes

## Contexte

Le back office doit piloter les catégories affichées dans le `GestionBottomNav` et les thèmes graphiques disponibles pour l’UI. Ce document formalise la structure des catégories (8 par défaut), leur lien avec le bottom nav administrable, et la palette de thèmes prêts à l’emploi. Il complète les règles de layout, d’Atomic Design et de thème dynamique déjà documentées.

## Portée / Scope

- Décrire les catégories par défaut gérées depuis le back office et leur affichage dans le `BottomNav`.
- Définir les thèmes graphiques disponibles (Standard, Dark, Oasis) et leurs tokens (couleurs, opacités).
- Indiquer comment changer le thème via `useActiveTheme` et `themeConfig`.


## 1. Catégories & BottomNav

- **8 catégories par défaut** : elles sont nommées `Catégorie 1`…`Catégorie 8` dans l’interface de référence.
- Chaque catégorie possède :
  - un identifiant (ex : `category_1`),
  - un label (ex : `Catégorie 1`),
  - un rôle requis (`user_type >= gestion`).
- Le `Back office` expose une grille pour renommer/activer/désactiver chaque catégorie ; les modifications se propagent à `GestionBottomNav`.
- `GestionBottomNav` affiche les catégories actives en `motion.button`, applique le thème actif (`buttonStyle`) et respecte l’ordre défini par l’admin.
- `BottomNav` sur desktop reprend les mêmes catégories mais peut afficher une version condensée.

Flow de mise à jour :
1. Super admin / admin modifie les catégories dans le panel du back office (`POST /api/admin/categories/:id/actions`).
2. Backend met à jour `gestionCategoriesConfig` (order, label, status).
3. Front refresh via `GET /api/admin/categories` ou websocket, `BottomNav` re-render.


## 2. Thèmes graphiques disponibles

### 2.1 Standard

- Fond principal : `theme.colors.section.background = #0b1120`, opacité `0.4`.
- Cards : `colorToRgba(theme.colors.section.card.background, 0.1)`.
- Textes : `theme.colors.text.primary = #f8fafc`, `theme.colors.text.secondary = #cbd5f5`.
- Survol : `colorToRgba(theme.colors.section.card.background, 0.2)`.
- Utilisation : layout par défaut, back office, homepage.

### 2.2 Dark

- Fond : `#05070d` (méthode `colorToRgba` pour cards 0.15).
- Accent : `theme.colors.accent = #14b8b2`.
- Textes : plus saturés pour contraste (`#f0f9ff`, `#94a3b8`).
- Idéal pour super admin / monitoring.

### 2.3 Oasis

- Fond : `colorToRgba('#032c2a', 0.8)` avec `section.card.background` 0.2.
- Accent : `theme.colors.accent = #34d399`.
- Textes : `#e0f2fe`, `#bae6fd`.
- Utiliser pour onboarding incroyable ou dashboards climat.


## 3. Mise en œuvre & tokens

- Le thème actif est géré via `themeConfig` (exporté par `useActiveTheme`).
- Règles :
  - Ne jamais hardcoder les couleurs : toujours passer par `theme.colors.*` ou `colorToRgba`.
  - `buttonStyle`, `cardStyle`, `textPrimaryStyle` basés sur les tokens ci-dessus.
  - `getModalOverlayStyle(theme, 0.5)` aligne l’overlay avec le thème.
- Lors de l’ajout d’un thème, créer un fichier `src/components/themes/<theme>.ts` exposant les variables et documenter dans ce fichier.


## 4. Tests & validation

- Vérifier que chaque thème (Standard, Dark, Oasis) est disponible depuis `useActiveTheme`.
- Tester le `BottomNav` après modification de catégories (ordre, activation) via mocks.
- Valider que les surfaces `card`, `button`, `overlay` changent correctement de couleur grâce aux tokens `colorToRgba`.

> Ce document garantit que les catégories et thèmes restent synchronisés entre le back office, le bottom nav et l’UI Kit, pour un rendu cohérent et contrôlé par rôle.
