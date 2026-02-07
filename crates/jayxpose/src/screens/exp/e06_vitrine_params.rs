//! XP-E06 — Vitrine paramètres.
//!
//! Configuration de la vitrine exposant : URL/slug, SEO, statut publication, activation pages.

use crate::data::{JayXposeDb, VitrineStatus};
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_exp_e06
/// @do: render_showcase_settings_form
/// @layer: app
/// Écran vitrine paramètres : slug/URL, SEO, statut publication, pages activées.
/// Affiche l'écran XP-E06 — Vitrine paramètres.
pub fn exp_e06_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // --- En-tête ---
            ui.horizontal(|ui| {
                if ui.button("← Tableau de bord").clicked() {
                    nav_request.replace(Some(ScreenId::ExpDashboard));
                }
                ui.heading("Paramètres vitrine");
            });
            ui.add_space(theme.section_spacing());

            let form = &mut state.fiche_form;

            // --- URL / Slug ---
            ui.collapsing("URL de la vitrine", |ui| {
                ui.add_space(theme.item_spacing());
                ui.label("Slug (identifiant URL)");
                let mut slug = form.vitrine_slug.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut slug).changed() {
                    form.vitrine_slug = Some(slug.clone());
                }
                ui.label(
                    egui::RichText::new(format!(
                        "Aperçu : https://jayxpose.example.com/vitrine/{}",
                        form.vitrine_slug.as_deref().unwrap_or("mon-entreprise")
                    ))
                    .color(theme.text_secondary()),
                );
            });
            ui.add_space(theme.item_spacing());

            // --- SEO ---
            ui.collapsing("SEO (référencement)", |ui| {
                ui.add_space(theme.item_spacing());
                ui.label("Titre SEO");
                let mut seo_title = form.seo_title.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut seo_title).changed() {
                    form.seo_title = Some(seo_title);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Description SEO");
                let mut seo_desc = form.seo_description.clone().unwrap_or_default();
                if ui.text_edit_multiline(&mut seo_desc).changed() {
                    form.seo_description = Some(seo_desc);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Mots-clés SEO (séparés par des virgules)");
                let mut seo_kw = form.seo_keywords.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut seo_kw).changed() {
                    form.seo_keywords = Some(seo_kw);
                }
            });
            ui.add_space(theme.item_spacing());

            // --- Statut vitrine ---
            ui.collapsing("Statut de la vitrine", |ui| {
                ui.add_space(theme.item_spacing());
                let current_status = VitrineStatus::from_str(
                    form.vitrine_status.as_deref().unwrap_or("brouillon"),
                );
                ui.label(
                    egui::RichText::new(format!("Statut actuel : {}", current_status.as_str()))
                        .size(theme.font_size_body())
                        .color(theme.accent()),
                );
                ui.add_space(theme.item_spacing());
                ui.horizontal(|ui| {
                    if ui.button("Publier").clicked() {
                        form.vitrine_status = Some(VitrineStatus::Publiee.as_str().to_string());
                    }
                    if ui.button("Suspendre").clicked() {
                        form.vitrine_status =
                            Some(VitrineStatus::Suspendue.as_str().to_string());
                    }
                    if ui.button("Brouillon").clicked() {
                        form.vitrine_status =
                            Some(VitrineStatus::Brouillon.as_str().to_string());
                    }
                });
            });
            ui.add_space(theme.item_spacing());

            // --- Pages vitrine ---
            ui.collapsing("Pages de la vitrine", |ui| {
                ui.add_space(theme.item_spacing());
                let page_types = ["accueil", "catalogue", "presentation", "contact"];
                for page_type in &page_types {
                    let existing = state
                        .vitrine_pages
                        .iter_mut()
                        .find(|p| p.page_type.as_deref() == Some(page_type));
                    match existing {
                        Some(page) => {
                            let mut visible = page.is_visible.unwrap_or(true);
                            if ui
                                .checkbox(&mut visible, format!("Page : {}", page_type))
                                .changed()
                            {
                                page.is_visible = Some(visible);
                            }
                        }
                        None => {
                            ui.label(
                                egui::RichText::new(format!(
                                    "Page {} (non configurée)",
                                    page_type
                                ))
                                .color(theme.text_secondary()),
                            );
                        }
                    }
                }
            });

            ui.add_space(theme.section_spacing());

            // --- Boutons ---
            ui.horizontal(|ui| {
                if ui.button("Enregistrer").clicked() {
                    // Sauvegarder le profil (contient les champs vitrine).
                    match db.exposant_upsert(&state.fiche_form) {
                        Ok(()) => {
                            // Sauvegarder les pages vitrine.
                            for page in &state.vitrine_pages {
                                let _ = db.vitrine_page_upsert(page);
                            }
                            state.status_message =
                                Some("Paramètres vitrine enregistrés.".to_string());
                        }
                        Err(e) => {
                            state.status_message =
                                Some(format!("Erreur enregistrement : {}", e));
                        }
                    }
                }
                if ui.button("Prévisualiser").clicked() {
                    nav_request.replace(Some(ScreenId::ExpVitrinePreview));
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
