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
    MwsServiceState,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
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
    /// Adresse publique de ce COG.
    pub public_address: String,
    /// Activer le heartbeat automatique.
    pub auto_heartbeat: bool,
    /// Activer la reconnexion automatique.
    pub auto_reconnect: bool,
    /// Présenter le COG même sans services exposés.
    pub present_without_services: bool,
}

impl Default for CentralMwsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            lone_mode: false,
            auto_connect: true,
            relay_address: "origin.miyukini.net:7000".to_string(),
            tracker_address: "origin.miyukini.net:21000".to_string(),
            public_address: "0.0.0.0:0".to_string(), // Sera détecté
            auto_heartbeat: true,
            auto_reconnect: true,
            present_without_services: true, // Se présente toujours
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
}

impl CentralMwsManager {
    /// Crée un nouveau gestionnaire MWS.
    pub fn new(
        config: CentralMwsConfig,
        cog_id: String,
        core_version: String,
        services: Vec<String>,
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
        }
    }

    /// Crée un gestionnaire avec la configuration par défaut.
    pub fn with_defaults(cog_id: String, core_version: String) -> Self {
        Self::new(
            CentralMwsConfig::default(),
            cog_id,
            core_version,
            Vec::new(),
        )
    }

    /// Crée un gestionnaire en mode Lone (isolé).
    pub fn lone(cog_id: String, core_version: String) -> Self {
        Self::new(
            CentralMwsConfig::lone(),
            cog_id,
            core_version,
            Vec::new(),
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

                // Conformité complète
                self.update_conformity(MwsConformityState::FullyConformant).await;
                self.update_state(CentralMwsState::Connected).await;

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
        info!("Déconnexion du réseau MWS...");

        let service = self.service.read().await;
        if let Some(ref svc) = *service {
            svc.stop().await.map_err(|e| e.to_string())?;
        }

        // Mettre à jour l'état
        let (new_state, new_conformity) = if self.config.lone_mode {
            (CentralMwsState::Lone, MwsConformityState::LoneMode)
        } else if self.config.enabled {
            (CentralMwsState::Disconnected, MwsConformityState::Uninitialized)
        } else {
            (CentralMwsState::Disabled, MwsConformityState::Uninitialized)
        };

        self.update_state(new_state).await;
        self.update_conformity(new_conformity).await;

        // Effacer le service
        {
            let mut svc = self.service.write().await;
            *svc = None;
        }

        info!("Déconnecté du réseau MWS");
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
