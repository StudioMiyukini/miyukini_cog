//! Vue MWS — Interface réseau Miyukini Webway System.
//!
//! Affiche l'état de connexion au réseau MWS et permet de :
//! - Se connecter / déconnecter du réseau
//! - Voir les étapes de conformité (Origin → Relay → Tracker)
//! - Rechercher des COGs et lobbys
//! - Créer et rejoindre des lobbys
//! - Activer/désactiver le mode Lone (COG isolé)

use dioxus::prelude::*;
use miyukini_central::{
    CentralMwsManager, CentralMwsState, MwsConformityState, MwsStatusSummary,
};
use std::sync::Arc;
use tokio::sync::RwLock;

/// État du composant MWS.
#[derive(Clone)]
pub struct MwsViewState {
    /// Gestionnaire MWS.
    pub manager: Arc<RwLock<Option<CentralMwsManager>>>,
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
}

impl Default for MwsViewState {
    fn default() -> Self {
        Self {
            manager: Arc::new(RwLock::new(None)),
            state: CentralMwsState::Disconnected,
            conformity: MwsConformityState::Uninitialized,
            is_lone: false,
            discovered_cogs: Vec::new(),
            discovered_lobbys: Vec::new(),
            search_query: String::new(),
            connecting: false,
            error: None,
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
#[component]
pub fn MwsNetworkView() -> Element {
    let mut mws_state = use_signal(MwsViewState::default);

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
                    MwsLoneModeToggle { state: mws_state }
                    
                    // Bouton connexion/déconnexion
                    MwsConnectionButton { state: mws_state }
                }
            }

            // État de connexion avec étapes de conformité
            MwsStatusCard { state: mws_state }

            // Barre de progression de conformité (pendant connexion)
            if mws_state.read().connecting {
                MwsConformityProgress { state: mws_state }
            }

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
fn MwsLoneModeToggle(mut state: Signal<MwsViewState>) -> Element {
    let is_lone = state.read().is_lone;

    let onclick = move |_| {
        let mut s = state.write();
        s.is_lone = !s.is_lone;
        if s.is_lone {
            s.state = CentralMwsState::Lone;
            s.conformity = MwsConformityState::LoneMode;
            s.discovered_cogs.clear();
            s.discovered_lobbys.clear();
        } else {
            s.state = CentralMwsState::Disconnected;
            s.conformity = MwsConformityState::Uninitialized;
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
    let progress_pct = (step as f32 / total_steps as f32 * 100.0) as u32;

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
fn MwsConnectionButton(mut state: Signal<MwsViewState>) -> Element {
    let current_state = state.read().state.clone();
    let conformity = state.read().conformity.clone();
    let connecting = state.read().connecting;
    let is_lone = state.read().is_lone;

    // Déterminer les propriétés du bouton via le résumé d'état
    let summary = MwsStatusSummary::from_state(&current_state, &conformity);

    let onclick = move |_| {
        let mut state = state.clone();
        spawn(async move {
            let current = state.read().state.clone();
            let is_lone = state.read().is_lone;

            if is_lone {
                return; // Pas d'action en mode Lone
            }

            if current == CentralMwsState::Connected || current == CentralMwsState::RelayConnected {
                // Déconnexion
                {
                    let mut s = state.write();
                    s.state = CentralMwsState::Disconnected;
                    s.conformity = MwsConformityState::Uninitialized;
                    s.discovered_cogs.clear();
                    s.discovered_lobbys.clear();
                }
            } else {
                // Connexion avec simulation du protocole complet
                simulate_mws_connection(&mut state).await;
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

/// Simule le protocole de connexion MWS complet.
async fn simulate_mws_connection(state: &mut Signal<MwsViewState>) {
    // Phase 1: Initialisation
    {
        let mut s = state.write();
        s.connecting = true;
        s.error = None;
        s.state = CentralMwsState::Connecting;
    }

    // Étape 1: Résolution Origin
    {
        let mut s = state.write();
        s.conformity = MwsConformityState::ResolvingOrigin;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    // Étape 2: Connexion TLS au Relay
    {
        let mut s = state.write();
        s.conformity = MwsConformityState::ConnectingRelay;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Étape 3: Enregistrement Relay
    {
        let mut s = state.write();
        s.conformity = MwsConformityState::RegisteringRelay;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    // Étape 4: Attente accusé Relay
    {
        let mut s = state.write();
        s.conformity = MwsConformityState::AwaitingRelayAck;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;

    // Étape 5: Session Relay établie
    {
        let mut s = state.write();
        s.conformity = MwsConformityState::RelaySessionEstablished;
        s.state = CentralMwsState::RelayConnected;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // Étape 6: Obtention du Permis
    {
        let mut s = state.write();
        s.conformity = MwsConformityState::ObtainingPermis;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;

    // Étape 7: Connexion au Tracker
    {
        let mut s = state.write();
        s.conformity = MwsConformityState::ConnectingTracker;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    // Étape 8: Annonce au Tracker
    {
        let mut s = state.write();
        s.conformity = MwsConformityState::AnnouncingTracker;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    // Étape 9: Attente accusé Tracker
    {
        let mut s = state.write();
        s.conformity = MwsConformityState::AwaitingTrackerAck;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    // Étape 10: Conformité complète
    {
        let mut s = state.write();
        s.conformity = MwsConformityState::FullyConformant;
        s.connecting = false;
        s.state = CentralMwsState::Connected;

        // Ajouter des lobbys de démonstration
        s.discovered_lobbys = vec![
            DiscoveredLobby {
                lobby_id: "lobby-1".to_string(),
                name: "Lord of the Click - Partie rapide".to_string(),
                host_cog_id: "cog-abc123".to_string(),
                pool_version: "1.0.0".to_string(),
                current_players: 3,
                max_players: 8,
                is_public: true,
                password_required: false,
            },
            DiscoveredLobby {
                lobby_id: "lobby-2".to_string(),
                name: "Survivor Coop".to_string(),
                host_cog_id: "cog-def456".to_string(),
                pool_version: "1.0.0".to_string(),
                current_players: 1,
                max_players: 4,
                is_public: true,
                password_required: false,
            },
        ];
    }
}

/// Carte d'état de connexion.
#[component]
fn MwsStatusCard(state: Signal<MwsViewState>) -> Element {
    let current_state = state.read().state.clone();
    let conformity = state.read().conformity.clone();
    let is_lone = state.read().is_lone;

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
                    span { "📡 Relay: origin.miyukini.net:7000" }
                    span { "📊 Tracker: origin.miyukini.net:21000" }
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
        let mut state = state.clone();
        spawn(async move {
            // TODO: appeler tracker_client.search_cogs()
            // Simuler des résultats
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            
            {
                let mut s = state.write();
                s.discovered_cogs = vec![
                    DiscoveredCog {
                        cog_id: "cog-abc123".to_string(),
                        core_version: "1.0.0".to_string(),
                        address: "192.168.1.100:8080".to_string(),
                        services: vec!["jayfestival".to_string(), "jayxpose".to_string()],
                        last_seen: "il y a 2 min".to_string(),
                    },
                    DiscoveredCog {
                        cog_id: "cog-def456".to_string(),
                        core_version: "1.0.0".to_string(),
                        address: "192.168.1.101:8080".to_string(),
                        services: vec!["miyuclicker".to_string()],
                        last_seen: "il y a 5 min".to_string(),
                    },
                ];
            }
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
                    "v{cog.core_version}"
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
