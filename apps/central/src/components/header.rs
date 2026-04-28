//! Header principal Steam-like avec navigation par onglets.

use crate::data::profile_display_name;
use crate::remote;
use crate::state::{use_app_state, MainTab, OpenTab};
use crate::theme::styles;
use central_remote::server::RemoteServerConfig;
use dioxus::prelude::*;

/// Header principal de l'application.
#[component]
pub fn Header() -> Element {
    let mut state = use_app_state();
    let ctx = use_context::<crate::state::AppContext>();
    let mut remote_state = ctx.remote_state;
    let theme = state.read().current_theme;
    let c = theme.palette();

    let display_name = state
        .read()
        .current_user
        .as_ref()
        .map_or_else(|| "Profil".to_string(), profile_display_name);
    let initial = display_name
        .chars()
        .next()
        .map_or('?', |c| c.to_uppercase().next().unwrap_or(c));

    rsx! {
        header {
            style: "{styles::header(theme)}",

            div {
                style: "display: flex; align-items: center; gap: 16px;",

                div {
                    style: "display: flex; align-items: center; gap: 8px; padding-right: 16px;",
                    span { style: "font-size: 20px;", "🌸" }
                }

                nav {
                    style: "display: flex; align-items: center; gap: 4px;",
                    for tab in MainTab::all() {
                        button {
                            style: "{styles::nav_tab(theme, state.read().main_tab == *tab)}",
                            onclick: move |_| {
                                let mut s = state.write();
                                s.main_tab = *tab;
                                match *tab {
                                    // "SALON" ramène à l'onglet Salon (Home, index 0)
                                    MainTab::Salon => {
                                        s.active_tab_index = 0;
                                    }
                                    // "SERVICES" ouvre directement le service Services (market)
                                    MainTab::Bibliotheque => {
                                        if let Some(idx) = s.open_tabs.iter().position(|t| t.service_id.as_deref() == Some("market")) {
                                            s.active_tab_index = idx;
                                        } else {
                                            let market_tab = OpenTab {
                                                id: "market".into(),
                                                title: "Services".into(),
                                                service_id: Some("market".into()),
                                                closable: true,
                                            };
                                            s.open_tabs.push(market_tab);
                                            s.active_tab_index = s.open_tabs.len() - 1;
                                        }
                                    }
                                    _ => {}
                                }
                            },
                            onmouseenter: move |_| {},
                            "{tab.label()}"
                        }
                    }
                }
            }

            div {
                style: "display: flex; align-items: center; gap: 16px;",

                div {
                    style: "position: relative;",
                    input {
                        style: "{styles::search_input(theme)}",
                        r#type: "text",
                        placeholder: "Rechercher dans le magasin",
                        value: "{state.read().search_query}",
                        oninput: move |evt| { state.write().search_query = evt.value(); }
                    }
                    span {
                        style: "position: absolute; right: 8px; top: 50%; transform: translateY(-50%); color: {c.text_secondary};",
                        "🔍"
                    }
                }

                button {
                    style: "background: transparent; border: none; color: {c.text_primary}; cursor: pointer; font-size: 16px; padding: 4px;",
                    "🔔"
                }

                // CentralRemote toggle
                {
                    let remote_enabled = remote_state.read().enabled;
                    let remote_addr = remote_state.read().server_addr.clone();
                    let remote_clients = remote_state.read().connected_clients;
                    let btn_color = if remote_enabled { "#4ade80" } else { c.text_secondary };
                    let title = if remote_enabled {
                        format!("Remote actif ({}){}", remote_addr.as_deref().unwrap_or(""), if remote_clients > 0 { format!(" — {} client(s)", remote_clients) } else { String::new() })
                    } else {
                        "Activer le remote".to_string()
                    };
                    rsx! {
                        button {
                            style: "background: transparent; border: none; color: {btn_color}; cursor: pointer; font-size: 16px; padding: 4px;",
                            title: "{title}",
                            onclick: move |_| {
                                let currently_enabled = remote_state.read().enabled;
                                if currently_enabled {
                                    // Désactiver le remote
                                    remote_state.write().enabled = false;
                                    remote_state.write().server_addr = None;
                                    tracing::info!("CentralRemote désactivé");
                                } else {
                                    // Activer le remote
                                    let mut rs = remote_state.write();
                                    rs.enabled = true;
                                    drop(rs);

                                    let config = RemoteServerConfig {
                                        listen_addr: std::env::var("CENTRAL_REMOTE_LISTEN")
                                            .unwrap_or_else(|_| "127.0.0.1:8091".into()),
                                        auth_db_path: std::env::current_dir()
                                            .unwrap_or_default()
                                            .join("central.db")
                                            .to_string_lossy()
                                            .to_string(),
                                    };

                                    // Sync l'état initial vers le bridge avant de démarrer le serveur
                                    remote::sync_state_to_bridge(&state.read());

                                    spawn(async move {
                                        match remote::start_remote(config).await {
                                            Ok(handle) => {
                                                let addr = handle.addr_string();
                                                tracing::info!("CentralRemote démarré sur {addr}");
                                                remote_state.write().server_addr = Some(addr);
                                                // Le handle est gardé en vie par le spawn — le serveur
                                                // tourne tant que Central tourne.
                                                std::mem::forget(handle);
                                            }
                                            Err(e) => {
                                                tracing::error!("CentralRemote échec: {e}");
                                                remote_state.write().enabled = false;
                                            }
                                        }
                                    });
                                }
                            },
                            "📡"
                        }
                    }
                }

                button {
                    style: "background: transparent; border: none; {styles::user_profile(theme)}",
                    onclick: move |_| { state.write().show_profile_window = true; },
                    div { style: "{styles::avatar(theme)}", "{initial}" }
                    span { style: "color: {c.text_primary}; font-size: 13px;", "{display_name}" }
                    span { style: "color: {c.text_secondary}; font-size: 10px;", "▼" }
                }
            }
        }
    }
}
