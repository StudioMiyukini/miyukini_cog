//! Outil COG spritesheet — découpage et chargement de sprites (UI Dioxus/Blitz).
//!
//! Permet de charger une texture (strip horizontal), de calculer le rect UV d'une frame.
//! Configuration des sprites pour le joueur et les ennemis.
//!
//! @id: lord_of_the_castle_spritesheet
//! @do: load_and_draw_spritesheet_frames
//! @role: ui
//! @layer: infra
//! @human: Chargement PNG, découpage strip horizontal ; configuration sprites personnages.

use image::RgbaImage;
use std::path::Path;

/// Rect UV (0–1) pour une frame de spritesheet.
#[derive(Debug, Clone, Copy)]
pub struct UvRect {
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
}

/// Position 2D écran ou monde.
#[derive(Debug, Clone, Copy)]
pub struct Pos2 {
    pub x: f32,
    pub y: f32,
}

/// Charge une image PNG depuis le chemin (RGBA).
/// Retourne `None` si le fichier est absent ou invalide.
#[must_use]
pub fn load_image_from_path(path: &Path) -> Option<RgbaImage> {
    let data = std::fs::read(path).ok()?;
    let img = image::load_from_memory(&data).ok()?;
    Some(img.to_rgba8())
}

/// Description d'un spritesheet horizontal : nombre de frames, taille d'une frame.
#[derive(Debug, Clone)]
pub struct SpritesheetDesc {
    /// Nombre de frames (ex. 6 pour Knight-Idle).
    pub frame_count: usize,
    /// Largeur d'une frame en pixels (texture).
    pub frame_width: u32,
    /// Hauteur d'une frame en pixels.
    pub frame_height: u32,
}

impl SpritesheetDesc {
    /// Crée une description pour un strip horizontal (toutes les frames ont la même taille).
    #[must_use]
    pub fn horizontal(total_width: u32, total_height: u32, frame_count: usize) -> Self {
        let frame_width = total_width / frame_count.max(1) as u32;
        Self {
            frame_count,
            frame_width,
            frame_height: total_height,
        }
    }

    /// Retourne le rect UV (0–1) pour la frame d'index donné (strip horizontal).
    #[must_use]
    pub fn frame_uv_rect(&self, frame_index: usize) -> UvRect {
        let n = self.frame_count.max(1);
        let i = frame_index % n;
        let u0 = (i as f32) / (n as f32);
        let u1 = ((i + 1) as f32) / (n as f32);
        UvRect {
            u0,
            v0: 0.0,
            u1,
            v1: 1.0,
        }
    }

    /// Taille d'une frame en pixels (pour dimensionner l'affichage à l'écran).
    #[must_use]
    pub fn frame_size(&self) -> (f32, f32) {
        (self.frame_width as f32, self.frame_height as f32)
    }
}

/// Dessine une frame de spritesheet centrée sur (center_screen_x, center_screen_y).
/// Stub après migration eGUI → Dioxus : à brancher sur un rendu canvas/Blitz.
/// La hitbox reste inchangée ; le sprite est affiché par-dessus.
/// Si `flip_h` est true, la frame est affichée en miroir horizontal.
#[allow(unused_variables)]
pub fn draw_sprite_frame(
    uv_rect: UvRect,
    center_screen: Pos2,
    frame_size: (f32, f32),
    flip_h: bool,
) {
    // Rendu à implémenter avec Dioxus/Blitz (canvas ou texture 2D).
}

// ═══════════════════════════════════════════════════════════════════════
// CONFIGURATION DES SPRITES — Tiny RPG Character Asset
// ═══════════════════════════════════════════════════════════════════════

/// Chemin de base vers les sprites (relatif au dossier racine).
pub const SPRITES_BASE_PATH: &str = "images/sprites/Tiny RPG Character Asset/Characters(100x100)";

/// Configuration d'un sprite animé (strip horizontal).
#[derive(Debug, Clone)]
pub struct SpriteConfig {
    /// Chemin relatif vers le fichier PNG (depuis SPRITES_BASE_PATH).
    pub path: &'static str,
    /// Nombre de frames dans le strip.
    pub frame_count: usize,
    /// Taille d'une frame (largeur, hauteur) en pixels.
    pub frame_size: (u32, u32),
    /// Durée d'une frame en secondes (pour l'animation).
    pub frame_duration_s: f32,
}

