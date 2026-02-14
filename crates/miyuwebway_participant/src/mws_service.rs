//! Service MWS — Coordonne la connexion au réseau Miyukini Webway.
//!
//! Service de haut niveau qui gère le Relay, le Tracker, et le heartbeat automatique.

use crate::errors::MiyuwebwayParticipantError;
use crate::protocol::{CogInfo, LobbyInfo, LobbySearchResult};
use crate::relay_client::{RelayClient, RelayClientConfig, RelaySession};
use crate::tracker_client::{TrackerAnnouncement, TrackerClient, TrackerClientConfig};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::{debug, info, warn};

/// Configuration du service MWS.
#[derive(Debug, Clone)]
pub struct MwsServiceConfig {
    /// Configuration du Relay.
    pub relay: RelayClientConfig,
    /// Configuration du Tracker.
    pub tracker: TrackerClientConfig,
    /// Activer le heartbeat automatique.
    pub auto_heartbeat: bool,
    /// Activer la reconnexion automatique.
    pub auto_reconnect: bool,
    /// Délai entre les tentatives de reconnexion (secondes).
    pub reconnect_delay: u64,
    /// Nombre max de tentatives de reconnexion.
    pub max_reconnect_attempts: u32,
}

impl Default for MwsServiceConfig {
    fn default() -> Self {
        Self {
            relay: RelayClientConfig::default(),
            tracker: TrackerClientConfig::default(),
            auto_heartbeat: true,
            auto_reconnect: true,
            reconnect_delay: 10,
            max_reconnect_attempts: 5,
        }
    }
}

/// État global du service MWS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MwsServiceState {
    /// Arrêté.
    Stopped,
    /// En cours de connexion au Relay.
    ConnectingRelay,
    /// Connecté au Relay, en cours d'annonce au Tracker.
    ConnectingTracker,
    /// Connecté et annoncé.
    Online,
    /// Déconnecté (erreur ou maintenance).
    Offline,
    /// En cours de reconnexion.
    Reconnecting,
}

/// Identité du COG pour le MWS.
#[derive(Debug, Clone)]
pub struct CogIdentity {
    /// ID du COG.
    pub cog_id: String,
    /// Version des Cores.
    pub core_version: String,
    /// Adresse de connexion publique.
    pub public_address: String,
    /// Services exposés.
    pub services: Vec<String>,
}

/// Service MWS principal.
pub struct MwsService {
    /// Configuration.
    config: MwsServiceConfig,
    /// Identité du COG.
    identity: Arc<RwLock<Option<CogIdentity>>>,
    /// Client Relay.
    relay_client: Arc<RelayClient>,
    /// Client Tracker.
    tracker_client: Arc<TrackerClient>,
    /// État du service.
    state: Arc<RwLock<MwsServiceState>>,
    /// Session Relay.
    relay_session: Arc<RwLock<Option<RelaySession>>>,
    /// Lobbys hébergés.
    lobbys: Arc<RwLock<Vec<LobbyInfo>>>,
    /// Flag d'arrêt.
    shutdown: Arc<RwLock<bool>>,
}

impl MwsService {
    /// Crée un nouveau service MWS.
    pub fn new(config: MwsServiceConfig) -> Self {
        Self {
            relay_client: Arc::new(RelayClient::new(config.relay.clone())),
            tracker_client: Arc::new(TrackerClient::new(config.tracker.clone())),
            config,
            identity: Arc::new(RwLock::new(None)),
            state: Arc::new(RwLock::new(MwsServiceState::Stopped)),
            relay_session: Arc::new(RwLock::new(None)),
            lobbys: Arc::new(RwLock::new(Vec::new())),
            shutdown: Arc::new(RwLock::new(false)),
        }
    }

    /// Crée un service avec la configuration par défaut.
    pub fn with_defaults() -> Self {
        Self::new(MwsServiceConfig::default())
    }

    /// Configure l'adresse Origin personnalisée.
    pub fn with_origin(mut self, relay_address: String, tracker_address: String) -> Self {
        self.config.relay.relay_address = relay_address.clone();
        self.config.relay.tls_domain = relay_address.split(':').next().unwrap_or("localhost").to_string();
        self.config.tracker.tracker_address = tracker_address;
        self.relay_client = Arc::new(RelayClient::new(self.config.relay.clone()));
        self.tracker_client = Arc::new(TrackerClient::new(self.config.tracker.clone()));
        self
    }

