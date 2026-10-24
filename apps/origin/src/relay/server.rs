//! @id     origin_relay_server
//! @do     definir_http_injector
//! @role   orchestration
//! @layer  app
//! @human  origin — module « relay server ».

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

use miyuwebway_participant::identite::{cog_id_bien_forme, verifier_register};
use miyuwebway_participant::protocol::{
    MAGIC_REPONSE_INJECTEE, MAGIC_REQUETE_INJECTEE, TAILLE_ID_REQUETE,
};

use super::identites::{RefusEpinglage, RegistreIdentites};
use super::metrics::RelayMetrics;
use super::permis_registry::PermisRegistry;
use super::rate_limiter::{RateLimitResult, RateLimiter, RateLimiterConfig};
use super::session::{PermisInfo, Session, SessionManager, SessionState};
use super::tunnel::{TunnelManager, TunnelMessage};
use super::verification::Verifier;
use crate::config::OriginConfig;
use crate::protocol::{
    CoreKeyPayload, Frame, MessageType, RegisterErrPayload, RegisterErrorCode, RegisterOkPayload,
    RegisterPayload, ServiceBlockPayload, VerifyPhase, VerifyResult, VerifyResultPayload,
    ED25519_SIGNATURE_SIZE, SESSION_ID_SIZE,
};

/// Nombre maximal de requêtes injectées en attente (toutes sessions, C-13).
const MAX_INJECTIONS_EN_ATTENTE: usize = 10_000;

/// Nombre maximal de requêtes injectées en attente pour une même session (C-13) : un COG
/// lent ou muet ne peut pas épuiser le plafond global au détriment des autres.
const MAX_INJECTIONS_PAR_SESSION: usize = 64;

/// Issue de la remise d'une trame DATA à l'injecteur.
#[derive(Debug, PartialEq, Eq)]
enum RemiseInjection {
    /// Réponse livrée à la requête en attente correspondante.
    Livree,
    /// Réponse encadrée mais id inconnu, expiré ou d'une autre session : jetée.
    Rejetee,
    /// Trame sans cadre de réponse injectée (trafic de tunnel COG-COG).
    NonInjection,
}

/// Pending HTTP injections (Phase 2 : Origin → COG via tunnel).
///
/// O-06 : chaque requête reçoit un identifiant aléatoire de 128 bits, transmis au COG
/// (`MWHQ` + id) et rappelé dans la réponse (`MWHR` + id). Les attentes sont indexées par
/// cet id et liées à la session émettrice : une réponse d'id inconnu, ou venant d'une autre
/// session, est jetée. Deux visiteurs concurrents ne peuvent donc jamais recevoir la
/// réponse destinée à l'autre.
/// Attentes : id de requête → (session émettrice, canal de réponse), plus le nombre
/// d'attentes par session (plafond par session, purge à la fermeture).
#[derive(Default)]
struct AttentesInjection {
    par_id: HashMap<[u8; TAILLE_ID_REQUETE], ([u8; SESSION_ID_SIZE], oneshot::Sender<Bytes>)>,
    par_session: HashMap<[u8; SESSION_ID_SIZE], usize>,
}

impl AttentesInjection {
    fn is_empty(&self) -> bool {
        self.par_id.is_empty()
    }

    /// Retire une attente et décrémente le compteur de sa session.
    fn retirer(
        &mut self,
        id: &[u8; TAILLE_ID_REQUETE],
    ) -> Option<([u8; SESSION_ID_SIZE], oneshot::Sender<Bytes>)> {
        let entree = self.par_id.remove(id)?;
        if let Some(n) = self.par_session.get_mut(&entree.0) {
            *n = n.saturating_sub(1);
            if *n == 0 {
                self.par_session.remove(&entree.0);
            }
        }
        Some(entree)
    }
}

/// Refus d'enregistrement d'une requête injectée (C-11).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
enum RefusInjection {
    /// Plafond global atteint.
    #[error("too many pending tunnel requests")]
    PlafondGlobal,
    /// Plafond de la session atteint.
    #[error("too many pending tunnel requests for this COG")]
    PlafondSession,
}

struct HttpInjector {
    pending: Arc<RwLock<AttentesInjection>>,
}

impl HttpInjector {
    fn new() -> Self {
        Self {
            pending: Arc::new(RwLock::new(AttentesInjection::default())),
        }
    }

    /// Enregistre une attente de réponse ; retourne l'id de requête et le receiver.
    async fn register(
        &self,
        session_id: [u8; SESSION_ID_SIZE],
    ) -> Result<([u8; TAILLE_ID_REQUETE], oneshot::Receiver<Bytes>), RefusInjection> {
        let (tx, rx) = oneshot::channel();
        let mut pending = self.pending.write().await;
        if pending.par_id.len() >= MAX_INJECTIONS_EN_ATTENTE {
            return Err(RefusInjection::PlafondGlobal);
        }
        if pending.par_session.get(&session_id).copied().unwrap_or(0)
            >= MAX_INJECTIONS_PAR_SESSION
        {
            return Err(RefusInjection::PlafondSession);
        }
        let mut id = [0u8; TAILLE_ID_REQUETE];
        loop {
            rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut id);
            if !pending.par_id.contains_key(&id) {
                break;
            }
        }
        pending.par_id.insert(id, (session_id, tx));
        *pending.par_session.entry(session_id).or_insert(0) += 1;
        Ok((id, rx))
    }

    /// Retire une attente (délai dépassé, échec d'envoi).
    async fn abandon(&self, id: &[u8; TAILLE_ID_REQUETE]) {
        self.pending.write().await.retirer(id);
    }

    /// Purge toutes les attentes d'une session fermée ; les demandeurs reçoivent une
    /// erreur immédiate (canal fermé) au lieu d'attendre le délai.
    async fn purger_session(&self, session_id: &[u8; SESSION_ID_SIZE]) -> usize {
        let mut pending = self.pending.write().await;
        let avant = pending.par_id.len();
        pending.par_id.retain(|_, (s, _)| s != session_id);
        pending.par_session.remove(session_id);
        avant - pending.par_id.len()
    }

    /// Livre une réponse encadrée à l'attente correspondante (id **et** session).
    async fn complete(&self, session_id: [u8; SESSION_ID_SIZE], data: &Bytes) -> RemiseInjection {
        let Some(reste) = data.strip_prefix(MAGIC_REPONSE_INJECTEE.as_slice()) else {
            return RemiseInjection::NonInjection;
        };
        if reste.len() < TAILLE_ID_REQUETE {
            return RemiseInjection::Rejetee;
        }
        let mut id = [0u8; TAILLE_ID_REQUETE];
        id.copy_from_slice(&reste[..TAILLE_ID_REQUETE]);
        let tx = {
            let mut pending = self.pending.write().await;
            match pending.par_id.get(&id) {
                Some((session_attendue, _)) if *session_attendue == session_id => {
                    pending.retirer(&id).map(|(_, tx)| tx)
                }
                Some(_) => {
                    warn!("Tunnel response with request id from another session dropped");
                    None
                }
                None => None,
            }
        };
        match tx {
            Some(tx) => {
                let corps = data.slice(MAGIC_REPONSE_INJECTEE.len() + TAILLE_ID_REQUETE..);
                let _ = tx.send(corps);
                RemiseInjection::Livree
            }
            None => RemiseInjection::Rejetee,
        }
    }
}

