//! Per-user bandwidth throttling middleware.
//!
//! Tracks bytes transferred per authenticated user within a sliding window.
//! Returns `429 Too Many Requests` when the configured limit is exceeded.
//! Disabled when `bandwidth_limit_mb_per_min` is 0 (the default).

use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use moka::sync::Cache;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use crate::common::di::AppState;

/// Per-user bandwidth counter with atomic byte tracking.
pub struct BandwidthCounter {
    bytes: AtomicU64,
}

impl BandwidthCounter {
    fn new() -> Self {
        Self {
            bytes: AtomicU64::new(0),
        }
    }

    fn add(&self, n: u64) -> u64 {
        self.bytes.fetch_add(n, Ordering::Relaxed) + n
    }

    fn get(&self) -> u64 {
        self.bytes.load(Ordering::Relaxed)
    }
}

/// Shared bandwidth tracker state.
pub struct BandwidthTracker {
    /// Per-user byte counters that expire after the window.
    counters: Cache<uuid::Uuid, Arc<BandwidthCounter>>,
    /// Limit in bytes per window.
    limit_bytes: u64,
}

impl BandwidthTracker {
    pub fn new(limit_mb_per_min: u64, window_secs: u64) -> Self {
        Self {
            counters: Cache::builder()
                .max_capacity(100_000)
                .time_to_live(Duration::from_secs(window_secs))
                .build(),
            limit_bytes: limit_mb_per_min * 1024 * 1024,
        }
    }

    /// Record bytes for a user. Returns true if still within limit.
    pub fn record(&self, user_id: uuid::Uuid, bytes: u64) -> bool {
        let counter = self
            .counters
            .get_with(user_id, || Arc::new(BandwidthCounter::new()));
        counter.add(bytes) <= self.limit_bytes
    }

    /// Check if user is already over limit (without adding bytes).
    pub fn is_over_limit(&self, user_id: uuid::Uuid) -> bool {
        self.counters
            .get(&user_id)
            .map(|c| c.get() > self.limit_bytes)
            .unwrap_or(false)
    }
}

/// Axum middleware that enforces per-user bandwidth limits.
///
/// Must be applied AFTER auth middleware so that `AuthUser` is available
/// in request extensions.
pub async fn bandwidth_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    let limit = state.config.security.bandwidth_limit_mb_per_min;

    // Skip if bandwidth limiting is disabled
    if limit == 0 {
        return next.run(request).await;
    }

    // Extract authenticated user (if present)
    let user_id = request
        .extensions()
        .get::<crate::interfaces::middleware::auth::AuthUser>()
        .map(|u| u.id);

    // Only throttle authenticated users
    let Some(user_id) = user_id else {
        return next.run(request).await;
    };

    // Get or create the tracker (lazy init via AppState extension)
    let tracker = state
        .bandwidth_tracker
        .get_or_init(|| Arc::new(BandwidthTracker::new(limit, 60)));

    // Pre-check: if already over limit, reject immediately
    if tracker.is_over_limit(user_id) {
        tracing::warn!(
            "Bandwidth limit exceeded for user {} ({} MB/min)",
            user_id,
            limit
        );
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [("retry-after", "60")],
            "Bandwidth limit exceeded",
        )
            .into_response();
    }

    // Estimate request size from Content-Length
    let request_size = request
        .headers()
        .get(axum::http::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    // Record incoming bytes
    if request_size > 0 && !tracker.record(user_id, request_size) {
        tracing::warn!(
            "Bandwidth limit exceeded for user {} (request: {} bytes, limit: {} MB/min)",
            user_id,
            request_size,
            limit
        );
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [("retry-after", "60")],
            "Bandwidth limit exceeded",
        )
            .into_response();
    }

    next.run(request).await
}
