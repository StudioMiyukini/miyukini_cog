# Plan P3 2026-03-06-mipower-workflow-editor

## Statut

- Etat : A completer
- Phase : P3
- Complexite : C5 — Strategique
- Responsable principal : Denis

## TL;DR

Plan C5 en 8 etapes + BUF. Bootstrap Tauri -> Backend Rust (SQLite + IPC) -> File watcher -> Dashboard frontend -> Lecteur rapport (Must work #1) -> Suivi temps reel -> Prompt builder -> Polish UI. Chaque etape = livrable testable. Francois (back) + Lise (front) en parallele sur E03-E06.

---

## DAG des etapes

```
E00 (Bootstrap Tauri)
  |
  E01 (Backend core : SQLite + modeles + IPC base)
  |
  +-- E02 (File watcher + indexation) ----+
  |                                        |
  +-- E03 (Dashboard frontend) -----------+
       |                                   |
       E04 (Lecteur rapport) [Must work]   E05 (Suivi temps reel)
       |
       E06 (Prompt builder)
       |
       E07 (Polish UI + design system)
       |
       BUF (buffer : corrections + nettoyage)
```

**Parallelisation** :
- E02 (Francois) + E03 (Lise) : parallele apres E01
- E04 (Lise) + E05 (Francois) : parallele apres E03+E02

---

## Table des etapes

| Etape | Titre | Agent(s) | Parallele | Taches |
|-------|-------|----------|-----------|--------|
| E00 | Bootstrap workspace Tauri | Francois | non | 4 |
| E01 | Backend core (SQLite + IPC) | Francois | non | 6 |
| E02 | File watcher + indexation | Francois | avec E03 | 4 |
| E03 | Dashboard frontend | Lise | avec E02 | 5 |
| E04 | Lecteur rapport (Must work) | Lise | apres E03 | 5 |
| E05 | Suivi temps reel | Francois | avec E04 | 4 |
| E06 | Prompt builder | Lise+Francois | apres E04 | 5 |
| E07 | Polish UI + design system | Lise | apres E06 | 4 |
| BUF | Buffer corrections | Francois+Lise | non | variable |

---

## Conventions

- Branche : `feat/mipower-workflow-editor`
- Commit prefix : `feat(mipower): E<NN> -- <description>`
- Tests : `cargo test -p mipower-tauri` (back) + `bun run test` (front)
- Lint : `cargo clippy --all-targets -- -D warnings` + `bun run check`
- Push : apres chaque commit (TDD pas 8)
- Checkpoint Denis : toutes les 5 taches

---

## Annotations (TDD)

Format par tache :
```
> Demarre a HH:MM:SS. Termine a HH:MM:SS avec [model] pour N tokens (mesures).
```

---

## Etapes — Fiches

Voir `plans_p3/etapes/` pour le detail de chaque etape.

| Fichier | Etape |
|---------|-------|
| etape-00.md | E00 Bootstrap |
| etape-01.md | E01 Backend core |
| etape-02.md | E02 File watcher |
| etape-03.md | E03 Dashboard |
| etape-04.md | E04 Lecteur rapport |
| etape-05.md | E05 Suivi temps reel |
| etape-06.md | E06 Prompt builder |
| etape-07.md | E07 Polish UI |
| etape-buf.md | BUF buffer |
