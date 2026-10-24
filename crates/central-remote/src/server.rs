//! Serveur HTTP + WebSocket embarqué pour CentralRemote.
//!
//! @id     central_remote_server
//! @do run_embedded_remote_server
//@human ! @role   infra
//! @layer toolkit
//! @human  Serveur Axum lancé par Central quand le remote est activé.
//! Expose : POST /auth (login), GET /ws (WebSocket), GET /status (santé).
//! Les routes d'administration de l'appairage exigent un pair loopback + secret.
//!
//! ## Chiffrement E2E mobile ↔ COG (R-023, fail-closed C-12)
//!
//! - `POST /api/bridge/e2e-handshake` : l'appareil envoie son offre v2 signée ;
//!   le COG retrouve la clé d'identité épinglée dans la whitelist (appareil
//!   inconnu = 403), vérifie l'offre (`PendingHandshake::complete`) et répond
//!   par sa propre offre v2.
//! - `POST /auth` chiffré : en-tête `x-miyukini-e2e-session` + corps
//!   `EncryptedMessage` v2 lié à `POST /auth` ; la réponse est chiffrée et liée
//!   au même identifiant de requête.
//! - `POST /auth` en clair (R-025) : **refusé partout par défaut** (E2E
//!   obligatoire). Avec [`RemoteServerConfig::allow_plaintext_local_auth`]
//!   (opt-in explicite, avertissement au démarrage), accepté depuis un pair
//!   loopback uniquement — utile au portail web local, mais un proxy inverse
//!   local ferait paraître « locale » toute requête : ne jamais l'activer
//!   derrière un proxy.
//!
//! ## Appairage v2 : engagement puis révélation (R-026 à R-028)
//!
//! - `POST /api/bridge/pair` : engagement de l'appareil → clé + aléa du COG.
//! - `POST /api/bridge/pair/reveal` : révélation de `Nd`, vérifiée.
//!   Engagement et révélation sont signés par la clé de l'appareil (preuve de
//!   possession) ; requête non signée = refus explicite, sans repli.
//! - `POST /api/bridge/pair/status` : suivi de la demande par l'appareil.
//! - Administration (loopback + secret) : `/api/bridge/devices` expose code
//!   SAS et empreinte de clé ; `/api/bridge/approve` exige le code affiché.
//! - Liste blanche persistée à côté de l'identité E2E (R-024).

use crate::bridge::RemoteBridge;
use crate::e2e::{load_or_create_cog_identity, E2eSessionStore, DEFAULT_IDENTITY_FILE};
use crate::paired_store::DEFAULT_PAIRED_DEVICES_FILE;
use crate::pairing::{
    mask_device_id, PairingConfig, PairingError, PairingManager, WhitelistValidateRequest,
    DEFAULT_API_TOKEN_TTL_SECS, DEFAULT_PENDING_TTL,
};
use crate::protocol::pairing::{
    check_pairing_version, PairCommitRequest, PairCommitResponse, PairRevealRequest,
    PairRevealResponse, PairStatusRequest, PairStatusResponse, PairingStatus,
    PAIRING_PROTOCOL_VERSION, PAIR_ENDPOINT, PAIR_REVEAL_ENDPOINT, PAIR_STATUS_ENDPOINT,
    STATUS_INVALID_SIGNATURE, STATUS_MISSING_SIGNATURE, STATUS_SOURCE_MISMATCH,
    STATUS_UNSUPPORTED_VERSION,
};
use crate::protocol::{RemoteAuthRequest, RemoteAuthResponse, RemoteCommand, RemoteEvent};
use crate::session::SessionManager;

use axum::body::Bytes;
use axum::extract::connect_info::ConnectInfo;
use axum::extract::DefaultBodyLimit;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, Request, State};
use axum::http::{header, HeaderMap, HeaderValue, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_util::{SinkExt, StreamExt};
use miyukini_central::auth::CentralAuthDb;
use miyukini_cog_bridge::e2e::MAX_HANDSHAKE_JSON_LEN;
use miyukini_cog_bridge::{
    identity_fingerprint, identity_fingerprint_hex, parse_commitment_hex, parse_identity_key_hex,
    parse_pairing_signature_hex, E2eHandshake, E2eIdentity, E2eRole, EncryptedMessage,
    PairingNonce, PendingHandshake, RequestBinding, E2E_SESSION_HEADER, PAIRING_SIGNATURE_SIZE,
};
use miyukini_connect::{AuthVerifyRequest, ConnectService, IdentitySetup, PermissionTier};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, VecDeque};
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

/// En-tête portant le secret d'administration (routes d'appairage desktop).
pub const ADMIN_SECRET_HEADER: &str = "x-central-admin-secret";

/// Fenêtre glissante de limitation de débit sur /auth.
const AUTH_RATE_WINDOW: Duration = Duration::from_secs(60);
/// Nombre maximal de tentatives /auth par IP dans la fenêtre.
const AUTH_RATE_MAX: usize = 10;
/// Fenêtre de limitation de débit des routes d'appairage / handshake E2E.
const BRIDGE_RATE_WINDOW: Duration = Duration::from_secs(60);
/// Nombre maximal d'appels appairage + handshake par IP dans la fenêtre.
const BRIDGE_RATE_MAX: usize = 30;
/// Nombre maximal d'appels de suivi d'appairage par IP et par minute
/// (l'appareil interroge toutes les 3 s pendant l'attente d'approbation).
const PAIR_STATUS_RATE_MAX: usize = 60;
/// Taille maximale du corps de /auth (clair ou enveloppe chiffrée, C-13).
pub const AUTH_MAX_BODY: usize = 16 * 1024;
/// Taille maximale du corps de /api/bridge/pair (C-13).
const PAIR_MAX_BODY: usize = 4 * 1024;
/// Point d'accès d'authentification (lié dans l'AAD des messages E2E).
pub const AUTH_ENDPOINT: &str = "/auth";
/// Intervalle de revérification du token pendant une session WS.
const WS_RECHECK_INTERVAL: Duration = Duration::from_secs(5);

/// Configuration du serveur CentralRemote.
#[derive(Debug, Clone)]
pub struct RemoteServerConfig {
    /// Adresse d'écoute (défaut : 127.0.0.1:8091).
    /// Le bind LAN (ex. `0.0.0.0:8091`) doit être demandé explicitement.
    pub listen_addr: String,
    /// Chemin vers la base auth Central.
    pub auth_db_path: String,
    /// Origines CORS autorisées (vide par défaut = aucun CORS cross-origin).
    /// Le client mobile natif n'en a pas besoin ; utile seulement pour un
    /// client navigateur (ex. portail web) explicitement déclaré.
    pub cors_allowed_origins: Vec<String>,
    /// Durée de validité des tokens API générés (secondes ; défaut 30 jours).
    pub api_token_ttl_secs: u64,
    /// Fichier de l'identité E2E longue durée du COG (`0600`). Défaut :
    /// `central-remote-e2e-identity.key` dans le dossier de la base auth.
    pub e2e_identity_path: Option<String>,
    /// Fichier de la liste blanche des appareils appairés (`0600`). Défaut :
    /// `central-remote-paired-devices.json` dans le dossier de l'identité E2E.
    pub paired_devices_path: Option<String>,
    /// Durée de vie d'une demande d'appairage en attente (secondes ; 0 = 10 min).
    pub pairing_pending_ttl_secs: u64,
    /// Révélation et relance d'appairage acceptées seulement depuis l'adresse
    /// source de l'engagement (**vrai par défaut**). Derrière un NAT les
    /// appareils partagent une adresse (liaison sans effet, signatures
    /// suffisantes) ; ne désactiver que si l'adresse source change entre deux
    /// requêtes d'un même appareil.
    pub pairing_require_same_source: bool,
    /// `/auth` en clair accepté depuis un pair loopback (R-025). **Faux par
    /// défaut** : E2E obligatoire partout. À n'activer que pour le portail web
    /// local, jamais derrière un proxy inverse local (toute requête y paraît
    /// locale). Un avertissement est journalisé au démarrage s'il est actif.
    pub allow_plaintext_local_auth: bool,
}

/// Vrai si un `/auth` en clair est accepté depuis `ip` (R-025).
fn plaintext_auth_allowed(allow_plaintext_local_auth: bool, ip: IpAddr) -> bool {
    allow_plaintext_local_auth && ip.to_canonical().is_loopback()
}

impl Default for RemoteServerConfig {
    fn default() -> Self {
        Self {
            // Loopback par défaut (fail-closed) ; le LAN nécessite une config explicite.
            listen_addr: "127.0.0.1:8091".into(),
            auth_db_path: "central.db".into(),
            cors_allowed_origins: Vec::new(),
            api_token_ttl_secs: DEFAULT_API_TOKEN_TTL_SECS,
            e2e_identity_path: None,
            paired_devices_path: None,
            pairing_pending_ttl_secs: DEFAULT_PENDING_TTL.as_secs(),
            pairing_require_same_source: true,
            allow_plaintext_local_auth: false,
        }
    }
}

/// Limiteur de débit à fenêtre glissante en mémoire, par IP.
struct RateLimiter {
    window: Duration,
    max: usize,
    hits: Mutex<HashMap<IpAddr, VecDeque<Instant>>>,
}

impl RateLimiter {
    fn new(window: Duration, max: usize) -> Self {
        Self { window, max, hits: Mutex::new(HashMap::new()) }
    }

    /// Enregistre une tentative ; false si la limite est dépassée (ou verrou empoisonné).
    fn check(&self, ip: IpAddr) -> bool {
        let Ok(mut hits) = self.hits.lock() else { return false };
        let now = Instant::now();
        // Purge opportuniste pour borner la mémoire.
        if hits.len() > 4096 {
            let window = self.window;
            hits.retain(|_, q| q.back().is_some_and(|t| now.duration_since(*t) < window));
        }
        let q = hits.entry(ip).or_default();
        while q.front().is_some_and(|t| now.duration_since(*t) >= self.window) {
            q.pop_front();
        }
        if q.len() >= self.max {
            return false;
        }
        q.push_back(now);
        true
    }
}

/// Comparaison en temps constant (via condensats SHA-256 : pas de fuite de longueur).
fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    let ha = Sha256::digest(a);
    let hb = Sha256::digest(b);
    let mut diff = 0u8;
    for (x, y) in ha.iter().zip(hb.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// État partagé du serveur remote.
#[derive(Clone)]
struct ServerState {
    bridge: RemoteBridge,
    sessions: Arc<SessionManager>,
    auth_db: Arc<CentralAuthDb>,
    connect: Arc<Mutex<ConnectService>>,
    pairing: Arc<PairingManager>,
    /// Secret d'administration (hex de 32 octets aléatoires), jamais exposé via HTTP.
    admin_secret: Arc<String>,
    auth_limiter: Arc<RateLimiter>,
    /// Limiteur des routes d'appairage et de handshake E2E.
    bridge_limiter: Arc<RateLimiter>,
    /// Limiteur du suivi d'appairage (`/api/bridge/pair/status`).
    status_limiter: Arc<RateLimiter>,
    /// Identité E2E longue durée du COG (secret effacé à la destruction).
    cog_identity: Arc<E2eIdentity>,
    /// Sessions E2E établies avec les appareils appairés.
    e2e_sessions: Arc<E2eSessionStore>,
    /// `/auth` en clair accepté depuis loopback (R-025, faux par défaut).
    allow_plaintext_local_auth: bool,
    /// Signal d'arrêt pour fermer les WebSocket ouverts.
    shutdown: tokio::sync::watch::Receiver<bool>,
}

fn generate_admin_secret() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Construit la couche CORS : aucune par défaut, liste blanche stricte sinon.
fn build_cors(origins: &[String]) -> Option<CorsLayer> {
    let parsed: Vec<HeaderValue> = origins
        .iter()
        .filter_map(|o| match HeaderValue::from_str(o.trim()) {
            Ok(v) if o.trim() != "*" => Some(v),
            _ => {
                warn!("CentralRemote: origine CORS ignorée: {o}");
                None
            }
        })
        .collect();
    if parsed.is_empty() {
        return None;
    }
    Some(
        CorsLayer::new()
            .allow_origin(parsed)
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]),
    )
}

