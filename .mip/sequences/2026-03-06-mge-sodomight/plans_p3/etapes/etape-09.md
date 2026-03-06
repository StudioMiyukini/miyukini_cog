# Etape 09 - Integration Central, packaging et exploitation locale

## Objectif

Brancher `Sodomight` a `Central` comme jeu standalone installable et operable sans casser l'autonomie de `mge/`.

## Taches

1. Produire le manifeste jeu et les metadonnees Market `Standalone`.
2. Definir la structure exacte du package: binaires, assets, data, manifestes, checksums.
3. Produire les scripts de build/package reproductibles pour `Sodomight`.
4. Verifier l'installation initiale depuis `Central`.
5. Verifier la mise a jour locale et les regles de migration de package.
6. Verifier le lancement, l'arret et la relance depuis `Central`.
7. Stabiliser les chemins runtime pour saves, caches, logs et crash reports.
8. Verifier que les saves survivent a reinstall/update quand la politique choisie l'exige.
9. Poser les signaux minimaux d'etat jeu -> `Central` si necessaires.
10. Documenter les prerequis machine, dossiers attendus et points de diagnostic.
11. Rediger le runbook d'exploitation locale: install, verification, rollback, nettoyage.
12. Ajouter les smoke tests package installe vs build local.

## Documentation de soutien

1. Documenter le contrat packaging `Central` / `Sodomight`.
2. Documenter la topologie des saves, caches, logs et assets installes.
3. Rediger un runbook minimal d'installation, verification et rollback local.

## Criteres de sortie

1. `Central` installe et lance `Sodomight` sans manipulation manuelle.
2. Les saves et assets sont retrouves de maniere stable apres relance.
3. Les procedures d'exploitation locale sont assez claires pour P4/P5.
