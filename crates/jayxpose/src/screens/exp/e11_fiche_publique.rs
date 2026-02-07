//! XP-E11 — Fiche publique / Confidentialité.
//!
//! Aperçu de la fiche publique et matrice de confidentialité (public/privé/service_only) par champ.

use crate::data::{ConfidentialiteProfil, JayXposeDb, Visibility};
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_exp_e11
/// @do: render_public_profile_and_privacy_matrix
/// @layer: app
/// Écran fiche publique : aperçu lecture seule + matrice confidentialité + toggle annuaire.
/// Champs du profil exposés dans la matrice de confidentialité.
const PRIVACY_FIELDS: &[(&str, &str)] = &[
    ("company_name", "Raison sociale"),
    ("slogan", "Slogan"),
    ("description_short", "Description courte"),
    ("description_long", "Description longue"),
    ("contact_email", "Email de contact"),
    ("contact_phone", "Téléphone"),
    ("adresse_siege", "Adresse du siège"),
    ("site_web", "Site web"),
    ("social_facebook", "Facebook"),
    ("social_instagram", "Instagram"),
    ("social_linkedin", "LinkedIn"),
    ("social_tiktok", "TikTok"),
    ("social_youtube", "YouTube"),
    ("social_pinterest", "Pinterest"),
    ("social_x", "X (ex-Twitter)"),
    ("secteur", "Secteur d'activité"),
    ("tags", "Tags"),
];

/// Affiche l'écran XP-E11 — Fiche publique / Confidentialité.
pub fn exp_e11_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    // Charger la confidentialité si nécessaire.
    if state.confidentialite.is_empty() {
        if let Some(ref eid) = state.current_exposant_id {
            if let Ok(rules) = db.confidentialite_by_exposant(eid) {
                state.confidentialite = rules;
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
                ui.heading("Fiche publique & Confidentialité");
            });
            ui.add_space(theme.section_spacing());

            // --- Aperçu fiche publique ---
            ui.label(
                egui::RichText::new("Aperçu de votre fiche publique")
                    .size(theme.font_size_heading())
                    .color(theme.text_primary()),
            );
            ui.add_space(theme.item_spacing());

            egui::Frame::new()
                .fill(theme.section_background())
                .corner_radius(theme.border_radius())
                .inner_margin(egui::Margin::same(12))
                .show(ui, |ui| {
                    let form = &state.fiche_form;
                    for &(field_name, label) in PRIVACY_FIELDS {
                        let value = get_field_value(form, field_name);
                        let visibility = get_field_visibility(&state.confidentialite, field_name);
                        if visibility == Visibility::Public {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("{} :", label))
                                        .color(theme.text_secondary()),
                                );
                                ui.label(
                                    egui::RichText::new(&value)
                                        .color(theme.text_primary()),
                                );
                            });
                        }
                    }
                });
            ui.add_space(theme.section_spacing());

            // --- Matrice de confidentialité ---
            ui.separator();
            ui.add_space(theme.item_spacing());
            ui.label(
                egui::RichText::new("Matrice de confidentialité")
                    .size(theme.font_size_heading())
                    .color(theme.text_primary()),
            );
            ui.add_space(theme.item_spacing());

            for &(field_name, label) in PRIVACY_FIELDS {
                let current_vis = get_field_visibility(&state.confidentialite, field_name);
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(label)
                            .color(theme.text_primary()),
                    );
                    ui.add_space(theme.item_spacing());

                    for vis_option in &[Visibility::Public, Visibility::Prive, Visibility::ServiceOnly] {
                        let selected = current_vis == *vis_option;
                        if ui
                            .selectable_label(selected, vis_option.as_str())
                            .clicked()
                        {
                            set_field_visibility(
                                &mut state.confidentialite,
                                &state.current_exposant_id,
                                field_name,
                                *vis_option,
                            );
                        }
                    }
                });
            }

            ui.add_space(theme.item_spacing());
            ui.separator();
            ui.add_space(theme.item_spacing());

            // --- Toggle visible annuaire ---
            let mut visible_annuaire = state.fiche_form.visible_annuaire.unwrap_or(true);
            if ui
                .checkbox(&mut visible_annuaire, "Visible dans l'annuaire public")
                .changed()
            {
                state.fiche_form.visible_annuaire = Some(visible_annuaire);
            }

            ui.add_space(theme.section_spacing());

            // --- Bouton Enregistrer ---
            if ui.button("Enregistrer").clicked() {
                // Sauvegarder les règles de confidentialité.
                let mut all_ok = true;
                for rule in &state.confidentialite {
                    if let Err(e) = db.confidentialite_upsert(rule) {
                        state.status_message =
                            Some(format!("Erreur confidentialité : {}", e));
                        all_ok = false;
                        break;
                    }
                }
                // Sauvegarder le toggle annuaire dans le profil.
                if all_ok {
                    match db.exposant_upsert(&state.fiche_form) {
                        Ok(()) => {
                            state.status_message =
                                Some("Confidentialité enregistrée.".to_string());
                        }
                        Err(e) => {
                            state.status_message =
                                Some(format!("Erreur profil : {}", e));
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

/// Récupère la valeur d'un champ du profil par son nom.
fn get_field_value(profile: &crate::data::ExposantProfile, field_name: &str) -> String {
    match field_name {
        "company_name" => profile.company_name.clone(),
        "slogan" => profile.slogan.clone(),
        "description_short" => profile.description_short.clone(),
        "description_long" => profile.description_long.clone(),
        "contact_email" => profile.contact_email.clone(),
        "contact_phone" => profile.contact_phone.clone(),
        "adresse_siege" => profile.adresse_siege.clone(),
        "site_web" => profile.site_web.clone(),
        "social_facebook" => profile.social_facebook.clone(),
        "social_instagram" => profile.social_instagram.clone(),
        "social_linkedin" => profile.social_linkedin.clone(),
        "social_tiktok" => profile.social_tiktok.clone(),
        "social_youtube" => profile.social_youtube.clone(),
        "social_pinterest" => profile.social_pinterest.clone(),
        "social_x" => profile.social_x.clone(),
        "secteur" => profile.secteur.clone(),
        "tags" => profile.tags.clone(),
        _ => None,
    }
    .unwrap_or_else(|| "—".to_string())
}

/// Récupère la visibilité d'un champ depuis les règles de confidentialité.
fn get_field_visibility(
    rules: &[ConfidentialiteProfil],
    field_name: &str,
) -> Visibility {
    rules
        .iter()
        .find(|r| r.field_name.as_deref() == Some(field_name))
        .and_then(|r| r.visibility.as_deref())
        .map(Visibility::from_str)
        .unwrap_or(Visibility::Public)
}

/// Met à jour la visibilité d'un champ dans le vecteur de confidentialité.
fn set_field_visibility(
    rules: &mut Vec<ConfidentialiteProfil>,
    exposant_id: &Option<String>,
    field_name: &str,
    visibility: Visibility,
) {
    if let Some(existing) = rules
        .iter_mut()
        .find(|r| r.field_name.as_deref() == Some(field_name))
    {
        existing.visibility = Some(visibility.as_str().to_string());
    } else {
        rules.push(ConfidentialiteProfil {
            id: None,
            exposant_id: exposant_id.clone(),
            field_name: Some(field_name.to_string()),
            visibility: Some(visibility.as_str().to_string()),
            updated_at: None,
        });
    }
}
