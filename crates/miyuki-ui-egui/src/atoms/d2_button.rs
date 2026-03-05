// @id: MUIE-D2Button @do: d2-button-atom @role: atom @layer: 6 @human: miyuk

//! Diablo 2 style button with stone background and gold text.
//!
//! Supports three visual variants: Primary (gold border + highlight),
//! Secondary (muted stone), and Menu (large, centered text for menu screens).

use crate::convert::rgba_to_color32;
use egui::{Color32, Response, Rounding, Stroke, Ui, Vec2};
use miyuki_ui_tokens::palette::d2::{D2MiscColors, D2_PALETTE};

/// Visual variant for the D2 button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum D2ButtonVariant {
    /// Gold-bordered primary action button.
    Primary,
    /// Muted stone secondary button.
    Secondary,
    /// Large menu screen button.
    Menu,
}

/// A Diablo 2-style button atom.
///
/// Renders a stone-textured button with gold text. The visual appearance
/// changes on hover and click to provide feedback.
pub struct D2Button<'a> {
    /// Button label text.
    pub text: &'a str,
    /// Visual variant.
    pub variant: D2ButtonVariant,
    /// Whether the button is disabled.
    pub disabled: bool,
}

impl<'a> D2Button<'a> {
    /// Create a new primary D2 button.
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            variant: D2ButtonVariant::Primary,
            disabled: false,
        }
    }

    /// Set the button variant.
    pub fn variant(mut self, variant: D2ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Draw the button and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let padding = match self.variant {
            D2ButtonVariant::Primary => Vec2::new(16.0, 6.0),
            D2ButtonVariant::Secondary => Vec2::new(12.0, 4.0),
            D2ButtonVariant::Menu => Vec2::new(32.0, 10.0),
        };

        let font_size = match self.variant {
            D2ButtonVariant::Primary => 12.0,
            D2ButtonVariant::Secondary => 11.0,
            D2ButtonVariant::Menu => 16.0,
        };

        let text_color = if self.disabled {
            rgba_to_color32(&D2_PALETTE.text_muted)
        } else {
            rgba_to_color32(&D2_PALETTE.accent_primary)
        };

        let rich_text = egui::RichText::new(self.text)
            .color(text_color)
            .size(font_size);

        let galley = ui.painter().layout_no_wrap(
            self.text.to_string(),
            egui::FontId::proportional(font_size),
            text_color,
        );

        let desired_size = galley.size() + padding * 2.0;
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Determine colors based on interaction state
            let (bg_color, border_color) = if self.disabled {
                (
                    rgba_to_color32(&D2_PALETTE.bg_primary),
                    rgba_to_color32(&D2_PALETTE.border_subtle),
                )
            } else if response.is_pointer_button_down_on() {
                (
                    rgba_to_color32(&D2_PALETTE.accent_primary_active),
                    rgba_to_color32(&D2_PALETTE.border_accent),
                )
            } else if response.hovered() {
                (
                    rgba_to_color32(&D2_PALETTE.bg_elevated),
                    rgba_to_color32(&D2_PALETTE.accent_primary_hover),
                )
            } else {
                (
                    rgba_to_color32(&D2_PALETTE.bg_secondary),
                    rgba_to_color32(&D2_PALETTE.border_default),
                )
            };

            let rounding = Rounding::same(2.0);

            // Background
            painter.rect_filled(rect, rounding, bg_color);

            // Border
            painter.rect_stroke(rect, rounding, Stroke::new(1.5, border_color));

            // Hover glow effect for primary variant
            if !self.disabled && response.hovered() && self.variant == D2ButtonVariant::Primary {
                let glow_color = Color32::from_rgba_premultiplied(200, 165, 70, 30);
                painter.rect_filled(rect.expand(1.0), Rounding::same(3.0), glow_color);
            }

            // Text
            let hover_text_color = if !self.disabled && response.hovered() {
                rgba_to_color32(&D2MiscColors::SKILL_ACTIVE)
            } else {
                text_color
            };

            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                self.text,
                egui::FontId::proportional(font_size),
                hover_text_color,
            );
        }

        let _ = rich_text; // used for type inference, actual rendering is via painter

        // Show tooltip label on hover if disabled
        if self.disabled {
            response.on_hover_text("Desactive")
        } else {
            response
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d2_button_builder() {
        let btn = D2Button::new("Test")
            .variant(D2ButtonVariant::Menu)
            .disabled(true);
        assert_eq!(btn.text, "Test");
        assert_eq!(btn.variant, D2ButtonVariant::Menu);
        assert!(btn.disabled);
    }

    #[test]
    fn test_d2_button_default_is_primary() {
        let btn = D2Button::new("Click");
        assert_eq!(btn.variant, D2ButtonVariant::Primary);
        assert!(!btn.disabled);
    }
}