    /// Démarre le service et connecte le COG au réseau MWS.
    pub async fn start(&self, identity: CogIdentity) -> Result<(), MiyuwebwayParticipantError> {
        info!("Starting MWS service for COG {}", identity.cog_id);

        // Stocker l'identité
        {
            let mut id = self.identity.write().await;
            *id = Some(identity.clone());
        }

        // Mettre à jour l'état
        {
            let mut state = self.state.write().await;
            *state = MwsServiceState::ConnectingRelay;
        }

        // 1. Connexion au Relay
        let session = self
            .relay_client
            .connect_and_register(&identity.cog_id, &identity.core_version, identity.services.clone())
            .await?;

        info!("Relay session established");

        {
            let mut s = self.relay_session.write().await;
            *s = Some(session.clone());
        }

        // 2. Annonce au Tracker
        {
            let mut state = self.state.write().await;
            *state = MwsServiceState::ConnectingTracker;
        }

        // Le permis_id reçu du Relay est une string UTF-8 (ex: "permis-xxx")
        // On doit le passer tel quel au Tracker, pas l'encoder en hex
        let permis_id = session
            .permis_id
            .map(|p| String::from_utf8_lossy(&p).to_string())
            .unwrap_or_default();

        let lobbys = self.lobbys.read().await.clone();

        let announcement = TrackerAnnouncement {
            cog_id: identity.cog_id.clone(),
            core_version: identity.core_version.clone(),
            permis_id,
            address: identity.public_address.clone(),
            services: identity.services.clone(),
            lobbys,
        };

        self.tracker_client.announce(&announcement).await?;

        info!("COG announced on MWS network");

        // 3. Mettre en ligne
        {
            let mut state = self.state.write().await;
            *state = MwsServiceState::Online;
        }

        // 4. Démarrer le heartbeat automatique si activé
        if self.config.auto_heartbeat {
            self.start_heartbeat_task(identity.cog_id.clone()).await;
        }

        Ok(())
    }

    /// Envoie un heartbeat au Tracker pour maintenir la connexion (appelable par le Central).
    pub async fn send_heartbeat(&self) -> Result<(), MiyuwebwayParticipantError> {
        self.ensure_online().await?;
        let identity = self.identity.read().await;
        let cog_id = identity
            .as_ref()
            .ok_or(MiyuwebwayParticipantError::NotConnected)?
            .cog_id
            .clone();
        drop(identity);
        tracing::debug!("[MWS Participant] send_heartbeat cog_id={}", &cog_id);
        self.tracker_client.heartbeat(&cog_id, 100, 0).await?;
        tracing::debug!("[MWS Participant] heartbeat ACK reçu pour {}", &cog_id);
        Ok(())
    }

    /// Démarre la tâche de heartbeat automatique.
    async fn start_heartbeat_task(&self, cog_id: String) {
        let tracker = Arc::clone(&self.tracker_client);
        let state = Arc::clone(&self.state);
        let shutdown = Arc::clone(&self.shutdown);
        let interval_secs = self.tracker_client.get_heartbeat_interval().await;

        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(interval_secs as u64));

