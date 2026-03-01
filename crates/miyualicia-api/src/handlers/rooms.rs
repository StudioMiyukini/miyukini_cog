//! Handlers pour les routes /rooms.

// @id: service.alicia.api.handlers.rooms
// @role: rooms_handler
// @layer: 7
// @human: Endpoints pieces Alicia API.
// @do: handle_rooms_requests

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;

use crate::auth::{JwtAuth, JwtScope};
use crate::dto::{DeviceDto, RoomDto};
use crate::errors::ApiError;
use crate::handlers::state::device_to_dto;
use crate::router::AppState;

/// GET /api/v1/alicia/rooms
///
/// Retourne la liste des pieces avec leurs dispositifs. JWT scope Read requis.
pub async fn list(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<RoomDto>>, ApiError> {
    claims.require_scope(&JwtScope::Read)?;

    let snapshot = state.alicia.snapshot().await;
    let rooms: Vec<RoomDto> = snapshot
        .rooms
        .iter()
        .map(|room| {
            let devices: Vec<DeviceDto> = room
                .devices
                .iter()
                .map(|ds| device_to_dto(&ds.device, &ds.state))
                .collect();
            RoomDto {
                id: room.id.clone(),
                name: room.name.clone(),
                devices,
            }
        })
        .collect();

    Ok(Json(rooms))
}

/// GET /api/v1/alicia/rooms/{room_id}
///
/// Retourne une piece specifique. JWT scope Read requis.
pub async fn get(
    JwtAuth(claims): JwtAuth,
    Path(room_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<RoomDto>, ApiError> {
    claims.require_scope(&JwtScope::Read)?;

    let snapshot = state.alicia.snapshot().await;
    let room = snapshot
        .rooms
        .iter()
        .find(|r| r.id == room_id)
        .ok_or_else(|| ApiError::NotFound(format!("piece '{room_id}'")))?;

    let devices: Vec<DeviceDto> = room
        .devices
        .iter()
        .map(|ds| device_to_dto(&ds.device, &ds.state))
        .collect();

    Ok(Json(RoomDto {
        id: room.id.clone(),
        name: room.name.clone(),
        devices,
    }))
}

/// GET /api/v1/alicia/rooms/{room_id}/devices
///
/// Retourne les dispositifs d'une piece. JWT scope Read requis.
pub async fn devices(
    JwtAuth(claims): JwtAuth,
    Path(room_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<DeviceDto>>, ApiError> {
    claims.require_scope(&JwtScope::Read)?;

    let snapshot = state.alicia.snapshot().await;
    let room = snapshot
        .rooms
        .iter()
        .find(|r| r.id == room_id)
        .ok_or_else(|| ApiError::NotFound(format!("piece '{room_id}'")))?;

    let devices: Vec<DeviceDto> = room
        .devices
        .iter()
        .map(|ds| device_to_dto(&ds.device, &ds.state))
        .collect();

    Ok(Json(devices))
}
