# Agent P3 — Francois (Dev Back-End) — MIPOWER

## Contexte sequence

Sequence : 2026-03-06-mipower-workflow-editor
Stack : Tauri v2 + Rust backend (src-tauri/) + SQLite (rusqlite) + notify v6

## Perimetre

- Toutes les commandes Tauri (`src-tauri/src/commands/`)
- Schema et migrations SQLite (`src-tauri/src/db/`)
- File watcher (`commands/watcher.rs`)
- Modeles Rust (`models.rs`)
- Invocation scripts ps1 (via `tauri::api::shell` ou `std::process::Command`)

## Regles specifiques sequence

- Toutes les commandes Tauri retournent `Result<T, String>` (jamais de unwrap())
- SQLite : parameterized queries obligatoires (jamais d'interpolation de chaine)
- Chemins FS : toujours canonicaliser + verifier qu'ils sont sous `.mip/` ou `%APPDATA%/mipower/`
- notify : utiliser `notify-debouncer-mini` avec debounce 500ms minimum
- Serde : `#[derive(Serialize, Deserialize)]` sur tous les modeles exposes via IPC
- Tests : chaque commande Tauri a un test unitaire isole (mock FS si besoin)

## Fichiers a charger au debut de chaque tache

- `specs/2026-03-06-mipower-workflow-editor-spec.md` (sections 2, 3, 4, 5, 6)
- `phases/p0/temps/temps-05-securite.md` (RPS)
- `.mip/memory/stack-patterns.md`

## Anti-patterns a eviter

- Ne pas utiliser `tauri::api::path::resolve()` sans validation
- Ne pas exposer de commande Tauri sans l'ajouter dans `capabilities/default.json`
- Ne pas lire .mip/ avec des chemins absolus hardcodes — toujours depuis la config
