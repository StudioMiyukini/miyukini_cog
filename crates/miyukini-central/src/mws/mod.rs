//! Module MWS — Miyukini Webway System pour le Central.
//!
//! Fournit l'intégration du réseau MWS au COG Central, permettant :
//! - Connexion au réseau MWS Origin
//! - Protocole de conformité complet (Origin → Relay → Tracker)
//! - Présentation sur le réseau même sans services exposés
//! - Découverte d'autres COGs et services
//! - Hébergement et recherche de lobbys
//! - Heartbeat automatique
//!
//! ## Mode COG Lone
//!
//! Un COG peut fonctionner en mode "Lone" (isolé) sans se connecter au réseau.
//! Ce mode est utile pour :
//! - Développement local
//! - Utilisation hors ligne
//! - Souveraineté complète des données

use miyuwebway_participant::{
    CogIdentity, CogInfo, LobbyInfo, LobbySearchResult, MwsService, MwsServiceConfig,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::{oneshot, RwLock};
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

// ═══════════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Configuration MWS pour le Central.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentralMwsConfig {
    /// Activer MWS.
    pub enabled: bool,
    /// Mode COG Lone (pas de connexion réseau).
    pub lone_mode: bool,
    /// Connexion automatique au démarrage.
    pub auto_connect: bool,
    /// Adresse du Relay Origin.
    pub relay_address: String,
    /// Adresse du Tracker Origin.
    pub tracker_address: String,
    /// Clé de conformité CORE (Phase C) pour le Relay en mode strict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub core_conformity_key: Option<Vec<u8>>,
    /// Adresse publique de ce COG.
    pub public_address: String,
    /// Activer le heartbeat automatique.
    pub auto_heartbeat: bool,
    /// Activer la reconnexion automatique.
    pub auto_reconnect: bool,
    /// Présenter le COG même sans services exposés.
    pub present_without_services: bool,
    /// Adresse d'écoute du mini serveur Home (ex. "0.0.0.0:8080"). Si présent, une page Home est exposée lorsque le COG est annoncé sur le Tracker.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home_http_bind: Option<String>,
    /// Afficher une carte JayXpose « Découvrir » sur la Home si une vitrine est publiée.
    #[serde(default)]
    pub expose_jayxpose_vitrine: bool,
    /// URL de base du serveur web Origin (vitrines). Ex. "http://origin.example.com:8080". Requis pour le lien « Découvrir ».
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jayxpose_vitrine_base_url: Option<String>,
}

impl Default for CentralMwsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            lone_mode: false,
            auto_connect: true,
            relay_address: "miyukini.com:7000".to_string(),
            tracker_address: "miyukini.com:21000".to_string(),
            core_conformity_key: None,
            public_address: "0.0.0.0:0".to_string(), // Sera détecté
            auto_heartbeat: true,
            auto_reconnect: true,
            present_without_services: true, // Se présente toujours
            home_http_bind: None, // Optionnel : "0.0.0.0:8080" pour exposer la Home
            expose_jayxpose_vitrine: false,
            jayxpose_vitrine_base_url: None,
        }
    }
}

impl CentralMwsConfig {
    /// Configuration pour un COG Lone (isolé).
    pub fn lone() -> Self {
        Self {
            enabled: false,
            lone_mode: true,
            auto_connect: false,
            ..Default::default()
        }
    }

