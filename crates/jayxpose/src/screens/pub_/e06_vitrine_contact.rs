//! XP-P06 — Page contact vitrine.
//!
//! Page de contact d'un exposant : informations publiques (email, téléphone,
//! adresse), réseaux sociaux, lien site web, et formulaire de contact simple.

use crate::data::JayXposeDb;
use crate::screens::pub_::PubState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use crate::ui;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_pub_e06
/// @do: render_vitrine_contact_page
/// @layer: app
///
/// Affiche la page contact complète d'un exposant (vitrine).
pub fn pub_e06_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut PubState,
    db: &Arc<JayXposeDb>,
) {
    // S'assurer que la fiche est chargée.
    if state.fiche_exposant.is_none() {
        if let Some(ref id) = state.selected_exposant_id {
            if let Ok(Some(profile)) = db.exposant_by_id(id) {
                state.fiche_exposant = Some(profile);
            }
            if let Ok(rules) = db.confidentialite_by_exposant(id) {
                state.confidentialite_rules = rules;
            }
        }
    }

    let company_name = state
        .fiche_exposant
        .as_ref()
        .and_then(|e| e.company_name.clone())
        .unwrap_or_else(|| "Exposant".to_string());

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // ── En-tête ──
            ui::heading(ui, theme, &format!("Contact — {company_name}"));
            ui.add_space(theme.section_spacing());

            if let Some(ref exposant) = state.fiche_exposant.clone() {
                // ── Informations de contact (si public) ──
                let has_contact_info = exposant.contact_email.is_some()
                    || exposant.contact_phone.is_some()
                    || exposant.adresse_siege.is_some();

                if has_contact_info {
                    ui::section(ui, theme, "Informations de contact", |ui| {
                        if is_field_public(
                            &state.confidentialite_rules,
                            "contact_email",
                        ) {
                            if let Some(ref email) = exposant.contact_email {
                                ui.horizontal(|ui| {
                                    ui::caption(ui, theme, "Email :");
                                    ui::body(ui, theme, email);
                                });
                            }
                        }

                        if is_field_public(
                            &state.confidentialite_rules,
                            "contact_phone",
                        ) {
                            if let Some(ref phone) = exposant.contact_phone {
                                ui.horizontal(|ui| {
                                    ui::caption(ui, theme, "Téléphone :");
                                    ui::body(ui, theme, phone);
                                });
                            }
                        }

                        if is_field_public(
                            &state.confidentialite_rules,
                            "adresse_siege",
                        ) {
                            if let Some(ref adresse) = exposant.adresse_siege {
                                ui.horizontal(|ui| {
                                    ui::caption(ui, theme, "Adresse :");
                                    ui::body(ui, theme, adresse);
                                });
                            }
                        }
                    });
                }

                // ── Réseaux sociaux ──
                let socials = [
                    ("Facebook", &exposant.social_facebook),
                    ("Instagram", &exposant.social_instagram),
                    ("LinkedIn", &exposant.social_linkedin),
                    ("TikTok", &exposant.social_tiktok),
                    ("YouTube", &exposant.social_youtube),
                    ("Pinterest", &exposant.social_pinterest),
                    ("X", &exposant.social_x),
                ];
                let has_social = socials.iter().any(|(_, v)| v.is_some());

                if has_social {
                    ui::section(ui, theme, "Réseaux sociaux", |ui| {
                        for (name, value) in &socials {
                            if let Some(ref url) = value {
                                ui.horizontal(|ui| {
                                    ui::caption(ui, theme, name);
                                    ui::body(ui, theme, url);
                                });
                            }
                        }
                    });
                }

                // ── Site web ──
                if let Some(ref site) = exposant.site_web {
                    ui::section(ui, theme, "Site web", |ui| {
                        ui::body(ui, theme, site);
                    });
                }
            } else {
                ui::caption(ui, theme, "Aucun exposant sélectionné.");
            }

            ui.add_space(theme.section_spacing());
            ui.separator();

            // ── Formulaire de contact ──
            ui::section(ui, theme, "Envoyer un message", |ui| {
                ui::form_field(ui, theme, "Votre nom", &mut state.contact_nom);
                ui.add_space(theme.item_spacing());
                ui::form_field(ui, theme, "Votre email", &mut state.contact_email);
                ui.add_space(theme.item_spacing());
                ui::form_textarea(ui, theme, "Votre message", &mut state.contact_message);
                ui.add_space(theme.item_spacing());

                let can_send = !state.contact_nom.is_empty()
                    && !state.contact_email.is_empty()
                    && !state.contact_message.is_empty();

                ui.add_enabled_ui(can_send, |ui| {
                    if ui::accent_button(ui, theme, "Envoyer").clicked() {
                        // Pas de backend pour l'instant : on réinitialise le formulaire.
                        state.contact_nom.clear();
                        state.contact_email.clear();
                        state.contact_message.clear();
                    }
                });

                if !can_send {
                    ui.add_space(4.0);
                    ui::caption(
                        ui,
                        theme,
                        "Remplissez tous les champs pour envoyer le message.",
                    );
                }
            });

            ui.add_space(theme.section_spacing());

            // ── Navigation ──
            ui.horizontal(|ui| {
                if ui::secondary_button(ui, theme, "Retour à la vitrine").clicked() {
                    let _ = nav_request.replace(Some(ScreenId::PubVitrine));
                }
                ui.add_space(theme.item_spacing());
                if ui::secondary_button(ui, theme, "Retour à la fiche").clicked() {
                    let _ = nav_request.replace(Some(ScreenId::PubFicheExposant));
                }
            });
        });
    });
}

/// Vérifie si un champ est marqué « public » dans les règles de confidentialité.
/// Si aucune règle n'existe pour le champ, il est considéré comme public par défaut.
fn is_field_public(
    rules: &[crate::data::ConfidentialiteProfil],
    field_name: &str,
) -> bool {
    rules
        .iter()
        .find(|r| r.field_name.as_deref() == Some(field_name))
        .map(|r| r.visibility.as_deref() == Some("public"))
        .unwrap_or(true)
}
