//! Handlers API pour la synchronisation P2P.
//!
//! @id: miyucloud_api_sync
//! @do: handle_sync_peer_management_status_conflict_resolution_endpoints
//! @role: api
//! @layer: app
//!
//! Endpoints REST pour la gestion des pairs de synchronisation,
//! le statut du moteur de sync, et la resolution des conflits.
//! Routes conformes a la spec P1 section 5.

use crate::api::auth::error_response;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use miyucloud::data::types::ConflictStatus;
use serde::Deserialize;
use std::sync::Arc;

// -----------------------------------------------------------------------
// Peers
// -----------------------------------------------------------------------

/// Liste tous les pairs de synchronisation.
pub async fn list_peers(State(state): State<Arc<AppState>>) -> Response {
    let db = &state.db;
    match db.sync_peer_list() {
        Ok(peers) => (StatusCode::OK, Json(peers)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}

/// Body pour l'enregistrement d'un pair.
#[derive(Deserialize)]
pub struct RegisterPeerBody {
    /// COG ID du pair.
    pub cog_id: String,
    /// Cle publique X25519 (base64).
    pub public_key: String,
    /// Nom affiche (optionnel).
    pub display_name: Option<String>,
}

/// Enregistre un nouveau pair (toujours non-fiable par defaut).
pub async fn register_peer(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterPeerBody>,
) -> Response {
    let db = &state.db;
    let now = chrono::Utc::now().to_rfc3339();

    let peer = miyucloud::data::types::SyncPeer {
        id: uuid::Uuid::new_v4().to_string(),
        cog_id: body.cog_id,
        display_name: body.display_name,
        public_key: body.public_key,
        last_seen_at: Some(now.clone()),
        last_sync_at: None,
        is_trusted: false,
        created_at: now,
    };

    match db.sync_peer_upsert(&peer) {
        Ok(saved) => (StatusCode::CREATED, Json(saved)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "REGISTER_PEER_ERROR",
            &e.to_string(),
        ),
    }
}

/// Approuve un pair (le rend fiable pour la synchronisation).
///
/// POST /api/sync/peers/{id}/trust
pub async fn trust_peer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Response {
    let db = &state.db;

    // Find peer by ID first to get its cog_id
    let peers = match db.sync_peer_list() {
        Ok(p) => p,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                &e.to_string(),
            );
        }
    };

    let peer = peers.iter().find(|p| p.id == id);
    let Some(peer) = peer else {
        return error_response(
            StatusCode::NOT_FOUND,
            "PEER_NOT_FOUND",
            "Le pair demande n'existe pas.",
        );
    };

    // Update peer as trusted
    let trusted_peer = miyucloud::data::types::SyncPeer {
        is_trusted: true,
        ..peer.clone()
    };

    match db.sync_peer_upsert(&trusted_peer) {
        Ok(saved) => (StatusCode::OK, Json(saved)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "TRUST_PEER_ERROR",
            &e.to_string(),
        ),
    }
}

/// Revoque la confiance d'un pair.
///
/// POST /api/sync/peers/{id}/untrust
pub async fn untrust_peer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Response {
    let db = &state.db;

    let peers = match db.sync_peer_list() {
        Ok(p) => p,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                &e.to_string(),
            );
        }
    };

    let peer = peers.iter().find(|p| p.id == id);
    let Some(peer) = peer else {
        return error_response(
            StatusCode::NOT_FOUND,
            "PEER_NOT_FOUND",
            "Le pair demande n'existe pas.",
        );
    };

    let untrusted_peer = miyucloud::data::types::SyncPeer {
        is_trusted: false,
        ..peer.clone()
    };

    match db.sync_peer_upsert(&untrusted_peer) {
        Ok(saved) => (StatusCode::OK, Json(saved)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "UNTRUST_PEER_ERROR",
            &e.to_string(),
        ),
    }
}

