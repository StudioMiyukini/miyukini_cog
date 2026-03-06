# E7 -- WOPI (Office Online)

## Statut : Termine
## Depend de : E3
## Agents : Hugo, Lise
## Taches : 6

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E7-01 | CODE | WOPI discovery endpoint (CheckFileInfo) | Hugo | crates/miyucloud-dav/src/wopi/discovery.rs | done |
| E7-02 | CODE | WOPI GetFile / PutFile handlers | Hugo | crates/miyucloud-dav/src/wopi/handlers.rs | done |
| E7-03 | CODE | WOPI token validation et proof keys | Hugo | crates/miyucloud-dav/src/wopi/auth.rs | done |
| E7-04 | CODE | wopi_router() axum integration | Hugo | crates/miyucloud-dav/src/wopi/mod.rs | done |
| E7-05 | TEST-U | Tests CheckFileInfo, token validation | Lise | tests/ | done |
| E7-06 | TEST-I | Test integration: WOPI flow complet (mock Office Online) | Lise | tests/ | deferred-E9 |


## Commit message template
`feat(miyucloud-dav): E7 -- WOPI discovery, GetFile/PutFile, proof keys`
