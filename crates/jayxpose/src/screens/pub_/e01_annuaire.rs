//! XP-P01 — Annuaire des exposants.
//!
//! Vue publique de l'annuaire : liste filtrable des exposants avec cartes,
//! recherche par texte et filtre par secteur d'activité.

use crate::data::JayXposeDb;
use crate::screens::pub_::PubState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use crate::ui;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_pub_e01
/// @do: render_annuaire_exposants_public
/// @layer: app
///
/// Affiche l'annuaire public des exposants avec filtres et grille de cartes.
pub fn pub_e01_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut PubState,
    db: &Arc<JayXposeDb>,
) {
    // Chargement initial des exposants.
    if !state.annuaire_loaded {
        if let Ok(list) = db.exposants_list_annuaire() {
            state.annuaire_exposants = list;
        }
        state.annuaire_loaded = true;
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // ── En-tête ──
            ui::heading(ui, theme, "Annuaire des exposants");
            ui.add_space(theme.item_spacing());

            let total = state.annuaire_exposants.len();
            ui::caption(
                ui,
                theme,
                &format!("{total} exposant{}", if total > 1 { "s" } else { "" }),
            );
            ui.add_space(theme.item_spacing());

            // ── Filtres ──
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Rechercher :")
                        .size(theme.font_size_body())
                        .color(theme.text_secondary()),
                );
                ui.text_edit_singleline(&mut state.search_query);

                ui.add_space(theme.item_spacing());

                // Collecter les secteurs distincts pour le filtre.
                let mut secteurs: Vec<String> = state
                    .annuaire_exposants
                    .iter()
                    .filter_map(|e| e.secteur.clone())
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();
                secteurs.sort();
                secteurs.insert(0, "Tous les secteurs".to_string());

                let current_label = state
                    .filter_secteur
                    .as_deref()
                    .unwrap_or("Tous les secteurs");
                egui::ComboBox::from_label("")
                    .selected_text(current_label)
                    .show_ui(ui, |ui| {
                        for s in &secteurs {
                            let is_all = s == "Tous les secteurs";
                            let selected = if is_all {
                                state.filter_secteur.is_none()
                            } else {
                                state.filter_secteur.as_deref() == Some(s.as_str())
                            };
                            if ui.selectable_label(selected, s).clicked() {
                                state.filter_secteur = if is_all {
                                    None
                                } else {
                                    Some(s.clone())
                                };
                            }
                        }
                    });
            });

            ui.add_space(theme.section_spacing());
            ui.separator();
            ui.add_space(theme.item_spacing());

            // ── Filtrage en mémoire ──
            let query_lower = state.search_query.to_lowercase();
            let filtered: Vec<_> = state
                .annuaire_exposants
                .iter()
                .filter(|e| {
                    // Filtre par texte.
                    let matches_text = query_lower.is_empty()
                        || e.company_name
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
                            .contains(&query_lower);

                    // Filtre par secteur.
                    let matches_secteur = match &state.filter_secteur {
                        Some(sec) => e.secteur.as_deref() == Some(sec.as_str()),
                        None => true,
                    };

                    matches_text && matches_secteur
                })
                .collect();

            // ── Grille de cartes (3 colonnes) ──
            let card_count = filtered.len();
            if card_count == 0 {
                ui.add_space(theme.section_spacing());
                ui::caption(ui, theme, "Aucun exposant trouvé.");
            } else {
                egui::Grid::new("annuaire_grid")
                    .num_columns(3)
                    .spacing([theme.item_spacing(), theme.item_spacing()])
                    .show(ui, |ui| {
                        for (i, exposant) in filtered.iter().enumerate() {
                            ui::card(ui, theme, |ui| {
                                // Logo placeholder.
                                if let Some(ref _logo) = exposant.logo_url {
                                    ui::caption(ui, theme, "[logo]");
                                }

                                // Nom entreprise.
                                let name =
                                    exposant.company_name.as_deref().unwrap_or("Sans nom");
                                ui.label(
                                    egui::RichText::new(name)
                                        .size(theme.font_size_body())
                                        .color(theme.text_primary())
                                        .strong(),
                                );

                                // Secteur.
                                if let Some(ref secteur) = exposant.secteur {
                                    ui::badge(ui, secteur, theme.accent());
                                }

                                // Description courte.
                                if let Some(ref desc) = exposant.description_short {
                                    ui::caption(ui, theme, desc);
                                }

                                ui.add_space(4.0);

                                // Bouton consulter.
                                if ui::accent_button(ui, theme, "Voir la fiche").clicked() {
                                    state.selected_exposant_id = exposant.id.clone();
                                    state.fiche_loaded = false;
                                    let _ = nav_request
                                        .replace(Some(ScreenId::PubFicheExposant));
                                }
                            });

                            // Nouvelle ligne toutes les 3 cartes.
                            if (i + 1) % 3 == 0 {
                                ui.end_row();
                            }
                        }
                    });
            }

            ui.add_space(theme.section_spacing());

            // ── Navigation ──
            ui.horizontal(|ui| {
                if ui::secondary_button(ui, theme, "Recherche avancée").clicked() {
                    let _ = nav_request.replace(Some(ScreenId::PubRecherche));
                }
            });
        });
    });
}