// -----------------------------------------------------------------------
// Sync Status
// -----------------------------------------------------------------------

/// Retourne le statut global de la synchronisation.
///
/// GET /api/sync/status
pub async fn sync_status(State(state): State<Arc<AppState>>) -> Response {
    let db = &state.db;

    // Build status from DB state (engine may not be running in tests)
    let peers = match db.sync_peer_list() {
        Ok(p) => p,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                &e.to_string(),
            );
        }
    };

    let conflicts = match db.sync_conflict_list_pending() {
        Ok(c) => c,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                &e.to_string(),
            );
        }
    };

    let status = miyucloud::data::types::SyncStatus {
        is_syncing: false, // Will be updated by engine if running
        peers_online: peers.iter().filter(|p| p.is_trusted).count() as u32,
        peers_total: peers.len() as u32,
        pending_uploads: 0,
        pending_downloads: 0,
        conflicts_pending: conflicts.len() as u32,
        last_sync_at: peers
            .iter()
            .filter_map(|p| p.last_sync_at.as_deref())
            .max()
            .map(String::from),
    };

    (StatusCode::OK, Json(status)).into_response()
}

// -----------------------------------------------------------------------
// Conflicts
// -----------------------------------------------------------------------

/// Liste les conflits de synchronisation en attente.
///
/// GET /api/sync/conflicts
pub async fn list_conflicts(State(state): State<Arc<AppState>>) -> Response {
    let db = &state.db;

    match db.sync_conflict_list_pending() {
        Ok(conflicts) => (StatusCode::OK, Json(conflicts)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}

/// Body pour la resolution d'un conflit.
#[derive(Deserialize)]
pub struct ResolveConflictBody {
    /// Resolution choisie : "local", "remote", "both".
    pub resolution: String,
}

/// Resout un conflit de synchronisation.
///
/// POST /api/sync/conflicts/{id}/resolve
pub async fn resolve_conflict(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<ResolveConflictBody>,
) -> Response {
    let db = &state.db;

    let status = match body.resolution.as_str() {
        "local" => ConflictStatus::ResolvedLocal,
        "remote" => ConflictStatus::ResolvedRemote,
        "both" => ConflictStatus::ResolvedBoth,
        other => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_RESOLUTION",
                &format!(
                    "Resolution invalide: '{other}'. Valeurs acceptees: local, remote, both."
                ),
            );
        }
    };

    match db.sync_conflict_resolve(&id, status) {
        Ok(()) => {
            let body = serde_json::json!({
                "resolved": true,
                "conflict_id": id,
                "resolution": body.resolution,
            });
            (StatusCode::OK, Json(body)).into_response()
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "RESOLVE_ERROR",
            &e.to_string(),
        ),
    }
}

/// Declenche un cycle de synchronisation manuel.
///
/// POST /api/sync/trigger
pub async fn trigger_sync(State(state): State<Arc<AppState>>) -> Response {
    let db = &state.db;
    let storage = &state.storage;
    let key_manager = &state.key_manager;

    // Create a temporary sync engine for one-shot sync
    let cog_id = state
        .config
        .owner_id
        .clone();
    let engine_config = miyucloud::sync::engine::SyncEngineConfig::new(
        cog_id,
        state.config.owner_id.clone(),
        String::new(), // No public key for one-shot
    );
    let engine = miyucloud::sync::engine::SyncEngine::new(engine_config);

    match engine.sync_once(db, storage, key_manager) {
        Ok(result) => {
            let body = serde_json::json!({
                "triggered": true,
                "peers_discovered": result.peers_discovered,
                "peers_synced": result.peers_synced,
                "peers_failed": result.peers_failed,
                "files_downloaded": result.files_downloaded,
                "files_uploaded": result.files_uploaded,
                "conflicts_detected": result.conflicts_detected,
            });
            (StatusCode::OK, Json(body)).into_response()
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "SYNC_ERROR",
            &e.to_string(),
        ),
    }
}
