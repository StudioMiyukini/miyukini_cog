# Specification technique -- Refonte MiyuCloud/OxiCloud

**Sequence** : 2026-03-06-miyucloud-oxicloud-refonte
**Auteur** : Francois (Dev Back-End) -- P0 T6
**Debut** : 2026-03-06T14:05:06Z | **Fin** : 2026-03-06T14:09:28Z

## TL;DR
Nouveau crate `crates/miyucloud-dav/` (WebDAV/CalDAV/CardDAV/WOPI) + extensions dans `crates/miyucloud/` (dedup, compression, thumbnails). 6 nouvelles tables SQLite. ~2680 LOC nouvelles. Architecture Clean OxiCloud mappee sur pattern COG. Score securite cible 96/100. WAL mode SQLite pour concurrence.

## Decision structurelle
Nouveau crate `miyucloud-dav` depend de `miyucloud`. WebDAV/CalDAV/CardDAV dans le nouveau crate. Dedup/thumbnails dans l'existant.

## Fichiers
- 12 fichiers existants modifies
- 26 fichiers nouveaux
- ~2,680 LOC nouvelles, ~120 LOC modifiees

## Types cles
- Calendar, CalendarEvent (CalDAV)
- AddressBook, Contact (CardDAV)
- ContentBlob, ContentHash (dedup)
- ContentAddressableStorage trait

## Securite integree (Victor T5)
- CVE-2025-6965 : bumper rusqlite
- Zeroize KeyManager master_key
- xml_security.rs : limite taille/profondeur, pas DTD
- path_validator.rs : whitelist WebDAV paths
- WAL mode SQLite pour concurrence

## 7 risques techniques documentes
Detail complet : voir document spec agent Francois T6 (conserve dans contexte sequence).
