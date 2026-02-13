//! Serveur Tracker TCP.
//!
//! Écoute sur le port 21000 et gère la découverte des COGs et lobbys.
//!
//! # Fonctionnalités
//!
//! - Annonce et découverte de COGs
//! - Gestion des pools par version
//! - Recherche de COGs, lobbys et services
//! - Création et gestion des lobbys
//! - Heartbeat pour maintenir les connexions

use bytes::BytesMut;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error, info, warn};

use super::catalog::Catalog;
use super::metrics::TrackerMetrics;
use super::pool::{CogEntry, LobbyEntry, PoolManager};
use super::protocol::{
    error_codes, AnnounceAckPayload, AnnouncePayload, CogSearchResult, CreateLobbyOkPayload,
    CreateLobbyPayload, ErrorPayload, HeartbeatPayload, JoinLobbyPayload, JoinLobbyResultPayload,
    LobbySearchResult, SearchCogsPayload, SearchCogsResultPayload, SearchLobbysPayload,
    SearchLobbysResultPayload, StatsResultPayload, TrackerError, TrackerMessage,
    TrackerMessageType, UpdateLobbyPayload,
};
use crate::config::OriginConfig;

/// Serveur Tracker.
pub struct TrackerServer {
    /// Configuration.
    config: Arc<OriginConfig>,
    /// Gestionnaire de pools.
    pool_manager: Arc<PoolManager>,
    /// Catalogue.
    catalog: Arc<Catalog>,
    /// Métriques.
    metrics: Arc<TrackerMetrics>,
}

impl TrackerServer {
    /// Crée un nouveau serveur tracker.
    #[must_use]
    pub fn new(config: Arc<OriginConfig>) -> Self {
        let pool_manager = Arc::new(PoolManager::new(
            config.tracker.pools.enable_version_isolation,
        ));
        let catalog = Arc::new(Catalog::new(Arc::clone(&pool_manager)));
        let metrics = Arc::new(TrackerMetrics::new());

        Self {
            config,
            pool_manager,
            catalog,
            metrics,
        }
    }

    /// Récupère le gestionnaire de pools.
    #[must_use]
    pub fn pool_manager(&self) -> Arc<PoolManager> {
        Arc::clone(&self.pool_manager)
    }

    /// Récupère le catalogue.
    #[must_use]
    pub fn catalog(&self) -> Arc<Catalog> {
        Arc::clone(&self.catalog)
    }

    /// Récupère les métriques.
    #[must_use]
    pub fn metrics(&self) -> Arc<TrackerMetrics> {
        Arc::clone(&self.metrics)
    }

    /// Démarre le serveur tracker.
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let tracker_addr: SocketAddr = format!(
            "{}:{}",
            self.config.tracker.host, self.config.tracker.port
        )
        .parse()?;

        let listener = TcpListener::bind(tracker_addr).await?;
        info!("📡 Tracker server listening on {}", tracker_addr);

        // Démarrer les tâches de maintenance
        self.start_maintenance_tasks();

