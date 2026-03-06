# P0 Temps 10 - Verification CI/CD

## Statut

- Etat : Termine
- Phase : P0 Temps 10
- Responsable principal : Hugo
- Debut : 2026-03-06T14:16:14Z
- Fin : 2026-03-06T14:20:00Z

## TL;DR

3 modifications obligatoires CI. Temps build supplementaire : +1m50 (cold), +20s (cache). Pas de nouveaux secrets CI. Pas de port supplementaire.

## Modifications CI obligatoires

1. **Paths triggers** : ajouter `crates/miyucloud-dav/**` aux triggers CI
2. **Clippy** : ajouter `clippy -p miyucloud-dav` au pipeline
3. **Tests** : ajouter `test -p miyucloud-dav` au pipeline

## Impact build

- Cold build : +1m50s (principalement crate `image`)
- Cached build : +20s
- Pas de nouveaux secrets CI requis

## Infrastructure

- Pas de port supplementaire : WebDAV/CalDAV/CardDAV sur port 11442 (path-based)
- Binaire unique inchange
- SQLite embarque (pas de service externe)
- Health checks : etendre `/admin/stats` avec metriques CalDAV/CardDAV/dedup
