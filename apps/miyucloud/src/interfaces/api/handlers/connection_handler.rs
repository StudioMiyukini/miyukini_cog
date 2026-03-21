//! Admin API handler for web connection management.
//!
//! Provides endpoints to view status and control MWS tunnel and DDNS modes.

use axum::{
    Json,
    extract::State,
    http::HeaderMap,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::common::di::AppState;
use crate::interfaces::errors::AppError;

type GlobalState = Arc<AppState>;

pub struct ConnectionHandler;

impl ConnectionHandler {
    /// GET /api/admin/connections/status
    pub async fn status(
        State(state): State<GlobalState>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        super::admin_handler::admin_guard_public(&state, &headers).await?;

        let mgr = state
            .connection_manager
            .as_ref()
            .ok_or_else(|| AppError::internal_error("Connection manager not available"))?;

        let status = mgr
            .status()
            .await
            .map_err(|e| AppError::internal_error(format!("Status query failed: {}", e)))?;

        Ok(Json(status))
    }

    /// POST /api/admin/connections/mws/connect
    pub async fn mws_connect(
        State(state): State<GlobalState>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        super::admin_handler::admin_guard_public(&state, &headers).await?;

        let mgr = state
            .connection_manager
            .as_ref()
            .ok_or_else(|| AppError::internal_error("Connection manager not available"))?;

        let mws = mgr
            .mws
            .as_ref()
            .ok_or_else(|| AppError::internal_error("MWS tunnel not configured"))?;

        mws.connect()
            .await
            .map_err(|e| AppError::internal_error(format!("MWS connect failed: {}", e)))?;

        Ok(Json(serde_json::json!({"status": "connected"})))
    }

    /// POST /api/admin/connections/mws/disconnect
    pub async fn mws_disconnect(
        State(state): State<GlobalState>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        super::admin_handler::admin_guard_public(&state, &headers).await?;

        let mgr = state
            .connection_manager
            .as_ref()
            .ok_or_else(|| AppError::internal_error("Connection manager not available"))?;

        let mws = mgr
            .mws
            .as_ref()
            .ok_or_else(|| AppError::internal_error("MWS tunnel not configured"))?;

        mws.disconnect()
            .await
            .map_err(|e| AppError::internal_error(format!("MWS disconnect failed: {}", e)))?;

        Ok(Json(serde_json::json!({"status": "disconnected"})))
    }

    /// POST /api/admin/connections/ddns/update
    pub async fn ddns_update(
        State(state): State<GlobalState>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        super::admin_handler::admin_guard_public(&state, &headers).await?;

        let mgr = state
            .connection_manager
            .as_ref()
            .ok_or_else(|| AppError::internal_error("Connection manager not available"))?;

        let ddns = mgr
            .ddns
            .as_ref()
            .ok_or_else(|| AppError::internal_error("DDNS not configured"))?;

        let ip = ddns
            .force_update()
            .await
            .map_err(|e| AppError::internal_error(format!("DDNS update failed: {}", e)))?;

        Ok(Json(serde_json::json!({"status": "updated", "ip": ip})))
    }

    /// POST /api/admin/connections/ddns/start
    pub async fn ddns_start(
        State(state): State<GlobalState>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        super::admin_handler::admin_guard_public(&state, &headers).await?;

        let mgr = state
            .connection_manager
            .as_ref()
            .ok_or_else(|| AppError::internal_error("Connection manager not available"))?;

        let ddns = mgr
            .ddns
            .as_ref()
            .ok_or_else(|| AppError::internal_error("DDNS not configured"))?;

        ddns.start()
            .await
            .map_err(|e| AppError::internal_error(format!("DDNS start failed: {}", e)))?;

        Ok(Json(serde_json::json!({"status": "started"})))
    }

    /// POST /api/admin/connections/ddns/stop
    pub async fn ddns_stop(
        State(state): State<GlobalState>,
        headers: HeaderMap,
    ) -> Result<impl IntoResponse, AppError> {
        super::admin_handler::admin_guard_public(&state, &headers).await?;

        let mgr = state
            .connection_manager
            .as_ref()
            .ok_or_else(|| AppError::internal_error("Connection manager not available"))?;

        let ddns = mgr
            .ddns
            .as_ref()
            .ok_or_else(|| AppError::internal_error("DDNS not configured"))?;

        ddns.stop().await;

        Ok(Json(serde_json::json!({"status": "stopped"})))
    }
}
