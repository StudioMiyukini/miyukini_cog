# MGE — Rendu 2D Isometrique Robuste

**Version :** 2.0
**Date :** 2026-02-23
**Statut :** Specification normative — Production Target
**Applicable a :** `mge-plugin-render-2d`, `allumina_prototype`, tout jeu MGE isometrique
**Audience :** Developpeurs moteur, developpeurs gameplay, pipeline artistes
**Remplace :** v1.0 (specification initiale 2026-02-23)
**Crate cible :** `mge/crates/mge-plugin-render-2d`

---

## Contexte

L'audit du pipeline de rendu `allumina_prototype` (fevrier 2026) a identifie cinq bugs critiques P1 et trois bugs P2 impactant la qualite visuelle, la portabilite multiplateforme et la fiabilite du tri de profondeur. Ce document formalise la specification normative v2.0 du pipeline graphique MGE, alignee sur les pratiques industrie etablies (Diablo II, Godot 4, Bevy, PixiJS) et concu pour etre a parite de feature avec les moteurs concurrents sur le segment 2D isometrique.

---

## Analyse concurrentielle

Avant de specifier le pipeline MGE, il est utile d'examiner comment les moteurs de reference ont resolu les memes problemes.

### Diablo II — Blizzard (2000)

Moteur 2D pur, 640x480, 256 couleurs par sprite palette. Reference du genre isometrique.

