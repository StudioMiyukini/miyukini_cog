// @id: MUIE-Orb @do: orb-atom @role: atom @layer: 6 @human: miyuk

//! Life and mana orbs -- the signature D2 HUD element.
//!
//! The orb is a circle that fills from bottom to top proportionally to
//! the current/max ratio. Life orbs are red, mana orbs are blue.

use crate::convert::rgba_to_color32;
use egui::{Color32, Pos2, Rect, Response, Ui, Vec2};
use miyuki_ui_tokens::palette::d2::{D2OrbColors, D2_PALETTE};

/// Orb type determines the fill color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrbType {
    /// Red life orb.
    Life,
    /// Blue mana orb.
    Mana,
}

/// A life or mana orb atom.
///
/// Renders a circular globe that fills from bottom to top. The fill level
/// is determined by `current / max`. Hovering shows a tooltip with the
/// exact values.
pub struct Orb {
    /// Current resource value.
    pub current: f32,
    /// Maximum resource value.
    pub max: f32,
    /// Type of orb (life or mana).
    pub orb_type: OrbType,
    /// Diameter of the orb in logical pixels.
    pub diameter: f32,
}

impl Orb {
    /// Create a new orb with default 64px diameter.
    pub fn new(current: f32, max: f32, orb_type: OrbType) -> Self {
        Self {
            current,
            max,
            orb_type,
            diameter: 64.0,
        }
    }

    /// Set custom diameter.
    pub fn diameter(mut self, diameter: f32) -> Self {
        self.diameter = diameter;
        self
    }

    /// Draw the orb and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let size = Vec2::splat(self.diameter);
        let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
        let rect = response.rect;
        let center = rect.center();
        let radius = self.diameter / 2.0;

        // Colors based on orb type
        let (fill_color, dark_color) = match self.orb_type {
            OrbType::Life => (
                rgba_to_color32(&D2OrbColors::LIFE),
                rgba_to_color32(&D2OrbColors::LIFE_DARK),
            ),
            OrbType::Mana => (
                rgba_to_color32(&D2OrbColors::MANA),
                rgba_to_color32(&D2OrbColors::MANA_DARK),
            ),
        };

        // Background circle (dark)
        painter.circle_filled(center, radius, dark_color);

        // Calculate fill percentage (clamped)
        let max_safe = if self.max > 0.0 { self.max } else { 1.0 };
        let pct = (self.current / max_safe).clamp(0.0, 1.0);

        // Fill from bottom to top using a clipped rectangle approach.
        // We draw a filled circle but clip it to only show the bottom portion.
        if pct > 0.0 {
            let fill_height = self.diameter * pct;
            let clip_rect =
                Rect::from_min_max(Pos2::new(rect.min.x, rect.max.y - fill_height), rect.max);

            // Use the painter's clip rect to restrict the fill
            let clipped = painter.with_clip_rect(clip_rect);
            clipped.circle_filled(center, radius, fill_color);
        }

        // Border ring (gold/metallic)
        let border_color = rgba_to_color32(&D2_PALETTE.border_default);
        painter.circle_stroke(center, radius, egui::Stroke::new(2.0, border_color));

        // Outer decorative ring
        let outer_color = rgba_to_color32(&D2_PALETTE.border_strong);
        painter.circle_stroke(center, radius + 1.0, egui::Stroke::new(1.0, outer_color));

        // Value text (current/max) centered
        let cur_i32 = self.current as i32;
        let max_i32 = self.max as i32;
        let value_text = format!("{cur_i32}/{max_i32}");
        painter.text(
            center,
            egui::Align2::CENTER_CENTER,
            &value_text,
            egui::FontId::proportional(10.0),
            Color32::WHITE,
        );

        // Tooltip
        let label = match self.orb_type {
            OrbType::Life => "Vie",
            OrbType::Mana => "Mana",
        };
        let tooltip = format!("{label} : {cur_i32}/{max_i32}");
        response.on_hover_text(tooltip)
    }
}

/// Compute the fill percentage (0.0-1.0) for an orb.
///
/// Utility for external callers that need the ratio without drawing.
pub fn orb_fill_pct(current: f32, max: f32) -> f32 {
    let max_safe = if max > 0.0 { max } else { 1.0 };
    (current / max_safe).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orb_fill_pct_full() {
        assert!((orb_fill_pct(100.0, 100.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_orb_fill_pct_half() {
        assert!((orb_fill_pct(50.0, 100.0) - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_orb_fill_pct_empty() {
        assert!((orb_fill_pct(0.0, 100.0) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_orb_fill_pct_overflow_clamped() {
        assert!((orb_fill_pct(150.0, 100.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_orb_fill_pct_negative_clamped() {
        assert!((orb_fill_pct(-10.0, 100.0) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_orb_fill_pct_zero_max() {
        // Should not divide by zero
        assert!((orb_fill_pct(50.0, 0.0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_orb_builder() {
        let orb = Orb::new(50.0, 100.0, OrbType::Life).diameter(80.0);
        assert!((orb.current - 50.0).abs() < f32::EPSILON);
        assert!((orb.max - 100.0).abs() < f32::EPSILON);
        assert_eq!(orb.orb_type, OrbType::Life);
        assert!((orb.diameter - 80.0).abs() < f32::EPSILON);
    }
}
