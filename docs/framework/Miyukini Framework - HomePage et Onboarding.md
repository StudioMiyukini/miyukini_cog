Miyukini Framework - HomePage et Onboarding

## Contexte

La page d’accueil et les flows d’onboarding sont les premiers points de contact avec Miyukini. Ils doivent refléter le système de layouts (header/body/bottom), respecter le UI Kit atomic (thème dynamique, tokens) et guider l’utilisateur connecté ou visiteur vers les modules pertinents (auth, back office, super admin). Ce cahier des charges décrit la structure, les composants clés et les règles d’accès qui rendent la HomePage réutilisable et cohérente.

## Portée / Scope

- Définir la structure page : header (actions globales), body (hero, modules, CTA), bottom (nav / actions critiques).
- Lister les composants UI Kit mobilisés (atoms/molecules/organisms) et leurs tokens.
- Documenter les flows d’onboarding (état connecté vs visiteur, onboarding progressif).
- Mentionner les dépendances layout/thème et les règles d’accès (auth, rôles).


## 1. Structure de la HomePage

- **Header** : utilise `Layout` avec `RoleSimulationButton`, `Toaster`, `CTA` (ou login pour visiteurs). Doit respecter les tokens `buttonStyle` pour les actions.
- **Hero section** : `organisms/sections/HeroModule` (titre, sous-titre, CTA principal, illustration). Utilise `atoms/ui/Button`, `atoms/icons`, `textPrimaryStyle`.
- **Modules prioritaires** : `organisms/sections/StatsGrid`, `molecules/cards/InfoCard`, `organisms/sections/EventCollection`. Chaque module s’imbrique dans `molecules/lists` et consomme `colorToRgba`.
- **CTA secondaires** : `atoms/ui/Button` (ghost/outline) ou `LinkButton` (custom) pour onboarding.
- **Body** : layout responsive (`body` `flex-col` mobile, `grid` desktop). Utiliser `templates/BackOfficeGrid` si le contenu nécessite colonnes.
- **Bottom** : `BottomNav` (auth roles) ou `GestionBottomNav` pour accès rapides.


## 2. Composants & UI Kit

- **Atoms** : `Button`, `Badge`, `Input`, `IconButton` (CTA, recherche). Couleurs : `theme.colors.section.card.background`, `theme.colors.text.primary`.
- **Molecules** : `FormField` (newsletter, onboarding steps), `SearchBar` (recherche module), `InfoCard`.
- **Organisms** : `HeroModule`, `StatsGrid`, `EventCollection`, `GestionBottomNav` (sur mobile).
- **Templates** : `HomepageModule` (hero+modules) engage le layout `header/body/bottom`.
- **Stories** : `stories/pages/homepage.stories.tsx` pour preview.

Tous les composants respectent `useActiveTheme` + tokens, pas de couleurs hardcodées. Les CTA utilisent `buttonStyle`.


## 3. Flows d’onboarding

- **Visiteur** :
  - Header : bouton `Se connecter` (CTA `atoms/ui/Button` ghost).
  - Body : hero + modules “Pourquoi Miyukini”, “Modules phares”.
  - Footer : CTA “Découvrir la démo”.
  - Sur action “Demande démo”, ouvrir `organisms/modals/BaseModal` (overlay `getModalOverlayStyle`).
- **Utilisateur connecté** :
  - Header : `RoleSimulationButton`, `EditionContext`.
  - Body : onboarding progressif via modules (checklist, onboarding cards). Exemple : `organisms/sections/OnboardingChecklist`.
  - Modules se déploient selon `user_type` (visitor vs gestion).
  - Onboarding onboarding-layers ??? We'll mention statuses.

## 4. Règles d’accès & layout

- Page d’accueil accessible même sans authentification.
- `BottomNav` visible uniquement pour utilisateurs connectés (`user_type >= gestion`).
- Onboarding progressif conditionné par `EditionContext` & `RoleSimulationContext`.
- Layout : `header/body/bottom` standard (header fixed, `main flex-1`, bottom nav visible ou réduite).
- Background modules utilisent `colorToRgba` (opacity 0.1-0.2) ; text via `textPrimaryStyle`.


## 5. Dépendances & intégration API

- Consomme `GET /api/homepage` et `GET /api/sections` (UI Kit components binding). Modules (hero, CTA) sont alimentés par ces données.
- `Hero` CTA déclenche `POST /api/notifications` pour onboarding push.
- Onboarding cards lisent `GET /api/accounts/me` (progress status, user_type).


## 6. Tests & QA

- Tester hero/CTA en mode mobile/desktop (fake widths).
- Vérifier qu’un visiteur voit uniquement les CTA de login/demo.
- Tester onboarding progressif : `EditionContext` (steps) et `RoleSimulationContext`.
- QA : valider que tous les composants utilisent `colorToRgba` et tokens via storybook.


## 7. Livrables

- Composants à produire : `HeroModule`, `StatsGrid`, `OnboardingChecklist`.
- Stories : `stories/pages/homepage.stories.tsx`.
- Monitoring : `src/pages/HomePage.tsx`, `src/components/layouts/Layout.tsx`, `src/components/sections/HeroModule.tsx`.


## 8. Critères d’acceptation (DoD)

- [ ] Hero + modules respectent `useActiveTheme`, `textPrimaryStyle`, `colorToRgba`.
- [ ] Flows visiteurs vs connectés documentés et/ou testés (CTA/login/bottom nav).
- [ ] Onboarding progressif fonctionnel (checklist, API status) + `RoleSimulationButton`.
- [ ] Layout header/body/bottom maintenu ; `BottomNav` conditionné à l’authentification.

> Ce cahier des charges garantit une HomePage alignée sur les standards de layout, l’UI Kit atomique et les règles d’accès pour visiteurs/gestionnaires. Chaque module devra pointer vers cette doc avant d’ajouter un nouveau composant.
