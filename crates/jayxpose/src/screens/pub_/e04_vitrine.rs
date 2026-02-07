//! XP-P04 — Site vitrine publique.
//!
//! Rendu complet de la vitrine d'un exposant : barre de navigation,
//! onglets (Accueil, Catalogue, Présentation, Contact), contenu dynamique.

use crate::data::JayXposeDb;
use crate::screens::pub_::PubState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use crate::ui;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// Labels des onglets vitrine.
const VITRINE_TABS: &[&str] = &["Accueil", "Catalogue", "Présentation", "Contact"];

/// @id: screen_pub_e04
/// @do: render_vitrine_publique_exposant
/// @layer: app
///
/// Affiche le site vitrine publique d'un exposant avec navigation par onglets.
pub fn pub_e04_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut PubState,
    db: &Arc<JayXposeDb>,
) {
    // Chargement initial.
    if !state.vitrine_loaded {
        if let Some(ref id) = state.selected_exposant_id {
            if let Ok(pages) = db.vitrine_pages_by_exposant(id) {
                state.vitrine_pages = pages;
            }
            if state.fiche_exposant.is_none() {
                if let Ok(Some(profile)) = db.exposant_by_id(id) {
                    state.fiche_exposant = Some(profile);
                }
            }
            if state.catalogue_produits.is_empty() {
                if let Ok(produits) = db.produits_by_exposant(id) {
                    state.catalogue_produits = produits;
                }
            }
        }
        state.vitrine_loaded = true;
    }

    let company_name = state
        .fiche_exposant
        .as_ref()
        .and_then(|e| e.company_name.clone())
        .unwrap_or_else(|| "Exposant".to_string());

    egui::CentralPanel::default().show(ctx, |ui| {
        // ── Barre supérieure ──
        egui::Frame::new()
            .fill(theme.section_background())
            .inner_margin(egui::Margin::symmetric(16, 8))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(&company_name)
                            .size(theme.font_size_heading())
                            .color(theme.text_primary())
                            .strong(),
                    );
                    ui.add_space(theme.section_spacing());

                    for (idx, tab_label) in VITRINE_TABS.iter().enumerate() {
                        let is_active = state.vitrine_tab == idx;
                        let text = if is_active {
                            egui::RichText::new(*tab_label)
                                .size(theme.font_size_body())
                                .color(theme.accent())
                                .strong()
                        } else {
                            egui::RichText::new(*tab_label)
                                .size(theme.font_size_body())
                                .color(theme.text_secondary())
                        };
                        if ui.add(egui::Label::new(text).sense(egui::Sense::click())).clicked() {
                            state.vitrine_tab = idx;
                        }
                        ui.add_space(theme.item_spacing());
                    }

                    // Bouton retour fiche.
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui::secondary_button(ui, theme, "Retour fiche").clicked() {
                            let _ = nav_request.replace(Some(ScreenId::PubFicheExposant));
                        }
                    });
                });
            });

        ui.add_space(theme.item_spacing());

        // ── Contenu de l'onglet ──
        egui::ScrollArea::vertical().show(ui, |ui| {
            match state.vitrine_tab {
                0 => render_tab_accueil(ui, theme, state),
                1 => render_tab_catalogue(ui, theme, state),
                2 => render_tab_presentation(ui, theme, state),
                3 => render_tab_contact(ui, theme, nav_request, state),
                _ => {}
            }
        });
    });
}

