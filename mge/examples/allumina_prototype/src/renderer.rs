//! @id allumina.prototype.renderer
//! @role render
//! @layer application
//! @domain allumina
//! @do wgpu_isometric_tilemap_and_sprite_rendering
//!
//! WgpuRenderer v2.0 — Rendu isométrique conforme au spec MGE Render v2.0.
//!
//! Corrections appliquées par rapport à la version précédente :
//! - Atlas variable-size : chaque sprite garde ses dimensions naturelles
//! - Pivot BottomCenter exact : (sx - sprite_w/2, sy - sprite_h) avec dimensions réelles
//! - Police embarquée via include_bytes! (cross-platform)
//! - Buffers CPU pré-alloués (pas de Vec::new() dans le hot-path)
//! - Séparation couches : tiles/sprites = viewport, texte/ui = screen
//! - Y-sort 3 critères déterministe : (wx+wy, wx, entity_id bits)

use bytemuck::{Pod, Zeroable};
use image::GenericImageView;
use std::path::Path;
use wgpu::util::DeviceExt;

use crate::dev_overlay::DevState;
use crate::gui::{
    GuiState, HudData,
    BTN_COMPETENCES_X, BTN_H, BTN_INVENTAIRE_X, BTN_PERSONNAGE_X, BTN_W, BTN_Y,
    CHAR_WIN_W, CHAR_WIN_H, SKILLS_WIN_W, SKILLS_WIN_H, INV_WIN_W, INV_WIN_H,
    WIN_TITLE_H, WIN_CLOSE_W,
};
use crate::isometric::{IsoCamera, TILE_HALF_H, TILE_HALF_W};
use crate::tilemap::TileMap;

// ── Capacités pré-allouées ────────────────────────────────────────────────────
const MAX_TILE_INSTANCES: usize = 16_384;
const MAX_SPRITE_INSTANCES: usize = 2_048;
const MAX_DEBUG_QUADS: usize = 2_048;
const MAX_TEXT_CHARS: usize = 8_192;
const MAX_DEBUG_QUADS_GUI: usize = 2_048; // capacité quads GUI

// ── Police embarquée ─────────────────────────────────────────────────────────
static FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/LiberationSans-Regular.ttf");
const FONT_SIZE_PX: f32 = 14.0;
// Atlas 2048px pour loger tous les glyphes ASCII (95 × ~10px avg = ~950px, marge confortable)
const TEXT_ATLAS_W: u32 = 2048;
const TEXT_ATLAS_H: u32 = 64;

// ── GPU structs ───────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct TileVertex {
    position: [f32; 2],
    uv: [f32; 2],
}

/// Instance pour sprite_instanced.wgsl (tile ou sprite, selon pipeline)
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct InstanceRaw {
    /// Position top-left en pixels écran
    screen_pos: [f32; 2],
    /// Taille en pixels (avec zoom)
    size: [f32; 2],
    /// Rect UV dans l'atlas [u_min, v_min, u_max, v_max]
    uv_rect: [f32; 4],
}

/// Instance pour colored_quad.wgsl (debug, ui)
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct ColoredInstanceRaw {
    screen_pos: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],
}

// ── Métadonnées par sprite ────────────────────────────────────────────────────

/// Entrée dans l'atlas de personnages : dimensions naturelles + UV
#[derive(Debug, Clone, Copy)]
struct CharEntry {
    /// Largeur naturelle du sprite en pixels
    sprite_w: f32,
    /// Hauteur naturelle du sprite en pixels
    sprite_h: f32,
    u_min: f32,
    v_min: f32,
    u_max: f32,
    v_max: f32,
}

// ── Erreurs ───────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum RendererInitError {
    SurfaceConfig,
    RequestDevice(wgpu::RequestDeviceError),
    Io(std::io::Error),
    Image(image::ImageError),
    NoFormat,
}

impl std::fmt::Display for RendererInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for RendererInitError {}

// ── Renderer principal ────────────────────────────────────────────────────────

pub struct WgpuRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    viewport_size: (f32, f32),

    // ── Tile pipeline (sprite.wgsl, vertex mesh par frame) ──
    tile_pipeline: wgpu::RenderPipeline,
    tile_texture: wgpu::Texture,
    tile_vertex_buf: wgpu::Buffer,
    tile_uniform_buf: wgpu::Buffer,
    tile_bind_group: wgpu::BindGroup,
    tile_atlas_w: u32,
    tile_atlas_h: u32,
    tile_num_graphic_ids: u32,
    /// Nombre de colonnes de tiles dans l'atlas grid (ex. 48 pour 6144/128)
    tile_atlas_cols: u32,
    /// Nombre de lignes de tiles dans l'atlas grid (ex. 2 pour 128/64)
    tile_atlas_rows: u32,

    // ── Sprite pipeline (sprite_instanced.wgsl) ──
    sprite_pipeline: wgpu::RenderPipeline,
    sprite_quad_buf: wgpu::Buffer,
    sprite_instance_buf: wgpu::Buffer,
    sprite_bind_group: wgpu::BindGroup,
    char_entries: Vec<CharEntry>,
    char_atlas_w: u32,
    char_atlas_h: u32,

    // ── Colored quad pipeline (colored_quad.wgsl) ──
    quad_pipeline: wgpu::RenderPipeline,
    quad_instance_buf: wgpu::Buffer,
    quad_bind_group: wgpu::BindGroup,

    // ── Text pipeline (réutilise sprite_instanced.wgsl, texture text atlas) ──
    text_pipeline: wgpu::RenderPipeline,
    text_instance_buf: wgpu::Buffer,
    text_bind_group: wgpu::BindGroup,
    glyph_cache: GlyphCache,

    // ── Shadow pipeline (réutilise colored_quad.wgsl) — ombres sous entités ──
    // Rendu AVANT les sprites pour que les ombres soient sous les personnages
    shadow_instance_buf: wgpu::Buffer,

    // ── Buffers CPU pré-alloués ──
    tile_vertices: Vec<TileVertex>,
    sprite_instances: Vec<InstanceRaw>,
    shadow_quads: Vec<ColoredInstanceRaw>,
    debug_quads: Vec<ColoredInstanceRaw>,
    text_instances: Vec<InstanceRaw>,
}

// ── Glyph cache ───────────────────────────────────────────────────────────────

struct GlyphInfo {
    u_min: f32,
    v_min: f32,
    u_max: f32,
    v_max: f32,
    width: f32,
    height: f32,
    advance: f32,
    offset_y: f32,
}

struct GlyphCache {
    font: fontdue::Font,
    atlas: Vec<u8>,
    glyphs: std::collections::HashMap<char, GlyphInfo>,
    cursor_x: u32,
}

impl GlyphCache {
    fn new() -> Self {
        let font = fontdue::Font::from_bytes(FONT_BYTES, fontdue::FontSettings::default())
            .expect("Chargement police embarquée");
        // Atlas RGBA : (blanc, blanc, blanc, alpha_glyph) → texte blanc sur fond transparent
        let atlas = vec![0u8; (TEXT_ATLAS_W * TEXT_ATLAS_H * 4) as usize];
        Self {
            font,
            atlas,
            glyphs: std::collections::HashMap::new(),
            cursor_x: 0,
        }
    }

    fn ensure_glyph(&mut self, ch: char) {
        if self.glyphs.contains_key(&ch) {
            return;
        }
        let (metrics, bitmap) = self.font.rasterize(ch, FONT_SIZE_PX);
        if metrics.width == 0 || metrics.height == 0 {
            // Espace ou glyph vide : stocker avec advance seulement
            self.glyphs.insert(ch, GlyphInfo {
                u_min: 0.0, v_min: 0.0, u_max: 0.0, v_max: 0.0,
                width: 0.0, height: 0.0,
                advance: metrics.advance_width,
                offset_y: 0.0,
            });
            return;
        }
        let gw = metrics.width as u32;
        let gh = metrics.height as u32;
        if self.cursor_x + gw > TEXT_ATLAS_W {
            return; // overflow atlas — on ignore le glyph
        }
        // Copier bitmap dans l'atlas RGBA : pixel = (255, 255, 255, coverage)
        // → rendu blanc avec transparence selon coverage fontdue
        for dy in 0..gh.min(TEXT_ATLAS_H) {
            for dx in 0..gw {
                let src = (dy * gw + dx) as usize;
                let dst_px = (dy * TEXT_ATLAS_W + self.cursor_x + dx) as usize;
                let dst_byte = dst_px * 4;
                if src < bitmap.len() && dst_byte + 3 < self.atlas.len() {
                    self.atlas[dst_byte]     = 255; // R
                    self.atlas[dst_byte + 1] = 255; // G
                    self.atlas[dst_byte + 2] = 255; // B
                    self.atlas[dst_byte + 3] = bitmap[src]; // A = coverage
                }
            }
        }
        let u_min = self.cursor_x as f32 / TEXT_ATLAS_W as f32;
        let u_max = (self.cursor_x + gw) as f32 / TEXT_ATLAS_W as f32;
        let v_min = 0.0f32;
        let v_max = gh as f32 / TEXT_ATLAS_H as f32;
        self.glyphs.insert(ch, GlyphInfo {
            u_min, v_min, u_max, v_max,
            width: gw as f32,
            height: gh as f32,
            advance: metrics.advance_width,
            offset_y: metrics.ymin as f32,
        });
        self.cursor_x += gw + 1;
    }
}

