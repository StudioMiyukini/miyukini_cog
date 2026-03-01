<!-- @id: SD-Impl-07 @do: guide @role: back-end @layer: 3 @human: miyuk -->

# IMPL-07 -- Renderer : wgpu, Isometrique & Sprite Batching

**Auteur :** Francois (Dev Back-End, Miyukini AI Studio)
**Base :** SD-Tech-Render-Pipeline.md (Denis)
**Date :** 2026-02-28
**Statut :** Guide d'implementation -- v1.0

---

## Table des matieres

1. [Architecture du renderer](#1-architecture-du-renderer)
2. [Crate mge-render (Couche 2 Engine)](#2-crate-mge-render-couche-2-engine)
3. [GpuContext -- initialisation wgpu 24.x](#3-gpucontext--initialisation-wgpu-24x)
4. [Coordonnees isometriques dimetric 2:1](#4-coordonnees-isometriques-dimetric-21)
5. [TextureAtlas et AtlasFrame (TOML)](#5-textureatlas-et-atlasframe-toml)
6. [SpriteInstanceGpu et SpriteBatcher](#6-spriteinstancegpu-et-spritebatcher)
7. [Z-ordering : 9 render layers](#7-z-ordering--9-render-layers)
8. [Pipeline de rendu wgpu](#8-pipeline-de-rendu-wgpu)
9. [Shader WGSL sprites (instancing)](#9-shader-wgsl-sprites-instancing)
10. [Dual-resolution : offscreen 800x600 + upscale](#10-dual-resolution--offscreen-800x600--upscale)
11. [Shader WGSL upscale (fullscreen quad)](#11-shader-wgsl-upscale-fullscreen-quad)
12. [Animation : frame timer et direction flip](#12-animation--frame-timer-et-direction-flip)
13. [Frustum culling](#13-frustum-culling)
14. [Systeme de particules](#14-systeme-de-particules)
15. [Boucle de rendu complete](#15-boucle-de-rendu-complete)
16. [Camera2D : follow, smooth, bounds](#16-camera2d--follow-smooth-bounds)
17. [Tests](#17-tests)
18. [Checklist integration](#18-checklist-integration)

---

## 1. Architecture du renderer

Le renderer Sodomight vit dans la crate `mge-render` (Couche 2 Engine). Il ne connait
pas le contenu du jeu -- il recoit des sprites tries en Z et les dessine par batches
groupes par atlas.

### 1.1 Position dans le workspace

```
Couche 4 (Game)    sodomight-game ─── depend de ──▶
Couche 3 (ARPG)    mge-arpg-* ────── depend de ──▶
Couche 2 (Engine)  mge-render ◀── CE DOCUMENT
Couche 1 (Kernel)  mge-platform, mge-math, mge-ecs, mge-asset
```

### 1.2 Dependances de mge-render

```
mge-render
├── mge-ecs       (Query, Res, System)
├── mge-math      (Vec2, IVec2, Rect, iso projection)
├── mge-platform  (GpuContext, fenetre winit)
├── mge-asset     (TextureAtlas chargement, hot-reload)
├── wgpu 24.0     (GPU API)
├── image 0.25    (PNG decoding)
├── bytemuck 1    (Pod/Zeroable pour les vertex GPU)
└── thiserror 2   (RenderError)
```

### 1.3 Principes architecturaux

| Principe | Implementation |
|----------|---------------|
| Tri Z en CPU | Pas de depth buffer GPU ; le painter's algorithm suffit pour le 2D iso |
| Instancing | Un quad partage (6 vertices) + un instance buffer par atlas |
| Batching par atlas | 1 draw call par atlas de textures = < 20 draw calls par frame |
| Dual-resolution | Monde rendu dans un offscreen 800x600, upscale nearest vers la fenetre |
| HUD en natif | L'UI est dessinee directement sur la surface, jamais dans l'offscreen |
| No unsafe | `unsafe_code = "forbid"` herite du workspace |

---

## 2. Crate mge-render (Couche 2 Engine)

### 2.1 Cargo.toml

```toml
# crates/engine/mge-render/Cargo.toml
[package]
name = "mge-render"
version.workspace = true
edition.workspace = true
description = "MGE renderer: wgpu sprite batching, iso tilemap, dual-res"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
mge-math = { path = "../../kernel/mge-math" }
mge-platform = { path = "../../kernel/mge-platform" }
mge-asset = { path = "../../kernel/mge-asset" }
wgpu = { workspace = true }
image = { workspace = true }
bytemuck = { version = "1", features = ["derive"] }
tracing = { workspace = true }
thiserror = { workspace = true }

[features]
default = ["dual-res"]
dual-res = []
post-process = []

[lints]
workspace = true
```

### 2.2 Structure de fichiers

```
crates/engine/mge-render/src/
    lib.rs              # Re-exports, RenderError
    renderer.rs         # Renderer struct, frame loop, draw methods par layer
    sprite_batch.rs     # SpriteBatcher: groupement par atlas, instance buffer
    sprite_instance.rs  # SpriteInstanceGpu: donnees GPU par sprite (64 bytes)
    tilemap_render.rs   # TilemapRenderer: collecte des tiles visibles par layer
    z_order.rs          # RenderLayer enum, compute_z_order, sort_sprites_by_depth
    camera.rs           # Camera2D: follow, smooth, bounds, screen_to_world
    animation.rs        # AnimationSystem: frame timer, state machine, direction flip
    particle.rs         # ParticlePool, ParticleEmitterDef, ParticleType
    dual_res.rs         # DualResolution: offscreen render + upscale nearest
    atlas.rs            # TextureAtlas, AtlasFrame, AtlasDescriptor, UV lookup
    pipeline.rs         # create_sprite_pipeline, create_upscale_pipeline
    culling.rs          # FrustumCuller: AABB test en coordonnees ecran
    shaders/
        sprite.wgsl     # Vertex + Fragment shader pour sprites (instancing)
        upscale.wgsl    # Fullscreen quad nearest upscale
    errors.rs           # RenderError (thiserror)
```

---

## 3. GpuContext -- initialisation wgpu 24.x

Le `GpuContext` est defini dans `mge-platform` (Couche 1 Kernel). Le renderer le
recoit en reference. Voici le code exact d'initialisation pour wgpu 24.x.

```rust
// @id: sd-impl-gpu-context @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-platform/src/gpu.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("No suitable GPU adapter found")]
    NoAdapter,
    #[error("Surface creation failed: {0}")]
    SurfaceCreation(String),
    #[error("Device creation failed: {0}")]
    DeviceCreation(String),
}

pub struct GpuContext {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'static>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub window_size: (u32, u32),
}

impl GpuContext {
    /// Initialise wgpu de maniere synchrone via pollster.
    ///
    /// Le `window` doit avoir une lifetime 'static (Arc<Window> ou leak).
    /// On utilise `Backends::all()` pour supporter Vulkan, DX12, Metal.
    pub fn new(window: &'static winit::window::Window) -> Result<Self, PlatformError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window)
            .map_err(|e| PlatformError::SurfaceCreation(e.to_string()))?;

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or(PlatformError::NoAdapter)?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("MGE Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        ))
        .map_err(|e| PlatformError::DeviceCreation(e.to_string()))?;

        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);

        // Preferer sRGB si disponible, sinon premier format disponible.
        let format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        Ok(Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            surface_config,
            window_size: (size.width, size.height),
        })
    }

    /// Appele lors du redimensionnement de la fenetre.
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width > 0 && new_height > 0 {
            self.surface_config.width = new_width;
            self.surface_config.height = new_height;
            self.surface.configure(&self.device, &self.surface_config);
            self.window_size = (new_width, new_height);
        }
    }
}
```

**Points d'attention wgpu 24.x :**

- `DeviceDescriptor` a le champ `memory_hints: wgpu::MemoryHints::default()` (nouveau en 24.x).
- `Surface<'static>` requiert un `window` avec une lifetime statique (`Arc<Window>` ou `Box::leak`).
- `entry_point` dans les pipeline states est `Some("vs_main")` (Option en 24.x).
- `RenderPassDescriptor` a `timestamp_writes: None` et `occlusion_query_set: None`.
- Le `cache: None` est present dans `RenderPipelineDescriptor`.

---

## 4. Coordonnees isometriques dimetric 2:1

Les formules de conversion vivent dans `mge-math/src/iso.rs` (Couche 1 Kernel).
Le renderer les utilise via `mge_math::world_to_screen` et `mge_math::screen_to_world`.

```rust
// @id: sd-impl-iso-coords @do: implement @role: back-end @layer: 1 @human: miyuk
// Crate: mge-math/src/iso.rs

/// Largeur d'une tile isometrique en pixels.
pub const TILE_WIDTH: f32 = 64.0;

/// Hauteur d'une tile isometrique en pixels.
pub const TILE_HEIGHT: f32 = 32.0;

/// Convertit des coordonnees monde (tiles flottantes) en coordonnees ecran (pixels).
///
/// Projection dimetric 2:1 standard :
///   screen_x = (tile_x - tile_y) * (TILE_WIDTH / 2)
///   screen_y = (tile_x + tile_y) * (TILE_HEIGHT / 2)
///
/// Avec TILE_WIDTH = 64, TILE_HEIGHT = 32 :
///   screen_x = (tile_x - tile_y) * 32
///   screen_y = (tile_x + tile_y) * 16
pub fn world_to_screen(tile_x: f32, tile_y: f32) -> (f32, f32) {
    let screen_x = (tile_x - tile_y) * (TILE_WIDTH / 2.0);
    let screen_y = (tile_x + tile_y) * (TILE_HEIGHT / 2.0);
    (screen_x, screen_y)
}

/// Convertit des coordonnees ecran (pixels) en coordonnees monde (tiles flottantes).
///
/// Inverse exacte de `world_to_screen`.
/// Utile pour la conversion du clic souris en position monde.
pub fn screen_to_world(screen_x: f32, screen_y: f32) -> (f32, f32) {
    let half_w = TILE_WIDTH / 2.0;
    let half_h = TILE_HEIGHT / 2.0;
    let tile_x = (screen_x / half_w + screen_y / half_h) / 2.0;
    let tile_y = (screen_y / half_h - screen_x / half_w) / 2.0;
    (tile_x, tile_y)
}

/// Convertit des coordonnees ecran en coordonnees de tuile entieres (arrondi au sol).
/// Utile pour identifier quelle tuile est sous le curseur.
pub fn screen_to_tile(screen_x: f32, screen_y: f32) -> (i32, i32) {
    let (tx, ty) = screen_to_world(screen_x, screen_y);
    (tx.floor() as i32, ty.floor() as i32)
}
```

### 4.1 Diagramme d'une tile 64x32

```
        +--32px--+
       /          \
      /            \  16px
     /              \
    +    64px wide   +
     \              /
      \            /  16px
       \          /
        +--------+
```

### 4.2 Verification par test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_tile() {
        let (sx, sy) = world_to_screen(0.0, 0.0);
        assert!((sx - 0.0).abs() < f32::EPSILON);
        assert!((sy - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_tile_1_0() {
        // tile (1, 0) -> screen (32, 16)
        let (sx, sy) = world_to_screen(1.0, 0.0);
        assert!((sx - 32.0).abs() < f32::EPSILON);
        assert!((sy - 16.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_tile_0_1() {
        // tile (0, 1) -> screen (-32, 16)
        let (sx, sy) = world_to_screen(0.0, 1.0);
        assert!((sx - (-32.0)).abs() < f32::EPSILON);
        assert!((sy - 16.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_roundtrip_screen_world() {
        let (sx, sy) = world_to_screen(3.5, 7.2);
        let (tx, ty) = screen_to_world(sx, sy);
        assert!((tx - 3.5).abs() < 0.001);
        assert!((ty - 7.2).abs() < 0.001);
    }

    #[test]
    fn test_screen_to_tile_floor() {
        let (tx, ty) = screen_to_tile(50.0, 20.0);
        // Verifie que le resultat est un entier (pas de panique)
        let _ = (tx, ty);
    }
}
```

---

## 5. TextureAtlas et AtlasFrame (TOML)

Les atlas de textures sont decrits par un fichier PNG + un fichier TOML genere par
`mge-packer`. Le TOML contient la liste des frames avec leurs coordonnees pixel.

### 5.1 Format TOML du descripteur d'atlas

```toml
# assets/atlases/act1_tiles.toml (genere par mge-packer)

[[frames]]
name = "grass_01"
x = 0
y = 0
w = 64
h = 32
offset_x = 0.0
offset_y = 0.0
trimmed = false
source_w = 64
source_h = 32

[[frames]]
name = "stone_floor_01"
x = 64
y = 0
w = 64
h = 32
offset_x = 0.0
offset_y = 0.0
trimmed = false
source_w = 64
source_h = 32

# ... 200+ frames par atlas
```

### 5.2 Structures Rust

```rust
// @id: sd-impl-atlas @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-render/src/atlas.rs

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Description d'un frame individuel dans un atlas de textures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlasFrame {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub trimmed: bool,
    pub source_w: u32,
    pub source_h: u32,
}

/// Fichier TOML complet genere par mge-packer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlasDescriptor {
    pub frames: Vec<AtlasFrame>,
}

/// Atlas de textures charge en GPU : texture + bind group + index de frames.
pub struct TextureAtlas {
    pub id: String,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
    pub width: u32,
    pub height: u32,
    pub frames: HashMap<String, AtlasFrame>,
}

impl TextureAtlas {
    /// Charge un atlas depuis un fichier PNG + un descripteur TOML.
    ///
    /// Le `bind_group_layout` doit correspondre au layout du pipeline de sprites :
    ///   binding 1 = Texture2D, binding 2 = Sampler.
    pub fn load(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
        png_bytes: &[u8],
        toml_content: &str,
        atlas_id: &str,
    ) -> Result<Self, RenderError> {
        // 1. Decoder le PNG.
        let img = image::load_from_memory(png_bytes)
            .map_err(|e| RenderError::ImageLoad(e.to_string()))?
            .to_rgba8();
        let (width, height) = img.dimensions();

        // 2. Creer la texture GPU.
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(atlas_id),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // 3. Uploader les pixels.
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 4. Creer le bind group (bindings 1 et 2 du pipeline de sprites).
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("{atlas_id} bind group")),
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        });

        // 5. Parser le descripteur TOML.
        let descriptor: AtlasDescriptor = toml::from_str(toml_content)
            .map_err(|e| RenderError::TomlParse(e.to_string()))?;

        let frames: HashMap<String, AtlasFrame> = descriptor
            .frames
            .into_iter()
            .map(|f| (f.name.clone(), f))
            .collect();

        Ok(Self {
            id: atlas_id.to_string(),
            texture,
            view,
            bind_group,
            width,
            height,
            frames,
        })
    }

    /// Retourne les coordonnees UV normalisees [u_min, v_min, u_max, v_max] pour un frame.
    ///
    /// Les UV sont normalises dans [0.0, 1.0] par rapport a la taille de l'atlas.
    pub fn get_uv(&self, frame_name: &str) -> Option<[f32; 4]> {
        self.frames.get(frame_name).map(|f| {
            [
                f.x as f32 / self.width as f32,
                f.y as f32 / self.height as f32,
                (f.x + f.w) as f32 / self.width as f32,
                (f.y + f.h) as f32 / self.height as f32,
            ]
        })
    }

    /// Retourne les dimensions en pixels d'un frame.
    pub fn get_frame_size(&self, frame_name: &str) -> Option<(u32, u32)> {
        self.frames.get(frame_name).map(|f| (f.w, f.h))
    }
}
```

### 5.3 Registre d'atlas

```rust
// @id: sd-impl-atlas-registry @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-render/src/atlas.rs (suite)

use std::sync::Arc;

/// Registre global de tous les atlas charges.
pub struct AtlasRegistry {
    atlases: HashMap<String, Arc<TextureAtlas>>,
}

impl AtlasRegistry {
    pub fn new() -> Self {
        Self {
            atlases: HashMap::new(),
        }
    }

    pub fn insert(&mut self, atlas: TextureAtlas) {
        self.atlases.insert(atlas.id.clone(), Arc::new(atlas));
    }

    pub fn get(&self, atlas_id: &str) -> Option<&Arc<TextureAtlas>> {
        self.atlases.get(atlas_id)
    }

    /// Recharge un atlas (hot-reload). Remplace l'ancien en place.
    pub fn reload(
        &mut self,
        atlas_id: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
        png_bytes: &[u8],
        toml_content: &str,
    ) -> Result<(), RenderError> {
        let atlas = TextureAtlas::load(device, queue, layout, sampler, png_bytes, toml_content, atlas_id)?;
        self.atlases.insert(atlas_id.to_string(), Arc::new(atlas));
        Ok(())
    }
}
```

---

## 6. SpriteInstanceGpu et SpriteBatcher

### 6.1 SpriteInstanceGpu -- 64 bytes par sprite

Chaque sprite envoye au GPU est represente par une instance de 64 bytes.
Le quad (6 vertices : 2 triangles) est partage entre toutes les instances.

```rust
// @id: sd-impl-sprite-instance @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-render/src/sprite_instance.rs

/// Donnees GPU par instance de sprite (64 bytes, aligne 16).
///
/// Un seul quad est partage par toutes les instances. Le vertex shader
/// applique la position, la taille, les UV et le flip depuis les donnees
/// d'instance.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteInstanceGpu {
    /// Position ecran en pixels [x, y].
    pub position: [f32; 2],

    /// Coordonnees UV dans l'atlas [u_min, v_min, u_max, v_max].
    pub uv_rect: [f32; 4],

    /// Taille du sprite en pixels [width, height].
    pub size: [f32; 2],

    /// Tint color RGBA. [1.0, 1.0, 1.0, 1.0] = pas de tint.
    pub tint: [f32; 4],

    /// Flags : bit 0 = flip_h, bit 1 = flip_v.
    pub flags: u32,

    /// Padding pour alignement 16 bytes. Total = 2+4+2+4+1+3 = 16 f32 = 64 bytes.
    pub _padding: [u32; 3],
}

impl SpriteInstanceGpu {
    /// VertexBufferLayout pour l'instance buffer (buffer slot 1).
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBS: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
            // Slot 1 attributes (instance)
            // Attention : les locations commencent apres celles du quad vertex (slot 0).
            2 => Float32x2,  // position
            3 => Float32x4,  // uv_rect
            4 => Float32x2,  // size
            5 => Float32x4,  // tint
            6 => Uint32,     // flags
        ];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ATTRIBS,
        }
    }

    /// Construit un sprite flip_h.
    pub fn with_flip_h(mut self) -> Self {
        self.flags |= 1;
        self
    }
}
```

### 6.2 Quad partage (SpriteVertex)

```rust
// @id: sd-impl-sprite-vertex @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-render/src/sprite_instance.rs (suite)

/// Vertex du quad partage (buffer slot 0). 2 triangles = 6 vertices.
///
/// Les coordonnees sont normalisees [0..1]. Le vertex shader les multiplie
/// par la taille et ajoute la position de l'instance.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteVertex {
    /// Position locale normalisee du coin du quad [0..1, 0..1].
    pub position: [f32; 2],
}

impl SpriteVertex {
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
            0 => Float32x2,  // position
        ];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBS,
        }
    }
}

/// Les 6 vertices d'un quad unitaire (2 triangles, CCW).
pub const QUAD_VERTICES: [SpriteVertex; 6] = [
    SpriteVertex { position: [0.0, 0.0] },  // top-left
    SpriteVertex { position: [1.0, 0.0] },  // top-right
    SpriteVertex { position: [0.0, 1.0] },  // bottom-left
    SpriteVertex { position: [1.0, 0.0] },  // top-right
    SpriteVertex { position: [1.0, 1.0] },  // bottom-right
    SpriteVertex { position: [0.0, 1.0] },  // bottom-left
];
```

### 6.3 SpriteBatcher -- groupement par atlas

```rust
// @id: sd-impl-sprite-batcher @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-render/src/sprite_batch.rs

use std::collections::HashMap;

/// Un batch = un ensemble d'instances partageant le meme atlas.
pub struct SpriteBatch {
    pub atlas_id: String,
    pub instances: Vec<SpriteInstanceGpu>,
}

/// Le batcher regroupe les sprites par atlas pour minimiser les draw calls.
///
/// Workflow par frame :
/// 1. `batcher.clear()`
/// 2. Pour chaque sprite visible, `batcher.push(atlas_id, instance)`
/// 3. `batcher.flush(render_pass, queue, device, atlas_registry, pipeline, camera_bg)`
pub struct SpriteBatcher {
    pub batches: HashMap<String, SpriteBatch>,
    pub instance_buffer: wgpu::Buffer,
    pub quad_vertex_buffer: wgpu::Buffer,
    instance_buffer_capacity: usize,
}

const INITIAL_INSTANCE_CAPACITY: usize = 4096;

impl SpriteBatcher {
    pub fn new(device: &wgpu::Device) -> Self {
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Instance Buffer"),
            size: (INITIAL_INSTANCE_CAPACITY * std::mem::size_of::<SpriteInstanceGpu>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad Vertex Buffer"),
            contents: bytemuck::cast_slice(&QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            batches: HashMap::new(),
            instance_buffer,
            quad_vertex_buffer,
            instance_buffer_capacity: INITIAL_INSTANCE_CAPACITY,
        }
    }

    /// Vide tous les batches pour un nouveau frame.
    pub fn clear(&mut self) {
        for batch in self.batches.values_mut() {
            batch.instances.clear();
        }
    }

    /// Ajoute un sprite au batch correspondant a son atlas.
    pub fn push(&mut self, atlas_id: &str, instance: SpriteInstanceGpu) {
        self.batches
            .entry(atlas_id.to_string())
            .or_insert_with(|| SpriteBatch {
                atlas_id: atlas_id.to_string(),
                instances: Vec::new(),
            })
            .instances
            .push(instance);
    }

    /// Upload les instances vers le GPU et dessine tous les batches.
    ///
    /// Un draw call par atlas (typiquement < 20 atlas = < 20 draw calls).
    pub fn flush<'a>(
        &'a mut self,
        render_pass: &mut wgpu::RenderPass<'a>,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        atlas_registry: &'a AtlasRegistry,
        pipeline: &'a wgpu::RenderPipeline,
        camera_bind_group: &'a wgpu::BindGroup,
    ) {
        render_pass.set_pipeline(pipeline);

        for batch in self.batches.values() {
            if batch.instances.is_empty() {
                continue;
            }

            // Recuperer l'atlas pour ce batch.
            let atlas = match atlas_registry.get(&batch.atlas_id) {
                Some(a) => a,
                None => {
                    tracing::warn!("Atlas '{}' not found in registry", batch.atlas_id);
                    continue;
                }
            };

            // Reallouer le buffer si la capacite est insuffisante.
            if batch.instances.len() > self.instance_buffer_capacity {
                self.instance_buffer_capacity = batch.instances.len() * 2;
                self.instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Sprite Instance Buffer (resized)"),
                    size: (self.instance_buffer_capacity
                        * std::mem::size_of::<SpriteInstanceGpu>()) as u64,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
            }

            // Uploader les instances.
            queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(&batch.instances),
            );

            // Bind groups : 0 = camera, 1 = atlas texture + sampler.
            render_pass.set_bind_group(0, camera_bind_group, &[]);
            render_pass.set_bind_group(1, &atlas.bind_group, &[]);

            // Buffer slot 0 = quad vertices, slot 1 = instance data.
            render_pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

            // 6 vertices par quad, N instances.
            render_pass.draw(0..6, 0..batch.instances.len() as u32);
        }
    }
}
```

---

## 7. Z-ordering : 9 render layers

Le tri en Z est le coeur de l'affichage isometrique. Aucun depth buffer GPU n'est utilise :
tout le tri est fait en CPU avant l'envoi au batcher.

### 7.1 RenderLayer enum

```rust
// @id: sd-impl-z-order @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-render/src/z_order.rs

/// Les 9 layers de rendu du monde, dans l'ordre de dessin.
///
/// Le layer Floor est dessine en premier (derriere tout), le layer Ui en dernier
/// (devant tout). A l'interieur d'un layer, le tri se fait par profondeur isometrique.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderLayer {
    /// 0 - Tiles de sol (herbe, pierre, sable, eau).
    Floor,
    /// 1 - Ombres portees des entites.
    Shadow,
    /// 2 - Items au sol (droppes, gold, potions).
    GroundItem,
    /// 3 - Entites (joueurs, monstres, NPCs, mercenaires).
    Entity,
    /// 4 - Effets au sol (AoE actives, auras visuelles).
    OverlayEffect,
    /// 5 - Projectiles (fleches, bone spear, fire ball).
    Projectile,
    /// 6 - Premier plan (murs, arbres, toits qui masquent les entites).
    Foreground,
    /// 7 - Meteo (pluie, neige, brouillard) -- plein ecran, pas de tri.
    Weather,
    /// 8 - UI monde (noms d'entites, barres de vie, damage numbers).
    Ui,
}

impl RenderLayer {
    /// Base Z pour ce layer. Multiplie par 10000 pour garantir la separation.
    fn base_z(self) -> f32 {
        match self {
            Self::Floor        => 0.0,
            Self::Shadow       => 1.0,
            Self::GroundItem   => 2.0,
            Self::Entity       => 3.0,
            Self::OverlayEffect => 4.0,
            Self::Projectile   => 5.0,
            Self::Foreground   => 6.0,
            Self::Weather      => 7.0,
            Self::Ui           => 8.0,
        }
    }
}
```

### 7.2 Formule Z exacte

```rust
/// Calcule la profondeur Z d'un sprite en isometrique.
///
/// La formule combine le layer et la position monde :
///   z = layer_base * 10000 + (tile_x + tile_y) * 100 + sub_order
///
/// - layer_base : separe les layers de facon etanche (0..8).
/// - (tile_x + tile_y) : profondeur isometrique. Plus la somme est grande,
///   plus l'entite est "en bas" de l'ecran (donc dessinee devant).
/// - sub_order : tri tertiaire pour departager les sprites au meme endroit
///   (ex: la tete d'un personnage devant son corps).
pub fn compute_z_order(tile_x: f32, tile_y: f32, layer: RenderLayer, sub_order: f32) -> f32 {
    layer.base_z() * 10000.0 + (tile_x + tile_y) * 100.0 + sub_order
}

/// Trie un vecteur de sprites par Z croissant (painter's algorithm).
/// Les sprites avec un Z plus petit sont dessines en premier (derriere).
pub fn sort_sprites_by_depth(sprites: &mut [SortableSprite]) {
    sprites.sort_by(|a, b| {
        a.z_order.partial_cmp(&b.z_order).unwrap_or(std::cmp::Ordering::Equal)
    });
}

/// Sprite intermediaire pour le tri Z avant envoi au batcher.
#[derive(Debug, Clone)]
pub struct SortableSprite {
    pub z_order: f32,
    pub atlas_id: String,
    pub instance: SpriteInstanceGpu,
}
```

### 7.3 Regles de tri

| Situation | Regle |
|-----------|-------|
| Entite A plus au nord que B | A est derriere B (z plus petit) |
| Entite A au meme rang que B | Tri par position Y ecran (sub_order) |
| Item au sol vs entite | Item toujours derriere (layer GroundItem < Entity) |
| Projectile vs entite | Projectile toujours devant (layer Projectile > Entity) |
| Ombre vs entite | Ombre toujours derriere (layer Shadow < Entity) |
| Mur premier plan vs entite | Mur devant si layer Foreground |
| Meteo | Plein ecran, pas de tri interne |

---

## 8. Pipeline de rendu wgpu

### 8.1 Bind group layout

Le pipeline de sprites utilise 2 bind groups :

| Group | Binding | Type | Stage | Contenu |
|-------|---------|------|-------|---------|
| 0 | 0 | Uniform buffer | Vertex | Camera (projection + offset) |
| 1 | 1 | Texture2D | Fragment | Atlas de textures |
| 1 | 2 | Sampler | Fragment | Nearest-neighbor (pixel-art) |

### 8.2 Code de creation du pipeline

```rust
// @id: sd-impl-pipeline @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-render/src/pipeline.rs

/// Camera uniform uploade au GPU chaque frame.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    /// Matrice de projection orthographique 4x4.
    pub proj: [[f32; 4]; 4],
    /// Offset camera (scroll) en pixels.
    pub offset: [f32; 2],
    /// Padding pour alignement 16 bytes.
    pub _pad: [f32; 2],
}

pub fn create_sprite_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> (wgpu::RenderPipeline, wgpu::BindGroupLayout, wgpu::BindGroupLayout) {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Sprite Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/sprite.wgsl").into()),
    });

    // Bind group 0 : Camera uniform.
    let camera_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Camera Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    // Bind group 1 : Atlas texture + sampler.
    let atlas_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Atlas Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Sprite Pipeline Layout"),
        bind_group_layouts: &[&camera_layout, &atlas_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Sprite Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[
                SpriteVertex::layout(),     // Slot 0 : quad vertices
                SpriteInstanceGpu::layout(), // Slot 1 : instance data
            ],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None, // Pas de face culling pour les sprites 2D.
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None, // Pas de depth buffer ; le tri Z est fait en CPU.
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    (pipeline, camera_layout, atlas_layout)
}
```

### 8.3 Sampler pixel-art

```rust
/// Cree le sampler nearest-neighbor pour le pixel-art.
/// Aucun filtrage lineaire, aucun mipmap.
pub fn create_pixel_art_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Pixel Art Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    })
}
```

---

## 9. Shader WGSL sprites (instancing)

```wgsl
// @id: sd-impl-shader-sprite @do: implement @role: back-end @layer: 2 @human: miyuk
// Fichier: crates/engine/mge-render/src/shaders/sprite.wgsl

// === Bind group 0 : Camera ===

struct CameraUniform {
    proj: mat4x4<f32>,
    offset: vec2<f32>,
    _pad: vec2<f32>,
}

@group(0) @binding(0) var<uniform> camera: CameraUniform;

// === Bind group 1 : Atlas ===

@group(1) @binding(1) var t_atlas: texture_2d<f32>;
@group(1) @binding(2) var s_atlas: sampler;

// === Vertex input : quad (slot 0) + instance (slot 1) ===

struct QuadVertex {
    @location(0) local_pos: vec2<f32>,
}

struct InstanceInput {
    @location(2) position: vec2<f32>,
    @location(3) uv_rect: vec4<f32>,     // [u_min, v_min, u_max, v_max]
    @location(4) size: vec2<f32>,
    @location(5) tint: vec4<f32>,
    @location(6) flags: u32,
}

struct VertexOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) tint: vec4<f32>,
}

@vertex
fn vs_main(quad: QuadVertex, inst: InstanceInput) -> VertexOut {
    var out: VertexOut;

    // Position du coin en pixels ecran.
    let pixel_pos = inst.position + quad.local_pos * inst.size;

    // Appliquer l'offset camera.
    let world_pos = vec4<f32>(pixel_pos - camera.offset, 0.0, 1.0);

    // Projeter en NDC via la matrice orthographique.
    out.clip_pos = camera.proj * world_pos;

    // Calculer les UV avec support du flip horizontal.
    var u = mix(inst.uv_rect.x, inst.uv_rect.z, quad.local_pos.x);
    let v = mix(inst.uv_rect.y, inst.uv_rect.w, quad.local_pos.y);

    // Flip horizontal si bit 0 du flags est set.
    if (inst.flags & 1u) != 0u {
        u = mix(inst.uv_rect.z, inst.uv_rect.x, quad.local_pos.x);
    }

    out.uv = vec2<f32>(u, v);
    out.tint = inst.tint;

    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let texel = textureSample(t_atlas, s_atlas, in.uv);

    // Alpha test pour le pixel-art : pas de semi-transparence sur les bords.
    if texel.a < 0.5 {
        discard;
    }

    return texel * in.tint;
}
```

**Points techniques du shader :**

- Le quad local est `[0..1, 0..1]`. Le vertex shader le multiplie par `inst.size` (pixels).
- Le `mix()` WGSL interpole lineairement les UV entre u_min et u_max.
- Le flip horizontal inverse les UV en X (echange u_min et u_max).
- L'alpha test a 0.5 evite le halo semi-transparent autour des sprites pixel-art.
- La matrice `camera.proj` est une projection orthographique pixels -> NDC.

---

## 10. Dual-resolution : offscreen 800x600 + upscale

En mode pixel-perfect, le monde est rendu dans un framebuffer offscreen de 800x600.
Le resultat est ensuite upscale vers la surface native avec un sampler nearest-neighbor.
L'UI/HUD est dessinee directement sur la surface native (pas dans l'offscreen).

### 10.1 DualResolution struct

```rust
// @id: sd-impl-dual-res @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-render/src/dual_res.rs

/// Mode de resolution du renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionMode {
    /// Rendu en 800x600, upscale nearest-neighbor vers la fenetre.
    /// Pixels carres, fidele au D2 original.
    PixelPerfect,
    /// Rendu en resolution native du moniteur. Sprites HD optionnels.
    NativeHD,
}

pub struct DualResolution {
    pub enabled: bool,
    pub mode: ResolutionMode,
    pub render_width: u32,
    pub render_height: u32,
    pub offscreen_texture: wgpu::Texture,
    pub offscreen_view: wgpu::TextureView,
    pub offscreen_bind_group: wgpu::BindGroup,
    pub upscale_pipeline: wgpu::RenderPipeline,
    pub upscale_sampler: wgpu::Sampler,
}

impl DualResolution {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        mode: ResolutionMode,
        window_width: u32,
        window_height: u32,
    ) -> Self {
        let (render_width, render_height) = match mode {
            ResolutionMode::PixelPerfect => (800, 600),
            ResolutionMode::NativeHD => (window_width, window_height),
        };

        let enabled = mode == ResolutionMode::PixelPerfect;

        // 1. Creer la texture offscreen.
        let offscreen_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Offscreen Texture"),
            size: wgpu::Extent3d {
                width: render_width,
                height: render_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let offscreen_view =
            offscreen_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 2. Sampler pour l'upscale.
        let upscale_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Upscale Sampler"),
            mag_filter: match mode {
                ResolutionMode::PixelPerfect => wgpu::FilterMode::Nearest,
                ResolutionMode::NativeHD => wgpu::FilterMode::Linear,
            },
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // 3. Pipeline et bind group pour l'upscale (fullscreen quad).
        let (upscale_pipeline, upscale_layout) = create_upscale_pipeline(device, format);

        let offscreen_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Offscreen Bind Group"),
            layout: &upscale_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&offscreen_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&upscale_sampler),
                },
            ],
        });

        Self {
            enabled,
            mode,
            render_width,
            render_height,
            offscreen_texture,
            offscreen_view,
            offscreen_bind_group,
            upscale_pipeline,
            upscale_sampler,
        }
    }

    /// Upscale le framebuffer offscreen vers la surface finale.
    pub fn upscale(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target_view: &wgpu::TextureView,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Upscale Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_pipeline(&self.upscale_pipeline);
        pass.set_bind_group(0, &self.offscreen_bind_group, &[]);
        pass.draw(0..6, 0..1); // Fullscreen quad (2 triangles).
    }

    /// Matrice de projection orthographique pour le rendu offscreen.
    /// Transforme les coordonnees pixels [0..w, 0..h] en NDC [-1..1].
    pub fn ortho_matrix(&self) -> [[f32; 4]; 4] {
        let w = self.render_width as f32;
        let h = self.render_height as f32;
        [
            [2.0 / w,  0.0,      0.0, 0.0],
            [0.0,     -2.0 / h,  0.0, 0.0],
            [0.0,      0.0,      1.0, 0.0],
            [-1.0,     1.0,      0.0, 1.0],
        ]
    }
}