- **Painter's algorithm strict :** les tuiles et sprites sont dessines arriere vers avant, sans depth buffer.
- **Pivot BottomCenter absolu :** la position monde d'une entite correspond aux "pieds" du sprite (bas-centre). Le quad est positionne tel que `top_left = (foot_x - w/2, foot_y - h)`.
- **Y-sort par `screen_y` des pieds :** avant chaque frame, les sprites sont tries par la coordonnee ecran de leur point pied. Pas de tri GPU-side.
- **Sprites DC6 independants :** chaque entite charge ses propres sprites (pas d'atlas global), compresses en RLE 256 couleurs.
- **Rendu CPU pur :** pas de GPU, blitter MMX, pas d'instancing.

**Lecon cle :** Le pivot BottomCenter + Y-sort par position pieds est la fondation correcte du rendu isometrique. Le MGE doit reproduire exactement ce comportement.

### Godot 4 — open source (GDScript/C++)

- **`CanvasItem.z_index`** : entier pour le tri de couche.
- **`y_sort_enabled`** sur les noeuds conteneurs : tri automatique des enfants par `position.y`.
- **`CanvasLayer`** : couche ecran pure, independante de la camera, pour UI et overlays.
- **DPI-awareness** : `DisplayServer.screen_get_dpi()` + `Window.content_scale_factor`.
- **TextureAtlas** : import via `.atlasv` avec metadata pivot par sprite.
- **Pixel-perfect** : `Viewport.snap_2d_vertices_to_pixel` + taille fixe viewport.

### Bevy — open source (Rust)

- **Y-sort GPU-side** : `Transform.translation.z = -transform.translation.y` pour exploiter le depth buffer.
- **`TextureAtlas`** avec metadata JSON (format compatible TexturePacker).
- **`Camera2d`** avec `OrthographicProjection` et `ScalingMode::WindowSize` pour pixel-perfect.
- **Render graph explicite** : passes nommees et ordonnees, pas de render pass monolithique.
- **`bevy_sprite`** : `Sprite.anchor = Anchor::BottomCenter` natif.

### PixiJS / Phaser — web (JavaScript)

- **`Container.sortableChildren = true`** + `zIndex` custom par sprite.
- **`sprite.anchor.set(0.5, 1.0)`** = BottomCenter natif.
- **Atlas JSON** format TexturePacker : `Texture.from()` avec metadata.
- **`renderer.resolution`** pour DPI scaling.

### Synthese

| Capacite | D2 | Godot | Bevy | MGE v2 cible |
|----------|----|----|------|-------------|
| Pivot configurable | BottomCenter fixe | `Sprite.offset` | `Anchor` enum | `SpritePivot` enum |
| Y-sort | CPU, screen_y pieds | `y_sort_enabled` | GPU z = -y | CPU, world_x+y |
| UI separee monde | Oui (couches) | `CanvasLayer` | `Camera.order` | Passes 100+ |
| Pixel-perfect | Non | `snap_2d_vertices` | `ScalingMode` | `ZoomMode::PixelPerfect` |
| Atlas metadata | Non (fichiers DC6) | `.atlasv` | JSON | `.atlas.json` |
| Font multiplateforme | Bitmaps embarques | Integree | `bevy_text` | `include_bytes!` |

---

## 1. Architecture du Pipeline

### 1.1 Vue d'ensemble : 5 passes logiques, 1 render pass GPU

Le pipeline MGE execute une seule `RenderPass` wgpu par frame, avec 5 sous-passes logiques ordonnees.

| PassId | LayerId | Nom | Contenu | Tri | Texture |
|--------|---------|-----|---------|-----|---------|
| 0 | 5 | `WorldBelow` | Tilemap sol + decor | Ordre grille (scan) | `grass_atlas` |
| 1 | 10 | `World` | Entites spritees | Y-sort deterministe | `character_atlas` |
| 2 | 15 | `WorldDebug` | Grille, path, hitbox, vision | Aucun | Aucune (quads couleurs) |
| 3 | 100 | `UI` | Panneaux, boutons, barres | `z_index` croissant | Aucune (quads couleurs) |
| 4 | 110 | `OverlayText` | Labels dev, metriques, log | Aucun | `text_atlas` |

**Flux de donnees normatif :**

```
Simulation Tick
      |
      v
[ ECS World ]
      |
      +---> iter2<Position2D, EntitySprite>
      |         |
      |         v
      |     Culling (distance^2 <= SPRITE_VISIBLE_TILES^2)
      |         |
      |         v
      |     Y-sort (world_x + world_y, x, entity_id)
      |
      +---> TileMap::visible_tiles(camera.visible_tile_bounds)
      |
      +---> DevState => debug overlays
      |
      v
[ CPU : remplissage buffers pre-alloues ]
      |
      +---> tile_instances:     Vec<InstanceRaw>          cap = MAX_TILE_INSTANCES
      +---> sprite_instances:   Vec<InstanceRaw>          cap = MAX_SPRITE_INSTANCES
      +---> world_overlay_inst: Vec<ColoredInstanceRaw>   cap = MAX_DEBUG_QUADS
      +---> gui_instances:      Vec<ColoredInstanceRaw>   cap = MAX_UI_QUADS
      +---> text_instances:     Vec<InstanceRaw>          cap = MAX_TEXT_INSTANCES
      |
      v
[ queue.write_buffer (upload GPU) ]
      |
      v
[ RenderPass unique : LoadOp::Clear(couleur_fond) ]
      |
      +---> Pass 0 : pipeline sprite.wgsl,          vertex_buffer tilemap
      +---> Pass 1 : pipeline sprite_instanced.wgsl, sprite_instances
      +---> Pass 2 : pipeline colored_quad.wgsl,    world_overlay_inst
      +---> Pass 3 : pipeline colored_quad.wgsl,    gui_instances
      +---> Pass 4 : pipeline sprite_instanced.wgsl, text_instances + text_bind_group
      |
      v
[ queue.submit + surface.present ]
```

### 1.2 Contraintes architecturales non-negociables

- `#![forbid(unsafe_code)]` sur toute la crate `mge-plugin-render-2d`.
- **Zero allocation dans le hot path** : tous les `Vec` sont pre-alloues avec `with_capacity` a l'initialisation du renderer ; le hot path n'appelle jamais `Vec::new()` ni `.collect()`.
- **Cross-platform** : aucun `#[cfg(target_os = "windows")]` sans equivalents Linux/macOS dans le chemin de rendu.
- **wgpu uniquement** comme backend GPU.
- **fontdue uniquement** pour la rasterisation CPU des polices.
- **Couches 0–99** : coordonnees Viewport Space (pixels, avec camera).
- **Couches 100+** : coordonnees Screen Space (pixels, sans camera, ancrees viewport). Un element UI ne doit jamais appeler `camera.project()`.

### 1.3 Integration plugin MGE

```rust
pub struct Render2DPlugin {
    pub config: Render2DConfig,
}

pub struct Render2DConfig {
    pub tile_half_w: f32,             // 32.0 (defaut Allumina, ratio 2:1)
    pub tile_half_h: f32,             // 16.0
    pub max_sprite_instances: usize,  // 2048
    pub max_tile_instances: usize,    // 16384
    pub zoom_mode: ZoomMode,          // PixelPerfect recommande
    pub font_size: f32,               // 14.0 (overlay dev)
    pub embedded_font: &'static [u8], // include_bytes! dans le jeu
}

impl Plugin for Render2DPlugin {
    fn name(&self) -> &str { "mge-plugin-render-2d" }

    fn dependencies(&self) -> &[&str] { &["mge-plugin-spatial"] }

    fn build(&self, engine: &mut Engine) {
        engine.register_component::<Sprite>();
        engine.register_component::<Camera2D>();
        engine.register_component::<RenderLayer>();
        engine.register_component::<SpritePivot>();
        engine.register_component::<SpriteAtlasRef>();

        // Phase 200 = apres toutes les phases de simulation
        engine.add_system(PhaseId(200), collect_render_data_system);
        engine.add_system(PhaseId(200), apply_layer_change_system);
    }
}
```

**Regle :** `WgpuRenderer` n'est PAS un systeme ECS. Il est appele par le Game Runtime apres `engine.tick()`, en lisant la resource `RenderQueue` remplie par `collect_render_data_system`. Ceci respecte le principe "simulation-first, tick != frame".

---

## 2. Systeme de Coordonnees

### 2.1 Les quatre espaces

**Espace Monde (World Space)**
- Unite : tiles (pas pixels).
- Origine : `(0.0, 0.0)` = coin superieur-gauche de la grille.
- Centre d'une tuile `(tx, ty)` : `WorldPos { x: tx as f32 + 0.5, y: ty as f32 + 0.5 }`.

**Espace Ecran Pre-Camera (Screen Raw)**

```rust
pub const TILE_HALF_W: f32 = 32.0;  // demi-largeur du losange en pixels
pub const TILE_HALF_H: f32 = 16.0;  // demi-hauteur du losange en pixels
// Ratio 2:1 = convention "2:1 isometric" standard industrie

/// Projection isometrique monde -> ecran (sans camera)
pub fn world_to_screen(world: WorldPos) -> ScreenPos {
    ScreenPos {
        x: (world.x - world.y) * TILE_HALF_W,
        y: (world.x + world.y) * TILE_HALF_H,
    }
}

/// Inverse : clic souris -> monde
pub fn screen_to_world(screen: ScreenPos) -> WorldPos {
    WorldPos {
        x: (screen.x / TILE_HALF_W + screen.y / TILE_HALF_H) / 2.0,
        y: (screen.y / TILE_HALF_H - screen.x / TILE_HALF_W) / 2.0,
    }
}
```

**Espace Viewport (avec camera)**

```rust
/// Application de la camera : pan + zoom
pub fn project(&self, world_x: f32, world_y: f32,
               viewport_w: f32, viewport_h: f32) -> (f32, f32) {
    let screen = world_to_screen(WorldPos::new(world_x, world_y));
    let center = world_to_screen(WorldPos::new(self.center_x, self.center_y));
    let dx = (screen.x - center.x) * self.zoom + viewport_w / 2.0;
    let dy = (screen.y - center.y) * self.zoom + viewport_h / 2.0;
    (dx, dy)
    // Resultat : pixels ecran, (0,0) = coin haut-gauche viewport
}
```

**Espace NDC (Normalized Device Coordinates — shader WGSL)**

```wgsl
// sprite_instanced.wgsl
// corner = coin haut-gauche du quad en pixels Viewport Space
let corner = in.position * in.instance_size + in.instance_pos;
out.clip_position = vec4<f32>(corner / view_size * 2.0 - 1.0, 0.0, 1.0);
out.clip_position.y = -out.clip_position.y;
// Inversion Y : pixel (0,0) haut-gauche -> NDC (+1,+1)
```

### 2.2 Contrat de pivot (correctif P1 canonique)

**Probleme identifie :** `camera.project(wx, wy)` retourne la position ecran des "pieds" de l'entite. Le shader interprete `screen_pos` comme le coin haut-gauche du quad. Sans correction pivot, le sprite s'affiche pieds au coin haut-gauche.

**Enum normatif :**

```rust
/// Pivot : point d'ancrage monde -> coordonnee ecran
/// Conforme a la convention Diablo II (BottomCenter par defaut)
#[derive(Debug, Clone, Copy, Default)]
pub enum SpritePivot {
    TopLeft,
    Center,
    #[default]
    BottomCenter,  // Defaut entites monde (pieds = point anchor)
}

impl SpritePivot {
    /// Retourne le coin haut-gauche du quad depuis la position pivot
    pub fn apply(&self, sx: f32, sy: f32, w: f32, h: f32) -> (f32, f32) {
        match self {
            SpritePivot::TopLeft      => (sx, sy),
            SpritePivot::Center       => (sx - w * 0.5, sy - h * 0.5),
            SpritePivot::BottomCenter => (sx - w * 0.5, sy - h),
        }
    }
}
```

**Application dans le renderer :**

```rust
// Pour tout sprite entite (Layer World)
// sx, sy = sortie camera.project() = position "pieds"
let (tl_x, tl_y) = sprite.pivot.apply(sx, sy, size_w, size_h);
instances.push(InstanceRaw {
    screen_pos: [tl_x, tl_y],
    size: [size_w, size_h],
    uv_rect: [uv_min.0, uv_min.1, uv_max.0, uv_max.1],
});
```

**Tableau des pivots par type d'entite :**

| Type | Pivot | Justification |
|------|-------|---------------|
| Personnage / PNJ | `BottomCenter` | "Pieds sur la tuile" — convention D2 |
| Tuile sol | `TopLeft` (implicite) | Pas de pivot, coordonnee deja correcte |
| Projectile | `Center` | Point d'impact = centre geometrique |
| Effet (explosion) | `Center` | Symetrique |
| Element UI | `TopLeft` | Coordonnee ecran directe |

### 2.3 Table de verification numerique

| Scenario | world_x | world_y | screen_x | screen_y | Notes |
|----------|---------|---------|----------|----------|-------|
| Tuile (0,0) centre | 0.5 | 0.5 | 0.0 | 16.0 | Origine iso |
| Tuile (1,0) centre | 1.5 | 0.5 | 32.0 | 32.0 | +1 sur X |
| Tuile (0,1) centre | 0.5 | 1.5 | -32.0 | 32.0 | +1 sur Y |
| Diagonale (2,2) | 2.5 | 2.5 | 0.0 | 80.0 | screen_x=0 sur diagonale |
| Camera (8,8) zoom=1 vp=1280x720 | 8.5 | 8.5 | 640.0 | 360.0 | Centre viewport |

---

## 3. Sprites et Atlas

### 3.1 Correctif P1 — Crop atlas silencieux

**Probleme :** `copy_sprite_to_atlas` force une copie dans un slot 100x100. Un sprite de 149x237 est rogne silencieusement.

**Correctif Phase A — validation stricte :**

```rust
#[derive(Debug)]
pub enum AtlasImportError {
    SizeMismatch {
        sprite_name: String,
        expected: (u32, u32),
        actual: (u32, u32),
    },
    Missing { path: String },
    AtlasFull { capacity: usize },
}

/// Import sprite avec validation — refuse le crop silencieux
pub fn import_sprite_validated(
    name: &str,
    rgba: &[u8],
    w: u32, h: u32,
    slot_w: u32, slot_h: u32,
) -> Result<AtlasSlot, AtlasImportError> {
    if w != slot_w || h != slot_h {
        return Err(AtlasImportError::SizeMismatch {
            sprite_name: name.to_string(),
            expected: (slot_w, slot_h),
            actual: (w, h),
        });
    }
    Ok(copy_to_atlas(rgba, w, h))
}
```

**Correctif Phase C — atlas variable avec metadata JSON :**

```rust
/// Metadata par sprite dans l'atlas
#[derive(Debug, Clone)]
pub struct SpriteAtlasEntry {
    pub atlas_rect: [u32; 4],    // x, y, w, h en pixels dans l'atlas
    pub size_px: (u32, u32),     // dimensions originales
    pub pivot_px: (u32, u32),    // pivot en pixels depuis haut-gauche
    pub layer_default: u32,
    pub z_bias: f32,
}

// Format fichier : assets/atlas/characters.atlas.json
// {
//   "version": 1,
//   "texture": "characters.png",
//   "entries": [
//     { "name": "player_idle", "rect": [0,0,64,128], "pivot": [32,128], "layer": 10 }
//   ]
// }
```

### 3.2 Structures GPU normatives

```rust
/// Quad unitaire partage (0..1) — 4 vertices, TriangleStrip
/// Meme buffer pour sprites et texte
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadVertex {
    position: [f32; 2],  // (0,0), (1,0), (0,1), (1,1)
    uv: [f32; 2],
}

/// Instance sprite/texte — upload CPU->GPU chaque frame
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    screen_pos: [f32; 2],  // Coin haut-gauche en pixels (apres pivot)
    size: [f32; 2],        // Dimensions en pixels (zoom applique)
    uv_rect: [f32; 4],     // [u_min, v_min, u_max, v_max]
}

/// Instance quad colore — overlays debug et UI
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColoredInstanceRaw {
    screen_pos: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],  // RGBA float [0..1]
}
```

### 3.3 Capacites pre-allouees normatives

```rust
// Pre-allocation a l'init, zero reallocation en hot path
pub const MAX_TILE_INSTANCES:   usize = 16_384;  // 128x128 tiles visibles max
pub const MAX_SPRITE_INSTANCES: usize = 2_048;   // entites monde
pub const MAX_DEBUG_QUADS:      usize = 1_024;   // overlays debug
pub const MAX_UI_QUADS:         usize = 256;     // elements UI
pub const MAX_TEXT_INSTANCES:   usize = 64;      // labels text
```

---

## 4. Systeme de Couches et Tri de Profondeur

### 4.1 LayerId normatives (alignees z-order-couches.md)

```rust
pub mod layers {
    pub const BACKGROUND:   u32 = 0;
    pub const WORLD_BELOW:  u32 = 5;   // Tilemap sol
    pub const WORLD:        u32 = 10;  // Entites Y-sorted
    pub const WORLD_ABOVE:  u32 = 15;  // Ponts, toits, ombres hautes
    pub const FOREGROUND:   u32 = 20;  // Brouillard, avant-plan
    pub const UI:           u32 = 100; // HUD, barres
    pub const OVERLAY:      u32 = 110; // Menus, debug text
}

// Sous-couches Allumina (dans les intervalles reserves)
pub mod allumina_layers {
    pub const SKY:          u32 = 0;
    pub const CLOUDS:       u32 = 1;
    pub const GROUND:       u32 = 5;
    pub const ENTITIES:     u32 = 10;
    pub const PROJECTILES:  u32 = 11;
    pub const EFFECTS:      u32 = 12;
    pub const ROOF:         u32 = 15;
    pub const FOG:          u32 = 20;
    pub const HUD:          u32 = 100;
    pub const MENUS:        u32 = 110;
    pub const CURSOR:       u32 = 111;
}
```

### 4.2 Algorithme Y-sort deterministe (correctif P1)

**Probleme :** L'iteration ECS `iter2::<Position2D, EntitySprite>()` produit un ordre arbitraire.

**Algorithme normatif a 3 criteres (reference : `main.rs:110-116`) :**

```rust
// Applique juste avant l'upload GPU, apres culling
// entity_sprites: Vec<(world_x, world_y, character_id, entity_id_bits: u64)>

entity_sprites.sort_by(|a, b| {
    // Cle 1 : profondeur iso — (x+y) croissant = dessine avant = derriere
    // screen_y = (x+y) * TILE_HALF_H donc trier par x+y = trier par screen_y
    let depth_a = a.0 + a.1;
    let depth_b = b.0 + b.1;

    depth_a.partial_cmp(&depth_b)
        .unwrap_or(Ordering::Equal)
        // Cle 2 : stabilite laterale (meme depth, positions X differentes)
        .then_with(|| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal))
        // Cle 3 : determinisme strict (meme frame = meme ordre = meme rendu)
        .then_with(|| a.3.cmp(&b.3))
});
```

**Pourquoi `world_x + world_y` et pas `screen_y` direct ?**

| Formule | Avantages | Inconvenients |
|---------|-----------|---------------|
| `world_x + world_y` | Cache-friendly, pas de calcul camera | Approximation si sprite tres large |
| `screen_y` (pieds) | Exact visuellement | Coupling camera, plus couteux |
| `-world_y` seul | Tres simple | Faux pour entites a meme Y mais X different |

Recommandation : `world_x + world_y` pour Phase A et B. `screen_y` des pieds reserve pour les cas pathologiques (sprites occupant plusieurs tiles).

**Illustration :**

```
Vue monde (dessus) :      Ordre de dessin correct :

    Y=0  Y=1  Y=2
X=0  A    .    .          1er : A (depth=1.0) — "au fond"
X=1  .    B    .          2e  : B (depth=3.0)
X=2  .    .    C          3e  : C (depth=5.0) — "au premier plan"

Entite C est visuellement devant B, B devant A.
```

### 4.3 Regle stricte separation monde/UI

```
REGLE :
  Couches 0-99  -> coordonnees Viewport Space (pixels, camera appliquee)
  Couches 100+  -> coordonnees Screen Space   (pixels, independants camera)

VIOLATION INTERDITE :
  Un element UI (bouton, barre de vie, panneau) NE DOIT PAS
  utiliser camera.project() pour calculer sa position.
  Il doit utiliser directement les coordonnees viewport (x_screen, y_screen).
```

---

## 5. Rendu GUI

### 5.1 Architecture de la passe GUI separee

```rust
/// Element GUI : position en coordonnees ecran, independant camera
pub struct GuiElement {
    pub anchor: GuiAnchor,
    pub offset: (f32, f32),     // offset depuis l'ancre en pixels
    pub size: (f32, f32),       // dimensions en pixels logiques
    pub color: [f32; 4],        // RGBA fond
    pub z_index: u32,           // ordre dans la passe UI
}

pub enum GuiAnchor {
    TopLeft,     // (0, 0) + offset
    TopRight,    // (viewport_w, 0) - size.x - offset.x
    BottomLeft,  // (0, viewport_h) - size.y + offset.y
    BottomRight, // (viewport_w - size.x - offset.x, viewport_h - size.y + offset.y)
    Center,      // (viewport_w/2 - size.w/2, viewport_h/2 - size.h/2) + offset
}

impl GuiAnchor {
    pub fn resolve(&self, offset: (f32,f32), size: (f32,f32),
                   vp_w: f32, vp_h: f32) -> (f32, f32) {
        match self {
            GuiAnchor::TopLeft     => (offset.0, offset.1),
            GuiAnchor::TopRight    => (vp_w - size.0 - offset.0, offset.1),
            GuiAnchor::BottomLeft  => (offset.0, vp_h - size.1 - offset.1),
            GuiAnchor::BottomRight => (vp_w - size.0 - offset.0, vp_h - size.1 - offset.1),
            GuiAnchor::Center      => (vp_w*0.5 - size.0*0.5 + offset.0,
                                       vp_h*0.5 - size.1*0.5 + offset.1),
        }
    }
}
```

### 5.2 DPI-awareness

```rust
/// Facteur DPI depuis winit
pub struct UiScaleConfig {
    /// Obtenu via window.scale_factor() (1.0 = 96dpi, 2.0 = 192dpi HiDPI)
    pub scale_factor: f64,
    /// Taille de police en points logiques (pas pixels physiques)
    pub base_font_size_pt: f32,
}

// Application :
// pixel_size = logical_size * scale_factor
// Bouton 100x30 logique sur HiDPI 2x -> 200x60 pixels physiques
```

---

## 6. Systeme de Polices

### 6.1 Architecture trois niveaux (correctif P1 portabilite)

**Probleme :** Le code initial charge via `WINDIR` uniquement (Windows-only). Sur Linux/macOS, `build_dev_text_atlas()` retourne `(vec![], 0, 0, vec![])` sans erreur visible.

```rust
/// Police embarquee (OBLIGATOIRE — toujours disponible)
/// A placer dans assets/fonts/ du jeu, charger via include_bytes!
const EMBEDDED_FONT: &[u8] =
    include_bytes!("../assets/fonts/LiberationSans-Regular.ttf");

/// Fallback systeme par plateforme
#[cfg(target_os = "windows")]
fn system_font_paths() -> Vec<PathBuf> {
    let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".into());
    vec![
        format!("{}\\Fonts\\arial.ttf", windir).into(),
        format!("{}\\Fonts\\segoeui.ttf", windir).into(),
    ]
}
#[cfg(target_os = "macos")]
fn system_font_paths() -> Vec<PathBuf> {
    vec![
        "/System/Library/Fonts/Supplemental/Arial.ttf".into(),
        "/Library/Fonts/Arial.ttf".into(),
    ]
}
#[cfg(all(unix, not(target_os = "macos")))]
fn system_font_paths() -> Vec<PathBuf> {
    vec![
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf".into(),
        "/usr/share/fonts/liberation/LiberationSans-Regular.ttf".into(),
        "/usr/share/fonts/truetype/freefont/FreeSans.ttf".into(),
    ]
}

/// Chargement avec ordre de priorite
pub fn load_font_robust() -> fontdue::Font {
    // 1. Embarquee : prioritaire
    if let Ok(f) = fontdue::Font::from_bytes(EMBEDDED_FONT, fontdue::FontSettings::default()) {
        return f;
    }
    // 2. Fallback OS
    for path in system_font_paths() {
        if let Ok(bytes) = std::fs::read(&path) {
            if let Ok(f) = fontdue::Font::from_bytes(&bytes, fontdue::FontSettings::default()) {
                return f;
            }
        }
    }
    // 3. Jamais silencieux : panic explicite plutot que rendu vide
    panic!("MGE: impossible de charger une police. Verifier assets/fonts/");
}
```

### 6.2 Polices recommandees — style Ragnarok Online

Ragnarok Online utilise **Gulim / GulimChe** (bitmap coreen) pour son HUD, avec un stroke noir et une taille de 10–12px. Le style est caracterise par : lisibilite maximale a petite taille, pixel grid strict, bords nets.

**Equivalents libres compatibles `fontdue` (TTF/OTF, OFL) :**

| Police | Style | Usage dans RO / Allumina | Licence | TTF | Note fontdue |
|--------|-------|--------------------------|---------|-----|-------------|
| **Silkscreen** | Pixel HUD strict | HUD principal, stats, menus | OFL | ~20 KB | Utiliser 8.0 ou 16.0 UNIQUEMENT |
| **Press Start 2P** | Titres retro pixel | Titres zones, menus principaux | OFL | ~60 KB | Multiples de 8.0 uniquement |
| **IM Fell English** | Serif medieval lisible | Dialogues, descriptions items | OFL | ~120 KB | 12.0–20.0 OK |
| **Cinzel** | Titling roman solennel | Headers zones, noms boss | OFL | ~80 KB | 16.0–32.0 recommande |
| **LiberationSans** (actuelle) | Sans-serif propre | Dev overlay, metriques | OFL | ~150 KB | 12.0–16.0 OK |
| **Dot Gothic 16** | Pixel asiatique proche RO | Si support CJK requis | OFL | ~4 MB | 16.0 uniquement |

**Recommandation principale Allumina :**

- **UI core / HUD / stats** : Silkscreen 8px ou 16px (pixel-perfect strict, style RO)
- **Dialogues / narratif** : IM Fell English 14px (lisible, medieval)
- **Dev overlay** : LiberationSans 14px (actuel, conserve)
- **Titres de zones** : Cinzel 24px (solennite)

**Regles d'integration fontdue :**

```rust
// REGLE : fontdue peut retourner metrics.width = 0 pour espaces,
// certains glyphes speciaux, retours a la ligne.
// Toujours valider avant placement.
let (metrics, bitmap) = font.rasterize(c, font_size);
if metrics.width == 0 || metrics.height == 0 {
    pen_x += metrics.advance_width;  // avancer sans placer de quad
    continue;
}

// REGLE pixel font : utiliser des multiples entiers de la taille native
// Silkscreen   : natif 8px → utiliser 8.0, 16.0, 24.0  (JAMAIS 14.0)
// Press Start  : natif 8px → idem
// LiberationSans: vectoriel → 12.0, 14.0, 16.0 tous valides

pub const FONT_SIZE_DEV_LABELS: f32 = 14.0;   // LiberationSans uniquement
pub const FONT_SIZE_HUD:        f32 = 8.0;    // Silkscreen pixel-perfect
pub const FONT_SIZE_DIALOGUE:   f32 = 14.0;   // IM Fell English
pub const FONT_SIZE_TITLE:      f32 = 24.0;   // Cinzel
```

### 6.3 Atlas de glyphes dynamique

```rust
pub struct TextAtlas {
    /// RGBA8 (fontdue alpha-only etendu : blanc * alpha)
    pub rgba: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub glyph_rects: Vec<GlyphRect>,
}

pub struct GlyphRect {
    pub character: char,
    pub font_size: f32,
    pub u_min: f32, pub v_min: f32,
    pub u_max: f32, pub v_max: f32,
    pub advance_width: f32,
    pub ymin: f32,  // offset baseline pour alignement correct
}

// Regles de layout :
// 1. atlas_w = 512 (power-of-two, minimum)
// 2. Placement gauche->droite, retour a la ligne si overflow largeur
// 3. Si overflow vertical : doubler atlas_h (512 -> 1024 -> 2048)
// 4. Charset minimal garanti : ASCII 32–126 + accents fr (a e i o u + majuscules)
```

---

## 7. Camera Pixel-Perfect

### 7.1 IsoCamera — specification normative

```rust
pub struct IsoCamera {
    pub center_x: f32,       // Centre viewport en tuiles monde
    pub center_y: f32,
    pub zoom: f32,           // Facteur zoom
    pub zoom_mode: ZoomMode,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ZoomMode {
    /// Zoom libre : n'importe quelle valeur float
    /// Accepte 1.4, 2.3, etc. — artefacts moire possibles avec FilterMode::Nearest
    #[default]
    Free,
    /// Zoom pixel-perfect : presets entiers uniquement (1x, 2x, 3x, 4x)
    /// Elimine le moire et le wobble sur sprites pixel art
    PixelPerfect,
}

pub const ZOOM_PRESETS_PP: &[f32] = &[1.0, 2.0, 3.0, 4.0];
```

### 7.2 Correctif P2 — Zoom entier

```rust
impl IsoCamera {
    pub fn set_zoom_clamped(&mut self, zoom: f32) {
        self.zoom = match self.zoom_mode {
            ZoomMode::Free => zoom.clamp(0.1, 8.0),
            ZoomMode::PixelPerfect => {
                // Arrondir au preset entier le plus proche dans [1, 4]
                zoom.round().clamp(1.0, 4.0)
            }
        };
    }
}
```

**Pourquoi zoom=1.4 est problematique :**

Avec `FilterMode::Nearest`, chaque pixel source est mappe sur exactement 1.4 pixel ecran. Certains pixels sources sont representes 1 fois, d'autres 2 fois, de facon irreguliere : le pattern produit un moire visible, notamment sur les bords des tuiles.

### 7.3 Correctif P2 — Snap camera sur grille pixel

```rust
impl IsoCamera {
    /// Snap la position camera pour que le centre viewport
    /// soit sur un pixel entier ecran. A appeler apres toute modification
    /// de center_x/center_y en mode PixelPerfect.
    pub fn snap_to_pixel_grid(&mut self) {
        if !matches!(self.zoom_mode, ZoomMode::PixelPerfect) {
            return;
        }
        // Arrondir center_x et center_y a la precision d'un pixel ecran.
        // 1 pixel ecran = 1 / (TILE_HALF_W * zoom) tuile en X
        let grid_x = 1.0 / (TILE_HALF_W * self.zoom);
        let grid_y = 1.0 / (TILE_HALF_H * self.zoom);
        self.center_x = (self.center_x / grid_x).round() * grid_x;
        self.center_y = (self.center_y / grid_y).round() * grid_y;
    }
}
```

### 7.4 Correctif P2 — Half-pixel snap WGSL (niveau 3)

Ajouter un uniform optionnel dans `sprite_instanced.wgsl` :

```wgsl
// Ajout dans sprite_instanced.wgsl
@group(0) @binding(2) var<uniform> view_size: vec2<f32>;
@group(0) @binding(3) var<uniform> pixel_snap: u32;  // 0 = off, 1 = on

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let corner = in.position * in.instance_size + in.instance_pos;

    // Pixel snap : arrondi au pixel entier avant conversion NDC
    // Elimine le sub-pixel wobble lors du scroll
    let snapped = select(corner, floor(corner + 0.5), pixel_snap != 0u);

    var out: VertexOutput;
    out.clip_position = vec4<f32>(snapped / view_size * 2.0 - 1.0, 0.0, 1.0);
    out.clip_position.y = -out.clip_position.y;
    out.uv = mix(in.instance_uv.xy, in.instance_uv.zw, in.uv);
    return out;
}
```

### 7.5 Culling de visibilite

```rust
/// Bornes tuiles visibles pour culling tilemap
pub fn visible_tile_bounds(
    &self,
    viewport_w: f32,
    viewport_h: f32,
    margin: i32,       // tuiles de marge pour sprites debordants (recommande: 2)
) -> (i32, i32, i32, i32) {
    let half_w = viewport_w / (2.0 * self.zoom * TILE_HALF_W);
    let half_h = viewport_h / (2.0 * self.zoom * TILE_HALF_H);
    let min_tx = (self.center_x - half_w - 1.0).floor() as i32 - margin;
    let max_tx = (self.center_x + half_w + 1.0).ceil()  as i32 + margin;
    let min_ty = (self.center_y - half_h - 1.0).floor() as i32 - margin;
    let max_ty = (self.center_y + half_h + 1.0).ceil()  as i32 + margin;
    (min_tx, min_ty, max_tx, max_ty)
}

// Culling entites (distance-carre en tuiles monde)
const SPRITE_VISIBLE_TILES: f32 = 10.0;
let max_dist_sq = SPRITE_VISIBLE_TILES * SPRITE_VISIBLE_TILES;

entity_sprites.retain(|&(wx, wy, _, _)| {
    let dx = wx - camera.center_x;
    let dy = wy - camera.center_y;
    dx * dx + dy * dy <= max_dist_sq
});
```

---

## 8. Performance et Budgets

### 8.1 Budgets cibles (desktop 1080p, scene dense dev)

| Metrique | Cible | Mesure | Outil |
|----------|-------|--------|-------|
| CPU prepare instances | <= 2.0 ms | `Instant::now()` autour de `queue.submit()` | `mge-profiler` |
| GPU frame 2D | <= 3.0 ms | `wgpu::Features::TIMESTAMP_QUERY` | wgpu timestamps |
| Y-sort 1000 entites | <= 0.1 ms | Bench isole | criterion |
| Allocations hot path | 0 alloc/frame | `cap > len` verifie avant boucle | custom hook |
| Tiles visibles max | 16 384 | Counter avant upload | renderer counter |
| Sprites visibles max | 2 048 | Counter avant upload | renderer counter |

### 8.2 Zero-allocation hot path — regles obligatoires

```rust
// BON : pre-allocation a l'init, clear() dans le hot path
pub struct WgpuRenderer {
    tile_instances:    Vec<InstanceRaw>,         // avec_capacity(MAX_TILE_INSTANCES)
    sprite_instances:  Vec<InstanceRaw>,         // avec_capacity(MAX_SPRITE_INSTANCES)
    world_overlays:    Vec<ColoredInstanceRaw>,  // avec_capacity(MAX_DEBUG_QUADS)
    gui_instances:     Vec<ColoredInstanceRaw>,  // avec_capacity(MAX_UI_QUADS)
    text_instances:    Vec<InstanceRaw>,         // avec_capacity(MAX_TEXT_INSTANCES)
}

fn prepare_frame(&mut self, ...) {
    // clear() ne desalloue pas la capacite
    self.tile_instances.clear();
    self.sprite_instances.clear();
    self.world_overlays.clear();
    self.gui_instances.clear();
    self.text_instances.clear();
    // puis push() sans depasser la capacite
}

// INTERDIT dans le hot path :
// let v: Vec<_> = iter.map(...).collect();   // allocation implicite
// Vec::new() + push()                        // peut reallouer
// let v = vec![x, y, z];                    // allocation implicite
```

### 8.3 RenderStats — observabilite

```rust
#[derive(Debug, Default, Clone)]
pub struct RenderStats {
    pub tiles_visible:    u32,
    pub sprites_rendered: u32,
    pub debug_quads:      u32,
    pub ui_quads:         u32,
    pub text_instances:   u32,
    pub cpu_prepare_ms:   f32,
    pub gpu_frame_ms:     Option<f32>,  // None si TIMESTAMP_QUERY indisponible
}
// Expose via engine.resource::<RenderStats>() en lecture seule
// Mise a jour par le renderer apres chaque frame
```

---

## 9. API Reference — mge-plugin-render-2d

### 9.1 Composants

```rust
use mge_ecs::Component;

/// Sprite : reference texture + dimensions + pivot
#[derive(Debug, Clone)]
pub struct Sprite {
    pub texture_id: u32,
    pub width: f32,
    pub height: f32,
    pub pivot: SpritePivot,       // defaut BottomCenter
    pub uv_rect: [f32; 4],        // [u_min, v_min, u_max, v_max]
}

/// Camera2D : configuration camera isometrique
#[derive(Debug, Clone)]
pub struct Camera2D {
    pub zoom: f32,
    pub zoom_mode: ZoomMode,
    pub offset_x: f32,
    pub offset_y: f32,
    pub pixel_snap: bool,
}

/// RenderLayer : couche de rendu + z_bias optionnel
#[derive(Debug, Clone)]
pub struct RenderLayer {
    pub layer: u32,
    pub z_bias: f32,  // override Y-sort au sein d'une couche
}

/// SpriteAtlasRef : reference a une entree d'atlas
#[derive(Debug, Clone)]
pub struct SpriteAtlasRef {
    pub atlas_id: u32,
    pub entry_index: u32,  // index dans SpriteAtlas::entries
}

impl Component for Sprite {}
impl Component for Camera2D {}
impl Component for RenderLayer {}
impl Component for SpritePivot {}
impl Component for SpriteAtlasRef {}
```

### 9.2 Evenements

```rust
#[derive(Debug, Clone)]
pub struct SetRenderLayerRequest { pub entity: EntityId, pub new_layer: u32 }

#[derive(Debug, Clone)]
pub struct SpriteLoadError { pub sprite_name: String, pub reason: String }

#[derive(Debug, Clone)]
pub struct RenderFrameComplete { pub stats: RenderStats }

impl Event for SetRenderLayerRequest {}
impl Event for SpriteLoadError {}
impl Event for RenderFrameComplete {}
```

### 9.3 Systemes

```rust
/// Phase 200 — collecte et tri des donnees de rendu depuis l'ECS
fn collect_render_data_system(world: &World, ctx: &mut Context) {
    let render_queue = ctx.resource_mut::<RenderQueue>();
    render_queue.sprite_instances.clear();

    let camera = world.get_unique::<Camera2D>();

    // 1. Collecter entites avec Sprite + Position2D
    for (pos, sprite, layer) in world.iter3::<Position2D, Sprite, RenderLayer>() {
        if layer.layer != layers::WORLD { continue; }

        // 2. Culling
        let dx = pos.x - camera.offset_x;
        let dy = pos.y - camera.offset_y;
        if dx*dx + dy*dy > SPRITE_VISIBLE_TILES * SPRITE_VISIBLE_TILES { continue; }

        render_queue.sprite_instances.push(SpriteInstance {
            world_x: pos.x, world_y: pos.y,
            sprite: sprite.clone(),
        });
    }

    // 3. Y-sort deterministe
    render_queue.sprite_instances.sort_by(|a, b| {
        (a.world_x + a.world_y).partial_cmp(&(b.world_x + b.world_y))
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.world_x.partial_cmp(&b.world_x).unwrap_or(Ordering::Equal))
    });
}

/// Phase 200 — applique les requetes de changement de couche
fn apply_layer_change_system(world: &mut World, ctx: &mut Context) {
    for event in ctx.events().iter::<SetRenderLayerRequest>() {
        if let Some(layer) = world.get_mut::<RenderLayer>(event.entity) {
            layer.layer = event.new_layer;
        }
    }
}
```

---

## 10. Plan de Migration en 3 Phases

### Phase A — Stabilisation critique (P1) — 2 jours

| Tache | Fichier | Detail |
|-------|---------|--------|
| A1 : Pivot uniforme | `renderer.rs` | Appliquer `SpritePivot::BottomCenter.apply()` dans `draw_sprites` |
| A2 : Y-sort entities | `main.rs` | Confirmer tri 3 criteres present et actif |
| A3 : Font embarquee | `dev_text.rs` | Verifier `include_bytes!` + tester sur Linux/macOS CI |
| A4 : Validation atlas | `renderer.rs` | `import_sprite_validated()` — erreur explicite si taille != slot |

**Critere de sortie :** Rendu correct et portable. Zero crop implicite. Sprites de toute taille generent une erreur a l'import si hors-slot.

### Phase B — Qualite visuelle (P2) — 1.5 jours

| Tache | Fichier | Detail |
|-------|---------|--------|
| B1 : Zoom entier | `isometric.rs` | `set_zoom_clamped()` avec mode `PixelPerfect` |
| B2 : Camera snap | `isometric.rs` | `snap_to_pixel_grid()` appele apres pan camera |
| B3 : WGSL pixel snap | `sprite_instanced.wgsl` | Uniform `pixel_snap` + `floor(corner + 0.5)` |
| B4 : DPI GUI | `renderer.rs` | `window.scale_factor()` applique aux GUI elements |

**Critere de sortie :** Zero wobble tile en scroll lent en mode `PixelPerfect`. Sprites nets a zoom 1x et 2x.

### Phase C — Industrialisation — 3 jours

| Tache | Fichier | Detail |
|-------|---------|--------|
| C1 : Atlas metadata JSON | `atlas_loader.rs` (nouveau) | Parser `.atlas.json`, `SpriteAtlasEntry` |
| C2 : Atlas variable | `renderer.rs` | Remplacer atlas 300x100 fixe par atlas dynamique |
| C3 : RenderStats resource | `renderer.rs` | Expose stats via resource MGE |
| C4 : Benchmark | `benches/render_bench.rs` | criterion : 1000 entites, Y-sort, GPU |
| C5 : CI golden images | `.github/workflows/` | Comparaison images 3 OS, tolerance zero pixel |

**Critere de sortie :** Pipeline production-ready. Tests visuels CI verts sur Windows/Linux/macOS.

---

## 11. Tests

### 11.1 Tests unitaires (sans wgpu — logique pure)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    // Test 1 : Pivot BottomCenter
    #[test]
    fn test_pivot_bottom_center() {
        let (x, y) = SpritePivot::BottomCenter.apply(100.0, 200.0, 64.0, 128.0);
        assert_eq!(x, 68.0);   // 100 - 32 = 68
        assert_eq!(y, 72.0);   // 200 - 128 = 72
    }

    // Test 2 : Pivot TopLeft inchange
    #[test]
    fn test_pivot_top_left() {
        let (x, y) = SpritePivot::TopLeft.apply(100.0, 200.0, 64.0, 64.0);
        assert_eq!(x, 100.0);
        assert_eq!(y, 200.0);
    }

    // Test 3 : Y-sort ordre correct
    #[test]
    fn test_ysort_order() {
        let mut sprites: Vec<(f32, f32, u8, u64)> = vec![
            (5.0, 3.0, 0, 1),  // depth = 8.0
            (1.0, 1.0, 0, 2),  // depth = 2.0
            (8.0, 7.0, 0, 3),  // depth = 15.0
        ];
        sprites.sort_by(|a, b|
            (a.0+a.1).partial_cmp(&(b.0+b.1)).unwrap_or(Ordering::Equal)
                .then_with(|| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal))
                .then_with(|| a.3.cmp(&b.3))
        );
        assert_eq!(sprites[0].3, 2);  // depth 2.0 en premier
        assert_eq!(sprites[1].3, 1);  // depth 8.0
        assert_eq!(sprites[2].3, 3);  // depth 15.0 en dernier
    }

    // Test 4 : Y-sort deterministe (tie-breaking)
    #[test]
    fn test_ysort_deterministic_tie() {
        let mut sprites: Vec<(f32, f32, u8, u64)> = vec![
            (2.0, 2.0, 0, 10),
            (2.0, 2.0, 0, 5),
            (2.0, 2.0, 0, 15),
        ];
        sprites.sort_by(|a, b|
            (a.0+a.1).partial_cmp(&(b.0+b.1)).unwrap_or(Ordering::Equal)
                .then_with(|| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal))
                .then_with(|| a.3.cmp(&b.3))
        );
        assert_eq!(sprites[0].3, 5);
        assert_eq!(sprites[1].3, 10);
        assert_eq!(sprites[2].3, 15);
    }

    // Test 5 : Projection isometrique — origine
    #[test]
    fn test_iso_projection_origin() {
        let s = world_to_screen(WorldPos::new(0.5, 0.5));
        assert!((s.x - 0.0).abs() < f32::EPSILON);
        assert!((s.y - 16.0).abs() < f32::EPSILON);
    }

    // Test 6 : Aller-retour projection
    #[test]
    fn test_iso_roundtrip() {
        let original = WorldPos::new(3.7, 5.2);
        let recovered = screen_to_world(world_to_screen(original));
        assert!((recovered.x - original.x).abs() < 1e-5);
        assert!((recovered.y - original.y).abs() < 1e-5);
    }

    // Test 7 : Atlas import — refus crop implicite
    #[test]
    fn test_atlas_import_size_mismatch() {
        let result = import_sprite_validated(
            "Test_joueur", &vec![0u8; 149*237*4], 149, 237, 100, 100
        );
        assert!(matches!(result, Err(AtlasImportError::SizeMismatch { .. })));
    }

    // Test 8 : Zoom pixel-perfect — presets entiers
    #[test]
    fn test_zoom_pixel_perfect_clamp() {
        let mut cam = IsoCamera { zoom: 1.0, zoom_mode: ZoomMode::PixelPerfect,
                                   center_x: 0.0, center_y: 0.0 };
        cam.set_zoom_clamped(1.4);
        assert_eq!(cam.zoom, 1.0);
        cam.set_zoom_clamped(1.7);
        assert_eq!(cam.zoom, 2.0);
        cam.set_zoom_clamped(3.8);
        assert_eq!(cam.zoom, 4.0);
    }

    // Test 9 : visible_tile_bounds contient la tuile centre
    #[test]
    fn test_visible_bounds_contains_center() {
        let cam = IsoCamera { center_x: 10.0, center_y: 10.0,
                               zoom: 1.0, zoom_mode: ZoomMode::Free };
        let (min_tx, min_ty, max_tx, max_ty) = cam.visible_tile_bounds(1280.0, 720.0, 0);
        assert!(min_tx <= 10 && 10 <= max_tx);
        assert!(min_ty <= 10 && 10 <= max_ty);
    }
}
```

### 11.2 Scenes de reference — golden images CI

| Scene | Description | Critere |
|-------|-------------|---------|
| `scene_01_single_sprite` | 1 entite centree, zoom=1, camera fixe | Pieds sur tuile centre, pivot correct |
| `scene_02_ysort_3entities` | 3 entites a depths differentes | Entite la plus "au sud" visuellement devant |
| `scene_03_tilemap_8x8` | Tilemap 8x8, zoom=1 | Grille iso sans wobble, tuiles alignees |
| `scene_04_devtext_labels` | Panneau dev ouvert | Labels "Dev", "Grille" visibles |
| `scene_05_pixelperfect_2x` | Scene 03 a zoom=2, mode PixelPerfect | Tiles exactement doubles, zero moire |

**Configuration CI :**

```yaml
# .github/workflows/render_tests.yml
strategy:
  matrix:
    os: [ubuntu-latest, windows-latest, macos-latest]
