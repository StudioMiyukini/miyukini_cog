//! Vue MWS — Interface réseau Miyukini Webway System.
//!
//! Affiche l'état de connexion au réseau MWS et permet de :
//! - Se connecter / déconnecter du réseau
//! - Voir les étapes de conformité (Origin → Relay → Tracker)
//! - Rechercher des COGs et lobbys
//! - Créer et rejoindre des lobbys
//! - Activer/désactiver le mode Lone (COG isolé)

use super::mws_settings::{
    apply_origin_url, default_mws_config, load_mws_config, origin_url_from_env, save_mws_config,
};
use crate::data::{profile_display_name, use_service_connections};
use crate::state::AppContext;
use dioxus::prelude::*;
#[cfg(feature = "service-jay1tribu")]
use jay1tribu::{
    set_mws_transport_sender, set_webway_connected, DispatchError, MwsTransportSender,
};
use miyukini_central::{
    CentralMwsConfig, CentralMwsManager, CentralMwsState, MwsConformityState, MwsStatusSummary,
};
use miyuwebway_participant::GovernedContext;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::RwLock as TokioRwLock;

/// Sender MWS pour Jay1Tribu : délègue à miyuwebway_participant::transport::send.
/// Le contexte est mis à jour à la connexion / déconnexion MWS.
pub(crate) struct CentralJay1TribuSender {
    ctx: RwLock<Option<GovernedContext>>,
}

impl CentralJay1TribuSender {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            ctx: RwLock::new(None),
        })
    }

    fn set_context(&self, ctx: Option<GovernedContext>) {
        if let Ok(mut g) = self.ctx.write() {
            *g = ctx;
        }
    }
}

#[cfg(feature = "service-jay1tribu")]
impl MwsTransportSender for CentralJay1TribuSender {
    fn send(&self, to: &str, payload: &[u8]) -> Result<(), DispatchError> {
        let guard = self
            .ctx
            .read()
            .map_err(|e| DispatchError::Transport(e.to_string()))?;
        let ctx = match guard.as_ref() {
            Some(c) => c.clone(),
            None => return Err(DispatchError::Transport("MWS non connecté".into())),
        };
        miyuwebway_participant::transport::send(&ctx, to, payload)
            .map_err(|e| DispatchError::Transport(e.to_string()))
    }
}

/// État du composant MWS.
#[derive(Clone)]
pub struct MwsViewState {
    /// Gestionnaire MWS (connexion réelle au réseau).
    pub manager: Arc<TokioRwLock<Option<CentralMwsManager>>>,
    pub config: CentralMwsConfig,
    /// État simplifié.
    pub state: CentralMwsState,
    /// État de conformité détaillé.
    pub conformity: MwsConformityState,
    /// Mode Lone actif.
    pub is_lone: bool,
    /// COGs découverts.
    pub discovered_cogs: Vec<DiscoveredCog>,
    /// Lobbys découverts.
    pub discovered_lobbys: Vec<DiscoveredLobby>,
    /// Recherche en cours.
    pub search_query: String,
    /// En cours de connexion.
    pub connecting: bool,
    /// Erreur éventuelle.
    pub error: Option<String>,
    pub boot_user_id: Option<String>,
    /// Sender MWS pour Jay1Tribu (envoi de messages via le Webway).
    pub jay1tribu_sender: Arc<CentralJay1TribuSender>,
}

impl Default for MwsViewState {
    fn default() -> Self {
        let config = load_mws_config();
        let (state, conformity) = initial_visual_state(&config);
        let jay1tribu_sender = CentralJay1TribuSender::new();
        #[cfg(feature = "service-jay1tribu")]
        set_mws_transport_sender(jay1tribu_sender.clone());
        Self {
            manager: Arc::new(TokioRwLock::new(None)),
            config: config.clone(),
            state,
            conformity,
            is_lone: config.lone_mode,
            discovered_cogs: Vec::new(),
            discovered_lobbys: Vec::new(),
            search_query: String::new(),
            connecting: false,
            error: None,
            boot_user_id: None,
            jay1tribu_sender,
        }
    }
}

fn initial_visual_state(config: &CentralMwsConfig) -> (CentralMwsState, MwsConformityState) {
    if config.lone_mode {
        (CentralMwsState::Lone, MwsConformityState::LoneMode)
    } else if config.enabled {
        (
            CentralMwsState::Disconnected,
            MwsConformityState::Uninitialized,
        )
    } else {
        (CentralMwsState::Disabled, MwsConformityState::Uninitialized)
    }
}

/// COG découvert sur le réseau.
#[derive(Clone, Debug, PartialEq)]
pub struct DiscoveredCog {
    pub cog_id: String,
    pub core_version: String,
    pub address: String,
    pub services: Vec<String>,
    pub last_seen: String,
}

/// Lobby découvert sur le réseau.
#[derive(Clone, Debug, PartialEq)]
pub struct DiscoveredLobby {
    pub lobby_id: String,
    pub name: String,
    pub host_cog_id: String,
    pub pool_version: String,
    pub current_players: u32,
    pub max_players: u32,
    pub is_public: bool,
    pub password_required: bool,
}

fn persist_config(state: &Signal<MwsViewState>) {
    let config = state.read().config.clone();
    if let Err(err) = save_mws_config(&config) {
        tracing::warn!("Sauvegarde config MWS impossible: {}", err);
    }
}

fn current_cog_id(display_name: String) -> String {
    if display_name.trim().is_empty() {
        "central-native".to_string()
    } else {
        display_name
    }
}