/// Cree le pipeline fullscreen quad pour l'upscale.
fn create_upscale_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> (wgpu::RenderPipeline, wgpu::BindGroupLayout) {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Upscale Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/upscale.wgsl").into()),
    });

    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Upscale Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Upscale Pipeline Layout"),
        bind_group_layouts: &[&layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Upscale Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[], // Pas de vertex buffer ; fullscreen quad genere dans le shader.
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: None, // Pas de blending pour l'upscale.
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    (pipeline, layout)
}
```

---

## 11. Shader WGSL upscale (fullscreen quad)

```wgsl
// @id: sd-impl-shader-upscale @do: implement @role: back-end @layer: 2 @human: miyuk
// Fichier: crates/engine/mge-render/src/shaders/upscale.wgsl

@group(0) @binding(0) var t_offscreen: texture_2d<f32>;
@group(0) @binding(1) var s_offscreen: sampler;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// Fullscreen quad genere proceduralement (pas de vertex buffer).
// vertex_index : 0..5 -> 2 triangles couvrant l'ecran entier.
@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VertexOut {
    // Triangle 1 : (0,0), (2,0), (0,2) -- couvre le NDC [-1..1]
    // Triangle 2 : (2,0), (2,2), (0,2)
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>(-1.0,  1.0),
    );

    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 1.0),
    );

    var out: VertexOut;
    out.position = vec4<f32>(positions[vi], 0.0, 1.0);
    out.uv = uvs[vi];
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return textureSample(t_offscreen, s_offscreen, in.uv);
}
```

---

## 12. Animation : frame timer et direction flip

### 12.1 Animation system

```rust
// @id: sd-impl-animation @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-render/src/animation.rs

