//! Organism Layout — structure principale (header + body container : sidebar + contenu).
//!
//! Body : conteneur aux dimensions d'affichage, marge 10 px, sidebar 20 % largeur (haut-gauche),
//! reste pour le contenu. Défilements selon besoin.

use crate::theme::JayFestivalTheme;
use eframe::egui::{Context, Frame};

/// Id header (évite collisions avec autres écrans).
const HEADER_ID: &str = "jayfestival_landing_header";
/// Id défilement sidebar.
const SIDEBAR_SCROLL_ID: &str = "jayfestival_landing_sidebar_scroll";
/// Id défilement contenu.
const CONTENT_SCROLL_ID: &str = "jayfestival_landing_content_scroll";

/// Marge entre le conteneur body et les éléments (sidebar, contenu).
const BODY_MARGIN: f32 = 10.0;
/// Écart entre sidebar et contenu.
const SIDEBAR_CONTENT_GAP: f32 = 10.0;
/// Part de la largeur utilisée par la sidebar (16 %).
const SIDEBAR_WIDTH_RATIO: f32 = 0.16;

/// @id: organism_layout
/// @do: define_layout_structure_with_header_body_sidebar_content
/// @layer: ui
/// @human: Layout : header, conteneur body (marge 10 px), sidebar 20 % (haut-gauche), contenu (reste).

/// @id: layout_render
/// @do: render_layout_with_header_body_sidebar_and_content
/// @layer: ui
/// Affiche le layout : TopBottomPanel (header), CentralPanel = body (marge 10 px, sidebar 20 %, contenu).
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
    eframe::egui::TopBottomPanel::top(HEADER_ID)
        .frame(Frame::default().fill(theme.background_primary()))
        .show(ctx, |ui| {
            header_ui(ui);
        });

    eframe::egui::CentralPanel::default()
        .frame(Frame::default().fill(theme.background_primary()))
        .show(ctx, |ui| {
            let available = ui.available_rect_before_wrap();
            let available_w = available.width();
            let available_h = available.height();

            let inner_w = (available_w - BODY_MARGIN * 2.0).max(0.0);
            let inner_h = (available_h - BODY_MARGIN * 2.0).max(0.0);
            let sidebar_w = inner_w * SIDEBAR_WIDTH_RATIO;
            let content_w = (inner_w - sidebar_w - SIDEBAR_CONTENT_GAP).max(0.0);

            ui.vertical(|ui| {
                ui.add_space(BODY_MARGIN);
                ui.horizontal(|ui| {
                    ui.add_space(BODY_MARGIN);
                    ui.allocate_ui(
                        eframe::egui::vec2(sidebar_w, inner_h),
                        |ui| {
                            let frame = Frame::default()
                                .fill(theme.navigation_container_background())
                                .inner_margin(eframe::egui::Margin::same(BODY_MARGIN as i8));
                            frame.show(ui, |ui| {
                                eframe::egui::ScrollArea::vertical()
                                    .id_salt(SIDEBAR_SCROLL_ID)
                                    .auto_shrink([false; 2])
                                    .show(ui, |ui| {
                                        sidebar_ui(ui);
                                    });
                            });
                        },
                    );
                    ui.add_space(SIDEBAR_CONTENT_GAP);
                    ui.allocate_ui(
                        eframe::egui::vec2(content_w, inner_h),
                        |ui| {
                            let frame = Frame::default()
                                .fill(theme.background_primary())
                                .inner_margin(eframe::egui::Margin::same(BODY_MARGIN as i8));
                            frame.show(ui, |ui| {
                                eframe::egui::ScrollArea::vertical()
                                    .id_salt(CONTENT_SCROLL_ID)
                                    .auto_shrink([false; 2])
                                    .show(ui, |ui| {
                                        content_ui(ui);
                                    });
                            });
                        },
                    );
                    ui.add_space(BODY_MARGIN);
                });
                ui.add_space(BODY_MARGIN);
            });
        });
}
