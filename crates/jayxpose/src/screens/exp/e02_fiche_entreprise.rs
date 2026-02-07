//! XP-E02 — Fiche entreprise.
//!
//! Formulaire complet pour éditer les informations de l'entreprise exposante :
//! identité, juridique, contacts, visuels, réseaux sociaux, activité.

use crate::data::JayXposeDb;
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_exp_e02
/// @do: render_company_profile_form
/// @layer: app
/// Écran fiche entreprise : formulaire sections pliables, bouton enregistrer.
/// Affiche l'écran XP-E02 — Fiche entreprise.
pub fn exp_e02_show(
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
                ui.heading("Fiche entreprise");
            });
            ui.add_space(theme.section_spacing());

            let form = &mut state.fiche_form;

            // --- Section Identité ---
            ui.collapsing("Section Identité", |ui| {
                ui.add_space(theme.item_spacing());
                ui.label("Raison sociale");
                let mut company_name = form.company_name.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut company_name).changed() {
                    form.company_name = Some(company_name);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Forme juridique");
                let mut legal_form = form.legal_form.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut legal_form).changed() {
                    form.legal_form = Some(legal_form);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Slogan / accroche");
                let mut slogan = form.slogan.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut slogan).changed() {
                    form.slogan = Some(slogan);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Description courte");
                let mut desc_short = form.description_short.clone().unwrap_or_default();
                if ui.text_edit_multiline(&mut desc_short).changed() {
                    form.description_short = Some(desc_short);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Description longue");
                let mut desc_long = form.description_long.clone().unwrap_or_default();
                if ui.text_edit_multiline(&mut desc_long).changed() {
                    form.description_long = Some(desc_long);
                }
            });

            ui.add_space(theme.item_spacing());

            // --- Section Juridique ---
            ui.collapsing("Section Juridique", |ui| {
                ui.add_space(theme.item_spacing());
                ui.label("SIRET");
                let mut siret = form.siret.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut siret).changed() {
                    form.siret = Some(siret);
                }

                ui.add_space(theme.item_spacing());
                ui.label("SIREN");
                let mut siren = form.siren.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut siren).changed() {
                    form.siren = Some(siren);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Code APE");
                let mut code_ape = form.code_ape.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut code_ape).changed() {
                    form.code_ape = Some(code_ape);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Numéro d'immatriculation");
                let mut num_immat = form.num_immatriculation.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut num_immat).changed() {
                    form.num_immatriculation = Some(num_immat);
                }
            });

            ui.add_space(theme.item_spacing());

            // --- Section Contact principal ---
            ui.collapsing("Section Contact principal", |ui| {
                ui.add_space(theme.item_spacing());
                ui.label("Email de contact");
                let mut email = form.contact_email.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut email).changed() {
                    form.contact_email = Some(email);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Téléphone");
                let mut phone = form.contact_phone.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut phone).changed() {
                    form.contact_phone = Some(phone);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Adresse du siège");
                let mut adresse = form.adresse_siege.clone().unwrap_or_default();
                if ui.text_edit_multiline(&mut adresse).changed() {
                    form.adresse_siege = Some(adresse);
                }
            });

            ui.add_space(theme.item_spacing());

            // --- Section Contact facturation ---
            ui.collapsing("Section Contact facturation", |ui| {
                ui.add_space(theme.item_spacing());
                ui.label("Nom contact facturation");
                let mut nom = form.contact_facturation_nom.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut nom).changed() {
                    form.contact_facturation_nom = Some(nom);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Email facturation");
                let mut email = form.contact_facturation_email.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut email).changed() {
                    form.contact_facturation_email = Some(email);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Téléphone facturation");
                let mut phone = form.contact_facturation_phone.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut phone).changed() {
                    form.contact_facturation_phone = Some(phone);
                }
            });

            ui.add_space(theme.item_spacing());

            // --- Section Contact logistique ---
            ui.collapsing("Section Contact logistique", |ui| {
                ui.add_space(theme.item_spacing());
                ui.label("Nom contact logistique");
                let mut nom = form.contact_logistique_nom.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut nom).changed() {
                    form.contact_logistique_nom = Some(nom);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Email logistique");
                let mut email = form.contact_logistique_email.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut email).changed() {
                    form.contact_logistique_email = Some(email);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Téléphone logistique");
                let mut phone = form.contact_logistique_phone.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut phone).changed() {
                    form.contact_logistique_phone = Some(phone);
                }
            });

            ui.add_space(theme.item_spacing());

            // --- Section Visuels ---
            ui.collapsing("Section Visuels", |ui| {
                ui.add_space(theme.item_spacing());
                ui.label("URL du logo");
                let mut logo = form.logo_url.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut logo).changed() {
                    form.logo_url = Some(logo);
                }

                ui.add_space(theme.item_spacing());
                ui.label("URL de la bannière");
                let mut banner = form.banner_url.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut banner).changed() {
                    form.banner_url = Some(banner);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Site web");
                let mut site = form.site_web.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut site).changed() {
                    form.site_web = Some(site);
                }
            });

            ui.add_space(theme.item_spacing());

            // --- Section Réseaux sociaux ---
            ui.collapsing("Section Réseaux sociaux", |ui| {
                ui.add_space(theme.item_spacing());
                ui.label("Facebook");
                let mut val = form.social_facebook.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut val).changed() {
                    form.social_facebook = Some(val);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Instagram");
                let mut val = form.social_instagram.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut val).changed() {
                    form.social_instagram = Some(val);
                }

                ui.add_space(theme.item_spacing());
                ui.label("LinkedIn");
                let mut val = form.social_linkedin.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut val).changed() {
                    form.social_linkedin = Some(val);
                }

                ui.add_space(theme.item_spacing());
                ui.label("TikTok");
                let mut val = form.social_tiktok.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut val).changed() {
                    form.social_tiktok = Some(val);
                }

                ui.add_space(theme.item_spacing());
                ui.label("YouTube");
                let mut val = form.social_youtube.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut val).changed() {
                    form.social_youtube = Some(val);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Pinterest");
                let mut val = form.social_pinterest.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut val).changed() {
                    form.social_pinterest = Some(val);
                }

                ui.add_space(theme.item_spacing());
                ui.label("X (ex-Twitter)");
                let mut val = form.social_x.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut val).changed() {
                    form.social_x = Some(val);
                }
            });

            ui.add_space(theme.item_spacing());

            // --- Section Activité ---
            ui.collapsing("Section Activité", |ui| {
                ui.add_space(theme.item_spacing());
                ui.label("Secteur d'activité");
                let mut secteur = form.secteur.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut secteur).changed() {
                    form.secteur = Some(secteur);
                }

                ui.add_space(theme.item_spacing());
                ui.label("Tags / mots-clés (séparés par des virgules)");
                let mut tags = form.tags.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut tags).changed() {
                    form.tags = Some(tags);
                }
            });

            ui.add_space(theme.section_spacing());

            // --- Bouton Enregistrer ---
            ui.horizontal(|ui| {
                if ui.button("Enregistrer").clicked() {
                    match db.exposant_upsert(&state.fiche_form) {
                        Ok(()) => {
                            state.status_message =
                                Some("Fiche entreprise enregistrée avec succès.".to_string());
                        }
                        Err(e) => {
                            state.status_message =
                                Some(format!("Erreur lors de l'enregistrement : {}", e));
                        }
                    }
                }
                if ui.button("Annuler").clicked() {
                    nav_request.replace(Some(ScreenId::ExpDashboard));
                }
            });

            // --- Message de statut ---
            if let Some(msg) = &state.status_message {
                ui.add_space(theme.item_spacing());
                ui.label(
                    egui::RichText::new(msg).color(theme.accent()),
                );
            }
        });
    });
}
