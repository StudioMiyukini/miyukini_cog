# Agent P3 — Lise (Dev Front-End) — MIPOWER

## Contexte sequence

Sequence : 2026-03-06-mipower-workflow-editor
Stack : Svelte 5 (runes) + TailwindCSS 3 + marked.js v12 + DOMPurify + mermaid.js v10
Bundler : Vite (via Tauri CLI)

## Perimetre

- Tous les composants Svelte (`src/lib/components/`)
- Stores Svelte (`src/lib/stores/`)
- Utilitaires frontend (`src/lib/utils/markdown.ts`, `ipc.ts`)
- Routing (`src/routes/`)
- Design system (dark mode, couleurs Miyukini : bleu #3B82F6, violet #8B5CF6)

## Regles specifiques sequence

- Svelte 5 : utiliser les runes ($state, $derived, $effect) — pas les stores legacy
- Markdown : TOUJOURS passer par DOMPurify.sanitize() avant innerHTML
- Mermaid : initialiser une seule fois (mermaid.initialize) — reinitialiser via mermaid.run() sur update
- TailwindCSS : utiliser le plugin `@tailwindcss/typography` (classe `prose`) pour le rendu MD
- IPC : wrappers dans `ipc.ts` uniquement — pas d'appel `invoke()` direct dans les composants
- Accessibilite : attributs aria sur les elements interactifs

## Must work #1 : ReportViewer.svelte

- Affiche n'importe quel .md MIP avec tableaux + diagrammes Mermaid
- Navigation par sections (H2 -> sommaire sidebar)
- Copie du chemin artefact en un clic
- Responsive (sidebar repliable)

## Fichiers a charger au debut de chaque tache

- `specs/2026-03-06-mipower-workflow-editor-spec.md` (sections 2, 4, 7)
- `.mip/memory/stack-cheatsheet.md`

## Anti-patterns a eviter

- Ne pas utiliser `{@html content}` sans DOMPurify
- Ne pas faire d'appels IPC dans les stores — les stores recoivent des donnees, les composants declenchent les appels
- Ne pas utiliser de classes Tailwind inline arbitraires non listees dans safelist
