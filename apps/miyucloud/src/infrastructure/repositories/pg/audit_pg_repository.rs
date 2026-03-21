//! PostgreSQL-backed audit log repository with batched async inserts.
//!
//! Events are sent to an mpsc channel and flushed in batches (every 100ms
//! or 50 events, whichever comes first) to avoid blocking the request path.

use crate::application::ports::audit_ports::AuditPort;
use crate::common::errors::DomainError;
use crate::domain::entities::audit_event::{
    AuditEvent, AuditFilter, AuditOutcome, AuditPagination,
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Batch configuration.
const BATCH_SIZE: usize = 50;
const FLUSH_INTERVAL_MS: u64 = 100;

/// Audit repository with background batch writer.
pub struct AuditPgRepository {
    tx: mpsc::UnboundedSender<AuditEvent>,
    pool: Arc<PgPool>,
}

impl AuditPgRepository {
    /// Create a new audit repository and spawn the background flush task.
    pub fn new(pool: Arc<PgPool>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let flush_pool = pool.clone();

        tokio::spawn(async move {
            Self::flush_loop(rx, flush_pool).await;
        });

        Self { tx, pool }
    }

    /// Background loop: collects events and flushes in batches.
    async fn flush_loop(mut rx: mpsc::UnboundedReceiver<AuditEvent>, pool: Arc<PgPool>) {
        let mut batch: Vec<AuditEvent> = Vec::with_capacity(BATCH_SIZE);

        loop {
            let timeout =
                tokio::time::sleep(std::time::Duration::from_millis(FLUSH_INTERVAL_MS));
            tokio::pin!(timeout);

            tokio::select! {
                Some(event) = rx.recv() => {
                    batch.push(event);
                    if batch.len() >= BATCH_SIZE {
                        Self::flush_batch(&pool, &mut batch).await;
                    }
                }
                _ = &mut timeout => {
                    if !batch.is_empty() {
                        Self::flush_batch(&pool, &mut batch).await;
                    }
                }
            }
        }
    }

    /// Insert a batch of events in a single transaction.
    async fn flush_batch(pool: &PgPool, batch: &mut Vec<AuditEvent>) {
        if batch.is_empty() {
            return;
        }

        let events: Vec<AuditEvent> = batch.drain(..).collect();
        let count = events.len();

        // Use a single transaction with multiple inserts for efficiency
        let result: Result<(), sqlx::Error> = async {
            let mut tx = pool.begin().await?;

            for event in &events {
                sqlx::query(
                    "INSERT INTO audit.events \
                     (id, timestamp, user_id, username, action, resource_type, \
                      resource_id, ip_address, user_agent, details, outcome) \
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8::inet, $9, $10, $11)"
                )
                .bind(event.id)
                .bind(event.timestamp)
                .bind(event.user_id)
                .bind(&event.username)
                .bind(event.action.to_string())
                .bind(event.resource_type.to_string())
                .bind(&event.resource_id)
                .bind(&event.ip_address)
                .bind(&event.user_agent)
                .bind(&event.details)
                .bind(event.outcome.to_string())
                .execute(&mut *tx)
                .await?;
            }

            tx.commit().await?;
            Ok(())
        }
        .await;

        match result {
            Ok(()) => {
                tracing::debug!("Flushed {} audit events to database", count);
            }
            Err(e) => {
                tracing::error!("Failed to flush {} audit events: {}", count, e);
                // Events are lost — in a production system you might want to
                // write them to a fallback file or retry queue.
            }
        }
    }
}

impl AuditPort for AuditPgRepository {
    fn record(&self, event: AuditEvent) {
        if let Err(e) = self.tx.send(event) {
            tracing::error!("Failed to enqueue audit event: {}", e);
        }
    }

    async fn query(
        &self,
        filter: AuditFilter,
        pagination: AuditPagination,
    ) -> Result<Vec<AuditEvent>, DomainError> {
        let rows = sqlx::query_as::<_, AuditRow>(
            "SELECT id, timestamp, user_id, username, action, resource_type, \
                    resource_id, ip_address::text, user_agent, details, outcome \
             FROM audit.events \
             WHERE ($1::uuid IS NULL OR user_id = $1) \
               AND ($2::text IS NULL OR action = $2) \
               AND ($3::text IS NULL OR resource_type = $3) \
               AND ($4::timestamptz IS NULL OR timestamp >= $4) \
               AND ($5::timestamptz IS NULL OR timestamp <= $5) \
               AND ($6::text IS NULL OR outcome = $6) \
             ORDER BY timestamp DESC \
             LIMIT $7 OFFSET $8"
        )
        .bind(filter.user_id)
        .bind(&filter.action)
        .bind(&filter.resource_type)
        .bind(filter.from)
        .bind(filter.to)
        .bind(&filter.outcome)
        .bind(pagination.limit)
        .bind(pagination.offset)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("AuditQuery", e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn count(&self, filter: AuditFilter) -> Result<i64, DomainError> {
        let row = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM audit.events \
             WHERE ($1::uuid IS NULL OR user_id = $1) \
               AND ($2::text IS NULL OR action = $2) \
               AND ($3::text IS NULL OR resource_type = $3) \
               AND ($4::timestamptz IS NULL OR timestamp >= $4) \
               AND ($5::timestamptz IS NULL OR timestamp <= $5) \
               AND ($6::text IS NULL OR outcome = $6)"
        )
        .bind(filter.user_id)
        .bind(&filter.action)
        .bind(&filter.resource_type)
        .bind(filter.from)
        .bind(filter.to)
        .bind(&filter.outcome)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| DomainError::internal_error("AuditCount", e.to_string()))?;

        Ok(row)
    }
}

/// Internal row type for mapping sqlx results.
#[derive(sqlx::FromRow)]
struct AuditRow {
    id: uuid::Uuid,
    timestamp: chrono::DateTime<chrono::Utc>,
    user_id: Option<uuid::Uuid>,
    username: Option<String>,
    action: String,
    resource_type: String,
    resource_id: Option<String>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    details: Option<serde_json::Value>,
    outcome: String,
}

impl From<AuditRow> for AuditEvent {
    fn from(row: AuditRow) -> Self {
        Self {
            id: row.id,
            timestamp: row.timestamp,
            user_id: row.user_id,
            username: row.username,
            action: serde_json::from_value(serde_json::Value::String(row.action.clone()))
                .unwrap_or(crate::domain::entities::audit_event::AuditAction::Login),
            resource_type: serde_json::from_value(serde_json::Value::String(
                row.resource_type.clone(),
            ))
            .unwrap_or(crate::domain::entities::audit_event::ResourceType::System),
            resource_id: row.resource_id,
            ip_address: row.ip_address,
            user_agent: row.user_agent,
            details: row.details,
            outcome: serde_json::from_value(serde_json::Value::String(row.outcome.clone()))
                .unwrap_or(AuditOutcome::Success),
        }
    }
}
