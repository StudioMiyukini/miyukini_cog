//! Handlers pour les routes /devices.

// @id: service.alicia.api.handlers.devices
// @role: device_command_handler
// @layer: 7
// @human: Endpoints dispositifs et commandes Alicia API.
// @do: handle_device_requests_and_commands

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use miyualicia_devices::{CommandSource, DeviceCommand};

use crate::auth::{JwtAuth, JwtScope};
use crate::dto::{CommandRequest, CommandResponse, DeviceDto};
use crate::errors::ApiError;
use crate::handlers::state::device_to_dto;
use crate::router::AppState;

/// GET /api/v1/alicia/devices
///
/// Retourne tous les dispositifs. JWT scope Read requis.
pub async fn list(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<DeviceDto>>, ApiError> {
    claims.require_scope(&JwtScope::Read)?;

    let devices = state.alicia.registry_snapshot().await;
    let dtos: Vec<DeviceDto> = devices.iter().map(|(d, s)| device_to_dto(d, s)).collect();

    Ok(Json(dtos))
}

/// GET /api/v1/alicia/devices/{id}
///
/// Retourne un dispositif specifique. JWT scope Read requis.
pub async fn get(
    JwtAuth(claims): JwtAuth,
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<DeviceDto>, ApiError> {
    claims.require_scope(&JwtScope::Read)?;

    let (device, device_state) = state
        .alicia
        .get_device(id)
        .await
        .map_err(|_| ApiError::NotFound(format!("dispositif {id}")))?;

    Ok(Json(device_to_dto(&device, &device_state)))
}

/// POST /api/v1/alicia/devices/{id}/command
///
/// Envoie une commande a un dispositif. JWT scope Write requis.
pub async fn command(
    JwtAuth(claims): JwtAuth,
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CommandRequest>,
) -> Result<Json<CommandResponse>, ApiError> {
    claims.require_scope(&JwtScope::Write)?;

    // Verifier que le dispositif existe
    let _ = state
        .alicia
        .get_device(id)
        .await
        .map_err(|_| ApiError::NotFound(format!("dispositif {id}")))?;

    let cmd = match req.value {
        Some(v) => DeviceCommand::with_value(id, req.action.clone(), v, CommandSource::Api),
        None => DeviceCommand::simple(id, req.action.clone(), CommandSource::Api),
    };

    match state.alicia.execute_command(cmd).await {
        Ok(latency_ms) => Ok(Json(CommandResponse {
            success: true,
            device_id: id,
            action: req.action,
            latency_ms,
            message: None,
        })),
        Err(e) => Ok(Json(CommandResponse {
            success: false,
            device_id: id,
            action: req.action,
            latency_ms: 0,
            message: Some(e.to_string()),
        })),
    }
}
