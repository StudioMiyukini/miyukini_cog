//! Minimap overlay (stub).
//!
//! Full implementation will render a top-right minimap with fog-of-war,
//! entity blips, and a toggle for full-screen map view. Currently a minimal
//! placeholder.

use egui::Context;

use crate::theme::D2Colors;

/// Draw the minimap overlay (stub).
///
/// In production this will render a corner minimap from tile data. Currently
/// shows a placeholder dark square with a label.
pub fn draw_minimap(ctx: &Context, screen_w: f32) {
    let minimap_size = 140.0;
    let margin = 8.0;
    let x = screen_w - minimap_size - margin;

    egui::Area::new("minimap".into())
        .fixed_pos(egui::Pos2::new(x, margin))
        .show(ctx, |ui| {
            let (resp, painter) = ui.allocate_painter(
                egui::Vec2::new(minimap_size, minimap_size),
                egui::Sense::click(),
            );
            let rect = resp.rect;

            // Dark background
            painter.rect_filled(
                rect,
                4.0,
                egui::Color32::from_rgba_premultiplied(10, 8, 5, 200),
            );
            painter.rect_stroke(
                rect,
                4.0,
                egui::Stroke::new(1.0, D2Colors::PANEL_BORDER),
            );

            // Placeholder label
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Minimap",
                egui::FontId::proportional(10.0),
                D2Colors::GOLD,
            );
        });
}
