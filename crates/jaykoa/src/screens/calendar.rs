//! Vue calendrier principale — grille semaine / jour / mois.
//!
//! @id: jaykoa_screen_calendar
//! @do: render_week_day_month_calendar_grid
//! @role: ui
//! @layer: app
//! @human: Affiche la grille temporelle avec les engagements positionnés.
//!
//! INVARIANT : cette vue est un REFLET de l'état. Aucune décision métier.

use crate::app_state::{AppState, ViewId};
use crate::data::types::{EntryType, TemporalEntry};
use crate::theme::JayKoaTheme;
use crate::ui::{atoms, molecules};
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, Timelike};
use egui::{Color32, Rect, ScrollArea, Ui, Vec2};

/// Données pré-chargées pour le rendu calendrier.
pub struct CalendarData {
    /// Entrées à afficher dans la plage courante.
    pub entries: Vec<TemporalEntry>,
    /// Conflits détectés (paires d'IDs).
    pub conflict_pairs: Vec<(String, String)>,
}

/// Affiche la vue calendrier selon la vue active.
pub fn show_calendar(
    ui: &mut Ui, state: &mut AppState, cal_data: &CalendarData,
    agendas_colors: &std::collections::HashMap<String, String>, theme: &JayKoaTheme,
) {
    match state.current_view {
        ViewId::Week => show_week_view(ui, state, cal_data, agendas_colors, theme),
        ViewId::Day => show_day_view(ui, state, cal_data, agendas_colors, theme),
        ViewId::Month => show_month_view(ui, state, cal_data, agendas_colors, theme),
        ViewId::Schedule => show_schedule_view(ui, state, cal_data, agendas_colors, theme),
        ViewId::Year => { ui.centered_and_justified(|ui| { ui.label(egui::RichText::new("Vue Année — à venir").size(16.0).color(theme.text_muted)); }); }
        ViewId::Settings => {}
    }
}

