# 09 - Exigences MGE runtime

## Mission du moteur

MGE doit etre un moteur specialise ARPG 2D/2.5D isometrique concu pour servir `Sodomight` d'abord, puis d'autres jeux du meme spectre ensuite.

## Architecture workspace cible

- `mge/`
  - `crates/mge-core`
  - `crates/mge-ecs`
  - `crates/mge-render`
  - `crates/mge-audio`
  - `crates/mge-input`
  - `crates/mge-nav`
  - `crates/mge-save`
  - `crates/mge-net`
  - `crates/mge-proto`
  - `crates/mge-server-core`
  - `crates/mge-replication`
  - `crates/mge-content`
  - `tools/mge-asset-baker`
  - `games/sodomight`
  - `services/mge-login-gateway` futur
  - `services/mge-realm` futur
  - `services/mge-zone` futur
  - `services/mge-social` futur

## Contraintes runtime

- Rust uniquement
- `unsafe` minimise et encapsule
- tick logique fixe
- rendu decouple du tick
- host/server autoritaire pour coop
- meme coeur de simulation pour solo, coop et MMO futur
- serialisation stable et versionnee
- protocoles reseau versionnes et bornes
- aucun client ne fait foi pour les degats, drops ou progression
- data packs externes pour skills, items, monstres, zones, quetes

## Systemes coeur

- ECS ou architecture data-oriented legere
- event bus
- scheduler de systems par phases
- replication bridge client <-> serveur
- interest management par cellule / zone
- state machine de jeu:
  - boot
  - main menu
  - loading
  - town
  - field
  - pause
  - game over
- save manager
- resource cache

## Outils de debug obligatoires

- console dev
- overlays runtime
- profiler simple
- capture seeds
- replay inputs futur
- visualisation collision / nav
- traces replication / latency / desync
- harness local dedicated server / zone server

## Tests minimaux requis

- tests formules stats / degats
- tests generation items
- tests save roundtrip
- tests pathfinding basiques
- tests schemas data
- smoke test boot + load + spawn + loot
- tests proto reseau et versioning
- test local-host == dedicated-host sur scenarios de base

## Raison de ne pas partir sur un engine tiers complet

- besoin d'un renderer parfaitement controle
- besoin de pipeline data et outils sur-mesure
- besoin d'un runtime cible ARPG et non generaliste
- besoin d'une independance claire vis-a-vis du reste de COG
