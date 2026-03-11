# E02 -- SpriteInstanceGpu + SpritePipeline::new()

## Statut : A faire
## Depend de : E00, E01
## Agents : Denis
## Taches : 12
## Commence : --
## Fini : --

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commence | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E02-01 | CODE | Creer pipeline.rs, ajouter use bytemuck::{Pod, Zeroable} | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E02-02 | CODE | Definir SpriteInstanceGpu #[repr(C)] Pod Zeroable (48 bytes) | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E02-03 | CODE | Assert compile-time : size_of == 48 | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E02-04 | CODE | Definir const MAX_INSTANCES: usize = 16_384 | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E02-05 | CODE | Definir QUAD_VERTICES (6 vertices [f32; 2]) | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E02-06 | CODE | SpritePipeline::new() — vertex buffer quad statique | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E02-07 | CODE | SpritePipeline::new() — instance buffer fixe (MAX_INSTANCES * 48) | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E02-08 | CODE | SpritePipeline::new() — texture 1x1 blanche + assert len==4 + sampler | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E02-09 | CODE | SpritePipeline::new() — bind group layouts (viewport + texture) + bind groups | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E02-10 | CODE | SpritePipeline::new() — shader module via include_wgsl! | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E02-11 | CODE | SpritePipeline::new() — RenderPipeline (ALPHA_BLENDING, TriangleList, vertex layouts) | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E02-12 | CHECK | Ajouter pub mod pipeline dans lib.rs + cargo check -p mge-render | Denis | `mge/crates/mge-render/src/lib.rs` | pending | -- | -- |

## Critere de sortie
`cargo check -p mge-render` vert. `SpritePipeline` compilable. Struct `SpriteInstanceGpu` validee par assert size.

## Commit message template
`feat(mge-render): E02 -- SpriteInstanceGpu + SpritePipeline::new()`
