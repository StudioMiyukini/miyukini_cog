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
//! - `POST /auth` en clair : refusé hors loopback (politique par défaut
//!   [`ClearAuthPolicy::LoopbackOnly`], portail web local), ou partout avec
//!   [`ClearAuthPolicy::Refuse`].

use crate::bridge::RemoteBridge;
use crate::e2e::{load_or_create_cog_identity, E2eSessionStore, DEFAULT_IDENTITY_FILE};
use crate::pairing::{
    mask_device_id, PairRequest, PairingError, PairingManager, WhitelistValidateRequest,
    DEFAULT_API_TOKEN_TTL_SECS,
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
    pairing_confirmation_code, parse_identity_key_hex, E2eHandshake, E2eIdentity, E2eRole,
    EncryptedMessage, PendingHandshake, RequestBinding, E2E_SESSION_HEADER,
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
    /// Politique d'acceptation de `/auth` sans chiffrement E2E.
    pub clear_auth_policy: ClearAuthPolicy,
}

/// Politique d'acceptation de `/auth` en clair (sans session E2E).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClearAuthPolicy {
    /// Clair accepté uniquement depuis un pair loopback (portail web local,
    /// tunnel SSH) ; tout pair réseau doit passer par une session E2E.
    #[default]
    LoopbackOnly,
    /// Clair toujours refusé : E2E obligatoire pour tout client.
    Refuse,
}

impl ClearAuthPolicy {
    /// Vrai si un `/auth` en clair est accepté depuis `ip`.
    pub fn allows(self, ip: IpAddr) -> bool {
        match self {
            Self::LoopbackOnly => ip.to_canonical().is_loopback(),
            Self::Refuse => false,
        }
    }
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
            clear_auth_policy: ClearAuthPolicy::LoopbackOnly,
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
    /// Identité E2E longue durée du COG (secret effacé à la destruction).
    cog_identity: Arc<E2eIdentity>,
    /// Sessions E2E établies avec les appareils appairés.
    e2e_sessions: Arc<E2eSessionStore>,
    /// Politique d'acceptation de /auth en clair.
    clear_auth_policy: ClearAuthPolicy,
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
            "/api/bridge/pair",
            post(handle_bridge_pair).layer(DefaultBodyLimit::max(PAIR_MAX_BODY)),
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
    let pairing = Arc::new(PairingManager::with_token_ttl(config.api_token_ttl_secs));

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
        cog_identity,
        e2e_sessions: Arc::new(E2eSessionStore::default()),
        clear_auth_policy: config.clear_auth_policy,
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
/// seulement si la [`ClearAuthPolicy`] l'autorise pour ce pair.
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

/// /auth en clair : uniquement si la politique l'autorise pour ce pair.
fn handle_auth_clear(state: &ServerState, peer: SocketAddr, body: &[u8]) -> Response {
    if !state.clear_auth_policy.allows(peer.ip()) {
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

/// POST /api/bridge/pair — demande d'appairage depuis un device mobile.
///
/// L'appareil fournit sa clé publique d'identité E2E (obligatoire, validée) ;
/// la réponse porte la clé publique d'identité du COG (à épingler) et le code
/// de confirmation à comparer avec celui affiché côté desktop. L'approbation
/// sur le desktop reste requise (inchangé).
async fn handle_bridge_pair(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    Json(req): Json<PairRequest>,
) -> Response {
    if !state.bridge_limiter.check(peer.ip().to_canonical()) {
        return StatusCode::TOO_MANY_REQUESTS.into_response();
    }
    let Ok(device_key) = parse_identity_key_hex(&req.e2e_identity_key) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"status": "invalid_e2e_identity_key"})),
        )
            .into_response();
    };
    let cog_key = state.cog_identity.public_key_bytes();
    let code = pairing_confirmation_code(&device_key, &cog_key);
    match state.pairing.request_pairing(
        &req.device_id,
        &req.device_name,
        &hex::encode(device_key),
        &code,
    ) {
        Ok(resp) => Json(serde_json::json!({
            "status": resp.status,
            "token": resp.token,
            "message": resp.message,
            "cog_identity_key": hex::encode(cog_key),
            "confirmation_code": code,
        }))
        .into_response(),
        Err(PairingError::InvalidDeviceId) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"status": "invalid_device_id"})),
        )
            .into_response(),
        Err(PairingError::TooManyPending | PairingError::Unavailable) => {
            StatusCode::SERVICE_UNAVAILABLE.into_response()
        }
    }
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
/// (admin local ; les `device_id` sont masqués).
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
                "confirmation_code": d.confirmation_code,
            })
        })
        .collect();
    Json(serde_json::json!({ "paired": paired, "pending": pending }))
}

/// Requête d'approbation/rejet.
#[derive(Deserialize)]
struct DeviceAction {
    device_id: String,
}