    /// Vérifie si le COG doit se connecter au réseau.
    pub fn should_connect(&self) -> bool {
        self.enabled && !self.lone_mode
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ÉTATS DE CONFORMITÉ
// ═══════════════════════════════════════════════════════════════════════════════

/// État détaillé du protocole de conformité MWS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MwsConformityState {
    /// Non initialisé.
    Uninitialized,
    /// Mode Lone actif (pas de connexion).
    LoneMode,
    /// Résolution de l'adresse Origin.
    ResolvingOrigin,
    /// Connexion TLS au Relay.
    ConnectingRelay,
    /// Enregistrement auprès du Relay (envoi Register).
    RegisteringRelay,
    /// Attente du RegisterAck.
    AwaitingRelayAck,
    /// Session Relay établie, passage aux phases A/B/C.
    RelaySessionEstablished,
    /// Obtention du Permis.
    ObtainingPermis,
    /// Permis obtenu, connexion au Tracker.
    ConnectingTracker,
    /// Annonce au Tracker.
    AnnouncingTracker,
    /// Attente de l'AnnounceAck.
    AwaitingTrackerAck,
    /// Conformité complète — COG présent sur le réseau.
    FullyConformant,
    /// Erreur de conformité.
    ConformityError(String),
}

impl MwsConformityState {
    /// Retourne une description lisible de l'état.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Uninitialized => "Non initialisé",
            Self::LoneMode => "Mode Lone — Hors réseau",
            Self::ResolvingOrigin => "Résolution de l'adresse Origin...",
            Self::ConnectingRelay => "Connexion TLS au Relay...",
            Self::RegisteringRelay => "Enregistrement auprès du Relay...",
            Self::AwaitingRelayAck => "Attente de l'accusé Relay...",
            Self::RelaySessionEstablished => "Session Relay établie",
            Self::ObtainingPermis => "Obtention du Permis...",
            Self::ConnectingTracker => "Connexion au Tracker...",
            Self::AnnouncingTracker => "Annonce au Tracker...",
            Self::AwaitingTrackerAck => "Attente de l'accusé Tracker...",
            Self::FullyConformant => "Conformité complète — En ligne",
            Self::ConformityError(_) => "Erreur de conformité",
        }
    }

    /// Retourne le numéro d'étape (0-10).
    pub fn step_number(&self) -> u8 {
        match self {
            Self::Uninitialized => 0,
            Self::LoneMode => 0,
            Self::ResolvingOrigin => 1,
            Self::ConnectingRelay => 2,
            Self::RegisteringRelay => 3,
            Self::AwaitingRelayAck => 4,
            Self::RelaySessionEstablished => 5,
            Self::ObtainingPermis => 6,
            Self::ConnectingTracker => 7,
            Self::AnnouncingTracker => 8,
            Self::AwaitingTrackerAck => 9,
            Self::FullyConformant => 10,
            Self::ConformityError(_) => 0,
        }
    }

    /// Vérifie si l'état est une erreur.
    pub fn is_error(&self) -> bool {
        matches!(self, Self::ConformityError(_))
    }

    /// Vérifie si le COG est en ligne.
    pub fn is_online(&self) -> bool {
        matches!(self, Self::FullyConformant)
    }
}

/// État du MWS dans le Central (simplifié pour l'UI).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CentralMwsState {
    /// Désactivé.
    Disabled,
    /// Mode Lone (isolé).
    Lone,
    /// Déconnecté.
    Disconnected,
    /// En cours de connexion.
    Connecting,
    /// Connecté au Relay (phase intermédiaire).
    RelayConnected,
    /// Connecté et annoncé (conformité complète).
    Connected,
    /// Erreur.
    Error(String),
}

// ═══════════════════════════════════════════════════════════════════════════════
// GESTIONNAIRE MWS
// ═══════════════════════════════════════════════════════════════════════════════

/// Gestionnaire MWS pour le Central.
///
/// Gère le cycle de vie complet de la connexion au réseau MWS :
/// 1. Résolution de l'adresse Origin
/// 2. Connexion TLS au Relay
/// 3. Enregistrement et obtention du Permis
/// 4. Annonce au Tracker
/// 5. Heartbeat automatique
pub struct CentralMwsManager {
    /// Configuration.
    config: CentralMwsConfig,
    /// Service MWS sous-jacent.
    service: Arc<RwLock<Option<MwsService>>>,
    /// État simplifié.
    state: Arc<RwLock<CentralMwsState>>,
    /// État de conformité détaillé.
    conformity_state: Arc<RwLock<MwsConformityState>>,
    /// ID du COG.
    cog_id: String,
    /// Version des Cores.
    core_version: String,
    /// Services exposés.
    services: Arc<RwLock<Vec<String>>>,
    /// Lobbys hébergés.
    lobbys: Arc<RwLock<Vec<LobbyInfo>>>,
    /// Callback de mise à jour d'état.
    state_callback: Arc<RwLock<Option<Box<dyn Fn(MwsConformityState) + Send + Sync>>>>,
    /// Envoi du signal d'arrêt du serveur Home (si actif).
    home_server_shutdown: Arc<RwLock<Option<oneshot::Sender<()>>>>,
    /// Slug de la première vitrine publiée (pour la carte JayXpose sur la Home).
    jayxpose_vitrine_slug: Option<String>,
}

impl CentralMwsManager {
    /// Crée un nouveau gestionnaire MWS.
    /// `jayxpose_vitrine_slug` : slug de la vitrine publiée à afficher sur la Home (optionnel).
    pub fn new(
        config: CentralMwsConfig,
        cog_id: String,
        core_version: String,
        services: Vec<String>,
        jayxpose_vitrine_slug: Option<String>,
    ) -> Self {
        let (state, conformity) = if config.lone_mode {
            (CentralMwsState::Lone, MwsConformityState::LoneMode)
        } else if config.enabled {
            (CentralMwsState::Disconnected, MwsConformityState::Uninitialized)
        } else {
            (CentralMwsState::Disabled, MwsConformityState::Uninitialized)
        };

        Self {
            config,
            service: Arc::new(RwLock::new(None)),
            state: Arc::new(RwLock::new(state)),
            conformity_state: Arc::new(RwLock::new(conformity)),
            cog_id,
            core_version,
            services: Arc::new(RwLock::new(services)),
            lobbys: Arc::new(RwLock::new(Vec::new())),
            state_callback: Arc::new(RwLock::new(None)),
            home_server_shutdown: Arc::new(RwLock::new(None)),
            jayxpose_vitrine_slug,
        }
    }