// -- Vue Semaine ------------------------------------------------------------
fn show_week_view(ui: &mut Ui, state: &mut AppState, cal_data: &CalendarData,
    agendas_colors: &std::collections::HashMap<String, String>, theme: &JayKoaTheme) {
    let week_start = state.week_start();
    let today = Local::now().date_naive();
    let now = Local::now().naive_local();

    // Header jours
    ui.horizontal(|ui| {
        ui.add_space(theme.hour_column_width);
        let col_width = (ui.available_width() - 8.0) / 7.0;
        for d in 0..7u32 {
            let date = week_start + chrono::Duration::days(d as i64);
            let is_today = date == today;
            ui.vertical(|ui| {
                ui.set_min_width(col_width); ui.set_max_width(col_width);
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new(day_short_fr(date.weekday())).size(11.0)
                        .color(if is_today { theme.accent } else { theme.text_secondary }));
                    if is_today { atoms::today_circle(ui, date.day(), theme); }
                    else { ui.label(egui::RichText::new(format!("{}", date.day())).size(13.0).color(theme.text_primary)); }
                });
            });
        }
    });
    atoms::thin_separator(ui, theme.grid_line);

    // Grille horaire scrollable
    ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        let available_width = ui.available_width();
        let col_width = (available_width - theme.hour_column_width - 8.0) / 7.0;
        let total_height = theme.hour_height * 24.0;
        let (grid_rect, _) = ui.allocate_exact_size(Vec2::new(available_width, total_height), egui::Sense::click());
        let painter = ui.painter_at(grid_rect);

        // Lignes horizontales (heures)
        let grid_stroke = theme.grid_line_stroke();
        for h in 0..24u8 {
            let y = grid_rect.top() + h as f32 * theme.hour_height;
            painter.line_segment([egui::pos2(grid_rect.left() + theme.hour_column_width, y), egui::pos2(grid_rect.right(), y)],
                grid_stroke);
            painter.text(egui::pos2(grid_rect.left() + 4.0, y + 2.0), egui::Align2::LEFT_TOP,
                format!("{:02}:00", h), egui::FontId::proportional(10.0), theme.text_secondary);
        }
        // Lignes verticales (jours)
        for d in 0..=7u32 {
            let x = grid_rect.left() + theme.hour_column_width + d as f32 * col_width;
            painter.line_segment([egui::pos2(x, grid_rect.top()), egui::pos2(x, grid_rect.bottom())],
                grid_stroke);
        }
        // Marqueur "Maintenant"
        if today >= week_start && today <= week_start + chrono::Duration::days(6) {
            let day_off = (today - week_start).num_days() as f32;
            let time_frac = now.hour() as f32 / 24.0 + now.minute() as f32 / 1440.0;
            let y = grid_rect.top() + time_frac * total_height;
            let x_s = grid_rect.left() + theme.hour_column_width + day_off * col_width;
            painter.circle_filled(egui::pos2(x_s, y), 4.0, theme.now_marker);
            painter.line_segment([egui::pos2(x_s, y), egui::pos2(x_s + col_width, y)], egui::Stroke::new(2.0, theme.now_marker));
        }
        // Événements
        for entry in &cal_data.entries {
            if let (Some(s_str), Some(e_str)) = (&entry.start_datetime, &entry.end_datetime) {
                if let (Some(s_dt), Some(e_dt)) = (parse_dt(s_str), parse_dt(e_str)) {
                    let ed = s_dt.date();
                    if ed < week_start || ed > week_start + chrono::Duration::days(6) { continue; }
                    let day_off = (ed - week_start).num_days() as f32;
                    let sf = s_dt.hour() as f32 / 24.0 + s_dt.minute() as f32 / 1440.0;
                    let ef = e_dt.hour() as f32 / 24.0 + e_dt.minute() as f32 / 1440.0;
                    let ys = grid_rect.top() + sf * total_height;
                    let h = ((ef - sf) * total_height).max(theme.hour_height * 0.5);
                    let x = grid_rect.left() + theme.hour_column_width + day_off * col_width + 2.0;
                    let er = Rect::from_min_size(egui::pos2(x, ys), Vec2::new(col_width - 4.0, h));
                    let color = entry_color(entry, agendas_colors, theme);
                    let ro = EntryType::from_str(entry.entry_type.as_deref().unwrap_or("internal")).is_readonly();
                    render_block(&painter, entry, er, color, ro);
                    if ui.rect_contains_pointer(er) && ui.input(|i| i.pointer.primary_clicked()) {
                        state.show_event_detail = true; state.detail_entry_id = entry.id.clone();
                    }
                }
            }
        }
    });
}

// -- Vue Jour ---------------------------------------------------------------
fn show_day_view(ui: &mut Ui, state: &mut AppState, cal_data: &CalendarData,
    agendas_colors: &std::collections::HashMap<String, String>, theme: &JayKoaTheme) {
    let ref_date = state.reference_date;
    let today = Local::now().date_naive();
    ui.horizontal(|ui| {
        ui.add_space(theme.hour_column_width);
        let is_today = ref_date == today;
        ui.label(egui::RichText::new(format!("{} {}", day_full_fr(ref_date.weekday()), ref_date.day())).size(16.0).strong()
            .color(if is_today { theme.accent } else { theme.text_primary }));
    });
    atoms::thin_separator(ui, theme.grid_line);

    ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        let aw = ui.available_width();
        let cw = aw - theme.hour_column_width - 8.0;
        let th = theme.hour_height * 24.0;
        let (gr, _) = ui.allocate_exact_size(Vec2::new(aw, th), egui::Sense::click());
        let p = ui.painter_at(gr);
        let grid_stroke = theme.grid_line_stroke();
        for h in 0..24u8 {
            let y = gr.top() + h as f32 * theme.hour_height;
            p.line_segment([egui::pos2(gr.left() + theme.hour_column_width, y), egui::pos2(gr.right(), y)], grid_stroke);
            p.text(egui::pos2(gr.left() + 4.0, y + 2.0), egui::Align2::LEFT_TOP, format!("{:02}:00", h), egui::FontId::proportional(10.0), theme.text_secondary);
        }
        for entry in &cal_data.entries {
            if let (Some(ss), Some(es)) = (&entry.start_datetime, &entry.end_datetime) {
                if let (Some(sd), Some(ed)) = (parse_dt(ss), parse_dt(es)) {
                    if sd.date() != ref_date { continue; }
                    let sf = sd.hour() as f32 / 24.0 + sd.minute() as f32 / 1440.0;
                    let ef = ed.hour() as f32 / 24.0 + ed.minute() as f32 / 1440.0;
                    let ys = gr.top() + sf * th;
                    let h = ((ef - sf) * th).max(theme.hour_height * 0.5);
                    let er = Rect::from_min_size(egui::pos2(gr.left() + theme.hour_column_width + 4.0, ys), Vec2::new(cw - 8.0, h));
                    let color = entry_color(entry, agendas_colors, theme);
                    let ro = EntryType::from_str(entry.entry_type.as_deref().unwrap_or("internal")).is_readonly();
                    render_block(&p, entry, er, color, ro);
                    if ui.rect_contains_pointer(er) && ui.input(|i| i.pointer.primary_clicked()) {
                        state.show_event_detail = true; state.detail_entry_id = entry.id.clone();
                    }
                }
            }
        }
    });
}

