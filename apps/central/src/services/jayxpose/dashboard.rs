//! Dashboard JayXpose (XP-E01) — tableau de bord exposant avec donnees reelles.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{StatCard, QuickAccessCard, ActionButton};
use super::{JayXposeSection, JayXposeState};

#[component]
pub fn Dashboard(state: Signal<JayXposeState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();
    let exposant_id = state.read().exposant_id.clone().unwrap_or_default();

    // Charger les donnees reelles
    let db = &conns.read().jayxpose;
    let exposant = db.exposant_by_id(&exposant_id).ok().flatten();
    let product_count = db.produits_by_exposant(&exposant_id).map(|v| v.len()).unwrap_or(0);
    let doc_count = db.documents_by_exposant(&exposant_id).map(|v| v.len()).unwrap_or(0);
    let page_count = db.vitrine_pages_by_exposant(&exposant_id).map(|v| v.len()).unwrap_or(0);
    let categories_count = db.categories_by_exposant(&exposant_id).map(|v| v.len()).unwrap_or(0);

    // Docs expirants (dans 30 jours)
    let expiring_docs = db.documents_by_exposant(&exposant_id)
        .unwrap_or_default()
        .into_iter()
        .filter(|d| {
            d.status.as_deref() == Some("valide") && d.expires_at.is_some()
        })
        .count();

    let company_name = exposant.as_ref()
        .and_then(|e| e.company_name.clone())
        .unwrap_or_else(|| "Mon entreprise".to_string());

    let vitrine_status = exposant.as_ref()
        .and_then(|e| e.vitrine_status.clone())
        .unwrap_or_else(|| "brouillon".to_string());

    let vitrine_color = match vitrine_status.as_str() {
        "publiee" => c.accent_green,
        "suspendue" => c.accent_orange,
        _ => c.text_muted,
    };

    // Completude profil
    let profile_completion = if let Some(ref e) = exposant {
        let fields: Vec<bool> = vec![
            e.company_name.as_ref().is_some_and(|s| !s.is_empty()),
            e.contact_email.as_ref().is_some_and(|s| !s.is_empty()),
            e.contact_phone.as_ref().is_some_and(|s| !s.is_empty()),
            e.siret.as_ref().is_some_and(|s| !s.is_empty()),
            e.description_short.as_ref().is_some_and(|s| !s.is_empty()),
            e.secteur.as_ref().is_some_and(|s| !s.is_empty()),
            e.logo_url.as_ref().is_some_and(|s| !s.is_empty()),
            e.site_web.as_ref().is_some_and(|s| !s.is_empty()),
        ];
        let filled = fields.iter().filter(|&&b| b).count();
        (filled * 100) / fields.len()
    } else {
        0
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tete avec nom + statut vitrine
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",

                div {
                    h2 {
                        style: "font-size: 24px; color: {c.text_white};",
                        "🏪 {company_name}"
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 8px; margin-top: 4px;",
                        span {
                            style: "padding: 4px 10px; background: {vitrine_color}20; color: {vitrine_color}; border-radius: 4px; font-size: 11px; font-weight: 500;",
                            "Vitrine: {vitrine_status}"
                        }
                    }
                }
            }

            // Jauge de completude du profil
            div {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px;",

                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;",
                    span { style: "font-size: 13px; color: {c.text_secondary};", "Completude du profil" }
                    span { style: "font-size: 13px; color: {c.text_white}; font-weight: 600;", "{profile_completion}%" }
                }
                div {
                    style: "width: 100%; height: 8px; background: {c.bg_hover}; border-radius: 4px; overflow: hidden;",
                    div {
                        style: "width: {profile_completion}%; height: 100%; background: {c.accent_blue}; border-radius: 4px; transition: width 0.3s;",
                    }
                }
                if profile_completion < 100 {
                    p {
                        style: "font-size: 11px; color: {c.text_muted}; margin-top: 8px;",
                        "Completez votre fiche entreprise pour ameliorer votre visibilite."
                    }
                }
            }

            // Alertes documents expirants
            if expiring_docs > 0 {
                div {
                    style: "background: {c.accent_orange}15; border: 1px solid {c.accent_orange}40; border-radius: 8px; padding: 12px 16px; display: flex; align-items: center; gap: 12px;",

                    span { style: "font-size: 20px;", "⚠️" }
                    div {
                        p {
                            style: "font-size: 13px; color: {c.accent_orange}; font-weight: 500;",
                            "{expiring_docs} document(s) a surveiller"
                        }
                        p {
                            style: "font-size: 11px; color: {c.text_secondary};",
                            "Verifiez les dates d'expiration dans votre coffre-fort."
                        }
                    }
                    button {
                        style: "margin-left: auto; padding: 6px 12px; background: {c.accent_orange}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;",
                        onclick: move |_| { state.write().section = JayXposeSection::Documents; },
                        "Voir"
                    }
                }
            }

            // Statistiques
            div {
                style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px;",

                StatCard { label: "Produits".to_string(), value: product_count.to_string(), icon: "📦".to_string(), color: c.accent_blue.to_string() }
                StatCard { label: "Categories".to_string(), value: categories_count.to_string(), icon: "🏷️".to_string(), color: "#8b5cf6".to_string() }
                StatCard { label: "Documents".to_string(), value: doc_count.to_string(), icon: "📄".to_string(), color: c.accent_orange.to_string() }
                StatCard { label: "Pages vitrine".to_string(), value: page_count.to_string(), icon: "🏪".to_string(), color: c.accent_green.to_string() }
            }

            // Acces rapides
            h3 {
                style: "font-size: 16px; color: {c.text_white};",
                "Acces rapides"
            }
            div {
                style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 12px;",

                QuickAccessCard {
                    icon: "🏢".to_string(),
                    title: "Fiche entreprise".to_string(),
                    count: format!("{profile_completion}% complet"),
                    onclick: move |_| { state.write().section = JayXposeSection::Entreprise; },
                }
                QuickAccessCard {
                    icon: "📦".to_string(),
                    title: "Mon catalogue".to_string(),
                    count: format!("{product_count} produit(s)"),
                    onclick: move |_| { state.write().section = JayXposeSection::Catalogue; },
                }
                QuickAccessCard {
                    icon: "🏪".to_string(),
                    title: "Ma vitrine".to_string(),
                    count: vitrine_status.clone(),
                    onclick: move |_| { state.write().section = JayXposeSection::Vitrine; },
                }
                QuickAccessCard {
                    icon: "📁".to_string(),
                    title: "Mes documents".to_string(),
                    count: format!("{doc_count} document(s)"),
                    onclick: move |_| { state.write().section = JayXposeSection::Documents; },
                }
                QuickAccessCard {
                    icon: "👁️".to_string(),
                    title: "Fiche publique".to_string(),
                    count: String::new(),
                    onclick: move |_| { state.write().section = JayXposeSection::FichePublique; },
                }
            }

            // Actions rapides
            h3 {
                style: "font-size: 16px; color: {c.text_white};",
                "Actions rapides"
            }
            div {
                style: "display: flex; gap: 12px;",

                ActionButton {
                    label: "Ajouter un produit".to_string(),
                    icon: "➕".to_string(),
                    accent: true,
                    onclick: move |_| {
                        let mut s = state.write();
                        s.editing_product_id = None;
                        s.section = JayXposeSection::NouveauProduit;
                    },
                }
                ActionButton {
                    label: "Ajouter un document".to_string(),
                    icon: "📤".to_string(),
                    onclick: move |_| { state.write().section = JayXposeSection::Documents; },
                }
                ActionButton {
                    label: "Modifier la vitrine".to_string(),
                    icon: "✏️".to_string(),
                    onclick: move |_| { state.write().section = JayXposeSection::Vitrine; },
                }
            }
        }
    }
}
