//! XP-P03 — Catalogue produits public.
//!
//! Vue publique du catalogue d'un exposant : grille de produits filtrable
//! par recherche texte et catégorie, produits vedettes en priorité.

use crate::data::JayXposeDb;
use crate::screens::pub_::PubState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use crate::ui;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_pub_e03
/// @do: render_catalogue_produits_public
/// @layer: app
///
/// Affiche le catalogue public des produits d'un exposant.
pub fn pub_e03_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut PubState,
    db: &Arc<JayXposeDb>,
) {
    // Chargement initial.
    if !state.catalogue_loaded {
        if let Some(ref id) = state.selected_exposant_id {
            if let Ok(produits) = db.produits_by_exposant(id) {
                state.catalogue_produits = produits;
            }
            if let Ok(cats) = db.categories_by_exposant(id) {
                state.catalogue_categories = cats;
            }
            // S'assurer que la fiche est chargée pour le nom.
            if state.fiche_exposant.is_none() {
                if let Ok(Some(profile)) = db.exposant_by_id(id) {
                    state.fiche_exposant = Some(profile);
                }
            }
        }
        state.catalogue_loaded = true;
    }

    let company_name = state
        .fiche_exposant
        .as_ref()
        .and_then(|e| e.company_name.clone())
        .unwrap_or_else(|| "Exposant".to_string());

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // ── En-tête ──
            ui::heading(ui, theme, &format!("Catalogue de {company_name}"));
            ui.add_space(theme.item_spacing());

            let total = state.catalogue_produits.len();
            ui::caption(
                ui,
                theme,
                &format!("{total} produit{}", if total > 1 { "s" } else { "" }),
            );
            ui.add_space(theme.item_spacing());

            // ── Filtres ──
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Rechercher :")
                        .size(theme.font_size_body())
                        .color(theme.text_secondary()),
                );
                ui.text_edit_singleline(&mut state.search_produit);

                ui.add_space(theme.item_spacing());

                // Filtre catégorie.
                let current_cat = state
                    .filter_categorie
                    .as_deref()
                    .unwrap_or("Toutes les catégories");
                egui::ComboBox::from_id_salt("filter_cat")
                    .selected_text(current_cat)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(
                                state.filter_categorie.is_none(),
                                "Toutes les catégories",
                            )
                            .clicked()
                        {
                            state.filter_categorie = None;
                        }
                        for cat in &state.catalogue_categories {
                            let cat_name =
                                cat.name.as_deref().unwrap_or("Sans nom");
                            let selected = state.filter_categorie.as_deref()
                                == cat.id.as_deref();
                            if ui.selectable_label(selected, cat_name).clicked() {
                                state.filter_categorie = cat.id.clone();
                            }
                        }
                    });
            });

            ui.add_space(theme.section_spacing());
            ui.separator();
            ui.add_space(theme.item_spacing());

            // ── Filtrage en mémoire ──
            let query_lower = state.search_produit.to_lowercase();
            let mut filtered: Vec<_> = state
                .catalogue_produits
                .iter()
                .filter(|p| {
                    let matches_text = query_lower.is_empty()
                        || p.name
                            .as_deref()
                            .unwrap_or_default()
                            .to_lowercase()
                            .contains(&query_lower)
                        || p.description
                            .as_deref()
                            .unwrap_or_default()
                            .to_lowercase()
                            .contains(&query_lower);

                    let matches_cat = match &state.filter_categorie {
                        Some(cat_id) => p.category_id.as_deref() == Some(cat_id.as_str()),
                        None => true,
                    };

                    matches_text && matches_cat
                })
                .collect();

            // Produits vedettes en premier.
            filtered.sort_by(|a, b| {
                let a_feat = a.is_featured.unwrap_or(false);
                let b_feat = b.is_featured.unwrap_or(false);
                b_feat.cmp(&a_feat).then_with(|| {
                    a.sort_order
                        .unwrap_or(i32::MAX)
                        .cmp(&b.sort_order.unwrap_or(i32::MAX))
                })
            });

            // ── Grille de produits (3 colonnes) ──
            if filtered.is_empty() {
                ui.add_space(theme.section_spacing());
                ui::caption(ui, theme, "Aucun produit trouvé.");
            } else {
                egui::Grid::new("catalogue_grid")
                    .num_columns(3)
                    .spacing([theme.item_spacing(), theme.item_spacing()])
                    .show(ui, |ui| {
                        for (i, produit) in filtered.iter().enumerate() {
                            ui::card(ui, theme, |ui| {
                                // Image placeholder.
                                ui::caption(ui, theme, "[image]");

                                // Badge vedette.
                                if produit.is_featured.unwrap_or(false) {
                                    ui::badge(
                                        ui,
                                        "Vedette",
                                        egui::Color32::from_rgb(234, 179, 8),
                                    );
                                }

                                // Nom du produit.
                                let name =
                                    produit.name.as_deref().unwrap_or("Produit");
                                ui.label(
                                    egui::RichText::new(name)
                                        .size(theme.font_size_body())
                                        .color(theme.text_primary())
                                        .strong(),
                                );

                                // Prix.
                                match produit.price {
                                    Some(price) => {
                                        let currency = produit
                                            .currency
                                            .as_deref()
                                            .unwrap_or("EUR");
                                        ui::body(
                                            ui,
                                            theme,
                                            &format!("{price:.2} {currency}"),
                                        );
                                    }
                                    None => {
                                        ui::caption(ui, theme, "Sur demande");
                                    }
                                }

                                // Badge disponibilité.
                                if let Some(ref avail) = produit.availability {
                                    let color = ui::availability_color(avail);
                                    let label = match avail.as_str() {
                                        "disponible" => "Disponible",
                                        "rupture" => "Rupture",
                                        "sur_commande" => "Sur commande",
                                        other => other,
                                    };
                                    ui::badge(ui, label, color);
                                }
                            });

                            if (i + 1) % 3 == 0 {
                                ui.end_row();
                            }
                        }
                    });
            }

            ui.add_space(theme.section_spacing());
            ui.separator();
            ui.add_space(theme.item_spacing());

            // ── Navigation ──
            if ui::secondary_button(ui, theme, "Retour à la fiche exposant").clicked() {
                let _ = nav_request.replace(Some(ScreenId::PubFicheExposant));
            }
        });
    });
}
