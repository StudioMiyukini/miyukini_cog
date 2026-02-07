//! Atomes UI JayKoa — éléments primitifs.
//!
//! @id: jaykoa_ui_atoms
//! @do: provide_primitive_ui_elements
//! @layer: ui

use crate::theme::JayKoaTheme;
use egui::{Color32, Ui, Vec2};

/// Badge coloré source (petit rectangle arrondi avec le nom du service).
pub fn source_badge(ui: &mut Ui, label: &str, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width().min(120.0), 16.0), egui::Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, egui::CornerRadius::same(3), color.linear_multiply(0.2));
    painter.text(rect.center(), egui::Align2::CENTER_CENTER, label, egui::FontId::proportional(10.0), color);
}

/// Pastille de couleur calendrier (petit cercle).
pub fn color_dot(ui: &mut Ui, color: Color32, size: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());
    ui.painter_at(rect).circle_filled(rect.center(), size / 2.0, color);
}

/// Séparateur horizontal fin.
pub fn thin_separator(ui: &mut Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), egui::Sense::hover());
    ui.painter_at(rect).rect_filled(rect, egui::CornerRadius::ZERO, color);
}

/// Cercle "aujourd'hui" autour d'un numéro de jour.
pub fn today_circle(ui: &mut Ui, day: u32, theme: &JayKoaTheme) {
    let text = format!("{}", day);
    let desired = Vec2::splat(24.0);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::click());
    let painter = ui.painter_at(rect);
    painter.circle_filled(rect.center(), 12.0, theme.today_highlight);
    painter.text(rect.center(), egui::Align2::CENTER_CENTER, text, egui::FontId::proportional(13.0), Color32::WHITE);
}