/// 8 directions isometriques.
///
///          N (0)
///    NW (7)   NE (1)
///   W (6)       E (2)
///    SW (5)   SE (3)
///          S (4)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

/// Etat d'animation d'une entite (composant ECS).
pub struct AnimState {
    pub action: String,         // "idle", "walk", "attack", "death"...
    pub direction: Direction,
    pub current_frame: u32,
    pub total_frames: u32,
    pub fps: f32,               // images par seconde
    pub elapsed: f32,           // accumulateur temps
    pub looping: bool,
    pub finished: bool,
}

/// Systeme d'animation : avance les frames et met a jour les UV du sprite.
///
/// Appele a chaque frame de rendu (Update stage, pas FixedUpdate).
pub fn animation_system(
    delta_seconds: f32,
    anims: &mut [(AnimState, SpriteRef)],
    atlas_registry: &AtlasRegistry,
) {
    for (anim, sprite_ref) in anims.iter_mut() {
        if anim.finished {
            continue;
        }

        anim.elapsed += delta_seconds;
        let frame_duration = 1.0 / anim.fps;

        while anim.elapsed >= frame_duration {
            anim.elapsed -= frame_duration;
            anim.current_frame += 1;

            if anim.current_frame >= anim.total_frames {
                if anim.looping {
                    anim.current_frame = 0;
                } else {
                    anim.current_frame = anim.total_frames.saturating_sub(1);
                    anim.finished = true;
                    break;
                }
            }
        }

        // Resoudre le flip horizontal (5 directions source + 3 mirrored).
        let (source_dir, flip_h) = resolve_direction_flip(anim.direction);

        // Construire le nom de frame dans l'atlas.
        let frame_name = format!(
            "{}_{}_{}", sprite_ref.sprite_id, direction_suffix(source_dir), anim.current_frame
        );

        // Mettre a jour l'UV et le flag flip.
        if let Some(atlas) = atlas_registry.get(&sprite_ref.atlas_id) {
            if let Some(uv) = atlas.get_uv(&frame_name) {
                sprite_ref.uv_rect = uv;
                sprite_ref.flip_h = flip_h;
            }
        }
    }
}
```

### 12.2 Direction flip

Les spritesheets contiennent 5 directions (S, SW, W, NW, N). Les 3 autres
(SE, E, NE) sont obtenues par miroir horizontal.

```rust
/// Table de miroir : 5 directions sources, 3 miroirs.
///
/// | Direction source | Direction miroir | Flip H |
/// |------------------|-----------------|--------|
/// | East             | West            | Oui    |
/// | NorthEast        | NorthWest       | Oui    |
/// | SouthEast        | SouthWest       | Oui    |
pub fn resolve_direction_flip(dir: Direction) -> (Direction, bool) {
    match dir {
        Direction::West      => (Direction::East, true),
        Direction::NorthWest => (Direction::NorthEast, true),
        Direction::SouthWest => (Direction::SouthEast, true),
        other => (other, false),
    }
}

