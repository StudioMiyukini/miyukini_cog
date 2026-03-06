# 10 - Rendu, assets et outils MGE

## Stack low-level retenue

- fenetre et evenements : `winit`
- GPU abstraction : `wgpu`
- audio runtime : `kira`
- collision/physics utilitaires : `Rapier` seulement si utile, sinon logique custom plus simple

## Renderer cible

- sprite renderer proprietaire data-driven
- frame graph simple et explicite
- atlas textures + texture arrays selon families
- camera isometrique configurable
- visibility/culling par chunks et cellules
- layers:
  - terrain
  - decals
  - props
  - entities
  - projectiles
  - VFX opaques
  - VFX alpha
  - UI world-space
  - UI screen-space
- lighting stylisee:
  - palette driven
  - emissive local
  - fog / darkness zones
  - normal maps optionnelles pour highlights et torches
  - color grading / LUT final

## Animation

- spritesheets bakees
- clips directionnels
- notify events:
  - active frame
  - footstep
  - cast release
  - projectile spawn
  - SFX cue
- blend minimum entre idle/walk/run et action locks

## Assets et textures

- assets generes en interne
- reference visuelle D2 sans reprise directe de fichiers tiers
- pipeline source:
  - concept sheets
  - blockout 3D ou paintover source
  - spritesheets
  - UI frames
  - icons
  - VFX sheets
- pipeline bake:
  - trim
  - atlas
  - metadata JSON/RON
  - hash
  - validation taille / naming

## Direction visuelle Sodomight

- dark fantasy medievale sale et lisible
- silhouettes franches a 8 directions
- valeurs contrastees, accents de feu, poison, sang et sorcellerie
- palette chaude/sombre avec variantes froides pour caveaux et catacombes
- UI gothic fonctionnelle, originale, plus epuree que D2
- aucune silhouette, rune, bordure UI ou texture ne doit etre un clone exact de D2

## Outils requis

- `mge-asset-baker`
- validateur de spritesheets
- previewer animation
- editeur de data tables
- captureur de thumbnails pour Central/Market
- overlay GPU timings / overdraw
- inspecteur materials / atlases / quality tiers

## Exigences techniques

- integer scaling optionnel
- rendu net a 60+ fps
- mode faible spec
- support multi-resolution
- compilation d'assets reproductible
- recovery device lost / resize / swapchain reset
- streaming asynchrone des textures et atlases
- budgets memoire et draw calls documentes
- tiers qualite `low`, `standard`, `high`
