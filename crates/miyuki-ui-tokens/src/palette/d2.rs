// @id: MUIT-PaletteD2 @do: d2-palette @role: token @layer: 5 @human: miyuk

use super::Palette;
use crate::color::Rgba;

/// D2 "Sodomight Medieval" palette -- dark, warm, gold accents.
///
/// References:
/// - `BRIEF-miyuki-ui-lib-d2-analysis.md` section 2 (Fabrice)
/// - `BRIEF-miyuki-ui-lib-art-direction.md` section 3.2 (Lise)
pub const D2_PALETTE: Palette = Palette {
    // Backgrounds (medieval surfaces)
    bg_base: Rgba::new(5, 4, 2),         // #050402 -- void black
    bg_primary: Rgba::new(10, 8, 5),     // #0a0805 -- deep dark
    bg_secondary: Rgba::new(28, 22, 14), // #1c160e -- panel / wood
    bg_surface: Rgba::with_alpha(40, 32, 16, 144), // slot empty
    bg_elevated: Rgba::with_alpha(60, 50, 25, 220), // slot hover
    bg_overlay: Rgba::new(15, 13, 8),    // #0f0d08 -- input bg

    // Text
    text_high: Rgba::new(255, 215, 0), // #ffd700 -- gold bright
    text_primary: Rgba::new(200, 190, 160), // #c8bea0 -- parchment
    text_secondary: Rgba::new(200, 165, 70), // #c8a546 -- gold standard
    text_muted: Rgba::new(96, 90, 70), // #605a46 -- disabled

    // Accent primary (gold)
    accent_primary: Rgba::new(200, 165, 70), // #c8a546 -- gold
    accent_primary_hover: Rgba::new(255, 215, 0), // #ffd700 -- bright gold
    accent_primary_active: Rgba::new(132, 100, 48), // #846430 -- dark gold
    accent_primary_subtle: Rgba::with_alpha(200, 165, 70, 40),

    // Accent secondary (life red)
    accent_secondary: Rgba::new(180, 20, 20), // #b41414
    accent_secondary_hover: Rgba::new(220, 40, 40), // brighter red
    accent_secondary_subtle: Rgba::with_alpha(180, 20, 20, 40),

    // Semantic (D2 uses elements as semantic colors)
    success: Rgba::new(0, 180, 0), // #00b400 -- set green
    success_subtle: Rgba::with_alpha(0, 180, 0, 40),
    warning: Rgba::new(255, 165, 0), // #ffa500 -- crafted orange
    warning_subtle: Rgba::with_alpha(255, 165, 0, 40),
    error: Rgba::new(255, 64, 64), // #ff4040 -- requirement red
    error_subtle: Rgba::with_alpha(255, 64, 64, 40),
    info: Rgba::new(105, 105, 255), // #6969ff -- magic blue
    info_subtle: Rgba::with_alpha(105, 105, 255, 40),

    // Borders
    border_subtle: Rgba::new(26, 20, 16), // #1a1410 -- stone dark
    border_default: Rgba::new(80, 60, 20), // #503c14 -- panel border
    border_strong: Rgba::new(132, 100, 48), // #846430 -- gold dark
    border_accent: Rgba::new(200, 165, 70), // #c8a546 -- gold
};

// -- D2-specific extended colors (not in the unified Palette) --

/// D2 item quality colors (rarity system).
///
/// These are the canonical Diablo 2 item quality colors used to
/// display item names with their rarity tier.
pub struct D2QualityColors;

impl D2QualityColors {
    /// Normal/white items -- parchment tone.
    pub const NORMAL: Rgba = Rgba::new(200, 190, 160);
    /// Socketed items -- gray.
    pub const SOCKETED: Rgba = Rgba::new(132, 132, 132);
    /// Magic items -- blue.
    pub const MAGIC: Rgba = Rgba::new(105, 105, 255);
    /// Rare items -- yellow.
    pub const RARE: Rgba = Rgba::new(255, 255, 100);
    /// Unique items -- dark gold.
    pub const UNIQUE: Rgba = Rgba::new(165, 110, 0);
    /// Set items -- green.
    pub const SET: Rgba = Rgba::new(0, 180, 0);
    /// Crafted items -- orange.
    pub const CRAFTED: Rgba = Rgba::new(255, 165, 0);
    /// Runeword items -- orange (same as crafted in D2).
    pub const RUNE_WORD: Rgba = Rgba::new(255, 165, 0);
}

/// D2 elemental damage/resistance colors.
pub struct D2ElementColors;

impl D2ElementColors {
    /// Fire damage -- red-orange.
    pub const FIRE: Rgba = Rgba::new(255, 69, 0);
    /// Cold damage -- ice blue.
    pub const COLD: Rgba = Rgba::new(68, 136, 255);
    /// Lightning damage -- bright yellow.
    pub const LIGHTNING: Rgba = Rgba::new(255, 255, 0);
    /// Poison damage -- bright green.
    pub const POISON: Rgba = Rgba::new(0, 255, 0);
    /// Physical damage -- parchment.
    pub const PHYSICAL: Rgba = Rgba::new(200, 190, 160);
    /// Magic damage -- blue.
    pub const MAGIC_DMG: Rgba = Rgba::new(105, 105, 255);
}

/// D2 orb colors (life and mana globes).
pub struct D2OrbColors;

