//! Serveur HTTP + WebSocket embarqué pour CentralRemote.
//!
//! @id     central_remote_server
//! @do run_embedded_remote_server
//@human ! @role   infra
//! @layer toolkit
//! @human  Serveur Axum lancé par Central quand le remote est activé.
//! Expose : POST /auth (login), GET /ws (WebSocket), GET /status (santé).
//! Les routes d'administration de l'appairage exigent un pair loopback + secret.

use crate::bridge::RemoteBridge;
use crate::pairing::{
    mask_device_id, PairRequest, PairingManager, WhitelistValidateRequest,
    DEFAULT_API_TOKEN_TTL_SECS,
};
use crate::protocol::{RemoteAuthRequest, RemoteAuthResponse, RemoteCommand, RemoteEvent};
use crate::session::SessionManager;

use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, Request, State};
use axum::http::{header, HeaderMap, HeaderValue, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_util::{SinkExt, StreamExt};
use miyukini_central::auth::CentralAuthDb;
use miyukini_connect::{AuthVerifyRequest, ConnectService, IdentitySetup, PermissionTier};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, VecDeque};
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
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
}

impl Default for RemoteServerConfig {
    fn default() -> Self {
        Self {
            // Loopback par défaut (fail-closed) ; le LAN nécessite une config explicite.
            listen_addr: "127.0.0.1:8091".into(),
            auth_db_path: "central.db".into(),
            cors_allowed_origins: Vec::new(),
            api_token_ttl_secs: DEFAULT_API_TOKEN_TTL_SECS,
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
            hits.retain(|_, q| q.back().map_or(false, |t| now.duration_since(*t) < window));
        }
        let q = hits.entry(ip).or_default();
        while q.front().map_or(false, |t| now.duration_since(*t) >= self.window) {
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
        .route("/auth", post(handle_auth))
        .route("/ws", get(handle_ws_upgrade))
        .route("/status", get(handle_status))
        // Endpoints d'appairage mobile (COG Bridge)
        .route("/api/bridge/validate", post(handle_bridge_validate))
        .route("/api/bridge/validate-token", get(handle_bridge_validate_token))
        .route("/api/bridge/pair", post(handle_bridge_pair))
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
    let auth_db = CentralAuthDb::open(Path::new(&config.auth_db_path).to_path_buf())
        .map_err(|e| format!("CentralRemote auth DB: {e}"))?;

    let connect = ConnectService::new("2026.03.05");
    let sessions = SessionManager::with_random_secret();
    let pairing = Arc::new(PairingManager::with_token_ttl(config.api_token_ttl_secs));

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
        _discovery: discovery_handle,
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
    /// Handle du listener UDP (gardé pour qu'il reste actif).
    _discovery: Option<crate::discovery::DiscoveryHandle>,
}

impl RemoteServerHandle {
    /// Arrête le serveur proprement (ferme aussi les WebSocket ouverts et l'UDP).
    pub fn shutdown(&mut self) {
        let _ = self.ws_shutdown_tx.send(true);
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        if let Some(d) = self._discovery.as_mut() {
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
    Json(RemoteAuthResponse {
        success: false,
        token: None,
        user_display_name: None,
        error: Some(msg.into()),
    })
}

/// POST /auth — authentification et obtention d'un token.
async fn handle_auth(
    State(state): State<ServerState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    Json(req): Json<RemoteAuthRequest>,
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

    let email = req.email.trim();
    if email.is_empty() || req.password.is_empty() {
        return auth_failure("Email et mot de passe requis").into_response();
    }

    // Recherche du profil par email.
    let profiles = match state.auth_db.list_profiles() {
        Ok(p) => p,
        Err(e) => {
            warn!("CentralRemote auth list_profiles error: {e}");
            return auth_failure("Erreur interne").into_response();
        }
    };

    let profile = profiles
        .into_iter()
        .find(|p| p.email.eq_ignore_ascii_case(email));

    let Some(profile) = profile else {
        // Email inconnu : travail Argon2 factice pour égaliser le temps de réponse.
        let mut dummy = IdentitySetup::new("remote-dummy");
        let _ = dummy.set_password(&req.password);
        return auth_failure("Identifiants incorrects").into_response();
    };

    // Seul chemin : Miyukini Connect (compteur d'échecs + verrouillage).
    // Aucun repli legacy : échec, verrouillage ou hash absent => refus.
    if !verify_connect_auth(&state, &profile.id, &req.password) {
        return auth_failure("Identifiants incorrects").into_response();
    }

    let display_name = profile
        .pseudonyme
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or(&profile.email)
        .to_string();

    let token = state.sessions.create_session(&profile.id, &display_name);

    Json(RemoteAuthResponse {
        success: true,
        token: Some(token),
        user_display_name: Some(display_name),
        error: None,
    })
    .into_response()
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

    let mut connect = match state.connect.lock() {
        Ok(c) => c,
        Err(_) => return false,
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

    match state.sessions.validate_token(&token) {
        Some(session) => {
            info!("CentralRemote WS upgrade for {}", session.display_name);
            ws.on_upgrade(move |socket| handle_ws_connection(socket, state, token))
        }
        None => {
            warn!("CentralRemote WS rejected: invalid token");
            // Refus avant l'upgrade.
            StatusCode::UNAUTHORIZED.into_response()
        }
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
async fn handle_bridge_pair(
    State(state): State<ServerState>,
    Json(req): Json<PairRequest>,
) -> Json<serde_json::Value> {
    let resp = state.pairing.request_pairing(&req.device_id, &req.device_name);
    Json(serde_json::json!({
        "status": resp.status,
        "token": resp.token,
        "message": resp.message,
    }))
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
        assert!(config.cors_allowed_origins.is_empty());
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
        state.pairing.request_pairing(full_id, "Pixel");
        let secret = state.admin_secret.as_str().to_string();
        let (st, body) = call(
            app_for(state.clone(), "127.0.0.1:5000"),
            admin_req("/api/bridge/devices", Some(&secret)),
        )
        .await;
        assert_eq!(st, StatusCode::OK);
        assert!(!body.contains(full_id), "device_id complet fuité: {body}");
        assert!(body.contains(&mask_device_id(full_id)));
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
        for _ in 0..(AUTH_RATE_MAX + 1) {
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
}