    /// Crée un gestionnaire avec la configuration par défaut.
    pub fn with_defaults(cog_id: String, core_version: String) -> Self {
        Self::new(
            CentralMwsConfig::default(),
            cog_id,
            core_version,
            Vec::new(),
            None,
        )
    }

    /// Crée un gestionnaire en mode Lone (isolé).
    pub fn lone(cog_id: String, core_version: String) -> Self {
        Self::new(
            CentralMwsConfig::lone(),
            cog_id,
            core_version,
            Vec::new(),
            None,
        )
    }

    /// Définit un callback pour les changements d'état.
    pub async fn set_state_callback<F>(&self, callback: F)
    where
        F: Fn(MwsConformityState) + Send + Sync + 'static,
    {
        let mut cb = self.state_callback.write().await;
        *cb = Some(Box::new(callback));
    }

    /// Met à jour l'état de conformité.
    async fn update_conformity(&self, new_state: MwsConformityState) {
        let old_state = {
            let mut state = self.conformity_state.write().await;
            let old = state.clone();
            *state = new_state.clone();
            old
        };

        if old_state != new_state {
            info!(
                "MWS Conformity: {} → {} (step {})",
                old_state.description(),
                new_state.description(),
                new_state.step_number()
            );

            // Appeler le callback si défini
            let callback = self.state_callback.read().await;
            if let Some(ref cb) = *callback {
                cb(new_state);
            }
        }
    }

    /// Met à jour l'état simplifié.
    async fn update_state(&self, new_state: CentralMwsState) {
        let mut state = self.state.write().await;
        *state = new_state;
    }

