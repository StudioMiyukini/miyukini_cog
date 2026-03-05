//! Handler pour GET /health (non protege).

// @id: service.alicia.api.handlers.health
// @role: health_handler
// @layer: 7
// @human: Endpoint de sante Alicia API.
// @do: respond_health_status

use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::dto::HealthDto;
use crate::errors::ApiError;
use crate::router::AppState;

/// GET /api/v1/alicia/health
///
/// Retourne l'etat de sante du service. Pas de JWT requis.
pub async fn health(State(state): State<Arc<AppState>>) -> Result<Json<HealthDto>, ApiError> {
    let uptime = state.start_time.elapsed().as_secs();
    let mqtt_connected = state.alicia.is_mqtt_connected().await;
    let devices_count = state.alicia.device_count().await;

    Ok(Json(HealthDto {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        uptime_secs: uptime,
        mqtt_connected,
        devices_count,
    }))
}
