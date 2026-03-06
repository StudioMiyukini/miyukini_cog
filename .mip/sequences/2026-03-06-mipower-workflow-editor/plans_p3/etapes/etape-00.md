# E00 -- Bootstrap workspace Tauri

## Statut : A faire
## Depend de : --
## Agents : Francois
## Taches : 4
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E00-01 | SETUP | Creer workspace Rust `mipower/` + Cargo.toml workspace | Francois | mipower/Cargo.toml | pending | -- | -- |
| E00-02 | SETUP | Scaffold Tauri v2 avec `cargo tauri init` + config tauri.conf.json | Francois | src-tauri/tauri.conf.json | pending | -- | -- |
| E00-03 | SETUP | Configurer Vite + Svelte 5 + TailwindCSS + package.json | Francois | package.json, vite.config.ts, tailwind.config.ts | pending | -- | -- |
| E00-04 | TEST | Test fumee : `cargo tauri dev` ouvre une fenetre blanche sans erreur | Francois | -- | pending | -- | -- |

## Commit message template
`feat(mipower): E00 -- bootstrap Tauri v2 + Svelte 5 + Tailwind`

## Criteres de completion
- `cargo tauri dev` lance sans erreur
- Fenetre WebView2 visible
- Svelte rendu (Hello World)
- TailwindCSS applique (classe test visible)
