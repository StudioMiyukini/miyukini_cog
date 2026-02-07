//! XP-E01 — Tableau de bord exposant (accueil).
//!
//! Vue d'ensemble : complétion profil, accès rapides catalogue, vitrine, documents, fiche publique.

use crate::data::JayXposeDb;
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_exp_e01
/// @do: render_exposant_dashboard_overview
/// @layer: app
/// Affiche le tableau de bord exposant : jauge complétion, cartes d'accès rapide.
/// Affiche l'écran XP-E01 — Tableau de bord exposant.
pub fn exp_e01_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    // Chargement initial des données si nécessaire.
    if !state.data_loaded {
        load_dashboard_data(state, db);
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // --- En-tête ---
            ui.heading("Tableau de bord exposant");
            ui.add_space(theme.item_spacing());

            let company_name = state
                .fiche_form
                .company_name
                .as_deref()
                .unwrap_or("Mon entreprise");
            let vitrine_status = state
                .fiche_form
                .vitrine_status
                .as_deref()
                .unwrap_or("brouillon");

            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(company_name)
                        .size(theme.font_size_heading())
                        .color(theme.text_primary()),
                );
                ui.label(
                    egui::RichText::new(format!("Vitrine : {}", vitrine_status))
                        .size(theme.font_size_body())
                        .color(theme.text_secondary()),
                );
            });
            ui.add_space(theme.section_spacing());

            // --- Jauge de complétion profil ---
            let completion = compute_profile_completion(&state.fiche_form);
            ui.label(
                egui::RichText::new(format!("Complétion du profil : {}%", completion))
                    .size(theme.font_size_body())
                    .color(theme.text_primary()),
            );
            let progress = completion as f32 / 100.0;
            ui.add(
                egui::ProgressBar::new(progress)
                    .text(format!("{}%", completion))
                    .desired_width(ui.available_width()),
            );
            ui.add_space(theme.section_spacing());

            // --- Cartes d'accès rapide ---
            ui.separator();
            ui.add_space(theme.item_spacing());
            ui.label(
                egui::RichText::new("Accès rapide")
                    .size(theme.font_size_heading())
                    .color(theme.text_primary()),
            );
            ui.add_space(theme.item_spacing());

            // Fiche entreprise
            egui::Frame::new()
                .fill(theme.card_background())
                .corner_radius(theme.border_radius())
                .inner_margin(egui::Margin::same(12))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Fiche entreprise")
                            .size(theme.font_size_body())
                            .color(theme.text_primary()),
                    );
                    ui.label(
                        egui::RichText::new("Gérer les informations de votre entreprise.")
                            .color(theme.text_secondary()),
                    );
                    if ui.button("Modifier ma fiche").clicked() {
                        nav_request.replace(Some(ScreenId::ExpFicheEntreprise));
                    }
                });
            ui.add_space(theme.item_spacing());

            // Mon catalogue
            let nb_produits = state.produits.len();
            egui::Frame::new()
                .fill(theme.card_background())
                .corner_radius(theme.border_radius())
                .inner_margin(egui::Margin::same(12))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(format!("Mon catalogue ({} produits)", nb_produits))
                            .size(theme.font_size_body())
                            .color(theme.text_primary()),
                    );
                    ui.label(
                        egui::RichText::new("Gérer vos produits et catégories.")
                            .color(theme.text_secondary()),
                    );
                    ui.horizontal(|ui| {
                        if ui.button("Voir le catalogue").clicked() {
                            nav_request.replace(Some(ScreenId::ExpCatalogueListe));
                        }
                        if ui.button("Catégories").clicked() {
                            nav_request.replace(Some(ScreenId::ExpCategories));
                        }
                    });
                });
            ui.add_space(theme.item_spacing());

            // Ma vitrine
            egui::Frame::new()
                .fill(theme.card_background())
                .corner_radius(theme.border_radius())
                .inner_margin(egui::Margin::same(12))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Ma vitrine")
                            .size(theme.font_size_body())
                            .color(theme.text_primary()),
                    );
                    ui.label(
                        egui::RichText::new("Personnaliser et publier votre vitrine en ligne.")
                            .color(theme.text_secondary()),
                    );
                    ui.horizontal(|ui| {
                        if ui.button("Paramètres").clicked() {
                            nav_request.replace(Some(ScreenId::ExpVitrineParams));
                        }
                        if ui.button("Présentation").clicked() {
                            nav_request.replace(Some(ScreenId::ExpVitrinePresentation));
                        }
                        if ui.button("Prévisualiser").clicked() {
                            nav_request.replace(Some(ScreenId::ExpVitrinePreview));
                        }
                    });
                });
            ui.add_space(theme.item_spacing());

            // Mes documents
            let nb_documents = state.documents.len();
            egui::Frame::new()
                .fill(theme.card_background())
                .corner_radius(theme.border_radius())
                .inner_margin(egui::Margin::same(12))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(format!("Mes documents ({})", nb_documents))
                            .size(theme.font_size_body())
                            .color(theme.text_primary()),
                    );
                    ui.label(
                        egui::RichText::new(
                            "Coffre-fort documentaire : RIB, assurance, Kbis, etc.",
                        )
                        .color(theme.text_secondary()),
                    );
                    ui.horizontal(|ui| {
                        if ui.button("Voir les documents").clicked() {
                            nav_request.replace(Some(ScreenId::ExpDocuments));
                        }
                        if ui.button("Partages").clicked() {
                            nav_request.replace(Some(ScreenId::ExpPartageDocuments));
                        }
                    });
                });
            ui.add_space(theme.item_spacing());

            // Ma fiche publique
            egui::Frame::new()
                .fill(theme.card_background())
                .corner_radius(theme.border_radius())
                .inner_margin(egui::Margin::same(12))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Ma fiche publique")
                            .size(theme.font_size_body())
                            .color(theme.text_primary()),
                    );
                    ui.label(
                        egui::RichText::new(
                            "Configurer la visibilité de vos informations dans l'annuaire.",
                        )
                        .color(theme.text_secondary()),
                    );
                    if ui.button("Gérer la confidentialité").clicked() {
                        nav_request.replace(Some(ScreenId::ExpFichePublique));
                    }
                });

            // --- Message de statut ---
            ui.add_space(theme.section_spacing());
            if let Some(msg) = &state.status_message {
                ui.label(
                    egui::RichText::new(msg)
                        .color(theme.accent()),
                );
            }
        });
    });
}