async fn load_discovery_snapshot(
    manager: &CentralMwsManager,
) -> (Vec<DiscoveredCog>, Vec<DiscoveredLobby>) {
    let cogs = manager
        .search_cogs(None, None, 50)
        .await
        .unwrap_or_default();
    let lobbys = manager
        .search_lobbys(None, true, 50)
        .await
        .unwrap_or_default();

    let discovered_cogs = cogs
        .into_iter()
        .map(|c| DiscoveredCog {
            cog_id: c.cog_id,
            core_version: c.core_version,
            address: c.address,
            services: c.services,
            last_seen: c.last_seen,
        })
        .collect();
    let discovered_lobbys = lobbys
        .into_iter()
        .map(|r| DiscoveredLobby {
            lobby_id: r.lobby.lobby_id,
            name: r.lobby.name,
            host_cog_id: r.host_cog_id,
            pool_version: r.pool_version,
            current_players: r.lobby.current_players,
            max_players: r.lobby.max_players,
            is_public: r.lobby.is_public,
            password_required: r.lobby.password_required,
        })
        .collect();

    (discovered_cogs, discovered_lobbys)
}

/// Vue principale du réseau MWS.
/// L'état MWS est fourni par le contexte App pour persister au changement d'onglet.
#[component]
pub fn MwsNetworkView() -> Element {
    let mws_state = use_context::<Signal<MwsViewState>>();
    // Compteur de rafraîchissement : chaque incrément force un re-render du composant.
    let tick = use_signal(|| 0u64);
    let conns = use_service_connections();

    // Lire tick pour s'y abonner — toute écriture déclenche un re-render.
    let _tick_val = *tick.read();

    // Polling de l'état MWS via le manager (toutes les 2s).
    // On passe par `tick` pour forcer le re-render quand l'état change.
    use_future(move || {
        let mut state = mws_state;
        let mut tick = tick;
        async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

                let arc = state.read().manager.clone();
                let guard = arc.read().await;
                if let Some(ref manager) = *guard {
                    let st: CentralMwsState = manager.get_state().await;
                    let conf = manager.get_conformity_state().await;
                    drop(guard);

                    let current_state = state.read().state.clone();
                    let current_conf = state.read().conformity.clone();
                    if st != current_state || conf != current_conf {
                        tracing::info!(
                            "[MWS UI poll] état changé: {:?} → {:?}, conformité: {:?} → {:?}",
                            current_state,
                            st,
                            current_conf,
                            conf
                        );
                        let connected = st == CentralMwsState::Connected;
                        #[cfg(feature = "service-jay1tribu")]
                        set_webway_connected(connected);
                        let mut s = state.write();
                        s.jay1tribu_sender.set_context(if connected {
                            Some(GovernedContext::new("mws-jay1tribu".to_string(), 0))
                        } else {
                            None
                        });
                        s.state = st;
                        s.conformity = conf;
                        s.connecting = false;
                        drop(s);
                        let n = *tick.read();
                        tick.set(n + 1);
                    }
                }
            }
        }
    });

    rsx! {
        div {
            class: "mws-view",
            style: "padding: 24px; height: 100%; overflow-y: auto; background: #1a1a2e;",

            // En-tête
            div {
                class: "mws-header",
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px;",

                div {
                    h1 {
                        style: "margin: 0; font-size: 24px; color: #fff;",
                        "🌐 Réseau MWS"
                    }
                    p {
                        style: "margin: 4px 0 0; color: #9ca3af; font-size: 14px;",
                        "Miyukini Webway System — Réseau P2P souverain"
                    }
                }

                // Boutons de contrôle
                div {
                    style: "display: flex; gap: 12px; align-items: center;",

                    // Toggle Mode Lone
                    MwsLoneModeToggle { state: mws_state, tick }

                    // Bouton connexion/déconnexion
                    MwsConnectionButton { state: mws_state, conns, tick }
                }
            }

            MwsConfigCard { state: mws_state, tick }

            // État de connexion avec étapes de conformité
            MwsStatusCard { state: mws_state }

            // Protocole de conformité MWS — toujours visible pour refléter l'état (étape 0/10 → 10/10)
            MwsConformityProgress { state: mws_state }

            // Mode Lone actif
            if mws_state.read().is_lone {
                div {
                    class: "mws-lone-mode",
                    style: "
                        background: linear-gradient(135deg, #1a1a2e 0%, #2d1f3d 100%);
                        border: 1px solid #f59e0b;
                        border-radius: 12px;
                        padding: 24px;
                        margin-bottom: 24px;
                        text-align: center;
                    ",

                    div { style: "font-size: 48px; margin-bottom: 16px;", "🏝️" }
                    h3 { style: "margin: 0 0 8px; color: #f59e0b;", "Mode Lone Activé" }
                    p { style: "color: #9ca3af; margin: 0;",
                        "Votre COG fonctionne en mode isolé. "
                        "Les données restent locales et aucune connexion réseau n'est établie."
                    }
                    p { style: "color: #6b7280; margin-top: 12px; font-size: 12px;",
                        "Désactivez le mode Lone pour rejoindre le réseau MWS."
                    }
                }
            }

            // Recherche (si connecté)
            if mws_state.read().state == CentralMwsState::Connected {
                MwsSearchSection { state: mws_state }
            }

            // Liste des lobbys découverts
            if !mws_state.read().discovered_lobbys.is_empty() {
                MwsLobbysSection { state: mws_state }
            }

            // Liste des COGs découverts
            if !mws_state.read().discovered_cogs.is_empty() {
                MwsCogsSection { state: mws_state }
            }

            // Message si déconnecté (et pas en mode Lone)
            if mws_state.read().state == CentralMwsState::Disconnected && !mws_state.read().is_lone {
                div {
                    class: "mws-disconnected",
                    style: "text-align: center; padding: 48px; color: #9ca3af;",

                    div { style: "font-size: 48px; margin-bottom: 16px;", "🔌" }
                    h3 { style: "margin: 0 0 8px; color: #fff;", "Non connecté au réseau" }
                    p {
                        "Connectez-vous au réseau MWS pour découvrir d'autres COGs et rejoindre des sessions."
                    }
                    p { style: "margin-top: 16px; font-size: 12px; color: #6b7280;",
                        "Même sans services exposés, votre COG se présentera sur le réseau pour établir sa présence."
                    }
                }
            }
        }
    }
}