/// Middleware des routes d'administration : pair loopback ET secret valide.
async fn require_local_admin(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Response {
    if !peer.ip().to_canonical().is_loopback() {
        warn!("CentralRemote admin refusé: pair non loopback {}", peer.ip());
        return StatusCode::FORBIDDEN.into_response();
    }
    let provided = req
        .headers()
        .get(ADMIN_SECRET_HEADER)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if provided.is_empty() || !ct_eq(provided.as_bytes(), state.admin_secret.as_bytes()) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    next.run(req).await
}

fn build_router(state: ServerState, cors: Option<CorsLayer>) -> Router {
    // Routes d'administration de l'appairage : desktop local uniquement.
    let admin = Router::new()
        .route("/api/bridge/generate-token", post(handle_bridge_generate_token))
        .route("/api/bridge/devices", get(handle_bridge_devices))
        .route("/api/bridge/approve", post(handle_bridge_approve))
        .route("/api/bridge/reject", post(handle_bridge_reject))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_local_admin));

    let app = Router::new()
        .route(
            "/auth",
            post(handle_auth).layer(DefaultBodyLimit::max(AUTH_MAX_BODY)),
        )
        .route("/ws", get(handle_ws_upgrade))
        .route("/status", get(handle_status))
        // Endpoints d'appairage mobile (COG Bridge)
        .route("/api/bridge/validate", post(handle_bridge_validate))
        .route("/api/bridge/validate-token", get(handle_bridge_validate_token))
        .route(
            PAIR_ENDPOINT,
            post(handle_bridge_pair).layer(DefaultBodyLimit::max(PAIR_MAX_BODY)),
        )
        .route(
            PAIR_REVEAL_ENDPOINT,
            post(handle_bridge_pair_reveal).layer(DefaultBodyLimit::max(PAIR_MAX_BODY)),
        )
        .route(
            PAIR_STATUS_ENDPOINT,
            post(handle_bridge_pair_status).layer(DefaultBodyLimit::max(PAIR_MAX_BODY)),
        )
        .route(
            "/api/bridge/e2e-handshake",
            post(handle_bridge_e2e_handshake)
                .layer(DefaultBodyLimit::max(MAX_HANDSHAKE_JSON_LEN)),
        )
        .route("/api/bridge/discovery", get(handle_bridge_discovery))
        .merge(admin)
        .with_state(state);

    match cors {
        Some(c) => app.layer(c),
        None => app,
    }
}

/// Lance le serveur CentralRemote. Retourne un handle pour l'arrêter.
/// Cette fonction est non-bloquante et spawn le serveur en background.
/// Le listener UDP de découverte LAN n'est lancé que si le serveur HTTP écoute
/// lui-même sur une interface non-loopback.
pub async fn start_remote_server(
    config: RemoteServerConfig,
    bridge: RemoteBridge,
) -> Result<RemoteServerHandle, String> {
    let auth_db = CentralAuthDb::open(Path::new(&config.auth_db_path))
        .map_err(|e| format!("CentralRemote auth DB: {e}"))?;

    let connect = ConnectService::new("2026.03.05");
    let sessions = SessionManager::with_random_secret();

    // Identité E2E longue durée du COG : chargée ou créée (0600), sinon refus
    // de démarrer (fail-closed).
    let identity_path = config.e2e_identity_path.as_ref().map_or_else(
        || {
            Path::new(&config.auth_db_path)
                .parent()
                .unwrap_or_else(|| Path::new(""))
                .join(DEFAULT_IDENTITY_FILE)
        },
        PathBuf::from,
    );
    let cog_identity = Arc::new(
        load_or_create_cog_identity(&identity_path).map_err(|e| format!("CentralRemote: {e}"))?,
    );
    let cog_identity_key_hex = hex::encode(cog_identity.public_key_bytes());
    let cog_identity_fingerprint = identity_fingerprint(&cog_identity.public_key_bytes());

    // Liste blanche persistée à côté de l'identité ; fichier corrompu = refus
    // de démarrer (fail-closed, R-024).
    let paired_path = config.paired_devices_path.as_ref().map_or_else(
        || {
            identity_path
                .parent()
                .unwrap_or_else(|| Path::new(""))
                .join(DEFAULT_PAIRED_DEVICES_FILE)
        },
        PathBuf::from,
    );
    let pairing_config = PairingConfig {
        pending_ttl: Duration::from_secs(config.pairing_pending_ttl_secs),
        require_same_source: config.pairing_require_same_source,
        ..PairingConfig::default()
    };
    let pairing = Arc::new(
        PairingManager::open(config.api_token_ttl_secs, pairing_config, paired_path.clone())
            .map_err(|e| {
                format!(
                    "CentralRemote: {e} ({}) — démarrage refusé ; corriger ou supprimer \
                     ce fichier (suppression = ré-appairage de tous les appareils)",
                    paired_path.display()
                )
            })?,
    );

    if config.allow_plaintext_local_auth {
        warn!(
            "CentralRemote: /auth en clair accepté depuis loopback \
             (allow_plaintext_local_auth) — ne jamais exposer derrière un proxy inverse local"
        );
    }

    let listener = TcpListener::bind(&config.listen_addr)
        .await
        .map_err(|e| format!("CentralRemote bind {}: {e}", config.listen_addr))?;

    let local_addr = listener
        .local_addr()
        .map_err(|e| format!("CentralRemote local_addr: {e}"))?;

    // Découverte UDP : seulement si le HTTP est lui-même exposé hors loopback.
    let discovery_handle = if crate::discovery::should_start_discovery(&local_addr) {
        match crate::discovery::start_discovery_listener(pairing.clone()).await {
            Ok(h) => {
                info!("Découverte LAN activée (UDP port 19847)");
                Some(h)
            }
            Err(e) => {
                warn!("Découverte LAN non activée: {e}");
                None
            }
        }
    } else {
        info!("Serveur en loopback : découverte LAN UDP désactivée");
        None
    };

    let admin_secret = Arc::new(generate_admin_secret());
    let (ws_shutdown_tx, ws_shutdown_rx) = tokio::sync::watch::channel(false);

    let pairing_for_handle = pairing.clone();
    let state = ServerState {
        bridge,
        sessions: Arc::new(sessions),
        auth_db: Arc::new(auth_db),
        connect: Arc::new(Mutex::new(connect)),
        pairing,
        admin_secret: admin_secret.clone(),
        auth_limiter: Arc::new(RateLimiter::new(AUTH_RATE_WINDOW, AUTH_RATE_MAX)),
        bridge_limiter: Arc::new(RateLimiter::new(BRIDGE_RATE_WINDOW, BRIDGE_RATE_MAX)),
        status_limiter: Arc::new(RateLimiter::new(BRIDGE_RATE_WINDOW, PAIR_STATUS_RATE_MAX)),
        cog_identity,
        e2e_sessions: Arc::new(E2eSessionStore::default()),
        allow_plaintext_local_auth: config.allow_plaintext_local_auth,
        shutdown: ws_shutdown_rx,
    };

    let app = build_router(state, build_cors(&config.cors_allowed_origins));

    info!("CentralRemote server listening on {local_addr}");

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    tokio::spawn(async move {
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .with_graceful_shutdown(async {
            let _ = shutdown_rx.await;
        })
        .await
        .ok();
        info!("CentralRemote server stopped");
    });

    Ok(RemoteServerHandle {
        addr: local_addr,
        shutdown_tx: Some(shutdown_tx),
        ws_shutdown_tx,
        pairing: pairing_for_handle,
        admin_secret,
        cog_identity_key_hex,
        cog_identity_fingerprint,
        discovery: discovery_handle,
    })
}

/// Handle pour contrôler le serveur remote.
/// Le serveur s'arrête quand le handle est `shutdown()` ou droppé.
pub struct RemoteServerHandle {
    /// Adresse effective d'écoute.
    pub addr: SocketAddr,
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
    ws_shutdown_tx: tokio::sync::watch::Sender<bool>,
    /// Gestionnaire de l'appairage (exposé pour l'UI Central Desktop).
    pub pairing: Arc<PairingManager>,
    admin_secret: Arc<String>,
    /// Clé publique d'identité E2E du COG (hex), publique.
    cog_identity_key_hex: String,
    /// Empreinte lisible de la clé d'identité du COG.
    cog_identity_fingerprint: String,
    /// Handle du listener UDP (gardé pour qu'il reste actif).
    discovery: Option<crate::discovery::DiscoveryHandle>,
}

impl RemoteServerHandle {
    /// Arrête le serveur proprement (ferme aussi les WebSocket ouverts et l'UDP).
    pub fn shutdown(&mut self) {
        let _ = self.ws_shutdown_tx.send(true);
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        if let Some(d) = self.discovery.as_mut() {
            d.shutdown();
        }
    }

    /// Adresse du serveur sous forme de string.
    pub fn addr_string(&self) -> String {
        self.addr.to_string()
    }

    /// Secret d'administration de l'appairage (à joindre en en-tête
    /// `x-central-admin-secret` pour les routes /api/bridge d'administration).
    /// Transmis à Central uniquement via cette API Rust, jamais via HTTP.
    pub fn admin_secret(&self) -> &str {
        &self.admin_secret
    }

    /// Clé publique d'identité E2E du COG (hex 32 octets), épinglée par les
    /// appareils à l'appairage (affichable sur le desktop).
    pub fn e2e_identity_public_key_hex(&self) -> &str {
        &self.cog_identity_key_hex
    }

    /// Empreinte lisible de la clé d'identité du COG (8 groupes de 4 hex),
    /// à afficher sur Central pour comparaison avec le téléphone.
    pub fn e2e_identity_fingerprint(&self) -> &str {
        &self.cog_identity_fingerprint
    }
}

