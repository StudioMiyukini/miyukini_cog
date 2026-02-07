//! XP-P05 — Recherche transverse.
//!
//! Recherche parmi tous les exposants : filtrage en mémoire sur
//! company_name, description_short et secteur. Résultats cliquables
//! vers la fiche exposant.

use crate::data::JayXposeDb;
use crate::screens::pub_::PubState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use crate::ui;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_pub_e05
/// @do: render_recherche_transverse_exposants
/// @layer: app
///
/// Affiche l'écran de recherche transverse avec filtrage en mémoire.
pub fn pub_e05_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut PubState,
    db: &Arc<JayXposeDb>,
) {
    // S'assurer que la liste des exposants est chargée.
    if !state.annuaire_loaded {
        if let Ok(list) = db.exposants_list_annuaire() {
            state.annuaire_exposants = list;
        }
        state.annuaire_loaded = true;
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // ── En-tête ──
            ui::heading(ui, theme, "Recherche");
            ui.add_space(theme.item_spacing());

            // ── Champ de recherche ──
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Rechercher :")
                        .size(theme.font_size_body())
                        .color(theme.text_secondary()),
                );
                let response = ui.text_edit_singleline(&mut state.search_query);

                ui.add_space(theme.item_spacing());

                // Bouton rechercher (ou Enter).
                let search_clicked =
                    ui::accent_button(ui, theme, "Rechercher").clicked();
                let enter_pressed =
                    response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if search_clicked || enter_pressed {
                    // Le filtrage se fait ci-dessous, pas besoin d'action supplémentaire.
                    ctx.request_repaint();
                }
            });

            ui.add_space(theme.section_spacing());
            ui.separator();
            ui.add_space(theme.item_spacing());

            // ── Résultats ──
            let query_lower = state.search_query.to_lowercase();

            if query_lower.is_empty() {
                ui::caption(
                    ui,
                    theme,
                    "Saisissez un terme pour rechercher parmi les exposants.",
                );
            } else {
                let results: Vec<_> = state
                    .annuaire_exposants
                    .iter()
                    .filter(|e| {
                        e.company_name
                            .as_deref()
                            .unwrap_or_default()
                            .to_lowercase()
                            .contains(&query_lower)
                            || e.description_short
                                .as_deref()
                                .unwrap_or_default()
                                .to_lowercase()
                                .contains(&query_lower)
                            || e.secteur
                                .as_deref()
                                .unwrap_or_default()
                                .to_lowercase()
                                .contains(&query_lower)
                    })
                    .collect();

                ui::caption(
                    ui,
                    theme,
                    &format!(
                        "{} résultat{}",
                        results.len(),
                        if results.len() > 1 { "s" } else { "" }
                    ),
                );
                ui.add_space(theme.item_spacing());

                if results.is_empty() {
                    ui::caption(
                        ui,
                        theme,
                        "Aucun exposant ne correspond à votre recherche.",
                    );
                } else {
                    for exposant in &results {
                        ui::card(ui, theme, |ui| {
                            let name = exposant
                                .company_name
                                .as_deref()
                                .unwrap_or("Sans nom");
                            ui.label(
                                egui::RichText::new(name)
                                    .size(theme.font_size_body())
                                    .color(theme.text_primary())
                                    .strong(),
                            );

                            if let Some(ref secteur) = exposant.secteur {
                                ui::badge(ui, secteur, theme.accent());
                            }

                            if let Some(ref desc) = exposant.description_short {
                                ui::caption(ui, theme, desc);
                            }

                            ui.add_space(4.0);

                            if ui::accent_button(ui, theme, "Voir la fiche").clicked() {
                                state.selected_exposant_id = exposant.id.clone();
                                state.fiche_loaded = false;
                                let _ = nav_request
                                    .replace(Some(ScreenId::PubFicheExposant));
                            }
                        });
                        ui.add_space(theme.item_spacing());
                    }
                }
            }

            ui.add_space(theme.section_spacing());

            // ── Navigation ──
            if ui::secondary_button(ui, theme, "Retour à l'annuaire").clicked() {
                let _ = nav_request.replace(Some(ScreenId::PubAnnuaire));
            }
        });
    });
}
