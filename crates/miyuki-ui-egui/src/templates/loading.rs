// @id: MUIE-TmplLoading @do: loading-template @role: template @layer: 6 @human: miyuk

//! Loading screen layout template.

use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::atoms::progress_bar::{D2ProgressBar, ProgressBarVariant};
use crate::convert::rgba_to_color32;

/// The loading screen layout template.
pub struct LoadingLayout;

impl LoadingLayout {
    /// Draw the loading screen.
    pub fn show(ctx: &Context, progress: f32, message: &str) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Dark background fill
            let bg = rgba_to_color32(&D2_PALETTE.bg_base);
            let screen = ui.max_rect();
            ui.painter().rect_filled(screen, 0.0, bg);

            ui.vertical_centered(|ui| {
                ui.add_space(screen.height() / 2.0 - 40.0);

                // Loading message
                let gold = rgba_to_color32(&D2_PALETTE.text_high);
                ui.label(egui::RichText::new(message).color(gold).size(16.0));

                ui.add_space(16.0);

                // Progress bar
                D2ProgressBar::new(progress, ProgressBarVariant::Loading)
                    .width(300.0)
                    .show(ui);

                // Percentage
                let pct_text = format!("{}%", (progress * 100.0) as i32);
                let text_color = rgba_to_color32(&D2_PALETTE.text_secondary);
                ui.label(egui::RichText::new(&pct_text).color(text_color).size(12.0));
            });
        });
    }
}
