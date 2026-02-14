//! Fiche entreprise (XP-E02) — gestion du profil exposant enrichi.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{FormField, FormTextarea, FormSection, PageHeader};
use super::JayXposeState;

#[component]
pub fn Entreprise(state: Signal<JayXposeState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();
    let exposant_id = state.read().exposant_id.clone().unwrap_or_default();

    // Charger le profil existant
    let existing = {
        let db = &conns.read().jayxpose;
        db.exposant_by_id(&exposant_id).ok().flatten().unwrap_or_default()
    };

    // Signaux pour chaque champ
    let mut company_name = use_signal(|| existing.company_name.clone().unwrap_or_default());
    let mut legal_form = use_signal(|| existing.legal_form.clone().unwrap_or_default());
    let mut slogan = use_signal(|| existing.slogan.clone().unwrap_or_default());
    let mut description_short = use_signal(|| existing.description_short.clone().unwrap_or_default());
    let mut description_long = use_signal(|| existing.description_long.clone().unwrap_or_default());
    let mut contact_email = use_signal(|| existing.contact_email.clone().unwrap_or_default());
    let mut contact_phone = use_signal(|| existing.contact_phone.clone().unwrap_or_default());
    let mut adresse_siege = use_signal(|| existing.adresse_siege.clone().unwrap_or_default());
    let mut adresse_correspondance = use_signal(|| existing.adresse_correspondance.clone().unwrap_or_default());
    let mut siret = use_signal(|| existing.siret.clone().unwrap_or_default());
    let mut siren = use_signal(|| existing.siren.clone().unwrap_or_default());
    let mut code_ape = use_signal(|| existing.code_ape.clone().unwrap_or_default());
    let mut num_immatriculation = use_signal(|| existing.num_immatriculation.clone().unwrap_or_default());
    let mut secteur = use_signal(|| existing.secteur.clone().unwrap_or_default());
    let mut site_web = use_signal(|| existing.site_web.clone().unwrap_or_default());
    let mut contact_fact_nom = use_signal(|| existing.contact_facturation_nom.clone().unwrap_or_default());
    let mut contact_fact_email = use_signal(|| existing.contact_facturation_email.clone().unwrap_or_default());
    let mut contact_fact_phone = use_signal(|| existing.contact_facturation_phone.clone().unwrap_or_default());
    let mut contact_log_nom = use_signal(|| existing.contact_logistique_nom.clone().unwrap_or_default());
    let mut contact_log_email = use_signal(|| existing.contact_logistique_email.clone().unwrap_or_default());
    let mut contact_log_phone = use_signal(|| existing.contact_logistique_phone.clone().unwrap_or_default());
    let mut social_facebook = use_signal(|| existing.social_facebook.clone().unwrap_or_default());
    let mut social_instagram = use_signal(|| existing.social_instagram.clone().unwrap_or_default());
    let mut social_linkedin = use_signal(|| existing.social_linkedin.clone().unwrap_or_default());
    let mut social_tiktok = use_signal(|| existing.social_tiktok.clone().unwrap_or_default());
    let mut social_youtube = use_signal(|| existing.social_youtube.clone().unwrap_or_default());
    let mut social_x = use_signal(|| existing.social_x.clone().unwrap_or_default());

    let mut save_message = use_signal(String::new);

    let on_save = move |_| {
        let profile = jayxpose::data::ExposantProfile {
            id: Some(exposant_id.clone()),
            company_name: Some(company_name.read().clone()),
            legal_form: Some(legal_form.read().clone()),
            slogan: Some(slogan.read().clone()),
            description_short: Some(description_short.read().clone()),
            description_long: Some(description_long.read().clone()),
            contact_email: Some(contact_email.read().clone()),
            contact_phone: Some(contact_phone.read().clone()),
            adresse_siege: Some(adresse_siege.read().clone()),
            adresse_correspondance: Some(adresse_correspondance.read().clone()),
            siret: Some(siret.read().clone()),
            siren: Some(siren.read().clone()),
            code_ape: Some(code_ape.read().clone()),
            num_immatriculation: Some(num_immatriculation.read().clone()),
            secteur: Some(secteur.read().clone()),
            site_web: Some(site_web.read().clone()),
            contact_facturation_nom: Some(contact_fact_nom.read().clone()),
            contact_facturation_email: Some(contact_fact_email.read().clone()),
            contact_facturation_phone: Some(contact_fact_phone.read().clone()),
            contact_logistique_nom: Some(contact_log_nom.read().clone()),
            contact_logistique_email: Some(contact_log_email.read().clone()),
            contact_logistique_phone: Some(contact_log_phone.read().clone()),
            social_facebook: Some(social_facebook.read().clone()),
            social_instagram: Some(social_instagram.read().clone()),
            social_linkedin: Some(social_linkedin.read().clone()),
            social_tiktok: Some(social_tiktok.read().clone()),
            social_youtube: Some(social_youtube.read().clone()),
            social_x: Some(social_x.read().clone()),
            ..Default::default()
        };
        let db = &conns.read().jayxpose;
        match db.exposant_upsert(&profile) {
            Ok(()) => save_message.set("Profil enregistre avec succes.".to_string()),
            Err(e) => save_message.set(format!("Erreur: {e}")),
        }
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px; max-width: 800px;",

            PageHeader {
                title: "🏢 Fiche entreprise".to_string(),
                subtitle: "Gerez les informations de votre entreprise".to_string(),
            }

            // Message de sauvegarde
            if !save_message.read().is_empty() {
                div {
                    style: "padding: 10px 16px; background: {c.accent_green}15; border: 1px solid {c.accent_green}40; border-radius: 4px; font-size: 13px; color: {c.accent_green};",
                    "{save_message}"
                }
            }

            // Section Identite
            FormSection {
                title: "Identite".to_string(),

                FormField { label: "Raison sociale".to_string(), value: company_name.read().clone(), placeholder: "Nom de l'entreprise".to_string(), oninput: move |evt: FormEvent| company_name.set(evt.value()) }
                FormField { label: "Forme juridique".to_string(), value: legal_form.read().clone(), placeholder: "SARL, SAS, Auto-entrepreneur...".to_string(), optional: true, oninput: move |evt: FormEvent| legal_form.set(evt.value()) }
                FormField { label: "Slogan".to_string(), value: slogan.read().clone(), placeholder: "Votre accroche courte".to_string(), optional: true, oninput: move |evt: FormEvent| slogan.set(evt.value()) }
                FormTextarea { label: "Description courte".to_string(), value: description_short.read().clone(), placeholder: "Resume pour l'annuaire (2-3 lignes)".to_string(), rows: 3, oninput: move |evt: FormEvent| description_short.set(evt.value()) }
                FormTextarea { label: "Description longue".to_string(), value: description_long.read().clone(), placeholder: "Presentation detaillee pour la vitrine".to_string(), rows: 6, optional: true, oninput: move |evt: FormEvent| description_long.set(evt.value()) }
            }

            // Section Juridique
            FormSection {
                title: "Informations juridiques".to_string(),

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",
                    FormField { label: "SIRET".to_string(), value: siret.read().clone(), placeholder: "14 chiffres".to_string(), oninput: move |evt: FormEvent| siret.set(evt.value()) }
                    FormField { label: "SIREN".to_string(), value: siren.read().clone(), placeholder: "9 chiffres".to_string(), optional: true, oninput: move |evt: FormEvent| siren.set(evt.value()) }
                    FormField { label: "Code APE/NAF".to_string(), value: code_ape.read().clone(), placeholder: "Ex: 4711B".to_string(), optional: true, oninput: move |evt: FormEvent| code_ape.set(evt.value()) }
                    FormField { label: "N° immatriculation".to_string(), value: num_immatriculation.read().clone(), placeholder: "RCS, RM...".to_string(), optional: true, oninput: move |evt: FormEvent| num_immatriculation.set(evt.value()) }
                }
            }

            // Section Contact principal
            FormSection {
                title: "Contact principal".to_string(),

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",
                    FormField { label: "Email".to_string(), value: contact_email.read().clone(), placeholder: "contact@entreprise.fr".to_string(), field_type: "email".to_string(), oninput: move |evt: FormEvent| contact_email.set(evt.value()) }
                    FormField { label: "Telephone".to_string(), value: contact_phone.read().clone(), placeholder: "+33 1 23 45 67 89".to_string(), field_type: "tel".to_string(), oninput: move |evt: FormEvent| contact_phone.set(evt.value()) }
                }
                FormTextarea { label: "Adresse siege".to_string(), value: adresse_siege.read().clone(), placeholder: "Adresse complete du siege social".to_string(), rows: 2, oninput: move |evt: FormEvent| adresse_siege.set(evt.value()) }
                FormTextarea { label: "Adresse de correspondance".to_string(), value: adresse_correspondance.read().clone(), placeholder: "Si differente du siege".to_string(), rows: 2, optional: true, oninput: move |evt: FormEvent| adresse_correspondance.set(evt.value()) }
            }

            // Section Contact facturation
            FormSection {
                title: "Contact facturation".to_string(),

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 16px;",
                    FormField { label: "Nom".to_string(), value: contact_fact_nom.read().clone(), optional: true, oninput: move |evt: FormEvent| contact_fact_nom.set(evt.value()) }
                    FormField { label: "Email".to_string(), value: contact_fact_email.read().clone(), field_type: "email".to_string(), optional: true, oninput: move |evt: FormEvent| contact_fact_email.set(evt.value()) }
                    FormField { label: "Telephone".to_string(), value: contact_fact_phone.read().clone(), field_type: "tel".to_string(), optional: true, oninput: move |evt: FormEvent| contact_fact_phone.set(evt.value()) }
                }
            }

            // Section Contact logistique
            FormSection {
                title: "Contact logistique".to_string(),

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 16px;",
                    FormField { label: "Nom".to_string(), value: contact_log_nom.read().clone(), optional: true, oninput: move |evt: FormEvent| contact_log_nom.set(evt.value()) }
                    FormField { label: "Email".to_string(), value: contact_log_email.read().clone(), field_type: "email".to_string(), optional: true, oninput: move |evt: FormEvent| contact_log_email.set(evt.value()) }
                    FormField { label: "Telephone".to_string(), value: contact_log_phone.read().clone(), field_type: "tel".to_string(), optional: true, oninput: move |evt: FormEvent| contact_log_phone.set(evt.value()) }
                }
            }

            // Section Activite
            FormSection {
                title: "Activite et web".to_string(),

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",
                    FormField { label: "Secteur d'activite".to_string(), value: secteur.read().clone(), placeholder: "Ex: Artisanat, Alimentaire...".to_string(), oninput: move |evt: FormEvent| secteur.set(evt.value()) }
                    FormField { label: "Site web".to_string(), value: site_web.read().clone(), placeholder: "https://www.exemple.fr".to_string(), field_type: "url".to_string(), optional: true, oninput: move |evt: FormEvent| site_web.set(evt.value()) }
                }
            }

            // Section Reseaux sociaux
            FormSection {
                title: "Reseaux sociaux".to_string(),

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",
                    FormField { label: "Facebook".to_string(), value: social_facebook.read().clone(), placeholder: "URL Facebook".to_string(), optional: true, oninput: move |evt: FormEvent| social_facebook.set(evt.value()) }
                    FormField { label: "Instagram".to_string(), value: social_instagram.read().clone(), placeholder: "URL Instagram".to_string(), optional: true, oninput: move |evt: FormEvent| social_instagram.set(evt.value()) }
                    FormField { label: "LinkedIn".to_string(), value: social_linkedin.read().clone(), placeholder: "URL LinkedIn".to_string(), optional: true, oninput: move |evt: FormEvent| social_linkedin.set(evt.value()) }
                    FormField { label: "TikTok".to_string(), value: social_tiktok.read().clone(), placeholder: "URL TikTok".to_string(), optional: true, oninput: move |evt: FormEvent| social_tiktok.set(evt.value()) }
                    FormField { label: "YouTube".to_string(), value: social_youtube.read().clone(), placeholder: "URL YouTube".to_string(), optional: true, oninput: move |evt: FormEvent| social_youtube.set(evt.value()) }
                    FormField { label: "X (Twitter)".to_string(), value: social_x.read().clone(), placeholder: "URL X".to_string(), optional: true, oninput: move |evt: FormEvent| social_x.set(evt.value()) }
                }
            }

            // Bouton sauvegarder
            div {
                style: "display: flex; justify-content: flex-end; padding-top: 8px;",
                button {
                    style: "padding: 12px 24px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px; font-weight: 500;",
                    onclick: on_save,
                    "💾 Enregistrer"
                }
            }
        }
    }
}
