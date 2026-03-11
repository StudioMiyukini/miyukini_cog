# Specification 2026-03-07-sodomight-dev-2

## Statut

- Etat : COMPLET
- Phase : P0 Temps 6
- Responsable principal : Francois
- Date : 2026-03-07

## TL;DR

Pipeline wgpu sprite instancie pour `mge-render`. Texture 1x1 blanche + tint par instance.
Scene initiale : grille isometrique 16x16 + joueur + HUD rectangles.

---

## Architecture

```
mge/crates/mge-render/
  src/
    lib.rs          [MODIFIE] GraphicsState + pipeline + render(&batch)
    pipeline.rs     [NOUVEAU] SpritePipeline + SpriteInstanceGpu + MAX_INSTANCES
    shader.wgsl     [NOUVEAU] WGSL vertex + fragment
    atlas.rs        [MODIFIE] AtlasHandle::new(u32) public
    batch.rs        [INCHANGE]
    camera.rs       [INCHANGE]
    pass.rs         [INCHANGE]
    ...

mge/games/sodomight/
  src/
    main.rs         [MODIFIE] SpriteBatch populate + render(&batch)

mge/Cargo.toml      [MODIFIE] bytemuck workspace dep
mge/crates/mge-render/Cargo.toml  [MODIFIE] bytemuck dep
```

---

## Types Rust cles a definir

### `SpriteInstanceGpu` (pipeline.rs)

```rust
/// GPU-side layout pour une instance sprite — 48 bytes, #[repr(C)] Pod.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteInstanceGpu {
    pub screen_pos: [f32; 2],   // offset 0  (pixels, top-left)
    pub uv:         [f32; 4],   // offset 8  (u_min, v_min, u_max, v_max)
    pub tint:       [f32; 4],   // offset 24 (RGBA multiplier)
    pub scale:      [f32; 2],   // offset 40 (width, height en pixels)
    pub _pad:       [f32; 2],   // offset 44 (alignement 48 bytes)
}

// Assertion compile-time (Victor rec. #3)
const _: () = assert!(std::mem::size_of::<SpriteInstanceGpu>() == 48);
```

**Contrainte** : tous les champs f32 ou [f32; N] — aucun bool, enum, Option.

### Constantes (pipeline.rs)

```rust
pub const MAX_INSTANCES: usize = 16_384;  // clamp securite (Victor rec. #1)

pub const QUAD_VERTICES: [[f32; 2]; 6] = [
    [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
    [0.0, 0.0], [1.0, 1.0], [0.0, 1.0],
];
```

---

## Schema de donnees — Shader WGSL (shader.wgsl)

```wgsl
// Group 0 — viewport uniform
struct Viewport { size: vec2<f32> }
@group(0) @binding(0) var<uniform> vp: Viewport;

// Group 1 — texture + sampler
@group(1) @binding(0) var t_sprite: texture_2d<f32>;
@group(1) @binding(1) var s_sprite: sampler;

struct VertIn {
    @location(0) quad_pos:  vec2<f32>,   // [0,1]x[0,1] depuis QUAD_VERTICES
    // Instance data (step_mode = Instance)
    @location(1) screen_pos: vec2<f32>,  // pixels top-left
    @location(2) uv:         vec4<f32>,  // u_min, v_min, u_max, v_max
    @location(3) tint:       vec4<f32>,
    @location(4) scale:      vec2<f32>,  // width, height pixels
}

struct VertOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) tex_uv: vec2<f32>,
    @location(1) tint:   vec4<f32>,
}

@vertex
fn vs_main(in: VertIn) -> VertOut {
    let px = in.screen_pos + in.quad_pos * in.scale;
    let ndc = vec2<f32>(
        px.x / vp.size.x * 2.0 - 1.0,
        1.0 - px.y / vp.size.y * 2.0,
    );
    let uv = vec2<f32>(
        mix(in.uv.x, in.uv.z, in.quad_pos.x),
        mix(in.uv.y, in.uv.w, in.quad_pos.y),
    );
    return VertOut(vec4<f32>(ndc, 0.0, 1.0), uv, in.tint);
}

@fragment
fn fs_main(in: VertOut) -> @location(0) vec4<f32> {
    let col = textureSample(t_sprite, s_sprite, in.tex_uv);
    return col * in.tint;
}
```

---

## API / Interfaces — SpritePipeline (pipeline.rs)

```rust
pub struct SpritePipeline {
    pipeline:         wgpu::RenderPipeline,
    vertex_buffer:    wgpu::Buffer,         // quad statique (6 vertices)
    instance_buffer:  wgpu::Buffer,         // MAX_INSTANCES x 48 bytes
    tex_bind_group:   wgpu::BindGroup,      // texture 1x1 + sampler
    vp_bind_group:    wgpu::BindGroup,      // viewport uniform
    vp_buffer:        wgpu::Buffer,         // [f32; 2] = [width, height]
}

impl SpritePipeline {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self;
    pub fn render(
        &self,
        queue:    &wgpu::Queue,
        encoder:  &mut wgpu::CommandEncoder,
        view:     &wgpu::TextureView,
        batch:    &crate::batch::SpriteBatch,
        viewport: [f32; 2],
    );
}
```