/// Encadre une requête HTTP injectée : `MWHQ` + id + requête.
fn encadrer_requete(id: &[u8; TAILLE_ID_REQUETE], requete: &[u8]) -> Bytes {
    let mut sortie = BytesMut::with_capacity(4 + TAILLE_ID_REQUETE + requete.len());
    sortie.extend_from_slice(MAGIC_REQUETE_INJECTEE);
    sortie.extend_from_slice(id);
    sortie.extend_from_slice(requete);
    sortie.freeze()
}

/// Contexte partagé par toutes les connexions du Relay.
struct ContexteRelay {
    sessions: Arc<SessionManager>,
    permis_registry: Option<Arc<PermisRegistry>>,
    identites: Arc<RegistreIdentites>,
    identite_relay: String,
    verifier: Arc<Verifier>,
    config: Arc<OriginConfig>,
    tracker_addresses: Bytes,
    tracker_signature: [u8; ED25519_SIGNATURE_SIZE],
    rate_limiter: Arc<RateLimiter>,
    tunnel_manager: Arc<TunnelManager>,
    http_injector: Arc<HttpInjector>,
    metrics: Arc<RelayMetrics>,
}

/// Serveur Relay.
pub struct RelayServer {
    /// Contexte partagé.
    ctx: Arc<ContexteRelay>,
    /// Accepteur TLS.
    tls_acceptor: Option<TlsAcceptor>,
}

impl RelayServer {
    /// Crée un nouveau serveur relay.
    ///
    /// Si `permis_registry` est fourni, les permis délivrés y sont enregistrés (liés au
    /// `cog_id` prouvé) pour que le Tracker puisse vérifier les ANNOUNCE.
    /// `identites` est le registre TOFU des clés COG, partagé avec le Tracker.
    pub async fn new(
        config: Arc<OriginConfig>,
        permis_registry: Option<Arc<PermisRegistry>>,
        identites: Arc<RegistreIdentites>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let tls_acceptor = Self::setup_tls(&config).await?;
        let mut serveur = Self::sans_tls(config, permis_registry, identites)?;
        serveur.tls_acceptor = Some(tls_acceptor);
        serveur.ctx.verifier.load_keys().await.ok();
        Ok(serveur)
    }

    /// Construit le serveur sans accepteur TLS (tests et assemblage).
    fn sans_tls(
        config: Arc<OriginConfig>,
        permis_registry: Option<Arc<PermisRegistry>>,
        identites: Arc<RegistreIdentites>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if !config.relay.strict_verification {
            warn!(
                "Relay en mode NON STRICT (relay.strict_verification = false) : la Phase A \
                 (clé Cores) et la Phase C sont ignorées. La preuve d'identité signée reste exigée."
            );
        }

        let sessions = Arc::new(SessionManager::new(i64::from(
            config.limits.tunnel_timeout_seconds,
        )));
        let verifier = Arc::new(Verifier::new(config.cores.keys_dir.clone()));

        // Préparer la liste des trackers
        let tracker_addresses = serde_json::to_vec(&vec![format!(
            "{}:{}",
            config.identity.ip, config.tracker.port
        )])?;

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

        let tunnel_manager = Arc::new(TunnelManager::new(i64::from(
            config.limits.tunnel_timeout_seconds,
        )));

        let identite_relay = config.relay.identite_relay(&config.identity);
        info!("Relay identity for REGISTER proofs: {}", identite_relay);

        Ok(Self {
            ctx: Arc::new(ContexteRelay {
                sessions,
                permis_registry,
                identites,
                identite_relay,
                verifier,
                config,
                tracker_addresses: Bytes::from(tracker_addresses),
                tracker_signature: [0u8; ED25519_SIGNATURE_SIZE],
                rate_limiter,
                tunnel_manager,
                http_injector: Arc::new(HttpInjector::new()),
                metrics: Arc::new(RelayMetrics::new()),
            }),
            tls_acceptor: None,
        })
    }

    /// Récupère les métriques.
    #[must_use]
    pub fn metrics(&self) -> Arc<RelayMetrics> {
        Arc::clone(&self.ctx.metrics)
    }

    /// Récupère le gestionnaire de sessions.
    #[must_use]
    pub fn sessions(&self) -> Arc<SessionManager> {
        Arc::clone(&self.ctx.sessions)
    }

    /// Récupère le gestionnaire de tunnels.
    #[must_use]
    pub fn tunnels(&self) -> Arc<TunnelManager> {
        Arc::clone(&self.ctx.tunnel_manager)
    }

    /// Injecte une requête HTTP vers un COG via le tunnel Relay (Phase 2).
    ///
    /// Résout `cog_id` → `session_id`, envoie la requête encadrée par un id unique (O-06),
    /// attend la réponse portant ce même id (timeout 30s).
    /// Retourne Err si le COG n'a pas de session active ou si timeout.
    pub async fn inject_http_request(
        &self,
        cog_id: &str,
        request_bytes: Bytes,
    ) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        let ctx = &self.ctx;
        // Résoudre cog_id → session
        let session = ctx
            .sessions
            .get_by_cog_id(cog_id)
            .await
            .ok_or_else(|| format!("No active Relay session for cog_id: {cog_id}"))?;

        let session_id = {
            let s = session.read().await;
            if !s.is_active() || !s.identite_verifiee {
                return Err("Session not active".into());
            }
            s.id
        };

        let (id_requete, rx) = ctx
            .http_injector
            .register(session_id)
            .await
            .map_err(|e| e.to_string())?;

        if let Err(e) = ctx
            .tunnel_manager
            .send_to_session(session_id, encadrer_requete(&id_requete, &request_bytes))
            .await
        {
            ctx.http_injector.abandon(&id_requete).await;
            return Err(format!("Failed to send to session: {e}").into());
        }

