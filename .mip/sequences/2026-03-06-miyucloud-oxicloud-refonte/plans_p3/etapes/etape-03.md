# E3 -- WebDAV Core

## Statut : Termine
## Depend de : E1
## Agents : Francois, Hugo, Lise, Victor
## Taches : 12

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E3-01 | CODE | WebDAV request parser (PROPFIND, PROPPATCH, MKCOL, etc.) | Francois | crates/miyucloud-dav/src/webdav/parser.rs | done |
| E3-02 | CODE | WebDAV response builder (multistatus XML) | Francois | crates/miyucloud-dav/src/webdav/response.rs | done |
| E3-03 | CODE | WebDAV property storage (dead properties) | Francois | crates/miyucloud-dav/src/webdav/properties.rs | done |
| E3-04 | CODE | PROPFIND handler (depth 0, 1, infinity) | Francois | crates/miyucloud-dav/src/webdav/handlers.rs | done |
| E3-05 | CODE | GET/PUT/DELETE handlers WebDAV | Francois | crates/miyucloud-dav/src/webdav/handlers.rs | done |
| E3-06 | CODE | MKCOL/COPY/MOVE handlers | Francois | crates/miyucloud-dav/src/webdav/handlers.rs | done |
| E3-07 | CODE | LOCK/UNLOCK handlers (optionnel class 2) | Francois | crates/miyucloud-dav/src/webdav/lock.rs | deferred |
| E3-08 | CODE | webdav_router() axum integration + DavStore State | Hugo | crates/miyucloud-dav/src/webdav/router.rs | done |
| E3-09 | CODE | path_validator.rs: whitelist, rejet .., normalisation | Victor | crates/miyucloud-dav/src/common/path_validator.rs | done |
| E3-10 | TEST-U | Tests parser/response XML WebDAV | Lise | tests/ | done |
| E3-11 | TEST-I | Test integration: PROPFIND/GET/PUT via reqwest | Lise | tests/ | deferred-E9 |
| E3-12 | TEST-S | Tests path traversal (../, encoded, unicode) | Victor | tests/ | done |

## Commit message template
`feat(miyucloud-dav): E3 -- WebDAV core handlers, router, path validation`
