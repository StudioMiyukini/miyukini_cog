# Plan P3 2026-03-07-sodomight-dev-2

## Statut

- Etat : PRET A EXECUTER
- Phase : P3
- Complexite : C4 — elevee
- Responsable principal : Denis
- Date : 2026-03-07

## TL;DR

6 etapes + BUF pour cabler le pipeline wgpu sprite instancie dans `mge-render` et afficher la scene rogue_camp.

## DAG des etapes

```
E00 (bytemuck dep)
  └─→ E01 (WGSL shader)
        └─→ E02 (SpriteInstanceGpu + SpritePipeline)
              └─→ E03 (GraphicsState cablage)
                    ├─→ E04 (AtlasHandle/MaterialHandle publics)
                    │         └─→ E05 (scene main.rs)
                    │                   └─→ BUF (tests + clippy)
```

## Etapes

---

### E00 — Dependances bytemuck

**Agent :** Denis
**Fichiers :**
- `mge/Cargo.toml`
- `mge/crates/mge-render/Cargo.toml`

**Taches :**
- [ ] Ajouter `bytemuck = { version = "1", features = ["derive"] }` dans `[workspace.dependencies]` de `mge/Cargo.toml`
- [ ] Ajouter `bytemuck.workspace = true` dans `[dependencies]` de `mge/crates/mge-render/Cargo.toml`
- [ ] `cargo check -p mge-render` passe

**Critere de sortie :** `cargo check -p mge-render` vert. `bytemuck` dans `cargo tree -p mge-render`.
**Dependances :** aucune

---

### E01 — Shader WGSL sprite instancie

**Agent :** Denis
**Fichiers :**
- `mge/crates/mge-render/src/shader.wgsl` (NOUVEAU)

**Taches :**
- [ ] Creer `shader.wgsl` selon la spec
- [ ] Group 0 : struct `Viewport { size: vec2<f32> }` uniform binding 0
- [ ] Group 1 : `texture_2d<f32>` binding 0 + sampler binding 1
- [ ] `@location(0)` quad_pos : vertex step
- [ ] `@location(1..4)` instance data : screen_pos, uv, tint, scale (instance step)
- [ ] `vs_main` : pixel → NDC (`px / vp.size * 2.0 - 1.0`, Y flip), UV interpolation
- [ ] `fs_main` : `textureSample(t_sprite, s_sprite, in.tex_uv) * in.tint`
- [ ] Syntaxe WGSL validee manuellement (locations coherentes avec layout E02)

**Critere de sortie :** Fichier complet et coherent avec le layout SpriteInstanceGpu E02.
**Dependances :** E00

---

### E02 — SpriteInstanceGpu + SpritePipeline::new()

**Agent :** Denis
**Fichiers :**
- `mge/crates/mge-render/src/pipeline.rs` (NOUVEAU)
- `mge/crates/mge-render/src/lib.rs` — ajouter `pub mod pipeline;`

**Taches :**
- [ ] Definir `SpriteInstanceGpu` : `#[repr(C)] #[derive(Pod, Zeroable)]`, 48 bytes
  - [ ] Champs : `screen_pos [f32; 2]`, `uv [f32; 4]`, `tint [f32; 4]`, `scale [f32; 2]`, `_pad [f32; 2]`
  - [ ] `const _: () = assert!(size_of::<SpriteInstanceGpu>() == 48);`
