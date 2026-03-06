# E05 -- Suivi temps reel

## Statut : A faire
## Depend de : E02 (watcher) + E03 (dashboard)
## Agents : Francois (backend) + Lise (composant)
## Taches : 4
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E05-01 | CODE | Backend : commande `get_progress(seq_path)` : lit temps-*.md -> compte "Etat : TERMINE" -> retourne { phase, done, total } | Francois | src-tauri/src/commands/sequences.rs | pending | -- | -- |
| E05-02 | CODE | Frontend : creer ProgressTracker.svelte : barre de progression par phase (P0 T1-T11, P3 etapes, P4, P5, P6), reactive via store | Lise | src/lib/components/ProgressTracker.svelte | pending | -- | -- |
| E05-03 | CODE | Frontend : subscribe a l'event `sequence-updated` -> appel get_progress -> refresh ProgressTracker | Lise | src/lib/stores/active.ts | pending | -- | -- |
| E05-04 | TEST-U | Test : modifier un temps-*.md avec "Etat : TERMINE" -> get_progress retourne done+1 (test unitaire Rust) | Francois | src-tauri/tests/progress_tests.rs | pending | -- | -- |

## Commit message template
`feat(mipower): E05 -- suivi temps reel progression phases`

## Criteres de completion
- Modifier un temps-*.md -> barre de progression updated en < 1s dans l'UI
- Jauge P0 correcte (11 temps possibles)
- Tests unitaires Rust passent