impl SpriteConfig {
    /// Calcule la position CSS background-position pour une frame donnée.
    #[must_use]
    pub fn background_position_for_frame(&self, frame_index: usize) -> String {
        let i = frame_index % self.frame_count.max(1);
        let x_offset = -(i as i32 * self.frame_size.0 as i32);
        format!("{}px 0px", x_offset)
    }

    /// Calcule la taille CSS background-size (largeur totale du strip).
    #[must_use]
    pub fn background_size(&self) -> String {
        let total_width = self.frame_count as u32 * self.frame_size.0;
        format!("{}px {}px", total_width, self.frame_size.1)
    }
}

/// Sprites du joueur (Knight).
pub mod player_sprites {
    use super::SpriteConfig;

    pub const IDLE: SpriteConfig = SpriteConfig {
        path: "Knight/Knight/Knight-Idle.png",
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };

    pub const WALK: SpriteConfig = SpriteConfig {
        path: "Knight/Knight/Knight-Walk.png",
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.1,
    };

    pub const ATTACK: SpriteConfig = SpriteConfig {
        path: "Knight/Knight/Knight-Attack01.png",
        frame_count: 4,
        frame_size: (100, 100),
        frame_duration_s: 0.1,
    };

    pub const HURT: SpriteConfig = SpriteConfig {
        path: "Knight/Knight/Knight-Hurt.png",
        frame_count: 2,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };

    pub const DEATH: SpriteConfig = SpriteConfig {
        path: "Knight/Knight/Knight-Death.png",
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };
}

/// Sprites des ennemis normaux (Skeleton).
pub mod enemy_normal_sprites {
    use super::SpriteConfig;

    pub const IDLE: SpriteConfig = SpriteConfig {
        path: "Skeleton/Skeleton/Skeleton-Idle.png",
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.2,
    };

    pub const WALK: SpriteConfig = SpriteConfig {
        path: "Skeleton/Skeleton/Skeleton-Walk.png",
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.12,
    };

    pub const ATTACK: SpriteConfig = SpriteConfig {
        path: "Skeleton/Skeleton/Skeleton-Attack01.png",
        frame_count: 4,
        frame_size: (100, 100),
        frame_duration_s: 0.1,
    };

    pub const HURT: SpriteConfig = SpriteConfig {
        path: "Skeleton/Skeleton/Skeleton-Hurt.png",
        frame_count: 2,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };

    pub const DEATH: SpriteConfig = SpriteConfig {
        path: "Skeleton/Skeleton/Skeleton-Death.png",
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };
}

/// Sprites des mini-boss (Orc).
pub mod enemy_miniboss_sprites {
    use super::SpriteConfig;

    pub const IDLE: SpriteConfig = SpriteConfig {
        path: "Orc/Orc/Orc-Idle.png",
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.2,
    };

    pub const WALK: SpriteConfig = SpriteConfig {
        path: "Orc/Orc/Orc-Walk.png",
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.12,
    };

    pub const ATTACK: SpriteConfig = SpriteConfig {
        path: "Orc/Orc/Orc-Attack01.png",
        frame_count: 4,
        frame_size: (100, 100),
        frame_duration_s: 0.1,
    };

    pub const HURT: SpriteConfig = SpriteConfig {
        path: "Orc/Orc/Orc-Hurt.png",
        frame_count: 2,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };

    pub const DEATH: SpriteConfig = SpriteConfig {
        path: "Orc/Orc/Orc-Death.png",
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };
}

/// Sprites des boss (Werebear).
pub mod enemy_boss_sprites {
    use super::SpriteConfig;

    pub const IDLE: SpriteConfig = SpriteConfig {
        path: "Werebear/Werebear/Werebear-Idle.png",
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.2,
    };

    pub const WALK: SpriteConfig = SpriteConfig {
        path: "Werebear/Werebear/Werebear-Walk.png",
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.12,
    };

    pub const ATTACK: SpriteConfig = SpriteConfig {
        path: "Werebear/Werebear/Werebear-Attack01.png",
        frame_count: 5,
        frame_size: (100, 100),
        frame_duration_s: 0.1,
    };

    pub const HURT: SpriteConfig = SpriteConfig {
        path: "Werebear/Werebear/Werebear-Hurt.png",
        frame_count: 2,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };

    pub const DEATH: SpriteConfig = SpriteConfig {
        path: "Werebear/Werebear/Werebear-Death.png",
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };
}