    /// Connecte au réseau MWS avec le protocole de conformité complet.
    ///
    /// Flux de connexion :
    /// 1. Résolution DNS de l'adresse Origin
    /// 2. Connexion TLS au Relay (port 7000)
    /// 3. Envoi du message REGISTER
    /// 4. Réception du REGISTER_ACK
    /// 5. Passage des phases A, B, C pour obtenir le Permis
    /// 6. Connexion au Tracker (port 21000)
    /// 7. Envoi de l'ANNOUNCE
    /// 8. Réception de l'ANNOUNCE_ACK
    /// 9. Conformité complète — COG présent sur le réseau
    pub async fn connect(&self) -> Result<(), String> {
        // Vérifier si en mode Lone
        if self.config.lone_mode {
            info!("COG en mode Lone — Pas de connexion réseau");
            self.update_conformity(MwsConformityState::LoneMode).await;
            self.update_state(CentralMwsState::Lone).await;
            return Ok(());
        }

        // Vérifier si MWS est activé
        if !self.config.enabled {
            return Err("MWS est désactivé".to_string());
        }

        info!("Démarrage du protocole de conformité MWS pour COG {}...", self.cog_id);
        self.update_state(CentralMwsState::Connecting).await;

        // Étape 1: Résolution Origin
        self.update_conformity(MwsConformityState::ResolvingOrigin).await;
        
        // Vérifier que l'adresse est valide
        let relay_host = self.config.relay_address.split(':').next()
            .ok_or("Adresse Relay invalide")?;
        info!("Origin résolu: {}", relay_host);

        // Étape 2: Connexion au Relay
        self.update_conformity(MwsConformityState::ConnectingRelay).await;

        // Créer la configuration MWS
        let mut mws_config = MwsServiceConfig::default();
        mws_config.relay.relay_address = self.config.relay_address.clone();
        mws_config.relay.tls_domain = relay_host.to_string();
        mws_config.relay.core_conformity_key = self.config.core_conformity_key.clone();
        mws_config.tracker.tracker_address = self.config.tracker_address.clone();
        mws_config.auto_heartbeat = self.config.auto_heartbeat;
        mws_config.auto_reconnect = self.config.auto_reconnect;

        // Créer le service MWS
        let service = MwsService::new(mws_config);

        // Créer l'identité du COG
        let services = self.services.read().await.clone();
        let identity = CogIdentity {
            cog_id: self.cog_id.clone(),
            core_version: self.core_version.clone(),
            public_address: self.config.public_address.clone(),
            services: services.clone(),
        };

        // Message de présentation
        if services.is_empty() && self.config.present_without_services {
            info!(
                "COG {} se présente sur le réseau MWS (pas de services exposés)",
                self.cog_id
            );
        } else {
            info!(
                "COG {} se présente sur le réseau MWS avec {} service(s): {:?}",
                self.cog_id,
                services.len(),
                services
            );
        }

        // Étape 3-6: Enregistrement Relay et obtention Permis
        self.update_conformity(MwsConformityState::RegisteringRelay).await;
        self.update_conformity(MwsConformityState::AwaitingRelayAck).await;

        // Démarrer le service (inclut connexion Relay + annonce Tracker)
        match service.start(identity).await {
            Ok(_) => {
                self.update_conformity(MwsConformityState::RelaySessionEstablished).await;
                self.update_conformity(MwsConformityState::ObtainingPermis).await;
                self.update_state(CentralMwsState::RelayConnected).await;

                // Étape 7-9: Annonce au Tracker
                self.update_conformity(MwsConformityState::ConnectingTracker).await;
                self.update_conformity(MwsConformityState::AnnouncingTracker).await;
                self.update_conformity(MwsConformityState::AwaitingTrackerAck).await;

                // Stocker le service
                {
                    let mut svc = self.service.write().await;
                    *svc = Some(service);
                }

                // Démarrer le heartbeat Central → Tracker pour maintenir la connexion.
                // Intervalle de 25s (le seuil Absent côté Origin est 90s, TTL cleanup 300s).
                let service_ref = Arc::clone(&self.service);
                let cog_id_log = self.cog_id.clone();
                let heartbeat_interval_secs = 25u64;
                info!("Démarrage heartbeat Central → Tracker (interval={}s, cog={})", heartbeat_interval_secs, &cog_id_log);
                tokio::spawn(async move {
                    let mut count: u64 = 0;
                    loop {
                        sleep(Duration::from_secs(heartbeat_interval_secs)).await;
                        count += 1;
                        let guard = service_ref.read().await;
                        let Some(ref svc) = *guard else {
                            warn!("[Heartbeat #{}] Service MWS disparu, arrêt du heartbeat (COG {})", count, cog_id_log);
                            break;
                        };
                        match svc.send_heartbeat().await {
                            Ok(()) => {
                                info!("[Heartbeat #{} OK] Central → Tracker (COG {})", count, cog_id_log);
                            }
                            Err(e) => {
                                warn!("[Heartbeat #{} ERREUR] Central → Tracker (COG {}): {}", count, cog_id_log, e);
                            }
                        }
                    }
                });

                // Conformité complète
                self.update_conformity(MwsConformityState::FullyConformant).await;
                self.update_state(CentralMwsState::Connected).await;

                // Démarrer le mini serveur Home si configuré (exposition automatique)
                if let Some(ref bind_addr) = self.config.home_http_bind {
                    let cog_id = self.cog_id.clone();
                    let core_version = self.core_version.clone();
                    let services = self.services.read().await.clone();
                    let home_jayxpose = (self.config.expose_jayxpose_vitrine
                        && self.config.jayxpose_vitrine_base_url.is_some()
                        && self.jayxpose_vitrine_slug.is_some())
                        .then(|| HomeJayXposeInfo {
                            base_url: self.config.jayxpose_vitrine_base_url.clone().unwrap(),
                            slug: self.jayxpose_vitrine_slug.clone().unwrap(),
                        });
                    let (tx, rx) = oneshot::channel();
                    let bind_for_server = bind_addr.clone();
                    let bind_for_log = bind_addr.clone();
                    tokio::spawn(async move {
                        match run_home_server(bind_for_server.clone(), cog_id, core_version, services, home_jayxpose, rx).await {
                            Ok(()) => info!("Serveur Home arrêté proprement"),
                            Err(e) => error!("❌ Serveur Home ERREUR (bind={}): {}", bind_for_server, e),
                        }
                    });
                    let mut shutdown_guard = self.home_server_shutdown.write().await;
                    *shutdown_guard = Some(tx);
                    info!("Page Home exposée sur http://{}", bind_for_log);
                }

                info!(
                    "✅ COG {} est maintenant PRÉSENT sur le réseau MWS",
                    self.cog_id
                );

                Ok(())
            }
            Err(e) => {
                let error_msg = e.to_string();
                error!("❌ Échec de la conformité MWS: {}", error_msg);

                self.update_conformity(MwsConformityState::ConformityError(error_msg.clone())).await;
                self.update_state(CentralMwsState::Error(error_msg.clone())).await;

                Err(error_msg)
            }
        }
    }

