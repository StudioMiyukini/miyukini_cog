// @id: MUIT-PaletteCog @do: cog-palette @role: token @layer: 5 @human: miyuk

use crate::color::Rgba;
use super::Palette;

/// COG "Miyukini Gaming" palette -- dark, modern, blue + sakura accents.
///
/// Reference: `BRIEF-miyuki-ui-lib-art-direction.md` section 2.2 (Lise).
pub const COG_PALETTE: Palette = Palette {
    // Backgrounds
    bg_base:      Rgba::new(14, 16, 21),     // #0e1015
    bg_primary:   Rgba::new(23, 26, 33),     // #171a21
    bg_secondary: Rgba::new(27, 40, 56),     // #1b2838
    bg_surface:   Rgba::new(30, 42, 58),     // #1e2a3a
    bg_elevated:  Rgba::new(36, 52, 71),     // #243447
    bg_overlay:   Rgba::new(42, 63, 95),     // #2a3f5f

    // Text
    text_high:      Rgba::new(255, 255, 255), // #ffffff
    text_primary:   Rgba::new(198, 212, 223), // #c6d4df
    text_secondary: Rgba::new(143, 152, 160), // #8f98a0
    text_muted:     Rgba::new(92, 104, 115),  // #5c6873

    // Accent primary (blue Miyukini)
    accent_primary:        Rgba::new(26, 159, 255),  // #1a9fff
    accent_primary_hover:  Rgba::new(71, 179, 255),  // #47b3ff
    accent_primary_active: Rgba::new(13, 138, 230),  // #0d8ae6
    accent_primary_subtle: Rgba::with_alpha(26, 159, 255, 26), // 10% alpha

    // Accent secondary (sakura Miyukini)
    accent_secondary:        Rgba::new(232, 160, 191), // #e8a0bf
    accent_secondary_hover:  Rgba::new(240, 184, 208), // #f0b8d0
    accent_secondary_subtle: Rgba::with_alpha(232, 160, 191, 26),

    // Semantic
    success:        Rgba::new(91, 163, 43),   // #5ba32b
    success_subtle: Rgba::with_alpha(91, 163, 43, 26),
    warning:        Rgba::new(255, 156, 26),  // #ff9c1a
    warning_subtle: Rgba::with_alpha(255, 156, 26, 26),
    error:          Rgba::new(224, 64, 64),   // #e04040
    error_subtle:   Rgba::with_alpha(224, 64, 64, 26),
    info:           Rgba::new(102, 192, 244), // #66c0f4
    info_subtle:    Rgba::with_alpha(102, 192, 244, 26),

    // Borders
    border_subtle:  Rgba::new(30, 42, 58),   // #1e2a3a
    border_default: Rgba::new(42, 63, 95),   // #2a3f5f
    border_strong:  Rgba::new(61, 90, 128),  // #3d5a80
    border_accent:  Rgba::new(26, 159, 255), // #1a9fff
};
