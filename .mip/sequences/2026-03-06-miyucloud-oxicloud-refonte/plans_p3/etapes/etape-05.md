# E5 -- CardDAV

## Statut : Termine
## Depend de : E3
## Agents : Francois, Lise
## Taches : 7

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E5-01 | CODE | CardDAV REPORT handler (addressbook-query, addressbook-multiget) | Francois | crates/miyucloud-dav/src/carddav/handlers.rs | done |
| E5-02 | CODE | vCard parser/serializer (VCARD 3.0/4.0) | Francois | crates/miyucloud-dav/src/carddav/vcard.rs | done |
| E5-03 | CODE | CardDAV CRUD: creer/lire/modifier/supprimer carnet d'adresses | Francois | crates/miyucloud-dav/src/carddav/service.rs | done |
| E5-04 | CODE | CardDAV CRUD: creer/lire/modifier/supprimer contact | Francois | crates/miyucloud-dav/src/carddav/service.rs | done |
| E5-05 | CODE | carddav_router() axum integration | Francois | crates/miyucloud-dav/src/carddav/mod.rs | done |
| E5-06 | TEST-U | Tests vCard parse/serialize, CRUD contacts | Lise | tests/ | done |
| E5-07 | TEST-I | Test integration: CardDAV flow complet (DAVx5-like) | Lise | tests/ | deferred-E9 |


## Commit message template
`feat(miyucloud-dav): E5 -- CardDAV REPORT, vCard parser, CRUD carnets/contacts`
