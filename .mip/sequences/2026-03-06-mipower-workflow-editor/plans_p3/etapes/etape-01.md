# E01 -- Backend core (SQLite + modeles + IPC base)

## Statut : Terminé
## Depend de : E00
## Agents : Francois
## Taches : 6
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E01-01 | CODE | Definir structs Rust (SequenceMeta, SequenceDetail, Artefact, Metrics) avec serde | Francois | src-tauri/src/models.rs | pending | -- | -- |
| E01-02 | CODE | Implementer `db::open()` : ouvre/cree mipower.db + migrations (CREATE TABLE IF NOT EXISTS) | Francois | src-tauri/src/db.rs | pending | -- | -- |
| E01-03 | TEST-U | Tests unitaires db::open() : fichier cree, schema present, idempotent | Francois | src-tauri/tests/db_tests.rs | pending | -- | -- |
| E01-04 | CODE | Implémenter `commands::sequences::list_sequences(mip_root)` : lit sequences/index.json -> upsert SQLite -> retourne Vec<SequenceMeta> | Francois | src-tauri/src/commands/sequences.rs | pending | -- | -- |
| E01-05 | CODE | Implementer `commands::sequences::read_artefact(path)` : valide chemin sous mip_root -> lit .md UTF-8 -> retourne String | Francois | src-tauri/src/commands/sequences.rs | pending | -- | -- |
| E01-06 | TEST-U | Tests : list_sequences avec index.json fixture, read_artefact path traversal rejete | Francois | src-tauri/tests/sequences_tests.rs | pending | -- | -- |

## Commit message template
`feat(mipower): E01 -- backend SQLite + IPC list_sequences + read_artefact`

## Criteres de completion
- mipower.db cree au lancement avec les 3 tables
- list_sequences retourne les sequences du fixture index.json
- read_artefact rejette tout chemin hors mip_root (test securite)
- 0 clippy warning
