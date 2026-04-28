//! Serveur HTTP + WebSocket embarqué pour CentralRemote.
//!
//! @id: central_remote_server @do: run_embedded_remote_server
//! @role: infra @layer: toolkit
//! @human: Serveur Axum lancé par Central quand le remote est activé.
//! Expose : POST /auth (login), GET /ws (WebSocket), GET /status (santé).

use crate::bridge::RemoteBridge;
use crate::pairing::{PairRequest, PairingManager, WhitelistValidateRequest};
use crate::protocol::{RemoteAuthRequest, RemoteAuthResponse, RemoteCommand, RemoteEvent};
use crate::session::SessionManager;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_util::{SinkExt, StreamExt};
use miyukini_central::auth::CentralAuthDb;
use miyukini_connect::{AuthVerifyRequest, ConnectError, ConnectService, IdentitySetup, PermissionTier};
use serde::Deserialize;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};

/// Configuration du serveur CentralRemote.
#[derive(Debug, Clone)]
pub struct RemoteServerConfig {
    /// Adresse d'écoute (défaut : 127.0.0.1:8091).
    pub listen_addr: String,
    /// Chemin vers la base auth Central.
    pub auth_db_path: String,
}

impl Default for RemoteServerConfig {
    fn default() -> Self {
        Self {
            // 0.0.0.0 pour accepter les connexions LAN depuis les clients mobiles
            listen_addr: "0.0.0.0:8091".into(),
            auth_db_path: "central.db".into(),
        }
    }
}

/// État partagé du serveur remote.
#[derive(Clone)]
struct ServerState {
    bridge: RemoteBridge,
    sessions: Arc<SessionManager>,
    auth_db: Arc<CentralAuthDb>,
    connect: Arc<Mutex<ConnectService>>,
    pairing: Arc<PairingManager>,
}

