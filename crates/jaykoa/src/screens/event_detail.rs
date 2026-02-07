//! Popover de détail d'un événement JayKoa.
//!
//! @id: jaykoa_screen_event_detail
//! @do: render_event_detail_popover
//! @role: ui
//! @layer: app
//! INVARIANTS : L'UI est un reflet. Les reflets externes sont en lecture seule.

use crate::app_state::AppState;
use crate::data::types::{EntryType, EventSource, TemporalEntry};
use crate::data::JayKoaDb;
use crate::theme::JayKoaTheme;
use crate::ui::molecules;
use egui::Context;
use std::sync::Arc;

/// Affiche la popover de détail d'un événement (fenêtre flottante au-dessus de l'agenda).
pub fn show_event_detail(ctx: &Context, state: &mut AppState, entry: &TemporalEntry, theme: &JayKoaTheme, db: &Arc<JayKoaDb>) {
    let et = EntryType::from_str(entry.entry_type.as_deref().unwrap_or("internal"));
    let ro = et.is_readonly();
    egui::Window::new("Détail de l'événement").collapsible(false).resizable(false).default_width(380.0).show(ctx, |ui| {
        ui.heading(egui::RichText::new(entry.title.as_deref().unwrap_or("(sans titre)")).size(18.0).strong());
        ui.add_space(8.0);
        if ro {
            let src = EventSource::from_str(entry.source_service.as_deref().unwrap_or("other"));
            let sc = molecules::hex_to_color32(src.default_color());
            ui.horizontal(|ui| { crate::ui::atoms::source_badge(ui, src.display_label(), sc); });
            ui.label(egui::RichText::new("Événement synchronisé (lecture seule)").size(11.0).color(theme.text_muted));
            ui.add_space(8.0);
        }
        if let (Some(s), Some(e)) = (&entry.start_datetime, &entry.end_datetime) {
            ui.horizontal(|ui| { ui.label("🕐");
                if entry.all_day { ui.label(fmt_date_range(s, e)); } else { ui.label(fmt_dt_range(s, e)); }
            }); ui.add_space(4.0);
        }
        if let Some(loc) = &entry.location { if !loc.is_empty() { ui.horizontal(|ui| { ui.label("📍"); ui.label(loc); }); ui.add_space(4.0); } }
        if let Some(desc) = &entry.description { if !desc.is_empty() { ui.label(egui::RichText::new("Description").size(12.0).strong()); ui.label(desc); ui.add_space(4.0); } }
        if let Some(st) = &entry.status {
            ui.horizontal(|ui| { ui.label(egui::RichText::new("Statut :").size(12.0).color(theme.text_secondary));
                let (l, c) = match st.as_str() { "confirmed"=>("Confirmé", egui::Color32::from_rgb(51,182,121)),
                    "tentative"=>("Provisoire", egui::Color32::from_rgb(251,188,4)),
                    "cancelled"=>("Annulé", egui::Color32::from_rgb(234,67,53)), _=>("?", theme.text_muted) };
                ui.label(egui::RichText::new(l).color(c).strong()); });
        }
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if !ro {
                if ui.button("Modifier").clicked() { state.show_event_detail = false; state.editing_entry_id = entry.id.clone(); state.show_full_editor = true; }
                if ui.button(egui::RichText::new("Supprimer").color(egui::Color32::from_rgb(234,67,53))).clicked() {
                    if let Some(eid) = &entry.id { let _ = db.entry_delete(eid); }
                    state.show_event_detail = false; state.detail_entry_id = None;
                }
            }
            if ui.button("Fermer").clicked() { state.show_event_detail = false; state.detail_entry_id = None; }
        });
    });
}

fn fmt_dt_range(s: &str, e: &str) -> String {
    let sd = &s[..10.min(s.len())]; let st = s.split('T').nth(1).map(|t| &t[..5.min(t.len())]).unwrap_or("--:--");
    let ed = &e[..10.min(e.len())]; let et = e.split('T').nth(1).map(|t| &t[..5.min(t.len())]).unwrap_or("--:--");
    if sd == ed { format!("{} · {} - {}", sd, st, et) } else { format!("{} {} → {} {}", sd, st, ed, et) }
}
fn fmt_date_range(s: &str, e: &str) -> String {
    let sd = &s[..10.min(s.len())]; let ed = &e[..10.min(e.len())];
    if sd == ed { format!("{} (journée)", sd) } else { format!("{} → {}", sd, ed) }
}