    /// Déconnecte du réseau MWS.
    pub async fn disconnect(&self) -> Result<(), String> {
        info!("[CentralMws] Déconnexion du réseau MWS…");

        // 1. Arrêter le serveur Home
        let mut shutdown_guard = self.home_server_shutdown.write().await;
        if let Some(tx) = shutdown_guard.take() {
            let _ = tx.send(());
            info!("[CentralMws] Serveur Home arrêté");
        }
        drop(shutdown_guard);

        // 2. Envoyer WITHDRAW au Tracker puis arrêter le service MWS
        {
            let service = self.service.read().await;
            if let Some(ref svc) = *service {
                info!("[CentralMws] Envoi WITHDRAW + arrêt du service MWS…");
                if let Err(e) = svc.stop().await {
                    warn!("[CentralMws] Erreur lors de l'arrêt: {}", e);
                } else {
                    info!("[CentralMws] Service MWS arrêté (WITHDRAW envoyé)");
                }
            } else {
                warn!("[CentralMws] Pas de service MWS actif à arrêter");
            }
        }

        // 3. Mettre à jour l'état interne
        let (new_state, new_conformity) = if self.config.lone_mode {
            (CentralMwsState::Lone, MwsConformityState::LoneMode)
        } else if self.config.enabled {
            (CentralMwsState::Disconnected, MwsConformityState::Uninitialized)
        } else {
            (CentralMwsState::Disabled, MwsConformityState::Uninitialized)
        };

        self.update_state(new_state.clone()).await;
        self.update_conformity(new_conformity.clone()).await;
        info!("[CentralMws] État → {:?}, Conformité → {:?}", new_state, new_conformity);

        // 4. Effacer le service
        {
            let mut svc = self.service.write().await;
            *svc = None;
        }

        info!("[CentralMws] ✅ Déconnexion complète du réseau MWS");
        Ok(())
    }

    /// Tente la connexion automatique si configurée.
    pub async fn auto_connect_if_enabled(&self) -> Result<(), String> {
        if self.config.should_connect() && self.config.auto_connect {
            info!("Connexion automatique au réseau MWS...");
            self.connect().await
        } else if self.config.lone_mode {
            info!("Mode Lone activé — Pas de connexion automatique");
            self.update_conformity(MwsConformityState::LoneMode).await;
            self.update_state(CentralMwsState::Lone).await;
            Ok(())
        } else {
            info!("Connexion automatique désactivée");
            Ok(())
        }
    }

    /// Recherche des COGs sur le réseau.
    pub async fn search_cogs(
        &self,
        version_filter: Option<String>,
        service_filter: Option<String>,
        limit: u32,
    ) -> Result<Vec<CogInfo>, String> {
        let service = self.service.read().await;
        let svc = service.as_ref().ok_or("Non connecté au réseau MWS")?;

        svc.search_cogs(version_filter, service_filter, limit)
            .await
            .map_err(|e| e.to_string())
    }

    /// Recherche des lobbys sur le réseau.
    pub async fn search_lobbys(
        &self,
        name_filter: Option<String>,
        public_only: bool,
        limit: u32,
    ) -> Result<Vec<LobbySearchResult>, String> {
        let service = self.service.read().await;
        let svc = service.as_ref().ok_or("Non connecté au réseau MWS")?;

        svc.search_lobbys(name_filter, public_only, limit)
            .await
            .map_err(|e| e.to_string())
    }

    /// Crée un lobby.
    pub async fn create_lobby(
        &self,
        name: String,
        max_players: u32,
        is_public: bool,
        password: Option<String>,
        metadata: serde_json::Value,
    ) -> Result<String, String> {
        let service = self.service.read().await;
        let svc = service.as_ref().ok_or("Non connecté au réseau MWS")?;

        let lobby = LobbyInfo {
            lobby_id: String::new(), // Sera assigné par le serveur
            name: name.clone(),
            is_public,
            password_required: password.is_some(),
            max_players,
            current_players: 1,
            metadata,
        };

        let lobby_id = svc.create_lobby(lobby.clone(), password)
            .await
            .map_err(|e| e.to_string())?;

        // Ajouter à la liste locale
        {
            let mut lobbys = self.lobbys.write().await;
            let mut new_lobby = lobby;
            new_lobby.lobby_id = lobby_id.clone();
            lobbys.push(new_lobby);
        }

        info!("Lobby '{}' créé avec ID: {}", name, lobby_id);
        Ok(lobby_id)
    }

    /// Retourne l'état simplifié.
    pub async fn get_state(&self) -> CentralMwsState {
        self.state.read().await.clone()
    }

    /// Retourne l'état de conformité détaillé.
    pub async fn get_conformity_state(&self) -> MwsConformityState {
        self.conformity_state.read().await.clone()
    }

    /// Vérifie si connecté (conformité complète).
    pub async fn is_connected(&self) -> bool {
        *self.state.read().await == CentralMwsState::Connected
    }

    /// Vérifie si le COG est présent sur le réseau.
    pub async fn is_online(&self) -> bool {
        self.conformity_state.read().await.is_online()
    }

    /// Vérifie si le COG est en mode Lone.
    pub async fn is_lone(&self) -> bool {
        self.config.lone_mode
    }

    /// Retourne les lobbys hébergés.
    pub async fn get_hosted_lobbys(&self) -> Vec<LobbyInfo> {
        self.lobbys.read().await.clone()
    }

    /// Retourne les services exposés.
    pub async fn get_services(&self) -> Vec<String> {
        self.services.read().await.clone()
    }

    /// Retourne l'ID du COG.
    pub fn get_cog_id(&self) -> &str {
        &self.cog_id
    }

    /// Retourne la version des Cores.
    pub fn get_core_version(&self) -> &str {
        &self.core_version
    }

