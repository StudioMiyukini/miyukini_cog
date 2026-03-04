// @id: MUIE-QualityText @do: quality-text-atom @role: atom @layer: 6 @human: miyuk

//! Quality-colored text for item names.
//!
//! Each item quality tier has a canonical D2 color. This atom renders text
//! in the appropriate color for the given rarity.

use egui::{Color32, Response, Ui};
use miyuki_ui_tokens::palette::d2::D2QualityColors;
use crate::convert::rgba_to_color32;

/// Item quality/rarity tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemQuality {
    /// White/parchment -- base items.
    Normal,
    /// Gray -- socketed items.
    Socketed,
    /// Blue -- magic items.
    Magic,
    /// Yellow -- rare items.
    Rare,
    /// Dark gold -- unique items.
    Unique,
    /// Green -- set items.
    Set,
    /// Orange -- crafted items.
    Crafted,
    /// Orange -- runeword items.
    RuneWord,
}

impl ItemQuality {
    /// Get the canonical D2 color for this quality tier.
    pub fn color(self) -> Color32 {
        match self {
            Self::Normal => rgba_to_color32(&D2QualityColors::NORMAL),
            Self::Socketed => rgba_to_color32(&D2QualityColors::SOCKETED),
            Self::Magic => rgba_to_color32(&D2QualityColors::MAGIC),
            Self::Rare => rgba_to_color32(&D2QualityColors::RARE),
            Self::Unique => rgba_to_color32(&D2QualityColors::UNIQUE),
            Self::Set => rgba_to_color32(&D2QualityColors::SET),
            Self::Crafted => rgba_to_color32(&D2QualityColors::CRAFTED),
            Self::RuneWord => rgba_to_color32(&D2QualityColors::RUNE_WORD),
        }
    }

    /// Parse a quality string into an `ItemQuality`.
    ///
    /// Falls back to `Normal` for unrecognized strings.
    pub fn from_str_lossy(s: &str) -> Self {
        match s {
            "socketed" => Self::Socketed,
            "magic" => Self::Magic,
            "rare" => Self::Rare,
            "unique" => Self::Unique,
            "set" => Self::Set,
            "crafted" => Self::Crafted,
            "rune_word" | "runeword" => Self::RuneWord,
            _ => Self::Normal,
        }
    }
}

/// Quality-colored text atom.
///
/// Renders a text string in the color corresponding to an item quality tier.
pub struct QualityText<'a> {
    /// The text to display.
    pub text: &'a str,
    /// The item quality (determines color).
    pub quality: ItemQuality,
    /// Font size in logical pixels.
    pub font_size: f32,
    /// Whether to render in bold.
    pub bold: bool,
}

impl<'a> QualityText<'a> {
    /// Create a new quality text.
    pub fn new(text: &'a str, quality: ItemQuality) -> Self {
        Self {
            text,
            quality,
            font_size: 12.0,
            bold: false,
        }
    }

    /// Set font size.
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set bold rendering.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Draw the quality text and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let color = self.quality.color();
        let mut rich = egui::RichText::new(self.text)
            .color(color)
            .size(self.font_size);
        if self.bold {
            rich = rich.strong();
        }
        ui.label(rich)
    }
}

/// Returns the egui [`Color32`] for a quality string.
///
/// Convenience function that mirrors the mge-ui `item_quality_color`.
pub fn quality_color_from_str(quality: &str) -> Color32 {
    ItemQuality::from_str_lossy(quality).color()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_colors_are_distinct() {
        let qualities = [
            ItemQuality::Normal,
            ItemQuality::Socketed,
            ItemQuality::Magic,
            ItemQuality::Rare,
            ItemQuality::Unique,
            ItemQuality::Set,
        ];
        for (i, a) in qualities.iter().enumerate() {
            for (j, b) in qualities.iter().enumerate() {
                if i != j {
                    assert_ne!(
                        a.color(),
                        b.color(),
                        "Quality colors at index {i} and {j} should differ"
                    );
                }
            }
        }
    }

    #[test]
    fn test_crafted_and_runeword_same_color() {
        assert_eq!(ItemQuality::Crafted.color(), ItemQuality::RuneWord.color());
    }

    #[test]
    fn test_from_str_lossy() {
        assert_eq!(ItemQuality::from_str_lossy("magic"), ItemQuality::Magic);
        assert_eq!(ItemQuality::from_str_lossy("set"), ItemQuality::Set);
        assert_eq!(ItemQuality::from_str_lossy("rune_word"), ItemQuality::RuneWord);
        assert_eq!(ItemQuality::from_str_lossy("runeword"), ItemQuality::RuneWord);
        assert_eq!(ItemQuality::from_str_lossy("unknown"), ItemQuality::Normal);
    }

    #[test]
    fn test_quality_color_from_str() {
        let magic_color = quality_color_from_str("magic");
        assert_eq!(magic_color, ItemQuality::Magic.color());
    }

    #[test]
    fn test_quality_text_builder() {
        let qt = QualityText::new("Windforce", ItemQuality::Unique)
            .font_size(14.0)
            .bold(true);
        assert_eq!(qt.text, "Windforce");
        assert_eq!(qt.quality, ItemQuality::Unique);
        assert!((qt.font_size - 14.0).abs() < f32::EPSILON);
        assert!(qt.bold);
    }
}