/// Toggle pour le mode Lone.
#[component]
fn MwsLoneModeToggle(mut state: Signal<MwsViewState>, mut tick: Signal<u64>) -> Element {
    let is_lone = state.read().is_lone;

    let onclick = move |_| {
        let mut state = state;
        let mut tick = tick;
        let will_be_lone = !state.read().is_lone;
        if will_be_lone {
            {
                let mut s = state.write();
                s.config.lone_mode = true;
                s.is_lone = true;
            }
            persist_config(&state);
            spawn(async move {
                real_mws_disconnect(state, tick).await;
            });
            return;
        } else {
            let mut s = state.write();
            s.config.lone_mode = false;
            s.is_lone = false;
            let (next_state, next_conformity) = initial_visual_state(&s.config);
            s.state = next_state;
            s.conformity = next_conformity;
        }
        persist_config(&state);
        {
            let n = *tick.read();
            tick.set(n + 1);
        }
    };

    let (bg_color, label) = if is_lone {
        ("#f59e0b", "🏝️ Lone")
    } else {
        ("#374151", "🌐 Réseau")
    };

    rsx! {
        button {
            style: "
                padding: 8px 16px;
                border: none;
                border-radius: 6px;
                background: {bg_color};
                color: white;
                font-size: 12px;
                font-weight: 500;
                cursor: pointer;
                transition: all 0.2s;
            ",
            onclick: onclick,
            "{label}"
        }
    }
}