// -- Vue Mois ---------------------------------------------------------------
fn show_month_view(ui: &mut Ui, state: &mut AppState, cal_data: &CalendarData,
    agendas_colors: &std::collections::HashMap<String, String>, theme: &JayKoaTheme) {
    let ref_date = state.reference_date;
    let today = Local::now().date_naive();
    let (year, month) = (ref_date.year(), ref_date.month());
    ui.horizontal(|ui| {
        let cw = ui.available_width() / 7.0;
        for d in &["Lundi","Mardi","Mercredi","Jeudi","Vendredi","Samedi","Dimanche"] {
            ui.vertical(|ui| { ui.set_min_width(cw); ui.set_max_width(cw);
                ui.label(egui::RichText::new(*d).size(11.0).color(theme.text_secondary)); });
        }
    });
    atoms::thin_separator(ui, theme.grid_line);

    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap_or(ref_date);
    let swd = first.weekday().num_days_from_monday();
    let mut dc = first - chrono::Duration::days(swd as i64);
    let cw = ui.available_width() / 7.0;

    for _w in 0..6 {
        ui.horizontal(|ui| {
            for _d in 0..7 {
                let in_m = dc.month() == month;
                let is_t = dc == today;
                let ds = dc.format("%Y-%m-%d").to_string();
                let day_entries: Vec<&TemporalEntry> = cal_data.entries.iter()
                    .filter(|e| e.start_datetime.as_ref().map(|s| s.starts_with(&ds)).unwrap_or(false)).collect();
                ui.vertical(|ui| {
                    ui.set_min_width(cw); ui.set_max_width(cw); ui.set_min_height(80.0);
                    let tc = if !in_m { theme.text_muted } else if is_t { theme.accent } else { theme.text_primary };
                    ui.label(egui::RichText::new(format!("{}", dc.day())).size(12.0).color(tc));
                    for entry in day_entries.iter().take(3) {
                        let t = entry.title.as_deref().unwrap_or("...");
                        let c = entry_color(entry, agendas_colors, theme);
                        ui.horizontal(|ui| {
                            let (dr, _) = ui.allocate_exact_size(Vec2::splat(6.0), egui::Sense::hover());
                            ui.painter_at(dr).circle_filled(dr.center(), 3.0, c);
                            let r = ui.label(egui::RichText::new(trunc(t, 15)).size(10.0).color(theme.text_primary));
                            if r.clicked() { state.show_event_detail = true; state.detail_entry_id = entry.id.clone(); }
                        });
                    }
                    if day_entries.len() > 3 { ui.label(egui::RichText::new(format!("+{}", day_entries.len()-3)).size(9.0).color(theme.text_muted)); }
                });
                dc = dc.succ_opt().unwrap_or(dc);
            }
        });
        atoms::thin_separator(ui, theme.grid_line);
        if dc.month() != month && dc.day() > 7 { break; }
    }
}

