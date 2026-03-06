# Spec render - mge-sodomight

## Stack

- `winit` pour fenetre et input platform
- `wgpu` pour GPU backend
- spritesheet batching
- camera isometrique
- passes:
  - culling / prepare
  - world opaque
  - props / entities
  - alpha / VFX
  - lighting
  - color grading
  - UI

## Exigences

- 60 fps cible
- fallback low spec
- hot-reload d'assets en dev
- capture debug overlay
- tiers qualite `low`, `standard`, `high`
- recovery device lost et resize robuste
- budgets GPU documentes

## Data

- `atlas.json`
- `anim_clips.json`
- `materials.json`
- `tileset.json`
- `quality_profiles.json`
- `palette_profiles.json`
