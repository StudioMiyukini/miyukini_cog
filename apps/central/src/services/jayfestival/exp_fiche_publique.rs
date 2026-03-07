//! EXP-E14 — Fiche publique exposant (aperçu et édition).
//!
//! @id: jf_exp_fiche_publique @do: render_exp_fiche_publique
//! @role: ui @layer: service
//! @human: Ecran EXP-E14 JayFestival: apercu et edition de la fiche publique exposant.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{Badge, ActionButton};

/// Gestion de la fiche publique exposant.
#[component]
pub fn ExpFichePublique() -> Element {
    let p = use_palette();
    let conns = use_service_connections();
    let mut edit_mode = use_signal(|| false);

    // Charger les données exposant
    let exposant = {
        let db = &conns.read().jayfestival;
        db.exposants_list(false).unwrap_or_default().into_iter().next()
    };

    let company_name = exposant.as_ref().and_then(|e| e.company_name.clone()).unwrap_or_else(|| "Mon entreprise".to_string());
    let stand_name = exposant.as_ref().and_then(|e| e.stand_name.clone()).unwrap_or_default();
    let description = exposant.as_ref().and_then(|e| e.description.clone()).unwrap_or_default();
    let category = exposant.as_ref().and_then(|e| e.category.clone()).unwrap_or_default();
    let site_web = exposant.as_ref().and_then(|e| e.site_web.clone()).unwrap_or_default();
    let visible = exposant.as_ref().and_then(|e| e.visible_repertoire).unwrap_or(false);

    let is_editing = *edit_mode.read();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                div {
                    h2 {
                        style: "font-size: 24px; color: {p.text_high}; margin-bottom: 8px;",
                        "Ma fiche publique"
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        Badge {
                            text: if visible { "Visible".to_string() } else { "Masquee".to_string() },
                            color: if visible { p.success.to_string() } else { p.text_muted.to_string() },
                        }
                        span {
                            style: "font-size: 12px; color: {p.text_muted};",
                            "dans l'annuaire public"
                        }
                    }
                }

                div {
                    style: "display: flex; gap: 8px;",

                    ActionButton {
                        label: "Apercu".to_string(),
                        icon: "👁️".to_string(),
                        accent: false,
                        onclick: move |_| {},
                    }

                    if is_editing {
                        ActionButton {
                            label: "Enregistrer".to_string(),
                            icon: "💾".to_string(),
                            accent: true,
                            onclick: move |_| { edit_mode.set(false); },
                        }
                    } else {
                        ActionButton {
                            label: "Modifier".to_string(),
                            icon: "✏️".to_string(),
                            accent: true,
                            onclick: move |_| { edit_mode.set(true); },
                        }
                    }
                }
            }

            // Aperçu de la fiche
            section {
                style: "background: {p.bg_secondary}; border-radius: 8px; padding: 24px;",

                // En-tête fiche
                div {
                    style: "display: flex; gap: 24px; margin-bottom: 24px; padding-bottom: 24px; border-bottom: 1px solid {p.border_default};",

                    // Logo
                    div {
                        style: "width: 120px; height: 120px; background: {p.bg_base}; border-radius: 12px; display: flex; align-items: center; justify-content: center; font-size: 48px;",
                        "🏪"
                    }

                    // Infos principales
                    div {
                        style: "flex: 1;",

                        if is_editing {
                            input {
                                r#type: "text",
                                style: "width: 100%; padding: 8px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_high}; font-size: 20px; font-weight: 600; margin-bottom: 8px;",
                                value: "{company_name}",
                            }
                        } else {
                            h3 {
                                style: "font-size: 20px; color: {p.text_high}; margin-bottom: 8px;",
                                "{company_name}"
                            }
                        }

                        if !stand_name.is_empty() || is_editing {
                            if is_editing {
                                input {
                                    r#type: "text",
                                    style: "width: 100%; padding: 6px 10px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_secondary}; font-size: 14px; font-style: italic; margin-bottom: 12px;",
                                    value: "{stand_name}",
                                    placeholder: "Nom commercial du stand",
                                }
                            } else {
                                p {
                                    style: "font-size: 14px; color: {p.text_secondary}; font-style: italic; margin-bottom: 12px;",
                                    "\"{stand_name}\""
                                }
                            }
                        }

                        div {
                            style: "display: flex; gap: 12px;",

                            if !category.is_empty() {
                                Badge {
                                    text: category.clone(),
                                    color: p.accent_primary.to_string(),
                                }
                            }

                            if !site_web.is_empty() {
                                span {
                                    style: "font-size: 13px; color: {p.accent_primary};",
                                    "🌐 {site_web}"
                                }
                            }
                        }
                    }
                }

                // Description
                div {
                    h4 {
                        style: "font-size: 14px; color: {p.text_high}; margin-bottom: 12px;",
                        "Description"
                    }

                    if is_editing {
                        textarea {
                            style: "width: 100%; padding: 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 14px; resize: vertical; min-height: 120px;",
                            value: "{description}",
                            placeholder: "Decrivez votre activite, vos produits, vos services...",
                        }
                    } else {
                        p {
                            style: "font-size: 14px; color: {p.text_primary}; line-height: 1.6;",
                            if description.is_empty() {
                                "Aucune description renseignee"
                            } else {
                                "{description}"
                            }
                        }
                    }
                }
            }

            // Paramètres de visibilité
            section {
                style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                    "Visibilite"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    VisibilityOption {
                        title: "Visible dans l'annuaire",
                        description: "Votre fiche apparait dans l'annuaire public des exposants",
                        enabled: visible,
                    }
                    VisibilityOption {
                        title: "Afficher les coordonnees",
                        description: "Email et telephone visibles sur la fiche publique",
                        enabled: true,
                    }
                    VisibilityOption {
                        title: "Autoriser les messages",
                        description: "Les visiteurs peuvent vous contacter via le formulaire",
                        enabled: true,
                    }
                }
            }

            // Statistiques
            section {
                style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                    "Statistiques de la fiche"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                    StatBox { label: "Vues ce mois", value: "127" }
                    StatBox { label: "Messages recus", value: "8" }
                    StatBox { label: "Clics site web", value: "23" }
                }
            }
        }
    }
}

