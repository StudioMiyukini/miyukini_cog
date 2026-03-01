<!-- @id: SD-Tech-Render-Pipeline @do: reference @role: tech-lead @layer: 3 @human: miyuk -->

# SD-Tech-Render-Pipeline -- Pipeline de Rendu wgpu Sodomight

**Auteur :** Denis (Chef Dev Senior, Miyukini AI Studio)
**Date :** 2026-02-28
**Statut :** Reference technique -- v1.0
**Projet :** Sodomight (clone fidele Diablo 2 LoD, assets maison)
**Moteur :** MGE (Miyukini Game Engine) -- wgpu + winit

---

## Table des matieres

1. [Architecture wgpu](#1-architecture-wgpu)
2. [Rendu isometrique dimetric 2:1](#2-rendu-isometrique-dimetric-21)
3. [Layers de rendu](#3-layers-de-rendu)
4. [Gestion des sprites](#4-gestion-des-sprites)
5. [Dual resolution](#5-dual-resolution)
6. [Batching et optimisation](#6-batching-et-optimisation)
7. [Systeme de particules](#7-systeme-de-particules)
8. [Hot-reload assets](#8-hot-reload-assets)
9. [Structures Rust du renderer](#9-structures-rust-du-renderer)
10. [Shaders WGSL](#10-shaders-wgsl)
11. [Invariants et performances](#11-invariants-et-performances)

---

## 1. Architecture wgpu

### 1.1 Initialisation Device, Queue, Surface

Le renderer wgpu est initialise via `mge-platform` au demarrage. La fenetre est
creee par winit, le device et la queue sont obtenus via wgpu.

```rust
// @id: sd-render-init @do: reference @role: engine @layer: 2 @human: miyuk
// Crate: mge-platform

pub struct GpuContext {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub window_size: (u32, u32),
}

impl GpuContext {
    pub fn new(window: &winit::window::Window) -> Result<Self, RenderError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window)
            .map_err(|e| RenderError::SurfaceCreation(e.to_string()))?;

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or(RenderError::NoAdapter)?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("MGE Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        ))
        .map_err(|e| RenderError::DeviceCreation(e.to_string()))?;

        let size = window.inner_size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
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
}
```

### 1.2 Swap chain et frame loop

Chaque frame, le renderer acquiert une texture de la swap chain, dessine dessus,
puis presente le resultat.

```rust
// @id: sd-render-frame-loop @do: reference @role: engine @layer: 2 @human: miyuk
// Crate: mge-render

pub fn render_frame(
    gpu: &GpuContext,
    renderer: &mut Renderer,
    world: &World,
) -> Result<(), RenderError> {
    // 1. Acquerir la texture de la swap chain.
    let output = gpu.surface.get_current_texture()
        .map_err(|e| RenderError::SwapChain(e.to_string()))?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    // 2. Creer un command encoder.
    let mut encoder = gpu.device.create_command_encoder(
        &wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        },
    );

    // 3. Render pass principal.
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Main Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: if renderer.dual_res.enabled {
                    &renderer.dual_res.offscreen_view
                } else {
                    &view
                },
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Dessiner chaque layer dans l'ordre.
        renderer.draw_floor_tiles(&mut render_pass, world);
        renderer.draw_shadows(&mut render_pass, world);
        renderer.draw_ground_items(&mut render_pass, world);
        renderer.draw_entities(&mut render_pass, world);
        renderer.draw_ground_effects(&mut render_pass, world);
        renderer.draw_projectiles(&mut render_pass, world);
        renderer.draw_world_ui(&mut render_pass, world);
    }

    // 4. Si dual-res, upscale vers la surface.
    if renderer.dual_res.enabled {
        renderer.dual_res.upscale(&mut encoder, &view, &gpu.device);
    }

    // 5. Render pass UI (toujours en resolution native).
    {
        let mut ui_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("UI Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
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

        renderer.draw_hud(&mut ui_pass, world);
        renderer.draw_cursor(&mut ui_pass, world);
    }

    // 6. Soumettre les commandes et presenter.
    gpu.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}
```

### 1.3 Pipeline de rendu principal

Le pipeline de rendu utilise un vertex shader et un fragment shader WGSL communs
a tous les sprites. Les sprites sont dessines via des instances (instance buffer).

```rust
// @id: sd-render-pipeline @do: reference @role: engine @layer: 2 @human: miyuk

pub fn create_sprite_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Sprite Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shaders/sprite.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Sprite Bind Group Layout"),
        entries: &[
            // Binding 0 : Camera uniform (projection + view matrix).
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
            // Binding 1 : Texture atlas.
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
            // Binding 2 : Sampler.
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
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Sprite Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[
                // Buffer 0 : Quad vertices (shared).
                SpriteVertex::layout(),
                // Buffer 1 : Instance data (per-sprite).
                SpriteInstance::layout(),
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
    })
}
```

---

## 2. Rendu isometrique dimetric 2:1

### 2.1 Formules de projection monde vers ecran

La projection dimetric 2:1 convertit les coordonnees monde (tiles) en coordonnees
ecran (pixels). Les tiles font 64x32 pixels en mode pixel-perfect.

```rust
// @id: sd-render-iso-projection @do: reference @role: engine @layer: 2 @human: miyuk
// Crate: mge-math

/// Convertit des coordonnees monde (tiles) en coordonnees ecran (pixels).
///
/// Projection dimetric 2:1 standard :
///   screen_x = (tile_x - tile_y) * (TILE_WIDTH / 2)
///   screen_y = (tile_x + tile_y) * (TILE_HEIGHT / 2)
///
/// Avec TILE_WIDTH = 64, TILE_HEIGHT = 32 :
///   screen_x = (tile_x - tile_y) * 32
///   screen_y = (tile_x + tile_y) * 16
pub fn world_to_screen(tile_x: f32, tile_y: f32) -> (f32, f32) {
    let screen_x = (tile_x - tile_y) * 32.0;
    let screen_y = (tile_x + tile_y) * 16.0;
    (screen_x, screen_y)
}

/// Convertit des coordonnees ecran (pixels) en coordonnees monde (tiles).
///
/// Inverse de la projection dimetric :
///   tile_x = (screen_x / 32 + screen_y / 16) / 2
///   tile_y = (screen_y / 16 - screen_x / 32) / 2
pub fn screen_to_world(screen_x: f32, screen_y: f32) -> (f32, f32) {
    let tile_x = (screen_x / 32.0 + screen_y / 16.0) / 2.0;
    let tile_y = (screen_y / 16.0 - screen_x / 32.0) / 2.0;
    (tile_x, tile_y)
}
```

### 2.2 Tiles 64x32px

Chaque tile de sol est un losange de 64 pixels de large et 32 pixels de haut.
Les tiles sont stockees dans des atlas PNG et referencees par index.

```
    +--32px--+
   /          \
  /            \  16px
 /              \
+       64px     +
 \              /
  \            /  16px
   \          /
    +--------+
```

### 2.3 Z-ordering : formule exacte pour le tri des sprites

Le tri des sprites en isometrique est critique pour l'affichage correct. Un sprite
"plus bas" sur l'ecran doit etre dessine devant un sprite "plus haut".

```rust
// @id: sd-render-z-order @do: reference @role: engine @layer: 2 @human: miyuk

/// Calcule la profondeur Z d'une entite en isometrique.
///
/// La formule combine la position monde et le layer de rendu :
///   z = layer_base * 10000 + (tile_x + tile_y) * 100 + sub_order
///
/// - layer_base : priorite du layer (0 = sol, 3 = entites, 5 = projectiles...)
/// - (tile_x + tile_y) : profondeur isometrique (plus la somme est grande,
///   plus l'entite est "en bas" de l'ecran, donc devant)
/// - sub_order : tri secondaire pour les entites au meme endroit
pub fn compute_z_order(tile_x: f32, tile_y: f32, layer: RenderLayer, sub_order: f32) -> f32 {
    let layer_base = match layer {
        RenderLayer::Floor => 0.0,
        RenderLayer::Shadow => 1.0,
        RenderLayer::GroundItem => 2.0,
        RenderLayer::Entity => 3.0,
        RenderLayer::OverlayEffect => 4.0,
        RenderLayer::Projectile => 5.0,
        RenderLayer::Foreground => 6.0,
        RenderLayer::Weather => 7.0,
        RenderLayer::Ui => 8.0,
    };
    layer_base * 10000.0 + (tile_x + tile_y) * 100.0 + sub_order
}

/// Trie les sprites par profondeur Z (du plus loin au plus proche).
/// Les sprites avec un Z plus petit sont dessines en premier (derriere).
pub fn sort_sprites_by_depth(sprites: &mut Vec<SpriteInstance>) {
    sprites.sort_by(|a, b| a.z_order.partial_cmp(&b.z_order).unwrap_or(std::cmp::Ordering::Equal));
}
```

### 2.4 Profondeur isometrique -- regles de tri

| Situation | Regle |
|-----------|-------|
| Entite A plus au nord que B | A est derriere B (z plus petit) |
| Entite A au meme rang que B | Tri par position Y ecran |
| Entite et mur au meme rang | Le mur est devant si son layer est Foreground |
| Item au sol | Toujours derriere les entites (layer GroundItem < Entity) |
| Projectile | Toujours devant les entites (layer Projectile > Entity) |
| Ombre | Toujours derriere les items (layer Shadow < GroundItem) |

---

## 3. Layers de rendu

### 3.1 Ordre exact de rendu (9 layers)

Le renderer dessine les layers dans un ordre strict. Chaque layer est un pass de
dessin separe avec son propre tri.

| # | Layer | Enum | Contenu | Tri |
|---|-------|------|---------|-----|
| 0 | Sol | `Floor` | Tiles de base (herbe, pierre, sable) | Par position dans la tilemap |
| 1 | Ombres | `Shadow` | Ombres portees des entites | Par position de l'entite parente |
| 2 | Objets au sol | `GroundItem` | Items droppes, gold, potions | Par position Z iso |
| 3 | Entites | `Entity` | Joueurs, monstres, NPCs, mercenaires | Par position Z iso (critique) |
| 4 | Effets au sol | `OverlayEffect` | AoE actives, zones de sort, auras visuelles | Par position Z iso |
| 5 | Projectiles | `Projectile` | Fleches, bone spear, fire ball, etc. | Par position Z iso |
| 6 | Premier plan | `Foreground` | Murs, arbres au premier plan, toits | Par position dans la tilemap |
| 7 | Meteo | `Weather` | Pluie, neige, brouillard | Pas de tri (plein ecran) |
| 8 | UI monde | `Ui` | Noms d'entites, barres de vie, damage numbers | Position ecran fixe |

### 3.2 Layers supplementaires (hors-monde)

En plus des layers monde, le HUD et le curseur sont dessines directement sur la
surface finale (pas dans le framebuffer offscreen en mode dual-res).

| Layer | Contenu | Notes |
|-------|---------|-------|
| HUD | Orbes vie/mana, belt, hotbar, minimap | Toujours en resolution native |
| Inventaire | Grille, paperdoll, skill tree (si ouvert) | Toujours en resolution native |
| Tooltips | Tooltip d'item, stat sheet | Dessine par-dessus tout |
| Curseur | Curseur souris (custom sprite) | Dernier element dessine |

---

## 4. Gestion des sprites

### 4.1 Format des assets

| Type | Format | Outil |
|------|--------|-------|
| Spritesheets | PNG 32-bit RGBA | Aseprite, mge-packer |
| Atlas descripteur | TOML (mge-packer output) | mge-packer |
| Maps | LDtk JSON | LDtk editor |
| Audio | OGG Vorbis | Audacity |

### 4.2 Chargement d'atlas (bind group wgpu)

Chaque atlas de textures est charge comme une texture GPU unique. Un bind group
est cree pour chaque atlas, contenant la texture et le sampler.

```rust
// @id: sd-render-atlas @do: reference @role: engine @layer: 2 @human: miyuk

pub struct TextureAtlas {
    pub id: String,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
    pub width: u32,
    pub height: u32,
    pub frames: HashMap<String, AtlasFrame>,
}

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

impl TextureAtlas {
    pub fn load(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
        image_path: &str,
        descriptor_path: &str,
    ) -> Result<Self, AssetError> {
        // 1. Charger l'image PNG.
        let img = image::open(image_path)
            .map_err(|e| AssetError::ImageLoad(e.to_string()))?
            .to_rgba8();

        let (width, height) = img.dimensions();

        // 2. Creer la texture GPU.
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(image_path),
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

        // 4. Creer le bind group.
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("{} bind group", image_path)),
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

        // 5. Charger le descripteur TOML.
        let descriptor_content = std::fs::read_to_string(descriptor_path)
            .map_err(|e| AssetError::FileRead(e.to_string()))?;
        let descriptor: AtlasDescriptor = toml::from_str(&descriptor_content)
            .map_err(|e| AssetError::TomlParse(e.to_string()))?;

        let frames: HashMap<String, AtlasFrame> = descriptor
            .frames
            .into_iter()
            .map(|f| (f.name.clone(), f))
            .collect();

        Ok(Self {
            id: image_path.to_string(),
            texture,
            view,
            bind_group,
            width,
            height,
            frames,
        })
    }

    /// Retourne les coordonnees UV normalisees pour une frame donnee.
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlasDescriptor {
    pub frames: Vec<AtlasFrame>,
}
```

### 4.3 Animation : frame timer, state machine

L'animation des sprites est geree par le composant `AnimState` (defini dans
SD-Tech-ECS-Components). Le systeme d'animation avance les frames a chaque
frame de rendu.

```rust
// @id: sd-render-animation @do: reference @role: engine @layer: 2 @human: miyuk

pub fn animation_system(
    time: Res<FrameTime>,
    mut query: Query<(&mut AnimState, &mut Sprite)>,
    atlas_registry: Res<AtlasRegistry>,
) {
    for (mut anim, mut sprite) in query.iter_mut() {
        if anim.finished {
            continue;
        }

        // Accumuler le temps.
        anim.elapsed += time.delta_seconds;

        // Calculer la duree par frame.
        let frame_duration = 1.0 / anim.fps;

        // Avancer les frames.
        while anim.elapsed >= frame_duration {
            anim.elapsed -= frame_duration;
            anim.current_frame += 1;

            if anim.current_frame >= anim.total_frames {
                if anim.looping {
                    anim.current_frame = 0;
                } else {
                    anim.current_frame = anim.total_frames - 1;
                    anim.finished = true;
                    break;
                }
            }
        }

        // Mettre a jour la region UV du sprite dans l'atlas.
        let frame_name = format!(
            "{}_{}_{}",
            sprite.sprite_id.0,
            direction_suffix(&anim.direction),
            anim.current_frame
        );

        if let Some(atlas) = atlas_registry.get(&sprite.atlas_id) {
            if let Some(uv) = atlas.get_uv(&frame_name) {
                sprite.uv_rect = UvRect {
                    x: (uv[0] * atlas.width as f32) as u32,
                    y: (uv[1] * atlas.height as f32) as u32,
                    w: ((uv[2] - uv[0]) * atlas.width as f32) as u32,
                    h: ((uv[3] - uv[1]) * atlas.height as f32) as u32,
                };
            }
        }
    }
}

/// Convertit une direction en suffixe de nom de frame.
fn direction_suffix(dir: &Direction) -> &'static str {
    match dir {
        Direction::South => "s",
        Direction::SouthWest => "sw",
        Direction::West => "w",
        Direction::NorthWest => "nw",
        Direction::North => "n",
        Direction::NorthEast => "ne",
        Direction::East => "e",
        Direction::SouthEast => "se",
    }
}
```

### 4.4 Sprite flip (miroir horizontal pour directions)

Pour economiser de la memoire, certaines directions sont rendues en miroir
horizontal d'une autre direction. Par convention :

| Direction source | Direction miroir | Flip H |
|------------------|-----------------|--------|
| East | West | Oui |
| NorthEast | NorthWest | Oui |
| SouthEast | SouthWest | Oui |

```rust
// @id: sd-render-flip @do: reference @role: engine @layer: 2 @human: miyuk

/// Determine si une direction doit utiliser un flip horizontal.
/// Retourne (direction_source_pour_atlas, flip_h).
pub fn resolve_direction_flip(dir: Direction) -> (Direction, bool) {
    match dir {
        Direction::West => (Direction::East, true),
        Direction::NorthWest => (Direction::NorthEast, true),
        Direction::SouthWest => (Direction::SouthEast, true),
        other => (other, false),
    }
}
```

### 4.5 Les 8 directions isometriques

```
         N (0)
   NW (7)   NE (1)
  W (6)       E (2)
   SW (5)   SE (3)
         S (4)
```

Les spritesheets contiennent 5 directions (S, SW, W, NW, N). Les 3 autres
(SE, E, NE) sont obtenues par flip horizontal.

---

## 5. Dual resolution

### 5.1 Mode pixel-perfect 800x600

En mode pixel-perfect, le monde est rendu dans un framebuffer offscreen de 800x600
pixels. Le resultat est ensuite upscale vers la resolution native de la fenetre
en utilisant un filtre nearest-neighbor (pas de flou).

```rust
// @id: sd-render-dual-res @do: reference @role: engine @layer: 2 @human: miyuk

pub struct DualResolution {
    pub enabled: bool,
    pub mode: ResolutionMode,
    pub offscreen_texture: wgpu::Texture,
    pub offscreen_view: wgpu::TextureView,
    pub offscreen_bind_group: wgpu::BindGroup,
    pub upscale_pipeline: wgpu::RenderPipeline,
    pub upscale_sampler: wgpu::Sampler,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionMode {
    /// Rendu en 800x600, upscale nearest-neighbor.
    PixelPerfect,
    /// Rendu en resolution native, sprites HD optionnels.
    NativeHD,
}

impl DualResolution {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        mode: ResolutionMode,
    ) -> Self {
        let (render_width, render_height) = match mode {
            ResolutionMode::PixelPerfect => (800, 600),
            ResolutionMode::NativeHD => (1920, 1080), // Ou la resolution native.
        };

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

        // Sampler nearest-neighbor pour l'upscale pixel-perfect.
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

        // Le upscale_pipeline et le bind_group sont crees via un fullscreen quad shader.
        // (Simplifie ici pour la documentation.)

        Self {
            enabled: true,
            mode,
            offscreen_texture,
            offscreen_view,
            offscreen_bind_group: todo!("Created from upscale_sampler + offscreen_view"),
            upscale_pipeline: todo!("Fullscreen quad pipeline"),
            upscale_sampler,
        }
    }

    /// Upscale le framebuffer offscreen vers la surface finale.
    pub fn upscale(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target_view: &wgpu::TextureView,
        _device: &wgpu::Device,
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
}
```

### 5.2 Bascule en runtime

Le joueur peut basculer entre les deux modes via les options. La bascule
recree le framebuffer offscreen et le sampler.

```rust
// @id: sd-render-mode-switch @do: reference @role: engine @layer: 2 @human: miyuk

pub fn switch_resolution_mode(
    renderer: &mut Renderer,
    device: &wgpu::Device,
    new_mode: ResolutionMode,
) {
    if renderer.dual_res.mode == new_mode {
        return;
    }

    renderer.dual_res = DualResolution::new(
        device,
        renderer.surface_format,
        new_mode,
    );
}
```

---

## 6. Batching et optimisation

### 6.1 Sprite batcher

Le sprite batcher regroupe tous les sprites qui utilisent le meme atlas de textures
en un seul draw call. Cela reduit massivement le nombre de state changes GPU.

```rust
// @id: sd-render-batcher @do: reference @role: engine @layer: 2 @human: miyuk

pub struct SpriteBatcher {
    /// Batches groupes par atlas_id.
    pub batches: HashMap<String, SpriteBatch>,
    /// Instance buffer partage (realloue si necessaire).
    pub instance_buffer: wgpu::Buffer,
    pub instance_buffer_capacity: usize,
}

pub struct SpriteBatch {
    pub atlas_id: String,
    pub instances: Vec<SpriteInstanceGpu>,
}

/// Donnees GPU par instance de sprite (64 bytes).
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SpriteInstanceGpu {
    /// Position ecran (pixels).
    pub position: [f32; 2],
    /// Coordonnees UV dans l'atlas [u_min, v_min, u_max, v_max].
    pub uv_rect: [f32; 4],
    /// Taille du sprite en pixels [width, height].
    pub size: [f32; 2],
    /// Tint color RGBA.
    pub tint: [f32; 4],
    /// Flags : bit 0 = flip_h, bit 1 = flip_v.
    pub flags: u32,
    /// Padding pour alignement 16 bytes.
    pub _padding: [u32; 3],
}

impl SpriteBatcher {
    pub fn new(device: &wgpu::Device) -> Self {
        let initial_capacity = 4096;
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Sprite Instance Buffer"),
            size: (initial_capacity * std::mem::size_of::<SpriteInstanceGpu>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            batches: HashMap::new(),
            instance_buffer,
            instance_buffer_capacity: initial_capacity,
        }
    }

    /// Vide tous les batches pour un nouveau frame.
    pub fn clear(&mut self) {
        for batch in self.batches.values_mut() {
            batch.instances.clear();
        }
    }

    /// Ajoute un sprite a dessiner.
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
    pub fn flush(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        atlas_registry: &AtlasRegistry,
        pipeline: &wgpu::RenderPipeline,
        camera_bind_group: &wgpu::BindGroup,
    ) {
        render_pass.set_pipeline(pipeline);

        for batch in self.batches.values() {
            if batch.instances.is_empty() {
                continue;
            }

            // Recuperer l'atlas.
            let atlas = match atlas_registry.get(&batch.atlas_id) {
                Some(a) => a,
                None => continue,
            };

            // Reallouer le buffer si necessaire.
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

            // Dessiner.
            render_pass.set_bind_group(0, camera_bind_group, &[]);
            render_pass.set_bind_group(1, &atlas.bind_group, &[]);
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.draw(0..6, 0..batch.instances.len() as u32);
        }
    }
}
```

### 6.2 Frustum culling

Seuls les sprites visibles a l'ecran sont envoyes au batcher. Le culling est
effectue en coordonnees ecran apres la projection isometrique.

```rust
// @id: sd-render-culling @do: reference @role: engine @layer: 2 @human: miyuk

pub struct FrustumCuller {
    pub screen_min_x: f32,
    pub screen_min_y: f32,
    pub screen_max_x: f32,
    pub screen_max_y: f32,
    /// Marge en pixels pour eviter le popping.
    pub margin: f32,
}

impl FrustumCuller {
    pub fn new(camera: &Camera, render_width: f32, render_height: f32) -> Self {
        Self {
            screen_min_x: camera.x - render_width / 2.0,
            screen_min_y: camera.y - render_height / 2.0,
            screen_max_x: camera.x + render_width / 2.0,
            screen_max_y: camera.y + render_height / 2.0,
            margin: 128.0, // 2 tiles de marge.
        }
    }

    /// Retourne true si le sprite est potentiellement visible.
    pub fn is_visible(&self, screen_x: f32, screen_y: f32, width: f32, height: f32) -> bool {
        screen_x + width >= self.screen_min_x - self.margin
            && screen_x <= self.screen_max_x + self.margin
            && screen_y + height >= self.screen_min_y - self.margin
            && screen_y <= self.screen_max_y + self.margin
    }
}
```

### 6.3 Statistiques de performance visees

| Metrique | Cible | Methode |
|----------|-------|---------|
| Draw calls par frame | < 20 | Batching par atlas |
| Sprites par frame | < 5000 | Frustum culling |
| Temps CPU render stage | < 4 ms | Tri Z optimise |
| Temps GPU | < 8 ms a 60 Hz | Instancing |
| VRAM atlas | < 256 MB | Atlas packs, pas de doublons |
| Frame time total | < 16.6 ms (60 Hz) | VSync |

---

## 7. Systeme de particules

### 7.1 Architecture

Les particules utilisent un pool pre-alloue pour eviter les allocations en cours
de jeu. Chaque emetteur definit les parametres de spawn.

```rust
// @id: sd-render-particles @do: reference @role: engine @layer: 2 @human: miyuk

pub struct ParticlePool {
    pub particles: Vec<Particle>,
    pub max_particles: usize,
    pub active_count: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub active: bool,
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub acceleration: [f32; 2],
    pub color: [f32; 4],
    pub color_end: [f32; 4],
    pub size: f32,
    pub size_end: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub rotation: f32,
    pub rotation_speed: f32,
}

impl ParticlePool {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: vec![Particle::default(); max_particles],
            max_particles,
            active_count: 0,
        }
    }

    /// Alloue une particule depuis le pool.
    pub fn spawn(&mut self) -> Option<&mut Particle> {
        for p in &mut self.particles {
            if !p.active {
                p.active = true;
                self.active_count += 1;
                return Some(p);
            }
        }
        None // Pool plein.
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
                self.active_count -= 1;
                continue;
            }

            let t = 1.0 - (p.lifetime / p.max_lifetime);

            // Mouvement.
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
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
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
```

### 7.2 Emetteurs de particules

```rust
// @id: sd-render-emitter @do: reference @role: engine @layer: 2 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticleEmitterDef {
    pub emitter_type: ParticleType,
    pub spawn_rate: f32,
    pub burst_count: u32,
    pub lifetime_min: f32,
    pub lifetime_max: f32,
    pub velocity_min: [f32; 2],
    pub velocity_max: [f32; 2],
    pub acceleration: [f32; 2],
    pub color_start: [f32; 4],
    pub color_end: [f32; 4],
    pub size_start: f32,
    pub size_end: f32,
    pub spread_angle: f32,
    pub sprite_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParticleType {
    Fire,
    Ice,
    Poison,
    Explosion,
    Blood,
    CoinRain,
    Lightning,
    Holy,
    Smoke,
    Dust,
    Rain,
    Snow,
    LevelUp,
    Custom,
}
```

### 7.3 Types de particules predefinies

| Type | Spawn rate | Lifetime | Couleur depart | Couleur fin | Gravite |
|------|-----------|----------|---------------|-------------|---------|
| Fire | 50/sec | 0.5-1.0 s | (1.0, 0.6, 0.0, 1.0) | (1.0, 0.0, 0.0, 0.0) | -20 (monte) |
| Ice | 30/sec | 0.8-1.5 s | (0.5, 0.8, 1.0, 1.0) | (0.8, 0.9, 1.0, 0.0) | 5 (tombe) |
| Poison | 20/sec | 1.0-2.0 s | (0.2, 0.8, 0.1, 0.8) | (0.1, 0.4, 0.0, 0.0) | -5 |
| Explosion | burst 30 | 0.3-0.6 s | (1.0, 0.8, 0.2, 1.0) | (0.5, 0.1, 0.0, 0.0) | 10 |
| Blood | burst 15 | 0.2-0.5 s | (0.8, 0.0, 0.0, 1.0) | (0.3, 0.0, 0.0, 0.0) | 30 |
| CoinRain | burst 20 | 0.5-1.0 s | (1.0, 0.85, 0.0, 1.0) | (0.8, 0.6, 0.0, 0.0) | 40 |
| LevelUp | burst 50 | 1.0-2.0 s | (1.0, 1.0, 0.5, 1.0) | (1.0, 1.0, 1.0, 0.0) | -30 |

---

## 8. Hot-reload assets

### 8.1 Systeme de watch

Le crate `notify` surveille le dossier `assets/` pour les modifications de fichiers.
Quand un fichier est modifie, l'asset correspondant est recharge sans redemarrage.

```rust
// @id: sd-render-hot-reload @do: reference @role: engine @layer: 2 @human: miyuk
// Crate: mge-asset

pub struct HotReloadWatcher {
    watcher: notify::RecommendedWatcher,
    rx: std::sync::mpsc::Receiver<notify::Result<notify::Event>>,
    pending_reloads: Vec<String>,
}

impl HotReloadWatcher {
    pub fn new(watch_path: &str) -> Result<Self, AssetError> {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })
        .map_err(|e| AssetError::WatcherInit(e.to_string()))?;

        notify::Watcher::watch(
            &mut watcher,
            std::path::Path::new(watch_path),
            notify::RecursiveMode::Recursive,
        )
        .map_err(|e| AssetError::WatcherInit(e.to_string()))?;

        Ok(Self {
            watcher,
            rx,
            pending_reloads: Vec::new(),
        })
    }

    /// Collecte les fichiers modifies depuis le dernier appel.
    pub fn poll(&mut self) -> Vec<String> {
        self.pending_reloads.clear();

        while let Ok(Ok(event)) = self.rx.try_recv() {
            if matches!(
                event.kind,
                notify::EventKind::Modify(_) | notify::EventKind::Create(_)
            ) {
                for path in event.paths {
                    if let Some(path_str) = path.to_str() {
                        self.pending_reloads.push(path_str.to_string());
                    }
                }
            }
        }

        self.pending_reloads.clone()
    }
}
```

### 8.2 Cache d'assets

```rust
// @id: sd-render-asset-cache @do: reference @role: engine @layer: 2 @human: miyuk

pub struct AssetCache {
    pub atlases: HashMap<String, Arc<TextureAtlas>>,
    pub hot_reload: Option<HotReloadWatcher>,
}

impl AssetCache {
    pub fn new(enable_hot_reload: bool, watch_path: &str) -> Result<Self, AssetError> {
        let hot_reload = if enable_hot_reload {
            Some(HotReloadWatcher::new(watch_path)?)
        } else {
            None
        };

        Ok(Self {
            atlases: HashMap::new(),
            hot_reload,
        })
    }

    /// Retourne un atlas, le chargeant si necessaire.
    pub fn get_atlas(
        &mut self,
        atlas_id: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
    ) -> Result<Arc<TextureAtlas>, AssetError> {
        if let Some(atlas) = self.atlases.get(atlas_id) {
            return Ok(Arc::clone(atlas));
        }

        let image_path = format!("assets/atlases/{}.png", atlas_id);
        let desc_path = format!("assets/atlases/{}.toml", atlas_id);

        let atlas = TextureAtlas::load(device, queue, layout, sampler, &image_path, &desc_path)?;
        let arc = Arc::new(atlas);
        self.atlases.insert(atlas_id.to_string(), Arc::clone(&arc));
        Ok(arc)
    }

    /// Verifie les modifications et recharge les assets modifies.
    pub fn check_hot_reload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
    ) {
        let modified_files = match &mut self.hot_reload {
            Some(watcher) => watcher.poll(),
            None => return,
        };

        for file_path in modified_files {
            // Determiner quel atlas est affecte.
            if file_path.ends_with(".png") || file_path.ends_with(".toml") {
                let atlas_id = extract_atlas_id(&file_path);
                if self.atlases.contains_key(&atlas_id) {
                    // Recharger l'atlas.
                    let image_path = format!("assets/atlases/{}.png", atlas_id);
                    let desc_path = format!("assets/atlases/{}.toml", atlas_id);

                    match TextureAtlas::load(device, queue, layout, sampler, &image_path, &desc_path) {
                        Ok(new_atlas) => {
                            self.atlases.insert(atlas_id, Arc::new(new_atlas));
                        }
                        Err(e) => {
                            eprintln!("Hot-reload failed for {}: {:?}", file_path, e);
                        }
                    }
                }
            }
        }
    }
}

fn extract_atlas_id(path: &str) -> String {
    std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}
```

---

## 9. Structures Rust du renderer

### 9.1 SpriteInstance

```rust
// @id: sd-render-sprite-instance @do: reference @role: engine @layer: 2 @human: miyuk

/// Instance de sprite a dessiner (donnees CPU avant upload GPU).
#[derive(Debug, Clone)]
pub struct SpriteInstance {
    /// Position en coordonnees ecran (pixels).
    pub position: Vec2,
    /// Coordonnees UV dans l'atlas [u_min, v_min, u_max, v_max].
    pub uv_rect: [f32; 4],
    /// Profondeur Z pour le tri (compute_z_order).
    pub z_order: f32,
    /// Tint RGBA (1.0 = pas de teinte).
    pub tint: [f32; 4],
    /// Flip horizontal.
    pub flip_x: bool,
    /// Taille du sprite en pixels.
    pub size: Vec2,
    /// Atlas source.
    pub atlas_id: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
```

### 9.2 RenderLayer

```rust
// @id: sd-render-layer @do: reference @role: engine @layer: 2 @human: miyuk

/// Couche de rendu avec ses sprites tries.
pub struct RenderLayerData {
    pub id: RenderLayer,
    pub sprites: Vec<SpriteInstance>,
    pub z_base: f32,
}

impl RenderLayerData {
    pub fn new(id: RenderLayer) -> Self {
        let z_base = match id {
            RenderLayer::Floor => 0.0,
            RenderLayer::Shadow => 10000.0,
            RenderLayer::GroundItem => 20000.0,
            RenderLayer::Entity => 30000.0,
            RenderLayer::OverlayEffect => 40000.0,
            RenderLayer::Projectile => 50000.0,
            RenderLayer::Foreground => 60000.0,
            RenderLayer::Weather => 70000.0,
            RenderLayer::Ui => 80000.0,
        };
        Self {
            id,
            sprites: Vec::new(),
            z_base,
        }
    }

    /// Trie les sprites par Z croissant (arriere vers avant).
    pub fn sort(&mut self) {
        self.sprites.sort_by(|a, b| {
            a.z_order
                .partial_cmp(&b.z_order)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }
}
```

### 9.3 Camera isometrique

```rust
// @id: sd-render-camera @do: reference @role: engine @layer: 2 @human: miyuk

pub struct Camera {
    /// Position du centre de la camera en coordonnees ecran.
    pub x: f32,
    pub y: f32,
    /// Position monde (tile) que la camera suit.
    pub follow_tile_x: f32,
    pub follow_tile_y: f32,
    /// Zoom (1.0 = normal).
    pub zoom: f32,
    /// Largeur et hauteur de rendu.
    pub render_width: f32,
    pub render_height: f32,
    /// Smooth follow speed (0.0-1.0, 1.0 = instantane).
    pub smooth_speed: f32,
}

impl Camera {
    /// Met a jour la position de la camera pour suivre une entite.
    pub fn update(&mut self, target_tile_x: f32, target_tile_y: f32, dt: f32) {
        let (target_screen_x, target_screen_y) = world_to_screen(target_tile_x, target_tile_y);

        // Interpolation lineaire pour le suivi smooth.
        let t = (self.smooth_speed * dt * 60.0).min(1.0);
        self.x += (target_screen_x - self.x) * t;
        self.y += (target_screen_y - self.y) * t;

        self.follow_tile_x = target_tile_x;
        self.follow_tile_y = target_tile_y;
    }

    /// Convertit des coordonnees ecran en coordonnees monde.
    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
        let world_screen_x = screen_x - self.render_width / 2.0 + self.x;
        let world_screen_y = screen_y - self.render_height / 2.0 + self.y;
        screen_to_world(world_screen_x, world_screen_y)
    }

    /// Retourne la matrice de projection orthographique pour le shader.
    pub fn projection_matrix(&self) -> [[f32; 4]; 4] {
        let hw = self.render_width / 2.0 / self.zoom;
        let hh = self.render_height / 2.0 / self.zoom;
        let left = self.x - hw;
        let right = self.x + hw;
        let bottom = self.y + hh;
        let top = self.y - hh;

        ortho_matrix(left, right, bottom, top)
    }
}

fn ortho_matrix(left: f32, right: f32, bottom: f32, top: f32) -> [[f32; 4]; 4] {
    let width = right - left;
    let height = top - bottom;
    [
        [2.0 / width, 0.0, 0.0, 0.0],
        [0.0, 2.0 / height, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [
            -(right + left) / width,
            -(top + bottom) / height,
            0.0,
            1.0,
        ],
    ]
}
```

---

## 10. Shaders WGSL

### 10.1 Sprite shader (vertex + fragment)

```wgsl
// @id: sd-render-shader-sprite @do: reference @role: engine @layer: 2 @human: miyuk
// Fichier: crates/engine/mge-render/src/shaders/sprite.wgsl

// === Uniforms ===

struct CameraUniform {
    projection: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(0) @binding(1)
var sprite_texture: texture_2d<f32>;

@group(0) @binding(2)
var sprite_sampler: sampler;

// === Vertex Input ===

struct VertexInput {
    // Quad vertices (0..5 pour 2 triangles).
    @builtin(vertex_index) vertex_index: u32,
};

// === Instance Input ===

struct InstanceInput {
    @location(0) position: vec2<f32>,
    @location(1) uv_min: vec2<f32>,
    @location(2) uv_max: vec2<f32>,
    @location(3) size: vec2<f32>,
    @location(4) tint: vec4<f32>,
    @location(5) flags: u32,
};

// === Vertex Output ===

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) tint: vec4<f32>,
};

// === Vertex Shader ===

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    // Quad : 2 triangles (0,1,2) et (2,3,0) via 6 vertices.
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0),  // top-left
        vec2<f32>(1.0, 0.0),  // top-right
        vec2<f32>(1.0, 1.0),  // bottom-right
        vec2<f32>(1.0, 1.0),  // bottom-right
        vec2<f32>(0.0, 1.0),  // bottom-left
        vec2<f32>(0.0, 0.0),  // top-left
    );

    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(0.0, 0.0),
    );

    let pos = positions[vertex.vertex_index];
    let uv_base = uvs[vertex.vertex_index];

    // Position du sprite en pixels.
    var world_pos = instance.position + pos * instance.size;

    // Flip horizontal.
    let flip_h = (instance.flags & 1u) != 0u;
    if (flip_h) {
        world_pos.x = instance.position.x + instance.size.x - pos.x * instance.size.x;
    }

    // Projection.
    var out: VertexOutput;
    out.clip_position = camera.projection * vec4<f32>(world_pos, 0.0, 1.0);

    // UV mapping dans l'atlas.
    var uv = mix(instance.uv_min, instance.uv_max, uv_base);
    if (flip_h) {
        uv.x = instance.uv_min.x + instance.uv_max.x - uv.x;
    }
    out.uv = uv;

    out.tint = instance.tint;

    return out;
}

// === Fragment Shader ===

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(sprite_texture, sprite_sampler, in.uv);

    // Discard pixels transparents.
    if (tex_color.a < 0.01) {
        discard;
    }

    // Appliquer la teinte.
    return tex_color * in.tint;
}
```

### 10.2 Upscale shader (fullscreen quad)

```wgsl
// @id: sd-render-shader-upscale @do: reference @role: engine @layer: 2 @human: miyuk
// Fichier: crates/engine/mge-render/src/shaders/upscale.wgsl

@group(0) @binding(0)
var source_texture: texture_2d<f32>;

@group(0) @binding(1)
var source_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // Fullscreen triangle (3 vertices couvrent tout l'ecran).
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>(-1.0, -1.0),
    );

    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 1.0),
    );

    var out: VertexOutput;
    out.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    out.uv = uvs[vertex_index];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(source_texture, source_sampler, in.uv);
}
```

---

## 11. Invariants et performances

### 11.1 Invariants du renderer

| Invariant | Description | Verification |
|-----------|-------------|-------------|
| Pas de depth buffer | Le tri Z est fait en CPU, pas en GPU | Architecture pipeline |
| Alpha blending obligatoire | Tous les sprites utilisent ALPHA_BLENDING | Pipeline state |
| Nearest-neighbor en pixel-perfect | Le sampler utilise FilterMode::Nearest | Sampler config |
| Tiles 64x32 fixe | La taille des tiles ne change jamais | Constante TILE_WIDTH/HEIGHT |
| Projection dimetric 2:1 | Ratio 2:1 strict (64 large, 32 haut) | Formules world_to_screen |
| UV dans [0,1] | Les UV sont normalises par rapport a l'atlas | get_uv() |
| Max 1 texture par draw call | Le batching garantit 1 atlas = 1 draw call | SpriteBatcher |
| Pas de unsafe dans mge-render | unsafe_code = "forbid" | Cargo.toml |

### 11.2 Budget de performance par stage

| Operation | Budget | Metrique |
|-----------|--------|----------|
| Frustum culling | < 0.5 ms | CPU |
| Z-sort | < 1.0 ms | CPU |
| Instance buffer upload | < 0.5 ms | CPU-GPU |
| Draw calls | < 2.0 ms | GPU |
| Upscale pass | < 0.3 ms | GPU |
| UI render | < 1.0 ms | GPU |
| Total render stage | < 5.3 ms | Frame budget a 60 Hz |

### 11.3 Limites du systeme

| Limite | Valeur | Justification |
|--------|--------|---------------|
| Sprites max par frame | 8192 | Instance buffer initial |
| Particules actives max | 4096 | Pool pre-alloue |
| Atlas max en VRAM | 32 | HashMap capacity |
| Taille max atlas | 4096x4096 px | Limite GPU courante |
| Emetteurs de particules max | 256 | Pool pre-alloue |

---

*Document redige par Denis, Chef Dev Senior -- Miyukini AI Studio*
*Revision : 2026-02-28 v1.0*