// ── Construction de l'atlas herbe ────────────────────────────────────────────

/// Charge les 8 tuiles herbe (Grass_a .. Grass_h) et les pack horizontalement.
/// Retourne (rgba, atlas_w, atlas_h, tile_w) — tile_w = largeur naturelle d'une tuile.
fn build_grass_atlas(dev_assets: &Path) -> (Vec<u8>, u32, u32, u32) {
    let names = ["Grass_a", "Grass_b", "Grass_c", "Grass_d",
                  "Grass_e", "Grass_f", "Grass_g", "Grass_h"];
    let mut tiles: Vec<(Vec<u8>, u32, u32)> = Vec::new();

    for name in &names {
        let p = dev_assets.join(format!("{}.png", name));
        match image::open(&p) {
            Ok(img) => {
                let (w, h) = img.dimensions();
                tiles.push((img.to_rgba8().into_raw(), w, h));
            }
            Err(e) => {
                log::warn!("Tuile herbe manquante {:?}: {}", p, e);
                // Fallback : tuile vert 128×64 (ratio 2:1 isométrique)
                let (fw, fh) = (128u32, 64u32);
                let mut rgba = vec![0u8; (fw * fh * 4) as usize];
                for i in (0..rgba.len()).step_by(4) {
                    rgba[i] = 60; rgba[i+1] = 140; rgba[i+2] = 60; rgba[i+3] = 255;
                }
                tiles.push((rgba, fw, fh));
            }
        }
    }

    // Normaliser toutes les tuiles à la même taille (première tuile fait référence)
    let (ref_w, ref_h) = (tiles[0].1, tiles[0].2);
    let total_w = ref_w * tiles.len() as u32;
    let total_h = ref_h;
    let mut atlas = vec![0u8; (total_w * total_h * 4) as usize];

    for (i, (src, w, h)) in tiles.iter().enumerate() {
        let ox = i as u32 * ref_w;
        let copy_w = (*w).min(ref_w);
        let copy_h = (*h).min(ref_h);
        for row in 0..copy_h {
            let src_start = (row * w * 4) as usize;
            let dst_start = ((row * total_w + ox) * 4) as usize;
            let len = (copy_w * 4) as usize;
            if src_start + len <= src.len() && dst_start + len <= atlas.len() {
                atlas[dst_start..dst_start + len].copy_from_slice(&src[src_start..src_start + len]);
            }
        }
    }

    (atlas, total_w, total_h, ref_w)
}

/// Largeur normalisée de tous les frames Knight (Walk = la plus large = 64px).
const KNIGHT_FRAME_W: u32 = 64;

/// Centre un frame `src` (src_w × src_h) dans une image plus large `target_w × src_h`.
/// Les colonnes non remplies restent transparentes (alpha=0).
fn pad_frame_to_width(src: &[u8], src_w: u32, src_h: u32, target_w: u32) -> Vec<u8> {
    if src_w >= target_w {
        return src.to_vec();
    }
    let mut dst = vec![0u8; (target_w * src_h * 4) as usize];
    let x_offset = (target_w - src_w) / 2;
    for row in 0..src_h {
        let src_start = (row * src_w * 4) as usize;
        let dst_start = ((row * target_w + x_offset) * 4) as usize;
        let len = (src_w * 4) as usize;
        if src_start + len <= src.len() && dst_start + len <= dst.len() {
            dst[dst_start..dst_start + len].copy_from_slice(&src[src_start..src_start + len]);
        }
    }
    dst
}

/// Extrait les frames d'une spritesheet horizontale (bande unique).
///
/// `frame_w` / `frame_h` : dimensions d'un seul frame.
/// `frame_count` : nombre de frames à extraire.
/// Retourne une liste de (rgba, frame_w, frame_h).
fn extract_frames(
    sheet_path: &Path,
    frame_w: u32,
    frame_h: u32,
    frame_count: u32,
) -> Vec<(Vec<u8>, u32, u32)> {
    let sheet_img = match image::open(sheet_path) {
        Ok(img) => img.to_rgba8(),
        Err(e) => {
            log::warn!("Spritesheet manquante {:?}: {}", sheet_path, e);
            let px = (frame_w * frame_h * 4) as usize;
            let mut fb = vec![0u8; px];
            for i in (0..fb.len()).step_by(4) {
                fb[i] = 255; fb[i + 1] = 0; fb[i + 2] = 255; fb[i + 3] = 255;
            }
            return (0..frame_count).map(|_| (fb.clone(), frame_w, frame_h)).collect();
        }
    };
    let sheet_bytes = sheet_img.as_raw();
    let (sheet_w, sheet_h) = sheet_img.dimensions();
    let fh = frame_h.min(sheet_h);

    (0..frame_count)
        .map(|fi| {
            let x0 = fi * frame_w;
            let mut frame = vec![0u8; (frame_w * fh * 4) as usize];
            for row in 0..fh {
                let src = ((row * sheet_w + x0) * 4) as usize;
                let dst = (row * frame_w * 4) as usize;
                let len = (frame_w * 4) as usize;
                if src + len <= sheet_bytes.len() {
                    frame[dst..dst + len].copy_from_slice(&sheet_bytes[src..src + len]);
                }
            }
            (frame, frame_w, fh)
        })
        .collect()
}

/// Construit l'atlas de personnages à partir des spritesheets Knight_1 et des sprites mobs.
///
/// Layout atlas :
///   0–9   Knight Idle  (10 × 29→64×86)
///   10–17 Knight Walk  (8 × 64×64, knight_walk.png)
///   18–27 Knight Dead  (10 × 48→64×86)
///   28–32 Mobs statiques (5 types, Dev_assets)
///   33–36 NPC village   (4 types × 64×64 : forgeron, recruteur, serveuse, tavernier)
///   37    Maison        (Medieval_Building_06.png, redimensionné ~128×96)
///   38–39 Arbres        (arbre_a.png, arbre_b.png, redimensionnés ~64×68)
fn build_char_atlas(dev_assets: &Path) -> (Vec<u8>, u32, u32, Vec<CharEntry>) {
    let knight_dir = dev_assets.parent().unwrap_or(dev_assets).join("Knight_1");

    let mut images: Vec<(Vec<u8>, u32, u32)> = Vec::new();

    // ── Frames joueur (Knight_1) ─────────────────────────────────────────────
    // Tous les frames sont normalisés à KNIGHT_FRAME_W (64px) avec centrage horizontal
    // pour éviter le saut visuel lors des transitions d'animation.
    //
    // Atlas layout :
    //   0–9   Knight Idle  (10 frames × 29→64×86)
    //   10–17 Knight Walk  (8 frames × 64×64, knight_walk.png)
    //   18–27 Knight Dead  (10 frames × 48→64×86)
    //   28–32 Mobs statiques (5 types)
    let idle_raw = extract_frames(&knight_dir.join("Idle.png"), 29, 86, 10);
    for (src, src_w, src_h) in &idle_raw {
        let padded = pad_frame_to_width(src, *src_w, *src_h, KNIGHT_FRAME_W);
        images.push((padded, KNIGHT_FRAME_W, *src_h));
    }
    // Walk : knight_walk.png (8 frames × 64×64) — déjà à KNIGHT_FRAME_W
    images.extend(extract_frames(&knight_dir.join("knight_walk.png"), 64, 64, 8)); // 10–17
    let dead_raw = extract_frames(&knight_dir.join("Dead.png"), 48, 86, 10);
    for (src, src_w, src_h) in &dead_raw {
        let padded = pad_frame_to_width(src, *src_w, *src_h, KNIGHT_FRAME_W);
        images.push((padded, KNIGHT_FRAME_W, *src_h));
    }

    // ── Sprites mobs (Dev_assets) ────────────────────────────────────────────
    let mob_files = [
        "Test_mob.png",
        "Test_boss.png",
        "Test_elite.png",
        "Test_archer_follower.png",
        "Test_guerrier_follower.png",
    ];
    for name in &mob_files {
        let p = dev_assets.join(name);
        match image::open(&p) {
            Ok(img) => {
                let (w, h) = img.dimensions();
                images.push((img.to_rgba8().into_raw(), w, h));
            }
            Err(e) => {
                log::warn!("Sprite mob manquant {:?}: {}", p, e);
                let (fw, fh) = (32u32, 48u32);
                let mut rgba = vec![0u8; (fw * fh * 4) as usize];
                for i in (0..rgba.len()).step_by(4) {
                    rgba[i] = 255; rgba[i + 1] = 0; rgba[i + 2] = 255; rgba[i + 3] = 255;
                }
                images.push((rgba, fw, fh));
            }
        }
    }

    // ── Sprites NPC village (Dev_assets/npc_village) ─────────────────────────
    // Indices 33–36 : forgeron, recruteur, serveuse, tavernier (64×64 chacun)
    let npc_files = [
        "npc_village/forgeron.png",
        "npc_village/recruteur.png",
        "npc_village/serveuse.png",
        "npc_village/tavernier.png",
    ];
    for name in &npc_files {
        let p = dev_assets.join(name);
        match image::open(&p) {
            Ok(img) => {
                let (w, h) = img.dimensions();
                images.push((img.to_rgba8().into_raw(), w, h));
            }
            Err(e) => {
                log::warn!("Sprite NPC manquant {:?}: {}", p, e);
                let (fw, fh) = (64u32, 64u32);
                let mut rgba = vec![0u8; (fw * fh * 4) as usize];
                for i in (0..rgba.len()).step_by(4) {
                    rgba[i] = 0; rgba[i + 1] = 200; rgba[i + 2] = 255; rgba[i + 3] = 255;
                }
                images.push((rgba, fw, fh));
            }
        }
    }

    // ── Maison (indice 37) : Medieval_Building_06.png redimensionné ~128×96 ──
    {
        let p = dev_assets.join("Medieval_Building_06.png");
        match image::open(&p) {
            Ok(img) => {
                let resized = img.resize(128, 96, image::imageops::FilterType::Lanczos3);
                let (w, h) = resized.dimensions();
                images.push((resized.to_rgba8().into_raw(), w, h));
            }
            Err(e) => {
                log::warn!("Maison manquante {:?}: {}", p, e);
                let (fw, fh) = (128u32, 96u32);
                let mut rgba = vec![0u8; (fw * fh * 4) as usize];
                for i in (0..rgba.len()).step_by(4) {
                    rgba[i] = 139; rgba[i + 1] = 90; rgba[i + 2] = 43; rgba[i + 3] = 255;
                }
                images.push((rgba, fw, fh));
            }
        }
    }

    // ── Arbres (indices 38–39) : arbre_a.png et arbre_b.png redimensionnés ~64×68 ──
    for name in &["arbre_a.png", "arbre_b.png"] {
        let p = dev_assets.join(name);
        match image::open(&p) {
            Ok(img) => {
                let resized = img.resize(64, 68, image::imageops::FilterType::Lanczos3);
                let (w, h) = resized.dimensions();
                images.push((resized.to_rgba8().into_raw(), w, h));
            }
            Err(e) => {
                log::warn!("Arbre manquant {:?}: {}", p, e);
                let (fw, fh) = (64u32, 68u32);
                let mut rgba = vec![0u8; (fw * fh * 4) as usize];
                for i in (0..rgba.len()).step_by(4) {
                    rgba[i] = 0; rgba[i + 1] = 128; rgba[i + 2] = 0; rgba[i + 3] = 255;
                }
                images.push((rgba, fw, fh));
            }
        }
    }

    log::info!("Char atlas : {} entrées ({} knight + {} mobs + {} npc + 1 house + 2 trees)",
        images.len(), 28, mob_files.len(), npc_files.len());

    // ── Pack horizontal, aligné bas ──────────────────────────────────────────
    let atlas_h = images.iter().map(|(_, _, h)| *h).max().unwrap_or(86);
    let atlas_w: u32 = images.iter().map(|(_, w, _)| *w).sum();

    let mut atlas = vec![0u8; (atlas_w * atlas_h * 4) as usize];
    let mut entries: Vec<CharEntry> = Vec::with_capacity(images.len());
    let mut cursor_x = 0u32;

    for (src, w, h) in &images {
        let sprite_w = *w;
        let sprite_h = *h;
        let y_offset = atlas_h - sprite_h;
        for row in 0..sprite_h {
            let src_start = (row * sprite_w * 4) as usize;
            let dst_row = row + y_offset;
            let dst_start = ((dst_row * atlas_w + cursor_x) * 4) as usize;
            let len = (sprite_w * 4) as usize;
            if src_start + len <= src.len() && dst_start + len <= atlas.len() {
                atlas[dst_start..dst_start + len]
                    .copy_from_slice(&src[src_start..src_start + len]);
            }
        }
        let u_min = cursor_x as f32 / atlas_w as f32;
        let u_max = (cursor_x + sprite_w) as f32 / atlas_w as f32;
        let v_min = y_offset as f32 / atlas_h as f32;
        let v_max = (y_offset + sprite_h) as f32 / atlas_h as f32;
        entries.push(CharEntry {
            sprite_w: sprite_w as f32,
            sprite_h: sprite_h as f32,
            u_min, v_min, u_max, v_max,
        });
        cursor_x += sprite_w;
    }

    (atlas, atlas_w, atlas_h, entries)
}