impl D2OrbColors {
    /// Life orb fill -- deep red.
    pub const LIFE: Rgba = Rgba::new(180, 20, 20);
    /// Life orb background -- dark red.
    pub const LIFE_DARK: Rgba = Rgba::new(110, 14, 14);
    /// Mana orb fill -- deep blue.
    pub const MANA: Rgba = Rgba::new(20, 40, 180);
    /// Mana orb background -- dark blue.
    pub const MANA_DARK: Rgba = Rgba::new(14, 20, 100);
}

/// D2 miscellaneous game colors.
pub struct D2MiscColors;

impl D2MiscColors {
    /// Active/selected skill highlight.
    pub const SKILL_ACTIVE: Rgba = Rgba::new(255, 200, 50);
    /// Stamina bar.
    pub const STAMINA: Rgba = Rgba::new(212, 200, 50);
    /// Experience bar.
    pub const XP_BAR: Rgba = Rgba::new(200, 165, 70);
    /// Copper metal accents.
    pub const COPPER: Rgba = Rgba::new(184, 115, 51);
    /// Silver metal accents.
    pub const SILVER: Rgba = Rgba::new(160, 160, 160);
    /// Light stone texture.
    pub const STONE_LIGHT: Rgba = Rgba::new(58, 48, 37);
    /// Dark stone texture.
    pub const STONE_DARK: Rgba = Rgba::new(26, 20, 16);
}

/// D2 text system colors (D2 color codes c0-c9).
///
/// These correspond to the in-game color code system used for
/// chat, item descriptions, and system messages.
pub struct D2SystemColors;

impl D2SystemColors {
    /// c0: Light gray -- general descriptions.
    pub const C0_LIGHT_GRAY: Rgba = Rgba::new(196, 196, 196);
    /// c1: Red -- warnings, requirements not met.
    pub const C1_RED: Rgba = Rgba::new(255, 64, 64);
    /// c2: Green -- set items.
    pub const C2_GREEN: Rgba = Rgba::new(0, 255, 0);
    /// c3: Blue -- magic items.
    pub const C3_BLUE: Rgba = Rgba::new(105, 105, 255);
    /// c4: Gold -- unique items.
    pub const C4_GOLD: Rgba = Rgba::new(199, 179, 119);
    /// c5: Dark gray -- socketed items.
    pub const C5_DARK_GRAY: Rgba = Rgba::new(105, 105, 105);
    /// c6: Black -- shadow text.
    pub const C6_BLACK: Rgba = Rgba::new(0, 0, 0);
    /// c7: Tan -- battle.net gold.
    pub const C7_TAN: Rgba = Rgba::new(208, 192, 144);
    /// c8: Orange -- crafted items.
    pub const C8_ORANGE: Rgba = Rgba::new(255, 168, 0);
    /// c9: Yellow -- rare items.
    pub const C9_YELLOW: Rgba = Rgba::new(255, 255, 100);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d2_quality_colors_are_distinct() {
        let colors = [
            D2QualityColors::NORMAL,
            D2QualityColors::SOCKETED,
            D2QualityColors::MAGIC,
            D2QualityColors::RARE,
            D2QualityColors::UNIQUE,
            D2QualityColors::SET,
        ];
        // Each quality color should be visually distinct
        for (i, a) in colors.iter().enumerate() {
            for (j, b) in colors.iter().enumerate() {
                if i != j {
                    assert_ne!(a, b, "Quality colors at index {i} and {j} should differ");
                }
            }
        }
    }

    #[test]
    fn test_d2_orb_colors_are_opaque() {
        assert_eq!(D2OrbColors::LIFE.a, 255);
        assert_eq!(D2OrbColors::LIFE_DARK.a, 255);
        assert_eq!(D2OrbColors::MANA.a, 255);
        assert_eq!(D2OrbColors::MANA_DARK.a, 255);
    }

    #[test]
    fn test_d2_palette_bg_base_is_darkest() {
        // bg_base should be the darkest background
        let luminance = |c: &Rgba| u32::from(c.r) + u32::from(c.g) + u32::from(c.b);
        assert!(luminance(&D2_PALETTE.bg_base) < luminance(&D2_PALETTE.bg_primary));
        assert!(luminance(&D2_PALETTE.bg_primary) < luminance(&D2_PALETTE.bg_secondary));
    }

    #[test]
    fn test_d2_system_colors_count() {
        // D2 has 10 color codes (c0-c9)
        let colors = [
            D2SystemColors::C0_LIGHT_GRAY,
            D2SystemColors::C1_RED,
            D2SystemColors::C2_GREEN,
            D2SystemColors::C3_BLUE,
            D2SystemColors::C4_GOLD,
            D2SystemColors::C5_DARK_GRAY,
            D2SystemColors::C6_BLACK,
            D2SystemColors::C7_TAN,
            D2SystemColors::C8_ORANGE,
            D2SystemColors::C9_YELLOW,
        ];
        assert_eq!(colors.len(), 10);
    }

    #[test]
    fn test_d2_element_fire_is_warm() {
        // Fire should have high red component
        assert!(D2ElementColors::FIRE.r > 200);
        assert!(D2ElementColors::FIRE.r > D2ElementColors::FIRE.b);
    }

    #[test]
    fn test_d2_element_cold_is_cool() {
        // Cold should have high blue component
        assert!(D2ElementColors::COLD.b > 200);
        assert!(D2ElementColors::COLD.b > D2ElementColors::COLD.r);
    }
}