impl Drop for RemoteServerHandle {
    fn drop(&mut self) {
        self.shutdown();
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Handlers
// ═══════════════════════════════════════════════════════════════════════════

fn auth_failure(msg: &str) -> Json<RemoteAuthResponse> {
    Json(auth_failure_response(msg))
}

/// POST /auth — authentification et obtention d'un token.
///
/// Avec l'en-tête [`E2E_SESSION_HEADER`] : corps `EncryptedMessage` v2 lié à
/// `POST /auth`, réponse chiffrée liée à la même requête. Sans : clair, accepté
/// seulement si `allow_plaintext_local_auth` est actif et le pair loopback.
async fn handle_auth(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if !state.auth_limiter.check(peer.ip().to_canonical()) {
        warn!("CentralRemote /auth: limite de débit dépassée pour {}", peer.ip());
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [(header::RETRY_AFTER, "60")],
            auth_failure("Trop de tentatives, réessayez plus tard"),
        )
            .into_response();
    }

    match headers.get(E2E_SESSION_HEADER) {
        Some(session_header) => handle_auth_e2e(&state, session_header, &body),
        None => handle_auth_clear(&state, peer, &body),
    }
}

/// /auth en clair : uniquement si l'option l'autorise et le pair est loopback.
fn handle_auth_clear(state: &ServerState, peer: SocketAddr, body: &[u8]) -> Response {
    if !plaintext_auth_allowed(state.allow_plaintext_local_auth, peer.ip()) {
        warn!("CentralRemote /auth en clair refusé (E2E requis) pour {}", peer.ip());
        return (
            StatusCode::FORBIDDEN,
            auth_failure("Chiffrement de bout en bout requis"),
        )
            .into_response();
    }
    let Ok(req) = serde_json::from_slice::<RemoteAuthRequest>(body) else {
        return (StatusCode::BAD_REQUEST, auth_failure("Requête invalide")).into_response();
    };
    Json(authenticate(state, &req)).into_response()
}

/// /auth chiffré : session E2E connue, appareil toujours appairé avec la même
/// clé, enveloppe v2 liée à `POST /auth` ; toute anomalie = refus (C-12).
fn handle_auth_e2e(
    state: &ServerState,
    session_header: &HeaderValue,
    body: &[u8],
) -> Response {
    let Some(entry) = session_header
        .to_str()
        .ok()
        .and_then(|h| state.e2e_sessions.get_hex(h))
    else {
        warn!("CentralRemote /auth E2E: session inconnue ou expirée");
        return StatusCode::UNAUTHORIZED.into_response();
    };
    if !state
        .pairing
        .is_paired_with_key(&entry.device_id, &entry.device_key_hex)
    {
        warn!(
            "CentralRemote /auth E2E: appareil {} révoqué",
            mask_device_id(&entry.device_id)
        );
        state.e2e_sessions.remove_device(&entry.device_id);
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let Ok(msg) = EncryptedMessage::from_json_bounded(body, AUTH_MAX_BODY) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(binding) = RequestBinding::for_received(&msg, "POST", AUTH_ENDPOINT) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let req: RemoteAuthRequest = match entry.session.decrypt_json(&binding, &msg) {
        Ok(r) => r,
        Err(e) => {
            warn!("CentralRemote /auth E2E refusé: {e}");
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };
    let resp = authenticate(state, &req);
    match entry.session.encrypt_json(&binding, &resp) {
        Ok(m) => Json(m).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

fn auth_failure_response(msg: &str) -> RemoteAuthResponse {
    RemoteAuthResponse {
        success: false,
        token: None,
        user_display_name: None,
        error: Some(msg.into()),
    }
}

/// Vérifie les identifiants (Miyukini Connect uniquement) et crée la session.
fn authenticate(state: &ServerState, req: &RemoteAuthRequest) -> RemoteAuthResponse {
    let email = req.email.trim();
    if email.is_empty() || req.password.is_empty() {
        return auth_failure_response("Email et mot de passe requis");
    }

    // Recherche du profil par email.
    let profiles = match state.auth_db.list_profiles() {
        Ok(p) => p,
        Err(e) => {
            warn!("CentralRemote auth list_profiles error: {e}");
            return auth_failure_response("Erreur interne");
        }
    };

    let profile = profiles
        .into_iter()
        .find(|p| p.email.eq_ignore_ascii_case(email));

    let Some(profile) = profile else {
        // Email inconnu : travail Argon2 factice pour égaliser le temps de réponse.
        let mut dummy = IdentitySetup::new("remote-dummy");
        let _ = dummy.set_password(&req.password);
        return auth_failure_response("Identifiants incorrects");
    };

    // Seul chemin : Miyukini Connect (compteur d'échecs + verrouillage).
    // Aucun repli legacy : échec, verrouillage ou hash absent => refus.
    if !verify_connect_auth(state, &profile.id, &req.password) {
        return auth_failure_response("Identifiants incorrects");
    }

    let display_name = profile
        .pseudonyme
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or(&profile.email)
        .to_string();

    let token = state.sessions.create_session(&profile.id, &display_name);

    RemoteAuthResponse {
        success: true,
        token: Some(token),
        user_display_name: Some(display_name),
        error: None,
    }
}

fn verify_connect_auth(state: &ServerState, profile_id: &str, password: &str) -> bool {
    let stored_hash = match state.auth_db.get_connect_password_hash(profile_id) {
        Ok(Some(h)) => h,
        Ok(None) => {
            // Pas de hash Connect : travail factice (timing), puis refus.
            let mut dummy = IdentitySetup::new("remote-dummy");
            let _ = dummy.set_password(password);
            return false;
        }
        Err(_) => return false,
    };

    let Ok(mut connect) = state.connect.lock() else {
        return false;
    };

    // N'écrase l'identité que si le hash a changé : les compteurs d'échecs /
    // verrouillages vivent dans le service et sont conservés dans tous les cas.
    if connect.password_hash_for(profile_id) != Some(stored_hash.as_str()) {
        let mut identity = IdentitySetup::new(profile_id.to_string());
        if identity.set_password_hash_for_import(stored_hash).is_err() {
            return false;
        }
        if connect.register_identity(identity).is_err() {
            return false;
        }
    }

    let runtime_state = connect.current_runtime_state();
    let request = AuthVerifyRequest {
        subject_id: profile_id.to_string(),
        password: password.to_string(),
        totp_code: None,
        requested_tier: PermissionTier::Basic,
        runtime_state,
    };

    // Toute erreur (InvalidCredentials, LockedOut, StepUp...) => refus.
    connect.auth_verify(request).is_ok()
}

/// Paramètres de connexion WebSocket.
#[derive(Deserialize)]
struct WsParams {
    /// Déprécié : token en query string (fuite dans logs/historique). Conservé
    /// uniquement pour les clients existants (portail web, mobile) ; préférer
    /// l'en-tête `Authorization: Bearer <token>`.
    token: Option<String>,
}

/// GET /ws — upgrade WebSocket. Token via `Authorization: Bearer` (préféré)
/// ou, par compatibilité, `?token=`.
async fn handle_ws_upgrade(
    State(state): State<ServerState>,
    Query(params): Query<WsParams>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> Response {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(str::to_string)
        .or(params.token)
        .unwrap_or_default();

    if let Some(session) = state.sessions.validate_token(&token) {
        info!("CentralRemote WS upgrade for {}", session.display_name);
        ws.on_upgrade(move |socket| handle_ws_connection(socket, state, token))
    } else {
        warn!("CentralRemote WS rejected: invalid token");
        // Refus avant l'upgrade.
        StatusCode::UNAUTHORIZED.into_response()
    }
}

/// Gère une connexion WebSocket authentifiée.
async fn handle_ws_connection(socket: WebSocket, state: ServerState, token: String) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let mut shutdown = state.shutdown.clone();
    let mut recheck = tokio::time::interval(WS_RECHECK_INTERVAL);

    // Envoyer le snapshot initial.
    let snapshot = state.bridge.current_snapshot();
    let initial = RemoteEvent::Snapshot(snapshot);
    if let Ok(json) = serde_json::to_string(&initial) {
        if ws_tx.send(Message::Text(json.into())).await.is_err() {
            return;
        }
    }

    // Souscrire aux événements du bridge.
    let mut event_rx = state.bridge.subscribe_events();

    // Boucle principale : forward events ↔ commands.
    loop {
        tokio::select! {
            // Arrêt du serveur
            _ = shutdown.changed() => {
                let _ = ws_tx.send(Message::Close(None)).await;
                break;
            }
            // Revérification périodique : expiration ou révocation du token
            _ = recheck.tick() => {
                if state.sessions.validate_token(&token).is_none() {
                    warn!("CentralRemote WS fermé : token expiré ou révoqué");
                    if let Ok(json) = serde_json::to_string(&RemoteEvent::Error {
                        message: "Token invalide ou expiré".into(),
                    }) {
                        let _ = ws_tx.send(Message::Text(json.into())).await;
                    }
                    let _ = ws_tx.send(Message::Close(None)).await;
                    break;
                }
            }
            // Événement Central → client WS
            Ok(event) = event_rx.recv() => {
                if let Ok(json) = serde_json::to_string(&event) {
                    if ws_tx.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
            }
            // Message client WS → commande Central
            msg = ws_rx.next() => {
                // Chaque commande revalide aussi le token (pas d'attente du tick).
                if state.sessions.validate_token(&token).is_none() {
                    let _ = ws_tx.send(Message::Close(None)).await;
                    break;
                }
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(cmd) = serde_json::from_str::<RemoteCommand>(&text) {
                            match &cmd {
                                RemoteCommand::Ping => {
                                    if let Ok(json) = serde_json::to_string(&RemoteEvent::Pong) {
                                        let _ = ws_tx.send(Message::Text(json.into())).await;
                                    }
                                }
                                RemoteCommand::RequestSnapshot => {
                                    let snapshot = state.bridge.current_snapshot();
                                    let evt = RemoteEvent::Snapshot(snapshot);
                                    if let Ok(json) = serde_json::to_string(&evt) {
                                        let _ = ws_tx.send(Message::Text(json.into())).await;
                                    }
                                }
                                _ => {
                                    state.bridge.send_command(cmd);
                                }
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }
}

/// GET /status — santé du serveur remote.
async fn handle_status(State(state): State<ServerState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "CentralRemote",
        "connected_clients": state.bridge.connected_clients(),
        "active_sessions": state.sessions.active_count(),
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// Handlers d'appairage mobile (COG Bridge)
// ═══════════════════════════════════════════════════════════════════════════

/// POST /api/bridge/validate — valide un device whitelisté (mode Whitelist LAN).
async fn handle_bridge_validate(
    State(state): State<ServerState>,
    Json(req): Json<WhitelistValidateRequest>,
) -> impl IntoResponse {
    if state.pairing.is_whitelisted(&req.device_id) {
        state.pairing.touch_device(&req.device_id);
        (StatusCode::OK, Json(serde_json::json!({"status": "ok"})))
    } else {
        (StatusCode::FORBIDDEN, Json(serde_json::json!({"status": "not_whitelisted"})))
    }
}

/// GET /api/bridge/validate-token — valide un token API (mode Clé API).
async fn handle_bridge_validate_token(
    State(state): State<ServerState>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("");

    if state.pairing.validate_api_token(token) {
        (StatusCode::OK, Json(serde_json::json!({"status": "ok"})))
    } else {
        (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"status": "invalid_token"})))
    }
}

/// Réponse JSON `{"status": <statut>}` avec un code HTTP.
fn status_reply(code: StatusCode, status: &str) -> Response {
    (code, Json(serde_json::json!({ "status": status }))).into_response()
}

/// Traduit une erreur d'appairage en réponse HTTP (une cause = un statut).
fn pairing_error_reply(e: &PairingError) -> Response {
    match e {
        PairingError::InvalidDeviceId => status_reply(StatusCode::BAD_REQUEST, "invalid_device_id"),
        PairingError::InvalidIdentityKey => {
            status_reply(StatusCode::BAD_REQUEST, "invalid_e2e_identity_key")
        }
        PairingError::TooManyPending => {
            status_reply(StatusCode::SERVICE_UNAVAILABLE, "too_many_pending")
        }
        PairingError::TooManyPendingForSource => {
            status_reply(StatusCode::TOO_MANY_REQUESTS, "too_many_pending_for_source")
        }
        PairingError::PendingConflict => status_reply(StatusCode::CONFLICT, "pending_conflict"),
        PairingError::KeyInUse => status_reply(StatusCode::CONFLICT, "key_in_use"),
        PairingError::AlreadyPaired => status_reply(StatusCode::CONFLICT, "already_paired"),
        PairingError::NoPending => status_reply(StatusCode::NOT_FOUND, "no_pending"),
        PairingError::AlreadyRevealed => status_reply(StatusCode::CONFLICT, "already_revealed"),
        PairingError::CommitmentMismatch => {
            status_reply(StatusCode::FORBIDDEN, "commitment_mismatch")
        }
        PairingError::NotRevealed => status_reply(StatusCode::CONFLICT, "not_revealed"),
        PairingError::CodeMismatch => status_reply(StatusCode::CONFLICT, "code_mismatch"),
        PairingError::InvalidSignature => {
            status_reply(StatusCode::UNAUTHORIZED, STATUS_INVALID_SIGNATURE)
        }
        PairingError::SourceMismatch => status_reply(StatusCode::FORBIDDEN, STATUS_SOURCE_MISMATCH),
        PairingError::Persistence(_) | PairingError::Unavailable => {
            status_reply(StatusCode::SERVICE_UNAVAILABLE, "unavailable")
        }
    }
}

/// Décode la signature d'appairage d'une requête. Absente (requête v2 non
/// signée, format antérieur) : refus explicite `missing_pairing_signature`,
/// sans repli ; mal formée : `invalid_pairing_signature`.
/// L'erreur porte le statut à renvoyer (HTTP 400).
fn pairing_signature_field(s: &str) -> Result<[u8; PAIRING_SIGNATURE_SIZE], &'static str> {
    if s.is_empty() {
        warn!("CentralRemote appairage refusé: requête non signée");
        return Err(STATUS_MISSING_SIGNATURE);
    }
    parse_pairing_signature_hex(s).map_err(|_| STATUS_INVALID_SIGNATURE)
}

/// Refus explicite d'une requête d'appairage d'une autre version (v1 comprise).
fn unsupported_version_reply(version: u8) -> Response {
    warn!("CentralRemote appairage refusé: version {version} (attendue {PAIRING_PROTOCOL_VERSION})");
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({
            "status": STATUS_UNSUPPORTED_VERSION,
            "expected_pairing_version": PAIRING_PROTOCOL_VERSION,
        })),
    )
        .into_response()
}

/// POST /api/bridge/pair — étape 1 de l'appairage v2 (engagement).
///
/// L'appareil envoie sa clé d'identité E2E et l'engagement `C` sur son aléa
/// `Nd` ; le COG enregistre la demande et répond avec sa clé d'identité et son
/// aléa `Nc`. Aucun code n'est transmis : chaque côté le calcule après la
/// révélation. L'approbation sur le desktop reste requise.
async fn handle_bridge_pair(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    body: Bytes,
) -> Response {
    let source = peer.ip().to_canonical();
    if !state.bridge_limiter.check(source) {
        return StatusCode::TOO_MANY_REQUESTS.into_response();
    }
    if let Err(v) = check_pairing_version(&body) {
        return unsupported_version_reply(v);
    }
    let Ok(req) = serde_json::from_slice::<PairCommitRequest>(&body) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_request");
    };
    let Ok(device_key) = parse_identity_key_hex(&req.e2e_identity_key) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_e2e_identity_key");
    };
    let Ok(commitment) = parse_commitment_hex(&req.commitment) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_commitment");
    };
    let signature = match pairing_signature_field(&req.commit_signature) {
        Ok(sig) => sig,
        Err(status) => return status_reply(StatusCode::BAD_REQUEST, status),
    };
    let cog_key = state.cog_identity.public_key_bytes();
    // La signature est vérifiée par le gestionnaire avant toute lecture d'état.
    match state.pairing.request_pairing(
        &req.device_id,
        &req.device_name,
        &hex::encode(device_key),
        commitment,
        &signature,
        source,
    ) {
        Ok(cog_nonce) => {
            info!(
                "CentralRemote: demande d'appairage v2 ({})",
                mask_device_id(&req.device_id)
            );
            Json(PairCommitResponse {
                pairing_version: PAIRING_PROTOCOL_VERSION,
                status: PairingStatus::Committed,
                cog_identity_key: hex::encode(cog_key),
                cog_nonce: cog_nonce.to_hex().to_string(),
                message: "Révélez votre aléa, puis comparez le code avec Central Desktop".into(),
            })
            .into_response()
        }
        Err(e) => {
            warn!(
                "CentralRemote appairage refusé ({}): {e}",
                mask_device_id(&req.device_id)
            );
            pairing_error_reply(&e)
        }
    }
}

/// POST /api/bridge/pair/reveal — étape 3 (révélation de `Nd`).
///
/// Refus : version ou signature absente (400), aucune demande — donc aucun
/// `Nc` émis (404), signature invalide (401) ou autre source (403) — demande
/// intacte —, déjà révélé (409), engagement incohérent valablement signé
/// (403, demande abandonnée).
async fn handle_bridge_pair_reveal(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    body: Bytes,
) -> Response {
    let source = peer.ip().to_canonical();
    if !state.bridge_limiter.check(source) {
        return StatusCode::TOO_MANY_REQUESTS.into_response();
    }
    if let Err(v) = check_pairing_version(&body) {
        return unsupported_version_reply(v);
    }
    let Ok(req) = serde_json::from_slice::<PairRevealRequest>(&body) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_request");
    };
    let Ok(device_nonce) = PairingNonce::from_hex(&req.device_nonce) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_device_nonce");
    };
    let signature = match pairing_signature_field(&req.reveal_signature) {
        Ok(sig) => sig,
        Err(status) => return status_reply(StatusCode::BAD_REQUEST, status),
    };
    let cog_key = state.cog_identity.public_key_bytes();
    match state
        .pairing
        .reveal_pairing(&req.device_id, &device_nonce, &signature, source, &cog_key)
    {
        Ok(_code) => Json(PairRevealResponse {
            status: PairingStatus::Revealed,
        })
        .into_response(),
        Err(e) => {
            warn!(
                "CentralRemote révélation refusée ({}): {e}",
                mask_device_id(&req.device_id)
            );
            pairing_error_reply(&e)
        }
    }
}