#[component]
fn MwsConfigCard(mut state: Signal<MwsViewState>, mut tick: Signal<u64>) -> Element {
    let config = state.read().config.clone();
    let relay_initial = config.relay_address.clone();
    let tracker_initial = config.tracker_address.clone();
    let public_initial = config.public_address.clone();
    let home_initial = config.home_http_bind.clone().unwrap_or_default();
    let mut relay_input = use_signal(|| relay_initial.clone());
    let mut tracker_input = use_signal(|| tracker_initial.clone());
    let mut public_input = use_signal(|| public_initial.clone());
    let mut home_input = use_signal(|| home_initial.clone());
    let mut feedback = use_signal(|| String::new());
    let mut feedback_is_error = use_signal(|| false);
    let origin_url = origin_url_from_env();

    let auto_connect = state.read().config.auto_connect;
    let auto_reconnect = state.read().config.auto_reconnect;
    let enabled = state.read().config.enabled;
    let enabled_bg = if enabled { "#10b981" } else { "#374151" };
    let auto_connect_bg = if auto_connect { "#8b5cf6" } else { "#374151" };
    let auto_reconnect_bg = if auto_reconnect { "#06b6d4" } else { "#374151" };
    let feedback_color = if feedback_is_error() {
        "#fca5a5"
    } else {
        "#86efac"
    };

    let toggle_auto_connect = move |_| {
        let new_value = {
            let mut s = state.write();
            s.config.auto_connect = !s.config.auto_connect;
            s.config.auto_connect
        };
        persist_config(&state);
        feedback_is_error.set(false);
        feedback.set(if new_value {
            "Connexion automatique activée par défaut.".to_string()
        } else {
            "Connexion automatique désactivée.".to_string()
        });
        let n = *tick.read();
        tick.set(n + 1);
    };

    let toggle_auto_reconnect = move |_| {
        let new_value = {
            let mut s = state.write();
            s.config.auto_reconnect = !s.config.auto_reconnect;
            s.config.auto_reconnect
        };
        persist_config(&state);
        feedback_is_error.set(false);
        feedback.set(if new_value {
            "Reconnexion automatique activée.".to_string()
        } else {
            "Reconnexion automatique désactivée.".to_string()
        });
        let n = *tick.read();
        tick.set(n + 1);
    };

    let toggle_enabled = move |_| {
        let should_disconnect = {
            let mut s = state.write();
            s.config.enabled = !s.config.enabled;
            !s.config.enabled
                && matches!(
                    s.state,
                    CentralMwsState::Connected
                        | CentralMwsState::Connecting
                        | CentralMwsState::RelayConnected
                )
        };

        if should_disconnect {
            persist_config(&state);
            spawn(async move {
                real_mws_disconnect(state, tick).await;
            });
            feedback_is_error.set(false);
            feedback.set("MWS désactivé. La session réseau est arrêtée.".to_string());
            return;
        }

        {
            let mut s = state.write();
            let (next_state, next_conformity) = initial_visual_state(&s.config);
            s.state = next_state;
            s.conformity = next_conformity;
        }
        persist_config(&state);
        feedback_is_error.set(false);
        feedback.set(if state.read().config.enabled {
            "MWS activé.".to_string()
        } else {
            "MWS désactivé.".to_string()
        });
        let n = *tick.read();
        tick.set(n + 1);
    };

    let save_config = move |_| {
        let relay_address = relay_input().trim().to_string();
        let tracker_address = tracker_input().trim().to_string();
        if relay_address.is_empty() || tracker_address.is_empty() {
            feedback_is_error.set(true);
            feedback.set("Relay et Tracker doivent être renseignés.".to_string());
            return;
        }

        let defaults = default_mws_config();
        let public_address = {
            let value = public_input().trim().to_string();
            if value.is_empty() {
                defaults.public_address
            } else {
                value
            }
        };
        let home_http_bind = {
            let value = home_input().trim().to_string();
            if value.is_empty() {
                defaults.home_http_bind
            } else {
                Some(value)
            }
        };

        {
            let mut s = state.write();
            s.config.relay_address = relay_address;
            s.config.tracker_address = tracker_address;
            s.config.public_address = public_address;
            s.config.home_http_bind = home_http_bind;
        }
        persist_config(&state);
        feedback_is_error.set(false);
        feedback.set(
            "Configuration enregistrée. Reconnectez MWS pour appliquer un nouveau Relay ou Tracker."
                .to_string(),
        );
        let n = *tick.read();
        tick.set(n + 1);
    };

    let reset_config = move |_| {
        let defaults = default_mws_config();
        relay_input.set(defaults.relay_address.clone());
        tracker_input.set(defaults.tracker_address.clone());
        public_input.set(defaults.public_address.clone());
        home_input.set(defaults.home_http_bind.clone().unwrap_or_default());
        {
            let mut s = state.write();
            s.config = defaults.clone();
            s.is_lone = defaults.lone_mode;
            if !matches!(
                s.state,
                CentralMwsState::Connected
                    | CentralMwsState::Connecting
                    | CentralMwsState::RelayConnected
            ) {
                let (next_state, next_conformity) = initial_visual_state(&s.config);
                s.state = next_state;
                s.conformity = next_conformity;
            }
        }
        persist_config(&state);
        feedback_is_error.set(false);
        feedback.set("Configuration MWS réinitialisée.".to_string());
        let n = *tick.read();
        tick.set(n + 1);
    };

    rsx! {
        div {
            class: "mws-config-card",
            style: "
                background: #12121a;
                border: 1px solid rgba(139, 92, 246, 0.2);
                border-radius: 12px;
                padding: 20px;
                margin-bottom: 24px;
            ",

            div {
                style: "display: flex; justify-content: space-between; gap: 16px; align-items: flex-start; margin-bottom: 16px; flex-wrap: wrap;",
                div {
                    h3 { style: "margin: 0; font-size: 15px; color: #fff;", "Configuration MWS" }
                    p {
                        style: "margin: 6px 0 0; color: #9ca3af; font-size: 12px; line-height: 1.5;",
                        "Central peut se rebrancher automatiquement sur Origin. Le Relay est en TLS et le Tracker final doit être officiel."
                    }
                }
                if let Some(origin_url) = origin_url.clone() {
                    button {
                        style: "padding: 10px 14px; border: 1px solid rgba(6, 182, 212, 0.35); border-radius: 8px; background: rgba(6, 182, 212, 0.12); color: #67e8f9; cursor: pointer;",
                        onclick: move |_| {
                            let mut next = state.read().config.clone();
                            apply_origin_url(&mut next, &origin_url);
                            relay_input.set(next.relay_address.clone());
                            tracker_input.set(next.tracker_address.clone());
                            {
                                let mut s = state.write();
                                s.config = next;
                            }
                            persist_config(&state);
                            feedback_is_error.set(false);
                            feedback.set("Adresses Origin synchronisées depuis MIYUKINI_ORIGIN_URL.".to_string());
                            let n = *tick.read();
                            tick.set(n + 1);
                        },
                        "Synchroniser avec Origin"
                    }
                }
            }

            div {
                style: "display: flex; gap: 10px; flex-wrap: wrap; margin-bottom: 16px;",
                button {
                    style: "padding: 8px 12px; border-radius: 999px; border: none; cursor: pointer; background: {enabled_bg}; color: #fff;",
                    onclick: toggle_enabled,
                    if enabled { "MWS actif" } else { "MWS inactif" }
                }
                button {
                    style: "padding: 8px 12px; border-radius: 999px; border: none; cursor: pointer; background: {auto_connect_bg}; color: #fff;",
                    onclick: toggle_auto_connect,
                    if auto_connect { "Connexion auto: oui" } else { "Connexion auto: non" }
                }
                button {
                    style: "padding: 8px 12px; border-radius: 999px; border: none; cursor: pointer; background: {auto_reconnect_bg}; color: #fff;",
                    onclick: toggle_auto_reconnect,
                    if auto_reconnect { "Reconnexion auto: oui" } else { "Reconnexion auto: non" }
                }
            }

            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 14px;",

                div {
                    style: "display: flex; flex-direction: column; gap: 6px;",
                    label { style: "font-size: 11px; color: #9ca3af; text-transform: uppercase; letter-spacing: 0.08em;", "Relay Origin" }
                    input {
                        r#type: "text",
                        value: "{relay_input()}",
                        style: "padding: 10px 12px; background: #0f172a; border: 1px solid rgba(255,255,255,0.08); border-radius: 8px; color: #fff;",
                        oninput: move |evt| relay_input.set(evt.value()),
                    }
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 6px;",
                    label { style: "font-size: 11px; color: #9ca3af; text-transform: uppercase; letter-spacing: 0.08em;", "Tracker Origin" }
                    input {
                        r#type: "text",
                        value: "{tracker_input()}",
                        style: "padding: 10px 12px; background: #0f172a; border: 1px solid rgba(255,255,255,0.08); border-radius: 8px; color: #fff;",
                        oninput: move |evt| tracker_input.set(evt.value()),
                    }
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 6px;",
                    label { style: "font-size: 11px; color: #9ca3af; text-transform: uppercase; letter-spacing: 0.08em;", "Adresse publique" }
                    input {
                        r#type: "text",
                        value: "{public_input()}",
                        style: "padding: 10px 12px; background: #0f172a; border: 1px solid rgba(255,255,255,0.08); border-radius: 8px; color: #fff;",
                        oninput: move |evt| public_input.set(evt.value()),
                    }
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 6px;",
                    label { style: "font-size: 11px; color: #9ca3af; text-transform: uppercase; letter-spacing: 0.08em;", "Bind Home HTTP" }
                    input {
                        r#type: "text",
                        value: "{home_input()}",
                        style: "padding: 10px 12px; background: #0f172a; border: 1px solid rgba(255,255,255,0.08); border-radius: 8px; color: #fff;",
                        oninput: move |evt| home_input.set(evt.value()),
                    }
                }
            }

            div {
                style: "display: flex; justify-content: space-between; gap: 12px; align-items: center; margin-top: 16px; flex-wrap: wrap;",
                p {
                    style: "margin: 0; font-size: 12px; color: #6b7280;",
                    if let Some(origin_url) = origin_url {
                        "Origin détecté: {origin_url}"
                    } else {
                        "Origin non fourni par variable d'environnement. Les adresses saisies seront utilisées."
                    }
                }
                div {
                    style: "display: flex; gap: 10px; flex-wrap: wrap;",
                    button {
                        style: "padding: 10px 14px; border-radius: 8px; border: 1px solid rgba(255,255,255,0.08); background: #1f2937; color: #fff; cursor: pointer;",
                        onclick: reset_config,
                        "Réinitialiser"
                    }
                    button {
                        style: "padding: 10px 14px; border-radius: 8px; border: none; background: #8b5cf6; color: #fff; cursor: pointer;",
                        onclick: save_config,
                        "Sauvegarder"
                    }
                }
            }

            if !feedback().is_empty() {
                p {
                    style: "margin: 16px 0 0; font-size: 12px; color: {feedback_color};",
                    "{feedback()}"
                }
            }
        }
    }
}

