// @id: MUIE-Minimap @do: minimap-organism @role: organism @layer: 6 @human: miyuk

//! Minimap overlay organism.

use crate::atoms::minimap_marker::MarkerType;
use crate::convert::rgba_to_color32;
use egui::{Color32, Context, Pos2, Vec2};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;

/// A marker to display on the minimap.
#[derive(Debug, Clone)]
pub struct MinimapEntity {
    /// Position on the minimap (0.0-1.0 normalized).
    pub x: f32,
    /// Position on the minimap (0.0-1.0 normalized).
    pub y: f32,
    /// Marker type.
    pub marker_type: MarkerType,
}

/// The minimap organism.
pub struct Minimap;

impl Minimap {
    /// Draw the minimap overlay at the top-right corner.
    pub fn show(ctx: &Context, entities: &[MinimapEntity], screen_w: f32, minimap_size: f32) {
        let margin = 10.0;
        let pos = Pos2::new(screen_w - minimap_size - margin, margin);

        egui::Area::new("minimap_overlay".into())
            .fixed_pos(pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let size = Vec2::splat(minimap_size);
                let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
                let rect = response.rect;

                // Background (semi-transparent dark)
                let bg = Color32::from_rgba_premultiplied(5, 4, 2, 180);
                painter.rect_filled(rect, 2.0, bg);

                // Border
                let border = rgba_to_color32(&D2_PALETTE.border_default);
                painter.rect_stroke(rect, 2.0, egui::Stroke::new(1.0, border));

                // Entities
                for entity in entities {
                    let ex = rect.min.x + entity.x.clamp(0.0, 1.0) * minimap_size;
                    let ey = rect.min.y + entity.y.clamp(0.0, 1.0) * minimap_size;
                    let color = entity.marker_type.color();
                    let radius = 2.0;

                    match entity.marker_type {
                        MarkerType::Player => {
                            painter.circle_filled(Pos2::new(ex, ey), radius + 1.0, color);
                        }
                        MarkerType::Waypoint => {
                            let half = radius;
                            painter.line_segment(
                                [Pos2::new(ex - half, ey), Pos2::new(ex + half, ey)],
                                egui::Stroke::new(1.5, color),
                            );
                            painter.line_segment(
                                [Pos2::new(ex, ey - half), Pos2::new(ex, ey + half)],
                                egui::Stroke::new(1.5, color),
                            );
                        }
                        _ => {
                            painter.circle_filled(Pos2::new(ex, ey), radius, color);
                        }
                    }
                }
            });
    }
}
