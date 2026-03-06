# P0 Temps 2 - Ideation

## Statut

- Etat : TERMINE
- Phase : P0 Temps 2
- Responsable principal : Maria/Lise

## TL;DR

MIPOWER V1 = app desktop Tauri (Rust + Svelte/Vanilla TS) locale, unifiee : Dashboard sequences live + Lecteur rapports Markdown riche + Suivi temps reel (file watcher) + Prompt builder. Lit les artefacts MIP existants. SQLite pour index interne. Must work #1 : lecteur rapport avec tableaux et diagrammes. Complexite C5.

## Perimetre

### IN (inclus dans la sequence — V1)

- Application Tauri : backend Rust + frontend Svelte ou Vanilla TS + TailwindCSS
- Dashboard : liste sequences depuis sequences/index.json, tri/filtre/recherche, badges statut
- Suivi temps reel : file watcher Rust (crate notify) -> events Tauri -> UI live
- Lecteur rapport : renderer Markdown riche (tableaux, Mermaid diagrams), navigation artefacts
- Prompt builder : formulaire structure -> premier prompt MIP + init arborescence sequence
- Index interne : SQLite (rusqlite) pour aggregation des metriques et meta-data
- Retrocompatibilite : lecture sequences/index.json + .md + metrics/*.json existants

### OUT (exclus de la sequence — V2+)

- Versioning git integre dans MIPOWER (V2)
- Knowledge Base interactive depuis .mip/memory/ (V2)
- Edition en ligne des artefacts .md dans MIPOWER (V2)
- Collaboration multi-utilisateurs (hors scope)
- Cloud sync (hors scope — local seulement)
- Integration directe LLM dans MIPOWER (hors scope V1)

## Decoupe fonctionnelle

| Bloc | Description | Priorite | Agents pressentis |
|------|-------------|----------|------------------|
| Core Tauri shell | Bootstrap app, systeme de routing, menu, dark mode | V1 | Francois (back) + Lise (front) |
| Dashboard sequences | Liste, tri, filtre, badges T/C/statut depuis index.json | V1 | Lise (front) + Francois (index) |
| File watcher | notify crate Rust -> canal Tauri events -> refresh UI | V1 | Francois |
| Lecteur rapport | Renderer MD riche, Mermaid, navigation sections | V1 — Must work #1 | Lise |
| Prompt builder | Formulaire guide -> generation prompt + init dossier | V1 | Lise (form) + Francois (init) |
| SQLite index | rusqlite, projection sequences/metrics, recherche full-text | V1 | Francois |
| UI Design System | Sidebar, panels, composants, dark mode, couleurs Miyukini | V1 | Lise |

## MVP — Definition minimale viable

App Tauri qui s'ouvre, affiche le dashboard des sequences (lecture sequences/index.json), permet d'ouvrir un rapport .md rendu avec tableaux et diagrammes. File watcher basique. Sans prompt builder ni SQLite. Valeur immediate : remplace le portail HTML statique avec une UX moderne.

## Dependances identifiees

| Dependance | Type | Statut |
|-----------|------|--------|
| Tauri v2 | externe | a integrer |
| Rust (cargo) | interne (workspace Miyukini-COG) | disponible |
| notify (file watcher) | externe crate Rust | a ajouter |
| rusqlite | externe crate Rust | a ajouter |
| Svelte ou Vanilla TS | externe frontend | a definir (T4/T6) |
| TailwindCSS | externe frontend | a integrer |
| Mermaid.js | externe frontend | a integrer |
| marked.js ou pulldown-cmark | externe (MD renderer) | a definir (T6) |
| sequences/index.json | interne MIP | disponible |
| scripts init-sequence*.ps1 | interne MIP | disponibles (a appeler depuis Tauri) |

## Complexite estimee

- Complexite sequence : **C5 — Strategique**
- Justification : Nouvelle app autonome, triple stack (Rust+Tauri+frontend), architecture from scratch, UI complete (4 blocs majeurs), file watcher concurrent, SQLite, retrocompatibilite MIP
- Confirme par utilisateur en T1. Sera valide par Denis en T8.

## Risques principaux

| Risque | Impact | Mitigation envisagee |
|--------|--------|---------------------|
| Complexite Tauri (setup, IPC, permissions) | Haut | T4 Denis evalue Tauri v2 ; T6 Francois spec les IPC precis ; plan de fallback web local |
| File watcher concurrent (debounce, lock FS) | Moyen | crate notify debounce integre ; tests unitaires watcher isoles |
| Scope creep (features infinies) | Haut | Perimetre V1 borde strict (voir OUT) ; Denis garde le plan |
| Maintenance triple stack | Moyen | Lise responsable frontend uniquement ; Francois Rust ; separation propre |

## Solutions envisagees

| Solution | Avantages | Inconvenients | Score |
|----------|-----------|--------------|-------|
| Tauri v2 + Svelte + Tailwind | Leger, natif, coherent Rust, DX excellent | Setup initial, IPC a apprendre | 9/10 |
| Tauri v2 + Vanilla TS + Tailwind | Encore plus leger, zero framework | Moins de composants prebuilt | 8/10 |
| Electron + React | Ecosysteme riche, familier | 200MB+, node_modules, non coherent Rust | 4/10 |
| Serveur HTTP Rust + navigateur | Zero install, simple | Experience desktop moins fluide, pas natif | 6/10 |

## Solution retenue

**Tauri v2 + Svelte + TailwindCSS** — a confirmer par Denis (T4) et Francois (T6). Fallback : Vanilla TS si Svelte ajoute trop de complexite.

## Statut final T2

- Etat : TERMINE
- Horodatage : 2026-03-06
- Agents : Maria + Lise (parallele)

