//! Fiche publique / annuaire (XP-E12) — visibilite, confidentialite, apercu.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::JayXposeState;

/// Champ avec selecteur de confidentialite.
const PRIVACY_FIELDS: &[(&str, &str)] = &[
    ("contact_email", "Email de contact"),
    ("contact_phone", "Telephone"),
    ("adresse_siege", "Adresse siege"),
    ("adresse_correspondance", "Adresse correspondance"),
    ("siret", "SIRET"),
    ("siren", "SIREN"),
    ("site_web", "Site web"),
    ("social_facebook", "Facebook"),
    ("social_instagram", "Instagram"),
    ("social_linkedin", "LinkedIn"),
];

const PRIVACY_LEVELS: &[(&str, &str)] = &[
    ("public", "Public"),
    ("authentifie", "Authentifie"),
    ("organisateur", "Organisateur"),
    ("prive", "Prive"),
];

#[component]
pub fn FichePublique(state: Signal<JayXposeState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();
    let exposant_id = state.read().exposant_id.clone().unwrap_or_default();

    // Charger donnees
    let db = &conns.read().jayxpose;
    let exposant = db.exposant_by_id(&exposant_id).ok().flatten().unwrap_or_default();
    let products = db.produits_by_exposant(&exposant_id).unwrap_or_default();
    let featured: Vec<_> = products.iter().filter(|p| p.is_featured.unwrap_or(false)).take(6).collect();
    let confidentialite = db.confidentialite_by_exposant(&exposant_id).unwrap_or_default();

    let visible_annuaire = exposant.visible_annuaire.unwrap_or(false);
    let vitrine_status = exposant.vitrine_status.as_deref().unwrap_or("brouillon");
    let company = exposant.company_name.as_deref().unwrap_or("Mon entreprise");
    let secteur = exposant.secteur.as_deref().unwrap_or("");
    let desc = exposant.description_short.as_deref().unwrap_or("Aucune description.");
    let slug_display = exposant.vitrine_slug.as_deref().unwrap_or("---");

    // Pre-calcul styles conditionnels
    let vis_btn_bg = if visible_annuaire { c.accent_green.to_string() } else { c.bg_hover.to_string() };
    let vis_btn_color = if visible_annuaire { "white".to_string() } else { c.text_secondary.to_string() };
    let vis_btn_text = if visible_annuaire { "Visible ✓" } else { "Masque" };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Header
            div {
                h2 { style: "font-size: 24px; color: {c.text_white};", "👁️ Fiche publique" }
                p { style: "font-size: 13px; color: {c.text_secondary}; margin-top: 4px;", "Gerez votre visibilite dans l'annuaire et la confidentialite de vos donnees." }
            }

            // Visibilite annuaire
            div {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; display: flex; flex-direction: column; gap: 16px;",

                h3 { style: "font-size: 14px; color: {c.text_white}; font-weight: 600; padding-bottom: 8px; border-bottom: 1px solid {c.border};", "Visibilite" }

                div {
                    style: "display: flex; align-items: center; justify-content: space-between;",

                    div {
                        p { style: "font-size: 14px; color: {c.text_white};", "Visible dans l'annuaire" }
                        p { style: "font-size: 12px; color: {c.text_muted}; margin-top: 2px;", "Votre fiche apparaitra dans l'annuaire public des exposants." }
                    }

                    button {
                        style: "padding: 8px 16px; background: {vis_btn_bg}; color: {vis_btn_color}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 13px;",
                        onclick: move |_| {
                            let mut exp = exposant.clone();
                            exp.visible_annuaire = Some(!visible_annuaire);
                            let db = &conns.read().jayxpose;
                            let _ = db.exposant_upsert(&exp);
                        },
                        "{vis_btn_text}"
                    }
                }

                if vitrine_status == "publiee" {
                    div {
                        style: "display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: {c.bg_hover}; border-radius: 4px;",
                        span { style: "font-size: 14px;", "🔗" }
                        p { style: "font-size: 13px; color: {c.text_primary};",
                            "Vitrine publiee : vitrine.miyukini.local/{slug_display}"
                        }
                    }
                }
            }

            // Confidentialite par champ
            div {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; display: flex; flex-direction: column; gap: 16px;",

                h3 { style: "font-size: 14px; color: {c.text_white}; font-weight: 600; padding-bottom: 8px; border-bottom: 1px solid {c.border};", "Confidentialite par champ" }

                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "Choisissez le niveau de visibilite pour chaque donnee sensible."
                }

                for (field_key, field_label) in PRIVACY_FIELDS.iter() {
                    {
                        let current = confidentialite.iter()
                            .find(|c| c.field_name.as_deref() == Some(field_key))
                            .and_then(|c| c.visibility.clone())
                            .unwrap_or_else(|| "prive".to_string());

                        let fk = field_key.to_string();
                        let fk_for_cb = fk.clone();
                        let fl = field_label.to_string();
                        let eid = exposant_id.clone();

                        rsx! {
                            div {
                                key: "{fk}",
                                style: "display: flex; align-items: center; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid {c.border};",

                                span { style: "font-size: 13px; color: {c.text_primary};", "{fl}" }

                                select {
                                    style: "padding: 6px 10px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 12px;",
                                    onchange: move |evt| {
                                        let conf = jayxpose::data::ConfidentialiteProfil {
                                            id: Some(uuid::Uuid::new_v4().to_string()),
                                            exposant_id: Some(eid.clone()),
                                            field_name: Some(fk_for_cb.clone()),
                                            visibility: Some(evt.value()),
                                            ..Default::default()
                                        };
                                        let db = &conns.read().jayxpose;
                                        let _ = db.confidentialite_upsert(&conf);
                                    },

                                    for (val, lbl) in PRIVACY_LEVELS.iter() {
                                        {
                                            let is_selected = current == *val;
                                            let v = val.to_string();
                                            let l = lbl.to_string();
                                            rsx! {
                                                option {
                                                    value: "{v}",
                                                    selected: is_selected,
                                                    "{l}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Apercu fiche publique
            div {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; display: flex; flex-direction: column; gap: 16px;",

                h3 { style: "font-size: 14px; color: {c.text_white}; font-weight: 600; padding-bottom: 8px; border-bottom: 1px solid {c.border};", "Apercu de la fiche publique" }

                div {
                    style: "border: 1px solid {c.border}; border-radius: 8px; overflow: hidden;",

                    // Banniere
                    div {
                        style: "background: linear-gradient(135deg, {c.accent_blue} 0%, #6366f1 100%); padding: 20px; text-align: center;",

                        div {
                            style: "width: 64px; height: 64px; background: rgba(255,255,255,0.2); border-radius: 50%; display: flex; align-items: center; justify-content: center; margin: 0 auto 8px;",
                            span { style: "font-size: 28px;", "🏢" }
                        }
                        h3 { style: "font-size: 18px; color: white;", "{company}" }
                        if !secteur.is_empty() {
                            p { style: "font-size: 12px; color: rgba(255,255,255,0.7);", "{secteur}" }
                        }
                    }

                    div {
                        style: "padding: 16px;",

                        p { style: "font-size: 13px; color: {c.text_secondary}; margin-bottom: 16px;", "{desc}" }

                        if !featured.is_empty() {
                            div {
                                h4 { style: "font-size: 14px; color: {c.text_white}; margin-bottom: 8px;", "Produits vedettes" }
                                div {
                                    style: "display: flex; gap: 8px; flex-wrap: wrap;",
                                    for p in featured.iter() {
                                        {
                                            let pid = p.id.clone().unwrap_or_default();
                                            let pname = p.name.clone().unwrap_or_else(|| "Produit".to_string());
                                            rsx! {
                                                div {
                                                    key: "{pid}",
                                                    style: "padding: 6px 12px; background: {c.bg_hover}; border-radius: 4px; font-size: 12px; color: {c.text_primary};",
                                                    "{pname}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
