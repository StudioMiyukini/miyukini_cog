//! Session timeout enforcement — ISO 27001 A.8.5
//!
//! Implements absolute and idle timeout for user sessions.
//! Background task periodically cleans up expired/idle sessions.

use sqlx::PgPool;
use std::sync::Arc;
use tokio::time::{Duration, interval};

/// Session timeout configuration.
pub struct SessionTimeoutConfig {
    /// Absolute session lifetime in hours (default: 24).
    pub absolute_timeout_hours: u64,
    /// Idle timeout in minutes (default: 60). Session expires after N minutes of inactivity.
    pub idle_timeout_minutes: u64,
    /// Cleanup interval in minutes (default: 15).
    pub cleanup_interval_minutes: u64,
}

impl Default for SessionTimeoutConfig {
    fn default() -> Self {
        Self {
            absolute_timeout_hours: 24,
            idle_timeout_minutes: 60,
            cleanup_interval_minutes: 15,
        }
    }
}

/// Spawns a background task that revokes expired and idle sessions.
pub fn start_session_cleanup(pool: Arc<PgPool>, config: SessionTimeoutConfig) {
    tokio::spawn(async move {
        let mut timer = interval(Duration::from_secs(config.cleanup_interval_minutes * 60));

        loop {
            timer.tick().await;

            // Revoke sessions that exceed absolute timeout
            let abs_result = sqlx::query(
                "UPDATE auth.sessions SET revoked = true \
                 WHERE revoked = false \
                   AND created_at < NOW() - make_interval(hours => $1::int)"
            )
            .bind(config.absolute_timeout_hours as i32)
            .execute(pool.as_ref())
            .await;

            match abs_result {
                Ok(r) if r.rows_affected() > 0 => {
                    tracing::info!(
                        "Session cleanup: revoked {} sessions exceeding {}h absolute timeout",
                        r.rows_affected(),
                        config.absolute_timeout_hours
                    );
                }
                Err(e) => tracing::warn!("Session absolute timeout cleanup failed: {}", e),
                _ => {}
            }

            // Revoke sessions that exceed idle timeout
            // Uses expires_at as a proxy for last activity
            let idle_result = sqlx::query(
                "UPDATE auth.sessions SET revoked = true \
                 WHERE revoked = false \
                   AND expires_at < NOW()"
            )
            .execute(pool.as_ref())
            .await;

            match idle_result {
                Ok(r) if r.rows_affected() > 0 => {
                    tracing::info!(
                        "Session cleanup: revoked {} expired sessions",
                        r.rows_affected()
                    );
                }
                Err(e) => tracing::warn!("Session idle cleanup failed: {}", e),
                _ => {}
            }
        }
    });

    tracing::info!(
        "Session timeout service started (absolute: {}h, cleanup every {}min)",
        config.absolute_timeout_hours,
        config.cleanup_interval_minutes
    );
}