            loop {
                timer.tick().await;

                // Vérifier l'arrêt
                if *shutdown.read().await {
                    debug!("Heartbeat task stopping");
                    break;
                }

                // Vérifier l'état
                let current_state = state.read().await.clone();
                if current_state != MwsServiceState::Online {
                    debug!("Not online, skipping heartbeat");
                    continue;
                }

                // Envoyer heartbeat
                match tracker.heartbeat(&cog_id, 100, 0).await {
                    Ok(_) => debug!("Heartbeat sent successfully"),
                    Err(e) => {
                        warn!("Heartbeat failed: {}", e);
                        // TODO: déclencher reconnexion si nécessaire
                    }
                }
            }
        });
    }

    /// Arrête le service et se déconnecte du réseau.
    pub async fn stop(&self) -> Result<(), MiyuwebwayParticipantError> {
        info!("Stopping MWS service");

        // Signaler l'arrêt
        {
            let mut shutdown = self.shutdown.write().await;
            *shutdown = true;
        }

        // Retirer du Tracker
        let identity = self.identity.read().await;
        if let Some(ref id) = *identity {
            if let Err(e) = self.tracker_client.withdraw(&id.cog_id).await {
                warn!("Failed to withdraw from Tracker: {}", e);
            }
        }

        // Mettre à jour l'état
        {
            let mut state = self.state.write().await;
            *state = MwsServiceState::Stopped;
        }

        info!("MWS service stopped");
        Ok(())
    }

    /// Recherche des COGs sur le réseau.
    pub async fn search_cogs(
        &self,
        version_filter: Option<String>,
        service_filter: Option<String>,
        limit: u32,
    ) -> Result<Vec<CogInfo>, MiyuwebwayParticipantError> {
        self.ensure_online().await?;
        self.tracker_client
            .search_cogs(version_filter, service_filter, limit)
            .await
    }

    /// Recherche des lobbys sur le réseau.
    pub async fn search_lobbys(
        &self,
        name_filter: Option<String>,
        public_only: bool,
        limit: u32,
    ) -> Result<Vec<LobbySearchResult>, MiyuwebwayParticipantError> {
        self.ensure_online().await?;
        self.tracker_client
            .search_lobbys(name_filter, public_only, limit)
            .await
    }

    /// Crée un lobby sur le réseau.
    pub async fn create_lobby(
        &self,
        lobby: LobbyInfo,
        password: Option<String>,
    ) -> Result<String, MiyuwebwayParticipantError> {
        self.ensure_online().await?;

        let identity = self.identity.read().await;
        let cog_id = identity
            .as_ref()
            .map(|i| i.cog_id.clone())
            .ok_or(MiyuwebwayParticipantError::NotConnected)?;

        let lobby_id = self
            .tracker_client
            .create_lobby(&cog_id, lobby.clone(), password)
            .await?;

        // Ajouter à la liste locale
        {
            let mut lobbys = self.lobbys.write().await;
            let mut new_lobby = lobby;
            new_lobby.lobby_id = lobby_id.clone();
            lobbys.push(new_lobby);
        }

        Ok(lobby_id)
    }

    /// Supprime un lobby.
    pub async fn delete_lobby(&self, lobby_id: &str) -> Result<(), MiyuwebwayParticipantError> {
        self.ensure_online().await?;

        let cog_id = self
            .identity
            .read()
            .await
            .as_ref()
            .map(|i| i.cog_id.clone())
            .ok_or(MiyuwebwayParticipantError::NotConnected)?;

        // Envoyer DELETE_LOBBY au Tracker et attendre DeleteLobbyOk
        self.tracker_client.delete_lobby(&cog_id, lobby_id).await?;

        // Retirer de la liste locale après succès Tracker
        {
            let mut lobbys = self.lobbys.write().await;
            lobbys.retain(|l| l.lobby_id != lobby_id);
        }

        Ok(())
    }

    /// Vérifie que le service est en ligne.
    async fn ensure_online(&self) -> Result<(), MiyuwebwayParticipantError> {
        let state = self.state.read().await;
        if *state != MwsServiceState::Online {
            return Err(MiyuwebwayParticipantError::NotConnected);
        }
        Ok(())
    }

    /// Retourne l'état actuel du service.
    pub async fn get_state(&self) -> MwsServiceState {
        self.state.read().await.clone()
    }

    /// Vérifie si le service est en ligne.
    pub async fn is_online(&self) -> bool {
        *self.state.read().await == MwsServiceState::Online
    }

    /// Retourne l'identité du COG.
    pub async fn get_identity(&self) -> Option<CogIdentity> {
        self.identity.read().await.clone()
    }

    /// Retourne la session Relay.
    pub async fn get_relay_session(&self) -> Option<RelaySession> {
        self.relay_session.read().await.clone()
    }

    /// Retourne les lobbys hébergés.
    pub async fn get_lobbys(&self) -> Vec<LobbyInfo> {
        self.lobbys.read().await.clone()
    }
}

/// Dépendance manquante : hex encoding
mod hex {
    pub fn encode(data: &[u8]) -> String {
        data.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = MwsServiceConfig::default();
        assert!(config.auto_heartbeat);
        assert!(config.auto_reconnect);
    }

    #[tokio::test]
    async fn test_service_creation() {
        let service = MwsService::with_defaults();
        assert_eq!(service.get_state().await, MwsServiceState::Stopped);
        assert!(!service.is_online().await);
    }
}