/// Barre de progression de conformité.
#[component]
fn MwsConformityProgress(state: Signal<MwsViewState>) -> Element {
    let conformity = state.read().conformity.clone();
    let step = conformity.step_number();
    let total_steps = 10u8;
    let progress_pct = (f32::from(step) / f32::from(total_steps) * 100.0) as u32;

    // Étapes de conformité
    let steps = [
        ("Résolution Origin", 1),
        ("Connexion TLS", 2),
        ("Enregistrement Relay", 3),
        ("Accusé Relay", 4),
        ("Session Relay", 5),
        ("Obtention Permis", 6),
        ("Connexion Tracker", 7),
        ("Annonce Tracker", 8),
        ("Accusé Tracker", 9),
        ("Conformité ✓", 10),
    ];

    rsx! {
        div {
            class: "mws-conformity-progress",
            style: "
                background: #12121a;
                border: 1px solid rgba(139, 92, 246, 0.2);
                border-radius: 12px;
                padding: 20px;
                margin-bottom: 24px;
            ",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;",
                h3 { style: "margin: 0; font-size: 14px; color: #fff;", "Protocole de conformité MWS" }
                span { style: "font-size: 12px; color: #9ca3af;", "Étape {step}/{total_steps}" }
            }

            // Barre de progression
            div {
                style: "
                    height: 4px;
                    background: #374151;
                    border-radius: 2px;
                    margin-bottom: 16px;
                    overflow: hidden;
                ",
                div {
                    style: "
                        height: 100%;
                        width: {progress_pct}%;
                        background: linear-gradient(90deg, #8b5cf6, #06b6d4);
                        border-radius: 2px;
                        transition: width 0.3s ease;
                    ",
                }
            }

            // Liste des étapes
            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px;",

                for (name, step_num) in steps.iter() {
                    {
                        let is_done = step >= *step_num;
                        let is_current = step == *step_num;
                        let bg = if is_done {
                            "rgba(16, 185, 129, 0.2)"
                        } else if is_current {
                            "rgba(139, 92, 246, 0.3)"
                        } else {
                            "rgba(55, 65, 81, 0.5)"
                        };
                        let color = if is_done {
                            "#10b981"
                        } else if is_current {
                            "#8b5cf6"
                        } else {
                            "#6b7280"
                        };
                        let border = if is_current { "1px solid #8b5cf6" } else { "none" };

                        rsx! {
                            span {
                                key: "{step_num}",
                                style: "
                                    padding: 4px 10px;
                                    background: {bg};
                                    color: {color};
                                    border: {border};
                                    border-radius: 4px;
                                    font-size: 11px;
                                    white-space: nowrap;
                                ",
                                "{name}"
                            }
                        }
                    }
                }
            }

            // Description actuelle
            div {
                style: "margin-top: 12px; padding-top: 12px; border-top: 1px solid rgba(255,255,255,0.1);",
                p { style: "margin: 0; font-size: 12px; color: #9ca3af;",
                    "{conformity.description()}"
                }
            }
        }
    }
}

