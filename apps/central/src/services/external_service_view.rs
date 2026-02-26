//! Vue d'un service externe — panneau de contrôle pour lancer/arrêter un service.
//!
//! Quand un utilisateur ouvre un onglet de service, Central affiche ce panneau
//! avec le statut, un bouton lancer/arrêter, et les infos du service.

use dioxus::prelude::*;
use crate::state::{use_app_state, use_service_manager};

/// Panneau de contrôle d'un service externe (lancé en tant que processus indépendant).
#[component]
pub fn ExternalServiceView(service_id: String) -> Element {
    let state = use_app_state();
    let manager = use_service_manager();
    let c = state.read().current_theme.palette();

    let service = state.read().services.iter()
        .find(|s| s.id == service_id)
        .cloned();

    let is_installed = manager.is_installed(&service_id);
    let is_running = manager.is_running(&service_id);

    let status_text = if is_running {
        "En cours d'ex\u{e9}cution"
    } else if is_installed {
        "Install\u{e9} \u{2014} Arr\u{ea}t\u{e9}"
    } else {
        "Non install\u{e9}"
    };

    let status_color = if is_running {
        "#10b981"  // Green
    } else if is_installed {
        "#6b7280"  // Gray
    } else {
        "#ef4444"  // Red
    };

    if let Some(svc) = service {
        let svc_name = svc.name.clone();
        let svc_desc = svc.description.clone();
        let svc_icon = svc.icon.clone();
        let svc_version = svc.version.clone();
        let svc_developer = svc.developer.clone();
        let type_color = svc.service_type.color();
        let type_label = svc.service_type.label();

        let sid_launch = service_id.clone();
        let sid_stop = service_id.clone();
        let manager_launch = manager.clone();
        let manager_stop = manager.clone();

        rsx! {
            div {
                style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; padding: 48px; gap: 32px;",

                // Icône du service (grand)
                div {
                    style: "width: 120px; height: 120px; background: linear-gradient(135deg, {type_color}30, {type_color}10); border-radius: 24px; display: flex; align-items: center; justify-content: center;",
                    span { style: "font-size: 56px;", "{svc_icon}" }
                }

                // Nom + description
                div {
                    style: "text-align: center; max-width: 500px;",
                    h1 { style: "font-size: 28px; color: {c.text_white}; font-weight: 600; margin-bottom: 8px;", "{svc_name}" }
                    p { style: "font-size: 14px; color: {c.text_secondary}; line-height: 1.6;", "{svc_desc}" }
                }

                // Statut
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    span { style: "width: 10px; height: 10px; border-radius: 50%; background: {status_color};" }
                    span { style: "font-size: 14px; color: {status_color}; font-weight: 500;", "{status_text}" }
                }

                // Boutons d'action
                div {
                    style: "display: flex; gap: 16px;",

                    if is_installed {
                        if is_running {
                            button {
                                style: "padding: 14px 40px; background: #ef4444; color: white; border: none; border-radius: 10px; font-size: 15px; font-weight: 600; cursor: pointer; transition: opacity 0.2s;",
                                onclick: move |_| {
                                    if let Err(e) = manager_stop.stop_service(&sid_stop) {
                                        tracing::error!("Arr\u{ea}t du service \u{e9}chou\u{e9}: {e}");
                                    }
                                },
                                "Arr\u{ea}ter"
                            }
                        } else {
                            button {
                                style: "padding: 14px 40px; background: {c.accent_blue}; color: white; border: none; border-radius: 10px; font-size: 15px; font-weight: 600; cursor: pointer; transition: opacity 0.2s;",
                                onclick: move |_| {
                                    let profile_id = state.read().current_user
                                        .as_ref()
                                        .map(|u| u.id.clone())
                                        .unwrap_or_default();
                                    if let Err(e) = manager_launch.launch_service(&sid_launch, &profile_id) {
                                        tracing::error!("Lancement du service \u{e9}chou\u{e9}: {e}");
                                    }
                                },
                                "Lancer"
                            }
                        }
                    } else {
                        button {
                            style: "padding: 14px 40px; background: {c.accent_green}; color: white; border: none; border-radius: 10px; font-size: 15px; font-weight: 600; cursor: pointer; opacity: 0.6;",
                            disabled: true,
                            "Installer depuis le Market"
                        }
                    }
                }

                // Infos complémentaires
                div {
                    style: "display: flex; gap: 24px; margin-top: 16px;",

                    div {
                        style: "text-align: center;",
                        p { style: "font-size: 11px; color: {c.text_muted};", "Version" }
                        p { style: "font-size: 13px; color: {c.text_secondary}; font-weight: 500;", "v{svc_version}" }
                    }
                    div {
                        style: "text-align: center;",
                        p { style: "font-size: 11px; color: {c.text_muted};", "D\u{e9}veloppeur" }
                        p { style: "font-size: 13px; color: {c.text_secondary}; font-weight: 500;", "{svc_developer}" }
                    }
                    div {
                        style: "text-align: center;",
                        p { style: "font-size: 11px; color: {c.text_muted};", "Type" }
                        div {
                            style: "display: flex; align-items: center; gap: 4px;",
                            span { style: "width: 6px; height: 6px; border-radius: 50%; background: {type_color};" }
                            p { style: "font-size: 13px; color: {c.text_secondary}; font-weight: 500;", "{type_label}" }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; color: {c.text_muted};",
                p { "Service introuvable : {service_id}" }
            }
        }
    }
}