    /// Retourne la configuration.
    pub fn get_config(&self) -> &CentralMwsConfig {
        &self.config
    }

    /// Met à jour la configuration.
    pub fn update_config(&mut self, config: CentralMwsConfig) {
        self.config = config;
    }

    /// Ajoute un service exposé.
    pub async fn add_service(&self, service_id: String) {
        let mut services = self.services.write().await;
        if !services.contains(&service_id) {
            services.push(service_id.clone());
            info!("Service '{}' ajouté aux services exposés", service_id);
        }
    }

    /// Retire un service exposé.
    pub async fn remove_service(&self, service_id: &str) {
        let mut services = self.services.write().await;
        services.retain(|s| s != service_id);
        info!("Service '{}' retiré des services exposés", service_id);
    }

    /// Active le mode Lone (déconnecte du réseau).
    pub async fn enable_lone_mode(&mut self) -> Result<(), String> {
        if self.is_connected().await {
            self.disconnect().await?;
        }
        self.config.lone_mode = true;
        self.config.enabled = false;
        self.update_conformity(MwsConformityState::LoneMode).await;
        self.update_state(CentralMwsState::Lone).await;
        info!("Mode Lone activé — COG isolé du réseau MWS");
        Ok(())
    }

    /// Désactive le mode Lone.
    pub async fn disable_lone_mode(&mut self) {
        self.config.lone_mode = false;
        self.config.enabled = true;
        self.update_conformity(MwsConformityState::Uninitialized).await;
        self.update_state(CentralMwsState::Disconnected).await;
        info!("Mode Lone désactivé — COG prêt à rejoindre le réseau MWS");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// INFORMATIONS RÉSEAU
// ═══════════════════════════════════════════════════════════════════════════════

/// Informations réseau MWS pour l'UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MwsNetworkInfo {
    /// État simplifié.
    pub state: CentralMwsState,
    /// État de conformité détaillé.
    pub conformity: MwsConformityState,
    /// Étape actuelle (0-10).
    pub conformity_step: u8,
    /// Description de l'étape.
    pub conformity_description: String,
    /// Mode Lone actif.
    pub is_lone: bool,
    /// Nombre de COGs visibles.
    pub visible_cogs: u32,
    /// Nombre de lobbys visibles.
    pub visible_lobbys: u32,
    /// Adresse du Relay.
    pub relay_address: String,
    /// Adresse du Tracker.
    pub tracker_address: String,
    /// Services exposés.
    pub services: Vec<String>,
    /// ID du COG.
    pub cog_id: String,
    /// Version des Cores.
    pub core_version: String,
}

impl MwsNetworkInfo {
    /// Crée une info réseau depuis le gestionnaire.
    pub async fn from_manager(manager: &CentralMwsManager) -> Self {
        let state = manager.get_state().await;
        let conformity = manager.get_conformity_state().await;
        let services = manager.get_services().await;

        Self {
            state,
            conformity_step: conformity.step_number(),
            conformity_description: conformity.description().to_string(),
            conformity,
            is_lone: manager.is_lone().await,
            visible_cogs: 0,    // TODO: mettre à jour via polling
            visible_lobbys: 0,  // TODO: mettre à jour via polling
            relay_address: manager.config.relay_address.clone(),
            tracker_address: manager.config.tracker_address.clone(),
            services,
            cog_id: manager.cog_id.clone(),
            core_version: manager.core_version.clone(),
        }
    }
}

/// Résumé de l'état MWS pour l'affichage rapide.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MwsStatusSummary {
    /// Icône d'état.
    pub icon: String,
    /// Couleur (hex).
    pub color: String,
    /// Texte court.
    pub short_text: String,
    /// Texte détaillé.
    pub detail_text: String,
    /// Peut se connecter.
    pub can_connect: bool,
    /// Peut se déconnecter.
    pub can_disconnect: bool,
}