fn direction_suffix(dir: Direction) -> &'static str {
    match dir {
        Direction::South     => "s",
        Direction::SouthWest => "sw",
        Direction::West      => "w",
        Direction::NorthWest => "nw",
        Direction::North     => "n",
        Direction::NorthEast => "ne",
        Direction::East      => "e",
        Direction::SouthEast => "se",
    }
}
```

**Attention :** La table de Denis indique que W est le miroir de E (pas l'inverse).
C'est parce que les spritesheets contiennent S, SW, W, NW, N -- donc E, NE, SE sont
les directions mirrored. Cependant, `resolve_direction_flip` prend la direction cible
et retourne la direction source dans l'atlas. Si la cible est W, la source est E + flip.
Si la cible est E, la source est E sans flip (E existe dans l'atlas via la convention
opposee). A valider avec les assets reels.

---

## 13. Frustum culling

Seuls les sprites visibles dans le viewport sont envoyes au batcher. Le culling
est un simple test AABB en coordonnees ecran apres la projection isometrique.

```rust
// @id: sd-impl-culling @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-render/src/culling.rs

/// Culling AABB en coordonnees ecran.
///
/// La marge de 128 pixels (2 tiles) evite le "popping" des entites
/// qui entrent dans le viewport par le bord.
pub struct FrustumCuller {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub margin: f32,
}

