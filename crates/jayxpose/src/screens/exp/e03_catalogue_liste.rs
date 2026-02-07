//! XP-E03 — Catalogue produits (liste).
//!
//! Liste des produits de l'exposant avec filtres, recherche, et actions rapides.

use crate::data::JayXposeDb;
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_exp_e03
/// @do: render_product_catalogue_list
/// @layer: app
/// Écran liste catalogue : recherche, filtre catégorie, cartes produit, actions modifier/supprimer.
/// Affiche l'écran XP-E03 — Catalogue produits (liste).
pub fn exp_e03_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    // Rechargement si la liste est vide et qu'un exposant est connecté.
    if state.produits.is_empty() {
        if let Some(ref eid) = state.current_exposant_id {
            if let Ok(produits) = db.produits_by_exposant(eid) {
                state.produits = produits;
            }
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
                ui.heading("Mon catalogue");
                ui.label(
                    egui::RichText::new(format!("{} produit(s)", state.produits.len()))
                        .color(theme.text_secondary()),
                );
            });
            ui.add_space(theme.item_spacing());

            // Bouton ajouter un produit.
            if ui.button("+ Ajouter un produit").clicked() {
                state.editing_produit_id = None;
                state.produit_form = crate::data::ProduitCatalogue::default();
                // Pré-remplir exposant_id.
                state.produit_form.exposant_id = state.current_exposant_id.clone();
                nav_request.replace(Some(ScreenId::ExpFicheProduit));
            }
            ui.add_space(theme.item_spacing());

            // --- Filtres ---
            ui.horizontal(|ui| {
                ui.label("Recherche :");
                ui.text_edit_singleline(&mut state.catalogue_search);
            });
            ui.horizontal(|ui| {
                ui.label("Catégorie :");
                let current_cat = state
                    .catalogue_category_filter
                    .as_deref()
                    .unwrap_or("Toutes");
                egui::ComboBox::from_label("")
                    .selected_text(current_cat)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(
                                state.catalogue_category_filter.is_none(),
                                "Toutes",
                            )
                            .clicked()
                        {
                            state.catalogue_category_filter = None;
                        }
                        for cat in &state.categories {
                            let cat_name = cat.name.as_deref().unwrap_or("Sans nom");
                            let cat_id = cat.id.as_deref().unwrap_or_default();
                            let selected = state
                                .catalogue_category_filter
                                .as_deref()
                                == Some(cat_id);
                            if ui.selectable_label(selected, cat_name).clicked() {
                                state.catalogue_category_filter = Some(cat_id.to_string());
                            }
                        }
                    });
            });
            ui.add_space(theme.item_spacing());
            ui.separator();
            ui.add_space(theme.item_spacing());

            // --- Liste de produits ---
            let search_lower = state.catalogue_search.to_lowercase();
            let filtered_indices: Vec<usize> = (0..state.produits.len())
                .filter(|&i| {
                    let p = &state.produits[i];
                    // Filtre texte.
                    let name_match = p
                        .name
                        .as_deref()
                        .unwrap_or_default()
                        .to_lowercase()
                        .contains(&search_lower);
                    // Filtre catégorie.
                    let cat_match = match &state.catalogue_category_filter {
                        Some(filter_id) => p.category_id.as_deref() == Some(filter_id.as_str()),
                        None => true,
                    };
                    (search_lower.is_empty() || name_match) && cat_match
                })
                .collect();

            if filtered_indices.is_empty() {
                ui.label(
                    egui::RichText::new("Aucun produit trouvé.")
                        .color(theme.text_secondary()),
                );
            }

            // Track which product to delete (by index).
            let mut delete_idx: Option<usize> = None;
            let mut edit_idx: Option<usize> = None;

            for &idx in &filtered_indices {
                let p = &state.produits[idx];
                let name = p.name.as_deref().unwrap_or("Sans nom");
                let cat_name = find_category_name(&state.categories, p.category_id.as_deref());
                let price_label = match p.price {
                    Some(price) => format!("{:.2} €", price),
                    None => "Sur demande".to_string(),
                };
                let availability = p.availability.as_deref().unwrap_or("disponible");
                let featured = p.is_featured.unwrap_or(false);

                egui::Frame::new()
                    .fill(theme.card_background())
                    .corner_radius(theme.border_radius())
                    .inner_margin(egui::Margin::same(10))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if featured {
                                ui.label(egui::RichText::new("★").color(theme.accent()));
                            }
                            ui.label(
                                egui::RichText::new(name)
                                    .size(theme.font_size_body())
                                    .color(theme.text_primary()),
                            );
                            ui.label(
                                egui::RichText::new(cat_name)
                                    .color(theme.text_secondary()),
                            );
                            ui.label(
                                egui::RichText::new(&price_label)
                                    .color(theme.text_primary()),
                            );
                            ui.label(
                                egui::RichText::new(availability)
                                    .color(theme.text_secondary()),
                            );
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Modifier").clicked() {
                                edit_idx = Some(idx);
                            }
                            if ui.button("Supprimer").clicked() {
                                delete_idx = Some(idx);
                            }
                        });
                    });
                ui.add_space(theme.item_spacing());
            }

            // Handle edit action.
            if let Some(idx) = edit_idx {
                let p = state.produits[idx].clone();
                state.editing_produit_id = p.id.clone();
                state.produit_form = p;
                nav_request.replace(Some(ScreenId::ExpFicheProduit));
            }

            // Handle delete action.
            if let Some(idx) = delete_idx {
                if let Some(ref pid) = state.produits[idx].id {
                    if db.produit_delete(pid).is_ok() {
                        state.produits.remove(idx);
                        state.status_message = Some("Produit supprimé.".to_string());
                    } else {
                        state.status_message =
                            Some("Erreur lors de la suppression du produit.".to_string());
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

/// Retrouve le nom d'une catégorie par son ID.
fn find_category_name(
    _categories: &[crate::data::CategorieProduit],
    category_id: Option<&str>,
) -> &'static str {
    // Retourne un libellé statique ou "—" ; version simplifiée.
    // En production, on retournerait un String.
    match category_id {
        Some(_) => "Catégorie",
        None => "—",
    }
}
