//! Vue générique pour un service.

use dioxus::prelude::*;
use crate::state::use_app_state;
/// Propriétés de la vue de service.
#[derive(Props, Clone, PartialEq)]
pub struct ServiceViewProps {
    pub service_id: String,
}

/// Vue générique d'un service (fallback).
#[component]
pub fn ServiceView(props: ServiceViewProps) -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
    let service = state.read().services.iter()
        .find(|s| s.id == props.service_id)
        .cloned();

    if let Some(service) = service {
        rsx! {
            div {
                style: "display: flex; flex-direction: column; gap: 24px;",

                // En-tête du service
                div {
                    style: "display: flex; gap: 24px; align-items: flex-start;",

                    // Icône large
                    div {
                        style: "width: 120px; height: 120px; background: linear-gradient(135deg, {c.bg_hover} 0%, {c.bg_secondary} 100%); border-radius: 16px; display: flex; align-items: center; justify-content: center; font-size: 48px;",
                        "{service.icon}"
                    }

                    // Informations
                    div {
                        style: "flex: 1;",

                        h1 {
                            style: "font-size: 28px; color: {c.text_white}; margin-bottom: 8px;",
                            "{service.name}"
                        }

                        p {
                            style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 16px; max-width: 600px;",
                            "{service.description}"
                        }

                        div {
                            style: "display: flex; gap: 12px; align-items: center;",

                            // Badge de type
                            span {
                                style: "padding: 4px 12px; background: {service.service_type.color()}20; color: {service.service_type.color()}; border-radius: 4px; font-size: 12px;",
                                "{service.service_type.label()}"
                            }

                            // Version
                            span {
                                style: "color: {c.text_muted}; font-size: 12px;",
                                "v{service.version}"
                            }

                            // Développeur
                            span {
                                style: "color: {c.text_muted}; font-size: 12px;",
                                "par {service.developer}"
                            }
                        }
                    }

                    // Actions
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",

                        if service.is_installed {
                            button {
                                style: "padding: 12px 32px; background: {c.accent_green}; color: white; border: none; border-radius: 4px; font-size: 14px; font-weight: 500; cursor: pointer;",
                                "▶ Lancer"
                            }
                        } else {
                            button {
                                style: "padding: 12px 32px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; font-size: 14px; font-weight: 500; cursor: pointer;",
                                "Installer"
                            }
                        }

                        button {
                            style: "padding: 8px 16px; background: transparent; color: {c.text_secondary}; border: 1px solid {c.border}; border-radius: 4px; font-size: 12px; cursor: pointer;",
                            if service.is_favorite { "★ Retirer des favoris" } else { "☆ Ajouter aux favoris" }
                        }
                    }
                }

                // Séparateur
                hr {
                    style: "border: none; border-top: 1px solid {c.border}; margin: 8px 0;",
                }

                // Contenu du service (placeholder)
                div {
                    style: "flex: 1; background: {c.bg_secondary}; border-radius: 8px; padding: 24px; min-height: 300px;",

                    div {
                        style: "text-align: center; color: {c.text_muted}; padding: 48px;",

                        span {
                            style: "font-size: 48px; display: block; margin-bottom: 16px; opacity: 0.5;",
                            "{service.icon}"
                        }
                        p { "Interface du service {service.name}" }
                        p {
                            style: "font-size: 12px; margin-top: 8px;",
                            "Le contenu spécifique sera chargé ici."
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                style: "text-align: center; padding: 48px; color: {c.text_secondary};",
                "Service non trouvé"
            }
        }
    }
}
