# E1 -- Schema SQL & Types domaine

## Statut : Termine
## Depend de : E0
## Agents : Francois, Lise
## Taches : 10

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E1-01 | CODE | Creer types Calendar, CalendarEvent (domaine CalDAV) | Francois | crates/miyucloud-dav/src/caldav/types.rs | done |
| E1-02 | CODE | Creer types AddressBook, Contact (domaine CardDAV) | Francois | crates/miyucloud-dav/src/carddav/types.rs | done |
| E1-03 | CODE | Creer types ContentBlob, ContentHash (dedup) | Francois | crates/miyucloud/src/domain/dedup_ops.rs | done |
| E1-04 | CODE | Definir trait ContentAddressableStorage | Francois | crates/miyucloud/src/storage/dedup.rs | done |
| E1-05 | CODE | Schema SQL: table cloud_calendars + cloud_calendar_events | Francois | crates/miyucloud-dav/src/caldav/schema.rs | done |
| E1-06 | CODE | Schema SQL: table cloud_addressbooks + cloud_contacts | Francois | crates/miyucloud-dav/src/carddav/schema.rs | done |
| E1-07 | CODE | Schema SQL: table cloud_content_blobs + cloud_file_blobs | Francois | crates/miyucloud/src/data/kindmother_db.rs | done |
| E1-08 | CODE | Migration SQLite: WAL mode + PRAGMA foreign_keys + busy_timeout | Francois | crates/miyucloud/src/data/kindmother_db.rs | done |
| E1-09 | TEST-U | Tests unitaires types domaine (Calendar, Contact, ContentBlob) | Lise | crates/miyucloud-dav/tests/ | done |
| E1-10 | TEST-U | Tests schema SQL: creation tables, contraintes FK, index | Lise | crates/miyucloud-dav/tests/ | done |

## Commit message template
`feat(miyucloud-dav): E1 -- schema SQL 6 tables, types domaine CalDAV/CardDAV/dedup`