steps:
  - run: cargo test --package mge-plugin-render-2d
  - run: cargo run --example render_golden_scenes
  - run: python scripts/compare_golden_images.py
    env:
      TOLERANCE_PIXELS: 0  # rendu deterministe, zero tolerance
```

---

## 12. Definition of Done

### Phase A — obligatoire pour merge

- [ ] `draw_sprites` utilise `SpritePivot::BottomCenter.apply()` — rendu identique a `draw_tilemap`
- [ ] Y-sort avec 3 criteres confirme actif avant upload GPU
- [ ] `EMBEDDED_FONT` charge sur Windows, Linux, macOS (CI valide)
- [ ] `import_sprite_validated()` retourne `SizeMismatch` si taille != slot
- [ ] Zero crop implicite subsiste dans le codebase

### Phase B — obligatoire pour release alpha

- [ ] `ZoomMode::PixelPerfect` force uniquement 1.0 / 2.0 / 3.0 / 4.0
- [ ] `snap_to_pixel_grid()` appele apres toute modification camera en mode PP
- [ ] Zero wobble tile en scroll lent (validation visuelle + golden image)
- [ ] `pixel_snap` uniform present dans `sprite_instanced.wgsl`

### Phase C — obligatoire pour release stable

- [ ] `.atlas.json` metadata parsee, `SpriteAtlasEntry` utilisee
- [ ] `RenderStats` expose via resource MGE apres chaque frame
- [ ] Y-sort 1000 entites <= 0.1 ms (criterion bench)
- [ ] CPU prepare <= 2 ms (scene dense reference)
- [ ] Tests visuels CI verts sur Windows / Linux / macOS, tolerance zero pixel
- [ ] `#![forbid(unsafe_code)]` enforce dans `mge-plugin-render-2d` (clippy)

