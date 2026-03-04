// @id: MUIE-TmplMenu @do: menu-template @role: template @layer: 6 @human: miyuk

//! Menu layout template -- dark background + centered panel.

use egui::Context;
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::organisms::main_menu::{MainMenu, MainMenuAction};
use crate::convert::rgba_to_color32;

/// The menu layout template.
pub struct MenuLayout;

impl MenuLayout {
    /// Draw the main menu layout.
    pub fn show(ctx: &Context, screen_w: f32, screen_h: f32) -> MainMenuAction {
        // Dark background
        egui::Area::new("menu_bg".into())
            .fixed_pos(egui::Pos2::ZERO)
            .order(egui::Order::Background)
            .show(ctx, |ui| {
                let screen = ui.ctx().screen_rect();
                let bg = rgba_to_color32(&D2_PALETTE.bg_base);
                ui.painter().rect_filled(screen, 0.0, bg);
            });

        // Main menu
        MainMenu::show(ctx, screen_w, screen_h)
    }
}