/// Lance le serveur CentralRemote. Retourne un handle pour l'arrêter.
/// Cette fonction est non-bloquante et spawn le serveur en background.
/// Lance également le listener UDP de découverte LAN pour les clients mobiles.
pub async fn start_remote_server(
    config: RemoteServerConfig,
    bridge: RemoteBridge,
) -> Result<RemoteServerHandle, String> {
    let auth_db = CentralAuthDb::open(Path::new(&config.auth_db_path).to_path_buf())
        .map_err(|e| format!("CentralRemote auth DB: {e}"))?;

    let connect = ConnectService::new("2026.03.05");
    let sessions = SessionManager::with_random_secret();
    let pairing = Arc::new(PairingManager::new());

    // Lancer le listener UDP de découverte LAN (best effort — si ça échoue, on continue)
    let discovery_handle = match crate::discovery::start_discovery_listener(pairing.clone()).await {
        Ok(h) => {
            info!("Découverte LAN activée (UDP port 19847)");
            Some(h)
        }
        Err(e) => {
            warn!("Découverte LAN non activée: {e}");
            None
        }
    };

    let pairing_for_handle = pairing.clone();
    let state = ServerState {
        bridge,
        sessions: Arc::new(sessions),
        auth_db: Arc::new(auth_db),
        connect: Arc::new(Mutex::new(connect)),
        pairing,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/auth", post(handle_auth))
        .route("/ws", get(handle_ws_upgrade))
        .route("/status", get(handle_status))
        // Endpoints d'appairage mobile (COG Bridge)
        .route("/api/bridge/validate", post(handle_bridge_validate))
        .route("/api/bridge/validate-token", get(handle_bridge_validate_token))
        .route("/api/bridge/pair", post(handle_bridge_pair))
        .route("/api/bridge/generate-token", post(handle_bridge_generate_token))
        .route("/api/bridge/discovery", get(handle_bridge_discovery))
        .route("/api/bridge/devices", get(handle_bridge_devices))
        .route("/api/bridge/approve", post(handle_bridge_approve))
        .route("/api/bridge/reject", post(handle_bridge_reject))
        .with_state(state)
        .layer(cors);

    let listener = TcpListener::bind(&config.listen_addr)
        .await
        .map_err(|e| format!("CentralRemote bind {}: {e}", config.listen_addr))?;

    let local_addr = listener
        .local_addr()
        .map_err(|e| format!("CentralRemote local_addr: {e}"))?;

    info!("CentralRemote server listening on {local_addr}");

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    tokio::spawn(async move {
        axum::serve(listener, app)
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
        pairing: pairing_for_handle,
        _discovery: discovery_handle,
    })
}

/// Handle pour contrôler le serveur remote.
pub struct RemoteServerHandle {
    /// Adresse effective d'écoute.
    pub addr: SocketAddr,
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
    /// Gestionnaire de l'appairage (exposé pour l'UI Central Desktop).
    pub pairing: Arc<PairingManager>,
    /// Handle du listener UDP (gardé pour qu'il reste actif).
    _discovery: Option<crate::discovery::DiscoveryHandle>,
}

impl RemoteServerHandle {
    /// Arrête le serveur proprement.
    pub fn shutdown(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }

    /// Adresse du serveur sous forme de string.
    pub fn addr_string(&self) -> String {
        self.addr.to_string()
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

/// POST /auth — authentification et obtention d'un token.
async fn handle_auth(
    State(state): State<ServerState>,
    Json(req): Json<RemoteAuthRequest>,
) -> Json<RemoteAuthResponse> {
    let email = req.email.trim();
    if email.is_empty() || req.password.is_empty() {
        return Json(RemoteAuthResponse {
            success: false,
            token: None,
            user_display_name: None,
            error: Some("Email et mot de passe requis".into()),
        });
    }

    // Recherche du profil par email.
    let profiles = match state.auth_db.list_profiles() {
        Ok(p) => p,
        Err(e) => {
            warn!("CentralRemote auth list_profiles error: {e}");
            return Json(RemoteAuthResponse {
                success: false,
                token: None,
                user_display_name: None,
                error: Some("Erreur interne".into()),
            });
        }
    };

    let profile = profiles
        .into_iter()
        .find(|p| p.email.eq_ignore_ascii_case(email));

    let Some(profile) = profile else {
        return Json(RemoteAuthResponse {
            success: false,
            token: None,
            user_display_name: None,
            error: Some("Identifiants incorrects".into()),
        });
    };

    // Vérification via Miyukini Connect (même logique que Central).
    let verified = verify_connect_auth(&state, &profile.id, &req.password);

    if !verified {
        // Fallback : vérification legacy via sign_in.
        match state.auth_db.sign_in(email, &req.password) {
            Ok(Some(_)) => {}
            _ => {
                return Json(RemoteAuthResponse {
                    success: false,
                    token: None,
                    user_display_name: None,
                    error: Some("Identifiants incorrects".into()),
                });
            }
        }
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
}

fn verify_connect_auth(state: &ServerState, profile_id: &str, password: &str) -> bool {
    let stored_hash = match state.auth_db.get_connect_password_hash(profile_id) {
        Ok(Some(h)) => h,
        _ => return false,
    };

    let mut connect = match state.connect.lock() {
        Ok(c) => c,
        Err(_) => return false,
    };

    let mut identity = IdentitySetup::new(profile_id.to_string());
    if identity.set_password_hash_for_import(stored_hash).is_err() {
        return false;
    }
    if connect.register_identity(identity).is_err() {
        return false;
    }

    let runtime_state = connect.current_runtime_state();
    let request = AuthVerifyRequest {
        subject_id: profile_id.to_string(),
        password: password.to_string(),
        totp_code: None,
        requested_tier: PermissionTier::Basic,
        runtime_state,
    };

    matches!(connect.auth_verify(request), Ok(_))
}

/// Paramètres de connexion WebSocket.
#[derive(Deserialize)]
struct WsParams {
    token: String,
}

/// GET /ws?token=... — upgrade WebSocket.
async fn handle_ws_upgrade(
    State(state): State<ServerState>,
    Query(params): Query<WsParams>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let session = state.sessions.validate_token(&params.token);
    match session {
        Some(session) => {
            info!(
                "CentralRemote WS upgrade for {}",
                session.display_name
            );
            ws.on_upgrade(move |socket| handle_ws_connection(socket, state))
        }
        None => {
            warn!("CentralRemote WS rejected: invalid token");
            // Axum ne permet pas facilement de refuser un upgrade avec un code custom,
            // on accepte puis on ferme immédiatement.
            ws.on_upgrade(|mut socket| async move {
                let _ = socket
                    .send(Message::Text(
                        serde_json::to_string(&RemoteEvent::Error {
                            message: "Token invalide ou expiré".into(),
                        })
                        .unwrap_or_default()
                        .into(),
                    ))
                    .await;
                let _ = socket.close().await;
            })
        }
    }
}

/// Gère une connexion WebSocket authentifiée.
async fn handle_ws_connection(socket: WebSocket, state: ServerState) {
    let (mut ws_tx, mut ws_rx) = socket.split();

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
    let token = state.pairing.generate_api_token(device_name);
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

/// GET /api/bridge/devices — liste les devices appairés et en attente.
async fn handle_bridge_devices(
    State(state): State<ServerState>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "paired": state.pairing.paired_devices(),
        "pending": state.pairing.pending_devices(),
    }))
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

    #[test]
    fn default_config() {
        let config = RemoteServerConfig::default();
        // 0.0.0.0 pour accepter les connexions LAN depuis les clients mobiles
        assert_eq!(config.listen_addr, "0.0.0.0:8091");
    }
}
