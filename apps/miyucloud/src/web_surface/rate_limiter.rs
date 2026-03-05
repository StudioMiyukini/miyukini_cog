//! Rate limiter pour la surface web.
//!
//! @id: miyucloud_web_rate_limiter
//! @do: limit_request_rate_per_ip_on_share_routes
//! @role: security
//! @layer: app
//!
//! Implementation maison basee sur un compteur par IP avec fenetre glissante.
//! - 10 requetes/minute par IP pour `/share/*`
//! - 3 tentatives de mot de passe/minute par IP
//! - Retourne 429 Too Many Requests avec header Retry-After
//!
//! INVARIANT : Le rate limiting est par IP, pas par token.

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

/// Configuration du rate limiter.
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Nombre max de requetes par fenetre.
    pub max_requests: u32,
    /// Duree de la fenetre.
    pub window: Duration,
    /// Fait confiance aux en-tetes proxy pour extraire l'IP.
    pub trust_proxy: bool,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            max_requests: 10,
            window: Duration::from_secs(60),
            trust_proxy: false,
        }
    }
}

/// Etat du rate limiter partage entre les requetes.
#[derive(Clone)]
pub struct RateLimiterState {
    /// Compteurs par IP.
    counters: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    /// Compteur de requetes pour declencher des purges periodiques.
    requests_seen: Arc<Mutex<u64>>,
    /// Configuration.
    config: RateLimiterConfig,
}

impl RateLimiterState {
    /// Cree un nouveau rate limiter avec la configuration donnee.
    pub fn new(config: RateLimiterConfig) -> Self {
        Self {
            counters: Arc::new(Mutex::new(HashMap::new())),
            requests_seen: Arc::new(Mutex::new(0)),
            config,
        }
    }

    /// Verifie si une IP est autorisee a effectuer une requete.
    ///
    /// Retourne `Ok(())` si la requete est autorisee, `Err(seconds_to_wait)` sinon.
    pub fn check(&self, ip: &str) -> Result<(), u64> {
        let mut counters = self
            .counters
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut requests_seen = self
            .requests_seen
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let now = Instant::now();
        let window = self.config.window;
        *requests_seen += 1;

        let timestamps = counters.entry(ip.to_string()).or_default();

        // Remove expired entries
        timestamps.retain(|t| now.duration_since(*t) < window);

        if timestamps.len() >= self.config.max_requests as usize {
            // Find the oldest timestamp to compute Retry-After
            let oldest = timestamps.first().copied().unwrap_or(now);
            let retry_after = window
                .checked_sub(now.duration_since(oldest))
                .unwrap_or(Duration::from_secs(1));
            return Err(retry_after.as_secs().max(1));
        }

        timestamps.push(now);

        // Purge globale periodique pour eviter la croissance du HashMap.
        if *requests_seen % 100 == 0 {
            counters.retain(|_, ts| {
                ts.retain(|t| now.duration_since(*t) < window);
                !ts.is_empty()
            });
        }
        Ok(())
    }
}

/// Layer Tower pour le rate limiting.
#[derive(Clone)]
pub struct RateLimiterLayer {
    state: RateLimiterState,
}

impl RateLimiterLayer {
    /// Cree un nouveau layer avec la configuration donnee.
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

/// Middleware Tower pour le rate limiting.
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
        let ip = extract_ip(&req, self.state.config.trust_proxy);
        let state = self.state.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            match state.check(&ip) {
                Ok(()) => inner.call(req).await,
                Err(retry_after) => {
                    let retry_str = retry_after.to_string();
                    let body = serde_json::json!({
                        "error": {
                            "code": "RATE_LIMITED",
                            "message": "Too many requests. Please try again later.",
                            "details": null
                        }
                    });
                    let response = (
                        StatusCode::TOO_MANY_REQUESTS,
                        [("Retry-After", retry_str.as_str())],
                        axum::Json(body),
                    )
                        .into_response();
                    Ok(response)
                }
            }
        })
    }
}

/// Extrait l'adresse IP d'une requete.
///
/// Si `trust_proxy` est actif, lit `X-Forwarded-For` puis `X-Real-IP`.
/// Sinon retourne `unknown`.
fn extract_ip(req: &Request<Body>, trust_proxy: bool) -> String {
    crate::web_surface::access_log::extract_client_ip(req.headers(), trust_proxy)
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_allows_under_threshold() {
        let config = RateLimiterConfig {
            max_requests: 3,
            window: Duration::from_secs(60),
            trust_proxy: false,
        };
        let state = RateLimiterState::new(config);

        assert!(state.check("192.168.1.1").is_ok());
        assert!(state.check("192.168.1.1").is_ok());
        assert!(state.check("192.168.1.1").is_ok());
    }

    #[test]
    fn test_rate_limit_blocks_over_threshold() {
        let config = RateLimiterConfig {
            max_requests: 2,
            window: Duration::from_secs(60),
            trust_proxy: false,
        };
        let state = RateLimiterState::new(config);

        assert!(state.check("192.168.1.1").is_ok());
        assert!(state.check("192.168.1.1").is_ok());
        // Third request should be blocked
        let result = state.check("192.168.1.1");
        assert!(result.is_err());
    }

    #[test]
    fn test_rate_limit_different_ips_independent() {
        let config = RateLimiterConfig {
            max_requests: 1,
            window: Duration::from_secs(60),
            trust_proxy: false,
        };
        let state = RateLimiterState::new(config);

        assert!(state.check("192.168.1.1").is_ok());
        assert!(state.check("192.168.1.2").is_ok());

        // First IP is now blocked
        assert!(state.check("192.168.1.1").is_err());
        // Second IP is now blocked
        assert!(state.check("192.168.1.2").is_err());
        // Third IP is still ok
        assert!(state.check("192.168.1.3").is_ok());
    }

    #[test]
    fn test_rate_limit_returns_retry_after() {
        let config = RateLimiterConfig {
            max_requests: 1,
            window: Duration::from_secs(60),
            trust_proxy: false,
        };
        let state = RateLimiterState::new(config);

        state.check("10.0.0.1").unwrap();
        let retry_after = state.check("10.0.0.1").unwrap_err();
        assert!(retry_after > 0);
        assert!(retry_after <= 60);
    }

    #[test]
    fn test_extract_ip_trust_proxy_false_returns_unknown() {
        let req = Request::builder()
            .header("X-Forwarded-For", "203.0.113.10")
            .body(Body::empty())
            .unwrap();
        assert_eq!(extract_ip(&req, false), "unknown");
    }

    #[test]
    fn test_extract_ip_trust_proxy_true_reads_forwarded() {
        let req = Request::builder()
            .header("X-Forwarded-For", "203.0.113.10, 10.0.0.1")
            .body(Body::empty())
            .unwrap();
        assert_eq!(extract_ip(&req, true), "203.0.113.10");
    }
}
