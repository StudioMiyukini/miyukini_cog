//! Serveur Relay TCP + TLS.
//!
//! Écoute sur le port 7000 et gère les connexions des COGs.
//!
//! # Fonctionnalités
//!
//! - Connexion TLS avec certificats
//! - Vérification de conformité en 3 phases
//! - Rate limiting et protection anti-abus
//! - Tunnels de données entre COGs
//! - Métriques et statistiques
//!
//! Note : Code préparé pour fonctionnalités futures (accesseurs).
#![allow(dead_code)]

use bytes::{Bytes, BytesMut};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio_rustls::rustls;
use tokio_rustls::TlsAcceptor;
use tracing::{debug, error, info, warn};

use super::metrics::RelayMetrics;
use super::permis_registry::PermisRegistry;
use super::rate_limiter::{RateLimiter, RateLimiterConfig, RateLimitResult};
use super::session::{PermisInfo, Session, SessionManager, SessionState};
use super::tunnel::{TunnelManager, TunnelMessage};
use super::verification::Verifier;
use crate::config::OriginConfig;
use crate::protocol::{
    CoreKeyPayload, Frame, MessageType, RegisterErrPayload, RegisterErrorCode,
    RegisterOkPayload, RegisterPayload, ServiceBlockPayload, VerifyPhase, VerifyResult,
    VerifyResultPayload, ED25519_SIGNATURE_SIZE, SESSION_ID_SIZE,
};

/// Pending HTTP injections (Phase 2 : Origin → COG via tunnel).
/// Quand Origin envoie une requête HTTP à un COG, on enregistre un oneshot.
/// Quand le COG renvoie DATA (réponse HTTP), handle_data complète le oneshot.
struct HttpInjector {
    pending: Arc<RwLock<HashMap<[u8; SESSION_ID_SIZE], oneshot::Sender<Bytes>>>>,
}

