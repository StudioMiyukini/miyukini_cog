//! Audit logging port — hexagonal boundary for persistent security audit trail.
//!
//! Implementations live in `infrastructure/repositories/pg/audit_pg_repository.rs`.

use crate::common::errors::DomainError;
use crate::domain::entities::audit_event::{AuditEvent, AuditFilter, AuditPagination};

/// Port for recording and querying audit events.
///
/// The implementation should batch inserts asynchronously to avoid
/// blocking the request path (e.g. via an mpsc channel with periodic flush).
pub trait AuditPort: Send + Sync + 'static {
    /// Record a single audit event. Should not block the caller.
    fn record(&self, event: AuditEvent);

    /// Query audit events with filtering and pagination.
    /// Used by the admin audit API.
    fn query(
        &self,
        filter: AuditFilter,
        pagination: AuditPagination,
    ) -> impl std::future::Future<Output = Result<Vec<AuditEvent>, DomainError>> + Send;

    /// Count audit events matching a filter (for pagination metadata).
    fn count(
        &self,
        filter: AuditFilter,
    ) -> impl std::future::Future<Output = Result<i64, DomainError>> + Send;
}
