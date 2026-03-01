//! Handler pour GET /state.

// @id: service.alicia.api.handlers.state
// @role: state_handler
// @layer: 7
// @human: Endpoint etat complet de la maison Alicia.
// @do: respond_home_state

use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::auth::{JwtAuth, JwtScope};
use crate::dto::{
    DeviceCapabilitiesDto, DeviceDto, DeviceStateDto, RoomDto, StateDto,
};
use crate::errors::ApiError;
use crate::router::AppState;

/// GET /api/v1/alicia/state
///
/// Retourne l'etat complet de la maison. JWT scope Read requis.
pub async fn get_state(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
) -> Result<Json<StateDto>, ApiError> {
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

    Ok(Json(StateDto {
        rooms,
        mqtt_connected: snapshot.mqtt_connected,
        total_devices: snapshot.total_devices,
        active_devices: snapshot.active_devices,
        timestamp: snapshot.timestamp,
    }))
}

/// Convertit un Device + DeviceState en DeviceDto (sans credentials).
pub fn device_to_dto(
    device: &miyualicia_devices::Device,
    state: &miyualicia_devices::DeviceState,
) -> DeviceDto {
    let color_rgb = state.color_rgb.map(|(r, g, b)| [r, g, b]);

    DeviceDto {
        id: device.id,
        room_id: device.room_id.clone(),
        device_type: format!("{:?}", device.device_type).to_lowercase(),
        name: device.name.clone(),
        protocol: format!("{}", device.protocol),
        address: device.address.clone(),
        capabilities: DeviceCapabilitiesDto {
            on_off: device.capabilities.on_off,
            dimmer: device.capabilities.dimmer,
            rgb: device.capabilities.rgb,
            position: device.capabilities.position,
            temperature_target: device.capabilities.temperature_target,
            power_measure: device.capabilities.power_measure,
        },
        state: DeviceStateDto {
            on: state.on,
            brightness: state.brightness,
            color_rgb,
            position: state.position,
            temperature_current: state.temperature_current,
            temperature_target: state.temperature_target,
            power_w: state.power_w,
            locked: state.locked,
            motion: state.motion,
            contact: state.contact,
            humidity: state.humidity,
            last_updated: state.updated_at,
            is_known: state.is_known(),
        },
        active: device.active,
    }
}
