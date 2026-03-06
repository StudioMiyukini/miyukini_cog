# E6 -- Thumbnails

## Statut : Termine
## Depend de : E1
## Agents : Francois, Lise
## Taches : 5

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E6-01 | CODE | Thumbnail generator (image crate: resize, crop) | Francois | crates/miyucloud-dav/src/thumbnails/handler.rs | done |
| E6-02 | CODE | Cache thumbnails (moka in-memory + disque) | Francois | crates/miyucloud-dav/src/thumbnails/cache.rs | done |
| E6-03 | CODE | API endpoint GET /api/files/{id}/thumbnail | Francois | crates/miyucloud-dav/src/thumbnails/handler.rs | done |
| E6-04 | TEST-U | Tests generation thumbnail (PNG, JPEG, formats supportes) | Lise | tests/ | done |
| E6-05 | TEST-I | Test integration: upload image, verifier thumbnail accessible | Lise | tests/ | deferred-E9 |


## Commit message template
`feat(miyucloud): E6 -- thumbnails generation, cache moka, API endpoint`
