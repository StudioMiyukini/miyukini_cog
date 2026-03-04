// @id: MUIE-MinimapMarker @do: minimap-marker-atom @role: atom @layer: 6 @human: miyuk

//! Minimap marker -- small colored dots/icons on the minimap overlay.

use egui::{Color32, Pos2, Response, Ui, Vec2};
use miyuki_ui_tokens::palette::d2::{D2OrbColors, D2MiscColors, D2_PALETTE};
use crate::convert::rgba_to_color32;

/// Type of entity shown on the minimap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerType {
    /// Player character (white dot).
    Player,
    /// Party member (green dot).
    PartyMember,
    /// NPC (yellow dot).
    Npc,
    /// Monster (red dot).
    Monster,
    /// Waypoint (purple cross).
    Waypoint,
    /// Item on ground (quality-colored dot).
    Item,
    /// Portal (blue circle).
    Portal,
}

impl MarkerType {
    /// Get the default color for this marker type.
    pub fn color(self) -> Color32 {
        match self {
            Self::Player => Color32::WHITE,
            Self::PartyMember => Color32::from_rgb(0, 200, 0),
            Self::Npc => rgba_to_color32(&D2MiscColors::SKILL_ACTIVE),
            Self::Monster => rgba_to_color32(&D2OrbColors::LIFE),
            Self::Waypoint => Color32::from_rgb(180, 100, 255),
            Self::Item => rgba_to_color32(&D2_PALETTE.accent_primary),
            Self::Portal => Color32::from_rgb(80, 120, 255),
        }
    }
}

/// A minimap marker atom.
pub struct MinimapMarker {
    /// Marker type (determines shape and color).
    pub marker_type: MarkerType,
    /// Override color (if set, takes priority over the type default).
    pub color_override: Option<Color32>,
    /// Marker radius in logical pixels.
    pub radius: f32,
}

impl MinimapMarker {
    /// Create a new minimap marker.
    pub fn new(marker_type: MarkerType) -> Self {
        Self {
            marker_type,
            color_override: None,
            radius: 3.0,
        }
    }

    /// Override the marker color.
    pub fn color(mut self, color: Color32) -> Self {
        self.color_override = Some(color);
        self
    }

    /// Set the marker radius.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Draw the marker and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let size = Vec2::splat(self.radius * 2.0 + 2.0);
        let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
        let center = response.rect.center();

        let color = self.color_override.unwrap_or_else(|| self.marker_type.color());

        match self.marker_type {
            MarkerType::Waypoint => {
                // Cross shape
                let half = self.radius;
                painter.line_segment(
                    [Pos2::new(center.x - half, center.y), Pos2::new(center.x + half, center.y)],
                    egui::Stroke::new(2.0, color),
                );
                painter.line_segment(
                    [Pos2::new(center.x, center.y - half), Pos2::new(center.x, center.y + half)],
                    egui::Stroke::new(2.0, color),
                );
            }
            MarkerType::Portal => {
                // Circle outline
                painter.circle_stroke(center, self.radius, egui::Stroke::new(1.5, color));
            }
            _ => {
                // Filled circle
                painter.circle_filled(center, self.radius, color);
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_type_colors_are_nonzero() {
        let types = [
            MarkerType::Player,
            MarkerType::PartyMember,
            MarkerType::Npc,
            MarkerType::Monster,
            MarkerType::Waypoint,
            MarkerType::Item,
            MarkerType::Portal,
        ];
        for mt in types {
            let c = mt.color();
            // At least one channel should be nonzero
            assert!(c.r() > 0 || c.g() > 0 || c.b() > 0);
        }
    }

    #[test]
    fn test_marker_builder() {
        let marker = MinimapMarker::new(MarkerType::Monster)
            .radius(5.0)
            .color(Color32::YELLOW);
        assert_eq!(marker.marker_type, MarkerType::Monster);
        assert!((marker.radius - 5.0).abs() < f32::EPSILON);
        assert_eq!(marker.color_override, Some(Color32::YELLOW));
    }
}
