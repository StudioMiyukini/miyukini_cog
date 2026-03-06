<!-- @id mem.patterns.stack
     @do store_stack_specific_patterns
     @role patterns
     @layer memory
     @human Patterns techniques valides par stack -->

# Patterns stack

> Complete depuis P0 T6 -- sequence miyucloud-oxicloud-refonte

## Patterns valides

- PAT-01: Axum 0.8.x router composition via `Router::merge()`
  - Exemple: `app.merge(webdav_router()).merge(caldav_router())`
  - Limites: un seul `Router` final, pas de routes conflictuelles

- PAT-02: KindMother SQLite (rusqlite) pour persistence
  - Exemple: `db.execute("CREATE TABLE IF NOT EXISTS ...", params![])?`
  - Limites: single-writer, WAL mode obligatoire pour concurrence

- PAT-03: Trait-based domain (Clean Architecture OxiCloud)
  - Exemple: `trait ContentAddressableStorage { fn store_blob(...); }`
  - Limites: pas d'async trait sans `async-trait` ou RPITIT

- PAT-04: SHA-256 content-addressable dedup
  - Exemple: `let hash = Sha256::digest(&content); store_blob(hash, content)`
  - Limites: refcount obligatoire pour eviter suppression prematuree

- PAT-05: Dioxus desktop UI avec composants miyuki-ui-dioxus
  - Exemple: `rsx! { MiyuButton { label: "Upload" } }`
  - Limites: pas de CSS-in-JS, utiliser tokens de design system

- PAT-06: Path-based routing pour protocoles DAV (port unique 11442)
  - Exemple: `/dav/` (WebDAV), `/caldav/` (CalDAV), `/carddav/` (CardDAV)
  - Limites: pas de virtual hosts, uniquement path-based

## Anti-patterns

- AP-STACK-01: Ne pas utiliser PostgreSQL (rester SQLite embarque)
- AP-STACK-02: Ne pas creer de processus separe pour DAV (binaire unique)
- AP-STACK-03: Ne pas utiliser `sqlx` -- incompatible avec KindMother
- AP-STACK-04: Ne pas importer le frontend React d'OxiCloud (garder Dioxus)
- AP-STACK-05: Ne pas ajouter Docker/infra OxiCloud (COG a son propre systeme)
