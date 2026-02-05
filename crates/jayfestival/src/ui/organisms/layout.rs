//! Organism Layout — structure principale (header + SidePanel + CentralPanel).
//!
//! Conforme [Specification UI] § 2. Style Catakana_Orga : sidebar brun foncé à gauche.

use crate::theme::JayFestivalTheme;
use eframe::egui::{Context, Frame};

/// @id: organism_layout
/// @do: define_layout_structure_with_header_sidebar_central
/// @layer: ui
/// @human: Layout applicatif : bandeau header, panneau latéral gauche (nav), zone centrale.

/// @id: layout_render
/// @do: render_layout_with_header_side_panel_and_central_panel
/// @layer: ui
/// Affiche le layout : TopBottomPanel (header), SidePanel gauche (sidebar brun), CentralPanel (contenu).
/// Les trois closures sont appelées à chaque frame (FnMut).
pub fn layout_show<H, S, C>(
    ctx: &Context,
    theme: &JayFestivalTheme,
    mut header_ui: H,
    mut sidebar_ui: S,
    mut content_ui: C,
) where
    H: FnMut(&mut eframe::egui::Ui),
    S: FnMut(&mut eframe::egui::Ui),
    C: FnMut(&mut eframe::egui::Ui),
{
    eframe::egui::TopBottomPanel::top("layout_header")
        .frame(Frame::default().fill(theme.background_primary()))
        .show(ctx, |ui| {
            header_ui(ui);
        });

    /// Largeur fixe 260 px ; egui redimensionne la sidebar par défaut (resizable: true) si on ne désactive pas.
    const SIDEBAR_WIDTH: f32 = 260.0;

    eframe::egui::SidePanel::left("layout_sidebar")
        .default_width(SIDEBAR_WIDTH)
        .resizable(false)
        .frame(Frame::default().fill(theme.navigation_container_background()))
        .show(ctx, |ui| {
            sidebar_ui(ui);
        });

    eframe::egui::CentralPanel::default()
        .frame(Frame::default().fill(theme.background_primary()))
        .show(ctx, |ui| {
            content_ui(ui);
        });
}
