//! Ma vitrine — Parametres, pages et previsualisation (XP-E06/07/08).

use dioxus::prelude::*;
use miyukini_service_ui::use_palette;
use super::components::{Badge, FormField, FormSection};
use super::JayXposeState;

/// Onglet actif dans la section Vitrine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum VitrineTab {
    #[default]
    Parametres,
    Pages,
    Preview,
}

#[component]
pub fn Vitrine(state: Signal<JayXposeState>) -> Element {
    let c = use_palette();
    let db = crate::use_db();
    let exposant_id = state.read().exposant_id.clone().unwrap_or_default();
    let mut active_tab = use_signal(|| VitrineTab::Parametres);

    // Charger donnees
    let exposant = db.exposant_by_id(&exposant_id).ok().flatten().unwrap_or_default();
    let exposant_publish = exposant.clone();
    let exposant_suspend = exposant.clone();
    let pages = db.vitrine_pages_by_exposant(&exposant_id).unwrap_or_default();

    let vitrine_status = exposant.vitrine_status.clone().unwrap_or_else(|| "brouillon".to_string());
    let status_color = match vitrine_status.as_str() {
        "publiee" => c.accent_green,
        "suspendue" => c.accent_orange,
        _ => c.text_muted,
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            // Header
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",

                div {
                    style: "display: flex; align-items: center; gap: 12px;",
                    h2 { style: "font-size: 24px; color: {c.text_white};", "🏪 Ma vitrine" }
                    Badge { text: vitrine_status.clone(), color: status_color.to_string() }
                }

                // Actions publication
                div {
                    style: "display: flex; gap: 8px;",

                    if vitrine_status != "publiee" {
                        {
                            let db_pub = db.clone();
                            rsx! {
                                button {
                                    style: "padding: 8px 16px; background: {c.accent_green}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px;",
                                    onclick: move |_| {
                                        let mut exp = exposant_publish.clone();
                                        exp.vitrine_status = Some("publiee".to_string());
                                        let _ = db_pub.exposant_upsert(&exp);
                                    },
                                    "🚀 Publier"
                                }
                            }
                        }
                    }
                    if vitrine_status == "publiee" {
                        {
                            let db_sus = db.clone();
                            rsx! {
                                button {
                                    style: "padding: 8px 16px; background: {c.accent_orange}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px;",
                                    onclick: move |_| {
                                        let mut exp = exposant_suspend.clone();
                                        exp.vitrine_status = Some("suspendue".to_string());
                                        let _ = db_sus.exposant_upsert(&exp);
                                    },
                                    "⏸ Suspendre"
                                }
                            }
                        }
                    }
                }
            }

            // Onglets
            div {
                style: "display: flex; gap: 2px; border-bottom: 1px solid {c.border};",

                TabButton { label: "Parametres", is_active: *active_tab.read() == VitrineTab::Parametres, onclick: move |_| active_tab.set(VitrineTab::Parametres) }
                TabButton { label: "Pages", is_active: *active_tab.read() == VitrineTab::Pages, onclick: move |_| active_tab.set(VitrineTab::Pages) }
                TabButton { label: "Previsualisation", is_active: *active_tab.read() == VitrineTab::Preview, onclick: move |_| active_tab.set(VitrineTab::Preview) }
            }

            // Contenu
            match *active_tab.read() {
                VitrineTab::Parametres => rsx! {
                    VitrineParametres { exposant_id: exposant_id.clone() }
                },
                VitrineTab::Pages => rsx! {
                    VitrinePages { exposant_id: exposant_id.clone(), pages: pages.clone() }
                },
                VitrineTab::Preview => rsx! {
                    VitrinePreview { exposant_id: exposant_id.clone() }
                },
            }
        }
    }
}

