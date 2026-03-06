# 22 - Construction du rendu moderne, robuste et scalable

## Cibles non negociables

- rendu isometrique lisible a 60 FPS minimum
- pipeline assez moderne pour tenir la charge sur le long terme
- robustesse face aux erreurs GPU, resize, perte de device et assets manquants
- scalabilite de la scene, des VFX et des resolutions sans reecrire le renderer

## Architecture frame

- simulation en tick fixe
- rendu interpolationnel decouple
- frame graph simple:
  - culling et preparation CPU
  - pass terrain/props opaques
  - pass entites/projectiles
  - pass VFX opaques puis alpha
  - pass lumiere, brouillard et color grading
  - pass UI world-space puis screen-space
- submission GPU triee par material, atlas et blend mode

## Construction detaillee

### Geometrie et draw model

- quads instancies pour sprites et tiles
- chunks monde pre-bakes pour decor statique
- batches dynamiques pour entites, projectiles et VFX
- depth ordering explicite par couche + `sort_key`

### Materiaux 2.5D

- albedo obligatoire
- mask emissive optionnel
- normal map optionnelle pour highlights localises
- mask de teinte pour recolors data-driven
- fog mask pour catacombes, caves et zones maudites

### Eclairage

- direction lumineuse globale stable par biome
- sources ponctuelles pour torches, sorts, portails et braziers
- contact shadow sobre ou blob shadow selon tier
- LUT finale par biome/etat narratif

### Camera et presentation

- camera isometrique stable, zoom borne
- frame pacing prioritaire sur les micro-effets
- tremblements limites et seulement sur evenements majeurs
- integer scaling optionnel pour look plus retro

## Robustesse

- fallback texture/material si asset manquant
- recovery sur `surface lost`, `outdated`, resize et recreations de swapchain
- hot reload reserve au dev, jamais requis pour le runtime release
- validation stricte des metadata avant upload GPU
- telemetrie frame time, draw calls, memoire atlas, overdraw

## Scalabilite

- streaming asynchrone des chunks et atlases
- budgets GPU par tier:
  - `low` : lumiere simple, moins de VFX alpha
  - `standard` : cible de reference
  - `high` : densite VFX, normals optionnelles, LUT plus riche
- culling par cellules de navigation et cellules de rendu
- generation de thumbnails et previews hors runtime principal

## Budget de reference

- 1080p / 60 FPS sur machine standard
- pic scene town inferieur a la scene boss + adds
- plafond initial vise:
  - batches terrain fortement amortis
  - batches dynamiques bornes par famille
  - memoire atlas controlee par familles d'assets

## Regles d'implementation

- aucune logique gameplay critique dans les shaders
- tout effet doit avoir un fallback basique
- aucune feature graphique ne doit imposer de casser la lisibilite combat
- aucune optimisation ne doit rendre la pipeline d'assets opaque