impl HttpInjector {
    fn new() -> Self {
        Self {
            pending: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Enregistre une attente de réponse et retourne le receiver.
    async fn register(&self, session_id: [u8; SESSION_ID_SIZE]) -> oneshot::Receiver<Bytes> {
        let (tx, rx) = oneshot::channel();
        let mut pending = self.pending.write().await;
        pending.insert(session_id, tx);
        rx
    }

    /// Complète une attente si elle existe (retourne true si complétée).
    async fn complete(&self, session_id: [u8; SESSION_ID_SIZE], data: Bytes) -> bool {
        let tx = {
            let mut pending = self.pending.write().await;
            pending.remove(&session_id)
        };
        if let Some(tx) = tx {
            let _ = tx.send(data);
            true
        } else {
            false
        }
    }
}

/// Serveur Relay.
pub struct RelayServer {
    /// Configuration.
    config: Arc<OriginConfig>,
    /// Gestionnaire de sessions.
    sessions: Arc<SessionManager>,
    /// Registre des permis (partagé avec le Tracker pour vérification ANNOUNCE).
    permis_registry: Option<Arc<PermisRegistry>>,
    /// Vérificateur de conformité.
    verifier: Arc<Verifier>,
    /// Accepteur TLS.
    tls_acceptor: Option<TlsAcceptor>,
    /// Adresses des trackers officiels (JSON).
    tracker_addresses: Bytes,
    /// Signature des adresses des trackers.
    tracker_signature: [u8; ED25519_SIGNATURE_SIZE],
    /// Rate limiter.
    rate_limiter: Arc<RateLimiter>,
    /// Gestionnaire de tunnels.
    tunnel_manager: Arc<TunnelManager>,
    /// Injecteur HTTP Phase 2 (Origin → COG via tunnel).
    http_injector: Arc<HttpInjector>,
    /// Métriques.
    metrics: Arc<RelayMetrics>,
}

impl RelayServer {
    /// Crée un nouveau serveur relay.
    ///
    /// Si `permis_registry` est fourni, les permis délivrés y sont enregistrés
    /// pour que le Tracker puisse vérifier les ANNOUNCE.
    pub async fn new(
        config: Arc<OriginConfig>,
        permis_registry: Option<Arc<PermisRegistry>>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Créer le gestionnaire de sessions
        let sessions = Arc::new(SessionManager::new(
            i64::from(config.limits.tunnel_timeout_seconds),
        ));

        // Créer le vérificateur
        let verifier = Arc::new(Verifier::new(config.cores.keys_dir.clone()));
        verifier.load_keys().await.ok();

        // Configurer TLS
        let tls_acceptor = Self::setup_tls(&config).await?;

        // Préparer la liste des trackers
        let tracker_addresses = serde_json::to_vec(&vec![format!(
            "{}:{}",
            config.identity.ip, config.tracker.port
        )])?;

        // Créer le rate limiter
        let rate_limiter_config = RateLimiterConfig {
            max_connections_per_ip: config.rate_limits.tcp_connections_per_ip,
            max_messages_per_session: config.rate_limits.requests_per_hour_per_cog / 60, // Par minute
            max_failed_registrations: 5,
            window_seconds: 60,
            block_duration_seconds: 300,
            require_pow_for_temp: config.pow.enabled,
            pow_difficulty: config.pow.difficulty_normal,
        };
        let rate_limiter = Arc::new(RateLimiter::new(rate_limiter_config));

        // Créer le gestionnaire de tunnels
        let tunnel_manager = Arc::new(TunnelManager::new(
            i64::from(config.limits.tunnel_timeout_seconds),
        ));

        // Créer les métriques
        let metrics = Arc::new(RelayMetrics::new());

        // Injecteur HTTP Phase 2
        let http_injector = Arc::new(HttpInjector::new());

        Ok(Self {
            config,
            sessions,
            permis_registry,
            verifier,
            tls_acceptor: Some(tls_acceptor),
            tracker_addresses: Bytes::from(tracker_addresses),
            tracker_signature: [0u8; ED25519_SIGNATURE_SIZE],
            rate_limiter,
            tunnel_manager,
            http_injector,
            metrics,
        })
    }

    /// Récupère les métriques.
    #[must_use]
    pub fn metrics(&self) -> Arc<RelayMetrics> {
        Arc::clone(&self.metrics)
    }

    /// Récupère le gestionnaire de sessions.
    #[must_use]
    pub fn sessions(&self) -> Arc<SessionManager> {
        Arc::clone(&self.sessions)
    }

    /// Récupère le gestionnaire de tunnels.
    #[must_use]
    pub fn tunnels(&self) -> Arc<TunnelManager> {
        Arc::clone(&self.tunnel_manager)
    }

    /// Injecte une requête HTTP vers un COG via le tunnel Relay (Phase 2).
    ///
    /// Résout cog_id → session_id, envoie la requête brute, attend la réponse (timeout 30s).
    /// Retourne Err si le COG n'a pas de session active ou si timeout.
    pub async fn inject_http_request(
        &self,
        cog_id: &str,
        request_bytes: Bytes,
    ) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        // Résoudre cog_id → session
        let session = self
            .sessions
            .get_by_cog_id(cog_id)
            .await
            .ok_or_else(|| format!("No active Relay session for cog_id: {}", cog_id))?;

        let session_id = {
            let s = session.read().await;
            if !s.is_active() {
                return Err("Session not active".into());
            }
            s.id
        };

        let rx = self.http_injector.register(session_id).await;

        self.tunnel_manager
            .send_to_session(session_id, request_bytes)
            .await
            .map_err(|e| format!("Failed to send to session: {}", e))?;

        // Attendre la réponse (timeout 30s)
        match tokio::time::timeout(
            tokio::time::Duration::from_secs(30),
            rx,
        )
        .await
        {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) => Err("COG closed connection before response".into()),
            Err(_) => Err("HTTP tunnel response timeout (30s)".into()),
        }
    }

