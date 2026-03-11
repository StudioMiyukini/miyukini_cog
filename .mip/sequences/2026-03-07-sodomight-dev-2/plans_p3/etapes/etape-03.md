# E03 -- Cabler GraphicsState::render(&batch)

## Statut : A faire
## Depend de : E02
## Agents : Denis
## Taches : 10
## Commence : --
## Fini : --

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commence | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E03-01 | CODE | Ajouter champ pipeline: SpritePipeline dans GraphicsState | Denis | `mge/crates/mge-render/src/lib.rs` | pending | -- | -- |
| E03-02 | CODE | Instancier SpritePipeline::new() dans GraphicsState::new() | Denis | `mge/crates/mge-render/src/lib.rs` | pending | -- | -- |
| E03-03 | CODE | Changer signature render() -> render(&mut self, batch: &SpriteBatch) -> Result<()> | Denis | `mge/crates/mge-render/src/lib.rs` | pending | -- | -- |
| E03-04 | CODE | Implementer SpritePipeline::render() — write_buffer viewport | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E03-05 | CODE | SpritePipeline::render() — conversion SpriteInstance -> SpriteInstanceGpu (UV=[0,0,1,1]) | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E03-06 | CODE | SpritePipeline::render() — clamp a MAX_INSTANCES + write_buffer instances | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E03-07 | CODE | SpritePipeline::render() — begin_render_pass LoadOp::Load (apres clear) | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E03-08 | CODE | SpritePipeline::render() — set_pipeline, set_bind_groups, set_vertex_buffers, draw | Denis | `mge/crates/mge-render/src/pipeline.rs` | pending | -- | -- |
| E03-09 | CODE | Appeler pipeline.render() dans GraphicsState::render() apres le clear pass | Denis | `mge/crates/mge-render/src/lib.rs` | pending | -- | -- |
| E03-10 | CHECK | cargo check -p mge-render + cargo test -p mge-render (tests CPU verts) | Denis | -- | pending | -- | -- |

## Critere de sortie
`cargo check -p mge-render` + `cargo test -p mge-render` verts. `render()` accepte un `&SpriteBatch`.

## GATE BIG_STEPS #1
Apres E03, pause pour validation humaine avant E04/E05.

## Commit message template
`feat(mge-render): E03 -- GraphicsState::render(&batch) cable`
