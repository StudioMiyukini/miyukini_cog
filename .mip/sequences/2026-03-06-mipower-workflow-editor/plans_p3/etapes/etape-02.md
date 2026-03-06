# E02 -- File watcher + indexation

## Statut : A faire
## Depend de : E01
## Agents : Francois
## Taches : 4
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E02-01 | CODE | Implementer watcher Rust (notify-debouncer-mini, debounce 500ms) sur le dossier sequences/ | Francois | src-tauri/src/commands/watcher.rs | pending | -- | -- |
| E02-02 | CODE | Emetre event Tauri `sequence-updated` avec payload { slug, path } a chaque changement detecte | Francois | src-tauri/src/commands/watcher.rs | pending | -- | -- |
| E02-03 | CODE | Commande `reindex(mip_root)` : re-scanne sequences/ + met a jour SQLite (upsert) | Francois | src-tauri/src/commands/sequences.rs | pending | -- | -- |
| E02-04 | TEST-U | Test : watcher emet un event apres modification d'un fichier dans le dossier surveille (test avec tempdir) | Francois | src-tauri/tests/watcher_tests.rs | pending | -- | -- |

## Commit message template
`feat(mipower): E02 -- file watcher sequences/ + reindex command`

## Criteres de completion
- Modifier un .md dans sequences/ -> event `sequence-updated` emis en < 600ms
- reindex() met a jour SQLite sans duplicat
- 0 clippy warning