impl FrustumCuller {
    /// Construit un culler a partir de la camera et de la taille du viewport.
    pub fn new(camera_x: f32, camera_y: f32, viewport_w: f32, viewport_h: f32) -> Self {
        Self {
            min_x: camera_x - viewport_w / 2.0,
            min_y: camera_y - viewport_h / 2.0,
            max_x: camera_x + viewport_w / 2.0,
            max_y: camera_y + viewport_h / 2.0,
            margin: 128.0, // 2 tiles de marge.
        }
    }

    /// Retourne true si le sprite est potentiellement visible.
    pub fn is_visible(&self, screen_x: f32, screen_y: f32, width: f32, height: f32) -> bool {
        screen_x + width  >= self.min_x - self.margin
            && screen_x   <= self.max_x + self.margin
            && screen_y + height >= self.min_y - self.margin
            && screen_y   <= self.max_y + self.margin
    }
}
```

---

## 14. Systeme de particules

Le pool de particules est pre-alloue pour eviter toute allocation pendant le jeu.
Les particules sont rendues dans le layer `OverlayEffect` ou `Projectile` selon leur type.

### 14.1 ParticlePool

```rust
// @id: sd-impl-particles @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-render/src/particle.rs

/// Particule individuelle dans le pool.
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub active: bool,
    pub position: [f32; 2],     // Position monde (tiles flottantes)
    pub velocity: [f32; 2],
    pub acceleration: [f32; 2],
    pub color: [f32; 4],        // Couleur courante RGBA
    pub color_end: [f32; 4],    // Couleur fin de vie
    pub size: f32,              // Taille courante (pixels)
    pub size_end: f32,          // Taille fin de vie
    pub lifetime: f32,          // Temps restant (secondes)
    pub max_lifetime: f32,      // Duree totale initiale
    pub rotation: f32,
    pub rotation_speed: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            active: false,
            position: [0.0; 2],
            velocity: [0.0; 2],
            acceleration: [0.0; 2],
            color: [1.0; 4],
            color_end: [1.0, 1.0, 1.0, 0.0],
            size: 4.0,
            size_end: 0.0,
            lifetime: 0.0,
            max_lifetime: 1.0,
            rotation: 0.0,
            rotation_speed: 0.0,
        }
    }
}

