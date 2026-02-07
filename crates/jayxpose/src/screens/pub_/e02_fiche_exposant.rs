//! XP-P02 — Fiche exposant publique.
//!
//! Affichage lecture seule de la fiche exposant : logo, description,
//! contact (si public), secteur/tags, réseaux sociaux, liens vers catalogue et vitrine.

use crate::data::JayXposeDb;
use crate::screens::pub_::PubState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use crate::ui;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_pub_e02
/// @do: render_fiche_exposant_public
/// @layer: app
///
/// Affiche la fiche publique d'un exposant avec respect de la confidentialité.
pub fn pub_e02_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut PubState,
    db: &Arc<JayXposeDb>,
) {
    // Chargement de la fiche et des règles de confidentialité.
    if !state.fiche_loaded {
        if let Some(ref id) = state.selected_exposant_id {
            if let Ok(Some(profile)) = db.exposant_by_id(id) {
                state.fiche_exposant = Some(profile);
            }
            if let Ok(rules) = db.confidentialite_by_exposant(id) {
                state.confidentialite_rules = rules;
            }
        }
        state.fiche_loaded = true;
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            match &state.fiche_exposant {
                None => {
                    ui::heading(ui, theme, "Fiche exposant");
                    ui.add_space(theme.item_spacing());
                    ui::caption(ui, theme, "Aucun exposant sélectionné.");
                }
                Some(exposant) => {
                    // ── En-tête : logo, nom, slogan ──
                    ui.horizontal(|ui| {
                        if exposant.logo_url.is_some() {
                            ui::caption(ui, theme, "[logo]");
                            ui.add_space(theme.item_spacing());
                        }
                        ui.vertical(|ui| {
                            let name =
                                exposant.company_name.as_deref().unwrap_or("Exposant");
                            ui::heading(ui, theme, name);
                            if let Some(ref slogan) = exposant.slogan {
                                ui.label(
                                    egui::RichText::new(slogan)
                                        .size(theme.font_size_body())
                                        .color(theme.text_secondary())
                                        .italics(),
                                );
                            }
                        });
                    });

                    ui.add_space(theme.section_spacing());
                    ui.separator();

                    // ── Description ──
                    ui::section(ui, theme, "Description", |ui| {
                        if let Some(ref desc) = exposant.description_long {
                            ui::body(ui, theme, desc);
                        } else if let Some(ref desc) = exposant.description_short {
                            ui::body(ui, theme, desc);
                        } else {
                            ui::caption(ui, theme, "Aucune description.");
                        }
                    });

                    // ── Contact (si public) ──
                    if is_field_public(&state.confidentialite_rules, "contact_email")
                        || is_field_public(&state.confidentialite_rules, "contact_phone")
                    {
                        ui::section(ui, theme, "Contact", |ui| {
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
                        });
                    }

                    // ── Secteur & Tags ──
                    ui::section(ui, theme, "Secteur & Tags", |ui| {
                        if let Some(ref secteur) = exposant.secteur {
                            ui.horizontal(|ui| {
                                ui::caption(ui, theme, "Secteur :");
                                ui::badge(ui, secteur, theme.accent());
                            });
                        }
                        if let Some(ref tags) = exposant.tags {
                            ui.add_space(4.0);
                            ui.horizontal_wrapped(|ui| {
                                ui::caption(ui, theme, "Tags :");
                                for tag in tags.split(',') {
                                    let tag = tag.trim();
                                    if !tag.is_empty() {
                                        ui::badge(
                                            ui,
                                            tag,
                                            theme.text_secondary(),
                                        );
                                    }
                                }
                            });
                        }
                    });

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

                    ui.add_space(theme.section_spacing());
                    ui.separator();
                    ui.add_space(theme.item_spacing());

                    // ── Liens de navigation ──
                    ui.horizontal(|ui| {
                        if ui::accent_button(ui, theme, "Voir le catalogue").clicked() {
                            state.catalogue_loaded = false;
                            let _ = nav_request
                                .replace(Some(ScreenId::PubCataloguePublic));
                        }
                        ui.add_space(theme.item_spacing());
                        if ui::accent_button(ui, theme, "Visiter la vitrine").clicked()
                        {
                            state.vitrine_loaded = false;
                            state.vitrine_tab = 0;
                            let _ =
                                nav_request.replace(Some(ScreenId::PubVitrine));
                        }
                    });

                    ui.add_space(theme.item_spacing());

                    if ui::secondary_button(ui, theme, "Retour à l'annuaire").clicked()
                    {
                        let _ =
                            nav_request.replace(Some(ScreenId::PubAnnuaire));
                    }
                }
            }
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
        .unwrap_or(true) // Par défaut, visible publiquement.
}