/// Calcule le chemin complet vers un sprite.
#[must_use]
pub fn sprite_full_path(relative_path: &str) -> String {
    format!("{}/{}", SPRITES_BASE_PATH, relative_path)
}

/// Calcule l'index de frame pour une animation en boucle.
#[must_use]
pub fn animation_frame_index(elapsed_s: f32, frame_count: usize, frame_duration_s: f32) -> usize {
    if frame_count == 0 || frame_duration_s <= 0.0 {
        return 0;
    }
    let total_duration = frame_count as f32 * frame_duration_s;
    let t = elapsed_s % total_duration;
    let frame = (t / frame_duration_s).floor() as usize;
    frame.min(frame_count - 1)
}

// ═══════════════════════════════════════════════════════════════════════
// CHARGEMENT DES SPRITES EN BASE64 POUR DIOXUS DESKTOP
// ═══════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// Cache global des sprites encodés en base64.
static SPRITE_CACHE: LazyLock<RwLock<HashMap<String, String>>> = LazyLock::new(|| {
    RwLock::new(HashMap::new())
});

/// Charge un sprite depuis le disque et retourne sa data URL base64.
/// Le résultat est mis en cache pour éviter de recharger le même fichier.
pub fn load_sprite_as_data_url(base_path: &Path, sprite_config: &SpriteConfig) -> Option<String> {
    let full_path = base_path.join(sprite_full_path(sprite_config.path));
    let cache_key = full_path.to_string_lossy().to_string();
    
    // Vérifier le cache d'abord
    if let Ok(cache) = SPRITE_CACHE.read() {
        if let Some(data_url) = cache.get(&cache_key) {
            return Some(data_url.clone());
        }
    }
    
    // Charger le fichier
    let data = std::fs::read(&full_path).ok()?;
    
    // Encoder en base64
    let b64 = BASE64.encode(&data);
    let data_url = format!("data:image/png;base64,{}", b64);
    
    // Mettre en cache
    if let Ok(mut cache) = SPRITE_CACHE.write() {
        cache.insert(cache_key, data_url.clone());
    }
    
    Some(data_url)
}

/// Charge tous les sprites nécessaires au démarrage du jeu.
/// Appeler cette fonction une seule fois au lancement pour pré-charger le cache.
pub fn preload_all_sprites(base_path: &Path) {
    let sprites_to_load = [
        &player_sprites::IDLE,
        &player_sprites::WALK,
        &player_sprites::ATTACK,
        &player_sprites::HURT,
        &player_sprites::DEATH,
        &enemy_normal_sprites::IDLE,
        &enemy_normal_sprites::WALK,
        &enemy_normal_sprites::ATTACK,
        &enemy_normal_sprites::HURT,
        &enemy_normal_sprites::DEATH,
        &enemy_miniboss_sprites::IDLE,
        &enemy_miniboss_sprites::WALK,
        &enemy_miniboss_sprites::ATTACK,
        &enemy_miniboss_sprites::HURT,
        &enemy_miniboss_sprites::DEATH,
        &enemy_boss_sprites::IDLE,
        &enemy_boss_sprites::WALK,
        &enemy_boss_sprites::ATTACK,
        &enemy_boss_sprites::HURT,
        &enemy_boss_sprites::DEATH,
    ];
    
    for sprite in sprites_to_load {
        if let Some(_) = load_sprite_as_data_url(base_path, sprite) {
            tracing::debug!("Sprite chargé: {}", sprite.path);
        } else {
            tracing::warn!("Impossible de charger le sprite: {}", sprite.path);
        }
    }
}

/// Génère le style CSS pour afficher un sprite avec animation.
/// Retourne None si le sprite n'a pas pu être chargé.
#[must_use]
pub fn sprite_css_style(
    base_path: &Path,
    sprite_config: &SpriteConfig,
    frame_index: usize,
    flip_h: bool,
) -> Option<String> {
    let data_url = load_sprite_as_data_url(base_path, sprite_config)?;
    let bg_pos = sprite_config.background_position_for_frame(frame_index);
    let bg_size = sprite_config.background_size();
    let transform = if flip_h { "scaleX(-1)" } else { "" };
    
    Some(format!(
        "background-image:url('{data_url}');background-position:{bg_pos};background-size:{bg_size};background-repeat:no-repeat;transform:{transform};image-rendering:pixelated;",
    ))
}