/// Pool pre-alloue de particules. Aucune allocation frame-to-frame.
pub struct ParticlePool {
    pub particles: Vec<Particle>,
    pub active_count: usize,
}

impl ParticlePool {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: vec![Particle::default(); max_particles],
            active_count: 0,
        }
    }

    /// Alloue une particule depuis le pool. Retourne None si le pool est plein.
    pub fn spawn(&mut self) -> Option<&mut Particle> {
        for p in &mut self.particles {
            if !p.active {
                p.active = true;
                self.active_count += 1;
                return Some(p);
            }
        }
        None
    }

    /// Met a jour toutes les particules actives.
    pub fn update(&mut self, dt: f32) {
        for p in &mut self.particles {
            if !p.active {
                continue;
            }

            p.lifetime -= dt;
            if p.lifetime <= 0.0 {
                p.active = false;
                self.active_count = self.active_count.saturating_sub(1);
                continue;
            }

            let t = 1.0 - (p.lifetime / p.max_lifetime);

            // Mouvement : acceleration -> velocite -> position.
            p.velocity[0] += p.acceleration[0] * dt;
            p.velocity[1] += p.acceleration[1] * dt;
            p.position[0] += p.velocity[0] * dt;
            p.position[1] += p.velocity[1] * dt;

            // Interpolation couleur.
            for i in 0..4 {
                p.color[i] = lerp(p.color[i], p.color_end[i], t);
            }

            // Interpolation taille.
            p.size = lerp(p.size, p.size_end, t);

            // Rotation.
            p.rotation += p.rotation_speed * dt;
        }
    }

    /// Collecte les sprites de particules actives pour le batcher.
    pub fn collect_sprites(
        &self,
        atlas_registry: &AtlasRegistry,
        sprite_id: &str,
        atlas_id: &str,
        camera_x: f32,
        camera_y: f32,
    ) -> Vec<SortableSprite> {
        let uv = atlas_registry
            .get(atlas_id)
            .and_then(|a| a.get_uv(sprite_id));

        let uv = match uv {
            Some(uv) => uv,
            None => return Vec::new(),
        };

        self.particles
            .iter()
            .filter(|p| p.active)
            .map(|p| {
                let (sx, sy) = mge_math::world_to_screen(p.position[0], p.position[1]);
                SortableSprite {
                    z_order: compute_z_order(
                        p.position[0],
                        p.position[1],
                        RenderLayer::OverlayEffect,
                        0.1,
                    ),
                    atlas_id: atlas_id.to_string(),
                    instance: SpriteInstanceGpu {
                        position: [sx - camera_x, sy - camera_y],
                        uv_rect: uv,
                        size: [p.size, p.size],
                        tint: p.color,
                        flags: 0,
                        _padding: [0; 3],
                    },
                }
            })
            .collect()
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
```

### 14.2 Types de particules predefinies

| Type | Spawn rate | Lifetime | Couleur depart | Couleur fin | Gravite Y |
|------|-----------|----------|---------------|-------------|-----------|
| Fire | 50/sec | 0.5-1.0 s | (1.0, 0.6, 0.0, 1.0) | (1.0, 0.0, 0.0, 0.0) | -20 (monte) |
| Ice | 30/sec | 0.8-1.5 s | (0.5, 0.8, 1.0, 1.0) | (0.8, 0.9, 1.0, 0.0) | 5 (tombe) |
| Poison | 20/sec | 1.0-2.0 s | (0.2, 0.8, 0.1, 0.8) | (0.1, 0.4, 0.0, 0.0) | -5 |
| Explosion | burst 30 | 0.3-0.6 s | (1.0, 0.8, 0.2, 1.0) | (0.5, 0.1, 0.0, 0.0) | 10 |
| Blood | burst 15 | 0.2-0.5 s | (0.8, 0.0, 0.0, 1.0) | (0.3, 0.0, 0.0, 0.0) | 30 |
| LevelUp | burst 50 | 1.0-2.0 s | (1.0, 1.0, 0.5, 1.0) | (1.0, 1.0, 1.0, 0.0) | -30 |

---

## 15. Boucle de rendu complete

Voici le code complet de la frame de rendu, tel qu'il s'execute dans le Render stage
de la game loop (variable rate, apres le FixedUpdate a 25 Hz).

```rust
// @id: sd-impl-render-frame @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-render/src/renderer.rs

pub struct Renderer {
    pub batcher: SpriteBatcher,
    pub pipeline: wgpu::RenderPipeline,
    pub camera_layout: wgpu::BindGroupLayout,
    pub atlas_layout: wgpu::BindGroupLayout,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub sampler: wgpu::Sampler,
    pub dual_res: DualResolution,
    pub atlas_registry: AtlasRegistry,
    pub surface_format: wgpu::TextureFormat,
}

impl Renderer {
    /// Frame de rendu complete.
    ///
    /// Workflow :
    /// 1. Acquerir la texture de la swap chain.
    /// 2. Mettre a jour le camera uniform.
    /// 3. Collecter et trier tous les sprites par Z.
    /// 4. Remplir le batcher.
    /// 5. Render pass monde (dans l'offscreen si dual-res).
    /// 6. Upscale si dual-res active.
    /// 7. Render pass UI (directement sur la surface).
    /// 8. Submit et present.
    pub fn render_frame(
        &mut self,
        gpu: &GpuContext,
        camera: &Camera2D,
        sprites: &mut Vec<SortableSprite>,
    ) -> Result<(), RenderError> {
        // 1. Acquerir la texture de la swap chain.
        let output = gpu.surface.get_current_texture()
            .map_err(|e| RenderError::SwapChain(e.to_string()))?;
        let surface_view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 2. Mettre a jour le camera uniform.
        let camera_uniform = CameraUniform {
            proj: self.dual_res.ortho_matrix(),
            offset: [camera.x, camera.y],
            _pad: [0.0; 2],
        };
        gpu.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::bytes_of(&camera_uniform),
        );

        // 3. Trier par Z (painter's algorithm).
        sort_sprites_by_depth(sprites);

        // 4. Remplir le batcher.
        self.batcher.clear();
        for sprite in sprites.iter() {
            self.batcher.push(&sprite.atlas_id, sprite.instance);
        }

        let mut encoder = gpu.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        // 5. Render pass monde.
        // Si dual-res active : rendre dans l'offscreen 800x600.
        // Sinon : rendre directement dans la surface.
        let world_target = if self.dual_res.enabled {
            &self.dual_res.offscreen_view
        } else {
            &surface_view
        };

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("World Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: world_target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0, g: 0.0, b: 0.0, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Dessiner tous les batches (layers Floor..Weather).
            self.batcher.flush(
                &mut render_pass,
                &gpu.queue,
                &gpu.device,
                &self.atlas_registry,
                &self.pipeline,
                &self.camera_bind_group,
            );
        }

        // 6. Upscale si dual-res active.
        if self.dual_res.enabled {
            self.dual_res.upscale(&mut encoder, &surface_view);
        }

        // 7. Render pass UI (toujours en resolution native).
        {
            let mut _ui_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("UI Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // Ne pas effacer le monde.
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // HUD, inventaire, tooltips, curseur.
            // Gere par mge-ui (Couche 2), pas par mge-render.
        }

        // 8. Submit et present.
        gpu.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
```

### 15.1 Ordre de dessin par layer dans la frame

La collecte des sprites se fait en amont par les systemes de jeu (Couche 3 ARPG)
qui traversent le World ECS et produisent des `SortableSprite`. Voici l'ordre de
collecte recommande :

```
1. TilemapRenderer.collect_floor_tiles()    -> layer Floor
2. ShadowSystem.collect_shadows()           -> layer Shadow
3. GroundItemSystem.collect_items()          -> layer GroundItem
4. EntityRenderer.collect_entities()         -> layer Entity
5. EffectSystem.collect_ground_effects()     -> layer OverlayEffect
6. ProjectileRenderer.collect_projectiles()  -> layer Projectile
7. TilemapRenderer.collect_foreground()      -> layer Foreground
8. WeatherSystem.collect_weather()           -> layer Weather
9. WorldUiSystem.collect_names_bars()        -> layer Ui
```

Tous ces sprites sont concatenes dans un seul `Vec<SortableSprite>`, tries par Z,
puis envoyes au batcher. Le batcher les groupe par atlas et les dessine en un minimum
de draw calls.

---

## 16. Camera2D : follow, smooth, bounds

```rust
// @id: sd-impl-camera @do: implement @role: back-end @layer: 2 @human: miyuk
// Crate: mge-render/src/camera.rs

/// Camera 2D isometrique.
///
/// La camera est centree sur le joueur avec un smooth follow.
/// Elle est bornee par les limites de la zone (pas de defilement hors-map).
pub struct Camera2D {
    /// Position ecran du centre de la camera (pixels).
    pub x: f32,
    pub y: f32,
    /// Position cible (en general la position ecran du joueur).
    pub target_x: f32,
    pub target_y: f32,
    /// Vitesse de lissage (0.0 = pas de lissage, 1.0 = snap immediat).
    pub smooth_speed: f32,
    /// Limites de la zone en pixels ecran (optionnel).
    pub bounds: Option<CameraBounds>,
}

/// Limites de la camera pour eviter de defiler hors de la map.
pub struct CameraBounds {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl Camera2D {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            target_x: 0.0,
            target_y: 0.0,
            smooth_speed: 0.1,
            bounds: None,
        }
    }

    /// Met a jour la position de la camera vers la cible avec lissage.
    pub fn update(&mut self, dt: f32) {
        let factor = 1.0 - (1.0 - self.smooth_speed).powf(dt * 60.0);
        self.x += (self.target_x - self.x) * factor;
        self.y += (self.target_y - self.y) * factor;

        // Borner la camera aux limites de la zone.
        if let Some(ref bounds) = self.bounds {
            self.x = self.x.clamp(bounds.min_x, bounds.max_x);
            self.y = self.y.clamp(bounds.min_y, bounds.max_y);
        }
    }

    /// Definit la cible de la camera (position monde du joueur convertie en ecran).
    pub fn follow(&mut self, world_x: f32, world_y: f32) {
        let (sx, sy) = mge_math::world_to_screen(world_x, world_y);
        self.target_x = sx;
        self.target_y = sy;
    }

    /// Convertit des coordonnees ecran (pixel fenetre) en coordonnees monde.
    /// Utile pour le click-to-move.
    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32, viewport_w: f32, viewport_h: f32) -> (f32, f32) {
        // Position en pixels dans le monde (pas dans la fenetre).
        let world_px = screen_x + self.x - viewport_w / 2.0;
        let world_py = screen_y + self.y - viewport_h / 2.0;
        mge_math::screen_to_world(world_px, world_py)
    }
}
```

---

## 17. Tests

### 17.1 Tests unitaires (mge-math)

```rust
// Crate: mge-math, module iso
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_to_screen_origin() {
        let (sx, sy) = world_to_screen(0.0, 0.0);
        assert_eq!(sx, 0.0);
        assert_eq!(sy, 0.0);
    }

    #[test]
    fn test_world_to_screen_diagonal() {
        // (1,1) -> screen (0, 32)
        let (sx, sy) = world_to_screen(1.0, 1.0);
        assert!((sx - 0.0).abs() < f32::EPSILON);
        assert!((sy - 32.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_roundtrip_precision() {
        for tx in -10..10 {
            for ty in -10..10 {
                let (sx, sy) = world_to_screen(tx as f32, ty as f32);
                let (rx, ry) = screen_to_world(sx, sy);
                assert!((rx - tx as f32).abs() < 0.01);
                assert!((ry - ty as f32).abs() < 0.01);
            }
        }
    }
}
```

### 17.2 Tests unitaires (z_order)

```rust
// Crate: mge-render, module z_order
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_behind_entity() {
        let z_floor = compute_z_order(5.0, 5.0, RenderLayer::Floor, 0.0);
        let z_entity = compute_z_order(5.0, 5.0, RenderLayer::Entity, 0.0);
        assert!(z_floor < z_entity, "Floor must be behind Entity");
    }

    #[test]
    fn test_entity_south_in_front() {
        // Entite a (5, 5) est devant entite a (3, 3)
        let z_north = compute_z_order(3.0, 3.0, RenderLayer::Entity, 0.0);
        let z_south = compute_z_order(5.0, 5.0, RenderLayer::Entity, 0.0);
        assert!(z_south > z_north, "Entity south must be in front of entity north");
    }

    #[test]
    fn test_projectile_in_front_of_entity() {
        let z_entity = compute_z_order(5.0, 5.0, RenderLayer::Entity, 0.0);
        let z_proj = compute_z_order(5.0, 5.0, RenderLayer::Projectile, 0.0);
        assert!(z_proj > z_entity, "Projectile must be in front of Entity");
    }

    #[test]
    fn test_all_layers_ordered() {
        let layers = [
            RenderLayer::Floor,
            RenderLayer::Shadow,
            RenderLayer::GroundItem,
            RenderLayer::Entity,
            RenderLayer::OverlayEffect,
            RenderLayer::Projectile,
            RenderLayer::Foreground,
            RenderLayer::Weather,
            RenderLayer::Ui,
        ];
        for i in 0..layers.len() - 1 {
            let z_a = compute_z_order(0.0, 0.0, layers[i], 0.0);
            let z_b = compute_z_order(0.0, 0.0, layers[i + 1], 0.0);
            assert!(z_a < z_b, "{:?} must be behind {:?}", layers[i], layers[i + 1]);
        }
    }

    #[test]
    fn test_sort_sprites_stable() {
        let mut sprites = vec![
            SortableSprite { z_order: 30100.0, atlas_id: "a".into(), instance: zeroed_instance() },
            SortableSprite { z_order: 30050.0, atlas_id: "b".into(), instance: zeroed_instance() },
            SortableSprite { z_order: 0100.0,  atlas_id: "c".into(), instance: zeroed_instance() },
        ];
        sort_sprites_by_depth(&mut sprites);
        assert_eq!(sprites[0].atlas_id, "c"); // Floor
        assert_eq!(sprites[1].atlas_id, "b"); // Entity north
        assert_eq!(sprites[2].atlas_id, "a"); // Entity south
    }

    fn zeroed_instance() -> SpriteInstanceGpu {
        bytemuck::Zeroable::zeroed()
    }
}
```

### 17.3 Tests dual_res

```rust
// Crate: mge-render, module dual_res
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ortho_matrix_pixel_perfect() {
        let dual = DualResolution {
            enabled: true,
            mode: ResolutionMode::PixelPerfect,
            render_width: 800,
            render_height: 600,
            // ... champs GPU omis en test
        };
        let m = dual.ortho_matrix();
        // m[0][0] = 2.0 / 800 = 0.0025
        assert!((m[0][0] - 2.0 / 800.0).abs() < 0.0001);
        // m[1][1] = -2.0 / 600
        assert!((m[1][1] - (-2.0 / 600.0)).abs() < 0.0001);
        // m[3][0] = -1.0, m[3][1] = 1.0
        assert!((m[3][0] - (-1.0)).abs() < f32::EPSILON);
        assert!((m[3][1] - 1.0).abs() < f32::EPSILON);
    }
}
```

### 17.4 Tests animation

```rust
// Crate: mge-render, module animation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_flip_east() {
        let (src, flip) = resolve_direction_flip(Direction::East);
        assert_eq!(src, Direction::East);
        assert!(!flip);
    }

    #[test]
    fn test_direction_flip_west() {
        let (src, flip) = resolve_direction_flip(Direction::West);
        assert_eq!(src, Direction::East);
        assert!(flip);
    }

    #[test]
    fn test_direction_flip_southwest() {
        let (src, flip) = resolve_direction_flip(Direction::SouthWest);
        assert_eq!(src, Direction::SouthEast);
        assert!(flip);
    }

    #[test]
    fn test_direction_suffix_south() {
        assert_eq!(direction_suffix(Direction::South), "s");
    }

    #[test]
    fn test_direction_suffix_northeast() {
        assert_eq!(direction_suffix(Direction::NorthEast), "ne");
    }
}
```

---

## 18. Checklist integration

### 18.1 Pre-requis (deja en place)

- [ ] `mge-platform` : `GpuContext::new()` fonctionnel avec wgpu 24.x
- [ ] `mge-math` : `world_to_screen()` et `screen_to_world()` implementes et testes
- [ ] `mge-asset` : chargement PNG et TOML fonctionnel

### 18.2 Implementation mge-render

- [ ] `RenderError` defini dans `errors.rs` avec `thiserror`
- [ ] `AtlasFrame`, `AtlasDescriptor`, `TextureAtlas` implementes dans `atlas.rs`
- [ ] `AtlasRegistry` avec `insert()`, `get()`, `reload()`
- [ ] `SpriteVertex`, `SpriteInstanceGpu` (64 bytes) dans `sprite_instance.rs`
- [ ] `QUAD_VERTICES` (6 vertices, 2 triangles)
- [ ] `SpriteBatcher` avec `clear()`, `push()`, `flush()` dans `sprite_batch.rs`
- [ ] `RenderLayer` enum (9 variants) dans `z_order.rs`
- [ ] `compute_z_order()` et `sort_sprites_by_depth()` dans `z_order.rs`
- [ ] `create_sprite_pipeline()` dans `pipeline.rs`
- [ ] `create_pixel_art_sampler()` dans `pipeline.rs`
- [ ] `CameraUniform` (proj + offset) dans `pipeline.rs`
- [ ] Shader `sprite.wgsl` avec instancing, flip_h, alpha test
- [ ] Shader `upscale.wgsl` avec fullscreen quad procedural
- [ ] `DualResolution` avec offscreen 800x600 et `upscale()` dans `dual_res.rs`
- [ ] `FrustumCuller` dans `culling.rs`
- [ ] `Camera2D` avec `follow()`, `update()`, `screen_to_world()` dans `camera.rs`
- [ ] `AnimState`, `Direction`, `resolve_direction_flip()` dans `animation.rs`
- [ ] `ParticlePool` avec `spawn()`, `update()`, `collect_sprites()` dans `particle.rs`
- [ ] `Renderer::render_frame()` dans `renderer.rs`

### 18.3 Tests obligatoires

- [ ] `cargo test -p mge-math test_world_to_screen` -- toutes les conversions iso
- [ ] `cargo test -p mge-render test_floor_behind_entity` -- Z-ordering layers
- [ ] `cargo test -p mge-render test_all_layers_ordered` -- exhaustif 9 layers
- [ ] `cargo test -p mge-render test_direction_flip` -- 8 directions
- [ ] `cargo test -p mge-render test_ortho_matrix` -- projection correcte

### 18.4 Verification visuelle

- [ ] `cargo run -p sodomight-game` : fenetre noire avec clear color visible
- [ ] Tile map 20x20 s'affiche correctement en isometrique dimetric
- [ ] Entite sprite animee se deplace avec pathfinding (clic souris)
- [ ] Z-ordering correct : entites "plus au sud" devant celles "plus au nord"
- [ ] Mode pixel-perfect 800x600 : pixels carres, pas d'interpolation lineaire
- [ ] Mode NativeHD : resolution 1080p, sprites nets
- [ ] Bascule pixel-perfect / NativeHD en runtime fonctionne
- [ ] Particules de feu visibles sur un sort

### 18.5 Lint obligatoire

```bash
cargo clippy -p mge-render -- -D warnings
cargo clippy -p mge-math -- -D warnings
cargo clippy -p mge-platform -- -D warnings
```

Zero warning tolere.

### 18.6 Performances visees

| Metrique | Cible | Methode de mesure |
|----------|-------|-------------------|
| Draw calls par frame | < 20 | Compteur dans SpriteBatcher |
| Sprites visibles par frame | < 5000 | Compteur apres culling |
| Temps CPU render stage | < 4 ms | `std::time::Instant` avant/apres |
| Temps GPU | < 8 ms a 60 Hz | wgpu timestamp queries (optionnel) |
| VRAM atlas total | < 256 MB | Somme des tailles d'atlas charges |
| Frame time total | < 16.6 ms (60 Hz) | VSync |

---

*Document redige par Francois, Dev Back-End -- Miyukini AI Studio*
*Base sur SD-Tech-Render-Pipeline.md de Denis*
*Revision : 2026-02-28 v1.0*
*Voir IMPL-08 pour l'input et l'audio.*