/// POST /api/bridge/pair/status — suivi d'une demande par l'appareil.
async fn handle_bridge_pair_status(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    body: Bytes,
) -> Response {
    if !state.status_limiter.check(peer.ip().to_canonical()) {
        return StatusCode::TOO_MANY_REQUESTS.into_response();
    }
    let Ok(req) = serde_json::from_slice::<PairStatusRequest>(&body) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_request");
    };
    let Ok(device_key) = parse_identity_key_hex(&req.e2e_identity_key) else {
        return status_reply(StatusCode::BAD_REQUEST, "invalid_e2e_identity_key");
    };
    let status = state
        .pairing
        .pairing_status(&req.device_id, &hex::encode(device_key));
    Json(PairStatusResponse { status }).into_response()
}

/// POST /api/bridge/e2e-handshake — handshake E2E v2 avec un appareil appairé.
///
/// Refus : offre mal formée ou version ≠ 2 (400), appareil inconnu (403),
/// offre non vérifiable — signature, rôle, clé éphémère (401).
async fn handle_bridge_e2e_handshake(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    body: Bytes,
) -> Response {
    if !state.bridge_limiter.check(peer.ip().to_canonical()) {
        return StatusCode::TOO_MANY_REQUESTS.into_response();
    }
    let offer = match E2eHandshake::from_json(&body) {
        Ok(o) => o,
        Err(e) => {
            warn!("CentralRemote handshake E2E refusé: {e}");
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "offre E2E invalide ou version non supportée"})),
            )
                .into_response();
        }
    };
    let Ok(device_key) = parse_identity_key_hex(&offer.identity_key) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let device_key_hex = hex::encode(device_key);
    let Some(device) = state.pairing.device_for_identity_key(&device_key_hex) else {
        warn!("CentralRemote handshake E2E refusé: appareil non appairé");
        return StatusCode::FORBIDDEN.into_response();
    };
    let (pending, cog_offer) =
        match PendingHandshake::initiate(E2eRole::Cog, &state.cog_identity, &device_key) {
            Ok(p) => p,
            Err(e) => {
                warn!("CentralRemote handshake E2E refusé: {e}");
                return StatusCode::FORBIDDEN.into_response();
            }
        };
    // Vérification complète de l'offre (signature avant dérivation, C-23).
    let session = match pending.complete(&offer) {
        Ok(s) => s,
        Err(e) => {
            warn!(
                "CentralRemote handshake E2E refusé ({}): {e}",
                mask_device_id(&device.device_id)
            );
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };
    if !state
        .e2e_sessions
        .insert(&device.device_id, &device_key_hex, session)
    {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    state.pairing.touch_device(&device.device_id);
    info!(
        "CentralRemote: session E2E v2 établie ({})",
        mask_device_id(&device.device_id)
    );
    Json(cog_offer).into_response()
}

/// POST /api/bridge/generate-token — génère un token API (appelé depuis le desktop).
async fn handle_bridge_generate_token(
    State(state): State<ServerState>,
    Json(req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let device_name = req
        .get("device_name")
        .and_then(|v| v.as_str())
        .unwrap_or("Mobile");
    let device_name: String = device_name.chars().take(64).collect();
    let token = state.pairing.generate_api_token(&device_name);
    Json(serde_json::json!({
        "token": token,
        "device_name": device_name,
    }))
}

/// GET /api/bridge/discovery — info de découverte LAN.
async fn handle_bridge_discovery(
    State(state): State<ServerState>,
) -> Json<serde_json::Value> {
    let info = state.pairing.discovery_info();
    Json(serde_json::to_value(info).unwrap_or_default())
}

/// GET /api/bridge/devices — liste les devices appairés et en attente
/// (admin local ; les `device_id` sont masqués). Chaque demande révélée porte
/// son code SAS et l'empreinte de la clé de l'appareil ; la réponse porte
/// aussi l'empreinte de la clé du COG.
async fn handle_bridge_devices(
    State(state): State<ServerState>,
) -> Json<serde_json::Value> {
    let paired: Vec<_> = state
        .pairing
        .paired_devices()
        .into_iter()
        .map(|d| {
            serde_json::json!({
                "device_id": mask_device_id(&d.device_id),
                "name": d.name,
                "paired_at": d.paired_at,
                "last_seen": d.last_seen,
                "mode": d.mode,
                "key_fingerprint": identity_fingerprint_hex(&d.e2e_identity_key),
            })
        })
        .collect();
    let pending: Vec<_> = state
        .pairing
        .pending_devices()
        .into_iter()
        .map(|d| {
            serde_json::json!({
                "device_id": mask_device_id(&d.device_id),
                "name": d.name,
                "requested_at": d.requested_at,
                "status": d.status,
                "confirmation_code": d.confirmation_code,
                "key_fingerprint": d.key_fingerprint,
                "expires_in_secs": d.expires_in_secs,
            })
        })
        .collect();
    Json(serde_json::json!({
        "cog_key_fingerprint": identity_fingerprint(&state.cog_identity.public_key_bytes()),
        "paired": paired,
        "pending": pending,
    }))
}

/// Requête de rejet.
#[derive(Deserialize)]
struct DeviceAction {
    device_id: String,
}

/// Requête d'approbation : le code SAS que l'administrateur a vu (R-027).
#[derive(Deserialize)]
struct ApproveAction {
    device_id: String,
    confirmation_code: String,
}

/// POST /api/bridge/approve — approuve une demande révélée (depuis le desktop)
/// si le code fourni est celui de la demande (temps constant).
async fn handle_bridge_approve(
    State(state): State<ServerState>,
    Json(req): Json<ApproveAction>,
) -> Response {
    match state
        .pairing
        .approve_device(&req.device_id, &req.confirmation_code)
    {
        Ok(()) => Json(serde_json::json!({"approved": true})).into_response(),
        Err(e) => {
            warn!(
                "CentralRemote approbation refusée ({}): {e}",
                mask_device_id(&req.device_id)
            );
            pairing_error_reply(&e)
        }
    }
}

/// POST /api/bridge/reject — refuse un device en attente (depuis le desktop).
async fn handle_bridge_reject(
    State(state): State<ServerState>,
    Json(req): Json<DeviceAction>,
) -> Json<serde_json::Value> {
    let ok = state.pairing.reject_device(&req.device_id);
    Json(serde_json::json!({"rejected": ok}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::extract::connect_info::MockConnectInfo;
    use tower::ServiceExt;

    const PASSWORD: &str = "Str0ng!Passw0rd-xyz";

    fn test_state() -> (ServerState, tokio::sync::watch::Sender<bool>, String) {
        let path = std::env::temp_dir().join(format!("central-remote-test-{}.db", uuid::Uuid::new_v4()));
        let db = CentralAuthDb::open(&path).expect("open db");
        let profile = db.sign_up("alice@example.org", PASSWORD, Some("Alice")).expect("sign_up");

        // Hash Connect (Argon2id) pour le profil.
        let mut ident = IdentitySetup::new(profile.id.clone());
        ident.set_password(PASSWORD).unwrap();
        let mut svc = ConnectService::new("test");
        svc.register_identity(ident).unwrap();
        let hash = svc.password_hash_for(&profile.id).unwrap().to_string();
        db.set_connect_password_hash(&profile.id, &hash).unwrap();

        // Profil legacy sans hash Connect.
        db.sign_up("legacy@example.org", PASSWORD, None).expect("sign_up legacy");

        let (tx, rx) = tokio::sync::watch::channel(false);
        let state = ServerState {
            bridge: RemoteBridge::new(),
            sessions: Arc::new(SessionManager::with_random_secret()),
            auth_db: Arc::new(db),
            connect: Arc::new(Mutex::new(ConnectService::new("test"))),
            pairing: Arc::new(PairingManager::new()),
            admin_secret: Arc::new(generate_admin_secret()),
            auth_limiter: Arc::new(RateLimiter::new(AUTH_RATE_WINDOW, AUTH_RATE_MAX)),
            bridge_limiter: Arc::new(RateLimiter::new(BRIDGE_RATE_WINDOW, BRIDGE_RATE_MAX)),
            status_limiter: Arc::new(RateLimiter::new(BRIDGE_RATE_WINDOW, PAIR_STATUS_RATE_MAX)),
            cog_identity: Arc::new(E2eIdentity::generate()),
            e2e_sessions: Arc::new(E2eSessionStore::default()),
            allow_plaintext_local_auth: false,
            shutdown: rx,
        };
        (state, tx, profile.id)
    }

    /// État de test avec `/auth` en clair accepté depuis loopback (tests de la
    /// logique d'authentification elle-même).
    fn test_state_plain() -> (ServerState, tokio::sync::watch::Sender<bool>, String) {
        let (mut state, tx, id) = test_state();
        state.allow_plaintext_local_auth = true;
        (state, tx, id)
    }

    /// Demande d'appairage honnête jusqu'à la révélation, directement sur le
    /// gestionnaire ; retourne l'identité de l'appareil et le code SAS.
    fn commit_reveal(state: &ServerState, device_id: &str) -> (E2eIdentity, String) {
        let device = E2eIdentity::generate();
        let cog_key = state.cog_identity.public_key_bytes();
        let code = direct_pair(&state.pairing, cog_key, &device, device_id, "192.168.1.30");
        (device, code)
    }

    /// Engagement + révélation signés, directement sur le gestionnaire ;
    /// retourne le code SAS (identique des deux côtés).
    fn direct_pair(
        mgr: &PairingManager,
        cog_key: [u8; 32],
        device: &E2eIdentity,
        device_id: &str,
        src: &str,
    ) -> String {
        let key = device.public_key_bytes();
        let pairing = DevicePairing::start(key);
        let c = parse_commitment_hex(&pairing.commitment_hex()).unwrap();
        let sig = parse_pairing_signature_hex(&pairing.commit_signature_hex(device, device_id).unwrap()).unwrap();
        let src: IpAddr = src.parse().unwrap();
        let nc = mgr
            .request_pairing(device_id, "Pixel", &hex::encode(key), c, &sig, src)
            .unwrap();
        let revealed = pairing.receive_cog_nonce(cog_key, nc).unwrap();
        let nd = PairingNonce::from_hex(&revealed.device_nonce_hex()).unwrap();
        let rsig =
            parse_pairing_signature_hex(&revealed.reveal_signature_hex(device, device_id).unwrap()).unwrap();
        let code = mgr.reveal_pairing(device_id, &nd, &rsig, src, &cog_key).unwrap();
        assert_eq!(code, revealed.code());
        code
    }

    fn app_for(state: ServerState, peer: &str) -> Router {
        let addr: SocketAddr = peer.parse().unwrap();
        build_router(state, None).layer(MockConnectInfo(addr))
    }

    async fn call(app: Router, req: axum::http::Request<Body>) -> (StatusCode, String) {
        let resp = app.oneshot(req).await.unwrap();
        let status = resp.status();
        let bytes = axum::body::to_bytes(resp.into_body(), 1 << 20).await.unwrap();
        (status, String::from_utf8_lossy(&bytes).to_string())
    }

    fn admin_req(uri: &str, secret: Option<&str>) -> axum::http::Request<Body> {
        let mut b = axum::http::Request::builder().method("GET").uri(uri);
        if let Some(s) = secret {
            b = b.header(ADMIN_SECRET_HEADER, s);
        }
        b.body(Body::empty()).unwrap()
    }

    fn auth_req(email: &str, password: &str) -> axum::http::Request<Body> {
        axum::http::Request::builder()
            .method("POST")
            .uri("/auth")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::json!({"email": email, "password": password}).to_string(),
            ))
            .unwrap()
    }

    #[test]
    fn default_config_is_loopback() {
        let config = RemoteServerConfig::default();
        assert_eq!(config.listen_addr, "127.0.0.1:8091");
        assert_eq!(config.cors_allowed_origins, Vec::<String>::new());
        assert_eq!(config.api_token_ttl_secs, DEFAULT_API_TOKEN_TTL_SECS);
    }

    #[test]
    fn cors_none_by_default_and_wildcard_ignored() {
        assert!(build_cors(&[]).is_none());
        assert!(build_cors(&["*".into()]).is_none());
        assert!(build_cors(&["http://localhost:3000".into()]).is_some());
    }

    #[test]
    fn ct_eq_works() {
        assert!(ct_eq(b"abc", b"abc"));
        assert!(!ct_eq(b"abc", b"abd"));
        assert!(!ct_eq(b"abc", b"abcd"));
    }

    #[test]
    fn rate_limiter_blocks_after_max() {
        let rl = RateLimiter::new(Duration::from_secs(60), 3);
        let ip: IpAddr = "10.0.0.1".parse().unwrap();
        assert!(rl.check(ip) && rl.check(ip) && rl.check(ip));
        assert!(!rl.check(ip));
        // Autre IP indépendante.
        assert!(rl.check("10.0.0.2".parse().unwrap()));
        // Fenêtre expirée.
        let rl = RateLimiter::new(Duration::from_millis(1), 1);
        assert!(rl.check(ip));
        std::thread::sleep(Duration::from_millis(5));
        assert!(rl.check(ip));
    }

    #[tokio::test]
    async fn admin_routes_refused_without_secret() {
        let (state, _tx, _) = test_state();
        for uri in ["/api/bridge/devices"] {
            let (st, _) = call(app_for(state.clone(), "127.0.0.1:5000"), admin_req(uri, None)).await;
            assert_eq!(st, StatusCode::UNAUTHORIZED);
            let (st, _) = call(app_for(state.clone(), "127.0.0.1:5000"), admin_req(uri, Some("wrong"))).await;
            assert_eq!(st, StatusCode::UNAUTHORIZED);
        }
        for (m, uri) in [
            ("POST", "/api/bridge/generate-token"),
            ("POST", "/api/bridge/approve"),
            ("POST", "/api/bridge/reject"),
        ] {
            let req = axum::http::Request::builder()
                .method(m)
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from("{\"device_id\":\"x\"}"))
                .unwrap();
            let (st, _) = call(app_for(state.clone(), "127.0.0.1:5000"), req).await;
            assert_eq!(st, StatusCode::UNAUTHORIZED, "{uri}");
        }
    }

    #[tokio::test]
    async fn admin_routes_refused_from_non_loopback_even_with_secret() {
        let (state, _tx, _) = test_state();
        let secret = state.admin_secret.as_str().to_string();
        for peer in ["192.168.1.50:5000", "[2001:db8::1]:5000"] {
            let (st, _) = call(
                app_for(state.clone(), peer),
                admin_req("/api/bridge/devices", Some(&secret)),
            )
            .await;
            assert_eq!(st, StatusCode::FORBIDDEN, "{peer}");
        }
    }

    #[tokio::test]
    async fn admin_routes_ok_with_secret_and_ids_masked() {
        let (state, _tx, _) = test_state();
        let full_id = "device-abcdef-1234567890";
        let (device, code) = commit_reveal(&state, full_id);
        let secret = state.admin_secret.as_str().to_string();
        let (st, body) = call(
            app_for(state.clone(), "127.0.0.1:5000"),
            admin_req("/api/bridge/devices", Some(&secret)),
        )
        .await;
        assert_eq!(st, StatusCode::OK);
        assert!(!body.contains(full_id), "device_id complet fuité: {body}");
        assert!(body.contains(&mask_device_id(full_id)));
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(v["pending"][0]["confirmation_code"], code.as_str());
        assert_eq!(v["pending"][0]["status"], "revealed");
        assert_eq!(
            v["pending"][0]["key_fingerprint"],
            identity_fingerprint(&device.public_key_bytes()).as_str()
        );
        assert_eq!(
            v["cog_key_fingerprint"],
            identity_fingerprint(&state.cog_identity.public_key_bytes()).as_str()
        );
    }

    #[tokio::test]
    async fn generate_token_has_expiry() {
        let (state, _tx, _) = test_state();
        let secret = state.admin_secret.as_str().to_string();
        let req = axum::http::Request::builder()
            .method("POST")
            .uri("/api/bridge/generate-token")
            .header("content-type", "application/json")
            .header(ADMIN_SECRET_HEADER, &secret)
            .body(Body::from("{\"device_name\":\"Tel\"}"))
            .unwrap();
        let (st, body) = call(app_for(state.clone(), "127.0.0.1:5000"), req).await;
        assert_eq!(st, StatusCode::OK);
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert!(state.pairing.validate_api_token(v["token"].as_str().unwrap()));
    }

    #[tokio::test]
    async fn lockout_cannot_be_bypassed() {
        let (state, _tx, _) = test_state_plain();
        // 5 échecs => verrouillage Connect (limite /auth : 10 par minute).
        for _ in 0..5 {
            let (_, body) = call(
                app_for(state.clone(), "127.0.0.1:6000"),
                auth_req("alice@example.org", "mauvais-mot-de-passe"),
            )
            .await;
            assert!(body.contains("\"success\":false"));
        }
        // Le bon mot de passe est désormais refusé : aucun repli legacy.
        let (_, body) = call(
            app_for(state.clone(), "127.0.0.1:6000"),
            auth_req("alice@example.org", PASSWORD),
        )
        .await;
        assert!(body.contains("\"success\":false"), "verrouillage contourné: {body}");
    }

    #[tokio::test]
    async fn valid_login_works_and_legacy_only_account_refused() {
        let (state, _tx, _) = test_state_plain();
        let (_, body) = call(
            app_for(state.clone(), "127.0.0.1:6001"),
            auth_req("alice@example.org", PASSWORD),
        )
        .await;
        assert!(body.contains("\"success\":true"), "{body}");
        // Compte sans hash Connect : pas de repli sign_in.
        let (_, body) = call(
            app_for(state.clone(), "127.0.0.1:6001"),
            auth_req("legacy@example.org", PASSWORD),
        )
        .await;
        assert!(body.contains("\"success\":false"), "{body}");
    }

    #[tokio::test]
    async fn auth_rate_limited_per_ip() {
        let (state, _tx, _) = test_state();
        let mut last = StatusCode::OK;
        for _ in 0..=AUTH_RATE_MAX {
            let (st, _) = call(
                app_for(state.clone(), "10.1.2.3:7000"),
                auth_req("inconnu@example.org", "x"),
            )
            .await;
            last = st;
        }
        assert_eq!(last, StatusCode::TOO_MANY_REQUESTS);
    }

    #[tokio::test]
    async fn ws_without_valid_token_refused() {
        let (state, _tx, _) = test_state();
        let (st, _) = call(
            app_for(state, "127.0.0.1:5000"),
            axum::http::Request::builder().uri("/ws").body(Body::empty()).unwrap(),
        )
        .await;
        assert_ne!(st, StatusCode::SWITCHING_PROTOCOLS);
        assert!(st.is_client_error());
    }

    // ── E2E mobile ↔ COG (R-023) ─────────────────────────────────────────

    use miyukini_cog_bridge::{DevicePairing, E2eSession};

    const PEER_LAN: &str = "192.168.1.20:7100";

    fn post_json(uri: &str, body: String) -> axum::http::Request<Body> {
        axum::http::Request::builder()
            .method("POST")
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap()
    }

    /// Appaire un appareil (demande + approbation desktop) et retourne son identité.
    fn paired_device(state: &ServerState, device_id: &str) -> E2eIdentity {
        let (device, code) = commit_reveal(state, device_id);
        state.pairing.approve_device(device_id, &code).unwrap();
        device
    }

    async fn handshake(state: &ServerState, device: &E2eIdentity) -> E2eSession {
        let cog_key = state.cog_identity.public_key_bytes();
        let (pending, offer) =
            PendingHandshake::initiate(E2eRole::Device, device, &cog_key).unwrap();
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", serde_json::to_string(&offer).unwrap()),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let cog_offer = E2eHandshake::from_json(body.as_bytes()).unwrap();
        pending.complete(&cog_offer).unwrap()
    }

    fn e2e_auth_req(
        session: &E2eSession,
        binding: &RequestBinding,
        email: &str,
        password: &str,
    ) -> (axum::http::Request<Body>, EncryptedMessage) {
        let msg = session
            .encrypt_json(
                binding,
                &RemoteAuthRequest {
                    email: email.into(),
                    password: password.into(),
                },
            )
            .unwrap();
        (raw_e2e_req(session, &msg), msg)
    }

    fn raw_e2e_req(session: &E2eSession, msg: &EncryptedMessage) -> axum::http::Request<Body> {
        axum::http::Request::builder()
            .method("POST")
            .uri(AUTH_ENDPOINT)
            .header("content-type", "application/json")
            .header(E2E_SESSION_HEADER, hex::encode(session.session_id()))
            .body(Body::from(serde_json::to_string(msg).unwrap()))
            .unwrap()
    }

    #[tokio::test]
    async fn e2e_handshake_paired_device_succeeds() {
        let (state, _tx, _) = test_state();
        let device = paired_device(&state, "device-0001-abcdef");
        let session = handshake(&state, &device).await;
        assert_eq!(state.e2e_sessions.len(), 1);
        assert!(state
            .e2e_sessions
            .get_hex(&hex::encode(session.session_id()))
            .is_some());
    }

    #[tokio::test]
    async fn e2e_handshake_unknown_device_refused() {
        let (state, _tx, _) = test_state();
        let stranger = E2eIdentity::generate();
        let (_p, offer) = PendingHandshake::initiate(
            E2eRole::Device,
            &stranger,
            &state.cog_identity.public_key_bytes(),
        )
        .unwrap();
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", serde_json::to_string(&offer).unwrap()),
        )
        .await;
        assert_eq!(st, StatusCode::FORBIDDEN);
        // Appareil seulement en attente (révélé, non approuvé) : refus également.
        let (pending_dev, _code) = commit_reveal(&state, "device-pending-01");
        let (_p, offer) = PendingHandshake::initiate(
            E2eRole::Device,
            &pending_dev,
            &state.cog_identity.public_key_bytes(),
        )
        .unwrap();
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", serde_json::to_string(&offer).unwrap()),
        )
        .await;
        assert_eq!(st, StatusCode::FORBIDDEN);
        assert!(state.e2e_sessions.is_empty());
    }

    #[tokio::test]
    async fn e2e_handshake_v1_offer_refused() {
        let (state, _tx, _) = test_state();
        let _device = paired_device(&state, "device-0002-abcdef");
        let v1 = serde_json::json!({
            "public_key": "00".repeat(32),
            "protocol_version": 1,
        });
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", v1.to_string()),
        )
        .await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        // Sans champ de version (v1 historique) : refus aussi.
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(
                "/api/bridge/e2e-handshake",
                format!("{{\"public_key\":\"{}\"}}", "00".repeat(32)),
            ),
        )
        .await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        // Corps surdimensionné : refusé avant tout décodage.
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", " ".repeat(MAX_HANDSHAKE_JSON_LEN + 1)),
        )
        .await;
        assert!(st.is_client_error());
        assert!(state.e2e_sessions.is_empty());
    }

    #[tokio::test]
    async fn e2e_handshake_tampered_offer_refused() {
        let (state, _tx, _) = test_state();
        let device = paired_device(&state, "device-0003-abcdef");
        let cog_key = state.cog_identity.public_key_bytes();
        // Clé éphémère substituée (signature invalide).
        let (_p, mut offer) = PendingHandshake::initiate(E2eRole::Device, &device, &cog_key).unwrap();
        let (_q, other) = PendingHandshake::initiate(E2eRole::Device, &device, &cog_key).unwrap();
        offer.ephemeral_key = other.ephemeral_key;
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", serde_json::to_string(&offer).unwrap()),
        )
        .await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        // Rôle usurpé (offre « COG »).
        let (_p, mut offer) = PendingHandshake::initiate(E2eRole::Device, &device, &cog_key).unwrap();
        offer.role = E2eRole::Cog;
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", serde_json::to_string(&offer).unwrap()),
        )
        .await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        // Offre signée pour un autre COG.
        let other_cog = E2eIdentity::generate();
        let (_p, offer) =
            PendingHandshake::initiate(E2eRole::Device, &device, &other_cog.public_key_bytes())
                .unwrap();
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json("/api/bridge/e2e-handshake", serde_json::to_string(&offer).unwrap()),
        )
        .await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        assert!(state.e2e_sessions.is_empty());
    }

    fn commit_body(device_id: &str, key_hex: &str, commitment_hex: &str, sig_hex: &str) -> String {
        serde_json::json!({
            "pairing_version": PAIRING_PROTOCOL_VERSION,
            "device_id": device_id,
            "device_name": "Pixel",
            "mode": "whitelist",
            "e2e_identity_key": key_hex,
            "commitment": commitment_hex,
            "commit_signature": sig_hex,
        })
        .to_string()
    }

    fn reveal_body(device_id: &str, nonce_hex: &str, sig_hex: &str) -> String {
        serde_json::json!({
            "pairing_version": PAIRING_PROTOCOL_VERSION,
            "device_id": device_id,
            "device_nonce": nonce_hex,
            "reveal_signature": sig_hex,
        })
        .to_string()
    }

    /// Corps d'engagement signé honnête pour `device`.
    fn signed_commit_body(device_id: &str, device: &E2eIdentity, pairing: &DevicePairing) -> String {
        commit_body(
            device_id,
            &hex::encode(device.public_key_bytes()),
            &pairing.commitment_hex(),
            &pairing.commit_signature_hex(device, device_id).unwrap(),
        )
    }

    /// Engagement HTTP signé ; retourne l'aléa du COG.
    async fn http_commit(
        state: &ServerState,
        peer: &str,
        device_id: &str,
        device: &E2eIdentity,
        pairing: &DevicePairing,
    ) -> PairingNonce {
        let (st, body) = call(
            app_for(state.clone(), peer),
            post_json(PAIR_ENDPOINT, signed_commit_body(device_id, device, pairing)),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let resp: PairCommitResponse = serde_json::from_str(&body).unwrap();
        PairingNonce::from_hex(&resp.cog_nonce).unwrap()
    }

    async fn pair_status(state: &ServerState, device_id: &str, key_hex: &str) -> String {
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(
                PAIR_STATUS_ENDPOINT,
                serde_json::json!({"device_id": device_id, "e2e_identity_key": key_hex}).to_string(),
            ),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        v["status"].as_str().unwrap().to_string()
    }

    fn admin_post(uri: &str, secret: &str, body: serde_json::Value) -> axum::http::Request<Body> {
        axum::http::Request::builder()
            .method("POST")
            .uri(uri)
            .header("content-type", "application/json")
            .header(ADMIN_SECRET_HEADER, secret)
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    #[tokio::test]
    async fn http_pairing_v2_end_to_end() {
        let (state, _tx, _) = test_state();
        let id = "device-0004-abcdef";
        let device = E2eIdentity::generate();
        let key = device.public_key_bytes();
        let key_hex = hex::encode(key);
        let pairing = DevicePairing::start(key);

        // 1. Engagement : la réponse porte la clé et l'aléa du COG, aucun code.
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_ENDPOINT, signed_commit_body(id, &device, &pairing)),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let resp: PairCommitResponse = serde_json::from_str(&body).unwrap();
        assert_eq!(resp.status, PairingStatus::Committed);
        assert_eq!(resp.pairing_version, PAIRING_PROTOCOL_VERSION);
        assert!(!body.contains("confirmation_code"));
        let cog_key = parse_identity_key_hex(&resp.cog_identity_key).unwrap();
        assert_eq!(cog_key, state.cog_identity.public_key_bytes());
        assert_eq!(pair_status(&state, id, &key_hex).await, "committed");

        // 3. Révélation.
        let nc = PairingNonce::from_hex(&resp.cog_nonce).unwrap();
        let revealed = pairing.receive_cog_nonce(cog_key, nc).unwrap();
        let rsig = revealed.reveal_signature_hex(&device, id).unwrap();
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &revealed.device_nonce_hex(), &rsig)),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        assert_eq!(pair_status(&state, id, &key_hex).await, "revealed");
        // Seconde révélation refusée.
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &revealed.device_nonce_hex(), &rsig)),
        )
        .await;
        assert_eq!(st, StatusCode::CONFLICT);

        // 4. Central affiche le même code que le téléphone.
        let pending = state.pairing.pending_devices();
        assert_eq!(pending[0].confirmation_code.as_deref(), Some(revealed.code()));
        assert!(!state.pairing.is_whitelisted(id), "jamais d'approbation implicite");

        // Approbation : mauvais code refusé, bon code accepté.
        let secret = state.admin_secret.as_str().to_string();
        let wrong = if revealed.code() == "00000000" { "11111111" } else { "00000000" };
        let (st, _) = call(
            app_for(state.clone(), "127.0.0.1:5000"),
            admin_post(
                "/api/bridge/approve",
                &secret,
                serde_json::json!({"device_id": id, "confirmation_code": wrong}),
            ),
        )
        .await;
        assert_eq!(st, StatusCode::CONFLICT);
        // Sans code : rejet par le désérialiseur.
        let (st, _) = call(
            app_for(state.clone(), "127.0.0.1:5000"),
            admin_post("/api/bridge/approve", &secret, serde_json::json!({"device_id": id})),
        )
        .await;
        assert!(st.is_client_error());
        assert!(!state.pairing.is_whitelisted(id));
        let (st, _) = call(
            app_for(state.clone(), "127.0.0.1:5000"),
            admin_post(
                "/api/bridge/approve",
                &secret,
                serde_json::json!({"device_id": id, "confirmation_code": revealed.code()}),
            ),
        )
        .await;
        assert_eq!(st, StatusCode::OK);
        assert_eq!(pair_status(&state, id, &key_hex).await, "approved");
        let paired = state.pairing.device_for_identity_key(&key_hex).unwrap();
        assert_eq!(paired.device_id, id);
        // La clé épinglée permet le handshake E2E.
        let _session = handshake(&state, &device).await;
    }

    #[tokio::test]
    async fn http_old_pairing_protocol_refused() {
        let (state, _tx, _) = test_state();
        let key_hex = hex::encode(E2eIdentity::generate().public_key_bytes());
        // Requête v1 (sans version ni engagement).
        let v1 = serde_json::json!({
            "device_id": "d-000000001", "device_name": "x",
            "mode": "whitelist", "e2e_identity_key": key_hex,
        });
        let (st, body) = call(app_for(state.clone(), PEER_LAN), post_json(PAIR_ENDPOINT, v1.to_string())).await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        assert!(body.contains(STATUS_UNSUPPORTED_VERSION), "{body}");
        // Version future.
        let mut v3: serde_json::Value =
            serde_json::from_str(&commit_body("d-000000001", &key_hex, &"00".repeat(32), &"00".repeat(64))).unwrap();
        v3["pairing_version"] = 3.into();
        let (st, body) = call(app_for(state.clone(), PEER_LAN), post_json(PAIR_ENDPOINT, v3.to_string())).await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        assert!(body.contains(STATUS_UNSUPPORTED_VERSION));
        // Révélation v1 refusée aussi.
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(
                PAIR_REVEAL_ENDPOINT,
                serde_json::json!({"device_id": "d-000000001", "device_nonce": "00".repeat(32)}).to_string(),
            ),
        )
        .await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        assert_eq!(state.pairing.pending_devices().len(), 0);
    }

    #[tokio::test]
    async fn http_reveal_wrong_nonce_or_without_commit_refused() {
        let (state, _tx, _) = test_state();
        // Révélation sans engagement préalable (aucun Nc émis).
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(
                PAIR_REVEAL_ENDPOINT,
                reveal_body("device-0010-abcdef", &"11".repeat(32), &"00".repeat(64)),
            ),
        )
        .await;
        assert_eq!(st, StatusCode::NOT_FOUND, "{body}");
        // Mauvais Nd valablement signé par Kd : appairage abandonné.
        let device = E2eIdentity::generate();
        let key_hex = hex::encode(device.public_key_bytes());
        let pairing = DevicePairing::start(device.public_key_bytes());
        let id = "device-0011-abcdef";
        let nc = http_commit(&state, PEER_LAN, id, &device, &pairing).await;
        let wrong_nd = PairingNonce::from_bytes([0x22; 32]);
        let sig = miyukini_cog_bridge::sign_pairing_reveal(&device, id, &wrong_nd, &nc).unwrap();
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &"22".repeat(32), &hex::encode(sig))),
        )
        .await;
        assert_eq!(st, StatusCode::FORBIDDEN, "{body}");
        assert!(body.contains("commitment_mismatch"));
        assert_eq!(state.pairing.pending_devices().len(), 0);
        assert_eq!(pair_status(&state, "device-0011-abcdef", &key_hex).await, "rejected");
    }

    #[tokio::test]
    async fn http_pending_overwrite_and_per_ip_cap_refused() {
        let (state, _tx, _) = test_state();
        let commit = |id: &str| {
            let device = E2eIdentity::generate();
            signed_commit_body(id, &device, &DevicePairing::start(device.public_key_bytes()))
        };
        let (st, _) = call(app_for(state.clone(), PEER_LAN), post_json(PAIR_ENDPOINT, commit("device-0020-abcdef"))).await;
        assert_eq!(st, StatusCode::OK);
        // Autre clé pour le même appareil, depuis une autre IP : conflit.
        let (st, body) = call(
            app_for(state.clone(), "192.168.1.99:7000"),
            post_json(PAIR_ENDPOINT, commit("device-0020-abcdef")),
        )
        .await;
        assert_eq!(st, StatusCode::CONFLICT);
        assert!(body.contains("pending_conflict"));
        // Plafond par IP (3) : la 4e demande depuis la même IP est refusée.
        for i in 1..3 {
            let (st, _) = call(
                app_for(state.clone(), PEER_LAN),
                post_json(PAIR_ENDPOINT, commit(&format!("device-002{i}-abcdef"))),
            )
            .await;
            assert_eq!(st, StatusCode::OK);
        }
        let (st, body) = call(app_for(state.clone(), PEER_LAN), post_json(PAIR_ENDPOINT, commit("device-0029-abcdef"))).await;
        assert_eq!(st, StatusCode::TOO_MANY_REQUESTS);
        assert!(body.contains("too_many_pending_for_source"));
        assert_eq!(state.pairing.pending_devices().len(), 3);
    }

    #[tokio::test]
    async fn http_unsigned_or_wrongly_signed_commit_refused() {
        let (state, _tx, _) = test_state();
        let device = E2eIdentity::generate();
        let key_hex = hex::encode(device.public_key_bytes());
        let pairing = DevicePairing::start(device.public_key_bytes());
        let id = "device-0030-abcdef";
        // Requête v2 non signée (format antérieur) : refus explicite, sans repli.
        let unsigned = serde_json::json!({
            "pairing_version": PAIRING_PROTOCOL_VERSION, "device_id": id, "device_name": "Pixel",
            "mode": "whitelist", "e2e_identity_key": key_hex, "commitment": pairing.commitment_hex(),
        });
        let (st, body) = call(app_for(state.clone(), PEER_LAN), post_json(PAIR_ENDPOINT, unsigned.to_string())).await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        assert!(body.contains(STATUS_MISSING_SIGNATURE), "{body}");
        // Signature mal formée.
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_ENDPOINT, commit_body(id, &key_hex, &pairing.commitment_hex(), "zz")),
        )
        .await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        assert!(body.contains(STATUS_INVALID_SIGNATURE), "{body}");
        // Signature d'une autre clé.
        let other = E2eIdentity::generate();
        let c = parse_commitment_hex(&pairing.commitment_hex()).unwrap();
        let sig = miyukini_cog_bridge::sign_pairing_commit(&other, id, &c).unwrap();
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_ENDPOINT, commit_body(id, &key_hex, &pairing.commitment_hex(), &hex::encode(sig))),
        )
        .await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        assert!(body.contains(STATUS_INVALID_SIGNATURE), "{body}");
        assert_eq!(state.pairing.pending_devices().len(), 0);
    }

    #[tokio::test]
    async fn http_bad_reveal_leaves_pending_entry_intact() {
        let (state, _tx, _) = test_state();
        let device = E2eIdentity::generate();
        let key_hex = hex::encode(device.public_key_bytes());
        let pairing = DevicePairing::start(device.public_key_bytes());
        let id = "device-0031-abcdef";
        let nc = http_commit(&state, PEER_LAN, id, &device, &pairing).await;
        let wrong = "33".repeat(32);
        // 1. Révélation non signée (v2 antérieure) : refus explicite.
        let unsigned = serde_json::json!({
            "pairing_version": PAIRING_PROTOCOL_VERSION, "device_id": id, "device_nonce": wrong,
        });
        let (st, body) =
            call(app_for(state.clone(), PEER_LAN), post_json(PAIR_REVEAL_ENDPOINT, unsigned.to_string())).await;
        assert_eq!(st, StatusCode::BAD_REQUEST);
        assert!(body.contains(STATUS_MISSING_SIGNATURE), "{body}");
        // 2. Signée par un tiers qui connaît device_id.
        let attacker = E2eIdentity::generate();
        let wrong_nd = PairingNonce::from_hex(&wrong).unwrap();
        let sig = miyukini_cog_bridge::sign_pairing_reveal(&attacker, id, &wrong_nd, &nc).unwrap();
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &wrong, &hex::encode(sig))),
        )
        .await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        assert!(body.contains(STATUS_INVALID_SIGNATURE), "{body}");
        // 3. Révélation honnête rejouée depuis une autre adresse.
        let revealed = pairing.receive_cog_nonce(state.cog_identity.public_key_bytes(), nc).unwrap();
        let rsig = revealed.reveal_signature_hex(&device, id).unwrap();
        let (st, body) = call(
            app_for(state.clone(), "192.168.1.99:7000"),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &revealed.device_nonce_hex(), &rsig)),
        )
        .await;
        assert_eq!(st, StatusCode::FORBIDDEN);
        assert!(body.contains(STATUS_SOURCE_MISMATCH), "{body}");
        // L'entrée est intacte : la révélation légitime aboutit.
        assert_eq!(pair_status(&state, id, &key_hex).await, "committed");
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &revealed.device_nonce_hex(), &rsig)),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        assert_eq!(pair_status(&state, id, &key_hex).await, "revealed");
    }

    #[tokio::test]
    async fn http_same_key_recommit_after_reveal_or_other_source_refused() {
        let (state, _tx, _) = test_state();
        let device = E2eIdentity::generate();
        let id = "device-0032-abcdef";
        let pairing = DevicePairing::start(device.public_key_bytes());
        let nc = http_commit(&state, PEER_LAN, id, &device, &pairing).await;
        // Relance (nouvel Nd) depuis une autre source : refusée.
        let retry = DevicePairing::start(device.public_key_bytes());
        let (st, body) = call(
            app_for(state.clone(), "192.168.1.99:7000"),
            post_json(PAIR_ENDPOINT, signed_commit_body(id, &device, &retry)),
        )
        .await;
        assert_eq!(st, StatusCode::FORBIDDEN);
        assert!(body.contains(STATUS_SOURCE_MISMATCH), "{body}");
        // Révélation, puis relance depuis la même source : refusée (PendingConflict).
        let revealed = pairing.receive_cog_nonce(state.cog_identity.public_key_bytes(), nc).unwrap();
        let rsig = revealed.reveal_signature_hex(&device, id).unwrap();
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_REVEAL_ENDPOINT, reveal_body(id, &revealed.device_nonce_hex(), &rsig)),
        )
        .await;
        assert_eq!(st, StatusCode::OK);
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(PAIR_ENDPOINT, signed_commit_body(id, &device, &retry)),
        )
        .await;
        assert_eq!(st, StatusCode::CONFLICT);
        assert!(body.contains("pending_conflict"), "{body}");
        assert_eq!(
            state.pairing.pending_devices()[0].confirmation_code.as_deref(),
            Some(revealed.code())
        );
    }

    #[tokio::test]
    async fn pair_without_valid_key_or_commitment_refused() {
        let (state, _tx, _) = test_state();
        // Sans clé : rejet par le désérialiseur.
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(
                PAIR_ENDPOINT,
                serde_json::json!({
                    "pairing_version": PAIRING_PROTOCOL_VERSION,
                    "device_id": "d-000000001", "device_name": "x", "mode": "whitelist",
                    "commitment": "00".repeat(32),
                })
                .to_string(),
            ),
        )
        .await;
        assert!(st.is_client_error());
        // Clé faible / mal formée.
        let mut weak = [0u8; 32];
        weak[0] = 1;
        for key in [hex::encode(weak), "zz".into(), "00".repeat(31)] {
            let (st, _) = call(
                app_for(state.clone(), PEER_LAN),
                post_json(PAIR_ENDPOINT, commit_body("d-000000001", &key, &"00".repeat(32), &"00".repeat(64))),
            )
            .await;
            assert_eq!(st, StatusCode::BAD_REQUEST);
        }
        // Engagement mal formé.
        let key_hex = hex::encode(E2eIdentity::generate().public_key_bytes());
        for c in ["", "zz", &"00".repeat(31)] {
            let (st, _) = call(
                app_for(state.clone(), PEER_LAN),
                post_json(PAIR_ENDPOINT, commit_body("d-000000001", &key_hex, c, &"00".repeat(64))),
            )
            .await;
            assert_eq!(st, StatusCode::BAD_REQUEST);
        }
        assert_eq!(state.pairing.pending_devices().len(), 0);
    }

    #[tokio::test]
    async fn auth_over_e2e_succeeds_and_response_bound() {
        let (state, _tx, _) = test_state();
        let device = paired_device(&state, "device-0005-abcdef");
        let session = handshake(&state, &device).await;
        let binding = RequestBinding::new("POST", AUTH_ENDPOINT).unwrap();
        let (req, _) = e2e_auth_req(&session, &binding, "alice@example.org", PASSWORD);
        let (st, body) = call(app_for(state.clone(), PEER_LAN), req).await;
        assert_eq!(st, StatusCode::OK);
        // Rien en clair dans la réponse.
        assert!(!body.contains("success"), "{body}");
        let msg = EncryptedMessage::from_json(body.as_bytes()).unwrap();
        let resp: RemoteAuthResponse = session.decrypt_json(&binding, &msg).unwrap();
        assert!(resp.success);
        assert!(resp.token.is_some());

        // Mauvais mot de passe : réponse chiffrée success=false.
        let binding = RequestBinding::new("POST", AUTH_ENDPOINT).unwrap();
        let (req, _) = e2e_auth_req(&session, &binding, "alice@example.org", "faux");
        let (st, body) = call(app_for(state.clone(), PEER_LAN), req).await;
        assert_eq!(st, StatusCode::OK);
        let msg = EncryptedMessage::from_json(body.as_bytes()).unwrap();
        let resp: RemoteAuthResponse = session.decrypt_json(&binding, &msg).unwrap();
        assert!(!resp.success);
    }

    #[tokio::test]
    async fn auth_e2e_replay_wrong_endpoint_and_unknown_session_refused() {
        let (state, _tx, _) = test_state();
        let device = paired_device(&state, "device-0006-abcdef");
        let session = handshake(&state, &device).await;

        // Rejeu d'une requête déjà acceptée.
        let binding = RequestBinding::new("POST", AUTH_ENDPOINT).unwrap();
        let (req, msg) = e2e_auth_req(&session, &binding, "alice@example.org", PASSWORD);
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req).await;
        assert_eq!(st, StatusCode::OK);
        let (st, _) = call(app_for(state.clone(), PEER_LAN), raw_e2e_req(&session, &msg)).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);

        // Chiffré produit pour un autre point d'accès.
        let other = RequestBinding::new("POST", "/api/autre").unwrap();
        let (req, _) = e2e_auth_req(&session, &other, "alice@example.org", PASSWORD);
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);

        // Session inconnue.
        let binding = RequestBinding::new("POST", AUTH_ENDPOINT).unwrap();
        let (_, msg) = e2e_auth_req(&session, &binding, "alice@example.org", PASSWORD);
        let req = axum::http::Request::builder()
            .method("POST")
            .uri(AUTH_ENDPOINT)
            .header("content-type", "application/json")
            .header(E2E_SESSION_HEADER, "00".repeat(32))
            .body(Body::from(serde_json::to_string(&msg).unwrap()))
            .unwrap();
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn auth_e2e_refused_after_revocation() {
        let (state, _tx, _) = test_state();
        let device = paired_device(&state, "device-0007-abcdef");
        let session = handshake(&state, &device).await;
        assert_eq!(state.pairing.revoke_device("device-0007-abcdef"), Ok(true));
        let binding = RequestBinding::new("POST", AUTH_ENDPOINT).unwrap();
        let (req, _) = e2e_auth_req(&session, &binding, "alice@example.org", PASSWORD);
        let (st, _) = call(app_for(state.clone(), PEER_LAN), req).await;
        assert_eq!(st, StatusCode::UNAUTHORIZED);
        assert!(state.e2e_sessions.is_empty());
    }

    #[tokio::test]
    async fn clear_auth_refused_from_network_peer() {
        let (state, _tx, _) = test_state();
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            auth_req("alice@example.org", PASSWORD),
        )
        .await;
        assert_eq!(st, StatusCode::FORBIDDEN);
        assert!(body.contains("\"success\":false"));
        assert!(!body.contains("token\":\""));
    }

    #[tokio::test]
    async fn clear_auth_refused_by_default_even_from_loopback() {
        let (state, _tx, _) = test_state();
        assert!(!RemoteServerConfig::default().allow_plaintext_local_auth);
        let (st, body) = call(
            app_for(state.clone(), "127.0.0.1:6100"),
            auth_req("alice@example.org", PASSWORD),
        )
        .await;
        assert_eq!(st, StatusCode::FORBIDDEN);
        assert!(!body.contains("token\":\""));
    }

    #[tokio::test]
    async fn clear_auth_loopback_only_when_flag_set() {
        let (state, _tx, _) = test_state_plain();
        let (st, body) = call(
            app_for(state.clone(), "127.0.0.1:6101"),
            auth_req("alice@example.org", PASSWORD),
        )
        .await;
        assert_eq!(st, StatusCode::OK);
        assert!(body.contains("\"success\":true"), "{body}");
        // Pair réseau : toujours refusé, même avec l'option.
        let (st, _) = call(app_for(state.clone(), PEER_LAN), auth_req("alice@example.org", PASSWORD)).await;
        assert_eq!(st, StatusCode::FORBIDDEN);
    }

    #[test]
    fn plaintext_auth_policy() {
        assert!(!plaintext_auth_allowed(false, "127.0.0.1".parse().unwrap()));
        assert!(plaintext_auth_allowed(true, "127.0.0.1".parse().unwrap()));
        assert!(plaintext_auth_allowed(true, "::ffff:127.0.0.1".parse().unwrap()));
        assert!(!plaintext_auth_allowed(true, "192.168.1.2".parse().unwrap()));
    }

    #[tokio::test]
    async fn auth_body_bounded() {
        let (state, _tx, _) = test_state();
        let (st, _) = call(
            app_for(state.clone(), "127.0.0.1:6200"),
            post_json(AUTH_ENDPOINT, " ".repeat(AUTH_MAX_BODY + 1)),
        )
        .await;
        assert_eq!(st, StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn server_start_creates_persistent_identity() {
        let dir = std::env::temp_dir().join(format!("central-remote-srv-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let config = RemoteServerConfig {
            listen_addr: "127.0.0.1:0".into(),
            auth_db_path: dir.join("central.db").to_string_lossy().to_string(),
            ..RemoteServerConfig::default()
        };
        let h1 = start_remote_server(config.clone(), RemoteBridge::new()).await.unwrap();
        let k1 = h1.e2e_identity_public_key_hex().to_string();
        drop(h1);
        assert!(dir.join(DEFAULT_IDENTITY_FILE).exists());
        let h2 = start_remote_server(config, RemoteBridge::new()).await.unwrap();
        assert_eq!(h2.e2e_identity_public_key_hex(), k1);
        assert_eq!(h2.e2e_identity_fingerprint().split(' ').count(), 8);
    }

    #[tokio::test]
    async fn server_whitelist_persists_and_corruption_blocks_start() {
        let dir = std::env::temp_dir().join(format!("central-remote-wl-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let config = RemoteServerConfig {
            listen_addr: "127.0.0.1:0".into(),
            auth_db_path: dir.join("central.db").to_string_lossy().to_string(),
            ..RemoteServerConfig::default()
        };
        let h1 = start_remote_server(config.clone(), RemoteBridge::new()).await.unwrap();
        let device = E2eIdentity::generate();
        let key = device.public_key_bytes();
        let cog_key = parse_identity_key_hex(h1.e2e_identity_public_key_hex()).unwrap();
        let code = direct_pair(&h1.pairing, cog_key, &device, "device-persist-01", "10.0.0.5");
        h1.pairing.approve_device("device-persist-01", &code).unwrap();
        drop(h1);

        let file = dir.join(DEFAULT_PAIRED_DEVICES_FILE);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&file).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o600);
        }
        let h2 = start_remote_server(config.clone(), RemoteBridge::new()).await.unwrap();
        assert!(h2.pairing.is_paired_with_key("device-persist-01", &hex::encode(key)));
        drop(h2);

        // Fichier corrompu : refus de démarrer (fail-closed), message explicite.
        std::fs::write(&file, b"{corrompu").unwrap();
        let Err(err) = start_remote_server(config, RemoteBridge::new()).await else {
            panic!("démarrage accepté avec une liste blanche corrompue");
        };
        assert!(err.contains("liste blanche"), "{err}");
    }
}
