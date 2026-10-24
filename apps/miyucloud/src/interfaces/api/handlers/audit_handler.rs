//! @id     miyucloud_interfaces_api_handlers_audit_handler
//! @do     definir_global_state
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « interfaces api handlers audit handler ».

//! Admin API handler for querying the persistent audit log.
//!
//! Requires admin role (extracteur `AdminUser`, après `auth_middleware`).
//! Provides filtering by user, action, date range, outcome.

use crate::interfaces::middleware::auth::AdminUser;
use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use std::sync::Arc;

use crate::common::di::AppState;
use crate::domain::entities::audit_event::{AuditFilter, AuditPagination};
use crate::interfaces::errors::AppError;

type GlobalState = Arc<AppState>;

pub struct AuditHandler;

impl AuditHandler {
    /// GET /api/admin/audit — query audit events with optional filters.
    ///
    /// Query params: user_id, action, resource_type, from, to, outcome, limit, offset
    pub async fn list_events(
        State(state): State<GlobalState>,
        _admin: AdminUser,
        Query(filter): Query<AuditFilter>,
        Query(pagination): Query<AuditPagination>,
    ) -> Result<impl IntoResponse, AppError> {
        let audit_service = state
            .audit_service
            .as_ref()
            .ok_or_else(|| AppError::internal_error("Audit service not available"))?;

        use crate::application::ports::audit_ports::AuditPort;

        let count = audit_service
            .count(filter.clone())
            .await
            .map_err(|e| AppError::internal_error(format!("Audit query failed: {}", e)))?;

        let events = audit_service
            .query(filter, pagination)
            .await
            .map_err(|e| AppError::internal_error(format!("Audit query failed: {}", e)))?;

        Ok(Json(serde_json::json!({
            "total": count,
            "events": events,
        })))
    }
}
