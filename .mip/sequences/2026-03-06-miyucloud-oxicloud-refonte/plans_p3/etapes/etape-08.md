# E8 -- Integration Central

## Statut : Termine
## Depend de : E3, E4, E5
## Agents : Hugo, Lise
## Taches : 6

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E8-01 | CODE | Composant Central: explorateur fichiers WebDAV | Hugo | apps/central/src/services/miyucloud/explorer.rs | done |
| E8-02 | CODE | Composant Central: vue calendrier CalDAV | Hugo | apps/central/src/services/miyucloud/calendar_view.rs | done |
| E8-03 | CODE | Composant Central: vue contacts CardDAV | Hugo | apps/central/src/services/miyucloud/contacts_view.rs | done |
| E8-04 | CODE | Integration auth Miyukini Connect dans flux DAV | Hugo | apps/central/src/services/miyucloud/client.rs | done |
| E8-05 | TEST-U | Tests composants Central (mock API) | Lise | tests/ | deferred-E9 |
| E8-06 | TEST-I | Test integration: navigation Central -> upload -> calendrier | Lise | tests/ | deferred-E9 |

## Commit message template
`feat(central): E8 -- integration MiyuCloud DAV dans Central UI`