// ── Helpers pipeline ─────────────────────────────────────────────────────────

fn create_texture_from_rgba(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    rgba: &[u8],
    width: u32,
    height: u32,
    label: &str,
) -> wgpu::Texture {
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(width * 4),
            rows_per_image: Some(height),
        },
        wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
    );
    tex
}

// ── Quad de base (2 triangles, [0,0]-[1,1]) ──────────────────────────────────

const QUAD_VERTICES: &[[f32; 4]] = &[
    [0.0, 0.0, 0.0, 0.0],
    [1.0, 0.0, 1.0, 0.0],
    [1.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 0.0],
    [1.0, 1.0, 1.0, 1.0],
    [0.0, 1.0, 0.0, 1.0],
];

// ── Impl WgpuRenderer ─────────────────────────────────────────────────────────

impl WgpuRenderer {
    pub async fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        surface: wgpu::Surface<'static>,
        adapter: &wgpu::Adapter,
        size: (u32, u32),
        dev_assets: &Path,
    ) -> Result<Self, RendererInitError> {
        let caps = surface.get_capabilities(adapter);
        let format = *caps.formats.first().ok_or(RendererInitError::NoFormat)?;

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        let vw = size.0 as f32;
        let vh = size.1 as f32;

        // ── Atlas terrain saisonnier ────────────────────────────────────────
        // Priorité : seasons_atlas_grid.png (96 tiles, grille 48×2, 6144×128)
        // Fallback : seasons_atlas_strip.png, puis Grass_a..h
        let grid_path = dev_assets.join("seasons_atlas_grid.png");
        let strip_path = dev_assets.join("seasons_atlas_strip.png");
        let (tile_rgba, tile_w_total, tile_h_total, tile_single_w, tile_atlas_cols) =
            if grid_path.exists() {
                match image::open(&grid_path) {
                    Ok(img) => {
                        let (w, h) = img.dimensions();
                        let cols = w / 128;
                        let rows = h / 64;
                        let total = cols * rows;
                        log::info!("Seasons grid atlas: {}×{} ({}cols × {}rows = {} tiles)", w, h, cols, rows, total);
                        (img.to_rgba8().into_raw(), w, h, 128u32, cols)
                    }
                    Err(e) => {
                        log::warn!("Seasons grid atlas error: {} — fallback grass", e);
                        let (a, b, c, d) = build_grass_atlas(dev_assets);
                        let cols = b / d.max(1);
                        (a, b, c, d, cols)
                    }
                }
            } else if strip_path.exists() {
                match image::open(&strip_path) {
                    Ok(img) => {
                        let (w, h) = img.dimensions();
                        let cols = w / 128;
                        log::info!("Seasons strip atlas: {}×{} ({} tiles)", w, h, cols);
                        (img.to_rgba8().into_raw(), w, h, 128u32, cols)
                    }
                    Err(e) => {
                        log::warn!("Seasons strip atlas error: {} — fallback grass", e);
                        let (a, b, c, d) = build_grass_atlas(dev_assets);
                        let cols = b / d.max(1);
                        (a, b, c, d, cols)
                    }
                }
            } else {
                log::info!("No seasons atlas — fallback grass 8 tiles");
                let (a, b, c, d) = build_grass_atlas(dev_assets);
                let cols = b / d.max(1);
                (a, b, c, d, cols)
            };
        let tile_atlas_rows = tile_h_total / 64u32.max(1);
        let tile_num_graphic_ids = tile_atlas_cols * tile_atlas_rows;
        log::info!("Tile atlas: {}×{} | {}cols × {}rows | {} tiles", tile_w_total, tile_h_total, tile_atlas_cols, tile_atlas_rows, tile_num_graphic_ids);
        let tile_tex = create_texture_from_rgba(&device, &queue, &tile_rgba, tile_w_total, tile_h_total, "tile_atlas");

        // ── Atlas personnages ────────────────────────────────────────────────
        let (char_rgba, char_atlas_w, char_atlas_h, char_entries) = build_char_atlas(dev_assets);
        let char_tex = create_texture_from_rgba(&device, &queue, &char_rgba, char_atlas_w, char_atlas_h, "char_atlas");

        // ── Sampler Nearest (pixel art) ──────────────────────────────────────
        let sampler_nearest = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // ── Shader tile (sprite.wgsl) ────────────────────────────────────────
        let tile_shader_src = include_str!("../assets/shaders/sprite.wgsl");
        let tile_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("tile_shader"),
            source: wgpu::ShaderSource::Wgsl(tile_shader_src.into()),
        });

        // ── Shader instanced (sprite_instanced.wgsl) ────────────────────────
        let inst_shader_src = include_str!("../assets/shaders/sprite_instanced.wgsl");
        let inst_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("inst_shader"),
            source: wgpu::ShaderSource::Wgsl(inst_shader_src.into()),
        });

        // ── Shader colored quad (colored_quad.wgsl) ──────────────────────────
        let quad_shader_src = include_str!("../assets/shaders/colored_quad.wgsl");
        let quad_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("quad_shader"),
            source: wgpu::ShaderSource::Wgsl(quad_shader_src.into()),
        });

        // ── Bind group layout : tile ─────────────────────────────────────────
        let tile_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("tile_bgl"),
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
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
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

        // ── Bind group layout : instanced (sprite + texte) ───────────────────
        let inst_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("inst_bgl"),
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

        // ── Bind group layout : colored quad ────────────────────────────────
        let quad_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("quad_bgl"),
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

        // ── Uniform buffers ──────────────────────────────────────────────────
        let view_size_data: [f32; 2] = [vw, vh];
        let view_size_bytes = bytemuck::cast_slice(&view_size_data);

        let tile_uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("tile_uniform"),
            contents: view_size_bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let sprite_uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("sprite_uniform"),
            contents: view_size_bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let quad_uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("quad_uniform"),
            contents: view_size_bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let text_uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text_uniform"),
            contents: view_size_bytes,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // ── Texture views ────────────────────────────────────────────────────
        let tile_tex_view = tile_tex.create_view(&Default::default());
        let char_tex_view = char_tex.create_view(&Default::default());

        // ── Bind groups ──────────────────────────────────────────────────────
        let tile_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("tile_bg"),
            layout: &tile_bgl,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&tile_tex_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler_nearest) },
                wgpu::BindGroupEntry { binding: 2, resource: tile_uniform_buf.as_entire_binding() },
            ],
        });

        let sprite_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sprite_bg"),
            layout: &inst_bgl,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: sprite_uniform_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&char_tex_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler_nearest) },
            ],
        });

        let quad_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("quad_bg"),
            layout: &quad_bgl,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: quad_uniform_buf.as_entire_binding() },
            ],
        });

        // ── Text atlas (RGBA blanc+alpha, rasterisé à la demande) ────────────
        let mut glyph_cache = GlyphCache::new();
        // Pré-rasteriser ASCII imprimable
        for ch in ' '..='~' {
            glyph_cache.ensure_glyph(ch);
        }
        // RGBA : chaque glyph est blanc (R=G=B=255) avec l'alpha fontdue
        let text_tex = create_texture_from_rgba(
            &device, &queue,
            &glyph_cache.atlas,
            TEXT_ATLAS_W, TEXT_ATLAS_H,
            "text_atlas",
        );
        // Sampler linear pour le texte
        let sampler_linear = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let text_tex_view = text_tex.create_view(&Default::default());
        let text_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("text_bg"),
            layout: &inst_bgl,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: text_uniform_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&text_tex_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler_linear) },
            ],
        });

        // ── Vertex layout tile ────────────────────────────────────────────────
        let tile_vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TileVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
        };

        // ── Vertex layout instanced ───────────────────────────────────────────
        // Quad vertex buffer : [position(2), uv(2)]
        let inst_quad_layout = wgpu::VertexBufferLayout {
            array_stride: 4 * 4,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
        };
        let inst_instance_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![2 => Float32x2, 3 => Float32x2, 4 => Float32x4],
        };

        // ── Vertex layout colored ─────────────────────────────────────────────
        let quad_instance_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ColoredInstanceRaw>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &wgpu::vertex_attr_array![2 => Float32x2, 3 => Float32x2, 4 => Float32x4],
        };

        // ── Pipeline tile ─────────────────────────────────────────────────────
        let tile_pl_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("tile_pl_layout"),
            bind_group_layouts: &[&tile_bgl],
            push_constant_ranges: &[],
        });
        let tile_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("tile_pipeline"),
            layout: Some(&tile_pl_layout),
            vertex: wgpu::VertexState {
                module: &tile_shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[tile_vertex_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &tile_shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
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

        // ── Pipeline sprite instanced ────────────────────────────────────────
        let inst_pl_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("inst_pl_layout"),
            bind_group_layouts: &[&inst_bgl],
            push_constant_ranges: &[],
        });
        let sprite_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("sprite_pipeline"),
            layout: Some(&inst_pl_layout),
            vertex: wgpu::VertexState {
                module: &inst_shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[inst_quad_layout.clone(), inst_instance_layout.clone()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &inst_shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
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

        // ── Pipeline text (même shader instanced, texture R8) ────────────────
        let text_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("text_pipeline"),
            layout: Some(&inst_pl_layout),
            vertex: wgpu::VertexState {
                module: &inst_shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[inst_quad_layout, inst_instance_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &inst_shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
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

        // ── Pipeline colored quad ────────────────────────────────────────────
        let quad_pl_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("quad_pl_layout"),
            bind_group_layouts: &[&quad_bgl],
            push_constant_ranges: &[],
        });
        let quad_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("quad_pipeline"),
            layout: Some(&quad_pl_layout),
            vertex: wgpu::VertexState {
                module: &quad_shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 4 * 4,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
                    },
                    quad_instance_layout,
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &quad_shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
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

        // ── Buffers GPU ───────────────────────────────────────────────────────
        let quad_bytes: &[u8] = bytemuck::cast_slice(QUAD_VERTICES);
        let sprite_quad_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("sprite_quad_buf"),
            contents: quad_bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });

        let tile_vertex_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("tile_vertex_buf"),
            size: (MAX_TILE_INSTANCES * 6 * std::mem::size_of::<TileVertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let sprite_instance_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("sprite_instance_buf"),
            size: (MAX_SPRITE_INSTANCES * std::mem::size_of::<InstanceRaw>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let quad_instance_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("quad_instance_buf"),
            size: (MAX_DEBUG_QUADS * std::mem::size_of::<ColoredInstanceRaw>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let text_instance_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("text_instance_buf"),
            size: (MAX_TEXT_CHARS * std::mem::size_of::<InstanceRaw>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        // Ombres (même capacité que les sprites, un shadow quad par entité)
        let shadow_instance_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("shadow_instance_buf"),
            size: (MAX_SPRITE_INSTANCES * std::mem::size_of::<ColoredInstanceRaw>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            viewport_size: (vw, vh),

            tile_pipeline,
            tile_texture: tile_tex,
            tile_vertex_buf,
            tile_uniform_buf,
            tile_bind_group,
            tile_atlas_w: tile_w_total,
            tile_atlas_h: tile_h_total,
            tile_num_graphic_ids,
            tile_atlas_cols,
            tile_atlas_rows,

            sprite_pipeline,
            sprite_quad_buf,
            sprite_instance_buf,
            sprite_bind_group,
            char_entries,
            char_atlas_w,
            char_atlas_h: char_atlas_h,

            quad_pipeline,
            quad_instance_buf,
            quad_bind_group,

            text_pipeline,
            text_instance_buf,
            text_bind_group,
            glyph_cache,

            shadow_instance_buf,

            tile_vertices: Vec::with_capacity(MAX_TILE_INSTANCES * 6),
            sprite_instances: Vec::with_capacity(MAX_SPRITE_INSTANCES),
            shadow_quads: Vec::with_capacity(MAX_SPRITE_INSTANCES),
            debug_quads: Vec::with_capacity(MAX_DEBUG_QUADS),
            text_instances: Vec::with_capacity(MAX_TEXT_CHARS),
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width.max(1);
        self.surface_config.height = height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
        self.viewport_size = (width as f32, height as f32);
        // Mettre à jour les uniforms de taille de vue
        let data: [f32; 2] = [width as f32, height as f32];
        let bytes = bytemuck::cast_slice(&data);
        self.queue.write_buffer(&self.tile_uniform_buf, 0, bytes);
    }

    // ── Dessin principal ──────────────────────────────────────────────────────

    /// Dessine un frame complet.
    /// `entity_sprites` : liste (world_x, world_y, sprite_id, hp_ratio, is_selected) déjà Y-sortée.
    /// `hp_ratio` : [0.0, 1.0] pour afficher la barre de vie, < 0.0 pour la masquer.
    /// `is_selected` : dessine un ovale cyan sous l'entité (style sélection RTS).
    /// `floating_texts` : liste (screen_x, screen_y, text) pour les dégâts flottants.
    pub fn draw_frame(
        &mut self,
        tilemap: &TileMap,
        camera: &IsoCamera,
        entity_sprites: impl Iterator<Item = (f32, f32, u8, f32, bool)>,
        dev_state: Option<&DevState>,
        stats_text: Option<&str>,
        floating_texts: &[(f32, f32, &str)],
        gui_state: &GuiState,
        hud: &HudData,
    ) -> Result<(), wgpu::SurfaceError> {
        let (vw, vh) = self.viewport_size;

        // ── Passe 1 : préparer les vertices de tiles ─────────────────────────
        self.tile_vertices.clear();
        let (min_tx, min_ty, max_tx, max_ty) = camera.visible_tile_bounds(vw, vh, 2);

        let cols = self.tile_atlas_cols.max(1);
        let rows = self.tile_atlas_rows.max(1);
        let tile_uv_w = 1.0 / cols as f32;
        let tile_uv_h = 1.0 / rows as f32;

        for (tx, ty, tile) in tilemap.visible_tiles(min_tx, min_ty, max_tx, max_ty) {
            if self.tile_vertices.len() + 6 > MAX_TILE_INSTANCES * 6 {
                break;
            }
            // Projection du centre de la tuile
            let (sx, sy) = camera.project(tx as f32 + 0.5, ty as f32 + 0.5, vw, vh);

            // Diamant isométrique : 2*TILE_HALF_W × TILE_HALF_H pixels (après zoom)
            let half_w = TILE_HALF_W * camera.zoom;
            let half_h = TILE_HALF_H * camera.zoom;

            // 4 sommets du diamant (top, right, bottom, left)
            let p_top =    [sx,          sy - half_h];
            let p_right =  [sx + half_w, sy          ];
            let p_bottom = [sx,          sy + half_h ];
            let p_left =   [sx - half_w, sy          ];

            // UV dans l'atlas grid selon graphic_id (col/row)
            let gid = tile.graphic_id as u32 % self.tile_num_graphic_ids.max(1);
            let col = gid % cols;
            let row = gid / cols;
            let u0 = col as f32 * tile_uv_w;
            let u1 = u0 + tile_uv_w;
            let v0 = row as f32 * tile_uv_h;
            let v1 = v0 + tile_uv_h;

            // UV sur le diamant : milieux des côtés de la texture
            let u_mid = (u0 + u1) * 0.5;
            let v_mid = (v0 + v1) * 0.5;

            // Quad en 2 triangles, pivot diamond (top→right→bottom→left)
            let verts = [
                TileVertex { position: p_top,    uv: [u_mid, v0  ] },
                TileVertex { position: p_right,   uv: [u1,   v_mid] },
                TileVertex { position: p_bottom,  uv: [u_mid, v1  ] },
                TileVertex { position: p_top,    uv: [u_mid, v0  ] },
                TileVertex { position: p_bottom,  uv: [u_mid, v1  ] },
                TileVertex { position: p_left,    uv: [u0,   v_mid] },
            ];
            self.tile_vertices.extend_from_slice(&verts);
        }

        // ── Passe 2a : ombres + sélection (AVANT sprites) ───────────────────────
        // Style OpenDiablo2 : diamant plat semi-transparent sous les pieds
        self.shadow_quads.clear();
        // ── Passe 2b : sprites entités ───────────────────────────────────────────
        self.sprite_instances.clear();
        self.debug_quads.clear(); // cleared ici pour inclure les HP bars dans le rendu
        for (wx, wy, sid, hp_ratio, is_selected) in entity_sprites {
            if self.sprite_instances.len() >= MAX_SPRITE_INSTANCES {
                break;
            }
            // Pied de l'entité en écran (BottomCenter pivot : position monde = pied)
            let (foot_sx, foot_sy) = camera.project(wx, wy, vw, vh);

            // Ovale sélection (cyan, AVANT l'ombre pour rester au sol)
            if is_selected && self.shadow_quads.len() < MAX_SPRITE_INSTANCES {
                let sel_w = TILE_HALF_W * camera.zoom * 1.6;
                let sel_h = TILE_HALF_H * camera.zoom * 1.0;
                self.shadow_quads.push(ColoredInstanceRaw {
                    screen_pos: [foot_sx - sel_w * 0.5, foot_sy - sel_h * 0.5],
                    size: [sel_w, sel_h],
                    color: [0.0, 0.8, 1.0, 0.45],
                });
            }

            // Ombre : petit diamant aplati centré aux pieds (style OpenDiablo2)
            if self.shadow_quads.len() < MAX_SPRITE_INSTANCES {
                let sh_w = TILE_HALF_W * camera.zoom * 1.2;
                let sh_h = TILE_HALF_H * camera.zoom * 0.7;
                self.shadow_quads.push(ColoredInstanceRaw {
                    screen_pos: [foot_sx - sh_w * 0.5, foot_sy - sh_h * 0.5],
                    size: [sh_w, sh_h],
                    color: [0.0, 0.0, 0.0, 0.28],
                });
            }

            let entry = match self.char_entries.get(sid as usize) {
                Some(e) => *e,
                None => continue,
            };

            // Sprite (BottomCenter pivot)
            let tl_x = foot_sx - entry.sprite_w * 0.5 * camera.zoom;
            let tl_y = foot_sy - entry.sprite_h * camera.zoom;
            self.sprite_instances.push(InstanceRaw {
                screen_pos: [tl_x, tl_y],
                size: [entry.sprite_w * camera.zoom, entry.sprite_h * camera.zoom],
                uv_rect: [entry.u_min, entry.v_min, entry.u_max, entry.v_max],
            });

            // ── HP bar (au-dessus du sprite) ──────────────────────────────────
            if hp_ratio >= 0.0 && self.debug_quads.len() + 2 <= MAX_DEBUG_QUADS {
                let bar_w = (entry.sprite_w * camera.zoom).min(32.0 * camera.zoom);
                let bar_h = 3.0_f32.max(camera.zoom);
                let bx = foot_sx - bar_w * 0.5;
                let by = tl_y - bar_h - 2.0;

                // Fond sombre
                self.debug_quads.push(ColoredInstanceRaw {
                    screen_pos: [bx, by],
                    size: [bar_w, bar_h],
                    color: [0.08, 0.08, 0.08, 0.88],
                });
                // Remplissage coloré (vert/jaune/rouge selon ratio)
                let (r, g) = if hp_ratio > 0.5 {
                    (0.15, 0.85)
                } else if hp_ratio > 0.25 {
                    (0.9, 0.7)
                } else {
                    (0.9, 0.1)
                };
                let fill_w = (bar_w * hp_ratio).max(0.0);
                if fill_w > 0.0 {
                    self.debug_quads.push(ColoredInstanceRaw {
                        screen_pos: [bx, by],
                        size: [fill_w, bar_h],
                        color: [r, g, 0.05, 0.92],
                    });
                }
            }
        }

        // ── Passe 3 : debug quads (grille isométrique si show_grid) ──────────
        if dev_state.map(|d| d.show_grid).unwrap_or(false) {
            for (tx, ty, _) in tilemap.visible_tiles(min_tx, min_ty, max_tx, max_ty) {
                if self.debug_quads.len() + 4 > MAX_DEBUG_QUADS {
                    break;
                }
                let (sx, sy) = camera.project(tx as f32 + 0.5, ty as f32 + 0.5, vw, vh);
                let hw = TILE_HALF_W * camera.zoom;
                let hh = TILE_HALF_H * camera.zoom;
                // 4 lignes du diamant (représentées comme quads de 1px)
                self.push_line_quad(sx, sy - hh, sx + hw, sy, [1.0, 1.0, 0.0, 0.3]);
                self.push_line_quad(sx + hw, sy, sx, sy + hh, [1.0, 1.0, 0.0, 0.3]);
                self.push_line_quad(sx, sy + hh, sx - hw, sy, [1.0, 1.0, 0.0, 0.3]);
                self.push_line_quad(sx - hw, sy, sx, sy - hh, [1.0, 1.0, 0.0, 0.3]);
            }
        }

        // ── Passe 4 : texte overlay (screen space, couche 100+) ───────────────
        self.text_instances.clear();
        if dev_state.map(|d| d.show_stats).unwrap_or(false) {
            if let Some(text) = stats_text {
                // Panneau fond semi-transparent
                if self.debug_quads.len() < MAX_DEBUG_QUADS {
                    self.debug_quads.push(ColoredInstanceRaw {
                        screen_pos: [8.0, 8.0],
                        size: [400.0, 24.0],
                        color: [0.0, 0.0, 0.0, 0.55],
                    });
                }
                self.push_text(text, 12.0, 14.0, [1.0, 1.0, 1.0, 1.0]);
            }
        }

        // ── Passe 4b : dégâts flottants (world space → screen) ──────────────
        for &(sx, sy, text) in floating_texts {
            self.push_text(text, sx, sy, [1.0, 0.3, 0.1, 1.0]);
        }

        // ── Passe 5 : GUI (HUD + fenêtres) ───────────────────────────────────
        self.draw_hud(vw, vh, hud);
        self.draw_nav_bar(vw, gui_state);
        if gui_state.show_character {
            let (wx, wy) = gui_state.char_pos;
            self.draw_window(wx, wy, CHAR_WIN_W, CHAR_WIN_H, "== PERSONNAGE ==",
                             [0.12, 0.2, 0.12, 1.0], [0.35, 0.55, 0.35, 1.0]);
            self.draw_character_content(wx, wy, hud);
        }
        if gui_state.show_skills {
            let (wx, wy) = gui_state.skills_pos;
            self.draw_window(wx, wy, SKILLS_WIN_W, SKILLS_WIN_H, "== COMPETENCES ==",
                             [0.1, 0.1, 0.25, 1.0], [0.3, 0.3, 0.6, 1.0]);
            self.draw_skills_content(wx, wy, hud);
        }
        if gui_state.show_inventory {
            let (wx, wy) = gui_state.inv_pos;
            self.draw_window(wx, wy, INV_WIN_W, INV_WIN_H, "== INVENTAIRE ==",
                             [0.14, 0.1, 0.04, 1.0], [0.55, 0.4, 0.15, 1.0]);
            self.draw_inventory_content(wx, wy);
        }

        // ── Upload GPU ────────────────────────────────────────────────────────
        if !self.tile_vertices.is_empty() {
            self.queue.write_buffer(
                &self.tile_vertex_buf,
                0,
                bytemuck::cast_slice(&self.tile_vertices),
            );
        }
        if !self.sprite_instances.is_empty() {
            self.queue.write_buffer(
                &self.sprite_instance_buf,
                0,
                bytemuck::cast_slice(&self.sprite_instances),
            );
        }
        if !self.shadow_quads.is_empty() {
            self.queue.write_buffer(
                &self.shadow_instance_buf,
                0,
                bytemuck::cast_slice(&self.shadow_quads),
            );
        }
        if !self.debug_quads.is_empty() {
            self.queue.write_buffer(
                &self.quad_instance_buf,
                0,
                bytemuck::cast_slice(&self.debug_quads),
            );
        }
        if !self.text_instances.is_empty() {
            self.queue.write_buffer(
                &self.text_instance_buf,
                0,
                bytemuck::cast_slice(&self.text_instances),
            );
        }

        // ── Rendu ─────────────────────────────────────────────────────────────
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("frame_encoder"),
        });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("main_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.05, g: 0.05, b: 0.08, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Pass 5 (WorldBelow) : tiles
            if !self.tile_vertices.is_empty() {
                rpass.set_pipeline(&self.tile_pipeline);
                rpass.set_bind_group(0, &self.tile_bind_group, &[]);
                rpass.set_vertex_buffer(0, self.tile_vertex_buf.slice(..));
                rpass.draw(0..self.tile_vertices.len() as u32, 0..1);
            }

            // Pass 7 (Shadow) — ombres sous entités, style OpenDiablo2, AVANT les sprites
            if !self.shadow_quads.is_empty() {
                rpass.set_pipeline(&self.quad_pipeline);
                rpass.set_bind_group(0, &self.quad_bind_group, &[]);
                rpass.set_vertex_buffer(0, self.sprite_quad_buf.slice(..));
                rpass.set_vertex_buffer(1, self.shadow_instance_buf.slice(..));
                rpass.draw(0..6, 0..self.shadow_quads.len() as u32);
            }

            // Pass 10 (World) : sprites entities
            if !self.sprite_instances.is_empty() {
                rpass.set_pipeline(&self.sprite_pipeline);
                rpass.set_bind_group(0, &self.sprite_bind_group, &[]);
                rpass.set_vertex_buffer(0, self.sprite_quad_buf.slice(..));
                rpass.set_vertex_buffer(1, self.sprite_instance_buf.slice(..));
                rpass.draw(0..6, 0..self.sprite_instances.len() as u32);
            }

            // Pass 15 (WorldDebug) + Pass 100 (UI) : debug quads et stats panel
            if !self.debug_quads.is_empty() {
                rpass.set_pipeline(&self.quad_pipeline);
                rpass.set_bind_group(0, &self.quad_bind_group, &[]);
                rpass.set_vertex_buffer(0, self.sprite_quad_buf.slice(..));
                rpass.set_vertex_buffer(1, self.quad_instance_buf.slice(..));
                rpass.draw(0..6, 0..self.debug_quads.len() as u32);
            }

            // Pass 110 (OverlayText) : texte
            if !self.text_instances.is_empty() {
                rpass.set_pipeline(&self.text_pipeline);
                rpass.set_bind_group(0, &self.text_bind_group, &[]);
                rpass.set_vertex_buffer(0, self.sprite_quad_buf.slice(..));
                rpass.set_vertex_buffer(1, self.text_instance_buf.slice(..));
                rpass.draw(0..6, 0..self.text_instances.len() as u32);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }

    // ── Helpers CPU ───────────────────────────────────────────────────────────

    /// Ajoute une ligne comme un quad très fin (approximation)
    fn push_line_quad(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, color: [f32; 4]) {
        if self.debug_quads.len() >= MAX_DEBUG_QUADS {
            return;
        }
        let dx = x1 - x0;
        let dy = y1 - y0;
        let len = (dx * dx + dy * dy).sqrt().max(1.0);
        // Quad aligné sur la ligne : position = milieu, taille = (len, 1)
        // Simplification : quad horizontal de 1px de haut (OK pour grille)
        let _ = len;
        self.debug_quads.push(ColoredInstanceRaw {
            screen_pos: [x0.min(x1), y0.min(y1)],
            size: [(dx.abs()).max(1.0), (dy.abs()).max(1.0)],
            color,
        });
    }

    /// Ajoute un quad de couleur (raccourci)
    fn push_quad(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        if self.debug_quads.len() < MAX_DEBUG_QUADS {
            self.debug_quads.push(ColoredInstanceRaw { screen_pos: [x, y], size: [w, h], color });
        }
    }

    // ── HUD bas : barres PV / Mana / XP ──────────────────────────────────────

    /// Dessine le HUD bas (barres de ressources).
    fn draw_hud(&mut self, vw: f32, vh: f32, hud: &HudData) {
        const HUD_H: f32 = 84.0;
        let hy = vh - HUD_H;

        // Fond HUD
        self.push_quad(0.0, hy, vw, HUD_H, [0.04, 0.04, 0.07, 0.92]);
        // Ligne séparatrice
        self.push_quad(0.0, hy, vw, 2.0, [0.25, 0.25, 0.35, 1.0]);

        // Indicateur safe zone
        if hud.in_safe_zone {
            self.push_quad(vw - 180.0, hy + 6.0, 172.0, 20.0, [0.1, 0.5, 0.1, 0.7]);
            self.push_text("ZONE SECURISEE", vw - 174.0, hy + 20.0, [0.4, 1.0, 0.4, 1.0]);
        }

        // Titre niveau + or
        let level_str = format!("Niv.{}", hud.level);
        self.push_text(&level_str, 8.0, hy + 18.0, [1.0, 0.85, 0.2, 1.0]);
        let gold_str = format!("Or:{}", hud.gold);
        self.push_text(&gold_str, 8.0, hy + 34.0, [1.0, 0.82, 0.1, 1.0]);

        // ── Barre PV ──────────────────────────────────────────────────────────
        let bar_y = hy + 28.0;
        let bar_h = 18.0;
        let bar_w = 220.0;
        let hp_x = 8.0;

        self.push_text("PV", hp_x, bar_y - 2.0, [0.9, 0.9, 0.9, 1.0]);
        // fond barre
        self.push_quad(hp_x + 24.0, bar_y, bar_w, bar_h, [0.12, 0.04, 0.04, 1.0]);
        // remplissage
        let hp_ratio = if hud.hp_max > 0 { (hud.hp as f32 / hud.hp_max as f32).clamp(0.0, 1.0) } else { 0.0 };
        let fill_w = (bar_w * hp_ratio).max(0.0);
        let (hr, hg) = if hp_ratio > 0.5 { (0.15, 0.85) } else if hp_ratio > 0.25 { (0.9, 0.6) } else { (0.9, 0.1) };
        self.push_quad(hp_x + 24.0, bar_y, fill_w, bar_h, [hr, hg, 0.05, 1.0]);
        // bordure
        self.push_quad(hp_x + 24.0, bar_y, bar_w, 1.0, [0.5, 0.1, 0.1, 1.0]);
        self.push_quad(hp_x + 24.0, bar_y + bar_h - 1.0, bar_w, 1.0, [0.5, 0.1, 0.1, 1.0]);
        // texte
        let hp_str = format!("{}/{}", hud.hp, hud.hp_max);
        self.push_text(&hp_str, hp_x + 24.0 + bar_w * 0.5 - 16.0, bar_y + 13.0, [1.0, 1.0, 1.0, 1.0]);

        // ── Barre Mana ────────────────────────────────────────────────────────
        let mn_x = 8.0 + bar_w + 36.0;
        self.push_text("MP", mn_x, bar_y - 2.0, [0.5, 0.7, 1.0, 1.0]);
        self.push_quad(mn_x + 24.0, bar_y, bar_w, bar_h, [0.04, 0.04, 0.18, 1.0]);
        let mn_ratio = if hud.mana_max > 0 { (hud.mana as f32 / hud.mana_max as f32).clamp(0.0, 1.0) } else { 0.0 };
        let mn_fill = (bar_w * mn_ratio).max(0.0);
        self.push_quad(mn_x + 24.0, bar_y, mn_fill, bar_h, [0.1, 0.35, 0.95, 1.0]);
        self.push_quad(mn_x + 24.0, bar_y, bar_w, 1.0, [0.1, 0.1, 0.5, 1.0]);
        self.push_quad(mn_x + 24.0, bar_y + bar_h - 1.0, bar_w, 1.0, [0.1, 0.1, 0.5, 1.0]);
        let mn_str = format!("{}/{}", hud.mana, hud.mana_max);
        self.push_text(&mn_str, mn_x + 24.0 + bar_w * 0.5 - 16.0, bar_y + 13.0, [1.0, 1.0, 1.0, 1.0]);

        // ── Barre Endurance (End) ─────────────────────────────────────────────
        let en_x = 8.0 + (bar_w + 36.0) * 2.0;
        self.push_text("End", en_x, bar_y - 2.0, [0.9, 0.65, 0.1, 1.0]);
        self.push_quad(en_x + 30.0, bar_y, bar_w, bar_h, [0.08, 0.06, 0.01, 1.0]);
        let en_ratio = if hud.end_max > 0 { (hud.end as f32 / hud.end_max as f32).clamp(0.0, 1.0) } else { 0.0 };
        let en_fill = (bar_w * en_ratio).max(0.0);
        let en_col = if en_ratio > 0.5 { [0.9, 0.7, 0.05, 1.0] } else { [0.75, 0.35, 0.05, 1.0] };
        self.push_quad(en_x + 30.0, bar_y, en_fill, bar_h, en_col);
        self.push_quad(en_x + 30.0, bar_y, bar_w, 1.0, [0.5, 0.3, 0.0, 1.0]);
        self.push_quad(en_x + 30.0, bar_y + bar_h - 1.0, bar_w, 1.0, [0.5, 0.3, 0.0, 1.0]);
        let en_str = format!("{}/{}", hud.end, hud.end_max);
        self.push_text(&en_str, en_x + 30.0 + bar_w * 0.5 - 16.0, bar_y + 13.0, [1.0, 1.0, 1.0, 1.0]);

        // ── Barre XP ──────────────────────────────────────────────────────────
        let xp_x = 8.0 + (bar_w + 36.0) * 3.0;
        self.push_text("XP", xp_x, bar_y - 2.0, [0.9, 0.75, 0.1, 1.0]);
        self.push_quad(xp_x + 24.0, bar_y, bar_w, bar_h, [0.1, 0.08, 0.02, 1.0]);
        let xp_ratio = if hud.xp_to_next > 0 { (hud.xp as f32 / hud.xp_to_next as f32).clamp(0.0, 1.0) } else { 1.0 };
        let xp_fill = (bar_w * xp_ratio).max(0.0);
        self.push_quad(xp_x + 24.0, bar_y, xp_fill, bar_h, [0.85, 0.65, 0.05, 1.0]);
        self.push_quad(xp_x + 24.0, bar_y, bar_w, 1.0, [0.5, 0.35, 0.0, 1.0]);
        self.push_quad(xp_x + 24.0, bar_y + bar_h - 1.0, bar_w, 1.0, [0.5, 0.35, 0.0, 1.0]);
        let xp_str = format!("{}/{}", hud.xp, hud.xp_to_next);
        self.push_text(&xp_str, xp_x + 24.0 + bar_w * 0.5 - 16.0, bar_y + 13.0, [1.0, 1.0, 1.0, 1.0]);

        // ── Ligne aptitudes rapides sous les barres ───────────────────────────
        let stat_y = bar_y + bar_h + 6.0;
        let atk_str = format!("Atk:{}", hud.atk_apt);
        let esq_str = format!("Esq:{}", hud.esq_apt);
        let par_str = format!("Par:{}", hud.par_apt);
        let dmg_str = format!("Dmg:~{}", hud.base_dmg);
        self.push_text(&atk_str, hp_x,          stat_y + 12.0, [1.0, 0.6, 0.2, 1.0]);
        self.push_text(&esq_str, hp_x + 80.0,   stat_y + 12.0, [0.3, 0.9, 0.5, 1.0]);
        self.push_text(&par_str, hp_x + 160.0,  stat_y + 12.0, [0.4, 0.7, 1.0, 1.0]);
        self.push_text(&dmg_str, hp_x + 240.0,  stat_y + 12.0, [1.0, 0.4, 0.1, 1.0]);
    }

    // ── Barre de navigation haut ──────────────────────────────────────────────

    fn draw_nav_bar(&mut self, vw: f32, gui_state: &GuiState) {
        // Fond barre nav
        self.push_quad(0.0, 0.0, vw, BTN_Y + BTN_H + 8.0, [0.04, 0.04, 0.08, 0.88]);
        self.push_quad(0.0, BTN_Y + BTN_H + 7.0, vw, 1.0, [0.22, 0.22, 0.3, 1.0]);

        // Bouton Personnage [P]
        let pcolor = if gui_state.show_character { [0.25, 0.4, 0.25, 1.0] } else { [0.12, 0.12, 0.18, 0.95] };
        self.push_quad(BTN_PERSONNAGE_X, BTN_Y, BTN_W, BTN_H, pcolor);
        self.push_quad(BTN_PERSONNAGE_X, BTN_Y, BTN_W, 1.0, [0.4, 0.5, 0.4, 1.0]);
        self.push_quad(BTN_PERSONNAGE_X, BTN_Y + BTN_H - 1.0, BTN_W, 1.0, [0.4, 0.5, 0.4, 1.0]);
        self.push_text("[P] Personnage", BTN_PERSONNAGE_X + 8.0, BTN_Y + BTN_H - 8.0, [0.9, 0.95, 0.9, 1.0]);

        // Bouton Compétences [C]
        let ccolor = if gui_state.show_skills { [0.25, 0.25, 0.45, 1.0] } else { [0.12, 0.12, 0.18, 0.95] };
        self.push_quad(BTN_COMPETENCES_X, BTN_Y, BTN_W, BTN_H, ccolor);
        self.push_quad(BTN_COMPETENCES_X, BTN_Y, BTN_W, 1.0, [0.35, 0.35, 0.55, 1.0]);
        self.push_quad(BTN_COMPETENCES_X, BTN_Y + BTN_H - 1.0, BTN_W, 1.0, [0.35, 0.35, 0.55, 1.0]);
        self.push_text("[C] Competences", BTN_COMPETENCES_X + 8.0, BTN_Y + BTN_H - 8.0, [0.85, 0.85, 1.0, 1.0]);

        // Bouton Inventaire [I]
        let icolor = if gui_state.show_inventory { [0.4, 0.3, 0.15, 1.0] } else { [0.12, 0.12, 0.18, 0.95] };
        self.push_quad(BTN_INVENTAIRE_X, BTN_Y, BTN_W, BTN_H, icolor);
        self.push_quad(BTN_INVENTAIRE_X, BTN_Y, BTN_W, 1.0, [0.55, 0.4, 0.15, 1.0]);
        self.push_quad(BTN_INVENTAIRE_X, BTN_Y + BTN_H - 1.0, BTN_W, 1.0, [0.55, 0.4, 0.15, 1.0]);
        self.push_text("[I] Inventaire", BTN_INVENTAIRE_X + 8.0, BTN_Y + BTN_H - 8.0, [1.0, 0.9, 0.7, 1.0]);
    }

    // ── Chrome générique de fenêtre (fond + bordure + barre de titre + bouton X) ──

    /// Dessine le chrome d'une fenêtre déplaçable : fond, bordure, barre de titre avec
    /// indicateur de drag (≡) à gauche et bouton [X] à droite.
    fn draw_window(
        &mut self,
        wx: f32, wy: f32,
        ww: f32, wh: f32,
        title: &str,
        title_bg: [f32; 4],
        border_col: [f32; 4],
    ) {
        // Fond principal
        self.push_quad(wx, wy, ww, wh, [0.06, 0.06, 0.1, 0.96]);
        // Bordures (4 cotés, 2px)
        self.push_quad(wx,          wy,          ww, 2.0,  border_col);
        self.push_quad(wx,          wy + wh - 2.0, ww, 2.0, border_col);
        self.push_quad(wx,          wy,          2.0, wh,   border_col);
        self.push_quad(wx + ww - 2.0, wy,        2.0, wh,   border_col);
        // Barre de titre
        self.push_quad(wx, wy, ww, WIN_TITLE_H, title_bg);
        // Indicateur drag (trois tirets courts) — à gauche
        self.push_quad(wx + 6.0, wy + 7.0,  14.0, 2.0, [0.6, 0.6, 0.6, 0.7]);
        self.push_quad(wx + 6.0, wy + 11.0, 14.0, 2.0, [0.6, 0.6, 0.6, 0.7]);
        self.push_quad(wx + 6.0, wy + 15.0, 14.0, 2.0, [0.6, 0.6, 0.6, 0.7]);
        // Texte titre
        self.push_text(title, wx + 28.0, wy + WIN_TITLE_H - 6.0, [0.9, 0.95, 0.9, 1.0]);
        // Bouton X (fermeture)
        self.push_quad(wx + ww - WIN_CLOSE_W, wy, WIN_CLOSE_W, WIN_TITLE_H, [0.55, 0.1, 0.1, 0.9]);
        self.push_text("X", wx + ww - WIN_CLOSE_W + 7.0, wy + WIN_TITLE_H - 6.0, [1.0, 0.6, 0.6, 1.0]);
    }

    // ── Contenu fenêtre Personnage ────────────────────────────────────────────

    fn draw_character_content(&mut self, wx: f32, wy: f32, hud: &HudData) {
        const LH: f32 = 20.0;   // line height
        const C2: f32 = 150.0;  // x décalage colonne droite
        let tx = wx + 14.0;
        let mut ty = wy + WIN_TITLE_H + 10.0;
        let col_lbl = [0.75, 0.75, 0.75, 1.0];
        let col_sep = [0.2, 0.22, 0.28, 1.0];

        // ── En-tête niveau ────────────────────────────────────────────────────
        let level_str = format!("Niveau : {}   XP : {}/{}", hud.level, hud.xp, hud.xp_to_next);
        self.push_text(&level_str, tx, ty, [1.0, 0.85, 0.2, 1.0]);
        ty += LH + 4.0;

        // ── Section : 10 Caractéristiques (2 colonnes) ───────────────────────
        self.push_quad(tx, ty, CHAR_WIN_W - 28.0, 1.0, col_sep);
        ty += 6.0;
        self.push_text("Caracteristiques", tx, ty, [0.6, 0.85, 0.6, 1.0]);
        ty += LH;

        // Col gauche : For Con Agi Dex Per | Col droite : Vol Int Sag Cha Luk
        let left: [(&str, u8); 5] = [
            ("For", hud.for_), ("Con", hud.con), ("Agi", hud.agi),
            ("Dex", hud.dex), ("Per", hud.per),
        ];
        let right: [(&str, u8); 5] = [
            ("Vol", hud.vol), ("Int", hud.int), ("Sag", hud.sag),
            ("Cha", hud.cha), ("Luk", hud.luk),
        ];
        for i in 0..5 {
            let (ll, lv) = left[i];
            let (rl, rv) = right[i];
            self.push_text(ll, tx, ty, col_lbl);
            let lv_str = format!("{}", lv);
            self.push_text(&lv_str, tx + 36.0, ty, [0.9, 0.95, 1.0, 1.0]);
            self.push_text(rl, tx + C2, ty, col_lbl);
            let rv_str = format!("{}", rv);
            self.push_text(&rv_str, tx + C2 + 36.0, ty, [0.9, 0.95, 1.0, 1.0]);
            ty += LH;
        }
        ty += 2.0;

        // ── Section : Stats dérivées ──────────────────────────────────────────
        self.push_quad(tx, ty, CHAR_WIN_W - 28.0, 1.0, col_sep);
        ty += 6.0;
        self.push_text("Stats derivees", tx, ty, [0.6, 0.85, 0.6, 1.0]);
        ty += LH;

        let derived: [(&str, String, &str, String); 3] = [
            ("PV",  format!("{}/{}", hud.hp,  hud.hp_max),  "PM",  format!("{}/{}", hud.mana, hud.mana_max)),
            ("End", format!("{}/{}", hud.end, hud.end_max), "Aggr",format!("{}", hud.for_ as i32 + hud.con as i32)),
            ("Pds", format!("{}", (hud.for_ as i32 + hud.con as i32) * 5), "", String::new()),
        ];
        for (ll, lv, rl, rv) in &derived {
            self.push_text(ll, tx, ty, col_lbl);
            self.push_text(lv, tx + 36.0, ty, [0.9, 0.95, 1.0, 1.0]);
            if !rl.is_empty() {
                self.push_text(rl, tx + C2, ty, col_lbl);
                self.push_text(rv, tx + C2 + 40.0, ty, [0.9, 0.95, 1.0, 1.0]);
            }
            ty += LH;
        }
        ty += 2.0;

        // ── Section : Aptitudes de combat ─────────────────────────────────────
        self.push_quad(tx, ty, CHAR_WIN_W - 28.0, 1.0, col_sep);
        ty += 6.0;
        self.push_text("Aptitudes combat", tx, ty, [0.6, 0.85, 0.6, 1.0]);
        ty += LH;

        let col_val = [0.9, 0.85, 0.6, 1.0];
        let apts: [(&str, i32); 11] = [
            ("atk (toucher)",    hud.atk_apt),
            ("atk speed",        hud.atk_speed_apt),
            ("jet (lancer)",     hud.jet_apt),
            ("esq (esquive)",    hud.esq_apt),
            ("par (parade)",     hud.par_apt),
            ("tirC (arc)",       hud.tir_corde_apt),
            ("tirP (pistolet)",  hud.tir_poing_apt),
            ("tirE (arbalete)",  hud.tir_epaule_apt),
            ("magie",            hud.magie_apt),
            ("cast speed",       hud.cast_speed_apt),
            ("Dmg (base CaC)",   hud.base_dmg),
        ];
        for (lbl, val) in &apts {
            self.push_text(lbl, tx, ty, col_lbl);
            let v_str = format!("{}", val);
            self.push_text(&v_str, tx + 160.0, ty, col_val);
            ty += LH;
        }
        let _ = ty;
    }

    // ── Contenu fenêtre Competences ───────────────────────────────────────────

    fn draw_skills_content(&mut self, wx: f32, wy: f32, hud: &HudData) {
        let tx = wx + 14.0;
        let mut ty = wy + WIN_TITLE_H + 10.0;
        let lh = 14.0;

        let col_cat   = [0.6, 0.85, 0.6, 1.0];
        let col_name  = [0.9, 0.9, 1.0, 1.0];
        let col_val   = [0.9, 0.85, 0.6, 1.0];
        let col_cap   = [0.5, 0.5, 0.7, 1.0];
        let col_sep   = [0.3, 0.3, 0.5, 1.0];

        self.push_text("-- Competences (C=carac) --", tx, ty, col_cat);
        ty += lh + 2.0;

        let mut last_cat: &str = "";
        for &(cat, name, val, cap) in &hud.competences {
            // Nouvelle catégorie → séparateur + titre
            if cat != last_cat {
                if !last_cat.is_empty() { ty += 2.0; }
                self.push_quad(tx, ty, SKILLS_WIN_W - 28.0, 1.0, col_sep);
                ty += 4.0;
                self.push_text(cat, tx, ty, col_cat);
                ty += lh;
                last_cat = cat;
            }
            // Nom de la compétence
            self.push_text(name, tx + 8.0, ty, col_name);
            // Valeur / Cap
            let val_str = format!("{}/{}", val, cap);
            self.push_text(&val_str, tx + 200.0, ty, col_val);
            // Barre de progression (mini)
            let bar_x = tx + 270.0;
            let bar_w = 80.0;
            let bar_h = 8.0;
            let ratio = if cap > 0 { (val as f32 / cap as f32).clamp(0.0, 1.0) } else { 0.0 };
            self.push_quad(bar_x, ty - 2.0, bar_w, bar_h, [0.12, 0.12, 0.18, 1.0]);
            self.push_quad(bar_x, ty - 2.0, bar_w * ratio, bar_h, [0.3, 0.6, 0.9, 1.0]);
            ty += lh;
        }

        // Luk info (pas de compétence)
        ty += 4.0;
        self.push_quad(tx, ty, SKILLS_WIN_W - 28.0, 1.0, col_sep);
        ty += 4.0;
        self.push_text("Chance (Luk)", tx, ty, col_cat);
        ty += lh;
        self.push_text("Pas de competence", tx + 8.0, ty, col_cap);
        self.push_text("(seuil crit, mod de)", tx + 8.0, ty + lh, col_cap);
        let _ = ty;
    }

    // ── Contenu fenêtre Inventaire ────────────────────────────────────────────

    fn draw_inventory_content(&mut self, wx: f32, wy: f32) {
        // Separateur vertical Equipement | Sac
        let sep_x = wx + 195.0;
        self.push_quad(sep_x, wy + WIN_TITLE_H + 2.0, 2.0, INV_WIN_H - WIN_TITLE_H - 4.0,
                       [0.3, 0.22, 0.08, 1.0]);

        // ── Section gauche : emplacements equipement ──────────────────────────
        let eq_x = wx + 10.0;
        let mut eq_y = wy + WIN_TITLE_H + 8.0;
        self.push_text("-- Equipement --", eq_x, eq_y, [0.85, 0.7, 0.3, 1.0]);
        eq_y += 18.0;

        let slot_size = 44.0;
        // (label_ASCII, dx_offset, dy_offset)
        let slots = [
            ("Tete",     0.0,  0.0),
            ("Torse",    0.0,  50.0),
            ("Jambes",   0.0,  100.0),
            ("Bottes",   0.0,  150.0),
            ("Arme D",   0.0,  220.0),
            ("Bouclier", 55.0, 220.0),
            ("Gants",    0.0,  270.0),
            ("Anneau",   0.0,  320.0),
        ];
        for (label, dx, dy) in &slots {
            let sx = eq_x + dx;
            let sy = eq_y + dy;
            self.push_quad(sx, sy, slot_size, slot_size, [0.1, 0.08, 0.05, 1.0]);
            self.push_quad(sx, sy,              slot_size, 1.0, [0.45, 0.32, 0.12, 1.0]);
            self.push_quad(sx, sy + slot_size - 1.0, slot_size, 1.0, [0.45, 0.32, 0.12, 1.0]);
            self.push_quad(sx, sy,              1.0, slot_size, [0.45, 0.32, 0.12, 1.0]);
            self.push_quad(sx + slot_size - 1.0, sy, 1.0, slot_size, [0.45, 0.32, 0.12, 1.0]);
            self.push_text(label, sx + 4.0, sy + slot_size - 6.0, [0.75, 0.62, 0.38, 1.0]);
        }

        // ── Section droite : grille inventaire 6×6 ───────────────────────────
        let inv_x = sep_x + 10.0;
        let inv_y = wy + WIN_TITLE_H + 8.0;
        self.push_text("-- Sac a dos --", inv_x, inv_y, [0.85, 0.7, 0.3, 1.0]);

        let cell = 54.0;
        let gap  = 2.0;
        for row in 0..6i32 {
            for col in 0..6i32 {
                let cx = inv_x + col as f32 * (cell + gap);
                let cy = inv_y + 18.0 + row as f32 * (cell + gap);
                self.push_quad(cx, cy, cell, cell, [0.09, 0.07, 0.04, 1.0]);
                self.push_quad(cx, cy,             cell, 1.0,   [0.32, 0.23, 0.08, 1.0]);
                self.push_quad(cx, cy + cell - 1.0, cell, 1.0,  [0.32, 0.23, 0.08, 1.0]);
                self.push_quad(cx, cy,             1.0,  cell,  [0.32, 0.23, 0.08, 1.0]);
                self.push_quad(cx + cell - 1.0, cy, 1.0, cell,  [0.32, 0.23, 0.08, 1.0]);
            }
        }
    }

    /// Soumet du texte dans le text_instances buffer (screen space)
    fn push_text(&mut self, text: &str, x: f32, y: f32, color: [f32; 4]) {
        let mut cursor = x;
        let baseline = y;
        for ch in text.chars() {
            if self.text_instances.len() >= MAX_TEXT_CHARS {
                break;
            }
            let glyph = match self.glyph_cache.glyphs.get(&ch) {
                Some(g) => g,
                None => continue,
            };
            if glyph.width > 0.0 {
                self.text_instances.push(InstanceRaw {
                    screen_pos: [cursor, baseline - glyph.height - glyph.offset_y],
                    size: [glyph.width, glyph.height],
                    uv_rect: [glyph.u_min, glyph.v_min, glyph.u_max, glyph.v_max],
                });
            }
            cursor += glyph.advance;
        }
        let _ = color; // Le shader texte utilise la couleur via le canal alpha R8 — TODO: passer via uniform
    }
}