/// Onglet Parametres vitrine (XP-E06).
#[component]
fn VitrineParametres(exposant_id: String) -> Element {
    let c = use_palette();
    let db = crate::use_db();

    let exposant = db.exposant_by_id(&exposant_id).ok().flatten().unwrap_or_default();

    let mut slug = use_signal(|| exposant.vitrine_slug.clone().unwrap_or_default());
    let mut seo_title = use_signal(|| exposant.seo_title.clone().unwrap_or_default());
    let mut seo_description = use_signal(|| exposant.seo_description.clone().unwrap_or_default());
    let mut seo_keywords = use_signal(|| exposant.seo_keywords.clone().unwrap_or_default());
    let mut save_msg = use_signal(String::new);

    let db_save = db.clone();
    let on_save = move |_| {
        let mut exp = exposant.clone();
        exp.vitrine_slug = Some(slug.read().clone());
        exp.seo_title = Some(seo_title.read().clone());
        exp.seo_description = Some(seo_description.read().clone());
        exp.seo_keywords = Some(seo_keywords.read().clone());
        match db_save.exposant_upsert(&exp) {
            Ok(()) => save_msg.set("Parametres enregistres.".to_string()),
            Err(e) => save_msg.set(format!("Erreur: {e}")),
        }
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px; max-width: 700px;",

            if !save_msg.read().is_empty() {
                div {
                    style: "padding: 10px 16px; background: {c.accent_green}15; border: 1px solid {c.accent_green}40; border-radius: 4px; font-size: 13px; color: {c.accent_green};",
                    "{save_msg}"
                }
            }

            FormSection {
                title: "URL de la vitrine".to_string(),

                FormField { label: "Slug".to_string(), value: slug.read().clone(), placeholder: "mon-entreprise".to_string(), oninput: move |evt: FormEvent| slug.set(evt.value()) }
                p {
                    style: "font-size: 11px; color: {c.text_muted};",
                    "URL : https://vitrine.miyukini.local/{slug}"
                }
            }

            FormSection {
                title: "Referencement SEO".to_string(),

                FormField { label: "Titre SEO".to_string(), value: seo_title.read().clone(), placeholder: "Titre affiche dans les moteurs de recherche".to_string(), optional: true, oninput: move |evt: FormEvent| seo_title.set(evt.value()) }
                FormField { label: "Meta description".to_string(), value: seo_description.read().clone(), placeholder: "Description courte pour le referencement".to_string(), optional: true, oninput: move |evt: FormEvent| seo_description.set(evt.value()) }
                FormField { label: "Mots-cles".to_string(), value: seo_keywords.read().clone(), placeholder: "mot1, mot2, mot3".to_string(), optional: true, oninput: move |evt: FormEvent| seo_keywords.set(evt.value()) }
            }

            div {
                style: "display: flex; justify-content: flex-end;",
                button {
                    style: "padding: 10px 24px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px;",
                    onclick: on_save,
                    "💾 Enregistrer"
                }
            }
        }
    }
}

