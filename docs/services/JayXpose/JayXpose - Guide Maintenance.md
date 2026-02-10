# JayXpose - Guide Maintenance

## Controles reguliers

- `cargo check -p jayxpose`
- `cargo check -p miyukini-central`
- verification `jayxpose.db`

## Operations courantes

- purge logs anciens `sync_logs`
- sauvegarde sqlite
- verification index et croissance DB

## Incident PoS

1. verifier `sync_logs`
2. lancer pull
3. traiter conflits
4. relancer push