#[component]
fn VisibilityOption(title: &'static str, description: &'static str, enabled: bool) -> Element {
    let p = use_palette();
    let toggle_bg = if enabled { p.accent_primary } else { p.bg_overlay };
    let toggle_pos = if enabled { "right: 2px" } else { "left: 2px" };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; padding: 12px; background: {p.bg_base}; border-radius: 6px;",

            div {
                style: "flex: 1;",
                p {
                    style: "font-size: 14px; color: {p.text_high}; margin-bottom: 2px;",
                    "{title}"
                }
                p {
                    style: "font-size: 12px; color: {p.text_muted};",
                    "{description}"
                }
            }

            // Toggle
            div {
                style: "width: 44px; height: 24px; background: {toggle_bg}; border-radius: 12px; position: relative; cursor: pointer; transition: background 0.2s;",

                div {
                    style: "width: 20px; height: 20px; background: white; border-radius: 50%; position: absolute; top: 2px; {toggle_pos}; transition: all 0.2s;",
                }
            }
        }
    }
}

#[component]
fn StatBox(label: &'static str, value: &'static str) -> Element {
    let p = use_palette();

    rsx! {
        div {
            style: "background: {p.bg_base}; border-radius: 8px; padding: 16px; text-align: center;",
            p {
                style: "font-size: 24px; color: {p.text_high}; font-weight: 600; margin-bottom: 4px;",
                "{value}"
            }
            p {
                style: "font-size: 12px; color: {p.text_muted};",
                "{label}"
            }
        }
    }
}
