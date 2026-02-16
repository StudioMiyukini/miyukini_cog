//! Vue MWS — Interface réseau Miyukini Webway System.
//!
//! Affiche l'état de connexion au réseau MWS et permet de :
//! - Se connecter / déconnecter du réseau
//! - Voir les étapes de conformité (Origin → Relay → Tracker)
//! - Rechercher des COGs et lobbys
//! - Créer et rejoindre des lobbys
//! - Activer/désactiver le mode Lone (COG isolé)

use crate::data::{profile_display_name, use_service_connections};
use crate::state::AppContext;
use dioxus::prelude::*;
use jay1tribu::{set_mws_transport_sender, set_webway_connected, DispatchError, MwsTransportSender};
use miyukini_central::{
    CentralMwsConfig, CentralMwsManager, CentralMwsState, MwsConformityState, MwsStatusSummary,
};
use miyuwebway_participant::GovernedContext;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::RwLock as TokioRwLock;

/// Sender MWS pour Jay1Tribu : délègue à miyuwebway_participant::transport::send.
/// Le contexte est mis à jour à la connexion / déconnexion MWS.
struct CentralJay1TribuSender {
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

impl MwsTransportSender for CentralJay1TribuSender {
    fn send(&self, to: &str, payload: &[u8]) -> Result<(), DispatchError> {
        let guard = self.ctx.read().map_err(|e| DispatchError::Transport(e.to_string()))?;
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
    /// Sender MWS pour Jay1Tribu (envoi de messages via le Webway).
    pub jay1tribu_sender: Arc<CentralJay1TribuSender>,
}

impl Default for MwsViewState {
    fn default() -> Self {
        let jay1tribu_sender = CentralJay1TribuSender::new();
        set_mws_transport_sender(jay1tribu_sender.clone());
        Self {
            manager: Arc::new(TokioRwLock::new(None)),
            state: CentralMwsState::Disconnected,
            conformity: MwsConformityState::Uninitialized,
            is_lone: false,
            discovered_cogs: Vec::new(),
            discovered_lobbys: Vec::new(),
            search_query: String::new(),
            connecting: false,
            error: None,
            jay1tribu_sender,
        }
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
                        tracing::info!("[MWS UI poll] état changé: {:?} → {:?}, conformité: {:?} → {:?}", current_state, st, current_conf, conf);
                        let connected = st == CentralMwsState::Connected;
                        set_webway_connected(connected);
                        let mut s = state.write();
                        s.jay1tribu_sender.set_context(
                            if connected {
                                Some(GovernedContext::new("mws-jay1tribu".to_string(), 0))
                            } else {
                                None
                            },
                        );
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
            spawn(async move {
                real_mws_disconnect(state, tick).await;
                let mut s = state.write();
                s.is_lone = true;
                s.state = CentralMwsState::Lone;
                s.conformity = MwsConformityState::LoneMode;
                s.discovered_cogs.clear();
                s.discovered_lobbys.clear();
                drop(s);
                let n = *tick.read();
                tick.set(n + 1);
            });
        } else {
            let mut s = state.write();
            s.jay1tribu_sender.set_context(None);
            drop(s);
            set_webway_connected(false);
            let mut s = state.write();
            s.is_lone = false;
            s.state = CentralMwsState::Disconnected;
            s.conformity = MwsConformityState::Uninitialized;
            drop(s);
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
    let user_display_name = app_ctx.state.read().current_user.as_ref()
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
/// Si une vitrine JayXpose est publiée, la carte « Découvrir » est affichée sur la Home.
/// `tick` est incrémenté après mise à jour pour forcer le re-render de l'UI (spawn → Dioxus).
/// `display_name` est le pseudonyme ou l'email de l'utilisateur connecté — utilisé comme identifiant COG sur le Webway.
async fn real_mws_connect(
    mut state: Signal<MwsViewState>,
    conns: Arc<crate::data::ServiceConnections>,
    mut tick: Signal<u64>,
    display_name: String,
) {
    tracing::info!("[MWS UI] real_mws_connect démarré (cog_id={})", &display_name);
    {
        let mut s = state.write();
        s.connecting = true;
        s.error = None;
        s.state = CentralMwsState::Connecting;
    }

    // Slug vitrine JayXpose publiée (pour la carte « Découvrir » sur la Home)
    let jayxpose_slug = conns.jayxpose.first_published_vitrine_slug().ok().flatten();
    let default_cfg = CentralMwsConfig::default();
    let (expose_jayxpose_vitrine, jayxpose_vitrine_base_url) = if jayxpose_slug.is_some() {
        let (host, _) = default_cfg.relay_address.split_once(':').unwrap_or((default_cfg.relay_address.as_str(), "7000"));
        (true, Some(format!("http://{host}:8080")))
    } else {
        (false, None)
    };
    // Exposer la page Home du COG quand annoncé : écoute HTTP et adresse annoncée au Tracker
    // Port 8090 pour éviter conflit avec Origin (8080 par défaut)
    // public_address : on annonce 0.0.0.0:8090 (adresse locale).
    // Le Tracker détecte l'adresse locale et la remplace automatiquement par l'IP réelle
    // du COG (vue par le Tracker) + le port 8090. Les visiteurs externes sont ainsi
    // redirigés vers l'IP publique du host.
    let config = CentralMwsConfig {
        home_http_bind: Some("0.0.0.0:8090".to_string()),
        public_address: "0.0.0.0:8090".to_string(),
        expose_jayxpose_vitrine,
        jayxpose_vitrine_base_url,
        ..default_cfg
    };

    let services = if jayxpose_slug.is_some() {
        vec!["jayxpose".to_string()]
    } else {
        vec![]
    };
    // Utiliser le display_name de l'utilisateur comme cog_id au lieu de "central-native"
    let cog_id = if display_name.is_empty() { "central-native".to_string() } else { display_name };
    // Passer la DB JayXpose au manager pour servir les pages vitrine localement
    let jayxpose_db = Some(Arc::clone(&conns.jayxpose));
    let manager = CentralMwsManager::new(
        config,
        cog_id,
        "0.1.0".to_string(),
        services,
        jayxpose_slug,
        jayxpose_db,
    );

    match manager.connect().await {
        Ok(()) => {
            let st = manager.get_state().await;
            let conf = manager.get_conformity_state().await;
            let cogs = manager
                .search_cogs(None, None, 50)
                .await
                .unwrap_or_default();
            let lobbys = manager
                .search_lobbys(None, true, 50)
                .await
                .unwrap_or_default();

            let mut s = state.write();
            s.manager = Arc::new(RwLock::new(Some(manager)));
            s.state = st;
            s.conformity = conf;
            s.connecting = false;
            s.error = None;
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
            drop(s);
            tracing::info!("[MWS UI] real_mws_connect OK → tick++");
            let n = *tick.read();
            tick.set(n + 1);
        }
        Err(e) => {
            tracing::error!("[MWS UI] real_mws_connect ERREUR: {}", &e);
            let mut s = state.write();
            s.connecting = false;
            s.error = Some(e.clone());
            s.state = CentralMwsState::Error(e);
            drop(s);
            let n = *tick.read();
            tick.set(n + 1);
        }
    }
}

/// Déconnexion réelle du réseau MWS : WITHDRAW au Tracker, arrêt du service, reset UI.
async fn real_mws_disconnect(mut state: Signal<MwsViewState>, mut tick: Signal<u64>) {
    tracing::info!("[MWS UI] real_mws_disconnect — début");

    // Indiquer visuellement que la déconnexion est en cours
    {
        let mut s = state.write();
        s.connecting = true; // Réutilisé comme indicateur "opération en cours"
    }
    let n = *tick.read();
    tick.set(n + 1);

    // Récupérer et consommer le manager (disconnect + withdraw)
    let arc = state.read().manager.clone();
    let mut guard = arc.write().await;
    if let Some(mgr) = guard.take() {
        tracing::info!("[MWS UI] Appel manager.disconnect() (WITHDRAW + arrêt)…");
        match mgr.disconnect().await {
            Ok(()) => tracing::info!("[MWS UI] manager.disconnect() OK"),
            Err(e) => tracing::warn!("[MWS UI] manager.disconnect() erreur: {}", e),
        }
    } else {
        tracing::warn!("[MWS UI] Pas de manager actif — déconnexion locale uniquement");
    }
    drop(guard);

    // Réinitialiser complètement l'état UI
    {
        let mut s = state.write();
        s.jay1tribu_sender.set_context(None);
        set_webway_connected(false);
        s.state = CentralMwsState::Disconnected;
        s.conformity = MwsConformityState::Uninitialized;
        s.discovered_cogs.clear();
        s.discovered_lobbys.clear();
        s.connecting = false;
        s.error = None;
        // Réinitialiser le manager pour le prochain connect
        s.manager = std::sync::Arc::new(tokio::sync::RwLock::new(None));
    }
    tracing::info!("[MWS UI] real_mws_disconnect terminé — état réinitialisé → tick++");
    let n = *tick.read();
    tick.set(n + 1);
}

/// Carte d'état de connexion.
#[component]
fn MwsStatusCard(state: Signal<MwsViewState>) -> Element {
    let current_state = state.read().state.clone();
    let conformity = state.read().conformity.clone();
    let _is_lone = state.read().is_lone;

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
                    span { "📡 Relay: miyukini.com:7000" }
                    span { "📊 Tracker: miyukini.com:21000" }
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
