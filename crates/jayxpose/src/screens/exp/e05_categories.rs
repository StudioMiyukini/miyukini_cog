//! XP-E05 — Gestion des catégories produits.
//!
//! Liste des catégories avec compteur produits, ajout et suppression.

use crate::data::{CategorieProduit, JayXposeDb};
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_exp_e05
/// @do: render_category_management_screen
/// @layer: app
/// Écran gestion des catégories : liste avec compteur, ajout, suppression.
/// Affiche l'écran XP-E05 — Gestion des catégories.
pub fn exp_e05_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    // Rechargement si nécessaire.
    if state.categories.is_empty() {
        if let Some(ref eid) = state.current_exposant_id {
            if let Ok(cats) = db.categories_by_exposant(eid) {
                state.categories = cats;
            }
        }
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // --- En-tête ---
            ui.horizontal(|ui| {
                if ui.button("← Tableau de bord").clicked() {
                    nav_request.replace(Some(ScreenId::ExpDashboard));
                }
                ui.heading("Gestion des catégories");
            });
            ui.add_space(theme.section_spacing());

            // --- Ajout de catégorie ---
            ui.horizontal(|ui| {
                ui.label("Nouvelle catégorie :");
                ui.text_edit_singleline(&mut state.new_category_name);
                if ui.button("Ajouter").clicked() && !state.new_category_name.trim().is_empty()
                {
                    let new_cat = CategorieProduit {
                        id: None,
                        exposant_id: state.current_exposant_id.clone(),
                        name: Some(state.new_category_name.trim().to_string()),
                        description: None,
                        sort_order: Some(state.categories.len() as i32),
                        created_at: None,
                    };
                    match db.categorie_insert(&new_cat) {
                        Ok(()) => {
                            state.new_category_name.clear();
                            state.status_message =
                                Some("Catégorie ajoutée.".to_string());
                            // Recharger.
                            if let Some(ref eid) = state.current_exposant_id {
                                if let Ok(cats) = db.categories_by_exposant(eid) {
                                    state.categories = cats;
                                }
                            }
                        }
                        Err(e) => {
                            state.status_message =
                                Some(format!("Erreur ajout catégorie : {}", e));
                        }
                    }
                }
            });
            ui.add_space(theme.item_spacing());
            ui.separator();
            ui.add_space(theme.item_spacing());

            // --- Liste des catégories ---
            if state.categories.is_empty() {
                ui.label(
                    egui::RichText::new("Aucune catégorie pour l'instant.")
                        .color(theme.text_secondary()),
                );
            }

            let mut delete_cat_idx: Option<usize> = None;
            for (idx, cat) in state.categories.iter().enumerate() {
                let cat_name = cat.name.as_deref().unwrap_or("Sans nom");
                let cat_id = cat.id.as_deref().unwrap_or_default();

                // Compter les produits dans cette catégorie.
                let nb_products = state
                    .produits
                    .iter()
                    .filter(|p| p.category_id.as_deref() == Some(cat_id))
                    .count();

                egui::Frame::new()
                    .fill(theme.card_background())
                    .corner_radius(theme.border_radius())
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(cat_name)
                                    .size(theme.font_size_body())
                                    .color(theme.text_primary()),
                            );
                            ui.label(
                                egui::RichText::new(format!("({} produits)", nb_products))
                                    .color(theme.text_secondary()),
                            );
                            if ui.button("Supprimer").clicked() {
                                delete_cat_idx = Some(idx);
                            }
                        });
                    });
                ui.add_space(theme.item_spacing());
            }

            // Gestion suppression.
            if let Some(idx) = delete_cat_idx {
                if let Some(ref cid) = state.categories[idx].id {
                    match db.categorie_delete(cid) {
                        Ok(()) => {
                            state.categories.remove(idx);
                            state.status_message =
                                Some("Catégorie supprimée.".to_string());
                        }
                        Err(e) => {
                            state.status_message =
                                Some(format!("Erreur suppression catégorie : {}", e));
                        }
                    }
                }
            }

            // --- Message de statut ---
            if let Some(msg) = &state.status_message {
                ui.add_space(theme.item_spacing());
                ui.label(egui::RichText::new(msg).color(theme.accent()));
            }
        });
    });
}
