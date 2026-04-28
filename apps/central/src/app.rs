//! Application principale Miyukini Central.

use std::sync::Arc;

use crate::components::{Header, TabBar};
use crate::data::ServiceConnections;
use crate::miou::bubble::MIOU_CSS;
use crate::miou::{
    decide, select_variante, templates::generate_bulle, ActionType, BotContext, BulleAction,
    BulleOutput, MiouBubbleOverlay,
};
use crate::remote::{self, RemoteState};
use crate::screens::{Connexion, ProfileWindow, RiteEntree};
use crate::service_manager::ServiceManager;
use crate::services::{auto_connect_after_login, ActiveServiceView, MwsNetworkView, MwsViewState};
use crate::state::{AppContext, AppState, MainTab};
use crate::theme::styles;
use dioxus::prelude::*;
use miyuki_ui_dioxus::context::provide_theme;
use miyuki_ui_tokens::COG_THEME;
use miyukini_central::CentralMwsState;

/// Point d'entrée de l'application.
#[component]
pub fn App() -> Element {
    // Initialiser le Service Manager (charge le registre des services installés)
    let service_manager = ServiceManager::init();

    // Fournir le theme miyuki-ui-dioxus à tous les composants enfants.
    provide_theme(COG_THEME.clone());

    use_context_provider(|| {
        let base_path = std::env::current_dir().unwrap_or_default();
        let connections =
            ServiceConnections::open(&base_path).expect("Impossible d'ouvrir la base auth");
        let connections = Arc::new(connections);
        let auth_db = &*connections.auth_db;
        let is_cog_virgin = auth_db.is_cog_virgin().unwrap_or(true);
        let last_profile = auth_db
            .get_current_profile_id()
            .ok()
            .flatten()
            .and_then(|id| auth_db.get_profile(&id).ok().flatten());
        let (last_login_email, last_login_pseudo) = last_profile
            .as_ref()
            .map(|p| (p.email.clone(), p.pseudonyme.clone().unwrap_or_default()))
            .unwrap_or_default();
        let state = AppState {
            is_cog_virgin,
            last_login_email,
            last_login_pseudo,
            ..AppState::new(&service_manager)
        };
        AppContext {
            connections: Signal::new(connections),
            state: Signal::new(state),
            service_manager: service_manager.clone(),
            remote_state: Signal::new(RemoteState::default()),
        }
    });

    let ctx = use_context::<AppContext>();
    let mut mws_state = use_context_provider(|| Signal::new(MwsViewState::default()));
    let state = ctx.state;
    let is_cog_virgin = state.read().is_cog_virgin;
    let has_user = state.read().current_user.is_some();
    let theme = state.read().current_theme;
    let c = theme.palette();

    // Signal pour la bulle Miou actuelle
    let mut current_bubble: Signal<Option<BulleOutput>> = use_signal(|| None);

    // Trigger de la première bulle Miou après connexion (délai 2-3s)
    let ctx_for_effect = ctx.clone();
    use_effect(move || {
        let mut ctx = ctx_for_effect.clone();
        let state_read = ctx.state.read();

        if state_read.current_user.is_some()
            && !state_read.is_cog_virgin
            && !state_read.rite_infos_pending
            && !state_read.miou_first_trigger_done
        {
            let pseudo = state_read
                .current_user
                .as_ref()
                .and_then(|u| u.pseudonyme.clone())
                .unwrap_or_else(|| "habitant".to_string());
            let prefs = state_read.miou_prefs.clone();
            let miou_state = state_read.miou_state.clone();

            // Plus de Jay1Tribu intégré — les données sociales viendront via IPC à terme
            let ami_connecte_recemment: Option<String> = None;
            drop(state_read);
            let ami_plus_delaisse: Option<(String, u32)> = None;

            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(2500)).await;

                let context = BotContext {
                    pseudo,
                    is_first_connection_of_session: true,
                    session_duration_minutes: 0,
                    bulles_actives: prefs.bulles_actives,
                    dnd_actif: prefs.dnd_actif,
                    max_bulles_par_session: prefs.frequence.max_bulles(),
                    bulles_deja_affichees: miou_state.bulles_count_this_session,
                    seuil_pause_minutes: prefs.seuil_pause_minutes,
                    rappels_pause_actives: prefs.rappels_pause_actives,
                    ami_connecte_recemment,
                    ami_plus_delaisse,
                    ..BotContext::default()
                };

                let decision = decide(&context, &miou_state);

                if let Some(categorie) = decision.categorie {
                    if let Some(template) = select_variante(categorie, &miou_state.variantes_used) {
                        let bulle = generate_bulle(&template, &context);
                        current_bubble.set(Some(bulle.clone()));

                        ctx.state.write().miou_first_trigger_done = true;
                        ctx.state
                            .write()
                            .miou_state
                            .record_bulle_shown(categorie, &template.id);
                    }
                }
            });
        }
    });

    let ctx_for_mws = ctx.clone();
    use_effect(move || {
        let current_user = ctx_for_mws.state.read().current_user.clone();
        let Some(profile) = current_user else {
            mws_state.write().boot_user_id = None;
            return;
        };

        let should_boot = {
            let mws = mws_state.read();
            mws.config.should_connect()
                && mws.config.auto_connect
                && mws.boot_user_id.as_deref() != Some(profile.id.as_str())
                && !matches!(
                    mws.state,
                    CentralMwsState::Connected
                        | CentralMwsState::Connecting
                        | CentralMwsState::RelayConnected
                )
        };

        if !should_boot {
            return;
        }

        mws_state.write().boot_user_id = Some(profile.id.clone());
        let display_name = crate::data::profile_display_name(&profile);
        let connections = ctx_for_mws.connections.read().clone();
        spawn(async move {
            auto_connect_after_login(mws_state, connections, display_name).await;
        });
    });

    // === CentralRemote: sync state → bridge ===
    let ctx_for_remote = ctx.clone();
    use_effect(move || {
        let state_read = ctx_for_remote.state.read();
        let remote_read = ctx_for_remote.remote_state.read();
        if remote_read.enabled && state_read.current_user.is_some() {
            remote::sync_state_to_bridge(&state_read);
        }
    });

    // === CentralRemote: process incoming commands (one-shot task) ===
    let ctx_for_remote_cmds = ctx.clone();
    use_effect(move || {
        let remote_enabled = ctx_for_remote_cmds.remote_state.read().enabled;
        if !remote_enabled {
            return;
        }
        if let Some(mut rx) = remote::remote_bridge().take_command_receiver() {
            let mut ctx_cmd = ctx_for_remote_cmds.clone();
            spawn(async move {
                while let Some(cmd) = rx.recv().await {
                    remote::apply_remote_command(&mut ctx_cmd, cmd);
                }
            });
        }
    });

    // Handler pour dismiss de la bulle
    let mut ctx_for_dismiss = ctx.clone();
    let on_dismiss = move |_| {
        {
            let bubble = current_bubble.read();
            if let Some(b) = bubble.as_ref() {
                if b.categorie.contains("pause") {
                    ctx_for_dismiss
                        .state
                        .write()
                        .miou_state
                        .record_pause_dismissed();
                }
            }
        }
        current_bubble.set(None);
    };

    // Handler pour actions de la bulle
    let mut ctx_for_action = ctx.clone();
    let on_action = move |action: BulleAction| match action.action_type {
        ActionType::Dismiss => {
            current_bubble.set(None);
        }
        ActionType::Pause => {
            ctx_for_action
                .state
                .write()
                .miou_state
                .record_pause_dismissed();
            current_bubble.set(None);
        }
        ActionType::OuvrirService => {
            if let Some(service_id) = action.payload {
                let services = ctx_for_action.state.read().services.clone();
                if let Some(service) = services.iter().find(|s| s.id == service_id) {
                    ctx_for_action.state.write().open_service(service);
                }
            }
            current_bubble.set(None);
        }
        ActionType::Custom => {
            current_bubble.set(None);
        }
    };

    rsx! {
        div {
            style: "{styles::main_container(theme)}",

            style { {GLOBAL_CSS} }
            style { {MIOU_CSS} }

            if is_cog_virgin || state.read().rite_infos_pending {
                RiteEntree {}
            } else if !has_user {
                Connexion {}
            } else {
                Header {}
                main {
                    style: "{styles::content_area(theme)}",
                    role: "main",

                    if matches!(state.read().main_tab, MainTab::Salon | MainTab::Bibliotheque) {
                        TabBar {}
                    }

                    div {
                        style: "{styles::content_panel(theme)}",

                        div {
                            style: "flex: 1; min-height: 0; display: flex; flex-direction: column; overflow-y: auto;",

                            {
                                let main_tab = state.read().main_tab;
                                match main_tab {
                                    MainTab::Salon | MainTab::Bibliotheque => rsx! { ActiveServiceView {} },
                                    MainTab::Communaute => rsx! { MwsNetworkView {} },
                                    MainTab::MesAmis => rsx! {
                                        div { style: "padding: 32px; color: #8f98a0;",
                                            "Service Jay1Tribu — installez-le depuis le Market."
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
                footer {
                    style: "display: flex; align-items: center; justify-content: space-between; height: 24px; background: {c.bg_header}; padding: 0 16px; font-size: 11px; color: {c.text_muted}; border-top: 1px solid {c.border};",
                    span { "Miyukini Central v0.2.0" }
                    span {
                        {
                            let remote_enabled = ctx.remote_state.read().enabled;
                            let remote_addr = ctx.remote_state.read().server_addr.clone();
                            let base = "COG: Actif \u{2022} KindMother: Connect\u{e9}";
                            if remote_enabled {
                                format!("{base} \u{2022} Remote: {}", remote_addr.as_deref().unwrap_or("D\u{e9}marrage…"))
                            } else {
                                base.to_string()
                            }
                        }
                    }
                }
                if state.read().show_profile_window {
                    ProfileWindow {}
                }

                MiouBubbleOverlay {
                    bubble: current_bubble,
                    on_dismiss: on_dismiss,
                    on_action: on_action,
                }
            }
        }
    }
}

/// CSS global injecté dans la page.
#[allow(clippy::needless_raw_string_hashes)]
const GLOBAL_CSS: &str = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, 'Roboto', sans-serif;
    -webkit-font-smoothing: antialiased;
    overflow: hidden;
}

::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

::-webkit-scrollbar-track {
    background: #1b2838;
}

::-webkit-scrollbar-thumb {
    background: #3d4f5f;
    border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
    background: #4d6070;
}

*:focus {
    outline: none;
}

*:focus-visible {
    outline: 2px solid #1a9fff;
    outline-offset: 2px;
}

button {
    font-family: inherit;
}

button, a, div {
    transition: background-color 0.15s ease, color 0.15s ease, border-color 0.15s ease, transform 0.15s ease;
}

button:hover:not(:disabled) {
    filter: brightness(1.1);
}

button:active:not(:disabled) {
    transform: scale(0.98);
}

::selection {
    background: #1a9fff;
    color: white;
}
"#;