    /// Configure TLS.
    async fn setup_tls(
        config: &OriginConfig,
    ) -> Result<TlsAcceptor, Box<dyn std::error::Error + Send + Sync>> {
        let cert_path = Path::new(&config.tls.cert_path);
        let cert_pem = tokio::fs::read(cert_path).await?;
        let certs = rustls_pemfile::certs(&mut &*cert_pem)
            .collect::<Result<Vec<_>, _>>()?;

        let key_path = Path::new(&config.tls.key_path);
        let key_pem = tokio::fs::read(key_path).await?;
        let key = rustls_pemfile::private_key(&mut &*key_pem)?
            .ok_or("No private key found")?;

        let tls_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)?;

        Ok(TlsAcceptor::from(Arc::new(tls_config)))
    }

    /// Démarre le serveur.
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr: SocketAddr = format!("{}:{}", self.config.relay.host, self.config.relay.port)
            .parse()?;

        let listener = TcpListener::bind(addr).await?;
        info!("🔐 Relay server listening on {}", addr);

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
        // Nettoyage des sessions expirées
        let sessions = Arc::clone(&self.sessions);
        let metrics = Arc::clone(&self.metrics);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                let cleaned = sessions.cleanup_expired().await;
                if cleaned > 0 {
                    info!("Cleaned up {} expired sessions", cleaned);
                    for _ in 0..cleaned {
                        metrics.record_session_expired();
                    }
                }
            }
        });

        // Nettoyage du rate limiter
        let rate_limiter = Arc::clone(&self.rate_limiter);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
                rate_limiter.cleanup().await;
            }
        });

        // Nettoyage des tunnels
        let tunnel_manager = Arc::clone(&self.tunnel_manager);
        let metrics = Arc::clone(&self.metrics);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                let cleaned = tunnel_manager.cleanup_expired().await;
                if cleaned > 0 {
                    info!("Cleaned up {} expired tunnels", cleaned);
                    for _ in 0..cleaned {
                        metrics.record_tunnel_closed();
                    }
                }
            }
        });
    }

    /// Gère une nouvelle connexion entrante.
    async fn handle_new_connection(&self, stream: TcpStream, peer_addr: SocketAddr) {
        let ip = peer_addr.ip();

        // Vérifier le rate limit
        match self.rate_limiter.check_connection(ip).await {
            RateLimitResult::Allowed => {}
            RateLimitResult::Exceeded { current, limit } => {
                warn!(
                    "Connection from {} rejected: rate limit exceeded ({}/{})",
                    peer_addr, current, limit
                );
                self.metrics.record_rejected_connection();
                return;
            }
            RateLimitResult::Blocked { reason, retry_after } => {
                warn!(
                    "Connection from {} blocked: {} (retry after {}s)",
                    peer_addr, reason, retry_after
                );
                self.metrics.record_rejected_connection();
                return;
            }
        }

        self.metrics.record_connection();
        debug!("New connection from {}", peer_addr);

        // Cloner les références nécessaires
        let tls_acceptor = self.tls_acceptor.clone();
        let sessions = Arc::clone(&self.sessions);
        let permis_registry = self.permis_registry.clone();
        let verifier = Arc::clone(&self.verifier);
        let config = Arc::clone(&self.config);
        let tracker_addresses = self.tracker_addresses.clone();
        let tracker_signature = self.tracker_signature;
        let rate_limiter = Arc::clone(&self.rate_limiter);
        let tunnel_manager = Arc::clone(&self.tunnel_manager);
        let http_injector = Arc::clone(&self.http_injector);
        let metrics = Arc::clone(&self.metrics);

        tokio::spawn(async move {
            let result = Self::handle_connection(
                stream,
                peer_addr,
                tls_acceptor,
                sessions,
                permis_registry,
                verifier,
                config,
                tracker_addresses,
                tracker_signature,
                rate_limiter,
                tunnel_manager,
                http_injector,
                Arc::clone(&metrics),
            )
            .await;

            if let Err(e) = result {
                debug!("Connection error from {}: {}", peer_addr, e);
            }

            metrics.record_connection_closed();
        });
    }

    /// Gère une connexion individuelle.
    async fn handle_connection(
        stream: TcpStream,
        peer_addr: SocketAddr,
        tls_acceptor: Option<TlsAcceptor>,
        sessions: Arc<SessionManager>,
        permis_registry: Option<Arc<PermisRegistry>>,
        verifier: Arc<Verifier>,
        config: Arc<OriginConfig>,
        tracker_addresses: Bytes,
        tracker_signature: [u8; ED25519_SIGNATURE_SIZE],
        rate_limiter: Arc<RateLimiter>,
        tunnel_manager: Arc<TunnelManager>,
        http_injector: Arc<HttpInjector>,
        metrics: Arc<RelayMetrics>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Appliquer TLS
        let mut stream: Box<dyn AsyncReadWriteExt> = if let Some(acceptor) = tls_acceptor {
            match acceptor.accept(stream).await {
                Ok(tls_stream) => Box::new(tls_stream),
                Err(e) => {
                    warn!("TLS handshake failed from {}: {}", peer_addr, e);
                    metrics.record_tls_error();
                    return Err(e.into());
                }
            }
        } else {
            Box::new(stream)
        };

        // Créer la session
        let session = sessions.create(peer_addr).await;
        let session_id = {
            let s = session.read().await;
            s.id
        };
        metrics.record_session_created();

        info!(
            "Session created for {}: {}",
            peer_addr,
            hex::encode(&session_id[..8])
        );

        // Créer un channel pour les messages sortants du tunnel
        let (tunnel_tx, mut tunnel_rx) = mpsc::channel::<TunnelMessage>(100);
        tunnel_manager.register_outbound(session_id, tunnel_tx).await;

        // Buffer de lecture
        let mut read_buf = BytesMut::with_capacity(8192);

        loop {
            tokio::select! {
                // Lire des données entrantes
                result = stream.read_buf(&mut read_buf) => {
                    match result {
                        Ok(0) => {
                            // Connexion fermée
                            break;
                        }
                        Ok(n) => {
                            metrics.record_message_received(n);

                            // Traiter les trames
                            while let Some(frame) = Self::try_parse_frame(&mut read_buf, &metrics)? {
                                // Vérifier le rate limit des messages
                                let session_id_arr: [u8; 16] = session_id[..16].try_into().unwrap();
                                if !rate_limiter.check_message(&session_id_arr).await.is_allowed() {
                                    warn!("Session {} rate limited", hex::encode(&session_id[..8]));
                                    break;
                                }

                                // Traiter la trame
                                let response = Self::handle_frame(
                                    &frame,
                                    &session,
                                    permis_registry.as_ref(),
                                    &verifier,
                                    &config,
                                    &tracker_addresses,
                                    &tracker_signature,
                                    &sessions,
                                    &tunnel_manager,
                                    &http_injector,
                                    peer_addr.ip(),
                                    &rate_limiter,
                                    &metrics,
                                )
                                .await;

                                // Envoyer la réponse
                                if let Some(response_frame) = response {
                                    let bytes = response_frame.to_bytes();
                                    metrics.record_message_sent(bytes.len());
                                    stream.write_all(&bytes).await?;
                                    stream.flush().await?;
                                }

                                // Vérifier si on doit fermer
                                let state = {
                                    let s = session.read().await;
                                    s.state
                                };

                                if state == SessionState::Closed {
                                    info!("Session closed for {}", peer_addr);
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            debug!("Read error from {}: {}", peer_addr, e);
                            break;
                        }
                    }
                }

                // Envoyer des messages de tunnel
                Some(msg) = tunnel_rx.recv() => {
                    // Construire une trame DATA
                    let frame = Frame::new(MessageType::Data, msg.data);
                    let bytes = frame.to_bytes();
                    metrics.record_message_sent(bytes.len());
                    metrics.record_data_relayed();

                    if stream.write_all(&bytes).await.is_err() {
                        break;
                    }
                    stream.flush().await?;
                }
            }
        }

        // Nettoyer
        tunnel_manager.unregister_outbound(&session_id).await;
        tunnel_manager.close_session_tunnels(&session_id).await;
        sessions.remove(&session_id).await;
        metrics.record_session_closed();
        info!("Connection closed for {}", peer_addr);

        Ok(())
    }

    /// Essaie de parser une trame du buffer.
    fn try_parse_frame(
        buf: &mut BytesMut,
        metrics: &RelayMetrics,
    ) -> Result<Option<Frame>, Box<dyn std::error::Error + Send + Sync>> {
        match Frame::parse(buf) {
            Ok(frame) => Ok(frame),
            Err(e) => {
                metrics.record_parse_error();
                Err(e.into())
            }
        }
    }

    /// Traite une trame reçue.
    async fn handle_frame(
        frame: &Frame,
        session: &Arc<RwLock<Session>>,
        permis_registry: Option<&Arc<PermisRegistry>>,
        verifier: &Arc<Verifier>,
        config: &Arc<OriginConfig>,
        tracker_addresses: &Bytes,
        tracker_signature: &[u8; ED25519_SIGNATURE_SIZE],
        sessions: &Arc<SessionManager>,
        tunnel_manager: &Arc<TunnelManager>,
        http_injector: &Arc<HttpInjector>,
        client_ip: std::net::IpAddr,
        rate_limiter: &Arc<RateLimiter>,
        metrics: &Arc<RelayMetrics>,
    ) -> Option<Frame> {
        debug!("Received {:?}", frame.header.message_type);

        match frame.header.message_type {
            MessageType::Register => {
                Self::handle_register(
                    &frame.payload,
                    session,
                    permis_registry,
                    verifier,
                    config,
                    tracker_addresses,
                    tracker_signature,
                    sessions,
                    client_ip,
                    rate_limiter,
                    metrics,
                )
                .await
            }
            MessageType::CoreKey => {
                Self::handle_core_key(
                    &frame.payload,
                    session,
                    verifier,
                    config,
                    permis_registry,
                    tracker_addresses,
                    tracker_signature,
                    sessions,
                    client_ip,
                    rate_limiter,
                    metrics,
                )
                .await
            }
            MessageType::ServiceBlock => {
                Self::handle_service_block(&frame.payload, session, verifier, metrics).await
            }
            MessageType::Heartbeat => {
                metrics.record_heartbeat();
                Self::handle_heartbeat(&frame.payload, session).await
            }
            MessageType::Close => {
                Self::handle_close(session).await;
                None
            }
            MessageType::Data => {
                Self::handle_data(&frame.payload, session, tunnel_manager, http_injector, metrics).await
            }
            _ => {
                warn!("Unhandled message type: {:?}", frame.header.message_type);
                metrics.record_protocol_error();
                None
            }
        }
    }

    /// Traite un message REGISTER.
    async fn handle_register(
        payload: &Bytes,
        session: &Arc<RwLock<Session>>,
        permis_registry: Option<&Arc<PermisRegistry>>,
        _verifier: &Arc<Verifier>,
        config: &Arc<OriginConfig>,
        tracker_addresses: &Bytes,
        tracker_signature: &[u8; ED25519_SIGNATURE_SIZE],
        sessions: &Arc<SessionManager>,
        client_ip: std::net::IpAddr,
        rate_limiter: &Arc<RateLimiter>,
        metrics: &Arc<RelayMetrics>,
    ) -> Option<Frame> {
        // Parser le payload
        let register = match RegisterPayload::parse(payload.clone()) {
            Ok(r) => r,
            Err(e) => {
                warn!("Failed to parse REGISTER: {}", e);
                metrics.record_registration(false);
                let err = RegisterErrPayload::new(
                    RegisterErrorCode::InternalError,
                    Bytes::from(format!("Parse error: {}", e)),
                );
                return Some(Frame::new(MessageType::RegisterErr, err.to_bytes()));
            }
        };

        // Extraire le cog_id
        let cog_id = match register.cog_id_str() {
            Some(id) => id,
            None => {
                metrics.record_registration(false);
                rate_limiter.record_failed_registration(client_ip).await;
                let err = RegisterErrPayload::new(
                    RegisterErrorCode::InternalError,
                    Bytes::from_static(b"Invalid cog_id encoding"),
                );
                return Some(Frame::new(MessageType::RegisterErr, err.to_bytes()));
            }
        };

        info!("REGISTER request from cog_id: {}", cog_id);

        // Vérifier si le cog_id est déjà enregistré
        if sessions.is_cog_registered(&cog_id).await {
            metrics.record_registration(false);
            let err = RegisterErrPayload::new(
                RegisterErrorCode::CogIdConflict,
                Bytes::from_static(b"cog_id already registered"),
            );
            return Some(Frame::new(MessageType::RegisterErr, err.to_bytes()));
        }

        // Vérifier le timestamp (anti-rejeu)
        let now = chrono::Utc::now().timestamp() as u64;
        let window = config.policies.timestamp_window_seconds;
        if register.timestamp < now.saturating_sub(window)
            || register.timestamp > now.saturating_add(window)
        {
            metrics.record_registration(false);
            rate_limiter.record_failed_registration(client_ip).await;
            let err = RegisterErrPayload::new(
                RegisterErrorCode::AuthFailed,
                Bytes::from_static(b"Timestamp out of window"),
            );
            return Some(Frame::new(MessageType::RegisterErr, err.to_bytes()));
        }

        // Mettre à jour la session
        let session_id = {
            let mut s = session.write().await;
            s.set_cog_info(
                cog_id.clone(),
                register.cog_type,
                register.os_type,
                register.core_version_str().unwrap_or_default(),
                register.passport_type,
                if register.parent_cog_id.is_empty() {
                    None
                } else {
                    std::str::from_utf8(&register.parent_cog_id)
                        .ok()
                        .map(String::from)
                },
            );
            s.set_environment_health(register.environment_health.clone());
            s.start_phase_a();
            s.id
        };
        sessions.register_cog(cog_id.clone(), session_id).await;

        // Mode strict : demander CORE_KEY (Phase A) puis Phase C, pas de REGISTER_OK ici
        if config.relay.strict_verification {
            let response = VerifyResultPayload::new(
                session_id,
                VerifyPhase::PhaseA,
                VerifyResult::ExtendedRequired,
                Bytes::from_static(b"Send CORE_KEY"),
            );
            return Some(Frame::new(MessageType::VerifyResult, response.to_bytes()));
        }

        // Mode non strict : simulation Phase A/C et REGISTER_OK immédiat
        let _core_version = register.core_version_str().unwrap_or_default();
        {
            let mut s = session.write().await;
            s.phase_a_passed = true;
            s.phase_c_passed = true;
        }
        metrics.record_phase_a(true);
        metrics.record_phase_c(true);

        let permis_id = {
            let s = session.read().await;
            s.generate_permis_id()
        };
        let permis_expires = chrono::Utc::now() + chrono::Duration::hours(24);

        let mut session_key = [0u8; 32];
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut session_key);

        {
            let mut s = session.write().await;
            let permis = PermisInfo {
                permis_id: permis_id.clone(),
                issued_at: chrono::Utc::now(),
                expires_at: permis_expires,
                scope: Bytes::from_static(b"{}"),
            };
            s.activate(permis, session_key);
        }

        metrics.record_registration(true);
        metrics.record_permit_issued();
        rate_limiter.reset_failed_registrations(client_ip).await;

        info!(
            "COG {} registered successfully, Permis: {}",
            cog_id, permis_id
        );

        if let Some(registry) = permis_registry {
            registry
                .register(permis_id.clone(), permis_expires)
                .await;
        }

        let response = RegisterOkPayload::build(
            session_id,
            Bytes::from(permis_id),
            permis_expires.timestamp() as u64,
            Bytes::from_static(b"{}"),
            tracker_addresses.clone(),
            *tracker_signature,
        );

        Some(Frame::new(MessageType::RegisterOk, response.to_bytes()))
    }

    /// Traite un message CORE_KEY.
    async fn handle_core_key(
        payload: &Bytes,
        session: &Arc<RwLock<Session>>,
        verifier: &Arc<Verifier>,
        config: &Arc<OriginConfig>,
        permis_registry: Option<&Arc<PermisRegistry>>,
        tracker_addresses: &Bytes,
        tracker_signature: &[u8; ED25519_SIGNATURE_SIZE],
        _sessions: &Arc<SessionManager>,
        client_ip: std::net::IpAddr,
        rate_limiter: &Arc<RateLimiter>,
        metrics: &Arc<RelayMetrics>,
    ) -> Option<Frame> {
        let core_key = match CoreKeyPayload::parse(payload.clone()) {
            Ok(k) => k,
            Err(e) => {
                warn!("Failed to parse CORE_KEY: {}", e);
                metrics.record_parse_error();
                return None;
            }
        };

        let core_version = {
            let s = session.read().await;
            s.core_version.clone().unwrap_or_default()
        };

        let result = verifier.verify_phase_a(&core_version, &core_key.key).await;
        let passed_a = result == VerifyResult::Ok;
        metrics.record_phase_a(passed_a);

        if passed_a {
            let mut s = session.write().await;
            s.complete_phase_a();
        }

        // Mode strict : après Phase A OK, faire Phase C puis REGISTER_OK
        if passed_a && config.relay.strict_verification {
            let (environment_health, cog_id) = {
                let s = session.read().await;
                let health = s
                    .environment_health
                    .clone()
                    .unwrap_or_else(Bytes::new);
                let cog_id = s.cog_id.clone();
                (health, cog_id)
            };
            let phase_c_result = verifier.verify_phase_c(&environment_health).await;
            let phase_c_ok = phase_c_result == VerifyResult::Ok;
            metrics.record_phase_c(phase_c_ok);

            if !phase_c_ok {
                let mut s = session.write().await;
                s.state = SessionState::VerifyingPhaseC;
                let response = VerifyResultPayload::new(
                    core_key.session_id,
                    VerifyPhase::PhaseC,
                    VerifyResult::Fail,
                    Bytes::from_static(b"Environment health check failed"),
                );
                return Some(Frame::new(MessageType::VerifyResult, response.to_bytes()));
            }

            let session_id = {
                let s = session.read().await;
                s.id
            };
            let cog_id = cog_id.unwrap_or_default();
            let permis_id = {
                let s = session.read().await;
                s.generate_permis_id()
            };
            let permis_expires = chrono::Utc::now() + chrono::Duration::hours(24);

            let mut session_key = [0u8; 32];
            rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut session_key);

            {
                let mut s = session.write().await;
                let permis = PermisInfo {
                    permis_id: permis_id.clone(),
                    issued_at: chrono::Utc::now(),
                    expires_at: permis_expires,
                    scope: Bytes::from_static(b"{}"),
                };
                s.phase_c_passed = true;
                s.activate(permis, session_key);
            }

            metrics.record_registration(true);
            metrics.record_permit_issued();
            metrics.record_phase_c(true);
            rate_limiter.reset_failed_registrations(client_ip).await;

            info!(
                "COG {} registered (strict 3-phase), Permis: {}",
                cog_id, permis_id
            );

            if let Some(registry) = permis_registry {
                registry
                    .register(permis_id.clone(), permis_expires)
                    .await;
            }

            let response = RegisterOkPayload::build(
                session_id,
                Bytes::from(permis_id),
                permis_expires.timestamp() as u64,
                Bytes::from_static(b"{}"),
                tracker_addresses.clone(),
                *tracker_signature,
            );
            return Some(Frame::new(MessageType::RegisterOk, response.to_bytes()));
        }

        let response = VerifyResultPayload::new(
            core_key.session_id,
            VerifyPhase::PhaseA,
            result,
            Bytes::new(),
        );
        Some(Frame::new(MessageType::VerifyResult, response.to_bytes()))
    }

    /// Traite un message SERVICE_BLOCK.
    async fn handle_service_block(
        payload: &Bytes,
        session: &Arc<RwLock<Session>>,
        verifier: &Arc<Verifier>,
        metrics: &Arc<RelayMetrics>,
    ) -> Option<Frame> {
        let block = match ServiceBlockPayload::parse(payload.clone()) {
            Ok(b) => b,
            Err(e) => {
                warn!("Failed to parse SERVICE_BLOCK: {}", e);
                metrics.record_parse_error();
                return None;
            }
        };

        let service_id = std::str::from_utf8(&block.service_id)
            .unwrap_or("unknown")
            .to_string();

        let result = verifier
            .verify_phase_b(&service_id, block.block_index, &block.encrypted_block)
            .await;

        let passed = result == VerifyResult::Ok;
        metrics.record_phase_b(passed);

        let response = VerifyResultPayload::new(
            block.session_id,
            VerifyPhase::PhaseB,
            result,
            Bytes::new(),
        )
        .with_service_id(block.service_id);

        if passed {
            let mut s = session.write().await;
            s.add_verified_service(service_id);
        }

        Some(Frame::new(MessageType::VerifyResult, response.to_bytes()))
    }

    /// Traite un message HEARTBEAT.
    async fn handle_heartbeat(
        payload: &Bytes,
        session: &Arc<RwLock<Session>>,
    ) -> Option<Frame> {
        {
            let mut s = session.write().await;
            s.touch();
        }
        Some(Frame::new(MessageType::HeartbeatAck, payload.clone()))
    }

    /// Traite un message CLOSE.
    async fn handle_close(session: &Arc<RwLock<Session>>) {
        let mut s = session.write().await;
        s.close();
        info!("Session closed by client");
    }

    /// Traite un message DATA.
    async fn handle_data(
        payload: &Bytes,
        session: &Arc<RwLock<Session>>,
        tunnel_manager: &Arc<TunnelManager>,
        http_injector: &Arc<HttpInjector>,
        metrics: &Arc<RelayMetrics>,
    ) -> Option<Frame> {
        // Vérifier que la session est active
        let (is_active, session_id, sequence) = {
            let mut s = session.write().await;
            if !s.is_active() {
                return None;
            }
            s.touch();
            s.messages_received += 1;
            let seq = s.sequence_number;
            s.sequence_number = s.sequence_number.wrapping_add(1);
            (true, s.id, seq)
        };

        if !is_active {
            warn!("DATA received on inactive session");
            return None;
        }

        // Phase 2 : si c'est une réponse à une requête HTTP injectée par Origin, compléter le oneshot
        if http_injector.complete(session_id, payload.clone()).await {
            debug!("HTTP response received via tunnel for session {}", hex::encode(&session_id[..8]));
            return None;
        }

        // Relayer via le tunnel COG-COG
        if let Err(e) = tunnel_manager
            .relay_data(session_id, payload.clone(), sequence)
            .await
        {
            debug!("Failed to relay data: {}", e);
        } else {
            metrics.record_data_relayed();
        }

        None
    }
}

/// Trait helper pour le polymorphisme TLS/non-TLS.
trait AsyncReadWriteExt: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send {}

impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send> AsyncReadWriteExt for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relay_metrics_created() {
        let metrics = RelayMetrics::new();
        assert_eq!(metrics.active_connections.load(std::sync::atomic::Ordering::Relaxed), 0);
    }
}
