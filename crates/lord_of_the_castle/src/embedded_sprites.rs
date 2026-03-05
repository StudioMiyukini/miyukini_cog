//! Sprites intégrés directement dans le binaire via include_bytes!
//! Cela permet d'afficher les sprites sans dépendre du système de fichiers.

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use std::sync::LazyLock;

/// Macro pour créer une data URL à partir de bytes PNG
macro_rules! png_data_url {
    ($bytes:expr) => {{
        let b64 = BASE64.encode($bytes);
        format!("data:image/png;base64,{}", b64)
    }};
}

// ═══════════════════════════════════════════════════════════════════════
// SPRITES DU JOUEUR (KNIGHT)
// ═══════════════════════════════════════════════════════════════════════

/// Knight Idle (6 frames, 600x100)
pub static KNIGHT_IDLE: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Knight/Knight/Knight-Idle.png"))
});

/// Knight Walk (6 frames, 600x100)
pub static KNIGHT_WALK: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Knight/Knight/Knight-Walk.png"))
});

/// Knight Attack (4 frames, 400x100)
pub static KNIGHT_ATTACK: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Knight/Knight/Knight-Attack01.png"))
});

/// Knight Hurt (2 frames, 200x100)
pub static KNIGHT_HURT: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Knight/Knight/Knight-Hurt.png"))
});

/// Knight Death (6 frames, 600x100)
pub static KNIGHT_DEATH: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Knight/Knight/Knight-Death.png"))
});

// ═══════════════════════════════════════════════════════════════════════
// SPRITES DES ENNEMIS NORMAUX (SKELETON)
// ═══════════════════════════════════════════════════════════════════════

/// Skeleton Idle (6 frames)
pub static SKELETON_IDLE: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Skeleton/Skeleton/Skeleton-Idle.png"))
});

/// Skeleton Walk (6 frames)
pub static SKELETON_WALK: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Skeleton/Skeleton/Skeleton-Walk.png"))
});

/// Skeleton Attack (4 frames)
pub static SKELETON_ATTACK: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Skeleton/Skeleton/Skeleton-Attack01.png"))
});

/// Skeleton Death (6 frames)
pub static SKELETON_DEATH: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Skeleton/Skeleton/Skeleton-Death.png"))
});

// ═══════════════════════════════════════════════════════════════════════
// SPRITES DES MINI-BOSS (ORC)
// ═══════════════════════════════════════════════════════════════════════

/// Orc Idle (6 frames)
pub static ORC_IDLE: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!(
        "../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Orc/Orc/Orc-Idle.png"
    ))
});

/// Orc Walk (6 frames)
pub static ORC_WALK: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!(
        "../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Orc/Orc/Orc-Walk.png"
    ))
});

/// Orc Attack (4 frames)
pub static ORC_ATTACK: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Orc/Orc/Orc-Attack01.png"))
});

/// Orc Death (6 frames)
pub static ORC_DEATH: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Orc/Orc/Orc-Death.png"))
});

// ═══════════════════════════════════════════════════════════════════════
// SPRITES DES BOSS (WEREBEAR)
// ═══════════════════════════════════════════════════════════════════════

/// Werebear Idle (6 frames)
pub static WEREBEAR_IDLE: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Werebear/Werebear/Werebear-Idle.png"))
});

/// Werebear Walk (6 frames)
pub static WEREBEAR_WALK: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Werebear/Werebear/Werebear-Walk.png"))
});

/// Werebear Attack (5 frames)
pub static WEREBEAR_ATTACK: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Werebear/Werebear/Werebear-Attack01.png"))
});

/// Werebear Death (6 frames)
pub static WEREBEAR_DEATH: LazyLock<String> = LazyLock::new(|| {
    png_data_url!(include_bytes!("../../../images/sprites/Tiny RPG Character Asset/Characters(100x100)/Werebear/Werebear/Werebear-Death.png"))
});

// ═══════════════════════════════════════════════════════════════════════
// HELPERS POUR LE RENDU CSS
// ═══════════════════════════════════════════════════════════════════════

/// Configuration d'un sprite pour le rendu CSS.
pub struct EmbeddedSprite {
    /// Data URL du sprite (base64)
    pub data_url: &'static LazyLock<String>,
    /// Nombre de frames
    pub frame_count: usize,
    /// Taille d'une frame (largeur, hauteur)
    pub frame_size: (u32, u32),
    /// Durée d'une frame en secondes
    pub frame_duration_s: f32,
}

impl EmbeddedSprite {
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

    /// Génère le style CSS complet pour afficher ce sprite.
    #[must_use]
    pub fn css_style(&self, frame_index: usize, flip_h: bool) -> String {
        let bg_pos = self.background_position_for_frame(frame_index);
        let bg_size = self.background_size();
        let transform = if flip_h { "transform:scaleX(-1);" } else { "" };

        format!(
            "background-image:url('{}');background-position:{};background-size:{};background-repeat:no-repeat;{transform}image-rendering:pixelated;",
            self.data_url.as_str(),
            bg_pos,
            bg_size,
        )
    }
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
// CONFIGURATIONS DES SPRITES
// ═══════════════════════════════════════════════════════════════════════

/// Sprites du joueur
pub mod player {
    use super::*;

    pub const IDLE: EmbeddedSprite = EmbeddedSprite {
        data_url: &KNIGHT_IDLE,
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };

    pub const WALK: EmbeddedSprite = EmbeddedSprite {
        data_url: &KNIGHT_WALK,
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.1,
    };

    pub const ATTACK: EmbeddedSprite = EmbeddedSprite {
        data_url: &KNIGHT_ATTACK,
        frame_count: 4,
        frame_size: (100, 100),
        frame_duration_s: 0.1,
    };

    pub const DEATH: EmbeddedSprite = EmbeddedSprite {
        data_url: &KNIGHT_DEATH,
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };
}

/// Sprites des ennemis normaux (Skeleton)
pub mod enemy_normal {
    use super::*;

    pub const IDLE: EmbeddedSprite = EmbeddedSprite {
        data_url: &SKELETON_IDLE,
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.2,
    };

    pub const WALK: EmbeddedSprite = EmbeddedSprite {
        data_url: &SKELETON_WALK,
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.12,
    };

    pub const DEATH: EmbeddedSprite = EmbeddedSprite {
        data_url: &SKELETON_DEATH,
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };
}

/// Sprites des mini-boss (Orc)
pub mod enemy_miniboss {
    use super::*;

    pub const IDLE: EmbeddedSprite = EmbeddedSprite {
        data_url: &ORC_IDLE,
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.2,
    };

    pub const WALK: EmbeddedSprite = EmbeddedSprite {
        data_url: &ORC_WALK,
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.12,
    };

    pub const DEATH: EmbeddedSprite = EmbeddedSprite {
        data_url: &ORC_DEATH,
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };
}

/// Sprites des boss (Werebear)
pub mod enemy_boss {
    use super::*;

    pub const IDLE: EmbeddedSprite = EmbeddedSprite {
        data_url: &WEREBEAR_IDLE,
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.2,
    };

    pub const WALK: EmbeddedSprite = EmbeddedSprite {
        data_url: &WEREBEAR_WALK,
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.12,
    };

    pub const DEATH: EmbeddedSprite = EmbeddedSprite {
        data_url: &WEREBEAR_DEATH,
        frame_count: 6,
        frame_size: (100, 100),
        frame_duration_s: 0.15,
    };
}
