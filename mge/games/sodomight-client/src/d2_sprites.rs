// @id: Sodomight-Client-Sprites @do: sprite-loader @role: back-end @layer: 4 @human: miyuk
//! Sprite loader for Sodomight entities.
//!
//! Loads placeholder sprites from `assets/placeholders/`. Each entity type
//! has a single static RGBA PNG at one of five standard sizes:
//!
//! | Class     | Width | Height | Ratio      |
//! |-----------|-------|--------|------------|
//! | Small     | 128   | 128    | 1:1        |
//! | Medium    | 128   | 256    | 1:2        |
//! | Humanoid  | 128   | 384    | 1:3        |
//! | Large     | 192   | 384    | 1.5:2      |
//! | Boss      | 256   | 768    | 2:6        |

use mge_render::{GpuTexture, SpritePipeline};

/// Base width for the sprite size system (pixels).
pub const SPRITE_BASE: u32 = 128;

/// Sprite size class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpriteSize {
    /// 128x128 — critters, objects.
    Small,
    /// 128x256 — medium bipeds, lesser enemies.
    Medium,
    /// 128x384 — humanoids (player, NPCs).
    Humanoid,
    /// 192x384 — large creatures.
    Large,
    /// 256x768 — act bosses.
    Boss,
}

impl SpriteSize {
    /// Pixel dimensions `(width, height)` for this size class.
    #[must_use]
    pub const fn dimensions(self) -> (u32, u32) {
        match self {
            Self::Small    => (SPRITE_BASE, SPRITE_BASE),
            Self::Medium   => (SPRITE_BASE, SPRITE_BASE * 2),
            Self::Humanoid => (SPRITE_BASE, SPRITE_BASE * 3),
            Self::Large    => (SPRITE_BASE * 3 / 2, SPRITE_BASE * 3 / 2 * 2),
            Self::Boss     => (SPRITE_BASE * 2, SPRITE_BASE * 2 * 3),
        }
    }
}

// ---------------------------------------------------------------------------
// Asset loading
// ---------------------------------------------------------------------------

/// Build a list of candidate paths for a placeholder sprite.
fn placeholder_prefixes() -> Vec<String> {
    let mut prefixes = vec![
        "assets/placeholders/".to_string(),
        "mge/assets/placeholders/".to_string(),
        "../mge/assets/placeholders/".to_string(),
        "../../assets/placeholders/".to_string(),
    ];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            let from_exe = exe_dir.join("../../assets/placeholders/");
            prefixes.push(format!("{}/", from_exe.display()));
        }
    }
    prefixes
}

/// Load a placeholder RGBA PNG from disk.
fn load_placeholder(filename: &str) -> Option<image::RgbaImage> {
    for prefix in &placeholder_prefixes() {
        let full = format!("{prefix}{filename}");
        if let Ok(img) = image::open(&full) {
            let rgba = img.to_rgba8();
            tracing::info!(
                "Loaded placeholder: {full} ({}x{})",
                rgba.width(),
                rgba.height()
            );
            return Some(rgba);
        }
    }
    tracing::warn!("Placeholder not found: {filename}");
    None
}

// ---------------------------------------------------------------------------
// GPU sprite
// ---------------------------------------------------------------------------

/// A GPU-resident sprite loaded from a placeholder PNG.
pub struct Sprite {
    /// GPU texture (with bind group for rendering).
    pub texture: GpuTexture,
    /// Pixel width of this sprite.
    pub width: u32,
    /// Pixel height of this sprite.
    pub height: u32,
    /// Size class this sprite belongs to.
    pub size: SpriteSize,
}

/// Load a single placeholder sprite and upload to GPU.
fn load_sprite(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    pipeline: &SpritePipeline,
    filename: &str,
    label: &str,
    size: SpriteSize,
) -> Option<Sprite> {
    let img = load_placeholder(filename)?;
    let w = img.width();
    let h = img.height();
    let texture = GpuTexture::from_image(device, queue, pipeline, &img, label);
    Some(Sprite {
        texture,
        width: w,
        height: h,
        size,
    })
}

// ---------------------------------------------------------------------------
// All textures for Act 1
// ---------------------------------------------------------------------------

/// All entity sprites for Act 1.
pub struct EntitySprites {
    /// Player character (humanoid).
    pub player: Option<Sprite>,
    /// Fallen Shaman enemy (medium biped).
    pub fallen_shaman: Option<Sprite>,
    /// Spike Fiend / Quill Rat enemy (small beast).
    pub spike_fiend: Option<Sprite>,
    /// Andariel act boss.
    pub andariel: Option<Sprite>,
    /// Akara NPC (humanoid).
    pub akara: Option<Sprite>,
    /// Deckard Cain NPC (humanoid).
    pub deckard_cain: Option<Sprite>,
    /// Campfire object (small).
    pub fire: Option<Sprite>,
    /// Barrel object (small).
    pub barrel: Option<Sprite>,
}

impl EntitySprites {
    /// Load all placeholder sprites and upload to GPU.
    pub fn load(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pipeline: &SpritePipeline,
    ) -> Self {
        Self {
            player: load_sprite(
                device, queue, pipeline,
                "player.png", "Player", SpriteSize::Humanoid,
            ),
            fallen_shaman: load_sprite(
                device, queue, pipeline,
                "fallen_shaman.png", "Fallen Shaman", SpriteSize::Medium,
            ),
            spike_fiend: load_sprite(
                device, queue, pipeline,
                "spike_fiend.png", "Spike Fiend", SpriteSize::Small,
            ),
            andariel: load_sprite(
                device, queue, pipeline,
                "andariel.png", "Andariel", SpriteSize::Boss,
            ),
            akara: load_sprite(
                device, queue, pipeline,
                "akara.png", "Akara", SpriteSize::Humanoid,
            ),
            deckard_cain: load_sprite(
                device, queue, pipeline,
                "deckard_cain.png", "Deckard Cain", SpriteSize::Humanoid,
            ),
            fire: load_sprite(
                device, queue, pipeline,
                "fire.png", "Campfire", SpriteSize::Small,
            ),
            barrel: load_sprite(
                device, queue, pipeline,
                "barrel.png", "Barrel", SpriteSize::Small,
            ),
        }
    }
}

/// Compute the screen-space size for a sprite, applying a scale factor.
pub fn sprite_screen_size(frame_w: u32, frame_h: u32, base_scale: f32) -> (f32, f32) {
    (frame_w as f32 * base_scale, frame_h as f32 * base_scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_size_dimensions() {
        assert_eq!(SpriteSize::Small.dimensions(), (128, 128));
        assert_eq!(SpriteSize::Medium.dimensions(), (128, 256));
        assert_eq!(SpriteSize::Humanoid.dimensions(), (128, 384));
        assert_eq!(SpriteSize::Large.dimensions(), (192, 384));
        assert_eq!(SpriteSize::Boss.dimensions(), (256, 768));
    }

    #[test]
    fn test_sprite_screen_size() {
        let (w, h) = sprite_screen_size(128, 128, 1.0);
        assert!((w - 128.0).abs() < f32::EPSILON);
        assert!((h - 128.0).abs() < f32::EPSILON);

        let (w2, h2) = sprite_screen_size(128, 384, 0.5);
        assert!((w2 - 64.0).abs() < f32::EPSILON);
        assert!((h2 - 192.0).abs() < f32::EPSILON);
    }
}
