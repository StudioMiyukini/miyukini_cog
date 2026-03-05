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

use jayxpose::data::JayXposeDb;
use miyuwebway_participant::{
    CogIdentity, CogInfo, LobbyInfo, LobbySearchResult, MwsService, MwsServiceConfig,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::{oneshot, RwLock};
use tokio::task::JoinHandle;
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
    /// Slug de sous-domaine personnalisé (optionnel).
    /// Si fourni, le COG sera accessible via `<slug>.miyukini.com`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdomain_slug: Option<String>,
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
            home_http_bind: None,           // Optionnel : "0.0.0.0:8080" pour exposer la Home
            expose_jayxpose_vitrine: false,
            jayxpose_vitrine_base_url: None,
            subdomain_slug: None,
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
    /// Handle de la tâche heartbeat (annulée à la déconnexion ou au drop).
    heartbeat_handle: Mutex<Option<JoinHandle<()>>>,
    /// Base de données JayXpose pour servir les pages vitrine localement.
    jayxpose_db: Option<Arc<JayXposeDb>>,
}

impl CentralMwsManager {
    /// Crée un nouveau gestionnaire MWS.
    /// `jayxpose_vitrine_slug` : slug de la vitrine publiée à afficher sur la Home (optionnel).
    /// `jayxpose_db` : référence à la base JayXpose pour servir les pages vitrine localement.
    pub fn new(
        config: CentralMwsConfig,
        cog_id: String,
        core_version: String,
        services: Vec<String>,
        jayxpose_vitrine_slug: Option<String>,
        jayxpose_db: Option<Arc<JayXposeDb>>,
    ) -> Self {
        let (state, conformity) = if config.lone_mode {
            (CentralMwsState::Lone, MwsConformityState::LoneMode)
        } else if config.enabled {
            (
                CentralMwsState::Disconnected,
                MwsConformityState::Uninitialized,
            )
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
            heartbeat_handle: Mutex::new(None),
            jayxpose_db,
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

        info!(
            "Démarrage du protocole de conformité MWS pour COG {}...",
            self.cog_id
        );
        self.update_state(CentralMwsState::Connecting).await;

        // Étape 1: Résolution Origin
        self.update_conformity(MwsConformityState::ResolvingOrigin)
            .await;

        // Vérifier que l'adresse est valide
        let relay_host = self
            .config
            .relay_address
            .split(':')
            .next()
            .ok_or("Adresse Relay invalide")?;
        info!("Origin résolu: {}", relay_host);

        // Étape 2: Connexion au Relay
        self.update_conformity(MwsConformityState::ConnectingRelay)
            .await;

        // Créer la configuration MWS
        let mut mws_config = MwsServiceConfig::default();
        mws_config.relay.relay_address = self.config.relay_address.clone();
        mws_config.relay.tls_domain = relay_host.to_string();
        mws_config.relay.core_conformity_key = self.config.core_conformity_key.clone();
        mws_config.tracker.tracker_address = self.config.tracker_address.clone();
        mws_config.auto_heartbeat = self.config.auto_heartbeat;
        mws_config.auto_reconnect = self.config.auto_reconnect;
        mws_config.subdomain_slug = self.config.subdomain_slug.clone();
        // Phase 2 : adresse pour le forwarding HTTP via tunnel (0.0.0.0 → 127.0.0.1)
        mws_config.home_http_bind = self.config.home_http_bind.as_ref().map(|addr| {
            if addr.starts_with("0.0.0.0:") {
                format!(
                    "127.0.0.1:{}",
                    addr.strip_prefix("0.0.0.0:").unwrap_or("8080")
                )
            } else {
                addr.clone()
            }
        });

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
        self.update_conformity(MwsConformityState::RegisteringRelay)
            .await;
        self.update_conformity(MwsConformityState::AwaitingRelayAck)
            .await;

        // Démarrer le service (inclut connexion Relay + annonce Tracker)
        match service.start(identity).await {
            Ok(_) => {
                self.update_conformity(MwsConformityState::RelaySessionEstablished)
                    .await;
                self.update_conformity(MwsConformityState::ObtainingPermis)
                    .await;
                self.update_state(CentralMwsState::RelayConnected).await;

                // Étape 7-9: Annonce au Tracker
                self.update_conformity(MwsConformityState::ConnectingTracker)
                    .await;
                self.update_conformity(MwsConformityState::AnnouncingTracker)
                    .await;
                self.update_conformity(MwsConformityState::AwaitingTrackerAck)
                    .await;

                // Stocker le service
                {
                    let mut svc = self.service.write().await;
                    *svc = Some(service);
                }

                // Démarrer le heartbeat Central → Tracker pour maintenir la connexion.
                // Une seule tâche par manager ; intervalle 30s (seuil Absent Origin 90s, TTL 300s).
                let service_ref = Arc::clone(&self.service);
                let cog_id_log = self.cog_id.clone();
                const HEARTBEAT_INTERVAL_SECS: u64 = 30;
                info!(
                    "Démarrage heartbeat Central → Tracker (interval={}s, cog={})",
                    HEARTBEAT_INTERVAL_SECS, &cog_id_log
                );
                let handle = tokio::spawn(async move {
                    let mut count: u64 = 0;
                    loop {
                        sleep(Duration::from_secs(HEARTBEAT_INTERVAL_SECS)).await;
                        count += 1;
                        let guard = service_ref.read().await;
                        let Some(ref svc) = *guard else {
                            warn!(
                                "[Heartbeat #{}] Service MWS disparu, arrêt du heartbeat (COG {})",
                                count, cog_id_log
                            );
                            break;
                        };
                        match svc.send_heartbeat().await {
                            Ok(()) => {
                                info!(
                                    "[Heartbeat #{} OK] Central → Tracker (COG {})",
                                    count, cog_id_log
                                );
                            }
                            Err(e) => {
                                warn!(
                                    "[Heartbeat #{} ERREUR] Central → Tracker (COG {}): {}",
                                    count, cog_id_log, e
                                );
                            }
                        }
                    }
                });
                if let Ok(mut guard) = self.heartbeat_handle.lock() {
                    *guard = Some(handle);
                }

                // Conformité complète
                self.update_conformity(MwsConformityState::FullyConformant)
                    .await;
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
                    let jayxpose_db = self.jayxpose_db.clone();
                    let (tx, rx) = oneshot::channel();
                    let bind_for_server = bind_addr.clone();
                    let bind_for_log = bind_addr.clone();
                    tokio::spawn(async move {
                        match run_home_server(
                            bind_for_server.clone(),
                            cog_id,
                            core_version,
                            services,
                            home_jayxpose,
                            jayxpose_db,
                            rx,
                        )
                        .await
                        {
                            Ok(()) => info!("Serveur Home arrêté proprement"),
                            Err(e) => {
                                error!("❌ Serveur Home ERREUR (bind={}): {}", bind_for_server, e)
                            }
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

                self.update_conformity(MwsConformityState::ConformityError(error_msg.clone()))
                    .await;
                self.update_state(CentralMwsState::Error(error_msg.clone()))
                    .await;

                Err(error_msg)
            }
        }
    }

    /// Déconnecte du réseau MWS.
    pub async fn disconnect(&self) -> Result<(), String> {
        info!("[CentralMws] Déconnexion du réseau MWS…");

        // 0. Arrêter la tâche heartbeat (évite tâches orphelines et doublons)
        if let Ok(mut guard) = self.heartbeat_handle.lock() {
            if let Some(handle) = guard.take() {
                handle.abort();
                info!("[CentralMws] Tâche heartbeat arrêtée");
            }
        }

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
            (
                CentralMwsState::Disconnected,
                MwsConformityState::Uninitialized,
            )
        } else {
            (CentralMwsState::Disabled, MwsConformityState::Uninitialized)
        };

        self.update_state(new_state.clone()).await;
        self.update_conformity(new_conformity.clone()).await;
        info!(
            "[CentralMws] État → {:?}, Conformité → {:?}",
            new_state, new_conformity
        );

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

        let lobby_id = svc
            .create_lobby(lobby.clone(), password)
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

    /// Met à jour les services exposés et envoie un RE-ANNOUNCE au Tracker.
    ///
    /// Cette méthode permet de modifier dynamiquement la liste des services
    /// annoncés au réseau MWS. Si connecté, envoie immédiatement la mise à jour
    /// au Tracker. Sinon, met seulement à jour la liste locale.
    ///
    /// # Arguments
    /// * `services` - Nouvelle liste complète des services à annoncer
    ///
    /// # Returns
    /// * `Ok(())` si la mise à jour a réussi
    /// * `Err(String)` si une erreur s'est produite
    pub async fn update_services(&self, services: Vec<String>) -> Result<(), String> {
        // Mettre à jour la liste locale
        {
            let mut local_services = self.services.write().await;
            *local_services = services.clone();
        }
        info!("Services locaux mis à jour: {:?}", services);

        // Si connecté, envoyer le RE-ANNOUNCE au Tracker
        let service = self.service.read().await;
        if let Some(ref svc) = *service {
            svc.update_services(services)
                .await
                .map_err(|e| format!("Erreur RE-ANNOUNCE: {}", e))?;
        } else {
            info!("Non connecté au MWS — mise à jour locale uniquement");
        }

        Ok(())
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
        self.update_conformity(MwsConformityState::Uninitialized)
            .await;
        self.update_state(CentralMwsState::Disconnected).await;
        info!("Mode Lone désactivé — COG prêt à rejoindre le réseau MWS");
    }
}

impl Drop for CentralMwsManager {
    /// Annule la tâche heartbeat si le manager est drop sans disconnect (ex. remplacement par un nouveau manager).
    fn drop(&mut self) {
        if let Ok(mut guard) = self.heartbeat_handle.lock() {
            if let Some(handle) = guard.take() {
                handle.abort();
            }
        }
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
            visible_cogs: 0,   // TODO: mettre à jour via polling
            visible_lobbys: 0, // TODO: mettre à jour via polling
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
#[allow(dead_code)]
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
    jayxpose_db: Option<Arc<JayXposeDb>>,
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
                let jayxpose_db = jayxpose_db.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_home_connection(stream, cog_id, core_version, services, home_jayxpose, jayxpose_db).await {
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
    jayxpose_db: Option<Arc<JayXposeDb>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use tokio::io::{AsyncBufReadExt, BufReader};

    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut first_line = String::new();
    reader.read_line(&mut first_line).await?;

    // Parser la requête: "GET /path HTTP/1.x"
    let parts: Vec<&str> = first_line.trim().split_whitespace().collect();
    let path = if parts.len() >= 2 && parts[0] == "GET" {
        parts[1]
    } else {
        "/"
    };

    // Consommer le reste des en-têtes
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        if line.trim().is_empty() {
            break;
        }
    }

    // Routage
    let (status, body) = route_request(
        path,
        &cog_id,
        &core_version,
        &services,
        home_jayxpose.as_ref(),
        jayxpose_db.as_ref(),
    );

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

/// Route les requêtes vers les handlers appropriés.
fn route_request(
    path: &str,
    cog_id: &str,
    core_version: &str,
    services: &[String],
    home_jayxpose: Option<&HomeJayXposeInfo>,
    jayxpose_db: Option<&Arc<JayXposeDb>>,
) -> (&'static str, String) {
    // Normaliser le path (enlever query string éventuelle)
    let path = path.split('?').next().unwrap_or("/");
    let path = path.trim_end_matches('/');
    let path = if path.is_empty() { "/" } else { path };

    match path {
        "/" => (
            "200 OK",
            home_portal_html(cog_id, core_version, services, home_jayxpose),
        ),
        "/jayxpose" => {
            if let Some(db) = jayxpose_db {
                if let Some(html) = jayxpose_home_html(db) {
                    ("200 OK", html)
                } else {
                    ("404 Not Found", not_found_html("Aucune vitrine publiée"))
                }
            } else {
                (
                    "503 Service Unavailable",
                    not_found_html("Service JayXpose non disponible"),
                )
            }
        }
        "/jayxpose/catalogue" => {
            if let Some(db) = jayxpose_db {
                if let Some(html) = jayxpose_catalogue_html(db) {
                    ("200 OK", html)
                } else {
                    ("404 Not Found", not_found_html("Catalogue non disponible"))
                }
            } else {
                (
                    "503 Service Unavailable",
                    not_found_html("Service JayXpose non disponible"),
                )
            }
        }
        "/jayxpose/contact" => {
            if let Some(db) = jayxpose_db {
                if let Some(html) = jayxpose_contact_html(db) {
                    ("200 OK", html)
                } else {
                    (
                        "404 Not Found",
                        not_found_html("Page contact non disponible"),
                    )
                }
            } else {
                (
                    "503 Service Unavailable",
                    not_found_html("Service JayXpose non disponible"),
                )
            }
        }
        _ if path.starts_with("/jayxpose/catalogue/") => {
            let product_id = path.strip_prefix("/jayxpose/catalogue/").unwrap_or("");
            if let Some(db) = jayxpose_db {
                if let Some(html) = jayxpose_product_html(db, product_id) {
                    ("200 OK", html)
                } else {
                    ("404 Not Found", not_found_html("Produit non trouvé"))
                }
            } else {
                (
                    "503 Service Unavailable",
                    not_found_html("Service JayXpose non disponible"),
                )
            }
        }
        _ => ("404 Not Found", not_found_html("Page non trouvée")),
    }
}

/// Page 404.
fn not_found_html(message: &str) -> String {
    format!(
        r#"<!DOCTYPE html><html lang="fr"><head><meta charset="UTF-8"><title>404</title>
<style>body {{ font-family: system-ui, sans-serif; background: #0a0a0f; color: #f0f0f5; display: flex; justify-content: center; align-items: center; min-height: 100vh; margin: 0; }}
.error {{ text-align: center; }} h1 {{ color: #8b5cf6; }}</style>
</head><body><div class="error"><h1>404</h1><p>{}</p><a href="/" style="color: #8b5cf6;">Retour à l'accueil</a></div></body></html>"#,
        html_escape(message)
    )
}

/// Génère la page portail du COG avec liens vers les services locaux.
fn home_portal_html(
    cog_id: &str,
    core_version: &str,
    services: &[String],
    _home_jayxpose: Option<&HomeJayXposeInfo>,
) -> String {
    // Génère des cartes pour chaque service exposé avec lien local
    let services_cards: String = if services.is_empty() {
        r#"<p class="text-muted">Aucun service exposé pour le moment.</p>"#.to_string()
    } else {
        services
            .iter()
            .map(|s| {
                let (title, desc, icon, path) = match s.as_str() {
                    "jayxpose" => (
                        "JayXpose",
                        "Vitrine et catalogue — Découvrez notre présentation et nos produits.",
                        "🛍️",
                        "/jayxpose",
                    ),
                    "jaybooking" => (
                        "JayBooking",
                        "Réservations et planification — Prenez rendez-vous facilement.",
                        "📅",
                        "/jaybooking",
                    ),
                    "jayfestival" => (
                        "JayFestival",
                        "Événements et festivals — Découvrez nos événements.",
                        "🎉",
                        "/jayfestival",
                    ),
                    _ => (s.as_str(), "Service disponible", "📦", "#"),
                };
                format!(
                    r#"<a href="{path}" class="card card-service">
                        <div class="card-icon">{icon}</div>
                        <div class="card-content">
                            <h3>{title}</h3>
                            <p class="card-desc">{desc}</p>
                        </div>
                        <span class="card-arrow">→</span>
                    </a>"#,
                    path = html_escape(path),
                    icon = icon,
                    title = html_escape(title),
                    desc = html_escape(desc),
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        r##"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{cog_id} — Portail</title>
    <style>
        :root {{ --primary: #8b5cf6; --primary-hover: #7c3aed; --bg: #0a0a0f; --bg-surface: #12121a; --bg-hover: #1a1a24; --text: #f0f0f5; --text-muted: #9ca3af; --border: rgba(139, 92, 246, 0.2); }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{ font-family: system-ui, -apple-system, sans-serif; background: var(--bg); color: var(--text); line-height: 1.6; min-height: 100vh; }}
        .hero {{ background: linear-gradient(135deg, var(--bg-surface), var(--bg)); padding: 3rem 2rem; text-align: center; border-bottom: 1px solid var(--border); }}
        .hero h1 {{ font-size: 2rem; margin-bottom: 0.5rem; color: var(--primary); }}
        .hero .subtitle {{ color: var(--text-muted); font-size: 1rem; }}
        .container {{ max-width: 800px; margin: 0 auto; padding: 2rem; }}
        .section-title {{ font-size: 1.25rem; color: var(--text-muted); margin-bottom: 1rem; font-weight: 500; }}
        .services-grid {{ display: flex; flex-direction: column; gap: 1rem; }}
        .card {{ background: var(--bg-surface); border: 1px solid var(--border); border-radius: 0.75rem; padding: 1.25rem 1.5rem; text-decoration: none; color: inherit; display: flex; align-items: center; gap: 1rem; transition: all 0.2s ease; }}
        .card:hover {{ background: var(--bg-hover); border-color: var(--primary); transform: translateX(4px); }}
        .card-icon {{ font-size: 2rem; }}
        .card-content {{ flex: 1; }}
        .card h3 {{ font-size: 1.1rem; margin-bottom: 0.25rem; color: var(--text); }}
        .card-desc {{ font-size: 0.875rem; color: var(--text-muted); margin: 0; }}
        .card-arrow {{ font-size: 1.25rem; color: var(--primary); opacity: 0; transition: opacity 0.2s; }}
        .card:hover .card-arrow {{ opacity: 1; }}
        .text-muted {{ color: var(--text-muted); }}
        .badge {{ display: inline-block; background: var(--primary); color: #fff; font-size: 0.7rem; padding: 4px 10px; border-radius: 4px; margin-top: 2rem; }}
        .footer {{ text-align: center; padding: 2rem; color: var(--text-muted); font-size: 0.8rem; }}
    </style>
</head>
<body>
    <div class="hero">
        <h1>🌐 {cog_id}</h1>
        <p class="subtitle">COG Miyukini — Cores {core_version}</p>
    </div>
    <div class="container">
        <h2 class="section-title">Services disponibles</h2>
        <div class="services-grid">
            {services_cards}
        </div>
        <span class="badge">Miyukini Webway System</span>
    </div>
    <div class="footer">
        <p>Propulsé par Miyukini COG · Réseau décentralisé souverain</p>
    </div>
</body>
</html>"##,
        cog_id = html_escape(cog_id),
        core_version = html_escape(core_version),
        services_cards = services_cards,
    )
}

// ═══════════════════════════════════════════════════════════════════════════════
// PAGES JAYXPOSE (vitrine locale)
// ═══════════════════════════════════════════════════════════════════════════════

/// Layout commun pour les pages JayXpose locales.
fn jayxpose_layout(title: &str, content: &str, active_page: &str) -> String {
    let nav_home_class = if active_page == "home" {
        "nav-link active"
    } else {
        "nav-link"
    };
    let nav_catalogue_class = if active_page == "catalogue" {
        "nav-link active"
    } else {
        "nav-link"
    };
    let nav_contact_class = if active_page == "contact" {
        "nav-link active"
    } else {
        "nav-link"
    };

    format!(
        r##"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>
        :root {{ --primary: #8b5cf6; --primary-hover: #7c3aed; --bg: #0a0a0f; --bg-surface: #12121a; --bg-hover: #1a1a24; --text: #f0f0f5; --text-muted: #9ca3af; --border: rgba(139, 92, 246, 0.2); --success: #10b981; }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{ font-family: system-ui, -apple-system, sans-serif; background: var(--bg); color: var(--text); line-height: 1.6; min-height: 100vh; }}
        header {{ background: var(--bg-surface); border-bottom: 1px solid var(--border); padding: 1rem 2rem; display: flex; justify-content: space-between; align-items: center; }}
        .logo {{ font-size: 1.25rem; font-weight: 700; color: var(--primary); text-decoration: none; }}
        nav {{ display: flex; gap: 0.5rem; }}
        .nav-link {{ padding: 0.5rem 1rem; border-radius: 6px; text-decoration: none; color: var(--text-muted); transition: all 0.2s; }}
        .nav-link:hover {{ color: var(--text); background: var(--bg-hover); }}
        .nav-link.active {{ color: var(--primary); background: rgba(139, 92, 246, 0.1); }}
        .container {{ max-width: 1000px; margin: 0 auto; padding: 2rem; }}
        .hero {{ background: linear-gradient(135deg, var(--bg-surface), var(--bg)); padding: 3rem 2rem; text-align: center; border-bottom: 1px solid var(--border); }}
        .hero h1 {{ font-size: 2rem; margin-bottom: 0.5rem; }}
        .hero p {{ color: var(--text-muted); font-size: 1.1rem; }}
        h2 {{ font-size: 1.5rem; margin: 2rem 0 1rem; color: var(--text); }}
        .products-grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 1.5rem; }}
        .product-card {{ background: var(--bg-surface); border: 1px solid var(--border); border-radius: 0.75rem; padding: 1.5rem; text-decoration: none; color: inherit; transition: all 0.2s; display: block; }}
        .product-card:hover {{ border-color: var(--primary); transform: translateY(-2px); }}
        .product-card h3 {{ font-size: 1.1rem; margin-bottom: 0.5rem; }}
        .product-price {{ font-size: 1.25rem; font-weight: 600; color: var(--primary); margin: 0.5rem 0; }}
        .product-avail {{ font-size: 0.8rem; color: var(--success); }}
        .product-avail.rupture {{ color: #ef4444; }}
        .back-link {{ display: inline-block; margin-bottom: 1rem; color: var(--primary); text-decoration: none; }}
        .back-link:hover {{ text-decoration: underline; }}
        .contact-info {{ background: var(--bg-surface); border: 1px solid var(--border); border-radius: 0.75rem; padding: 2rem; }}
        .contact-info h3 {{ margin-bottom: 1rem; }}
        .contact-item {{ display: flex; gap: 1rem; margin-bottom: 1rem; align-items: center; }}
        .contact-item span {{ color: var(--text-muted); }}
        .footer {{ text-align: center; padding: 2rem; color: var(--text-muted); font-size: 0.8rem; border-top: 1px solid var(--border); margin-top: 3rem; }}
        .text-muted {{ color: var(--text-muted); }}
    </style>
</head>
<body>
    <header>
        <a href="/" class="logo">← Portail COG</a>
        <nav>
            <a href="/jayxpose" class="{nav_home_class}">Accueil</a>
            <a href="/jayxpose/catalogue" class="{nav_catalogue_class}">Catalogue</a>
            <a href="/jayxpose/contact" class="{nav_contact_class}">Contact</a>
        </nav>
    </header>
    {content}
    <div class="footer">
        <p>Vitrine propulsée par JayXpose · Miyukini COG</p>
    </div>
</body>
</html>"##,
        title = html_escape(title),
        content = content,
        nav_home_class = nav_home_class,
        nav_catalogue_class = nav_catalogue_class,
        nav_contact_class = nav_contact_class,
    )
}

/// Page d'accueil JayXpose (vitrine publiée).
fn jayxpose_home_html(db: &JayXposeDb) -> Option<String> {
    // Récupérer l'exposant avec vitrine publiée
    let exposants = db.exposants_list_annuaire().ok()?;
    let exposant = exposants
        .into_iter()
        .find(|e| e.vitrine_status.as_deref() == Some("publiee"))?;

    let company_name = exposant.company_name.as_deref().unwrap_or("Vitrine");
    let slogan = exposant.slogan.as_deref().unwrap_or("");
    let desc = exposant.description_short.as_deref().unwrap_or("");
    let desc_long = exposant.description_long.as_deref().unwrap_or(desc);

    // Produits vedettes
    let produits = db
        .produits_by_exposant(exposant.id.as_deref().unwrap_or(""))
        .ok()
        .unwrap_or_default();
    let vedettes: Vec<_> = produits
        .into_iter()
        .filter(|p| p.is_featured == Some(true))
        .take(6)
        .collect();

    let vedettes_html: String = if vedettes.is_empty() {
        String::new()
    } else {
        let cards: String = vedettes
            .iter()
            .map(|p| {
                let id = p.id.as_deref().unwrap_or("");
                let name = p.name.as_deref().unwrap_or("Produit");
                let price = p
                    .price
                    .map(|x| format!("{:.2} €", x))
                    .unwrap_or_else(|| "Sur demande".to_string());
                format!(
                    r#"<a href="/jayxpose/catalogue/{id}" class="product-card">
                        <h3>{name}</h3>
                        <p class="product-price">{price}</p>
                    </a>"#,
                    id = html_escape(id),
                    name = html_escape(name),
                    price = html_escape(&price),
                )
            })
            .collect();
        format!(
            r#"<h2>Produits vedettes</h2><div class="products-grid">{}</div>"#,
            cards
        )
    };

    let content = format!(
        r#"<div class="hero">
            <h1>{company_name}</h1>
            <p>{slogan}</p>
        </div>
        <div class="container">
            <p style="font-size: 1.1rem; margin-bottom: 2rem;">{desc_long}</p>
            {vedettes_html}
        </div>"#,
        company_name = html_escape(company_name),
        slogan = html_escape(slogan),
        desc_long = html_escape(desc_long),
        vedettes_html = vedettes_html,
    );

    Some(jayxpose_layout(company_name, &content, "home"))
}

/// Page catalogue JayXpose.
fn jayxpose_catalogue_html(db: &JayXposeDb) -> Option<String> {
    let exposants = db.exposants_list_annuaire().ok()?;
    let exposant = exposants
        .into_iter()
        .find(|e| e.vitrine_status.as_deref() == Some("publiee"))?;

    let company_name = exposant.company_name.as_deref().unwrap_or("Vitrine");
    let exposant_id = exposant.id.as_deref().unwrap_or("");
    let produits = db
        .produits_by_exposant(exposant_id)
        .ok()
        .unwrap_or_default();

    let availability_label = |a: Option<&String>| -> (&str, &str) {
        let s: &str = a.map(|x| x.as_str()).unwrap_or("disponible");
        match s {
            "rupture" => ("Rupture", "rupture"),
            "sur_commande" => ("Sur commande", ""),
            _ => ("Disponible", ""),
        }
    };

    let cards: String = if produits.is_empty() {
        r#"<p class="text-muted">Aucun produit disponible pour le moment.</p>"#.to_string()
    } else {
        produits
            .iter()
            .map(|p| {
                let id = p.id.as_deref().unwrap_or("");
                let name = p.name.as_deref().unwrap_or("Produit");
                let price = p
                    .price
                    .map(|x| format!("{:.2} €", x))
                    .unwrap_or_else(|| "Sur demande".to_string());
                let (avail_text, avail_class) = availability_label(p.availability.as_ref());
                format!(
                    r#"<a href="/jayxpose/catalogue/{id}" class="product-card">
                        <h3>{name}</h3>
                        <p class="product-price">{price}</p>
                        <span class="product-avail {avail_class}">{avail_text}</span>
                    </a>"#,
                    id = html_escape(id),
                    name = html_escape(name),
                    price = html_escape(&price),
                    avail_text = html_escape(avail_text),
                    avail_class = avail_class,
                )
            })
            .collect()
    };

    let content = format!(
        r#"<div class="container">
            <h2>Catalogue — {company_name}</h2>
            <div class="products-grid">{cards}</div>
        </div>"#,
        company_name = html_escape(company_name),
        cards = cards,
    );

    Some(jayxpose_layout(
        &format!("Catalogue — {}", company_name),
        &content,
        "catalogue",
    ))
}

/// Page fiche produit JayXpose.
fn jayxpose_product_html(db: &JayXposeDb, product_id: &str) -> Option<String> {
    let produit = db.produit_by_id(product_id).ok()??;
    let name = produit.name.as_deref().unwrap_or("Produit");
    let desc = produit.description.as_deref().unwrap_or("");
    let price = produit
        .price
        .map(|x| format!("{:.2} €", x))
        .unwrap_or_else(|| "Sur demande".to_string());
    let availability = match produit.availability.as_deref().unwrap_or("disponible") {
        "rupture" => ("Rupture de stock", "rupture"),
        "sur_commande" => ("Sur commande", ""),
        _ => ("Disponible", ""),
    };

    let content = format!(
        r#"<div class="container">
            <a href="/jayxpose/catalogue" class="back-link">← Retour au catalogue</a>
            <h1 style="font-size: 2rem; margin-bottom: 1rem;">{name}</h1>
            <p class="product-price" style="font-size: 1.5rem;">{price}</p>
            <p class="product-avail {avail_class}" style="margin-bottom: 2rem;">Disponibilité : {avail_text}</p>
            <div style="background: var(--bg-surface); border: 1px solid var(--border); border-radius: 0.75rem; padding: 1.5rem;">
                <h3 style="margin-bottom: 1rem;">Description</h3>
                <p>{desc}</p>
            </div>
        </div>"#,
        name = html_escape(name),
        price = html_escape(&price),
        avail_text = html_escape(availability.0),
        avail_class = availability.1,
        desc = html_escape(desc),
    );

    Some(jayxpose_layout(name, &content, "catalogue"))
}

/// Page contact JayXpose.
fn jayxpose_contact_html(db: &JayXposeDb) -> Option<String> {
    let exposants = db.exposants_list_annuaire().ok()?;
    let exposant = exposants
        .into_iter()
        .find(|e| e.vitrine_status.as_deref() == Some("publiee"))?;

    let company_name = exposant.company_name.as_deref().unwrap_or("Vitrine");
    let email = exposant.contact_email.as_deref().unwrap_or("");
    let phone = exposant.contact_phone.as_deref().unwrap_or("");
    let site_web = exposant.site_web.as_deref().unwrap_or("");

    // Réseaux sociaux
    let mut socials = Vec::new();
    if let Some(fb) = exposant.social_facebook.as_ref().filter(|s| !s.is_empty()) {
        socials.push(format!(
            r#"<a href="{}" target="_blank" rel="noopener">Facebook</a>"#,
            html_escape(fb)
        ));
    }
    if let Some(ig) = exposant.social_instagram.as_ref().filter(|s| !s.is_empty()) {
        socials.push(format!(
            r#"<a href="{}" target="_blank" rel="noopener">Instagram</a>"#,
            html_escape(ig)
        ));
    }
    if let Some(li) = exposant.social_linkedin.as_ref().filter(|s| !s.is_empty()) {
        socials.push(format!(
            r#"<a href="{}" target="_blank" rel="noopener">LinkedIn</a>"#,
            html_escape(li)
        ));
    }
    if let Some(x) = exposant.social_x.as_ref().filter(|s| !s.is_empty()) {
        socials.push(format!(
            r#"<a href="{}" target="_blank" rel="noopener">X (Twitter)</a>"#,
            html_escape(x)
        ));
    }

    let socials_html = if socials.is_empty() {
        String::new()
    } else {
        format!(
            r#"<div class="contact-item"><span>Réseaux sociaux</span><div style="display: flex; gap: 1rem;">{}</div></div>"#,
            socials.join(" ")
        )
    };

    let content = format!(
        r#"<div class="container">
            <h2>Contact — {company_name}</h2>
            <div class="contact-info">
                <h3>Coordonnées</h3>
                {email_html}
                {phone_html}
                {site_html}
                {socials_html}
            </div>
        </div>"#,
        company_name = html_escape(company_name),
        email_html = if email.is_empty() {
            String::new()
        } else {
            format!(
                r#"<div class="contact-item"><span>📧 Email</span><a href="mailto:{}" style="color: var(--primary);">{}</a></div>"#,
                html_escape(email),
                html_escape(email)
            )
        },
        phone_html = if phone.is_empty() {
            String::new()
        } else {
            format!(
                r#"<div class="contact-item"><span>📞 Téléphone</span><span>{}</span></div>"#,
                html_escape(phone)
            )
        },
        site_html = if site_web.is_empty() {
            String::new()
        } else {
            format!(
                r#"<div class="contact-item"><span>🌐 Site web</span><a href="{}" target="_blank" rel="noopener" style="color: var(--primary);">{}</a></div>"#,
                html_escape(site_web),
                html_escape(site_web)
            )
        },
        socials_html = socials_html,
    );

    Some(jayxpose_layout(
        &format!("Contact — {}", company_name),
        &content,
        "contact",
    ))
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
        let manager = CentralMwsManager::with_defaults("test-cog".to_string(), "1.0.0".to_string());

        assert_eq!(manager.get_state().await, CentralMwsState::Disconnected);
        assert!(!manager.is_connected().await);
        assert!(!manager.is_lone().await);
    }

    #[tokio::test]
    async fn test_lone_manager() {
        let manager = CentralMwsManager::lone("lone-cog".to_string(), "1.0.0".to_string());

        assert_eq!(manager.get_state().await, CentralMwsState::Lone);
        assert!(manager.is_lone().await);
        assert!(!manager.is_connected().await);
    }

    #[tokio::test]
    async fn test_service_management() {
        let manager = CentralMwsManager::with_defaults("test-cog".to_string(), "1.0.0".to_string());

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

        let lone_summary =
            MwsStatusSummary::from_state(&CentralMwsState::Lone, &MwsConformityState::LoneMode);
        assert_eq!(lone_summary.icon, "🏝️");
        assert!(!lone_summary.can_connect);
        assert!(!lone_summary.can_disconnect);
    }
}