- [ ] Definir `pub const MAX_INSTANCES: usize = 16_384;`
- [ ] `SpritePipeline::new(device, queue, format)` :
  - [ ] Vertex buffer quad (6 vertices `[f32; 2]`, statique)
  - [ ] Instance buffer fixe (MAX_INSTANCES x 48 bytes, VERTEX|COPY_DST)
  - [ ] Texture 1x1 blanche RGBA `[255u8; 4]` + `assert_eq!(data.len(), 4)` (Victor rec. #4)
  - [ ] Sampler Nearest/Nearest
  - [ ] Bind group layouts : vp_bgl (viewport uniform) + tex_bgl (texture+sampler)
  - [ ] Bind groups crees depuis les layouts
  - [ ] Viewport uniform buffer `[f32; 2]` (UNIFORM|COPY_DST, 8 bytes)
  - [ ] Shader via `device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"))`
  - [ ] `RenderPipeline` : ALPHA_BLENDING, TriangleList, vertex buffers [quad vertex + instance]
  - [ ] `VertexBufferLayout` instance : stride=48, step_mode=Instance, 4 attributes (loc 1-4)
- [ ] `pub mod pipeline;` dans `lib.rs`
- [ ] `cargo check -p mge-render`

**Critere de sortie :** `cargo check -p mge-render` vert. SpritePipeline compilable.
**Dependances :** E00, E01

---

### E03 — Cabler GraphicsState::render(&batch)

**Agent :** Denis
**Fichiers :**
- `mge/crates/mge-render/src/lib.rs`

**Taches :**
- [ ] Ajouter champ `pipeline: pipeline::SpritePipeline` dans `GraphicsState`
- [ ] Dans `new()` : appeler `pipeline::SpritePipeline::new(&device, &queue, format)` apres `surface.configure`
- [ ] Modifier signature `render` -> `pub fn render(&mut self, batch: &batch::SpriteBatch) -> Result<()>`
- [ ] Corps de `render()` :
  - [ ] Acquerir `output` (inchange)
  - [ ] Creer encoder (inchange)
  - [ ] Clear pass avec `LoadOp::Clear(self.clear_color)` (inchange)
  - [ ] Appeler `self.pipeline.render(&self.queue, &mut encoder, &view, batch, [w as f32, h as f32])`
  - [ ] `queue.submit([encoder.finish()])` ; `output.present()` (inchange)
- [ ] `SpritePipeline::render()` interne :
  - [ ] `queue.write_buffer(&vp_buffer, 0, bytemuck::cast_slice(&viewport))`
  - [ ] Conversion CPU->GPU : `instances[..count.min(MAX_INSTANCES)]` -> `Vec<SpriteInstanceGpu>`
  - [ ] UV = `[0.0, 0.0, 1.0, 1.0]` pour atlas 1x1 (hardcode OK pour P0)
  - [ ] `queue.write_buffer(&instance_buffer, 0, bytemuck::cast_slice(&gpu_data))`
  - [ ] `begin_render_pass` avec `LoadOp::Load` (ne pas ecraser le clear)
  - [ ] `set_pipeline`, `set_bind_group(0, vp)`, `set_bind_group(1, tex)`
  - [ ] `set_vertex_buffer(0, quad)`, `set_vertex_buffer(1, instances)`
  - [ ] `draw(0..6, 0..count as u32)`
- [ ] `cargo check -p mge-render`
- [ ] `cargo test -p mge-render` (anciens tests CPU doivent rester verts)

**Critere de sortie :** `cargo check -p mge-render` + `cargo test -p mge-render` verts.
**Dependances :** E02

---

### E04 — AtlasHandle + MaterialHandle publics

**Agent :** Denis
**Fichiers :**
- `mge/crates/mge-render/src/atlas.rs`

**Taches :**
- [ ] `AtlasHandle` : ajouter `pub fn new(id: u32) -> Self { Self(id) }`
- [ ] `AtlasHandle` : remplacer `new_test` par alias `#[cfg(test)] pub(crate) fn new_test(id: u32) -> Self { Self::new(id) }`
- [ ] Meme traitement pour `MaterialHandle`
- [ ] `cargo test -p mge-render` vert (verifier que `new_test` reste utilisable dans tests existants)
- [ ] `cargo clippy -p mge-render -- -D warnings` propre

**Critere de sortie :** `AtlasHandle::new(0)` compilable sans `#[cfg(test)]`. Tests existants verts.
**Dependances :** E03

---

### E05 — Scene rogue_camp dans main.rs

**Agent :** Lise
**Fichiers :**
- `mge/games/sodomight/src/main.rs`

**Taches :**
- [ ] Imports : `mge_render::batch::{RenderLayer, SortKey, SpriteInstance, SpriteBatch}`, `mge_render::atlas::{AtlasHandle, MaterialHandle, SpriteRect}`, `mge_render::camera::IsoCamera`
- [ ] Ajouter champ `batch: SpriteBatch` dans `SodomightApp`
- [ ] Ajouter champ `camera: IsoCamera` avec `focus: [8.0, 8.0]` (centre grille)
- [ ] Constantes handles : `AtlasHandle::new(0)`, `MaterialHandle::new(0)`, `MaterialHandle::new(1)`, `MaterialHandle::new(2)`
- [ ] Dans `RedrawRequested`, avant `renderer.render()` :
  - [ ] `self.batch.clear()`
  - [ ] Grille 16x16 : boucle `(tx 0..16, ty 0..16)`, `cam.world_to_screen(tx, ty)`, layer Terrain, tint brun/vert alternes, scale [80.0, 40.0]
  - [ ] Joueur : `world_to_screen(8.0, 8.0)`, layer Entities, tint [1.0, 0.15, 0.15, 1.0], scale [48.0, 48.0]
  - [ ] HUD sante : screen_pos fixe [16.0, h-40.0], layer UiScreen, tint [0.85, 0.1, 0.1, 1.0], scale [120.0, 16.0]
  - [ ] HUD mana : screen_pos fixe [16.0, h-20.0], layer UiScreen, tint [0.1, 0.3, 0.9, 1.0], scale [80.0, 16.0]
  - [ ] `self.batch.sort()`
- [ ] `renderer.render(&self.batch)` (nouvelle signature)
- [ ] `cargo check -p sodomight`

**Critere de sortie :** `cargo check -p sodomight` vert. 259 instances pushees par frame.
**Dependances :** E03, E04

---

### BUF — Tests + clippy + revue finale

**Agent :** Francois (review) + Denis (corrections)
**Fichiers :**
- `mge/crates/mge-render/src/pipeline.rs`
- Corrections si necessaire

**Taches :**
- [ ] Tests `#[cfg(test)]` dans `pipeline.rs` :
  - [ ] `sprite_instance_gpu_size` : `assert_eq!(size_of::<SpriteInstanceGpu>(), 48)`
  - [ ] `vertex_layout_stride` : stride == 48 dans `desc()`
  - [ ] `max_instances_invariant` : `assert!(MAX_INSTANCES <= 65_536)`
  - [ ] `gpu_instance_fields` : conversion minimale, verifier `screen_pos` + `tint` corrects
- [ ] `cargo test -p mge-render` : 0 failed (anciens + nouveaux)
- [ ] `cargo clippy -p mge-render -- -D warnings` : 0 violation
- [ ] `cargo clippy -p sodomight -- -D warnings` : 0 violation
- [ ] Revue : aucun `unsafe` dans les fichiers modifies
- [ ] `cargo run -p sodomight` : fenetre non-vide (grille iso visible a l'ecran)

**Critere de sortie :** Tests verts, clippy propre, rendu visible.
**Dependances :** E05

---

## Agents mobilises

| Agent | Role | Etapes |
|-------|------|--------|
| Denis | Implementation GPU (pipeline, shaders, wiring) | E00, E01, E02, E03, E04 |
| Lise | Scene initiale (main.rs, batch populate) | E05 |
| Francois | Review, tests BUF | BUF |

## Risques P3

| Risque | Proba | Mitigation |
|--------|-------|-----------|
| WGSL erreur detectable seulement au runtime | Moyenne | `cargo run` obligatoire en BUF avant validation |
| `bytemuck::Pod` panic si struct malformee | Faible | Assert compile-time en E02 |
| Signature `render()` casse `main.rs` | Certaine (attendue) | E03 + E05 coordonnes — E03 avant E05 |
| UV flip (sprites inverses verticalement) | Faible | Shader flippe V si observe a l'execution |
| `AtlasHandle::new` dans const context | Faible | `const fn` requis — verifier si stable sur Rust edition 2021 |

## Criteres de sortie P3

- [ ] Toutes les etapes E00-BUF Terminees
- [ ] `cargo test -p mge-render` : 0 failed
- [ ] `cargo clippy -p mge-render -- -D warnings` : 0 violations
- [ ] `cargo clippy -p sodomight -- -D warnings` : 0 violations
- [ ] `cargo run -p sodomight` : grille iso + joueur + HUD visibles
- [ ] Score securite >= 88/100
- [ ] Score efficience >= 15/20