impl MwsStatusSummary {
    /// Génère un résumé depuis l'état.
    pub fn from_state(state: &CentralMwsState, conformity: &MwsConformityState) -> Self {
        match state {
            CentralMwsState::Disabled => Self {
                icon: "⚫".to_string(),
                color: "#6b7280".to_string(),
                short_text: "Désactivé".to_string(),
                detail_text: "MWS est désactivé dans la configuration".to_string(),
                can_connect: false,
                can_disconnect: false,
            },
            CentralMwsState::Lone => Self {
                icon: "🏝️".to_string(),
                color: "#f59e0b".to_string(),
                short_text: "Mode Lone".to_string(),
                detail_text: "COG isolé — Pas de connexion réseau".to_string(),
                can_connect: false,
                can_disconnect: false,
            },
            CentralMwsState::Disconnected => Self {
                icon: "🔌".to_string(),
                color: "#6b7280".to_string(),
                short_text: "Déconnecté".to_string(),
                detail_text: "Non connecté au réseau MWS".to_string(),
                can_connect: true,
                can_disconnect: false,
            },
            CentralMwsState::Connecting => Self {
                icon: "🔄".to_string(),
                color: "#f59e0b".to_string(),
                short_text: "Connexion...".to_string(),
                detail_text: conformity.description().to_string(),
                can_connect: false,
                can_disconnect: true,
            },
            CentralMwsState::RelayConnected => Self {
                icon: "📡".to_string(),
                color: "#06b6d4".to_string(),
                short_text: "Relay OK".to_string(),
                detail_text: "Connecté au Relay, annonce au Tracker...".to_string(),
                can_connect: false,
                can_disconnect: true,
            },
            CentralMwsState::Connected => Self {
                icon: "🌐".to_string(),
                color: "#10b981".to_string(),
                short_text: "En ligne".to_string(),
                detail_text: "Conformité complète — Présent sur le réseau".to_string(),
                can_connect: false,
                can_disconnect: true,
            },
            CentralMwsState::Error(msg) => Self {
                icon: "❌".to_string(),
                color: "#ef4444".to_string(),
                short_text: "Erreur".to_string(),
                detail_text: msg.clone(),
                can_connect: true,
                can_disconnect: false,
            },
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SERVEUR HOME COG (layout commun, exposition automatique quand annoncé)
// ═══════════════════════════════════════════════════════════════════════════════

/// Informations pour afficher la carte JayXpose sur la Home (lien « Découvrir »).
#[derive(Clone)]
struct HomeJayXposeInfo {
    base_url: String,
    slug: String,
}

/// Lance le mini serveur HTTP qui sert la page Home du COG (GET /).
/// S'arrête quand `shutdown` reçoit un signal.
async fn run_home_server(
    bind_addr: String,
    cog_id: String,
    core_version: String,
    services: Vec<String>,
    home_jayxpose: Option<HomeJayXposeInfo>,
    mut shutdown: oneshot::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(&bind_addr).await?;
    info!("Serveur Home COG en écoute sur http://{}", bind_addr);

    loop {
        tokio::select! {
            _ = &mut shutdown => {
                info!("Serveur Home arrêté (déconnexion MWS)");
                break;
            }
            accept_result = listener.accept() => {
                let (stream, addr) = match accept_result {
                    Ok(x) => x,
                    Err(e) => {
                        warn!("Accept Home: {}", e);
                        continue;
                    }
                };
                info!("[Home Server] Connexion entrante depuis {}", addr);
                let cog_id = cog_id.clone();
                let core_version = core_version.clone();
                let services = services.clone();
                let home_jayxpose = home_jayxpose.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_home_connection(stream, cog_id, core_version, services, home_jayxpose).await {
                        warn!("Home connection: {}", e);
                    }
                });
            }
        }
    }
    Ok(())
}

async fn handle_home_connection(
    mut stream: tokio::net::TcpStream,
    cog_id: String,
    core_version: String,
    services: Vec<String>,
    home_jayxpose: Option<HomeJayXposeInfo>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use tokio::io::{AsyncBufReadExt, BufReader};

    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut first_line = String::new();
    reader.read_line(&mut first_line).await?;

    let is_get_root = first_line.trim().starts_with("GET / ") || first_line.trim() == "GET /";

    // Consommer le reste des en-têtes
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        if line.trim().is_empty() {
            break;
        }
    }