/// POST /api/bridge/approve — approuve un device en attente (depuis le desktop).
async fn handle_bridge_approve(
    State(state): State<ServerState>,
    Json(req): Json<DeviceAction>,
) -> Json<serde_json::Value> {
    let ok = state.pairing.approve_device(&req.device_id);
    Json(serde_json::json!({"approved": ok}))
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
            cog_identity: Arc::new(E2eIdentity::generate()),
            e2e_sessions: Arc::new(E2eSessionStore::default()),
            clear_auth_policy: ClearAuthPolicy::LoopbackOnly,
            shutdown: rx,
        };
        (state, tx, profile.id)
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
        state
            .pairing
            .request_pairing(full_id, "Pixel", &"aa".repeat(32), "123456")
            .unwrap();
        let secret = state.admin_secret.as_str().to_string();
        let (st, body) = call(
            app_for(state.clone(), "127.0.0.1:5000"),
            admin_req("/api/bridge/devices", Some(&secret)),
        )
        .await;
        assert_eq!(st, StatusCode::OK);
        assert!(!body.contains(full_id), "device_id complet fuité: {body}");
        assert!(body.contains(&mask_device_id(full_id)));
        assert!(body.contains("\"confirmation_code\":\"123456\""));
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
        let (state, _tx, _) = test_state();
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
        let (state, _tx, _) = test_state();
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

    use miyukini_cog_bridge::E2eSession;

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
        let device = E2eIdentity::generate();
        let key = hex::encode(device.public_key_bytes());
        state
            .pairing
            .request_pairing(device_id, "Pixel", &key, "000000")
            .unwrap();
        assert!(state.pairing.approve_device(device_id));
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
        // Appareil seulement en attente (non approuvé) : refus également.
        let pending_dev = E2eIdentity::generate();
        state
            .pairing
            .request_pairing(
                "device-pending-01",
                "x",
                &hex::encode(pending_dev.public_key_bytes()),
                "1",
            )
            .unwrap();
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

    #[tokio::test]
    async fn pair_exchanges_and_pins_keys() {
        let (state, _tx, _) = test_state();
        let device = E2eIdentity::generate();
        let device_key = hex::encode(device.public_key_bytes());
        let (st, body) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(
                "/api/bridge/pair",
                serde_json::json!({
                    "device_id": "device-0004-abcdef",
                    "device_name": "Pixel",
                    "mode": "whitelist",
                    "e2e_identity_key": device_key,
                })
                .to_string(),
            ),
        )
        .await;
        assert_eq!(st, StatusCode::OK, "{body}");
        let v: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(v["status"], "pending");
        let cog_key = state.cog_identity.public_key_bytes();
        assert_eq!(v["cog_identity_key"], hex::encode(cog_key));
        let code = pairing_confirmation_code(&device.public_key_bytes(), &cog_key);
        assert_eq!(v["confirmation_code"], code.as_str());
        let pending = state.pairing.pending_devices();
        assert_eq!(pending[0].e2e_identity_key, device_key);
        assert_eq!(pending[0].confirmation_code, code);
        // Approbation desktop : la clé est épinglée dans la whitelist.
        assert!(state.pairing.approve_device("device-0004-abcdef"));
        let paired = state.pairing.device_for_identity_key(&device_key).unwrap();
        assert_eq!(paired.device_id, "device-0004-abcdef");
    }

    #[tokio::test]
    async fn pair_without_valid_key_refused() {
        let (state, _tx, _) = test_state();
        // Sans clé : rejet par le désérialiseur.
        let (st, _) = call(
            app_for(state.clone(), PEER_LAN),
            post_json(
                "/api/bridge/pair",
                serde_json::json!({"device_id": "d-000000001", "device_name": "x", "mode": "whitelist"})
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
                post_json(
                    "/api/bridge/pair",
                    serde_json::json!({
                        "device_id": "d-000000001", "device_name": "x",
                        "mode": "whitelist", "e2e_identity_key": key,
                    })
                    .to_string(),
                ),
            )
            .await;
            assert_eq!(st, StatusCode::BAD_REQUEST);
        }
        assert!(state.pairing.pending_devices().is_empty());
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
        assert!(state.pairing.revoke_device("device-0007-abcdef"));
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
    async fn clear_auth_refused_everywhere_with_refuse_policy() {
        let (mut state, _tx, _) = test_state();
        state.clear_auth_policy = ClearAuthPolicy::Refuse;
        let (st, _) = call(
            app_for(state.clone(), "127.0.0.1:6100"),
            auth_req("alice@example.org", PASSWORD),
        )
        .await;
        assert_eq!(st, StatusCode::FORBIDDEN);
    }

    #[test]
    fn clear_auth_policy_default_is_loopback_only() {
        let p = RemoteServerConfig::default().clear_auth_policy;
        assert_eq!(p, ClearAuthPolicy::LoopbackOnly);
        assert!(p.allows("127.0.0.1".parse().unwrap()));
        assert!(p.allows("::ffff:127.0.0.1".parse().unwrap()));
        assert!(!p.allows("192.168.1.2".parse().unwrap()));
        assert!(!ClearAuthPolicy::Refuse.allows("127.0.0.1".parse().unwrap()));
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
    }
}
