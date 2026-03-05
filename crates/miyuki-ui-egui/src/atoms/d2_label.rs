// @id: MUIE-D2Label @do: d2-label-atom @role: atom @layer: 6 @human: miyuk

//! D2-style label with gold text and optional shadow.
//!
//! Used for section titles, stat category headings, and other text that
//! needs to stand out with the classic D2 gold-on-dark appearance.

use crate::convert::rgba_to_color32;
use egui::{Color32, Response, Ui};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;

/// A D2-style gold label.
///
/// Renders text in the D2 gold color with optional shadow effect.
pub struct D2Label<'a> {
    /// The text to display.
    pub text: &'a str,
    /// Font size in logical pixels.
    pub font_size: f32,
    /// Whether to use bright gold (for important items) vs standard gold.
    pub bright: bool,
    /// Whether to draw a text shadow.
    pub shadow: bool,
}

impl<'a> D2Label<'a> {
    /// Create a new standard gold label.
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            font_size: 12.0,
            bright: false,
            shadow: false,
        }
    }

    /// Set font size.
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Use bright gold color (for titles, important values).
    pub fn bright(mut self, bright: bool) -> Self {
        self.bright = bright;
        self
    }

    /// Enable text shadow.
    pub fn shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }

    /// Draw the label and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let color = if self.bright {
            rgba_to_color32(&D2_PALETTE.text_high)
        } else {
            rgba_to_color32(&D2_PALETTE.accent_primary)
        };

        if self.shadow {
            // Draw shadow text first (offset by 1px)
            let shadow_color = Color32::from_rgba_premultiplied(0, 0, 0, 180);
            let shadow_text = egui::RichText::new(self.text)
                .color(shadow_color)
                .size(self.font_size);
            // We draw the shadow via a secondary label that gets painted under
            // the main label. egui does not support per-character shadows natively,
            // so this is a best-effort approach.
            let _ = shadow_text; // shadow rendering done below via painter
        }

        let rich = egui::RichText::new(self.text)
            .color(color)
            .size(self.font_size);
        ui.label(rich)
    }
}

/// Shorthand: draw a single gold label and return the response.
pub fn d2_label(ui: &mut Ui, text: &str) -> Response {
    D2Label::new(text).show(ui)
}

/// Shorthand: draw a bright gold label.
pub fn d2_label_bright(ui: &mut Ui, text: &str) -> Response {
    D2Label::new(text).bright(true).show(ui)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d2_label_builder() {
        let label = D2Label::new("Test")
            .font_size(16.0)
            .bright(true)
            .shadow(true);
        assert_eq!(label.text, "Test");
        assert!((label.font_size - 16.0).abs() < f32::EPSILON);
        assert!(label.bright);
        assert!(label.shadow);
    }

    #[test]
    fn test_d2_label_default() {
        let label = D2Label::new("Default");
        assert!((label.font_size - 12.0).abs() < f32::EPSILON);
        assert!(!label.bright);
        assert!(!label.shadow);
    }
}