// -- Vue Planning -----------------------------------------------------------
fn show_schedule_view(ui: &mut Ui, state: &mut AppState, cal_data: &CalendarData,
    agendas_colors: &std::collections::HashMap<String, String>, theme: &JayKoaTheme) {
    if cal_data.entries.is_empty() {
        ui.centered_and_justified(|ui| { ui.label(egui::RichText::new("Aucun événement").size(14.0).color(theme.text_muted)); });
        return;
    }
    ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        let mut last_date: Option<NaiveDate> = None;
        for entry in &cal_data.entries {
            if let Some(ss) = &entry.start_datetime {
                if let Some(sd) = parse_dt(ss) {
                    let date = sd.date();
                    if last_date != Some(date) {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new(date.format("%A %d %B %Y").to_string()).size(13.0).strong());
                        atoms::thin_separator(ui, theme.grid_line);
                        last_date = Some(date);
                    }
                    let color = entry_color(entry, agendas_colors, theme);
                    let ro = EntryType::from_str(entry.entry_type.as_deref().unwrap_or("internal")).is_readonly();
                    ui.horizontal(|ui| {
                        let (dr, _) = ui.allocate_exact_size(Vec2::splat(10.0), egui::Sense::hover());
                        ui.painter_at(dr).circle_filled(dr.center(), 5.0, color);
                        let ts = ss.split('T').nth(1).map(|t| &t[..5.min(t.len())]).unwrap_or("--:--");
                        ui.label(egui::RichText::new(ts).size(12.0).color(theme.text_secondary));
                        let r = ui.label(egui::RichText::new(entry.title.as_deref().unwrap_or("?")).size(13.0));
                        if r.clicked() { state.show_event_detail = true; state.detail_entry_id = entry.id.clone(); }
                        if ro { ui.label(egui::RichText::new(format!("[{}]", entry.source_service.as_deref().unwrap_or("?"))).size(10.0).color(color)); }
                    });
                }
            }
        }
    });
}

// -- Helpers ----------------------------------------------------------------
fn entry_color(e: &TemporalEntry, ac: &std::collections::HashMap<String, String>, theme: &JayKoaTheme) -> Color32 {
    if let Some(c) = &e.color { return molecules::hex_to_color32(c); }
    if let Some(s) = &e.source_service { return theme.event_color_for_source(s); }
    if let Some(et) = &e.entry_type { return theme.event_color_for_source(et); }
    if let Some(aid) = &e.agenda_id { if let Some(c) = ac.get(aid) { return molecules::hex_to_color32(c); } }
    theme.event_internal
}

fn render_block(p: &egui::Painter, e: &TemporalEntry, r: Rect, c: Color32, ro: bool) {
    let bg = if ro { c.linear_multiply(0.15) } else { c.linear_multiply(0.85) };
    p.rect_filled(r, egui::CornerRadius::same(4), bg);
    if ro { p.rect_filled(Rect::from_min_size(r.min, Vec2::new(3.0, r.height())), egui::CornerRadius::ZERO, c); }
    let tc = if ro { c } else { Color32::WHITE };
    p.text(r.min + Vec2::new(6.0, 3.0), egui::Align2::LEFT_TOP, trunc(e.title.as_deref().unwrap_or("?"), 25), egui::FontId::proportional(11.0), tc);
    if let Some(s) = &e.start_datetime { if let Some(tp) = s.split('T').nth(1) {
        p.text(r.min + Vec2::new(6.0, 16.0), egui::Align2::LEFT_TOP, &tp[..5.min(tp.len())], egui::FontId::proportional(9.0), tc.linear_multiply(0.7));
    }}
}

fn parse_dt(s: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S").or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")).ok()
}
fn trunc(s: &str, m: usize) -> &str { if s.len() <= m { s } else { &s[..m] } }
fn day_short_fr(wd: chrono::Weekday) -> &'static str { match wd { chrono::Weekday::Mon=>"Lun.", chrono::Weekday::Tue=>"Mar.", chrono::Weekday::Wed=>"Mer.", chrono::Weekday::Thu=>"Jeu.", chrono::Weekday::Fri=>"Ven.", chrono::Weekday::Sat=>"Sam.", chrono::Weekday::Sun=>"Dim." } }
fn day_full_fr(wd: chrono::Weekday) -> &'static str { match wd { chrono::Weekday::Mon=>"Lundi", chrono::Weekday::Tue=>"Mardi", chrono::Weekday::Wed=>"Mercredi", chrono::Weekday::Thu=>"Jeudi", chrono::Weekday::Fri=>"Vendredi", chrono::Weekday::Sat=>"Samedi", chrono::Weekday::Sun=>"Dimanche" } }
