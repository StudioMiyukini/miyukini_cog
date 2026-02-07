//! Écran d'export JayKoa — popover de génération iCal / PDF (futur).
//!
//! @id: jaykoa_screen_export
//! @do: render_export_popover_with_ical_generation
//! @role: ui
//! @layer: app
//! @human: Panneau d'export : l'utilisateur choisit un format (iCal pour l'instant),
//!         génère l'export, et peut copier le contenu .ics depuis un TextEdit readonly.
//!         PDF prévu en Phase 2 (bouton grisé).

use crate::app_state::AppState;
use crate::data::JayKoaDb;
use crate::export::ical;
use crate::theme::JayKoaTheme;
use chrono::Datelike;
use egui::{ScrollArea, Ui};
use std::sync::Arc;

/// Format d'export sélectionné.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExportFormat {
    ICal,
    Pdf,
}

/// État interne du panneau d'export (persisté entre frames via `static`-like pattern
/// ou stocké dans un `once_cell` local ; ici on utilise des variables egui `memory`).
struct ExportPanelState {
    format: ExportFormat,
    generated_content: String,
    generated: bool,
}

impl Default for ExportPanelState {
    fn default() -> Self {
        Self {
            format: ExportFormat::ICal,
            generated_content: String::new(),
            generated: false,
        }
    }
}

/// Affiche le panneau d'export (popover/panneau central).
///
/// Charge les entrées visibles via `db.entries_in_range()` avec les mêmes
/// paramètres que le calendrier, puis génère le contenu iCal.
pub fn show_export(
    ui: &mut Ui,
    state: &mut AppState,
    db: &Arc<JayKoaDb>,
    theme: &JayKoaTheme,
) {
    egui::Window::new("Exporter l'agenda")
        .collapsible(false)
        .resizable(true)
        .default_width(520.0)
        .default_height(480.0)
        .show(ui.ctx(), |ui| {
            // --- Récupérer l'état persisté dans la mémoire egui ---
            let panel_id = ui.id().with("export_panel_state");
            let mut panel = ui
                .ctx()
                .data_mut(|d| d.get_temp::<ExportPanelState>(panel_id))
                .unwrap_or_default();

            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("Exporter l'agenda")
                    .size(18.0)
                    .strong()
                    .color(theme.text_primary),
            );
            ui.add_space(12.0);

            // --- Sélection du format ---
            ui.label(
                egui::RichText::new("Format d'export")
                    .size(13.0)
                    .strong()
                    .color(theme.text_secondary),
            );
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.radio_value(&mut panel.format, ExportFormat::ICal, "iCalendar (.ics)");
                ui.add_enabled_ui(false, |ui| {
                    ui.radio_value(&mut panel.format, ExportFormat::Pdf, "PDF (Phase 2)");
                });
            });
            ui.add_space(12.0);

            // --- Bouton Générer ---
            if ui
                .button(
                    egui::RichText::new("📥 Générer l'export")
                        .size(14.0)
                        .strong(),
                )
                .clicked()
            {
                let entries = load_visible_entries(state, db);
                panel.generated_content = ical::entries_to_ical(&entries, "JayKoa Export");
                panel.generated = true;
            }
            ui.add_space(12.0);

            // --- Contenu généré ---
            if panel.generated {
                ui.label(
                    egui::RichText::new(
                        "Fichier .ics généré. Copiez le contenu ou utilisez un gestionnaire de fichiers.",
                    )
                    .size(11.0)
                    .color(theme.text_muted),
                );
                ui.add_space(8.0);

                ScrollArea::vertical()
                    .max_height(280.0)
                    .show(ui, |ui| {
                        let mut content = panel.generated_content.clone();
                        ui.add(
                            egui::TextEdit::multiline(&mut content)
                                .desired_width(f32::INFINITY)
                                .desired_rows(16)
                                .interactive(false)
                                .font(egui::TextStyle::Monospace),
                        );
                    });
                ui.add_space(8.0);
            }

            // --- Bouton Fermer ---
            ui.horizontal(|ui| {
                if ui.button("Fermer").clicked() {
                    state.show_export = false;
                    // Réinitialiser l'état du panneau
                    panel.generated = false;
                    panel.generated_content.clear();
                }
            });

            // --- Persister l'état dans la mémoire egui ---
            ui.ctx().data_mut(|d| d.insert_temp(panel_id, panel));
        });
}

/// Charge les entrées visibles pour la plage courante (même logique que le
/// calendrier dans `app.rs`).
fn load_visible_entries(state: &AppState, db: &Arc<JayKoaDb>) -> Vec<crate::data::types::TemporalEntry> {
    let agendas = db.agendas_by_profile(&state.profile_id).unwrap_or_default();
    let vis: Vec<String> = agendas
        .iter()
        .filter(|a| a.visible)
        .filter_map(|a| a.id.clone())
        .collect();
    if vis.is_empty() {
        return Vec::new();
    }

    let (start, end) = match state.current_view {
        crate::app_state::ViewId::Day => {
            let d = state.reference_date;
            (format!("{}T00:00:00", d), format!("{}T23:59:59", d))
        }
        crate::app_state::ViewId::Week => {
            let ws = state.week_start();
            let we = state.week_end();
            (format!("{}T00:00:00", ws), format!("{}T23:59:59", we))
        }
        crate::app_state::ViewId::Month => {
            let (y, m) = (state.reference_date.year(), state.reference_date.month());
            let first = chrono::NaiveDate::from_ymd_opt(y, m, 1).unwrap_or(state.reference_date);
            let last = (if m == 12 {
                chrono::NaiveDate::from_ymd_opt(y + 1, 1, 1)
            } else {
                chrono::NaiveDate::from_ymd_opt(y, m + 1, 1)
            })
            .unwrap_or(state.reference_date)
                - chrono::Duration::days(1);
            (
                format!("{}T00:00:00", first),
                format!("{}T23:59:59", last),
            )
        }
        _ => {
            let ws = state.week_start();
            (
                format!("{}T00:00:00", ws),
                format!("{}T23:59:59", ws + chrono::Duration::days(30)),
            )
        }
    };

    db.entries_in_range(&vis, &start, &end).unwrap_or_default()
}

/// Implémentation de Clone pour `ExportPanelState` (requis par egui `data_mut`).
impl Clone for ExportPanelState {
    fn clone(&self) -> Self {
        Self {
            format: self.format,
            generated_content: self.generated_content.clone(),
            generated: self.generated,
        }
    }
}
