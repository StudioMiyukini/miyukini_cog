//! XP-E04 — Fiche produit (création / modification).
//!
//! Formulaire complet d'édition d'un produit du catalogue exposant.

use crate::data::{Availability, JayXposeDb};
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_exp_e04
/// @do: render_product_edit_form
/// @layer: app
/// Écran fiche produit : formulaire nom, description, prix, catégorie, disponibilité, vedette ; boutons enregistrer/annuler/supprimer.
/// Affiche l'écran XP-E04 — Fiche produit.
pub fn exp_e04_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    let is_edit = state.editing_produit_id.is_some();
    let title = if is_edit {
        "Modifier le produit"
    } else {
        "Nouveau produit"
    };

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // --- En-tête ---
            ui.horizontal(|ui| {
                if ui.button("← Catalogue").clicked() {
                    nav_request.replace(Some(ScreenId::ExpCatalogueListe));
                }
                ui.heading(title);
            });
            ui.add_space(theme.section_spacing());

            let form = &mut state.produit_form;

            // Nom.
            ui.label("Nom du produit");
            let mut name = form.name.clone().unwrap_or_default();
            if ui.text_edit_singleline(&mut name).changed() {
                form.name = Some(name);
            }
            ui.add_space(theme.item_spacing());

            // Description.
            ui.label("Description");
            let mut desc = form.description.clone().unwrap_or_default();
            if ui.text_edit_multiline(&mut desc).changed() {
                form.description = Some(desc);
            }
            ui.add_space(theme.item_spacing());

            // Prix.
            ui.label("Prix (laisser vide = sur demande)");
            let mut price_str = match form.price {
                Some(p) => format!("{:.2}", p),
                None => String::new(),
            };
            if ui.text_edit_singleline(&mut price_str).changed() {
                form.price = price_str.trim().parse::<f64>().ok();
            }
            ui.add_space(theme.item_spacing());

            // Catégorie (ComboBox).
            ui.label("Catégorie");
            let current_cat_name = find_category_name_owned(&state.categories, form.category_id.as_deref());
            egui::ComboBox::from_label("Catégorie produit")
                .selected_text(&current_cat_name)
                .show_ui(ui, |ui| {
                    if ui.selectable_label(form.category_id.is_none(), "Aucune").clicked() {
                        form.category_id = None;
                    }
                    for cat in &state.categories {
                        let cname = cat.name.as_deref().unwrap_or("Sans nom");
                        let cid = cat.id.as_deref().unwrap_or_default();
                        let selected = form.category_id.as_deref() == Some(cid);
                        if ui.selectable_label(selected, cname).clicked() {
                            form.category_id = Some(cid.to_string());
                        }
                    }
                });
            ui.add_space(theme.item_spacing());

            // Disponibilité (ComboBox).
            ui.label("Disponibilité");
            let avail = Availability::from_str(
                form.availability.as_deref().unwrap_or("disponible"),
            );
            let avail_label = avail.as_str();
            egui::ComboBox::from_label("Disponibilité produit")
                .selected_text(avail_label)
                .show_ui(ui, |ui| {
                    for option in &[
                        Availability::Disponible,
                        Availability::Rupture,
                        Availability::SurCommande,
                    ] {
                        let selected = avail == *option;
                        if ui.selectable_label(selected, option.as_str()).clicked() {
                            form.availability = Some(option.as_str().to_string());
                        }
                    }
                });
            ui.add_space(theme.item_spacing());

            // Produit vedette.
            let mut featured = form.is_featured.unwrap_or(false);
            if ui.checkbox(&mut featured, "Produit vedette (mis en avant)").changed() {
                form.is_featured = Some(featured);
            }
            ui.add_space(theme.section_spacing());

            // --- Boutons ---
            ui.horizontal(|ui| {
                if ui.button("Enregistrer").clicked() {
                    save_product(state, db, is_edit);
                }
                if ui.button("Annuler").clicked() {
                    nav_request.replace(Some(ScreenId::ExpCatalogueListe));
                }
                if is_edit && ui.button("Supprimer").clicked() {
                    if let Some(ref pid) = state.editing_produit_id {
                        if db.produit_delete(pid).is_ok() {
                            state.produits.retain(|p| p.id.as_deref() != Some(pid.as_str()));
                            state.status_message = Some("Produit supprimé.".to_string());
                            nav_request.replace(Some(ScreenId::ExpCatalogueListe));
                        } else {
                            state.status_message =
                                Some("Erreur lors de la suppression.".to_string());
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

/// Enregistre le produit (insertion ou mise à jour).
fn save_product(state: &mut ExpState, db: &Arc<JayXposeDb>, is_edit: bool) {
    let result = if is_edit {
        db.produit_update(&state.produit_form)
    } else {
        db.produit_insert(&state.produit_form)
    };
    match result {
        Ok(()) => {
            state.status_message = Some("Produit enregistré avec succès.".to_string());
            // Recharger les produits.
            if let Some(ref eid) = state.current_exposant_id {
                if let Ok(produits) = db.produits_by_exposant(eid) {
                    state.produits = produits;
                }
            }
        }
        Err(e) => {
            state.status_message = Some(format!("Erreur : {}", e));
        }
    }
}

/// Retrouve le nom d'une catégorie par ID.
fn find_category_name_owned(
    categories: &[crate::data::CategorieProduit],
    category_id: Option<&str>,
) -> String {
    match category_id {
        Some(cid) => categories
            .iter()
            .find(|c| c.id.as_deref() == Some(cid))
            .and_then(|c| c.name.clone())
            .unwrap_or_else(|| "Inconnue".to_string()),
        None => "Aucune".to_string(),
    }
}