    let (status, body) = if is_get_root {
        (
            "200 OK",
            home_page_html(&cog_id, &core_version, &services, home_jayxpose.as_ref()),
        )
    } else {
        (
            "404 Not Found",
            r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>404</title></head><body><h1>404</h1><p>Page non trouvée.</p></body></html>"#.to_string(),
        )
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status,
        body.len(),
        body
    );
    writer.write_all(response.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

/// Génère la page Home COG (layout commun à tous les COGs).
fn home_page_html(cog_id: &str, core_version: &str, services: &[String], home_jayxpose: Option<&HomeJayXposeInfo>) -> String {
    let services_list: String = if services.is_empty() {
        "<li>Aucun service exposé</li>".to_string()
    } else {
        services
            .iter()
            .map(|s| format!("<li>{}</li>", html_escape(s)))
            .collect()
    };

    let jayxpose_card: String = if let Some(j) = home_jayxpose {
        let url = format!("{}/vitrine/{}", j.base_url.trim_end_matches('/'), j.slug);
        format!(
            r##"
        <div class="card card-service">
            <h2>JayXpose</h2>
            <p class="card-desc">Vitrine et catalogue — Découvrez notre présentation et nos produits.</p>
            <a href="{url}" class="btn-discover" target="_blank" rel="noopener">Découvrir</a>
        </div>"##,
            url = html_escape(&url),
        )
    } else {
        String::new()
    };

    format!(
        r##"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Home — {cog_id}</title>
    <style>
        :root {{ --primary: #8b5cf6; --bg: #0a0a0f; --bg-surface: #12121a; --text: #f0f0f5; --text-muted: #9ca3af; --border: rgba(139, 92, 246, 0.2); }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{ font-family: system-ui, sans-serif; background: var(--bg); color: var(--text); line-height: 1.6; padding: 2rem; }}
        .container {{ max-width: 720px; margin: 0 auto; }}
        h1 {{ font-size: 1.75rem; margin-bottom: 0.5rem; color: var(--primary); }}
        .subtitle {{ color: var(--text-muted); margin-bottom: 1.5rem; font-size: 0.9rem; }}
        .card {{ background: var(--bg-surface); border: 1px solid var(--border); border-radius: 0.75rem; padding: 1.25rem; margin-bottom: 1rem; }}
        .card h2 {{ font-size: 1.1rem; margin-bottom: 0.75rem; color: var(--text-muted); }}
        .card-desc {{ font-size: 0.9rem; color: var(--text); margin-bottom: 1rem; }}
        .btn-discover {{ display: inline-block; background: var(--primary); color: #fff; padding: 8px 16px; border-radius: 6px; text-decoration: none; font-weight: 600; }}
        .btn-discover:hover {{ opacity: 0.9; }}
        ul {{ list-style: none; }}
        ul li {{ padding: 0.25rem 0; }}
        .badge {{ display: inline-block; background: var(--primary); color: #fff; font-size: 0.7rem; padding: 2px 8px; border-radius: 4px; margin-top: 1rem; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🌐 {cog_id}</h1>
        <p class="subtitle">COG Miyukini — Cores {core_version} · Point d'entrée web</p>
        <div class="card">
            <h2>Services disponibles</h2>
            <ul>{services_list}</ul>
        </div>{jayxpose_card}
        <span class="badge">Miyukini Webway System</span>
    </div>
</body>
</html>"##,
        cog_id = html_escape(cog_id),
        core_version = html_escape(core_version),
        services_list = services_list,
        jayxpose_card = jayxpose_card,
    )
}

fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CentralMwsConfig::default();
        assert!(config.enabled);
        assert!(!config.lone_mode);
        assert!(config.auto_connect);
        assert!(config.auto_heartbeat);
        assert!(config.present_without_services);
    }

    #[test]
    fn test_lone_config() {
        let config = CentralMwsConfig::lone();
        assert!(!config.enabled);
        assert!(config.lone_mode);
        assert!(!config.auto_connect);
    }

    #[test]
    fn test_should_connect() {
        let default_config = CentralMwsConfig::default();
        assert!(default_config.should_connect());

        let lone_config = CentralMwsConfig::lone();
        assert!(!lone_config.should_connect());
    }

    #[test]
    fn test_conformity_state_steps() {
        assert_eq!(MwsConformityState::Uninitialized.step_number(), 0);
        assert_eq!(MwsConformityState::ConnectingRelay.step_number(), 2);
        assert_eq!(MwsConformityState::FullyConformant.step_number(), 10);
    }

    #[tokio::test]
    async fn test_manager_creation() {
        let manager = CentralMwsManager::with_defaults(
            "test-cog".to_string(),
            "1.0.0".to_string(),
        );

        assert_eq!(manager.get_state().await, CentralMwsState::Disconnected);
        assert!(!manager.is_connected().await);
        assert!(!manager.is_lone().await);
    }

    #[tokio::test]
    async fn test_lone_manager() {
        let manager = CentralMwsManager::lone(
            "lone-cog".to_string(),
            "1.0.0".to_string(),
        );

        assert_eq!(manager.get_state().await, CentralMwsState::Lone);
        assert!(manager.is_lone().await);
        assert!(!manager.is_connected().await);
    }

    #[tokio::test]
    async fn test_service_management() {
        let manager = CentralMwsManager::with_defaults(
            "test-cog".to_string(),
            "1.0.0".to_string(),
        );

        manager.add_service("jayfestival".to_string()).await;
        manager.add_service("jayxpose".to_string()).await;

        let services = manager.get_services().await;
        assert_eq!(services.len(), 2);
        assert!(services.contains(&"jayfestival".to_string()));

        manager.remove_service("jayfestival").await;
        let services = manager.get_services().await;
        assert_eq!(services.len(), 1);
    }

    #[test]
    fn test_status_summary() {
        let summary = MwsStatusSummary::from_state(
            &CentralMwsState::Connected,
            &MwsConformityState::FullyConformant,
        );
        assert_eq!(summary.icon, "🌐");
        assert!(summary.can_disconnect);
        assert!(!summary.can_connect);

        let lone_summary = MwsStatusSummary::from_state(
            &CentralMwsState::Lone,
            &MwsConformityState::LoneMode,
        );
        assert_eq!(lone_summary.icon, "🏝️");
        assert!(!lone_summary.can_connect);
        assert!(!lone_summary.can_disconnect);
    }
}