/// Onglet Pages (gestion des pages vitrine).
#[component]
fn VitrinePages(exposant_id: String, pages: Vec<jayxpose::data::VitrinePage>) -> Element {
    let c = use_palette();
    let db = crate::use_db();

    let page_types = ["accueil", "catalogue", "presentation", "contact"];

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px;",

            p {
                style: "font-size: 13px; color: {c.text_secondary};",
                "Gerez les pages de votre vitrine. Activez ou desactivez chaque page."
            }

            for pt in page_types.iter() {
                {
                    let existing = pages.iter().find(|p| p.page_type.as_deref() == Some(pt));
                    let is_visible = existing.and_then(|p| p.is_visible).unwrap_or(false);
                    let page_id = existing.and_then(|p| p.id.clone());
                    let pt_str = pt.to_string();
                    let pt_icon = match *pt {
                        "accueil" => "🏠",
                        "catalogue" => "📦",
                        "presentation" => "📖",
                        "contact" => "📧",
                        _ => "📄",
                    };
                    let pt_label = match *pt {
                        "accueil" => "Page d'accueil",
                        "catalogue" => "Catalogue produits",
                        "presentation" => "Presentation",
                        "contact" => "Page contact",
                        _ => pt,
                    };

                    let status_color = if is_visible { c.accent_green } else { c.text_muted };
                    let status_text = if is_visible { "Active" } else { "Inactive" };
                    let btn_bg = if is_visible { c.accent_green.to_string() } else { c.bg_hover.to_string() };
                    let btn_color = if is_visible { "white".to_string() } else { c.text_secondary.to_string() };
                    let btn_text = if is_visible { "Desactiver" } else { "Activer" };
                    let eid = exposant_id.clone();
                    let db_upsert = db.clone();

                    rsx! {
                        div {
                            key: "{pt}",
                            style: "display: flex; align-items: center; justify-content: space-between; padding: 16px; background: {c.bg_secondary}; border-radius: 8px;",

                            div {
                                style: "display: flex; align-items: center; gap: 12px;",
                                span { style: "font-size: 24px;", "{pt_icon}" }
                                div {
                                    h4 { style: "font-size: 14px; color: {c.text_white};", "{pt_label}" }
                                    p { style: "font-size: 11px; color: {c.text_muted};", "Type: {pt}" }
                                }
                            }

                            div {
                                style: "display: flex; align-items: center; gap: 12px;",

                                span {
                                    style: "font-size: 12px; color: {status_color};",
                                    "{status_text}"
                                }
                                button {
                                    style: "padding: 6px 12px; background: {btn_bg}; color: {btn_color}; border: 1px solid {c.border}; border-radius: 4px; cursor: pointer; font-size: 12px;",
                                    onclick: move |_| {
                                        let page = jayxpose::data::VitrinePage {
                                            id: page_id.clone().or_else(|| Some(uuid::Uuid::new_v4().to_string())),
                                            exposant_id: Some(eid.clone()),
                                            page_type: Some(pt_str.clone()),
                                            is_visible: Some(!is_visible),
                                            ..Default::default()
                                        };
                                        let _ = db_upsert.vitrine_page_upsert(&page);
                                    },
                                    "{btn_text}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Onglet Preview (XP-E08).
#[component]
fn VitrinePreview(exposant_id: String) -> Element {
    let c = use_palette();
    let db = crate::use_db();

    let exposant = db.exposant_by_id(&exposant_id).ok().flatten().unwrap_or_default();
    let products = db.produits_by_exposant(&exposant_id).unwrap_or_default();
    let featured: Vec<_> = products.iter().filter(|p| p.is_featured.unwrap_or(false)).take(6).collect();

    let company = exposant.company_name.as_deref().unwrap_or("Mon entreprise");
    let slogan = exposant.slogan.as_deref().unwrap_or("");
    let desc = exposant.description_short.as_deref().unwrap_or("Aucune description.");

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px; border: 1px solid {c.border}; border-radius: 8px; overflow: hidden;",

            // Banniere
            div {
                style: "background: linear-gradient(135deg, {c.accent_blue} 0%, #6366f1 100%); padding: 40px 32px; text-align: center;",

                h1 { style: "font-size: 28px; color: white; font-weight: 700;", "{company}" }
                if !slogan.is_empty() {
                    p { style: "font-size: 16px; color: rgba(255,255,255,0.8); margin-top: 8px;", "{slogan}" }
                }
            }

            // Contenu
            div {
                style: "padding: 24px;",

                // Description
                div {
                    style: "margin-bottom: 32px;",
                    h2 { style: "font-size: 18px; color: {c.text_white}; margin-bottom: 8px;", "A propos" }
                    p { style: "font-size: 14px; color: {c.text_secondary}; line-height: 1.6;", "{desc}" }
                }

                // Produits vedettes
                if !featured.is_empty() {
                    div {
                        h2 { style: "font-size: 18px; color: {c.text_white}; margin-bottom: 16px;", "Produits vedettes" }
                        div {
                            style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                            for p in featured.iter() {
                                div {
                                    key: "{p.id.as_deref().unwrap_or(\"\")}",
                                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; text-align: center;",

                                    div {
                                        style: "width: 100%; height: 80px; background: {c.bg_hover}; border-radius: 4px; display: flex; align-items: center; justify-content: center; font-size: 32px; margin-bottom: 12px;",
                                        "📦"
                                    }
                                    h3 { style: "font-size: 14px; color: {c.text_white};", "{p.name.as_deref().unwrap_or(\"Produit\")}" }
                                    p {
                                        style: "font-size: 13px; color: {c.accent_blue}; margin-top: 4px;",
                                        {p.price.map_or_else(|| "Sur demande".to_string(), |pr| format!("{pr:.2} EUR"))}
                                    }
                                }
                            }
                        }
                    }
                }

                // Contact
                div {
                    style: "margin-top: 32px; padding-top: 20px; border-top: 1px solid {c.border};",
                    h2 { style: "font-size: 18px; color: {c.text_white}; margin-bottom: 12px;", "Contact" }
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px; font-size: 13px; color: {c.text_secondary};",
                        if let Some(ref email) = exposant.contact_email {
                            p { "📧 {email}" }
                        }
                        if let Some(ref phone) = exposant.contact_phone {
                            p { "📞 {phone}" }
                        }
                        if let Some(ref web) = exposant.site_web {
                            p { "🌐 {web}" }
                        }
                    }
                }
            }
        }
    }
}

/// Bouton onglet.
#[component]
fn TabButton(label: &'static str, is_active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let c = use_palette();
    let bg = if is_active { c.accent_blue } else { "transparent" };
    let color = if is_active { "white" } else { c.text_secondary };

    rsx! {
        button {
            style: "padding: 8px 16px; background: {bg}; color: {color}; border: none; border-radius: 4px 4px 0 0; cursor: pointer; font-size: 13px; font-weight: 500; transition: all 0.2s;",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}
