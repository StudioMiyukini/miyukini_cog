//! Rate limiter IP-based pour le COG Web Portal.
//!
//! @id: cog_web_portal_rate_limiter @do: limit_request_rate_per_ip
//! @role: security @layer: app
//! @human: Rate limiter Portal — fenêtre glissante par IP, 60 req/min, retourne 429 avec Retry-After.
//!
//! Adapté de apps/miyucloud/src/web_surface/rate_limiter.rs.

#![allow(clippy::redundant_closure_for_method_calls)]

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tower::{Layer, Service};

/// Configuration du rate limiter Portal.
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Nombre max de requêtes par fenêtre.
    pub max_requests: u32,
    /// Durée de la fenêtre.
    pub window: Duration,
    /// Fait confiance aux en-têtes proxy pour extraire l'IP.
    pub trust_proxy: bool,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            max_requests: 60,
            window: Duration::from_secs(60),
            trust_proxy: false,
        }
    }
}

/// État partagé du rate limiter.
#[derive(Clone)]
pub struct RateLimiterState {
    counters: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    requests_seen: Arc<Mutex<u64>>,
    config: RateLimiterConfig,
}

impl RateLimiterState {
    pub fn new(config: RateLimiterConfig) -> Self {
        Self {
            counters: Arc::new(Mutex::new(HashMap::new())),
            requests_seen: Arc::new(Mutex::new(0)),
            config,
        }
    }

    /// Retourne `true` si la requête est autorisée, `false` si le quota est dépassé.
    pub fn check_and_record(&self, ip: &str) -> bool {
        let now = Instant::now();
        let window = self.config.window;
        let max = self.config.max_requests as usize;

        let Ok(mut counters) = self.counters.lock() else {
            return true; // fail open
        };

        // Purge périodique toutes les 1000 requêtes.
        {
            let mut seen = self.requests_seen.lock().unwrap_or_else(|e| e.into_inner());
            *seen += 1;
            if seen.is_multiple_of(1000) {
                counters.retain(|_, timestamps| {
                    timestamps.retain(|t| now.duration_since(*t) < window);
                    !timestamps.is_empty()
                });
            }
        }

        let timestamps = counters.entry(ip.to_string()).or_default();
        timestamps.retain(|t| now.duration_since(*t) < window);

        if timestamps.len() >= max {
            return false;
        }
        timestamps.push(now);
        true
    }
}

/// Layer de rate limiting.
#[derive(Clone)]
pub struct RateLimiterLayer {
    state: RateLimiterState,
}

impl RateLimiterLayer {
    pub fn new(config: RateLimiterConfig) -> Self {
        Self {
            state: RateLimiterState::new(config),
        }
    }
}

impl<S> Layer<S> for RateLimiterLayer {
    type Service = RateLimiterMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimiterMiddleware {
            inner,
            state: self.state.clone(),
        }
    }
}

/// Middleware de rate limiting.
#[derive(Clone)]
pub struct RateLimiterMiddleware<S> {
    inner: S,
    state: RateLimiterState,
}

impl<S> Service<Request<Body>> for RateLimiterMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        let state = self.state.clone();

        let ip = extract_ip(&req, state.config.trust_proxy);
        let allowed = state.check_and_record(&ip);

        Box::pin(async move {
            if !allowed {
                let body = "Trop de requêtes. Veuillez réessayer dans un moment.";
                let response = (
                    StatusCode::TOO_MANY_REQUESTS,
                    [("Retry-After", "60")],
                    body,
                )
                    .into_response();
                return Ok(response);
            }
            inner.call(req).await
        })
    }
}

fn extract_ip(req: &Request<Body>, trust_proxy: bool) -> String {
    if trust_proxy {
        if let Some(forwarded) = req
            .headers()
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
        {
            if let Some(first) = forwarded.split(',').next() {
                return first.trim().to_string();
            }
        }
    }
    // Fallback — no real socket addr in axum without ConnectInfo; use placeholder.
    "unknown".to_string()
}
