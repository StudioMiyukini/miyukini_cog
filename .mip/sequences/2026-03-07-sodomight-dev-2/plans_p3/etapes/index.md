# Index des etapes P3

## Vue d'ensemble

52 taches, 6 etapes + buffer (20%). DAG: E00 -> E01 -> E02 -> E03 -> E04 -> E05 -> BUF
**Statut : TERMINE — toutes les etapes sont DONE.**

## Etapes

| Fichier | Etape | Titre | Taches | Statut |
|---------|-------|-------|--------|--------|
| etape-00.md | E00 | Dependances bytemuck | 3/3 | DONE |
| etape-01.md | E01 | Shader WGSL sprite instancie | 7/7 | DONE |
| etape-02.md | E02 | SpriteInstanceGpu + SpritePipeline::new() | 12/12 | DONE |
| etape-03.md | E03 | Cabler GraphicsState::render(&batch) | 10/10 | DONE |
| etape-04.md | E04 | AtlasHandle + MaterialHandle publics | 5/5 | DONE |
| etape-05.md | E05 | Scene rogue_camp dans main.rs | 9/9 | DONE |
| etape-buf.md | BUF | Tests + clippy + validation visuelle | 6/6 | DONE |

## Gates BIG_STEPS

- **Gate #1** : apres E03 (pipeline GPU cable) — PASSE (user "go full auto")
- **Gate #2** : apres E05 (scene visible) — PASSE (user "go full auto")
