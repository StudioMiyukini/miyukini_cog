//! Diablo II-style dark medieval theme for egui.
//!
//! Provides [`D2Colors`] (17 color constants), [`apply_d2_theme`] to configure
//! an egui context, and [`item_quality_color`] for quality-based text coloring.

use egui::{Color32, Rounding, Stroke, Style, Visuals};

/// Palette of Diablo II-inspired colors.
pub struct D2Colors;

impl D2Colors {
    /// Deep dark background (almost black, warm undertone).
    pub const BG_DARK: Color32 = Color32::from_rgb(10, 8, 5);
    /// Panel / window background.
    pub const PANEL_BG: Color32 = Color32::from_rgb(28, 22, 14);
    /// Panel border (dark gold).
    pub const PANEL_BORDER: Color32 = Color32::from_rgb(80, 60, 20);
    /// Standard gold for labels and highlights.
    pub const GOLD: Color32 = Color32::from_rgb(200, 165, 70);
    /// Bright gold for important values.
    pub const GOLD_BRIGHT: Color32 = Color32::from_rgb(255, 215, 0);
    /// Life orb red.
    pub const RED_LIFE: Color32 = Color32::from_rgb(180, 20, 20);
    /// Mana orb blue.
    pub const BLUE_MANA: Color32 = Color32::from_rgb(20, 40, 180);
    /// Normal (white-ish) item/text color.
    pub const TEXT_NORMAL: Color32 = Color32::from_rgb(200, 190, 160);
    /// Magic (blue) item text.
    pub const TEXT_MAGIC: Color32 = Color32::from_rgb(100, 100, 255);
    /// Rare (yellow) item text.
    pub const TEXT_RARE: Color32 = Color32::from_rgb(255, 255, 100);
    /// Unique (dark gold/brown) item text.
    pub const TEXT_UNIQUE: Color32 = Color32::from_rgb(165, 110, 0);
    /// Set (green) item text.
    pub const TEXT_SET: Color32 = Color32::from_rgb(0, 180, 0);
    /// Rune-word (orange) item text.
    pub const TEXT_RUNE_WORD: Color32 = Color32::from_rgb(255, 165, 0);
    /// Active/highlighted skill.
    pub const SKILL_ACTIVE: Color32 = Color32::from_rgb(255, 200, 50);
    /// Empty slot background (semi-transparent).
    pub const SLOT_EMPTY: Color32 = Color32::from_rgba_premultiplied(40, 30, 15, 200);
    /// Hovered slot background (semi-transparent).
    pub const SLOT_HOVER: Color32 = Color32::from_rgba_premultiplied(60, 50, 25, 220);
}

/// Apply the full D2 theme to an egui context.
///
/// Sets dark visuals, compact spacing, gold-tinted borders, and warm panel
/// backgrounds. Call this once after creating the `egui::Context`.
pub fn apply_d2_theme(ctx: &egui::Context) {
    let style = Style {
        visuals: Visuals {
            dark_mode: true,
            panel_fill: D2Colors::PANEL_BG,
            window_fill: D2Colors::PANEL_BG,
            window_stroke: Stroke::new(2.0, D2Colors::PANEL_BORDER),
            window_rounding: Rounding::same(2.0),
            override_text_color: Some(D2Colors::TEXT_NORMAL),
            ..Visuals::dark()
        },
        spacing: egui::style::Spacing {
            item_spacing: egui::vec2(4.0, 4.0),
            window_margin: egui::Margin::same(6.0),
            button_padding: egui::vec2(8.0, 4.0),
            ..egui::style::Spacing::default()
        },
        ..Style::default()
    };

    ctx.set_style(style);

    // Font definitions -- in production, load a bitmap TTF (e.g. ExocetBlizzard-like).
    let fonts = egui::FontDefinitions::default();
    ctx.set_fonts(fonts);
}

/// Returns the text [`Color32`] matching an item quality string.
///
/// Recognized qualities: `"magic"`, `"rare"`, `"unique"`, `"set"`, `"rune_word"`.
/// Everything else falls back to [`D2Colors::TEXT_NORMAL`].
pub fn item_quality_color(quality: &str) -> Color32 {
    match quality {
        "magic" => D2Colors::TEXT_MAGIC,
        "rare" => D2Colors::TEXT_RARE,
        "unique" => D2Colors::TEXT_UNIQUE,
        "set" => D2Colors::TEXT_SET,
        "rune_word" => D2Colors::TEXT_RUNE_WORD,
        _ => D2Colors::TEXT_NORMAL,
    }
}
