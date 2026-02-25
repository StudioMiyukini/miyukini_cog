//! Vue détaillée d'un service dans le Market.

use dioxus::prelude::*;
use crate::state::{use_app_state, ServiceRegistry};
use super::MarketState;

#[component]
pub fn ServiceDetail(state: Signal<MarketState>) -> Element {
    let app = use_app_state();
    let c = app.read().current_theme.palette();

    let service_id = state.read().selected_service_id.clone().unwrap_or_default();

    // Chercher dans les services installés puis dans les disponibles
    let installed = app.read().services.clone();
    let available = ServiceRegistry::local_available();

    let service = installed.iter()
        .chain(available.iter())
        .find(|s| s.id == service_id)
        .cloned();

    if let Some(svc) = service {
        let is_installed = svc.is_installed;
        let type_color = svc.service_type.color();
        let type_label = svc.service_type.label();
        let source_color = svc.source.badge_color();
        let source_label = svc.source.label();
        let svc_id = svc.id.clone();
        let svc_name = svc.name.clone();
        let svc_desc = svc.description.clone();
        let svc_icon = svc.icon.clone();
        let svc_version = svc.version.clone();
        let svc_developer = svc.developer.clone();

        // Pré-calculer les styles composés pour éviter les interpolations complexes
        let icon_bg = format!("width: 100px; height: 100px; background: linear-gradient(135deg, {type_color}30, {type_color}10); border-radius: 16px; display: flex; align-items: center; justify-content: center; flex-shrink: 0;");
        let type_badge_style = format!("display: flex; align-items: center; gap: 4px; padding: 4px 10px; background: {type_color}15; border: 1px solid {type_color}30; border-radius: 6px; font-size: 12px; color: {type_color};");
        let type_dot_style = format!("width: 6px; height: 6px; border-radius: 50%; background: {type_color};");
        let source_badge_style = format!("padding: 4px 10px; background: {source_color}15; border: 1px solid {source_color}30; border-radius: 6px; font-size: 12px; color: {source_color};");
        let desinstaller_style = format!("padding: 12px 24px; background: transparent; color: {0}; border: 1px solid {0}40; border-radius: 8px; font-size: 14px; cursor: pointer; transition: all 0.2s;", c.accent_red);
        let version_text = format!("v{svc_version}");
        let statut_label = if is_installed { "Install\u{e9}" } else { "Disponible" };
        let statut_icon = if is_installed { "\u{2705}" } else { "\u{1f4e5}" };

        // Info card data
        let type_label_str = type_label.to_string();

        rsx! {
            div {
                style: "display: flex; flex-direction: column; gap: 24px; max-width: 800px;",

                // Bouton retour
                button {
                    style: "align-self: flex-start; padding: 6px 14px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_secondary}; cursor: pointer; font-size: 12px; display: flex; align-items: center; gap: 6px;",
                    onclick: move |_| { state.write().selected_service_id = None; },
                    "\u{2190} Retour au catalogue"
                }

                // Header du service
                div {
                    style: "display: flex; gap: 24px; align-items: flex-start;",

                    div {
                        style: "{icon_bg}",
                        span { style: "font-size: 48px;", "{svc_icon}" }
                    }

                    div {
                        style: "flex: 1;",
                        h1 { style: "font-size: 28px; color: {c.text_white}; font-weight: 600; margin-bottom: 8px;", "{svc_name}" }
                        p { style: "font-size: 14px; color: {c.text_secondary}; line-height: 1.6; margin-bottom: 12px;", "{svc_desc}" }

                        div {
                            style: "display: flex; gap: 8px; flex-wrap: wrap;",

                            span {
                                style: "{type_badge_style}",
                                span { style: "{type_dot_style}" }
                                "{type_label}"
                            }
                            span {
                                style: "{source_badge_style}",
                                "{source_label}"
                            }
                            span {
                                style: "padding: 4px 10px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 6px; font-size: 12px; color: {c.text_muted};",
                                "{version_text}"
                            }
                        }
                    }
                }

                // Actions
                div {
                    style: "display: flex; gap: 12px;",
                    if is_installed {
                        button {
                            style: "padding: 12px 32px; background: {c.accent_blue}; color: white; border: none; border-radius: 8px; font-size: 14px; font-weight: 600; cursor: pointer; transition: background 0.2s;",
                            onclick: {
                                let svc_clone = svc.clone();
                                let mut app_state = app.clone();
                                move |_| {
                                    app_state.write().open_service(&svc_clone);
                                }
                            },
                            "Ouvrir le service"
                        }
                        button {
                            style: "{desinstaller_style}",
                            onclick: move |_| {
                                tracing::info!("D\u{e9}sinstallation de {} non disponible en Phase 1", svc_id);
                            },
                            "D\u{e9}sinstaller"
                        }
                    } else {
                        button {
                            style: "padding: 12px 32px; background: {c.accent_green}; color: white; border: none; border-radius: 8px; font-size: 14px; font-weight: 600; cursor: pointer; transition: background 0.2s;",
                            onclick: move |_| {
                                tracing::info!("Installation de {} \u{2014} Phase 1 : activez la feature Cargo et recompilez", svc_id);
                            },
                            "Installer"
                        }
                    }
                }

                div { style: "height: 1px; background: {c.border};" }

                // Informations détaillées
                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                    InfoRow { label: "D\u{e9}veloppeur", value: svc_developer, icon: "\u{1f464}" }
                    InfoRow { label: "Version", value: svc_version, icon: "\u{1f3f7}\u{fe0f}" }
                    InfoRow { label: "Type", value: type_label_str, icon: "\u{2699}\u{fe0f}" }
                    InfoRow { label: "Statut", value: statut_label.to_string(), icon: statut_icon }
                }

                // Note Phase 1
                div {
                    style: "background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 8px; padding: 16px;",
                    div {
                        style: "display: flex; gap: 12px; align-items: flex-start;",
                        span { style: "font-size: 18px;", "\u{2139}\u{fe0f}" }
                        div {
                            p {
                                style: "font-size: 13px; color: {c.text_white}; font-weight: 500; margin-bottom: 4px;",
                                "Phase 1 \u{2014} Installation par compilation"
                            }
                            p {
                                style: "font-size: 12px; color: {c.text_secondary}; line-height: 1.5;",
                                "Les services officiels sont int\u{e9}gr\u{e9}s via les features Cargo. Pour installer ou d\u{e9}sinstaller un service, modifiez les features dans Cargo.toml et recompilez Central. La Phase 2 permettra l'installation dynamique."
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                style: "text-align: center; padding: 80px;",
                button {
                    style: "padding: 8px 16px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 6px; color: {c.text_secondary}; cursor: pointer; font-size: 12px; margin-bottom: 24px;",
                    onclick: move |_| { state.write().selected_service_id = None; },
                    "\u{2190} Retour"
                }
                p { style: "color: {c.text_muted};", "Service introuvable." }
            }
        }
    }
}

/// Ligne d'information dans la vue détaillée.
#[component]
fn InfoRow(label: &'static str, value: String, icon: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 14px; display: flex; align-items: center; gap: 12px;",
            span { style: "font-size: 20px;", "{icon}" }
            div {
                p { style: "font-size: 11px; color: {c.text_muted}; margin-bottom: 2px;", "{label}" }
                p { style: "font-size: 14px; color: {c.text_white}; font-weight: 500;", "{value}" }
            }
        }
    }
}
