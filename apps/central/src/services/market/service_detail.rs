//! Vue détaillée d'un service dans le Market.

use dioxus::prelude::*;
use crate::state::{use_app_state, use_service_manager, ServiceRegistry};
use crate::market_client::MarketClient;
use super::{MarketState, InstallStatus};

#[component]
pub fn ServiceDetail(state: Signal<MarketState>) -> Element {
    let app = use_app_state();
    let manager = use_service_manager();
    let c = app.read().current_theme.palette();

    let service_id = state.read().selected_service_id.clone().unwrap_or_default();

    // Chercher dans la liste complète (installés + catalogue)
    let all_services = ServiceRegistry::installed_services(&manager);
    let service = all_services.iter()
        .find(|s| s.id == service_id)
        .cloned();

    if let Some(svc) = service {
        let is_installed = svc.is_installed;
        let is_running = manager.is_running(&service_id);
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

        let icon_bg = format!("width: 100px; height: 100px; background: linear-gradient(135deg, {type_color}30, {type_color}10); border-radius: 16px; display: flex; align-items: center; justify-content: center; flex-shrink: 0;");
        let type_badge_style = format!("display: flex; align-items: center; gap: 4px; padding: 4px 10px; background: {type_color}15; border: 1px solid {type_color}30; border-radius: 6px; font-size: 12px; color: {type_color};");
        let type_dot_style = format!("width: 6px; height: 6px; border-radius: 50%; background: {type_color};");
        let source_badge_style = format!("padding: 4px 10px; background: {source_color}15; border: 1px solid {source_color}30; border-radius: 6px; font-size: 12px; color: {source_color};");
        let version_text = format!("v{svc_version}");

        let statut_label = if is_running {
            "En cours"
        } else if is_installed {
            "Install\u{e9}"
        } else {
            "Disponible"
        };
        let statut_icon = if is_running {
            "\u{1f7e2}" // green circle
        } else if is_installed {
            "\u{2705}"
        } else {
            "\u{1f4e5}"
        };

        let type_label_str = type_label.to_string();

        // Vérifier si une mise à jour est disponible
        let update_info = state.read().available_updates.iter()
            .find(|(id, _, _)| id == &svc_id)
            .cloned();
        let has_update = update_info.is_some();

        let sid_launch = svc_id.clone();
        let sid_stop = svc_id.clone();
        let sid_uninstall = svc_id.clone();
        let sid_update = svc_id.clone();
        let manager_launch = manager.clone();
        let manager_stop = manager.clone();
        let manager_uninstall = manager.clone();
        let manager_update = manager.clone();

        rsx! {
            div {
                style: "display: flex; flex-direction: column; gap: 24px; max-width: 800px;",

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
                    style: "display: flex; gap: 12px; flex-wrap: wrap;",
                    if is_installed {
                        if is_running {
                            // Service en cours d'exécution
                            button {
                                style: "padding: 12px 32px; background: #ef4444; color: white; border: none; border-radius: 8px; font-size: 14px; font-weight: 600; cursor: pointer;",
                                onclick: move |_| {
                                    if let Err(e) = manager_stop.stop_service(&sid_stop) {
                                        tracing::error!("Arr\u{ea}t \u{e9}chou\u{e9}: {e}");
                                    }
                                },
                                "Arr\u{ea}ter"
                            }
                        } else {
                            // Service installé mais arrêté
                            button {
                                style: "padding: 12px 32px; background: {c.accent_blue}; color: white; border: none; border-radius: 8px; font-size: 14px; font-weight: 600; cursor: pointer;",
                                onclick: {
                                    let app_state = app.clone();
                                    move |_| {
                                        let profile_id = app_state.read().current_user
                                            .as_ref()
                                            .map(|u| u.id.clone())
                                            .unwrap_or_default();
                                        if let Err(e) = manager_launch.launch_service(&sid_launch, &profile_id) {
                                            tracing::error!("Lancement \u{e9}chou\u{e9}: {e}");
                                        }
                                    }
                                },
                                "Lancer"
                            }
                            button {
                                style: "padding: 12px 24px; background: transparent; color: {c.accent_red}; border: 1px solid {c.accent_red}40; border-radius: 8px; font-size: 14px; cursor: pointer;",
                                onclick: move |_| {
                                    match manager_uninstall.uninstall_service(&sid_uninstall) {
                                        Ok(()) => {
                                            tracing::info!("Service d\u{e9}sinstall\u{e9}: {}", sid_uninstall);
                                            state.write().selected_service_id = None;
                                        }
                                        Err(e) => tracing::error!("D\u{e9}sinstallation \u{e9}chou\u{e9}e: {e}"),
                                    }
                                },
                                "D\u{e9}sinstaller"
                            }
                        }

                        // Bouton « Mettre à jour » si une nouvelle version est disponible
                        if has_update {
                            {
                                let is_updating = state.read().install_status.as_ref()
                                    .map_or(false, |(id, st)| id == &svc_id && matches!(st, InstallStatus::Downloading | InstallStatus::Installing));
                                let update_done = state.read().install_status.as_ref()
                                    .map_or(false, |(id, st)| id == &svc_id && matches!(st, InstallStatus::Done));
                                let update_error = state.read().install_status.as_ref()
                                    .and_then(|(id, st)| if id == &svc_id { if let InstallStatus::Error(e) = st { Some(e.clone()) } else { None } } else { None });
                                let update_label = if is_updating {
                                    state.read().install_status.as_ref()
                                        .map(|(_, st)| match st {
                                            InstallStatus::Downloading => "T\u{e9}l\u{e9}chargement...",
                                            InstallStatus::Installing => "Installation...",
                                            _ => "Mettre \u{e0} jour",
                                        })
                                        .unwrap_or("Mettre \u{e0} jour")
                                } else if update_done {
                                    "\u{e0} jour !"
                                } else {
                                    "Mettre \u{e0} jour"
                                };
                                let new_version = update_info.as_ref().map(|(_, _, v)| v.clone()).unwrap_or_default();
                                let update_btn_bg = if is_updating || update_done {
                                    c.bg_hover.to_string()
                                } else {
                                    "#f59e0b".to_string()
                                };
                                let sid_u = sid_update.clone();
                                let mgr_u = manager_update.clone();

                                rsx! {
                                    div {
                                        style: "display: flex; flex-direction: column; gap: 6px;",
                                        button {
                                            style: "padding: 10px 24px; background: {update_btn_bg}; color: white; border: none; border-radius: 8px; font-size: 13px; font-weight: 600; cursor: pointer; display: flex; align-items: center; gap: 8px;",
                                            disabled: is_updating || update_done,
                                            onclick: move |_| {
                                                let sid = sid_u.clone();
                                                let new_ver = new_version.clone();
                                                let mgr = mgr_u.clone();
                                                let mut st = state;
                                                st.write().install_status = Some((sid.clone(), InstallStatus::Downloading));

                                                spawn(async move {
                                                    let origin_url = std::env::var("MIYUKINI_ORIGIN_URL")
                                                        .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
                                                    let client = MarketClient::new(&origin_url);

                                                    match client.download_package(&sid, &new_ver).await {
                                                        Ok(bytes) => {
                                                            st.write().install_status = Some((sid.clone(), InstallStatus::Installing));
                                                            match mgr.update_from_package(&sid, &bytes) {
                                                                Ok(_) => {
                                                                    tracing::info!("Service mis \u{e0} jour: {sid} -> v{new_ver}");
                                                                    st.write().install_status = Some((sid.clone(), InstallStatus::Done));
                                                                    // Retirer de la liste des mises à jour disponibles
                                                                    st.write().available_updates.retain(|(id, _, _)| id != &sid);
                                                                }
                                                                Err(e) => {
                                                                    tracing::error!("Mise \u{e0} jour \u{e9}chou\u{e9}e: {e}");
                                                                    st.write().install_status = Some((sid.clone(), InstallStatus::Error(e)));
                                                                }
                                                            }
                                                        }
                                                        Err(e) => {
                                                            tracing::error!("T\u{e9}l\u{e9}chargement mise \u{e0} jour \u{e9}chou\u{e9}: {e}");
                                                            st.write().install_status = Some((sid.clone(), InstallStatus::Error(e.to_string())));
                                                        }
                                                    }
                                                });
                                            },
                                            "\u{2B06}\u{FE0F} {update_label}"
                                        }
                                        if let Some(ref info) = update_info {
                                            p {
                                                style: "font-size: 11px; color: #f59e0b;",
                                                "v{info.1} \u{2192} v{info.2}"
                                            }
                                        }
                                        if let Some(err) = update_error {
                                            p {
                                                style: "font-size: 12px; color: {c.accent_red};",
                                                "{err}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Service non installé — téléchargement + installation
                        {
                            let is_installing = state.read().install_status.as_ref()
                                .map_or(false, |(id, st)| id == &svc_id && matches!(st, InstallStatus::Downloading | InstallStatus::Installing));
                            let install_error = state.read().install_status.as_ref()
                                .and_then(|(id, st)| if id == &svc_id { if let InstallStatus::Error(e) = st { Some(e.clone()) } else { None } } else { None });
                            let install_done = state.read().install_status.as_ref()
                                .map_or(false, |(id, st)| id == &svc_id && matches!(st, InstallStatus::Done));
                            let install_label = if is_installing {
                                let status_label = state.read().install_status.as_ref()
                                    .map(|(_, st)| match st {
                                        InstallStatus::Downloading => "Telechargement...",
                                        InstallStatus::Installing => "Installation...",
                                        _ => "Installer",
                                    })
                                    .unwrap_or("Installer");
                                status_label
                            } else if install_done {
                                "Installe !"
                            } else {
                                "Installer"
                            };
                            let btn_bg = if is_installing || install_done {
                                c.bg_hover.to_string()
                            } else {
                                c.accent_green.to_string()
                            };
                            let svc_id_install = svc_id.clone();
                            let svc_version_install = svc_version.clone();
                            let manager_install = manager.clone();

                            rsx! {
                                div {
                                    style: "display: flex; flex-direction: column; gap: 8px;",
                                    button {
                                        style: "padding: 12px 32px; background: {btn_bg}; color: white; border: none; border-radius: 8px; font-size: 14px; font-weight: 600; cursor: pointer;",
                                        disabled: is_installing || install_done,
                                        onclick: move |_| {
                                            let sid = svc_id_install.clone();
                                            let ver = svc_version_install.clone();
                                            let mgr = manager_install.clone();
                                            let mut st = state;
                                            // Marquer le début du téléchargement
                                            st.write().install_status = Some((sid.clone(), InstallStatus::Downloading));

                                            spawn(async move {
                                                let origin_url = std::env::var("MIYUKINI_ORIGIN_URL")
                                                    .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
                                                let client = MarketClient::new(&origin_url);

                                                match client.download_package(&sid, &ver).await {
                                                    Ok(bytes) => {
                                                        st.write().install_status = Some((sid.clone(), InstallStatus::Installing));
                                                        match mgr.install_from_package(&bytes) {
                                                            Ok(installed_id) => {
                                                                tracing::info!("Service installe: {installed_id}");
                                                                st.write().install_status = Some((sid.clone(), InstallStatus::Done));
                                                            }
                                                            Err(e) => {
                                                                tracing::error!("Installation echouee: {e}");
                                                                st.write().install_status = Some((sid.clone(), InstallStatus::Error(e)));
                                                            }
                                                        }
                                                    }
                                                    Err(e) => {
                                                        tracing::error!("Telechargement echoue: {e}");
                                                        st.write().install_status = Some((sid.clone(), InstallStatus::Error(e.to_string())));
                                                    }
                                                }
                                            });
                                        },
                                        "{install_label}"
                                    }
                                    if let Some(err) = install_error {
                                        p {
                                            style: "font-size: 12px; color: {c.accent_red};",
                                            "{err}"
                                        }
                                    }
                                }
                            }
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

                // Note architecture
                div {
                    style: "background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 8px; padding: 16px;",
                    div {
                        style: "display: flex; gap: 12px; align-items: flex-start;",
                        span { style: "font-size: 18px;", "\u{2139}\u{fe0f}" }
                        div {
                            p {
                                style: "font-size: 13px; color: {c.text_white}; font-weight: 500; margin-bottom: 4px;",
                                "Service ind\u{e9}pendant"
                            }
                            p {
                                style: "font-size: 12px; color: {c.text_secondary}; line-height: 1.5;",
                                "Chaque service est un programme ind\u{e9}pendant. Central le t\u{e9}l\u{e9}charge, l'installe et le lance dans sa propre fen\u{ea}tre. Les mises \u{e0} jour sont automatiques via le Market."
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
