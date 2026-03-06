# P0 Temps 6 - Specification technique

## Statut

- Etat : Termine
- Phase : P0 Temps 6
- Responsable principal : Francois
- Debut : 2026-03-06T14:05:06Z
- Fin : 2026-03-06T14:09:28Z

## TL;DR

Nouveau crate `crates/miyucloud-dav/` (WebDAV/CalDAV/CardDAV/WOPI) + extensions dans `crates/miyucloud/` (dedup, compression, thumbnails). 6 nouvelles tables SQLite. ~2680 LOC nouvelles. Architecture Clean OxiCloud mappee sur pattern COG. Score securite cible 96/100. WAL mode SQLite pour concurrence.

## Decision structurelle

Nouveau crate `miyucloud-dav` depend de `miyucloud`. WebDAV/CalDAV/CardDAV dans le nouveau crate. Dedup/thumbnails dans l'existant.

## Fichiers

- 12 fichiers existants modifies
- 26 fichiers nouveaux (22 dans miyucloud-dav, 4 dans miyucloud)
- ~2,680 LOC nouvelles, ~120 LOC modifiees

## Types cles

- `Calendar`, `CalendarEvent` (CalDAV)
- `AddressBook`, `Contact` (CardDAV)
- `ContentBlob`, `ContentHash` (dedup)
- `ContentAddressableStorage` trait (store_blob, read_blob, increment/decrement_refcount, blob_exists)

## Schema SQL -- 6 nouvelles tables

- `cloud_calendars`, `cloud_calendar_events` (CalDAV)
- `cloud_addressbooks`, `cloud_contacts` (CardDAV)
- `cloud_content_blobs`, `cloud_file_blobs` (dedup)
- WAL mode : `PRAGMA journal_mode=WAL; PRAGMA busy_timeout=5000; PRAGMA foreign_keys=ON;`

## Securite integree (Victor T5)

- CVE-2025-6965 : bumper rusqlite
- Zeroize KeyManager master_key
- xml_security.rs : limite taille/profondeur, pas DTD
- path_validator.rs : whitelist WebDAV paths

## Router integration

- `webdav_router()`, `caldav_router()`, `carddav_router()` merges dans le router axum existant
- Paths : `/dav/`, `/caldav/`, `/carddav/`

## Artefact source

Voir [spec.md](../../../specs/2026-03-06-miyucloud-oxicloud-refonte-spec.md)