/// Bouton de connexion/déconnexion.
#[component]
fn MwsConnectionButton(
    mut state: Signal<MwsViewState>,
    conns: Signal<Arc<crate::data::ServiceConnections>>,
    tick: Signal<u64>,
) -> Element {
    let current_state = state.read().state.clone();
    let conformity = state.read().conformity.clone();
    let connecting = state.read().connecting;
    let is_lone = state.read().is_lone;

    let summary = MwsStatusSummary::from_state(&current_state, &conformity);

    // Récupérer le display_name de l'utilisateur pour l'identifiant COG sur le Webway
    let app_ctx = use_context::<AppContext>();
    let user_display_name = app_ctx
        .state
        .read()
        .current_user
        .as_ref()
        .map(|p| profile_display_name(p))
        .unwrap_or_else(|| "central-native".to_string());

    let onclick = move |_| {
        let state = state;
        let conns_arc = conns.read().clone();
        let tick = tick;
        let dn = user_display_name.clone();
        spawn(async move {
            let current = state.read().state.clone();
            let is_lone = state.read().is_lone;

            if is_lone {
                return;
            }

            if current == CentralMwsState::Connected || current == CentralMwsState::RelayConnected {
                real_mws_disconnect(state, tick).await;
            } else {
                real_mws_connect(state, conns_arc, tick, dn).await;
            }
        });
    };

    let (label, bg_color, disabled) = if is_lone {
        ("Mode Lone", "#374151", true)
    } else if connecting {
        ("Connexion...", "#f59e0b", true)
    } else if summary.can_disconnect {
        ("Déconnecter", "#ef4444", false)
    } else if summary.can_connect {
        ("Se connecter", "#10b981", false)
    } else {
        ("—", "#374151", true)
    };

    let opacity = if disabled { "0.6" } else { "1" };

    rsx! {
        button {
            style: "
                padding: 12px 24px;
                border: none;
                border-radius: 8px;
                background: {bg_color};
                color: white;
                font-weight: 600;
                cursor: pointer;
                transition: opacity 0.2s;
                opacity: {opacity};
            ",
            disabled: disabled,
            onclick: onclick,
            "{label}"
        }
    }
}

/// Connexion réelle au réseau MWS via CentralMwsManager et miyuwebway_participant.
/// Active le serveur Home (page de présentation du COG) lorsque annoncé sur le Tracker.
/// `tick` est incrémenté après mise à jour pour forcer le re-render de l'UI (spawn → Dioxus).
/// `display_name` est le pseudonyme ou l'email de l'utilisateur connecté — utilisé comme identifiant COG sur le Webway.
async fn connect_with_manager_state(
    mut state: Signal<MwsViewState>,
    _conns: Arc<crate::data::ServiceConnections>,
    display_name: String,
    honor_auto_connect: bool,
) -> Result<(), String> {
    tracing::info!(
        "[MWS UI] connect_with_manager_state start (cog_id={}, honor_auto_connect={})",
        &display_name,
        honor_auto_connect
    );
    {
        let mut s = state.write();
        s.connecting = true;
        s.error = None;
        s.state = CentralMwsState::Connecting;
    }

    let mut config = {
        let s = state.read();
        if s.config.relay_address.trim().is_empty() || s.config.tracker_address.trim().is_empty() {
            default_mws_config()
        } else {
            s.config.clone()
        }
    };

    if let Some(origin_url) = origin_url_from_env() {
        let uses_default_host = config.relay_address == "miyukini.com:7000"
            && config.tracker_address == "miyukini.com:21000";
        if uses_default_host {
            apply_origin_url(&mut config, &origin_url);
        }
    }

    let services: Vec<String> = Vec::new();

    let manager = CentralMwsManager::new(
        config.clone(),
        current_cog_id(display_name),
        "0.1.0".to_string(),
        services,
    );

    let result = if honor_auto_connect {
        manager.auto_connect_if_enabled().await
    } else {
        manager.connect().await
    };

    match result {
        Ok(()) => {
            let st = manager.get_state().await;
            let conf = manager.get_conformity_state().await;
            let (discovered_cogs, discovered_lobbys) = if st == CentralMwsState::Connected {
                load_discovery_snapshot(&manager).await
            } else {
                (Vec::new(), Vec::new())
            };
            let connected = st == CentralMwsState::Connected;
            #[cfg(feature = "service-jay1tribu")]
            set_webway_connected(connected);

            let mut s = state.write();
            s.manager = Arc::new(TokioRwLock::new(Some(manager)));
            s.config = config.clone();
            s.state = st;
            s.conformity = conf;
            s.is_lone = config.lone_mode;
            s.connecting = false;
            s.error = None;
            s.discovered_cogs = discovered_cogs;
            s.discovered_lobbys = discovered_lobbys;
            s.jay1tribu_sender.set_context(if connected {
                Some(GovernedContext::new("mws-jay1tribu".to_string(), 0))
            } else {
                None
            });
            Ok(())
        }
        Err(err) => {
            tracing::error!("[MWS UI] connect_with_manager_state error: {}", &err);
            let mut s = state.write();
            s.connecting = false;
            s.error = Some(err.clone());
            s.state = CentralMwsState::Error(err.clone());
            s.jay1tribu_sender.set_context(None);
            #[cfg(feature = "service-jay1tribu")]
            set_webway_connected(false);
            Err(err)
        }
    }
}

async fn disconnect_with_manager_state(mut state: Signal<MwsViewState>) {
    tracing::info!("[MWS UI] disconnect_with_manager_state start");

    {
        let mut s = state.write();
        s.connecting = true;
    }

    let arc = state.read().manager.clone();
    let mut guard = arc.write().await;
    if let Some(mgr) = guard.take() {
        tracing::info!("[MWS UI] Appel manager.disconnect() (WITHDRAW + arrêt)...");
        if let Err(err) = mgr.disconnect().await {
            tracing::warn!("[MWS UI] manager.disconnect() erreur: {}", err);
        }
    }
    drop(guard);

    let (new_state, new_conformity, is_lone) = {
        let s = state.read();
        let (state_value, conformity_value) = initial_visual_state(&s.config);
        (state_value, conformity_value, s.config.lone_mode)
    };

    {
        let mut s = state.write();
        s.jay1tribu_sender.set_context(None);
        #[cfg(feature = "service-jay1tribu")]
        set_webway_connected(false);
        s.state = new_state;
        s.conformity = new_conformity;
        s.is_lone = is_lone;
        s.discovered_cogs.clear();
        s.discovered_lobbys.clear();
        s.connecting = false;
        s.error = None;
        s.manager = Arc::new(TokioRwLock::new(None));
    }
}