---

## 13. References croisees

| Document | Chemin | Relation |
|----------|--------|----------|
| Audit rendu 2D iso | `docs/Miyukini_Game_Engine/audit-rendu-2d-isometrique.md` | Analyse detaillee des bugs P1/P2 |
| Z-order couches | `docs/Miyukini_Game_Engine/legacy/points/01-affichage-rendu/z-order-couches.md` | Specification LayerId |
| Gestion sprites | `docs/Miyukini_Game_Engine/legacy/points/01-affichage-rendu/gestion-sprites.md` | Pivot, atlas, SpriteInstance |
| Performance Philosophy | `docs/Miyukini_Game_Engine/MGE - Performance Philosophy.md` | Zero alloc, SoA, batch |
| Core Specification | `docs/Miyukini_Game_Engine/MGE - Core Specification Technique.md` | Engine, tick, World |
| Plugin Contract | `docs/Miyukini_Game_Engine/MGE - Plugin Contract.md` | trait Plugin, build() |
| Pack Architecture | `docs/Miyukini_Game_Engine/MGE - Pack Architecture.md` | Position de mge-plugin-render-2d |
| IsoCamera | `mge/examples/allumina_prototype/src/isometric.rs` | Implementation reference |
| WgpuRenderer | `mge/examples/allumina_prototype/src/renderer.rs` | Implementation reference |
| Font system | `mge/examples/allumina_prototype/src/dev_text.rs` | Reference actuelle |
| Shaders | `mge/examples/allumina_prototype/assets/shaders/` | WGSL reference |
| Plugin scaffolding | `mge/crates/mge-plugin-render-2d/src/` | Cible d'implementation |
| Allumina Audit MVP | `docs/services/Allumina/Allumina - Audit Technique MVP 2026-02.md` | Contexte conformite |

---

**Document** : MGE — Rendu 2D Isometrique Robuste
**Version** : 2.0
**Date** : 2026-02-23
**Statut** : Specification normative — Production Target
**Auteur** : Studio Miyukini — Equipe Moteur