        // Accepter les connexions
        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    self.handle_new_connection(stream, peer_addr).await;
                }
                Err(e) => {
                    error!("Accept error: {}", e);
                    self.metrics.record_internal_error();
                }
            }
        }
    }

    /// Démarre les tâches de maintenance.
    fn start_maintenance_tasks(&self) {
        // Nettoyage des COGs inactifs
        let pool_manager = Arc::clone(&self.pool_manager);
        let metrics = Arc::clone(&self.metrics);
        let timeout = i64::from(self.config.limits.tunnel_timeout_seconds);

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                let cleaned = pool_manager.cleanup_all(timeout).await;
                if cleaned > 0 {
                    info!("Cleaned up {} inactive COGs from pools", cleaned);
                    for _ in 0..cleaned {
                        metrics.record_cog_expired();
                    }
                }
            }
        });
    }

    /// Gère une nouvelle connexion.
    async fn handle_new_connection(&self, stream: TcpStream, peer_addr: SocketAddr) {
        self.metrics.record_connection();
        debug!("Tracker connection from {}", peer_addr);

        let pool_manager = Arc::clone(&self.pool_manager);
        let config = Arc::clone(&self.config);
        let metrics = Arc::clone(&self.metrics);

        tokio::spawn(async move {
            if let Err(e) = Self::handle_connection(stream, peer_addr, pool_manager, config, Arc::clone(&metrics)).await {
                debug!("Tracker connection error from {}: {}", peer_addr, e);
            }
            metrics.record_connection_closed();
        });
    }

    /// Gère une connexion individuelle.
    async fn handle_connection(
        mut stream: TcpStream,
        peer_addr: SocketAddr,
        pool_manager: Arc<PoolManager>,
        config: Arc<OriginConfig>,
        metrics: Arc<TrackerMetrics>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut buf = BytesMut::with_capacity(4096);

        loop {
            // Lire des données
            let n = stream.read_buf(&mut buf).await?;
            if n == 0 {
                break;
            }
            metrics.record_bytes_received(n);

            // Traiter les messages
            while let Some(msg) = Self::try_parse_message(&mut buf, &metrics)? {
                debug!("Received {:?} from {}", msg.header.message_type, peer_addr);

                // Traiter le message
                let response = Self::handle_message(
                    msg,
                    &pool_manager,
                    &config,
                    &metrics,
                )
                .await;

                // Envoyer la réponse
                if let Some(response_msg) = response {
                    let bytes = response_msg.to_bytes();
                    metrics.record_bytes_sent(bytes.len());
                    stream.write_all(&bytes).await?;
                }
            }
        }

        Ok(())
    }

    /// Essaie de parser un message.
    fn try_parse_message(
        buf: &mut BytesMut,
        metrics: &TrackerMetrics,
    ) -> Result<Option<TrackerMessage>, TrackerError> {
        match TrackerMessage::parse(buf) {
            Ok(msg) => Ok(msg),
            Err(TrackerError::InsufficientData) => Ok(None),
            Err(e) => {
                metrics.record_protocol_error();
                Err(e)
            }
        }
    }

    /// Traite un message.
    async fn handle_message(
        msg: TrackerMessage,
        pool_manager: &Arc<PoolManager>,
        config: &Arc<OriginConfig>,
        metrics: &Arc<TrackerMetrics>,
    ) -> Option<TrackerMessage> {
        match msg.header.message_type {
            TrackerMessageType::Announce => {
                Self::handle_announce(&msg.payload, pool_manager, config, metrics).await
            }
            TrackerMessageType::Withdraw => {
                Self::handle_withdraw(&msg.payload, pool_manager, metrics).await
            }
            TrackerMessageType::Heartbeat => {
                Self::handle_heartbeat(&msg.payload, pool_manager, metrics).await
            }
            TrackerMessageType::SearchCogs => {
                Self::handle_search_cogs(&msg.payload, pool_manager, metrics).await
            }
            TrackerMessageType::SearchLobbys => {
                Self::handle_search_lobbys(&msg.payload, pool_manager, metrics).await
            }
            TrackerMessageType::CreateLobby => {
                Self::handle_create_lobby(&msg.payload, pool_manager, config, metrics).await
            }
            TrackerMessageType::UpdateLobby => {
                Self::handle_update_lobby(&msg.payload, pool_manager, metrics).await
            }
            TrackerMessageType::DeleteLobby => {
                Self::handle_delete_lobby(&msg.payload, pool_manager, metrics).await
            }
            TrackerMessageType::JoinLobby => {
                Self::handle_join_lobby(&msg.payload, pool_manager, config, metrics).await
            }
            TrackerMessageType::GetStats => {
                Self::handle_get_stats(pool_manager, metrics).await
            }
            _ => {
                warn!("Unhandled tracker message type: {:?}", msg.header.message_type);
                None
            }
        }
    }

    /// Traite une annonce.
    async fn handle_announce(
        payload: &[u8],
        pool_manager: &Arc<PoolManager>,
        config: &Arc<OriginConfig>,
        metrics: &Arc<TrackerMetrics>,
    ) -> Option<TrackerMessage> {
        let announce = match AnnouncePayload::parse(payload) {
            Ok(a) => a,
            Err(e) => {
                metrics.record_announce(false);
                return Some(TrackerMessage::new(
                    TrackerMessageType::Error,
                    ErrorPayload::new(error_codes::INTERNAL_ERROR, e.to_string()).to_bytes(),
                ));
            }
        };

        info!(
            "ANNOUNCE from {} (version: {}, services: {:?})",
            announce.cog_id, announce.core_version, announce.services
        );

        // TODO: vérifier le permis_id avec le Relay

        // Récupérer ou créer le pool
        let pool = pool_manager.get_or_create_pool(&announce.core_version).await;

        // Créer l'entrée COG
        let entry = CogEntry {
            cog_id: announce.cog_id.clone(),
            core_version: announce.core_version.clone(),
            address: announce.address.clone(),
            last_seen: chrono::Utc::now(),
            services: announce.services.clone(),
            lobbys: Vec::new(),
        };

        // Ajouter au pool
        pool.add_cog(entry).await;
        metrics.record_announce(true);

        // Répondre
        let ack = AnnounceAckPayload {
            success: true,
            message: "Registered successfully".to_string(),
            heartbeat_interval: config.limits.heartbeat_interval_seconds,
            ttl: config.limits.tunnel_timeout_seconds,
        };

        Some(TrackerMessage::new(
            TrackerMessageType::AnnounceAck,
            ack.to_bytes(),
        ))
    }

    /// Traite un retrait.
    async fn handle_withdraw(
        payload: &[u8],
        pool_manager: &Arc<PoolManager>,
        metrics: &Arc<TrackerMetrics>,
    ) -> Option<TrackerMessage> {
        // Le payload est juste le cog_id en JSON
        let cog_id: String = match serde_json::from_slice(payload) {
            Ok(id) => id,
            Err(_) => return None,
        };

        info!("WITHDRAW from {}", cog_id);

        // Chercher et supprimer dans tous les pools
        let versions = pool_manager.list_versions().await;
        for version in versions {
            if let Some(pool) = pool_manager.get_pool(&version).await {
                pool.remove_cog(&cog_id).await;
            }
        }

        metrics.record_cog_withdrawn();
        None
    }

    /// Traite un heartbeat.
    async fn handle_heartbeat(
        payload: &[u8],
        pool_manager: &Arc<PoolManager>,
        metrics: &Arc<TrackerMetrics>,
    ) -> Option<TrackerMessage> {
        let heartbeat = match HeartbeatPayload::parse(payload) {
            Ok(h) => h,
            Err(_) => return None,
        };

        // Mettre à jour last_seen dans le pool
        let versions = pool_manager.list_versions().await;
        for version in versions {
            if let Some(pool) = pool_manager.get_pool(&version).await {
                pool.touch_cog(&heartbeat.cog_id).await;
            }
        }

        metrics.record_heartbeat();

        // Répondre avec HeartbeatAck
        let ack = HeartbeatPayload {
            cog_id: heartbeat.cog_id,
            timestamp: chrono::Utc::now().timestamp() as u64,
        };

        Some(TrackerMessage::new(
            TrackerMessageType::HeartbeatAck,
            ack.to_bytes(),
        ))
    }

    /// Traite une recherche de COGs.
    async fn handle_search_cogs(
        payload: &[u8],
        pool_manager: &Arc<PoolManager>,
        metrics: &Arc<TrackerMetrics>,
    ) -> Option<TrackerMessage> {
        let search = match SearchCogsPayload::parse(payload) {
            Ok(s) => s,
            Err(_) => {
                return Some(TrackerMessage::new(
                    TrackerMessageType::Error,
                    ErrorPayload::new(error_codes::INTERNAL_ERROR, "Invalid search payload").to_bytes(),
                ));
            }
        };

        metrics.record_cog_search();

        let mut results = Vec::new();
        let limit = search.limit.unwrap_or(100) as usize;
        let offset = search.offset.unwrap_or(0) as usize;

        // Chercher dans les pools
        let versions = if let Some(v) = &search.version {
            vec![v.clone()]
        } else {
            pool_manager.list_versions().await
        };

        for version in versions {
            if let Some(pool) = pool_manager.get_pool(&version).await {
                let cogs = pool.list_cogs().await;
                for cog in cogs {
                    // Filtrer par query si présent
                    if let Some(query) = &search.query {
                        let query_lower = query.to_lowercase();
                        if !cog.cog_id.to_lowercase().contains(&query_lower) {
                            continue;
                        }
                    }

                    // Filtrer par service si présent
                    if let Some(service) = &search.service {
                        if !cog.services.contains(service) {
                            continue;
                        }
                    }

                    results.push(CogSearchResult {
                        cog_id: cog.cog_id,
                        core_version: cog.core_version,
                        address: cog.address,
                        services: cog.services,
                        lobby_count: cog.lobbys.len() as u32,
                    });
                }
            }
        }

        let total = results.len() as u32;

        // Appliquer pagination
        let paginated: Vec<_> = results.into_iter().skip(offset).take(limit).collect();

        let response = SearchCogsResultPayload {
            results: paginated,
            total,
        };

        Some(TrackerMessage::new(
            TrackerMessageType::SearchCogsResult,
            response.to_bytes(),
        ))
    }

    /// Traite une recherche de lobbys.
    async fn handle_search_lobbys(
        payload: &[u8],
        pool_manager: &Arc<PoolManager>,
        metrics: &Arc<TrackerMetrics>,
    ) -> Option<TrackerMessage> {
        let search = match SearchLobbysPayload::parse(payload) {
            Ok(s) => s,
            Err(_) => {
                return Some(TrackerMessage::new(
                    TrackerMessageType::Error,
                    ErrorPayload::new(error_codes::INTERNAL_ERROR, "Invalid search payload").to_bytes(),
                ));
            }
        };

        metrics.record_lobby_search();

        let mut results = Vec::new();
        let limit = search.limit.unwrap_or(100) as usize;
        let offset = search.offset.unwrap_or(0) as usize;
        let available_only = search.available_only.unwrap_or(false);

        // Chercher dans les pools
        let lobbys = pool_manager.list_all_public_lobbys().await;

        for (version, cog_id, lobby) in lobbys {
            // Filtrer par version
            if let Some(v) = &search.version {
                if &version != v {
                    continue;
                }
            }

            // Filtrer par query
            if let Some(query) = &search.query {
                let query_lower = query.to_lowercase();
                if !lobby.name.to_lowercase().contains(&query_lower) {
                    continue;
                }
            }

            // Filtrer par disponibilité
            if available_only && lobby.current_players >= lobby.max_players {
                continue;
            }

            results.push(LobbySearchResult {
                lobby_id: lobby.lobby_id,
                name: lobby.name,
                host_cog_id: cog_id,
                pool_version: version,
                password_required: lobby.password_required,
                current_players: lobby.current_players,
                max_players: lobby.max_players,
                metadata: serde_json::from_str(&lobby.metadata).unwrap_or(serde_json::Value::Null),
            });
        }

        let total = results.len() as u32;
        let paginated: Vec<_> = results.into_iter().skip(offset).take(limit).collect();

        let response = SearchLobbysResultPayload {
            results: paginated,
            total,
        };

        Some(TrackerMessage::new(
            TrackerMessageType::SearchLobbysResult,
            response.to_bytes(),
        ))
    }

    /// Traite la création d'un lobby.
    async fn handle_create_lobby(
        payload: &[u8],
        pool_manager: &Arc<PoolManager>,
        config: &Arc<OriginConfig>,
        metrics: &Arc<TrackerMetrics>,
    ) -> Option<TrackerMessage> {
        let create = match CreateLobbyPayload::parse(payload) {
            Ok(c) => c,
            Err(_) => {
                return Some(TrackerMessage::new(
                    TrackerMessageType::CreateLobbyErr,
                    ErrorPayload::new(error_codes::INTERNAL_ERROR, "Invalid payload").to_bytes(),
                ));
            }
        };

        // Trouver le COG
        let cog_entry = pool_manager.find_cog(&create.cog_id).await;
        if cog_entry.is_none() {
            return Some(TrackerMessage::new(
                TrackerMessageType::CreateLobbyErr,
                ErrorPayload::new(error_codes::COG_NOT_FOUND, "COG not found").to_bytes(),
            ));
        }

        let cog = cog_entry.unwrap();

        // Vérifier la limite de lobbys par COG
        if cog.lobbys.len() >= config.tracker.lobbys.max_lobbys_per_cog as usize {
            return Some(TrackerMessage::new(
                TrackerMessageType::CreateLobbyErr,
                ErrorPayload::new(error_codes::INTERNAL_ERROR, "Max lobbys reached").to_bytes(),
            ));
        }

        // Générer l'ID du lobby
        let lobby_id = format!("lobby-{}-{}", create.cog_id, chrono::Utc::now().timestamp());

        // Créer le lobby
        let lobby = LobbyEntry {
            lobby_id: lobby_id.clone(),
            name: create.name,
            is_public: create.is_public,
            password_required: create.password.is_some(),
            max_players: create.max_players,
            current_players: 1, // L'hôte
            metadata: serde_json::to_string(&create.metadata).unwrap_or_default(),
        };

        // Trouver le pool et mettre à jour le COG
        if let Some(pool) = pool_manager.get_pool(&cog.core_version).await {
            let mut lobbys = cog.lobbys.clone();
            lobbys.push(lobby);
            pool.update_cog(&create.cog_id, cog.services.clone(), lobbys).await;
        }

        metrics.record_lobby_created();
        info!("Lobby {} created by {}", lobby_id, create.cog_id);

        let response = CreateLobbyOkPayload { lobby_id };

        Some(TrackerMessage::new(
            TrackerMessageType::CreateLobbyOk,
            response.to_bytes(),
        ))
    }

    /// Traite la mise à jour d'un lobby.
    async fn handle_update_lobby(
        payload: &[u8],
        pool_manager: &Arc<PoolManager>,
        metrics: &Arc<TrackerMetrics>,
    ) -> Option<TrackerMessage> {
        let update = match UpdateLobbyPayload::parse(payload) {
            Ok(u) => u,
            Err(_) => return None,
        };

        // Trouver le COG
        let cog = pool_manager.find_cog(&update.cog_id).await?;

        // Mettre à jour le lobby
        let mut lobbys = cog.lobbys.clone();
        for lobby in &mut lobbys {
            if lobby.lobby_id == update.lobby_id {
                if let Some(name) = &update.name {
                    lobby.name = name.clone();
                }
                if let Some(players) = update.current_players {
                    lobby.current_players = players;
                }
                if let Some(meta) = &update.metadata {
                    lobby.metadata = serde_json::to_string(meta).unwrap_or_default();
                }
                break;
            }
        }

        // Sauvegarder
        if let Some(pool) = pool_manager.get_pool(&cog.core_version).await {
            pool.update_cog(&update.cog_id, cog.services.clone(), lobbys).await;
        }

        debug!("Lobby {} updated", update.lobby_id);
        None
    }

    /// Traite la suppression d'un lobby.
    async fn handle_delete_lobby(
        payload: &[u8],
        pool_manager: &Arc<PoolManager>,
        metrics: &Arc<TrackerMetrics>,
    ) -> Option<TrackerMessage> {
        // Payload: { "cog_id": "...", "lobby_id": "..." }
        #[derive(serde::Deserialize)]
        struct DeletePayload {
            cog_id: String,
            lobby_id: String,
        }

        let delete: DeletePayload = serde_json::from_slice(payload).ok()?;

        // Trouver le COG
        let cog = pool_manager.find_cog(&delete.cog_id).await?;

        // Supprimer le lobby
        let lobbys: Vec<_> = cog.lobbys
            .into_iter()
            .filter(|l| l.lobby_id != delete.lobby_id)
            .collect();

        // Sauvegarder
        if let Some(pool) = pool_manager.get_pool(&cog.core_version).await {
            pool.update_cog(&delete.cog_id, cog.services.clone(), lobbys).await;
        }

        metrics.record_lobby_deleted();
        info!("Lobby {} deleted", delete.lobby_id);
        None
    }

    /// Traite une demande de join.
    async fn handle_join_lobby(
        payload: &[u8],
        pool_manager: &Arc<PoolManager>,
        config: &Arc<OriginConfig>,
        metrics: &Arc<TrackerMetrics>,
    ) -> Option<TrackerMessage> {
        let join = match JoinLobbyPayload::parse(payload) {
            Ok(j) => j,
            Err(_) => {
                metrics.record_join_request(false);
                return Some(TrackerMessage::new(
                    TrackerMessageType::JoinLobbyResult,
                    JoinLobbyResultPayload {
                        success: false,
                        message: "Invalid payload".to_string(),
                        host_address: None,
                    }
                    .to_bytes(),
                ));
            }
        };

        // Chercher le lobby
        let lobbys = pool_manager.list_all_public_lobbys().await;
        let found = lobbys.iter().find(|(_, _, l)| l.lobby_id == join.lobby_id);

        if let Some((_, cog_id, lobby)) = found {
            // Vérifier la capacité
            if lobby.current_players >= lobby.max_players {
                metrics.record_join_request(false);
                return Some(TrackerMessage::new(
                    TrackerMessageType::JoinLobbyResult,
                    JoinLobbyResultPayload {
                        success: false,
                        message: "Lobby is full".to_string(),
                        host_address: None,
                    }
                    .to_bytes(),
                ));
            }

            // Vérifier le mot de passe
            if lobby.password_required {
                // TODO: vérifier le mot de passe
                // Pour l'instant on accepte sans vérification
            }

            // Trouver l'adresse du COG hôte
            let host_cog = pool_manager.find_cog(cog_id).await;
            let host_address = host_cog.map(|c| c.address);

            metrics.record_join_request(true);
            info!("COG {} joined lobby {}", join.cog_id, join.lobby_id);

            Some(TrackerMessage::new(
                TrackerMessageType::JoinLobbyResult,
                JoinLobbyResultPayload {
                    success: true,
                    message: "Joined successfully".to_string(),
                    host_address,
                }
                .to_bytes(),
            ))
        } else {
            metrics.record_join_request(false);
            Some(TrackerMessage::new(
                TrackerMessageType::JoinLobbyResult,
                JoinLobbyResultPayload {
                    success: false,
                    message: "Lobby not found".to_string(),
                    host_address: None,
                }
                .to_bytes(),
            ))
        }
    }

    /// Traite une demande de statistiques.
    async fn handle_get_stats(
        pool_manager: &Arc<PoolManager>,
        metrics: &Arc<TrackerMetrics>,
    ) -> Option<TrackerMessage> {
        let total_cogs = pool_manager.total_cog_count().await as u32;
        let total_lobbys = pool_manager.list_all_public_lobbys().await.len() as u32;
        let versions = pool_manager.list_versions().await;

        let response = StatsResultPayload {
            total_cogs,
            total_lobbys,
            versions,
            uptime_seconds: metrics.uptime_seconds(),
        };

        Some(TrackerMessage::new(
            TrackerMessageType::StatsResult,
            response.to_bytes(),
        ))
    }
}
