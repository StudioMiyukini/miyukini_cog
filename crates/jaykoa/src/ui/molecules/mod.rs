//! Molécules UI JayKoa — compositions d'atomes.
//!
//! @id: jaykoa_ui_molecules
//! @do: provide_composite_ui_elements
//! @layer: ui

use crate::theme::JayKoaTheme;
use egui::{Color32, Ui};

/// Ligne d'agenda dans la sidebar (checkbox + pastille + nom).
pub fn agenda_row(ui: &mut Ui, name: &str, color_hex: &str, visible: &mut bool, _theme: &JayKoaTheme) -> bool {
    let color = hex_to_color32(color_hex);
    let mut changed = false;
    ui.horizontal(|ui| {
        if ui.checkbox(visible, "").changed() { changed = true; }
        crate::ui::atoms::color_dot(ui, color, 10.0);
        ui.label(egui::RichText::new(name).size(13.0));
    });
    changed
}

/// Sélecteur de vue (Jour / Semaine / Mois / Année / Planning).
pub fn view_selector(ui: &mut Ui, current: &mut crate::app_state::ViewId, theme: &JayKoaTheme) {
    use crate::app_state::ViewId;
    let views = [ViewId::Day, ViewId::Week, ViewId::Month, ViewId::Year, ViewId::Schedule];
    ui.horizontal(|ui| {
        for view in &views {
            let selected = *current == *view;
            let text = egui::RichText::new(view.label()).size(12.0);
            let text = if selected { text.color(theme.accent).strong() } else { text.color(theme.text_secondary) };
            if ui.selectable_label(selected, text).clicked() { *current = *view; }
        }
    });
}

/// Boutons de navigation temporelle (< Aujourd'hui >).
pub fn navigation_buttons(ui: &mut Ui, state: &mut crate::app_state::AppState, _theme: &JayKoaTheme) {
    ui.horizontal(|ui| {
        if ui.button("◀").clicked() { state.go_prev(); }
        if ui.button("Aujourd'hui").clicked() { state.go_today(); }
        if ui.button("▶").clicked() { state.go_next(); }
        ui.add_space(12.0);
        ui.label(egui::RichText::new(state.navigation_title()).size(16.0).strong());
    });
}

/// Convertit un code hex (#RRGGBB) en `Color32`.
pub fn hex_to_color32(hex: &str) -> Color32 {
    let hex = hex.trim_start_matches('#');
    if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(100);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(100);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(100);
        Color32::from_rgb(r, g, b)
    } else { Color32::GRAY }
}