        // Attendre la réponse (timeout 30s)
        let resultat = tokio::time::timeout(tokio::time::Duration::from_secs(30), rx).await;
        match resultat {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) => {
                ctx.http_injector.abandon(&id_requete).await;
                Err("COG closed connection before response".into())
            }
            Err(_) => {
                ctx.http_injector.abandon(&id_requete).await;
                Err("HTTP tunnel response timeout (30s)".into())
            }
        }
    }

    /// Configure TLS.
    async fn setup_tls(
        config: &OriginConfig,
    ) -> Result<TlsAcceptor, Box<dyn std::error::Error + Send + Sync>> {
        let cert_path = Path::new(&config.tls.cert_path);
        let cert_pem = tokio::fs::read(cert_path).await?;
        let certs = rustls_pemfile::certs(&mut &*cert_pem).collect::<Result<Vec<_>, _>>()?;

        let key_path = Path::new(&config.tls.key_path);
        let key_pem = tokio::fs::read(key_path).await?;
        let key = rustls_pemfile::private_key(&mut &*key_pem)?.ok_or("No private key found")?;

        let tls_versions: Vec<&'static rustls::SupportedProtocolVersion> =
            match config.tls.min_version.as_str() {
                "1.3" => vec![&rustls::version::TLS13],
                _ => vec![&rustls::version::TLS12, &rustls::version::TLS13],
            };

        let tls_config = rustls::ServerConfig::builder_with_protocol_versions(&tls_versions)
            .with_no_client_auth()
            .with_single_cert(certs, key)?;

        Ok(TlsAcceptor::from(Arc::new(tls_config)))
    }

    /// Démarre le serveur.
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr: SocketAddr = format!(
            "{}:{}",
            self.ctx.config.relay.host, self.ctx.config.relay.port
        )
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
                    self.ctx.metrics.record_internal_error();
                }
            }
        }
    }

    /// Démarre les tâches de maintenance.
    fn start_maintenance_tasks(&self) {
        // Nettoyage des sessions expirées
        let ctx = Arc::clone(&self.ctx);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                let cleaned = ctx.sessions.cleanup_expired().await;
                if cleaned > 0 {
                    info!("Cleaned up {} expired sessions", cleaned);
                    for _ in 0..cleaned {
                        ctx.metrics.record_session_expired();
                    }
                }
                // O-15 : purge des permis expirés.
                if let Some(registry) = &ctx.permis_registry {
                    let purges = registry.cleanup_expired().await;
                    for _ in 0..purges {
                        ctx.metrics.record_permit_expired();
                    }
                }
            }
        });

        // Nettoyage du rate limiter
        let rate_limiter = Arc::clone(&self.ctx.rate_limiter);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
                rate_limiter.cleanup().await;
            }
        });

        // Nettoyage des tunnels
        let tunnel_manager = Arc::clone(&self.ctx.tunnel_manager);
        let metrics = Arc::clone(&self.ctx.metrics);
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
        let ctx = Arc::clone(&self.ctx);

        // Vérifier le rate limit
        match ctx.rate_limiter.check_connection(ip).await {
            RateLimitResult::Allowed => {}
            RateLimitResult::Exceeded { current, limit } => {
                warn!(
                    "Connection from {} rejected: rate limit exceeded ({}/{})",
                    peer_addr, current, limit
                );
                ctx.metrics.record_rejected_connection();
                return;
            }
            RateLimitResult::Blocked {
                reason,
                retry_after,
            } => {
                warn!(
                    "Connection from {} blocked: {} (retry after {}s)",
                    peer_addr, reason, retry_after
                );
                ctx.metrics.record_rejected_connection();
                return;
            }
        }

        ctx.metrics.record_connection();
        debug!("New connection from {}", peer_addr);

        let tls_acceptor = self.tls_acceptor.clone();
        tokio::spawn(async move {
            let metrics = Arc::clone(&ctx.metrics);
            let result = Self::handle_connection(stream, peer_addr, tls_acceptor, ctx).await;

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
        ctx: Arc<ContexteRelay>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Appliquer TLS
        let mut stream: Box<dyn AsyncReadWriteExt> = if let Some(acceptor) = tls_acceptor {
            match acceptor.accept(stream).await {
                Ok(tls_stream) => Box::new(tls_stream),
                Err(e) => {
                    warn!("TLS handshake failed from {}: {}", peer_addr, e);
                    ctx.metrics.record_tls_error();
                    return Err(e.into());
                }
            }
        } else {
            Box::new(stream)
        };

        // Créer la session
        let session = ctx.sessions.create(peer_addr).await;
        let (session_id, defi) = {
            let s = session.read().await;
            (s.id, s.defi)
        };
        ctx.metrics.record_session_created();

        info!(
            "Session created for {}: {}",
            peer_addr,
            hex::encode(&session_id[..8])
        );

        // O-01 : ouvrir la session par le défi d'identité (32 octets aléatoires).
        let challenge = Frame::new(MessageType::Challenge, Bytes::copy_from_slice(&defi));
        let bytes = challenge.to_bytes();
        ctx.metrics.record_message_sent(bytes.len());
        stream.write_all(&bytes).await?;
        stream.flush().await?;

        // Créer un channel pour les messages sortants du tunnel
        let (tunnel_tx, mut tunnel_rx) = mpsc::channel::<TunnelMessage>(100);
        ctx.tunnel_manager
            .register_outbound(session_id, tunnel_tx)
            .await;

        // Buffer de lecture
        let mut read_buf = BytesMut::with_capacity(8192);

        let resultat: Result<(), Box<dyn std::error::Error + Send + Sync>> = async {
            loop {
                tokio::select! {
                    // Lire des données entrantes
                    result = stream.read_buf(&mut read_buf) => {
                        match result {
                            Ok(0) => break,
                            Ok(n) => {
                                ctx.metrics.record_message_received(n);

                                // Traiter les trames
                                while let Some(frame) = Self::try_parse_frame(&mut read_buf, &ctx.metrics)? {
                                    if !ctx.rate_limiter.check_message(&session_id).await.is_allowed() {
                                        warn!("Session {} rate limited", hex::encode(&session_id[..8]));
                                        return Ok(());
                                    }

                                    let response = Self::handle_frame(&frame, &session, &ctx, peer_addr.ip()).await;

                                    if let Some(response_frame) = response {
                                        let bytes = response_frame.to_bytes();
                                        ctx.metrics.record_message_sent(bytes.len());
                                        stream.write_all(&bytes).await?;
                                        stream.flush().await?;
                                    }

                                    let state = session.read().await.state;
                                    if state == SessionState::Closed {
                                        info!("Session closed for {}", peer_addr);
                                        return Ok(());
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
                        let frame = Frame::new(MessageType::Data, msg.data);
                        let bytes = frame.to_bytes();
                        ctx.metrics.record_message_sent(bytes.len());
                        ctx.metrics.record_data_relayed();

                        if stream.write_all(&bytes).await.is_err() {
                            break;
                        }
                        stream.flush().await?;
                    }
                }
            }
            Ok(())
        }
        .await;

        // Nettoyer (toujours, même en cas d'erreur). Le permis n'est pas révoqué ici : un COG
        // sans tunnel ferme sa session Relay juste après REGISTER_OK et annonce ensuite au
        // Tracker. Le permis est lié au cog_id prouvé et purgé à expiration (O-15 partiel).
        ctx.tunnel_manager.unregister_outbound(&session_id).await;
        ctx.tunnel_manager.close_session_tunnels(&session_id).await;
        ctx.http_injector.purger_session(&session_id).await;
        ctx.sessions.remove(&session_id).await;
        ctx.rate_limiter.remove_session(&session_id).await;
        ctx.metrics.record_session_closed();
        info!("Connection closed for {}", peer_addr);

        resultat
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
        ctx: &ContexteRelay,
        client_ip: std::net::IpAddr,
    ) -> Option<Frame> {
        debug!("Received {:?}", frame.header.message_type);

        match frame.header.message_type {
            MessageType::Register => {
                Self::handle_register(&frame.payload, session, ctx, client_ip).await
            }
            MessageType::CoreKey => {
                Self::handle_core_key(&frame.payload, session, ctx, client_ip).await
            }
            MessageType::ServiceBlock => {
                Self::handle_service_block(&frame.payload, session, ctx).await
            }
            MessageType::Heartbeat => {
                ctx.metrics.record_heartbeat();
                Self::handle_heartbeat(&frame.payload, session).await
            }
            MessageType::Close => {
                Self::handle_close(session).await;
                None
            }
            MessageType::Data => Self::handle_data(&frame.payload, session, ctx).await,
            _ => {
                warn!("Unhandled message type: {:?}", frame.header.message_type);
                ctx.metrics.record_protocol_error();
                None
            }
        }
    }

    /// Construit une trame `REGISTER_ERR`.
    #[allow(clippy::unnecessary_wraps)] // utilisé comme valeur de retour `Option<Frame>`
    fn refus(code: RegisterErrorCode, message: &'static [u8]) -> Option<Frame> {
        let err = RegisterErrPayload::new(code, Bytes::from_static(message));
        Some(Frame::new(MessageType::RegisterErr, err.to_bytes()))
    }

    /// Traite un message REGISTER.
    ///
    /// O-01 : le REGISTER doit porter la clé publique d'identité et la signature du défi
    /// de la session ; le `cog_id` doit être l'empreinte de cette clé ; une clé différente
    /// de celle déjà épinglée pour ce `cog_id` est refusée. L'épinglage (TOFU) n'a lieu qu'à
    /// la délivrance du permis, après les phases du mode strict.
    /// Le défi n'est utilisable qu'une fois par session.
    #[allow(clippy::too_many_lines)] // séquence de contrôles fail-closed, lue d'un bloc
    async fn handle_register(
        payload: &Bytes,
        session: &Arc<RwLock<Session>>,
        ctx: &ContexteRelay,
        client_ip: std::net::IpAddr,
    ) -> Option<Frame> {
        let metrics = &ctx.metrics;
        let rate_limiter = &ctx.rate_limiter;

        // Un seul essai par session et par défi (anti-rejeu, anti-force brute).
        let defi = {
            let mut s = session.write().await;
            if s.defi_consomme || s.state != SessionState::Pending {
                metrics.record_registration(false);
                rate_limiter.record_failed_registration(client_ip).await;
                s.close();
                return Self::refus(RegisterErrorCode::AuthFailed, b"Challenge already used");
            }
            s.defi_consomme = true;
            s.defi
        };

        // Parser le payload
        let register = match RegisterPayload::parse(payload) {
            Ok(r) => r,
            Err(e) => {
                warn!("Failed to parse REGISTER: {}", e);
                metrics.record_registration(false);
                rate_limiter.record_failed_registration(client_ip).await;
                session.write().await.close();
                return Self::refus(
                    RegisterErrorCode::InternalError,
                    b"Invalid registration payload",
                );
            }
        };

        // Extraire le cog_id (forme stricte : empreinte base32 de 26 caractères)
        let cog_id = match register.cog_id_str() {
            Some(id) if cog_id_bien_forme(&id) => id,
            _ => {
                metrics.record_registration(false);
                rate_limiter.record_failed_registration(client_ip).await;
                session.write().await.close();
                return Self::refus(RegisterErrorCode::InvalidToken, b"Invalid cog_id");
            }
        };

        // O-01 : preuve d'identité obligatoire dans les deux modes (strict ou non).
        let (Some(cle), Some(signature)) = (register.cle_publique, register.signature) else {
            warn!("Unsigned REGISTER refused for cog_id {}", cog_id);
            metrics.record_registration(false);
            rate_limiter.record_failed_registration(client_ip).await;
            session.write().await.close();
            return Self::refus(RegisterErrorCode::AuthFailed, b"Signed identity proof required");
        };
        if let Err(e) = verifier_register(
            &cle,
            &signature,
            &defi,
            &cog_id,
            &ctx.identite_relay,
            &register.corps_signe,
        ) {
            warn!("REGISTER identity proof refused for {}: {}", cog_id, e);
            metrics.record_registration(false);
            rate_limiter.record_failed_registration(client_ip).await;
            session.write().await.close();
            return Self::refus(RegisterErrorCode::AuthFailed, b"Invalid identity proof");
        }

        // TOFU : refuser une clé différente d'une clé déjà épinglée. L'épinglage lui-même
        // n'a lieu qu'à la délivrance du permis (identité entièrement vérifiée).
        if let Err(e) = ctx.identites.verifier_compatible(&cog_id, &cle).await {
            warn!("REGISTER refused for {}: {}", cog_id, e);
            metrics.record_registration(false);
            rate_limiter.record_failed_registration(client_ip).await;
            session.write().await.close();
            return Self::refus(RegisterErrorCode::AuthFailed, b"Identity key mismatch");
        }

        info!("REGISTER with valid identity proof from cog_id: {}", cog_id);

        // Vérifier le timestamp (défense en profondeur ; le défi assure la fraîcheur)
        let now = chrono::Utc::now().timestamp().max(0).cast_unsigned();
        let window = ctx.config.policies.timestamp_window_seconds;
        if register.timestamp < now.saturating_sub(window)
            || register.timestamp > now.saturating_add(window)
        {
            metrics.record_registration(false);
            rate_limiter.record_failed_registration(client_ip).await;
            session.write().await.close();
            return Self::refus(RegisterErrorCode::AuthFailed, b"Timestamp out of window");
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
                        .filter(|p| cog_id_bien_forme(p))
                        .map(String::from)
                },
            );
            s.set_environment_health(register.environment_health.clone());
            s.identite_verifiee = true;
            s.cle_identite = Some(cle);
            s.start_phase_a();
            s.id
        };
        // L'identité est prouvée : la nouvelle session remplace une éventuelle session
        // précédente du même COG (plus de blocage « CogIdConflict » par un usurpateur).
        if ctx.sessions.is_cog_registered(&cog_id).await {
            info!("COG {} re-registered: new authenticated session replaces the old one", cog_id);
        }
        ctx.sessions.register_cog(cog_id.clone(), session_id).await;

        // Mode strict : demander CORE_KEY (Phase A) puis Phase C, pas de REGISTER_OK ici
        if ctx.config.relay.strict_verification {
            let response = VerifyResultPayload::new(
                session_id,
                VerifyPhase::PhaseA,
                VerifyResult::ExtendedRequired,
                Bytes::from_static(b"Send CORE_KEY"),
            );
            return Some(Frame::new(MessageType::VerifyResult, response.to_bytes()));
        }

        // Mode non strict (explicite, journalisé au démarrage) : pas de Phase A/C,
        // mais identité prouvée ci-dessus.
        {
            let mut s = session.write().await;
            s.phase_a_passed = true;
            s.phase_c_passed = true;
        }
        metrics.record_phase_a(true);
        metrics.record_phase_c(true);

        Some(Self::delivrer_permis(session, ctx, &cog_id, client_ip).await)
    }

    /// Délivre un permis lié au `cog_id` prouvé et construit `REGISTER_OK`.
    ///
    /// O-01 : c'est ici, et seulement ici, que la clé d'identité est épinglée — après la
    /// preuve REGISTER et les phases du mode strict. Un refus d'épinglage (clé concurrente,
    /// registre plein, persistance impossible) refuse l'enregistrement (fail-closed).
    async fn delivrer_permis(
        session: &Arc<RwLock<Session>>,
        ctx: &ContexteRelay,
        cog_id: &str,
        client_ip: std::net::IpAddr,
    ) -> Frame {
        let (session_id, permis_id, cle) = {
            let s = session.read().await;
            (s.id, s.generate_permis_id(), s.cle_identite)
        };
        let epinglage = match cle {
            Some(cle) if session.read().await.identite_verifiee => {
                ctx.identites.epingler(cog_id, &cle).await
            }
            _ => Err(RefusEpinglage::CleDifferente),
        };
        if let Err(e) = epinglage {
            warn!("Permit refused for {}: {}", cog_id, e);
            ctx.metrics.record_registration(false);
            ctx.rate_limiter.record_failed_registration(client_ip).await;
            session.write().await.close();
            let err = RegisterErrPayload::new(
                RegisterErrorCode::AuthFailed,
                Bytes::from_static(b"Identity pinning refused"),
            );
            return Frame::new(MessageType::RegisterErr, err.to_bytes());
        }
        let permis_expires = chrono::Utc::now() + chrono::Duration::hours(24);

        let mut session_key = [0u8; 32];
        rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut session_key);

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

        ctx.metrics.record_registration(true);
        ctx.metrics.record_permit_issued();
        ctx.rate_limiter.reset_failed_registrations(client_ip).await;

        info!("COG {} registered, Permis: {}", cog_id, permis_id);

        if let Some(registry) = &ctx.permis_registry {
            registry
                .register(permis_id.clone(), cog_id.to_string(), permis_expires)
                .await;
        }

        let response = RegisterOkPayload::build(
            session_id,
            Bytes::from(permis_id),
            permis_expires.timestamp().max(0).cast_unsigned(),
            Bytes::from_static(b"{}"),
            ctx.tracker_addresses.clone(),
            ctx.tracker_signature,
        );
        Frame::new(MessageType::RegisterOk, response.to_bytes())
    }

    /// Traite un message `CORE_KEY`.
    ///
    /// Accepté uniquement après un REGISTER dont l'identité a été prouvée, en Phase A.
    async fn handle_core_key(
        payload: &Bytes,
        session: &Arc<RwLock<Session>>,
        ctx: &ContexteRelay,
        client_ip: std::net::IpAddr,
    ) -> Option<Frame> {
        let metrics = &ctx.metrics;
        let core_key = match CoreKeyPayload::parse(payload.clone()) {
            Ok(k) => k,
            Err(e) => {
                warn!("Failed to parse CORE_KEY: {}", e);
                metrics.record_parse_error();
                return None;
            }
        };

        let (core_version, cog_id, session_id) = {
            let s = session.read().await;
            if !s.identite_verifiee || s.state != SessionState::VerifyingPhaseA {
                warn!("CORE_KEY refused: no proven identity or wrong phase");
                metrics.record_protocol_error();
                return Self::refus(RegisterErrorCode::AuthFailed, b"REGISTER proof required first");
            }
            (
                s.core_version.clone().unwrap_or_default(),
                s.cog_id.clone().unwrap_or_default(),
                s.id,
            )
        };
        if core_key.session_id != session_id {
            metrics.record_protocol_error();
            return Self::refus(RegisterErrorCode::AuthFailed, b"Session mismatch");
        }

        let result = ctx.verifier.verify_phase_a(&core_version, &core_key.key).await;
        let passed_a = result == VerifyResult::Ok;
        metrics.record_phase_a(passed_a);

        if !passed_a {
            ctx.rate_limiter.record_failed_registration(client_ip).await;
            let response =
                VerifyResultPayload::new(session_id, VerifyPhase::PhaseA, result, Bytes::new());
            return Some(Frame::new(MessageType::VerifyResult, response.to_bytes()));
        }

        session.write().await.complete_phase_a();

        // Après Phase A OK : Phase C puis REGISTER_OK
        let environment_health = {
            let s = session.read().await;
            s.environment_health.clone().unwrap_or_else(Bytes::new)
        };
        let phase_c_result = ctx.verifier.verify_phase_c(&environment_health).await;
        let phase_c_ok = phase_c_result == VerifyResult::Ok;
        metrics.record_phase_c(phase_c_ok);

        if !phase_c_ok {
            let mut s = session.write().await;
            s.state = SessionState::VerifyingPhaseC;
            let response = VerifyResultPayload::new(
                session_id,
                VerifyPhase::PhaseC,
                VerifyResult::Fail,
                Bytes::from_static(b"Environment health check failed"),
            );
            return Some(Frame::new(MessageType::VerifyResult, response.to_bytes()));
        }

        Some(Self::delivrer_permis(session, ctx, &cog_id, client_ip).await)
    }

    /// Traite un message `SERVICE_BLOCK` (identité prouvée requise).
    async fn handle_service_block(
        payload: &Bytes,
        session: &Arc<RwLock<Session>>,
        ctx: &ContexteRelay,
    ) -> Option<Frame> {
        let metrics = &ctx.metrics;
        if !session.read().await.identite_verifiee {
            metrics.record_protocol_error();
            return None;
        }
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

        let result = ctx
            .verifier
            .verify_phase_b(&service_id, block.block_index, &block.encrypted_block)
            .await;

        let passed = result == VerifyResult::Ok;
        metrics.record_phase_b(passed);

        let response =
            VerifyResultPayload::new(block.session_id, VerifyPhase::PhaseB, result, Bytes::new())
                .with_service_id(block.service_id);

        if passed {
            let mut s = session.write().await;
            s.add_verified_service(service_id);
        }

        Some(Frame::new(MessageType::VerifyResult, response.to_bytes()))
    }

    /// Traite un message HEARTBEAT.
    async fn handle_heartbeat(payload: &Bytes, session: &Arc<RwLock<Session>>) -> Option<Frame> {
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
        ctx: &ContexteRelay,
    ) -> Option<Frame> {
        // Vérifier que la session est active
        let (session_id, sequence) = {
            let mut s = session.write().await;
            if !s.is_active() || !s.identite_verifiee {
                warn!("DATA received on inactive session");
                return None;
            }
            s.touch();
            s.messages_received += 1;
            let seq = s.sequence_number;
            s.sequence_number = s.sequence_number.wrapping_add(1);
            (s.id, seq)
        };

        // Phase 2 : réponse à une requête HTTP injectée par Origin (id + session vérifiés)
        match ctx.http_injector.complete(session_id, payload).await {
            RemiseInjection::Livree => {
                debug!(
                    "HTTP response received via tunnel for session {}",
                    hex::encode(&session_id[..8])
                );
                return None;
            }
            RemiseInjection::Rejetee => {
                warn!(
                    "Tunnel response with unknown request id dropped (session {})",
                    hex::encode(&session_id[..8])
                );
                return None;
            }
            RemiseInjection::NonInjection => {}
        }

        // Relayer via le tunnel COG-COG
        if let Err(e) = ctx
            .tunnel_manager
            .relay_data(session_id, payload.clone(), sequence)
            .await
        {
            debug!("Failed to relay data: {}", e);
        } else {
            ctx.metrics.record_data_relayed();
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
    use miyuwebway_participant::identite::{message_register, IdentiteCog};
    use miyuwebway_participant::protocol::RegisterPayload as RegisterClient;
    use sha2::{Digest, Sha256};

    fn serveur(strict: bool) -> (RelayServer, Arc<PermisRegistry>, Arc<RegistreIdentites>) {
        let mut config = OriginConfig::default();
        config.relay.strict_verification = strict;
        let permis = Arc::new(PermisRegistry::new());
        let identites = Arc::new(RegistreIdentites::new());
        let serveur = RelayServer::sans_tls(
            Arc::new(config),
            Some(Arc::clone(&permis)),
            Arc::clone(&identites),
        )
        .expect("serveur");
        (serveur, permis, identites)
    }

    fn adresse() -> SocketAddr {
        "203.0.113.7:4444".parse().expect("adresse")
    }

    fn register_client(cog_id: &str) -> RegisterClient {
        RegisterClient {
            cog_id: cog_id.to_string(),
            core_version: "1.0.0".to_string(),
            cog_type: 3,
            os_type: 2,
            passport_type: 0,
            services: vec![],
            nonce: [1u8; 16],
            timestamp: chrono::Utc::now().timestamp().cast_unsigned(),
        }
    }

    fn code_refus(frame: &Frame) -> Option<RegisterErrorCode> {
        if frame.header.message_type != MessageType::RegisterErr {
            return None;
        }
        RegisterErrorCode::from_byte(frame.payload[0])
    }

    async fn enregistrer(
        serveur: &RelayServer,
        identite: &IdentiteCog,
    ) -> (Arc<RwLock<Session>>, Option<Frame>) {
        let session = serveur.ctx.sessions.create(adresse()).await;
        let defi = session.read().await.defi;
        let payload = register_client(&identite.cog_id()).to_origin_bytes_signe(
            identite,
            &defi,
            &serveur.ctx.identite_relay,
        );
        let reponse =
            RelayServer::handle_register(&payload, &session, &serveur.ctx, adresse().ip()).await;
        (session, reponse)
    }

    #[tokio::test]
    async fn strict_par_defaut() {
        assert!(OriginConfig::default().relay.strict_verification);
    }

    #[tokio::test]
    async fn register_non_signe_refuse() {
        for strict in [true, false] {
            let (serveur, _, _) = serveur(strict);
            let identite = IdentiteCog::generer();
            let session = serveur.ctx.sessions.create(adresse()).await;
            let payload = register_client(&identite.cog_id()).to_origin_bytes();
            let r = RelayServer::handle_register(&payload, &session, &serveur.ctx, adresse().ip())
                .await
                .expect("réponse");
            assert_eq!(code_refus(&r), Some(RegisterErrorCode::AuthFailed));
            assert_eq!(session.read().await.state, SessionState::Closed);
            assert!(!serveur.ctx.sessions.is_cog_registered(&identite.cog_id()).await);
        }
    }

    #[tokio::test]
    async fn register_signature_falsifiee_refuse() {
        let (serveur, _, _) = serveur(true);
        let identite = IdentiteCog::generer();
        let session = serveur.ctx.sessions.create(adresse()).await;
        // Signé sur un autre défi (rejeu d'une preuve ancienne).
        let payload = register_client(&identite.cog_id()).to_origin_bytes_signe(
            &identite,
            &[0xAA; 32],
            &serveur.ctx.identite_relay,
        );
        let r = RelayServer::handle_register(&payload, &session, &serveur.ctx, adresse().ip())
            .await
            .expect("réponse");
        assert_eq!(code_refus(&r), Some(RegisterErrorCode::AuthFailed));

        // Signé pour un autre Relay.
        let session = serveur.ctx.sessions.create(adresse()).await;
        let defi = session.read().await.defi;
        let payload = register_client(&identite.cog_id()).to_origin_bytes_signe(
            &identite,
            &defi,
            "relay.pirate.example",
        );
        let r = RelayServer::handle_register(&payload, &session, &serveur.ctx, adresse().ip())
            .await
            .expect("réponse");
        assert_eq!(code_refus(&r), Some(RegisterErrorCode::AuthFailed));
    }

    #[tokio::test]
    async fn register_cog_id_usurpe_refuse() {
        let (serveur, _, _) = serveur(true);
        let attaquant = IdentiteCog::generer();
        let victime = IdentiteCog::generer();
        let session = serveur.ctx.sessions.create(adresse()).await;
        let defi = session.read().await.defi;
        // REGISTER portant le cog_id de la victime, signé par la clé de l'attaquant.
        let mut corps = BytesMut::from(&register_client(&victime.cog_id()).to_origin_bytes()[..]);
        corps.extend_from_slice(&attaquant.cle_publique());
        let message = message_register(&defi, &victime.cog_id(), &serveur.ctx.identite_relay, &corps);
        let _ = Sha256::digest(&message);
        let signature = attaquant.signer_register(&defi, &serveur.ctx.identite_relay, &corps);
        corps.extend_from_slice(&signature);
        let r = RelayServer::handle_register(&corps.freeze(), &session, &serveur.ctx, adresse().ip())
            .await
            .expect("réponse");
        assert_eq!(code_refus(&r), Some(RegisterErrorCode::AuthFailed));
        assert!(!serveur.ctx.sessions.is_cog_registered(&victime.cog_id()).await);
    }

    #[tokio::test]
    async fn cle_differente_pour_cog_id_epingle_refusee() {
        let (serveur, _, identites) = serveur(true);
        let identite = IdentiteCog::generer();
        // Une autre clé est déjà épinglée pour ce cog_id.
        identites
            .epingler(&identite.cog_id(), &[0x11; 32])
            .await
            .expect("épinglage");
        let (_, r) = enregistrer(&serveur, &identite).await;
        assert_eq!(
            code_refus(&r.expect("réponse")),
            Some(RegisterErrorCode::AuthFailed)
        );
    }

    #[tokio::test]
    async fn defi_a_usage_unique() {
        let (serveur, _, _) = serveur(true);
        let identite = IdentiteCog::generer();
        let (session, r) = enregistrer(&serveur, &identite).await;
        assert_eq!(
            r.expect("réponse").header.message_type,
            MessageType::VerifyResult
        );
        let defi = session.read().await.defi;
        let payload = register_client(&identite.cog_id()).to_origin_bytes_signe(
            &identite,
            &defi,
            &serveur.ctx.identite_relay,
        );
        let r = RelayServer::handle_register(&payload, &session, &serveur.ctx, adresse().ip())
            .await
            .expect("réponse");
        assert_eq!(code_refus(&r), Some(RegisterErrorCode::AuthFailed));
    }

    #[tokio::test]
    async fn flux_strict_complet_et_permis_lie() {
        let (serveur, permis, identites) = serveur(true);
        serveur
            .ctx
            .verifier
            .add_core_key("1.0.0".to_string(), Sha256::digest(b"cle-cores").to_vec())
            .await;
        let identite = IdentiteCog::generer();

        // CORE_KEY avant REGISTER : refusé.
        let session_anonyme = serveur.ctx.sessions.create(adresse()).await;
        let id_anonyme = session_anonyme.read().await.id;
        let mut ck = BytesMut::new();
        ck.extend_from_slice(&id_anonyme);
        ck.extend_from_slice(&(9u16).to_be_bytes());
        ck.extend_from_slice(b"cle-cores");
        let r = RelayServer::handle_core_key(&ck.freeze(), &session_anonyme, &serveur.ctx, adresse().ip())
            .await
            .expect("réponse");
        assert_eq!(code_refus(&r), Some(RegisterErrorCode::AuthFailed));

        // REGISTER signé → VERIFY_RESULT (Phase A demandée).
        let (session, r) = enregistrer(&serveur, &identite).await;
        assert_eq!(r.expect("réponse").header.message_type, MessageType::VerifyResult);
        // Pas encore épinglé : les phases du mode strict ne sont pas passées.
        assert!(identites.cle_epinglee(&identite.cog_id()).await.is_none());

        // CORE_KEY → REGISTER_OK, permis lié au cog_id prouvé.
        let id = session.read().await.id;
        let mut ck = BytesMut::new();
        ck.extend_from_slice(&id);
        ck.extend_from_slice(&(9u16).to_be_bytes());
        ck.extend_from_slice(b"cle-cores");
        let r = RelayServer::handle_core_key(&ck.freeze(), &session, &serveur.ctx, adresse().ip())
            .await
            .expect("réponse");
        assert_eq!(r.header.message_type, MessageType::RegisterOk);
        let permis_id = session.read().await.permis.clone().expect("permis").permis_id;
        assert!(permis.is_valid_for(&permis_id, &identite.cog_id()).await);
        assert!(!permis.is_valid_for(&permis_id, &IdentiteCog::generer().cog_id()).await);
        // Épinglé seulement après la délivrance du permis.
        assert_eq!(
            identites.cle_epinglee(&identite.cog_id()).await,
            Some(identite.cle_publique())
        );
    }

    #[tokio::test]
    async fn register_echoue_n_epingle_rien() {
        let (serveur, _, identites) = serveur(true);
        serveur
            .ctx
            .verifier
            .add_core_key("1.0.0".to_string(), Sha256::digest(b"cle-cores").to_vec())
            .await;

        // Preuve invalide (mauvais défi) : aucun épinglage.
        let identite = IdentiteCog::generer();
        let session = serveur.ctx.sessions.create(adresse()).await;
        let payload = register_client(&identite.cog_id()).to_origin_bytes_signe(
            &identite,
            &[0xAA; 32],
            &serveur.ctx.identite_relay,
        );
        let r = RelayServer::handle_register(&payload, &session, &serveur.ctx, adresse().ip())
            .await
            .expect("réponse");
        assert_eq!(code_refus(&r), Some(RegisterErrorCode::AuthFailed));
        assert!(identites.cle_epinglee(&identite.cog_id()).await.is_none());

        // Preuve valide mais Phase A échouée (mauvaise clé Cores) : aucun épinglage.
        let identite = IdentiteCog::generer();
        let (session, r) = enregistrer(&serveur, &identite).await;
        assert_eq!(r.expect("réponse").header.message_type, MessageType::VerifyResult);
        let id = session.read().await.id;
        let mut ck = BytesMut::new();
        ck.extend_from_slice(&id);
        ck.extend_from_slice(&(9u16).to_be_bytes());
        ck.extend_from_slice(b"mauvaise!");
        let r = RelayServer::handle_core_key(&ck.freeze(), &session, &serveur.ctx, adresse().ip())
            .await
            .expect("réponse");
        assert_eq!(r.header.message_type, MessageType::VerifyResult);
        assert!(identites.cle_epinglee(&identite.cog_id()).await.is_none());

        // Aucune place du registre consommée.
        assert_eq!(identites.nombre().await, 0);
    }

    #[tokio::test]
    async fn permis_refuse_si_cle_epinglee_entre_temps() {
        // Course : une autre clé est épinglée entre REGISTER et la délivrance du permis.
        let (serveur, permis, identites) = serveur(true);
        serveur
            .ctx
            .verifier
            .add_core_key("1.0.0".to_string(), Sha256::digest(b"cle-cores").to_vec())
            .await;
        let identite = IdentiteCog::generer();
        let (session, _) = enregistrer(&serveur, &identite).await;
        identites
            .epingler(&identite.cog_id(), &[0x22; 32])
            .await
            .expect("épinglage");
        let id = session.read().await.id;
        let mut ck = BytesMut::new();
        ck.extend_from_slice(&id);
        ck.extend_from_slice(&(9u16).to_be_bytes());
        ck.extend_from_slice(b"cle-cores");
        let r = RelayServer::handle_core_key(&ck.freeze(), &session, &serveur.ctx, adresse().ip())
            .await
            .expect("réponse");
        assert_eq!(code_refus(&r), Some(RegisterErrorCode::AuthFailed));
        assert_eq!(session.read().await.state, SessionState::Closed);
        assert!(session.read().await.permis.is_none());
        let _ = permis;
    }

    #[tokio::test]
    async fn plafond_injections_par_session_et_purge_a_la_fermeture() {
        let inj = HttpInjector::new();
        let session = [1u8; SESSION_ID_SIZE];
        let autre = [2u8; SESSION_ID_SIZE];
        let mut receveurs = Vec::new();
        for _ in 0..MAX_INJECTIONS_PAR_SESSION {
            receveurs.push(inj.register(session).await.expect("sous le plafond"));
        }
        assert_eq!(
            inj.register(session).await.err(),
            Some(RefusInjection::PlafondSession)
        );
        // Une autre session n'est pas affectée.
        let (_, _rx_autre) = inj.register(autre).await.expect("autre session");

        // Un abandon libère une place.
        let (id0, _) = receveurs.remove(0);
        inj.abandon(&id0).await;
        receveurs.push(inj.register(session).await.expect("place libérée"));

        // Fermeture de la session : toutes ses attentes sont purgées, les demandeurs
        // voient le canal fermé immédiatement.
        assert_eq!(inj.purger_session(&session).await, MAX_INJECTIONS_PAR_SESSION);
        for (_, rx) in receveurs {
            assert!(rx.await.is_err());
        }
        assert_eq!(inj.pending.read().await.par_id.len(), 1);
        assert!(!inj.pending.read().await.par_session.contains_key(&session));
        // Après purge, le plafond est de nouveau disponible.
        assert!(inj.register(session).await.is_ok());
    }

    #[tokio::test]
    async fn non_strict_exige_quand_meme_la_preuve() {
        let (serveur, permis, _) = serveur(false);
        let identite = IdentiteCog::generer();
        let (session, r) = enregistrer(&serveur, &identite).await;
        assert_eq!(r.expect("réponse").header.message_type, MessageType::RegisterOk);
        let permis_id = session.read().await.permis.clone().expect("permis").permis_id;
        assert!(permis.is_valid_for(&permis_id, &identite.cog_id()).await);
    }

    #[tokio::test]
    async fn injecteur_ne_croise_jamais_les_reponses() {
        let inj = HttpInjector::new();
        let session = [1u8; SESSION_ID_SIZE];
        let autre_session = [2u8; SESSION_ID_SIZE];
        let (id_a, rx_a) = inj.register(session).await.expect("a");
        let (id_b, rx_b) = inj.register(session).await.expect("b");
        assert_ne!(id_a, id_b);

        let reponse = |id: &[u8; TAILLE_ID_REQUETE], corps: &[u8]| {
            let mut b = BytesMut::new();
            b.extend_from_slice(MAGIC_REPONSE_INJECTEE);
            b.extend_from_slice(id);
            b.extend_from_slice(corps);
            b.freeze()
        };

        // Id inconnu : jeté.
        assert_eq!(
            inj.complete(session, &reponse(&[9u8; 16], b"x")).await,
            RemiseInjection::Rejetee
        );
        // Bon id mais autre session : jeté, l'attente reste.
        assert_eq!(
            inj.complete(autre_session, &reponse(&id_a, b"vol")).await,
            RemiseInjection::Rejetee
        );
        // Réponses dans l'ordre inverse : chacune va à son demandeur.
        assert_eq!(
            inj.complete(session, &reponse(&id_b, b"pour-b")).await,
            RemiseInjection::Livree
        );
        assert_eq!(
            inj.complete(session, &reponse(&id_a, b"pour-a")).await,
            RemiseInjection::Livree
        );
        assert_eq!(rx_a.await.expect("a"), Bytes::from_static(b"pour-a"));
        assert_eq!(rx_b.await.expect("b"), Bytes::from_static(b"pour-b"));
        // Rejeu de la même réponse : jeté.
        assert_eq!(
            inj.complete(session, &reponse(&id_a, b"rejeu")).await,
            RemiseInjection::Rejetee
        );
        // Trame non encadrée : trafic de tunnel ordinaire.
        assert_eq!(
            inj.complete(session, &Bytes::from_static(b"HTTP/1.1 200 OK")).await,
            RemiseInjection::NonInjection
        );
        // Réponse tronquée : jetée.
        assert_eq!(
            inj.complete(session, &Bytes::from_static(b"MWHR123")).await,
            RemiseInjection::Rejetee
        );
    }

    #[tokio::test]
    async fn injections_concurrentes_bout_en_bout() {
        const N: usize = 8;
        let (serveur, _, _) = serveur(false);
        let serveur = Arc::new(serveur);
        let identite = IdentiteCog::generer();
        let (session, r) = enregistrer(&serveur, &identite).await;
        assert_eq!(r.expect("réponse").header.message_type, MessageType::RegisterOk);
        let session_id = session.read().await.id;
        let (tx, mut rx) = mpsc::channel::<TunnelMessage>(16);
        serveur.ctx.tunnel_manager.register_outbound(session_id, tx).await;

        // COG simulé : reçoit N requêtes puis répond dans l'ordre inverse, en renvoyant le chemin.
        let ctx = Arc::clone(&serveur.ctx);
        let session_cog = Arc::clone(&session);
        let cog = tokio::spawn(async move {
            let mut recues = Vec::new();
            while recues.len() < N {
                let msg = rx.recv().await.expect("requête");
                let reste = msg.data.strip_prefix(MAGIC_REQUETE_INJECTEE.as_slice()).expect("cadre");
                let (id, requete) = reste.split_at(TAILLE_ID_REQUETE);
                recues.push((id.to_vec(), requete.to_vec()));
            }
            for (id, requete) in recues.into_iter().rev() {
                let mut b = BytesMut::new();
                b.extend_from_slice(MAGIC_REPONSE_INJECTEE);
                b.extend_from_slice(&id);
                b.extend_from_slice(b"HTTP/1.1 200 OK\r\n\r\n");
                b.extend_from_slice(&requete);
                RelayServer::handle_data(&b.freeze(), &session_cog, &ctx).await;
            }
        });

        let mut taches = Vec::new();
        for i in 0..N {
            let serveur = Arc::clone(&serveur);
            let cog_id = identite.cog_id();
            taches.push(tokio::spawn(async move {
                let requete = format!("GET /visiteur-{i} HTTP/1.1\r\n\r\n");
                let reponse = serveur
                    .inject_http_request(&cog_id, Bytes::from(requete.clone()))
                    .await
                    .expect("réponse");
                (requete, reponse)
            }));
        }
        for t in taches {
            let (requete, reponse) = t.await.expect("tâche");
            assert!(reponse.ends_with(requete.as_bytes()), "réponse croisée");
        }
        cog.await.expect("cog");
        assert!(serveur.ctx.http_injector.pending.read().await.is_empty());
    }

    #[test]
    fn test_relay_metrics_created() {
        let metrics = RelayMetrics::new();
        assert_eq!(
            metrics
                .active_connections
                .load(std::sync::atomic::Ordering::Relaxed),
            0
        );
    }
}
