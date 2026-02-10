# JayXpose - Integration JayFestival

## Scope

- profil exposant
- catalogue expose
- fiche publique
- partage documents via mandat

## Flux

1. JayXpose prepare profil/catalogue
2. JayFestival consomme payload d'exposition
3. statut et trace dans `sync_logs`

## Audit

- source: `jayfestival`
- type: `profil_pull` ou `catalog_publish`
