# Site officiel Miyukini COG

Site de documentation et de présentation de l’écosystème **Miyukini COG**, hébergeable sur Vercel. Contenu : onboarding, documentation (pyramide, Cores, glossaire), pages Services et **mock COG** basé sur les fonctionnalités réelles du Hub.

## Stack

- **Vite** + **React** + **TypeScript**
- **React Router** pour la navigation
- CSS modules + variables (thème sombre, accent teal)

## Commandes

```bash
npm install
npm run dev      # Développement (http://localhost:5173)
npm run build    # Build production
npm run preview  # Prévisualisation du build
```

## Déploiement Vercel

1. Projet lié au repo (ou sous-dossier).
2. **Root Directory** : `site_officiel_miyukini_cog` (si mono-repo).
3. **Build Command** : `npm run build`
4. **Output Directory** : `dist`

Le fichier `vercel.json` est déjà configuré (build, rewrites SPA).

## Structure

- `src/pages` — Accueil, Onboarding, Docs, Cores, Services, Mock COG
- `src/data` — Données doc (cores, pyramide, services, glossaire)
- `src/components` — Layout, Card

## Push uniquement de ce sous-projet

Pour pousser uniquement ce dossier vers un repo dédié (ex. déploiement séparé) :

- **Option A** : Depuis la racine du monorepo, configurer Vercel avec **Root Directory** = `site_officiel_miyukini_cog`.
- **Option B** : Créer un dépôt séparé et copier le contenu de `site_officiel_miyukini_cog` (ou utiliser `git subtree split` / sous-module).
