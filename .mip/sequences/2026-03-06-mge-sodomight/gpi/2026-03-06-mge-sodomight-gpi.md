# GPI securite - mge-sodomight

## Menaces prioritaires

1. Installation d'un package falsifie
2. Save editing et corruption de progression
3. RPC local non valide entre Central et Sodomight
4. Reutilisation d'assets ou de manifests alteres
5. Future triche coop si le host n'est pas autoritaire

## Mesures obligatoires

- hash et checksum du package
- validation stricte du manifeste
- versioning de saves
- schema validation des data packs
- host autoritaire pour coop
- messages reseau valides et tailles bornees

## Mesures P3

- crate `mge-save` avec header versionne
- signature ou checksum des data packs
- validation forte du chargement des assets
- separation du binaire de jeu et des contenus data

## Mesures P4

- audits sur package installable
- fuzzing basique parsers data/save
- tests d'integrite de chargement

## Go / No-Go

- pas de lancement online sans couche `mge-net` autoritaire
- pas de distribution publique sans checksum package
