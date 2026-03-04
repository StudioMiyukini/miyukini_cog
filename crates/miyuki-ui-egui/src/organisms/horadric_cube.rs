// @id: MUIE-Cube @do: horadric-cube-organism @role: organism @layer: 6 @human: miyuk

//! Horadric cube organism -- 3x4 grid + transmute button.

use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::atoms::d2_button::D2Button;
use crate::convert::rgba_to_color32;

/// Cube grid dimensions.
pub const CUBE_COLS: usize = 3;
/// Cube grid rows.
pub const CUBE_ROWS: usize = 4;

/// The Horadric Cube organism.
pub struct HoradricCube;

impl HoradricCube {
    /// Draw the Horadric Cube as a window.
    ///
    /// Returns `true` if the Transmute button was clicked.
    pub fn show(ctx: &Context, is_open: &mut bool) -> bool {
        let mut transmuted = false;

        if !*is_open {
            return false;
        }

        egui::Window::new("Cube Horadrique")
            .resizable(false)
            .collapsible(false)
            .open(is_open)
            .default_pos([300.0, 100.0])
            .show(ctx, |ui| {
                let gold = rgba_to_color32(&D2_PALETTE.accent_primary);
                ui.label(
                    egui::RichText::new("Cube Horadrique (3x4)")
                        .color(gold)
                        .size(12.0),
                );
                ui.separator();

                // 3x4 grid placeholder
                let cell_size = 29.0;
                let grid_w = CUBE_COLS as f32 * cell_size;
                let grid_h = CUBE_ROWS as f32 * cell_size;
                let (resp, painter) = ui.allocate_painter(
                    egui::Vec2::new(grid_w, grid_h),
                    egui::Sense::click(),
                );
                let origin = resp.rect.min;

                painter.rect_filled(resp.rect, 2.0, egui::Color32::from_rgb(15, 12, 8));

                let line_color = egui::Color32::from_rgb(40, 30, 15);
                for col in 0..=CUBE_COLS {
                    let x = origin.x + col as f32 * cell_size;
                    painter.line_segment(
                        [egui::Pos2::new(x, origin.y), egui::Pos2::new(x, origin.y + grid_h)],
                        egui::Stroke::new(0.5, line_color),
                    );
                }
                for row in 0..=CUBE_ROWS {
                    let y = origin.y + row as f32 * cell_size;
                    painter.line_segment(
                        [egui::Pos2::new(origin.x, y), egui::Pos2::new(origin.x + grid_w, y)],
                        egui::Stroke::new(0.5, line_color),
                    );
                }

                ui.separator();

                if D2Button::new("Transmuter").show(ui).clicked() {
                    transmuted = true;
                }
            });

        transmuted
    }
}