/// Onglet Accueil : description + produits vedettes.
fn render_tab_accueil(ui: &mut egui::Ui, theme: &JayXposeTheme, state: &PubState) {
    ui::section(ui, theme, "Bienvenue", |ui| {
        // Contenu de la page accueil si disponible.
        if let Some(page) = state
            .vitrine_pages
            .iter()
            .find(|p| p.page_type.as_deref() == Some("accueil"))
        {
            if let Some(ref content) = page.content {
                ui::body(ui, theme, content);
            }
        } else if let Some(ref exposant) = state.fiche_exposant {
            if let Some(ref desc) = exposant.description_long {
                ui::body(ui, theme, desc);
            } else if let Some(ref desc) = exposant.description_short {
                ui::body(ui, theme, desc);
            }
        }
    });

    // Produits vedettes.
    let featured: Vec<_> = state
        .catalogue_produits
        .iter()
        .filter(|p| p.is_featured.unwrap_or(false))
        .collect();

    if !featured.is_empty() {
        ui::section(ui, theme, "Produits vedettes", |ui| {
            egui::Grid::new("vitrine_featured_grid")
                .num_columns(3)
                .spacing([theme.item_spacing(), theme.item_spacing()])
                .show(ui, |ui| {
                    for (i, produit) in featured.iter().enumerate() {
                        ui::card(ui, theme, |ui| {
                            let name = produit.name.as_deref().unwrap_or("Produit");
                            ui.label(
                                egui::RichText::new(name)
                                    .size(theme.font_size_body())
                                    .color(theme.text_primary())
                                    .strong(),
                            );
                            match produit.price {
                                Some(price) => {
                                    let currency =
                                        produit.currency.as_deref().unwrap_or("EUR");
                                    ui::body(ui, theme, &format!("{price:.2} {currency}"));
                                }
                                None => {
                                    ui::caption(ui, theme, "Sur demande");
                                }
                            }
                        });
                        if (i + 1) % 3 == 0 {
                            ui.end_row();
                        }
                    }
                });
        });
    }
}

/// Onglet Catalogue : grille de tous les produits.
fn render_tab_catalogue(ui: &mut egui::Ui, theme: &JayXposeTheme, state: &PubState) {
    ui::section(ui, theme, "Catalogue complet", |ui| {
        if state.catalogue_produits.is_empty() {
            ui::caption(ui, theme, "Aucun produit dans le catalogue.");
        } else {
            egui::Grid::new("vitrine_catalogue_grid")
                .num_columns(3)
                .spacing([theme.item_spacing(), theme.item_spacing()])
                .show(ui, |ui| {
                    for (i, produit) in state.catalogue_produits.iter().enumerate() {
                        ui::card(ui, theme, |ui| {
                            ui::caption(ui, theme, "[image]");

                            let name = produit.name.as_deref().unwrap_or("Produit");
                            ui.label(
                                egui::RichText::new(name)
                                    .size(theme.font_size_body())
                                    .color(theme.text_primary())
                                    .strong(),
                            );

                            match produit.price {
                                Some(price) => {
                                    let currency =
                                        produit.currency.as_deref().unwrap_or("EUR");
                                    ui::body(ui, theme, &format!("{price:.2} {currency}"));
                                }
                                None => {
                                    ui::caption(ui, theme, "Sur demande");
                                }
                            }

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
    });
}

/// Onglet Présentation : contenu texte de la page « presentation ».
fn render_tab_presentation(ui: &mut egui::Ui, theme: &JayXposeTheme, state: &PubState) {
    ui::section(ui, theme, "Présentation", |ui| {
        if let Some(page) = state
            .vitrine_pages
            .iter()
            .find(|p| p.page_type.as_deref() == Some("presentation"))
        {
            if let Some(ref content) = page.content {
                ui::body(ui, theme, content);
            } else {
                ui::caption(ui, theme, "Contenu en cours de rédaction.");
            }
        } else {
            ui::caption(ui, theme, "Page de présentation non disponible.");
        }
    });
}

/// Onglet Contact : redirige vers la page contact dédiée.
fn render_tab_contact(
    ui: &mut egui::Ui,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &PubState,
) {
    ui::section(ui, theme, "Contact", |ui| {
        if let Some(ref exposant) = state.fiche_exposant {
            if let Some(ref email) = exposant.contact_email {
                ui.horizontal(|ui| {
                    ui::caption(ui, theme, "Email :");
                    ui::body(ui, theme, email);
                });
            }
            if let Some(ref phone) = exposant.contact_phone {
                ui.horizontal(|ui| {
                    ui::caption(ui, theme, "Téléphone :");
                    ui::body(ui, theme, phone);
                });
            }
        }

        ui.add_space(theme.item_spacing());

        if ui::accent_button(ui, theme, "Page contact complète").clicked() {
            let _ = nav_request.replace(Some(ScreenId::PubVitrineContact));
        }
    });
}