pub(crate) async fn auto_connect_after_login(
    state: Signal<MwsViewState>,
    conns: Arc<crate::data::ServiceConnections>,
    display_name: String,
) {
    let _ = connect_with_manager_state(state, conns, display_name, true).await;
}

async fn real_mws_connect(
    state: Signal<MwsViewState>,
    conns: Arc<crate::data::ServiceConnections>,
    mut tick: Signal<u64>,
    display_name: String,
) {
    let _ = connect_with_manager_state(state, conns, display_name, false).await;
    let n = *tick.read();
    tick.set(n + 1);
}

/// Déconnexion réelle du réseau MWS : WITHDRAW au Tracker, arrêt du service, reset UI.
async fn real_mws_disconnect(state: Signal<MwsViewState>, mut tick: Signal<u64>) {
    disconnect_with_manager_state(state).await;
    let n = *tick.read();
    tick.set(n + 1);
}

/// Carte d'état de connexion.
#[component]
fn MwsStatusCard(state: Signal<MwsViewState>) -> Element {
    let current_state = state.read().state.clone();
    let conformity = state.read().conformity.clone();
    let _is_lone = state.read().is_lone;
    let config = state.read().config.clone();
    let auto_connect_label = if config.auto_connect { "on" } else { "off" };

    // Utiliser le résumé d'état
    let summary = MwsStatusSummary::from_state(&current_state, &conformity);

    // Détails supplémentaires selon l'état
    let detail_text = match &current_state {
        CentralMwsState::Connected => "Conformité complète — Présent sur le réseau".to_string(),
        CentralMwsState::RelayConnected => "Connecté au Relay, annonce au Tracker...".to_string(),
        CentralMwsState::Connecting => conformity.description().to_string(),
        CentralMwsState::Lone => "COG isolé — Données locales uniquement".to_string(),
        CentralMwsState::Disconnected => "Non connecté au réseau MWS".to_string(),
        CentralMwsState::Disabled => "MWS désactivé dans la configuration".to_string(),
        CentralMwsState::Error(e) => e.clone(),
    };

    rsx! {
        div {
            class: "mws-status-card",
            style: "
                background: #12121a;
                border: 1px solid rgba(139, 92, 246, 0.2);
                border-radius: 12px;
                padding: 16px;
                margin-bottom: 24px;
            ",

            // Ligne principale
            div {
                style: "display: flex; align-items: center; gap: 16px;",

                // Icône animée
                div {
                    style: "
                        width: 48px;
                        height: 48px;
                        border-radius: 12px;
                        background: rgba(139, 92, 246, 0.1);
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-size: 24px;
                    ",
                    "{summary.icon}"
                }

                // Texte principal
                div {
                    style: "flex: 1;",
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        p {
                            style: "margin: 0; color: #fff; font-weight: 600; font-size: 16px;",
                            "{summary.short_text}"
                        }
                        // Indicateur de couleur
                        div {
                            style: "
                                width: 8px;
                                height: 8px;
                                border-radius: 50%;
                                background: {summary.color};
                                box-shadow: 0 0 8px {summary.color};
                            ",
                        }
                    }
                    p {
                        style: "margin: 4px 0 0; color: #9ca3af; font-size: 13px;",
                        "{detail_text}"
                    }
                }

                // Stats (si connecté)
                if current_state == CentralMwsState::Connected {
                    div {
                        style: "display: flex; gap: 24px;",

                        div {
                            style: "text-align: center;",
                            div { style: "font-size: 20px; font-weight: 600; color: #8b5cf6;",
                                "{state.read().discovered_cogs.len()}"
                            }
                            div { style: "font-size: 12px; color: #9ca3af;", "COGs" }
                        }

                        div {
                            style: "text-align: center;",
                            div { style: "font-size: 20px; font-weight: 600; color: #06b6d4;",
                                "{state.read().discovered_lobbys.len()}"
                            }
                            div { style: "font-size: 12px; color: #9ca3af;", "Lobbys" }
                        }
                    }
                }
            }

            // Détails serveur (si connecté ou en connexion)
            if matches!(current_state, CentralMwsState::Connected | CentralMwsState::RelayConnected | CentralMwsState::Connecting) {
                div {
                    style: "
                        margin-top: 12px;
                        padding-top: 12px;
                        border-top: 1px solid rgba(255,255,255,0.1);
                        display: flex;
                        gap: 24px;
                        font-size: 12px;
                        color: #6b7280;
                    ",
                    span { "📡 Relay: {config.relay_address}" }
                    span { "📊 Tracker: {config.tracker_address}" }
                    span { "⚙️ Auto: {auto_connect_label}" }
                    if conformity.is_online() {
                        span { style: "color: #10b981;", "✓ Permis actif" }
                    }
                }
            }
        }
    }
}