/// Charge les données du dashboard (profil, produits, documents, etc.).
fn load_dashboard_data(state: &mut ExpState, db: &Arc<JayXposeDb>) {
    let exposant_id = match &state.current_exposant_id {
        Some(id) => id.clone(),
        None => return,
    };

    // Charger le profil exposant.
    if let Ok(Some(profile)) = db.exposant_by_id(&exposant_id) {
        state.fiche_form = profile;
    }

    // Charger les produits.
    if let Ok(produits) = db.produits_by_exposant(&exposant_id) {
        state.produits = produits;
    }

    // Charger les catégories.
    if let Ok(categories) = db.categories_by_exposant(&exposant_id) {
        state.categories = categories;
    }

    // Charger les documents.
    if let Ok(documents) = db.documents_by_exposant(&exposant_id) {
        state.documents = documents;
    }

    // Charger les partages.
    if let Ok(partages) = db.partages_by_exposant(&exposant_id) {
        state.partages = partages;
    }

    // Charger les pages vitrine.
    if let Ok(pages) = db.vitrine_pages_by_exposant(&exposant_id) {
        state.vitrine_pages = pages;
    }

    // Charger la confidentialité.
    if let Ok(rules) = db.confidentialite_by_exposant(&exposant_id) {
        state.confidentialite = rules;
    }

    state.data_loaded = true;
}

/// Calcule le pourcentage de complétion du profil exposant (0–100).
fn compute_profile_completion(profile: &crate::data::ExposantProfile) -> u8 {
    let fields: Vec<bool> = vec![
        profile.company_name.is_some(),
        profile.legal_form.is_some(),
        profile.slogan.is_some(),
        profile.description_short.is_some(),
        profile.description_long.is_some(),
        profile.contact_email.is_some(),
        profile.contact_phone.is_some(),
        profile.adresse_siege.is_some(),
        profile.siret.is_some(),
        profile.logo_url.is_some(),
        profile.banner_url.is_some(),
        profile.site_web.is_some(),
        profile.secteur.is_some(),
        profile.social_linkedin.is_some(),
    ];
    let filled = fields.iter().filter(|&&v| v).count();
    let total = fields.len();
    if total == 0 {
        return 0;
    }
    ((filled as f64 / total as f64) * 100.0).round() as u8
}