**render() comportement :**
1. Mise a jour `vp_buffer` via `queue.write_buffer`
2. Conversion `batch.instances()[..count.min(MAX_INSTANCES)]` -> `Vec<SpriteInstanceGpu>` (UV = [0,0,1,1] pour atlas 1x1)
3. `queue.write_buffer(&instance_buffer, 0, bytemuck::cast_slice(&gpu_data))`
4. `begin_render_pass` avec `LoadOp::Load` (apres le clear brun)
5. `draw(0..6, 0..count as u32)` — 1 draw call, texture 1x1 partagee

**Modifications GraphicsState (lib.rs) :**

```rust
pub struct GraphicsState {
    // champs existants...
    pipeline: crate::pipeline::SpritePipeline,  // NOUVEAU
}

// Nouvelle signature :
pub fn render(&mut self, batch: &crate::batch::SpriteBatch) -> anyhow::Result<()>;
```

---

## Securite

| Recommandation Victor | Mise en oeuvre |
|-----------------------|----------------|
| Clamp MAX_INSTANCES | `instances.len().min(MAX_INSTANCES)` avant write_buffer |
| No unwrap shader | `create_shader_module` retourne Result, propager via `?` |
| Const assert struct | `const _: () = assert!(size_of::<SpriteInstanceGpu>() == 48)` |
| Assert texture data | `assert_eq!(white.len(), 4)` avant write_texture |
| Shader statique | `include_str!("shader.wgsl")` — jamais genere dynamiquement |

Score securite estime : **88/100**

---

## Dependances

### Modifiees

| Cargo.toml | Ajout |
|-----------|-------|
| `mge/Cargo.toml` | `bytemuck = { version = "1", features = ["derive"] }` dans workspace.dependencies |
| `mge-render/Cargo.toml` | `bytemuck.workspace = true` |

### Inchangees

Toutes les autres dependances de `mge-render` (wgpu 28, winit, anyhow, serde, mge-content, pollster) restent identiques.

---

## Composants — Scene initiale rogue_camp (main.rs)

### Contenu du SpriteBatch par frame

| Layer | Contenu | Count |
|-------|---------|-------|
| Terrain | Grille 16x16 tiles iso (brun/vert alternes) | 256 |
| Entities | Joueur (quad rouge 48x48) centre en [8,8] | 1 |
| UiScreen | Barre sante rouge (120x16) | 1 |
| UiScreen | Barre mana bleue (80x16) | 1 |

Total : **259 instances** (sous MAX_INSTANCES=16384)

### Handles production

```rust
use mge_render::atlas::{AtlasHandle, MaterialHandle};

// AtlasHandle::new() public apres E04
const ATLAS_0:     AtlasHandle    = AtlasHandle::new(0);
const MAT_TERRAIN: MaterialHandle = MaterialHandle::new(0);
const MAT_PLAYER:  MaterialHandle = MaterialHandle::new(1);
const MAT_HUD:     MaterialHandle = MaterialHandle::new(2);
```

### Projection isometrique

- Camera : `focus = [8.0, 8.0]`, `pixels_per_unit = 64.0`, `zoom = 1.0`
- Tile (tx, ty) -> `cam.world_to_screen(tx as f32, ty as f32)` -> screen_pos
- Taille tile : `scale = [80.0, 40.0]` (ratio 2:1 isometrique)

### Modifications atlas.rs — AtlasHandle + MaterialHandle publics

```rust
impl AtlasHandle {
    /// Constructeur public production.
    pub fn new(id: u32) -> Self { Self(id) }

    /// Alias test — maintenu pour compatibilite tests existants.
    #[cfg(test)]
    pub(crate) fn new_test(id: u32) -> Self { Self::new(id) }
}
// Meme pattern pour MaterialHandle
```

---

## Criteres d'acceptance

| Critere | Verification |
|---------|-------------|
| Fenetre non-vide (tiles visibles) | Visuel — grille iso coloree a l'ecran |
| IsoCamera cablée via world_to_screen | Code audit dans main.rs |
| SpriteBatch sort() avant render | Code audit |
| Clamp MAX_INSTANCES dans render | Code audit + const assert |
| `cargo test -p mge-render` : 0 failed | CI locale |
| `cargo clippy -p mge-render -D warnings` : 0 | CI locale |
| `cargo clippy -p sodomight -D warnings` : 0 | CI locale |
| Aucun `unsafe` dans les fichiers modifies | Code audit + clippy |
| Score securite >= 88/100 | Audit Victor (BUF) |