/// Section de recherche.
#[component]
fn MwsSearchSection(mut state: Signal<MwsViewState>) -> Element {
    let search_cogs = move |_| {
        let mut state = state;
        spawn(async move {
            let arc = state.read().manager.clone();
            let guard = arc.read().await;
            let Some(ref manager) = *guard else { return };
            let query = state.read().search_query.clone();
            let version_filter = if query.is_empty() {
                None
            } else {
                Some(query.clone())
            };
            let cogs = manager
                .search_cogs(version_filter, None, 50)
                .await
                .unwrap_or_default();
            let lobbys = manager
                .search_lobbys(Some(query).filter(|q| !q.is_empty()), true, 50)
                .await
                .unwrap_or_default();
            drop(guard);

            let mut s = state.write();
            s.discovered_cogs = cogs
                .into_iter()
                .map(|c| DiscoveredCog {
                    cog_id: c.cog_id,
                    core_version: c.core_version,
                    address: c.address,
                    services: c.services,
                    last_seen: c.last_seen,
                })
                .collect();
            s.discovered_lobbys = lobbys
                .into_iter()
                .map(|r| DiscoveredLobby {
                    lobby_id: r.lobby.lobby_id,
                    name: r.lobby.name,
                    host_cog_id: r.host_cog_id,
                    pool_version: r.pool_version,
                    current_players: r.lobby.current_players,
                    max_players: r.lobby.max_players,
                    is_public: r.lobby.is_public,
                    password_required: r.lobby.password_required,
                })
                .collect();
        });
    };

    rsx! {
        div {
            class: "mws-search",
            style: "margin-bottom: 24px;",

            div {
                style: "display: flex; gap: 12px;",

                input {
                    r#type: "text",
                    placeholder: "Rechercher un COG ou un lobby...",
                    style: "
                        flex: 1;
                        padding: 12px 16px;
                        background: #12121a;
                        border: 1px solid rgba(139, 92, 246, 0.2);
                        border-radius: 8px;
                        color: #fff;
                        font-size: 14px;
                    ",
                    value: "{state.read().search_query}",
                    oninput: move |e| {
                        state.write().search_query = e.value();
                    }
                }

                button {
                    style: "
                        padding: 12px 24px;
                        background: #8b5cf6;
                        border: none;
                        border-radius: 8px;
                        color: white;
                        font-weight: 500;
                        cursor: pointer;
                    ",
                    onclick: search_cogs,
                    "🔍 Rechercher"
                }
            }
        }
    }
}

/// Section des lobbys.
#[component]
fn MwsLobbysSection(state: Signal<MwsViewState>) -> Element {
    rsx! {
        div {
            class: "mws-lobbys",
            style: "margin-bottom: 24px;",

            h2 {
                style: "margin: 0 0 16px; font-size: 18px; color: #fff;",
                "🎮 Lobbys disponibles"
            }

            div {
                style: "display: flex; flex-direction: column; gap: 12px;",

                for lobby in state.read().discovered_lobbys.iter() {
                    LobbyCard { lobby: lobby.clone() }
                }
            }
        }
    }
}

/// Carte de lobby.
#[component]
fn LobbyCard(lobby: DiscoveredLobby) -> Element {
    let players_color = if lobby.current_players >= lobby.max_players {
        "#ef4444"
    } else if lobby.current_players as f32 / lobby.max_players as f32 > 0.7 {
        "#f59e0b"
    } else {
        "#10b981"
    };

    rsx! {
        div {
            class: "lobby-card",
            style: "
                background: #12121a;
                border: 1px solid rgba(139, 92, 246, 0.2);
                border-radius: 8px;
                padding: 16px;
                display: flex;
                justify-content: space-between;
                align-items: center;
                transition: border-color 0.2s;
                cursor: pointer;
            ",

            div {
                h3 {
                    style: "margin: 0 0 4px; font-size: 16px; color: #fff;",
                    "{lobby.name}"
                    if lobby.password_required {
                        span { style: "margin-left: 8px;", "🔒" }
                    }
                }
                p {
                    style: "margin: 0; font-size: 12px; color: #9ca3af;",
                    "Hôte: {lobby.host_cog_id} • Version {lobby.pool_version}"
                }
            }

            div {
                style: "display: flex; align-items: center; gap: 16px;",

                div {
                    style: "text-align: right;",
                    div {
                        style: "font-weight: 600; color: {players_color};",
                        "{lobby.current_players}/{lobby.max_players}"
                    }
                    div {
                        style: "font-size: 12px; color: #9ca3af;",
                        "joueurs"
                    }
                }

                button {
                    style: "
                        padding: 8px 16px;
                        background: #8b5cf6;
                        border: none;
                        border-radius: 6px;
                        color: white;
                        font-weight: 500;
                        cursor: pointer;
                    ",
                    "Rejoindre"
                }
            }
        }
    }
}

/// Section des COGs.
#[component]
fn MwsCogsSection(state: Signal<MwsViewState>) -> Element {
    rsx! {
        div {
            class: "mws-cogs",

            h2 {
                style: "margin: 0 0 16px; font-size: 18px; color: #fff;",
                "🖥️ COGs sur le réseau"
            }

            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 12px;",

                for cog in state.read().discovered_cogs.iter() {
                    CogCard { cog: cog.clone() }
                }
            }
        }
    }
}

/// Carte de COG.
#[component]
fn CogCard(cog: DiscoveredCog) -> Element {
    rsx! {
        div {
            class: "cog-card",
            style: "
                background: #12121a;
                border: 1px solid rgba(139, 92, 246, 0.2);
                border-radius: 8px;
                padding: 16px;
            ",

            div {
                style: "display: flex; justify-content: space-between; margin-bottom: 12px;",

                h3 {
                    style: "margin: 0; font-size: 14px; color: #fff; font-family: monospace;",
                    "{cog.cog_id}"
                }
                span {
                    style: "
                        padding: 2px 8px;
                        background: rgba(139, 92, 246, 0.2);
                        border-radius: 4px;
                        font-size: 12px;
                        color: #8b5cf6;
                    ",
                    "vers. {cog.core_version}"
                }
            }

            p {
                style: "margin: 0 0 8px; font-size: 12px; color: #9ca3af;",
                "📍 {cog.address}"
            }

            div {
                style: "display: flex; flex-wrap: wrap; gap: 4px;",

                for service in cog.services.iter() {
                    span {
                        style: "
                            padding: 2px 8px;
                            background: rgba(6, 182, 212, 0.2);
                            border-radius: 4px;
                            font-size: 11px;
                            color: #06b6d4;
                        ",
                        "{service}"
                    }
                }
            }

            p {
                style: "margin: 8px 0 0; font-size: 11px; color: #6b7280;",
                "Vu {cog.last_seen}"
            }
        }
    }
}
