//! API JSON CalDAV -- liste et creation de calendriers et evenements.
//!
//! @id: miyucloud_caldav_json_api
//! @do: expose_caldav_data_as_json
//! @role: api
//! @layer: app
//!
//! Routes protegees par `X-COG-Token` (comme toutes les routes /api/*).
//! Les types de reponse JSON correspondent exactement aux types `CalendarEntry`
//! et `CalendarEvent` de `apps/central/src/services/miyucloud/state.rs`
//! pour que la deserialisation dans Central fonctionne sans adaptation.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

// ── Types de reponse JSON (compatibles avec CalendarEntry/CalendarEvent) ─────

/// Compatible avec `CalendarEntry` dans Central/state.rs.
/// Le champ `name` contient l'UUID du calendrier (identifiant stable).
#[derive(Serialize)]
pub struct CalendarJson {
    owner_id: String,
    name: String,
    display_name: String,
    color: Option<String>,
    ctag: String,
    created_at: String,
}

/// Compatible avec `CalendarEvent` dans Central/state.rs.
#[derive(Serialize)]
pub struct CalendarEventJson {
    uid: String,
    /// UUID du calendrier parent (utilise comme calendar_name dans Central).
    calendar_name: String,
    owner_id: String,
    summary: String,
    dtstart: String,
    dtend: Option<String>,
    location: Option<String>,
    description: Option<String>,
    etag: String,
    updated_at: String,
}

/// Corps de la requete de creation de calendrier.
#[derive(Deserialize)]
pub struct CreateCalendarRequest {
    pub display_name: String,
}

// ── Handlers ─────────────────────────────────────────────────────────────────

/// GET /api/calendars -- liste les calendriers du proprietaire.
pub async fn list_calendars(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CalendarJson>>, (StatusCode, String)> {
    let owner_id = state.config.owner_id.clone();
    let calendars = state
        .caldav_svc
        .list_calendars(&owner_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let json = calendars
        .into_iter()
        .map(|c| CalendarJson {
            owner_id: c.owner_id,
            name: c.id.to_string(),
            display_name: c.display_name,
            color: c.color,
            ctag: c.ctag,
            created_at: c.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(json))
}

/// POST /api/calendars -- cree un nouveau calendrier.
pub async fn create_calendar(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateCalendarRequest>,
) -> Result<(StatusCode, Json<CalendarJson>), (StatusCode, String)> {
    let owner_id = state.config.owner_id.clone();
    let cal = state
        .caldav_svc
        .create_calendar(&owner_id, &body.display_name)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let json = CalendarJson {
        owner_id: cal.owner_id,
        name: cal.id.to_string(),
        display_name: cal.display_name,
        color: cal.color,
        ctag: cal.ctag,
        created_at: cal.created_at.to_rfc3339(),
    };

    Ok((StatusCode::CREATED, Json(json)))
}

/// GET /api/calendars/{id}/events -- liste les evenements d'un calendrier.
pub async fn list_events(
    State(state): State<Arc<AppState>>,
    Path(calendar_id): Path<String>,
) -> Result<Json<Vec<CalendarEventJson>>, (StatusCode, String)> {
    let owner_id = state.config.owner_id.clone();
    let events = state
        .caldav_svc
        .list_events(&calendar_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let json = events
        .into_iter()
        .map(|ev| CalendarEventJson {
            uid: ev.uid,
            calendar_name: ev.calendar_id.to_string(),
            owner_id: owner_id.clone(),
            summary: ev.summary.unwrap_or_default(),
            dtstart: ev
                .dtstart
                .map_or_else(String::new, |dt| dt.to_rfc3339()),
            dtend: ev.dtend.map(|dt| dt.to_rfc3339()),
            location: None,
            description: None,
            etag: ev.etag,
            updated_at: ev.updated_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(json))
}
