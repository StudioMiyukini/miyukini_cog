# 11 - Integration Central, packaging, ops

## Contrainte de base

`mge/` doit rester independant du reste du projet COG, mais les jeux produits doivent s'installer et se lancer depuis `Central`.

## Ce que le depot permet deja

- `Central` sait installer un service via un binaire et un `service.manifest.json`
- le registre des services est stocke dans `%LOCALAPPDATA%/Miyukini-COG/services/`
- chaque service a un dossier dedie avec `bin/`, `data/` et manifeste
- `ExecutionMode::Standalone` est deja supporte par le protocole Market

## Implications pour Sodomight

- `Sodomight` doit etre package comme service Market `Standalone`
- le binaire du jeu vit dans le dossier service installe
- les donnees runtime et saves sont dans `data/`
- `Central` peut servir de launcher, d'ecran d'information et de point de mise a jour

## Manifeste cible

- `id`: `sodomight`
- `name`: `Sodomight`
- `service_type`: `interne_cog`
- `source`: `officiel`
- `execution_mode`: `standalone`
- permissions minimales:
  - `storage`
  - `identity` seulement si online profile ou social
  - `webway` seulement si coop/reseau est active

## Pipeline ops

1. build `mge/games/sodomight`
2. assembler package `.msp`
3. injecter manifeste et checksums
4. publier sur Origin
5. installer via Central
6. lancer depuis Central

## Besoins post-MVP

- patch delta
- rollback de version
- migration de saves
- crash dump lisible
- telemetrie opt-in

## Risques

- couplage excessif entre `Central` et le runtime du jeu
- dependance directe du jeu aux crates UI du hub
- manifeste sous-specifie pour les assets lourds multi-plateforme

## Regle

`Central` lance, installe, desinstalle et suit l'etat; `Sodomight` garde son runtime, ses saves, ses options et sa boucle de jeu.
