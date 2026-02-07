//! Organismes UI JayKoa — compositions de molécules.
//!
//! @id: jaykoa_ui_organisms
//! @do: provide_high_level_ui_compositions
//! @layer: ui

use crate::app_state::AppState;
use crate::data::{Agenda, JayKoaDb};
use crate::theme::JayKoaTheme;
use crate::ui::molecules;
use chrono::{Datelike, Local, NaiveDate};
use egui::{Color32, ScrollArea, Ui, Vec2};
use std::sync::Arc;

/// Header bar : navigation + sélecteur de vue + bouton créer.
pub fn header_bar(ui: &mut Ui, state: &mut AppState, theme: &JayKoaTheme) {
    ui.horizontal(|ui| {
        ui.set_min_height(theme.header_height);
        ui.label(egui::RichText::new("🗓️ JayKoa").size(18.0).strong().color(theme.text_primary));
        ui.add_space(16.0);
        molecules::navigation_buttons(ui, state, theme);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(egui::RichText::new("+ Créer").size(13.0).strong()).clicked() {
                state.show_quick_add = true;
                state.editing_entry_id = None;
            }
            ui.add_space(8.0);
            if ui.button(egui::RichText::new("📤 Exporter").size(12.0)).clicked() {
                state.show_export = !state.show_export;
            }
            ui.add_space(8.0);
            molecules::view_selector(ui, &mut state.current_view, theme);
            ui.add_space(8.0);
            if ui.button("🔍").clicked() { state.show_search = !state.show_search; }
        });
    });
    crate::ui::atoms::thin_separator(ui, theme.grid_line);
}

/// Sidebar gauche : mini-calendrier + liste des agendas.
pub fn sidebar(ui: &mut Ui, state: &mut AppState, agendas: &mut Vec<Agenda>, theme: &JayKoaTheme, db: &Arc<JayKoaDb>) {
    ui.set_min_width(theme.sidebar_width);
    ui.set_max_width(theme.sidebar_width);
    ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(8.0);
        mini_calendar(ui, state, theme);
        ui.add_space(16.0);
        crate::ui::atoms::thin_separator(ui, theme.grid_line);
        ui.add_space(8.0);
        ui.label(egui::RichText::new("Mes calendriers").size(12.0).color(theme.text_secondary));
        ui.add_space(4.0);
        for agenda in agendas.iter_mut() {
            let color_hex = agenda.color.as_deref().unwrap_or("#4285F4");
            let name = agenda.name.as_deref().unwrap_or("Sans nom");
            if molecules::agenda_row(ui, name, color_hex, &mut agenda.visible, theme) {
                if let Some(aid) = &agenda.id { let _ = db.agenda_set_visible(aid, agenda.visible); }
            }
        }

        // --- Section Filtres ---
        ui.add_space(16.0);
        crate::ui::atoms::thin_separator(ui, theme.grid_line);
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Filtres").size(12.0).color(theme.text_secondary));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let toggle_label = if state.filters_active { "✓ Actifs" } else { "○ Inactifs" };
                if ui.small_button(egui::RichText::new(toggle_label).size(10.0)).clicked() {
                    state.filters_active = !state.filters_active;
                }
            });
        });

        if state.filters_active {
            ui.add_space(8.0);
            // --- Filtres par Source ---
            ui.label(egui::RichText::new("Source").size(11.0).color(theme.text_primary).strong());
            ui.add_space(2.0);
            for (label, key) in [("JayKoa (interne)", "jaykoa"), ("JayFestival", "jayfestival"), ("JayRDV", "jayrdv")] {
                let mut checked = state.filter_sources.contains(key);
                if ui.checkbox(&mut checked, egui::RichText::new(label).size(11.0)).changed() {
                    if checked { state.filter_sources.insert(key.to_string()); }
                    else { state.filter_sources.remove(key); }
                }
            }

            ui.add_space(8.0);
            // --- Filtres par Type ---
            ui.label(egui::RichText::new("Type").size(11.0).color(theme.text_primary).strong());
            ui.add_space(2.0);
            for (label, key) in [
                ("Interne", "internal"),
                ("Reflet JayFestival", "reflect_jayfestival"),
                ("Reflet JayRDV", "reflect_jayrdv"),
                ("Autre", "reflect_other"),
            ] {
                let mut checked = state.filter_entry_types.contains(key);
                if ui.checkbox(&mut checked, egui::RichText::new(label).size(11.0)).changed() {
                    if checked { state.filter_entry_types.insert(key.to_string()); }
                    else { state.filter_entry_types.remove(key); }
                }
            }
        }
    });
}

/// Mini-calendrier (mois courant, cliquable).
pub fn mini_calendar(ui: &mut Ui, state: &mut AppState, theme: &JayKoaTheme) {
    let ref_date = state.reference_date;
    let today = Local::now().date_naive();
    let year = ref_date.year();
    let month = ref_date.month();

    ui.horizontal(|ui| {
        if ui.small_button("◀").clicked() {
            let (y, m) = if month == 1 { (year - 1, 12) } else { (year, month - 1) };
            if let Some(d) = NaiveDate::from_ymd_opt(y, m, 1) { state.reference_date = d; }
        }
        ui.label(egui::RichText::new(format!("{} {}", crate::app_state::month_name_fr(month), year)).size(12.0).strong());
        if ui.small_button("▶").clicked() {
            let (y, m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
            if let Some(d) = NaiveDate::from_ymd_opt(y, m, 1) { state.reference_date = d; }
        }
    });
    ui.add_space(4.0);
    ui.horizontal(|ui| { for d in &["Lu","Ma","Me","Je","Ve","Sa","Di"] { ui.label(egui::RichText::new(*d).size(10.0).color(theme.text_muted)); } });

    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap_or(ref_date);
    let start_wd = first_day.weekday().num_days_from_monday();
    let mut day_cursor = first_day - chrono::Duration::days(start_wd as i64);

    for _week in 0..6 {
        ui.horizontal(|ui| {
            for _d in 0..7 {
                let in_month = day_cursor.month() == month;
                let is_today = day_cursor == today;
                let is_selected = day_cursor == ref_date;
                let text = format!("{}", day_cursor.day());
                let tc = if !in_month { theme.text_muted } else if is_today { Color32::WHITE } else { theme.text_primary };
                let fill = if is_today { theme.today_highlight } else if is_selected { theme.accent.linear_multiply(0.2) } else { Color32::TRANSPARENT };
                let btn = egui::Button::new(egui::RichText::new(&text).size(10.0).color(tc))
                    .min_size(Vec2::splat(20.0)).fill(fill).corner_radius(egui::CornerRadius::same(10));
                if ui.add(btn).clicked() { state.reference_date = day_cursor; }
                day_cursor = day_cursor.succ_opt().unwrap_or(day_cursor);
            }
        });
        if day_cursor.month() != month && day_cursor.day() > 7 { break; }
    }
}
