Miyukini Framework - Setup & Build

## Contexte

Ce guide reprend l’ensemble de la documentation existante (UI Kit, layouts responsives, screens portables, auth, API, thèmes, monitoring) et décrit **pas à pas** la manière de mettre en place le projet, d’installer ses dépendances, de générer les premiers composants et de lancer la web app/devenir utilisable. Il garantit que chaque développeur démarre sur une base saine, prête à être portée vers du natif.

## Portée / Scope

- Préparer l’environnement (Node, dependances, outils IA).
- Installer les packages nécessaires (frontend, outils UI Kit, lint, tests).
- Générer les premiers composants + screens respectant les règles d’atomic design/écrans.
- Lancer la stack (`npm run dev`, tests, surveillance).
- Référencer les docs pertinentes (thèmes, auth, API, prompts) pour chaque étape.


## 1. Pré-requis

1. Installer Node 20+ (LTS). Vérifier `node -v` et `npm -v`.
2. Cloner le repo `MiyukiniFramework`.
3. Copier `.env.example` → `.env.local` (mode dev) et renseigner les variables liées à Supabase/API (voir `docs/framework/Miyukini Framework - Architecture Globale et Dépendances.md`).
4. Installer Git, Chrome (pour DevTools) et un terminal (PowerShell est déjà valide).
5. Optionnel : installer `nvm` pour gérer la version Node.


## 2. Installation des dépendances

1. `npm install` (ou `pnpm install` si configuré) à la racine.
2. Vérifier les dépendances critiques :
   - UI kit : `@/components/ui`, `shadcn/ui` (voir `docs/framework/Miyukini Framework - UI Kit Catalogue.md`).
   - Hooks : `useActiveTheme`, `useAdminConfig`, `useIsMobile`.
   - Utils : `themeUtils`, `apiClient`, `loggingClient`.
3. Installer les outils de qualité :
   - Lint : `eslint`, `prettier` (config info dans root).
   - Tests : `jest`, `react-testing-library`, `cypress` / `playwright`.
   - Docs : `storybook` (s'il existe).
4. Mettre à jour les scripts `package.json` si besoin (ex : `npm run storybook`, `npm run lint`, `npm run test`).


## 3. Générer les composants & screens

1. Créer un screen : `src/features/<module>/ui/screens/<Name>Screen.tsx`.
   - Ajouter le `ScreenContract` (screenName, module, intent, layout) en commentaire.
   - Utiliser uniquement les composants UI Kit (`atoms`, `molecules`, `organisms`).
   - Ajouter TODO markers : `// TODO data binding`, `// TODO usecase injection`, `// TODO navigation handling`.
   - Ne pas importer `useNavigate`, `useParams`, `fetch`, `supabase`.
2. Placer la page correspondante dans `src/app/<route>/page.tsx` (Next) ou `src/routes` (Vite) :
   - Exemple : `export default function HomePage() { return <HomeScreen /> }`.
   - La page devient un wrapper jetable.
3. Respecter le triptyque `AppShellScreen / ContentStack / FAB` pour layout.
4. Créer les composants observables (hero, cards, bottom nav) décrits dans `docs/framework/Miyukini Framework - HomePage et Onboarding.md` et `docs/framework/Miyukini Framework - Catégories et Thèmes.md`.


## 4. Configuration thèmes & auth

1. Copier les thèmes Standard/Dark/Oasis depuis `docs/framework/Miyukini Framework - Catégories et Thèmes.md` dans `src/components/themes/`.
   - Chaque thème exporte `theme.colors`, `section`, `text`, `accent`.
   - `useActiveTheme` switcher (env var ou admin panel) pour sélectionner le thème.
2. Ajouter le compte Super Admin de test (`miyukini@gmail.com`, `070287`) dans les fixtures/dev data.
3. Assurer que les hooks (`useAuth`, `useRoleSimulation`) consomment les endpoints listés dans `docs/framework/Miyukini Framework - API et Contrats Données.md`.
4. Vérifier que chaque élément UI (buttons, cards, modals, bottom nav) utilise `colorToRgba`, `textPrimaryStyle`, `getModalOverlayStyle`.


## 5. Développement & run

1. `npm run dev` (ou `npm run dev -- --hostname 0.0.0.0` pour accès externe).
2. Ouvrir Chrome à `http://localhost:3000` (ou autre port) pour voir la home.
3. Pour la page d’accueil, vérifier :
   - Hero + CTA (use `HomeScreen`).
   - Bottom nav affichant catégories 1-8 (depend de `Catégories et Thèmes`).
   - Thèmes via `useActiveTheme`.
4. Pour back office / super admin : `GET /api/admin/categories`, `GET /api/superadmin/metrics`.


## 6. Tests & QA

1. Lancer `npm run lint` + `npm run test`.
2. Storybook / Chromatic (si configuré) : vérifier `atoms`, `molecules`, `organisms`.
3. Cypress/playwright : flows auth, bottom nav, role switch.
4. Monitoring : `loggingClient`, `Auth logs`, `bottom nav sync` (voir `docs/framework/Miyukini Framework - Tests & Monitoring.md`).


## 7. Déploiement & maintenance

1. Ajouter hooks `pre-commit` ou `commitlint` (voir docs root).
2. Documenter chaque nouveau screen dans `docs/framework/` (ScreenContract + TODO markers).
3. Garder la doc `docs/framework/guardrails/ia_anti_mobile_debt.md` à jour.
4. Mettre à jour `docs/framework/Miyukini Framework - Architecture Globale et Dépendances.md` et `Tests & Monitoring` si la stack évolue.


## 8. Checklist finale

- [ ] Node 20+, `npm install`.
- [ ] Screens sans router/hooks, avec ScreenContract.
- [ ] Themes / UI Kit appliqués (colorToRgba, getModalOverlayStyle).
- [ ] Auth + Super Admin fixture ready.
- [ ] API contracts respectés (auth, accounts, layout, admin).
- [ ] Tests/storybook/monitoring exécutés.
- [ ] Documentation dossiers mise à jour (`docs/framework/...`).

> En suivant ce guide tu peux passer d’un repo vide à un framework fonctionnel prêt à servir une web app et à être porté vers Android natif avec le minimum de refactor. Quellen prochaines étapes (ex: template screen, prompt IA) disponibles sur demande.
