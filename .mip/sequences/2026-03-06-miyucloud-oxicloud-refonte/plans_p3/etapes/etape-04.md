# E4 -- CalDAV

## Statut : Termine
## Depend de : E3
## Agents : Francois, Lise
## Taches : 8

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E4-01 | CODE | CalDAV REPORT handler (calendar-query, calendar-multiget) | Francois | crates/miyucloud-dav/src/caldav/handlers.rs | done |
| E4-02 | CODE | iCalendar parser/serializer (VCALENDAR, VEVENT) | Francois | crates/miyucloud-dav/src/caldav/ical.rs | done |
| E4-03 | CODE | CalDAV CRUD: creer/lire/modifier/supprimer calendrier | Francois | crates/miyucloud-dav/src/caldav/service.rs | done |
| E4-04 | CODE | CalDAV CRUD: creer/lire/modifier/supprimer evenement | Francois | crates/miyucloud-dav/src/caldav/service.rs | done |
| E4-05 | CODE | xml_security.rs: desactiver DTD, limite taille/profondeur | Francois | crates/miyucloud-dav/src/common/xml_security.rs | done |
| E4-06 | CODE | caldav_router() axum integration | Francois | crates/miyucloud-dav/src/caldav/mod.rs | done |
| E4-07 | TEST-U | Tests iCalendar parse/serialize, CRUD calendrier | Lise | tests/ | done |
| E4-08 | TEST-I | Test integration: CalDAV flow complet (Thunderbird-like) | Lise | tests/ | deferred-E9 |


## Commit message template
`feat(miyucloud-dav): E4 -- CalDAV REPORT, iCal parser, CRUD calendriers/evenements`
