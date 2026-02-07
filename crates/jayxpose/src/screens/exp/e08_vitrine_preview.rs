//! XP-E08 — Prévisualisation vitrine.
//!
//! Rendu lecture seule des pages vitrine (onglets : Accueil, Catalogue, Présentation, Contact).

use crate::data::JayXposeDb;
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_exp_e08
/// @do: render_showcase_preview_readonly
/// @layer: app
/// Écran prévisualisation vitrine : onglets pages, rendu lecture seule, boutons retour/publier.
/// Affiche l'écran XP-E08 — Prévisualisation vitrine.
pub fn exp_e08_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    let tab_labels = ["Accueil", "Catalogue", "Présentation", "Contact"];
    let tab_types = ["accueil", "catalogue", "presentation", "contact"];

    egui::CentralPanel::default().show(ctx, |ui| {
        // --- En-tête ---
        ui.horizontal(|ui| {
            if ui.button("← Paramètres vitrine").clicked() {
                nav_request.replace(Some(ScreenId::ExpVitrineParams));
            }
            ui.heading("Prévisualisation vitrine");
        });
        ui.add_space(theme.item_spacing());

        // --- Onglets ---
        ui.horizontal(|ui| {
            for (i, label) in tab_labels.iter().enumerate() {
                let selected = state.vitrine_preview_tab == i;
                if ui
                    .selectable_label(selected, *label)
                    .clicked()
                {
                    state.vitrine_preview_tab = i;
                }
            }
        });
        ui.separator();
        ui.add_space(theme.item_spacing());

        // --- Contenu de l'onglet sélectionné ---
        egui::ScrollArea::vertical().show(ui, |ui| {
            let current_type = tab_types
                .get(state.vitrine_preview_tab)
                .copied()
                .unwrap_or("accueil");

            let page = state
                .vitrine_pages
                .iter()
                .find(|p| p.page_type.as_deref() == Some(current_type));

            egui::Frame::new()
                .fill(theme.section_background())
                .corner_radius(theme.border_radius())
                .inner_margin(egui::Margin::same(16))
                .show(ui, |ui| {
                    match page {
                        Some(p) => {
                            let visible = p.is_visible.unwrap_or(true);
                            if !visible {
                                ui.label(
                                    egui::RichText::new("Cette page est masquée.")
                                        .color(theme.text_secondary()),
                                );
                                return;
                            }

                            // Titre de la page.
                            ui.label(
                                egui::RichText::new(
                                    tab_labels
                                        .get(state.vitrine_preview_tab)
                                        .copied()
                                        .unwrap_or("Page"),
                                )
                                .size(theme.font_size_heading())
                                .color(theme.text_primary()),
                            );
                            ui.add_space(theme.item_spacing());

                            // Contenu.
                            let content =
                                p.content.as_deref().unwrap_or("Aucun contenu.");
                            ui.label(
                                egui::RichText::new(content)
                                    .color(theme.text_primary()),
                            );

                            // Pour l'onglet Catalogue, afficher un résumé produits.
                            if current_type == "catalogue" {
                                ui.add_space(theme.item_spacing());
                                ui.separator();
                                ui.add_space(theme.item_spacing());
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{} produit(s) dans le catalogue",
                                        state.produits.len()
                                    ))
                                    .color(theme.text_secondary()),
                                );
                                for prod in &state.produits {
                                    let pname =
                                        prod.name.as_deref().unwrap_or("Sans nom");
                                    let pprice = match prod.price {
                                        Some(p) => format!("{:.2} €", p),
                                        None => "Sur demande".to_string(),
                                    };
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new(pname)
                                                .color(theme.text_primary()),
                                        );
                                        ui.label(
                                            egui::RichText::new(pprice)
                                                .color(theme.text_secondary()),
                                        );
                                    });
                                }
                            }

                            // Pour l'onglet Accueil, afficher le profil résumé.
                            if current_type == "accueil" {
                                ui.add_space(theme.item_spacing());
                                let company = state
                                    .fiche_form
                                    .company_name
                                    .as_deref()
                                    .unwrap_or("Mon entreprise");
                                let slogan = state
                                    .fiche_form
                                    .slogan
                                    .as_deref()
                                    .unwrap_or("");
                                ui.label(
                                    egui::RichText::new(company)
                                        .size(theme.font_size_heading())
                                        .color(theme.accent()),
                                );
                                if !slogan.is_empty() {
                                    ui.label(
                                        egui::RichText::new(slogan)
                                            .color(theme.text_secondary()),
                                    );
                                }
                            }
                        }
                        None => {
                            ui.label(
                                egui::RichText::new(
                                    "Cette page n'a pas encore été configurée.",
                                )
                                .color(theme.text_secondary()),
                            );
                        }
                    }
                });

            ui.add_space(theme.section_spacing());

            // --- Boutons ---
            ui.horizontal(|ui| {
                if ui.button("Retour aux paramètres").clicked() {
                    nav_request.replace(Some(ScreenId::ExpVitrineParams));
                }
                if ui.button("Publier la vitrine").clicked() {
                    state.fiche_form.vitrine_status =
                        Some("publiee".to_string());
                    match db.exposant_upsert(&state.fiche_form) {
                        Ok(()) => {
                            state.status_message =
                                Some("Vitrine publiée avec succès.".to_string());
                        }
                        Err(e) => {
                            state.status_message =
                                Some(format!("Erreur publication : {}", e));
                        }
                    }
                }
            });

            // --- Message de statut ---
            if let Some(msg) = &state.status_message {
                ui.add_space(theme.item_spacing());
                ui.label(egui::RichText::new(msg).color(theme.accent()));
            }
        });
    });
}
