//! @id     miyucloud_interfaces_middleware_rate_limit
//! @do     definir_trust_proxy
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « interfaces middleware rate limit ».

//! IP-based rate limiting middleware for authentication endpoints.
//!
//! Uses `moka` TTL caches (already a project dependency) to track request
//! counts per client IP.  Each protected endpoint group gets its own
//! [`RateLimiter`] instance with independently tuneable limits.
//!
//! M-06 : la clé est l'adresse IP du pair TCP (`ConnectInfo<SocketAddr>`,
//! fourni par `into_make_service_with_connect_info`). Les en-têtes
//! `X-Forwarded-For` / `X-Real-Ip` ne sont lus que si le pair est un proxy
//! de confiance déclaré (`MIYUCLOUD_TRUSTED_PROXIES`, liste d'IP ou de CIDR ;
//! `MIYUCLOUD_TRUST_PROXY_HEADERS=true` seul = proxy local 127.0.0.1/::1).
//! Le client retenu est alors l'entrée **la plus à droite** de
//! `X-Forwarded-For` qui n'est pas un proxy de confiance (les entrées de
//! gauche sont forgeables).
//!
//! When the limit is exceeded a `429 Too Many Requests` response is returned
//! with a `Retry-After` header indicating how many seconds to wait.

use axum::{
    extract::ConnectInfo,
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use moka::sync::Cache;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

/// Proxys de confiance, lus une seule fois depuis l'environnement.
static TRUSTED_PROXIES: OnceLock<TrustedProxies> = OnceLock::new();

/// Ensemble de réseaux (IP + longueur de préfixe) dont on accepte les
/// en-têtes de transfert.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TrustedProxies {
    nets: Vec<(IpAddr, u8)>,
}

impl TrustedProxies {
    /// Aucun proxy de confiance : les en-têtes de transfert sont ignorés.
    pub fn none() -> Self {
        Self::default()
    }

    /// Analyse une liste `ip[/préfixe]` séparée par des virgules ; les
    /// entrées invalides sont ignorées (et journalisées).
    pub fn parse(list: &str) -> Self {
        let mut nets = Vec::new();
        for raw in list.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            let (ip_part, prefix_part) = match raw.split_once('/') {
                Some((ip, p)) => (ip, Some(p)),
                None => (raw, None),
            };
            let Ok(ip) = ip_part.trim().parse::<IpAddr>() else {
                tracing::warn!("Ignoring invalid trusted proxy entry: {}", raw);
                continue;
            };
            let max = if ip.is_ipv4() { 32 } else { 128 };
            let prefix = match prefix_part {
                Some(p) => match p.trim().parse::<u8>() {
                    Ok(v) if v <= max => v,
                    _ => {
                        tracing::warn!("Ignoring invalid trusted proxy prefix: {}", raw);
                        continue;
                    }
                },
                None => max,
            };
            nets.push((ip, prefix));
        }
        Self { nets }
    }

    /// Proxy local uniquement (127.0.0.1 et ::1).
    pub fn loopback() -> Self {
        Self::parse("127.0.0.1,::1")
    }

    /// Vrai si `ip` appartient à l'un des réseaux de confiance.
    pub fn contains(&self, ip: &IpAddr) -> bool {
        self.nets
            .iter()
            .any(|(net, prefix)| ip_in_net(ip, net, *prefix))
    }

    fn from_env() -> Self {
        if let Ok(list) = std::env::var("MIYUCLOUD_TRUSTED_PROXIES")
            && !list.trim().is_empty()
        {
            return Self::parse(&list);
        }
        let legacy = std::env::var("MIYUCLOUD_TRUST_PROXY_HEADERS")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
        if legacy {
            Self::loopback()
        } else {
            Self::none()
        }
    }
}

fn ip_in_net(ip: &IpAddr, net: &IpAddr, prefix: u8) -> bool {
    match (ip, net) {
        (IpAddr::V4(a), IpAddr::V4(n)) => {
            let shift = 32u32.saturating_sub(u32::from(prefix));
            let mask = if prefix == 0 { 0 } else { u32::MAX << shift };
            (u32::from(*a) & mask) == (u32::from(*n) & mask)
        }
        (IpAddr::V6(a), IpAddr::V6(n)) => {
            let shift = 128u32.saturating_sub(u32::from(prefix));
            let mask = if prefix == 0 { 0 } else { u128::MAX << shift };
            (u128::from(*a) & mask) == (u128::from(*n) & mask)
        }
        // IPv4 encapsulée dans IPv6 (::ffff:a.b.c.d).
        (IpAddr::V6(a), IpAddr::V4(_)) => a
            .to_ipv4_mapped()
            .is_some_and(|v4| ip_in_net(&IpAddr::V4(v4), net, prefix)),
        _ => false,
    }
}

/// Décision pure (M-06) : IP client à utiliser comme clé de limitation.
///
/// - sans pair connu : `"unknown"` (seau commun, fail-closed) ;
/// - pair non fiable : IP du pair, en-têtes ignorés ;
/// - pair fiable : entrée la plus à droite de `X-Forwarded-For` qui n'est
///   pas un proxy de confiance, sinon `X-Real-Ip`, sinon le pair.
pub fn client_ip_from(
    peer: Option<IpAddr>,
    forwarded_for: Option<&str>,
    real_ip: Option<&str>,
    trusted: &TrustedProxies,
) -> String {
    let Some(peer) = peer else {
        return "unknown".to_string();
    };
    if !trusted.contains(&peer) {
        return peer.to_string();
    }
    if let Some(xff) = forwarded_for {
        for entry in xff.rsplit(',').map(str::trim) {
            match entry.parse::<IpAddr>() {
                Ok(ip) if trusted.contains(&ip) => continue,
                Ok(ip) => return ip.to_string(),
                // Entrée illisible : on ne remonte pas plus loin (forgeable).
                Err(_) => return peer.to_string(),
            }
        }
    }
    if let Some(ip) = real_ip.and_then(|v| v.trim().parse::<IpAddr>().ok())
        && !trusted.contains(&ip)
    {
        return ip.to_string();
    }
    peer.to_string()
}

/// A simple sliding-window counter keyed by IP address.
///
/// Each key lives for `window` seconds; every request increments the counter.
/// Once the counter reaches `max_requests` the request is rejected.
#[derive(Clone)]
pub struct RateLimiter {
    /// Maps `IP -> request_count` with automatic TTL expiration.
    cache: Cache<String, u32>,
    /// Maximum requests allowed within the window.
    max_requests: u32,
    /// Window duration in seconds (also used for `Retry-After`).
    window_secs: u64,
}

impl RateLimiter {
    /// Create a new rate limiter.
    ///
    /// * `max_requests` — ceiling per IP within the window
    /// * `window_secs`  — sliding window duration
    /// * `max_entries`  — upper bound on tracked IPs (evicts LRU when exceeded)
    pub fn new(max_requests: u32, window_secs: u64, max_entries: u64) -> Self {
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(window_secs))
            .max_capacity(max_entries)
            .build();
        Self {
            cache,
            max_requests,
            window_secs,
        }
    }

    /// Check whether the IP is allowed. Returns `Ok(current_count)` or
    /// `Err(StatusCode::TOO_MANY_REQUESTS)`.
    #[allow(clippy::result_unit_err)]
    pub fn check_and_increment(&self, ip: &str) -> Result<u32, ()> {
        let key = ip.to_string();
        // moka's entry API lets us atomically read-modify-write.
        // On first access the entry is inserted with count = 1 and the TTL
        // starts. Subsequent accesses within the window increment the count.
        let count = self.cache.entry(key).or_insert_with(|| 0).into_value() + 1;

        // Write back the incremented value.  Because `or_insert_with` returns
        // the *existing* value when the key was already present, we must always
        // re-insert so the counter actually advances. The TTL of the **first**
        // insert still governs eviction because moka uses insert-time TTL.
        // However, on re-insert moka resets the TTL — for rate limiting this
        // is fine because it means the window "slides" forward on activity.
        self.cache.insert(ip.to_string(), count);

        if count > self.max_requests {
            Err(())
        } else {
            Ok(count)
        }
    }

    /// Seconds the client should wait before retrying.
    pub fn retry_after(&self) -> u64 {
        self.window_secs
    }
}

// ─── Axum middleware factories ──────────────────────────────────────────────

/// Extract the real client IP (M-06).
///
/// Uses the TCP peer address (`ConnectInfo<SocketAddr>`); proxy headers are
/// only honoured when the peer is a configured trusted proxy.
pub fn extract_client_ip<B>(req: &Request<B>) -> String {
    let trusted = TRUSTED_PROXIES.get_or_init(TrustedProxies::from_env);
    let headers = req.headers();
    let peer = req
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|addr| addr.0.ip());
    if peer.is_none() {
        tracing::warn!("Rate limiting: no ConnectInfo on request — using shared bucket");
    }
    client_ip_from(
        peer,
        headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()),
        headers.get("x-real-ip").and_then(|v| v.to_str().ok()),
        trusted,
    )
}

/// Proxys de confiance configurés (`MIYUCLOUD_TRUSTED_PROXIES`).
pub fn trusted_proxies() -> &'static TrustedProxies {
    TRUSTED_PROXIES.get_or_init(TrustedProxies::from_env)
}

/// Build a rate-limit response with the standard `Retry-After` header.
pub fn too_many_requests(retry_after: u64) -> Response {
    let body = serde_json::json!({
        "error": "Too many requests",
        "retry_after_secs": retry_after,
    });
    let mut resp = (StatusCode::TOO_MANY_REQUESTS, axum::Json(body)).into_response();
    if let Ok(val) = HeaderValue::from_str(&retry_after.to_string()) {
        resp.headers_mut().insert("retry-after", val);
    }
    resp
}

/// Axum middleware: rate-limit login attempts.
///
/// Inject via:
/// ```ignore
/// .layer(axum::middleware::from_fn_with_state(limiter, rate_limit_login))
/// ```
pub async fn rate_limit_login(
    State(limiter): axum::extract::State<Arc<RateLimiter>>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let ip = extract_client_ip(&req);
    match limiter.check_and_increment(&ip) {
        Ok(_) => next.run(req).await,
        Err(()) => {
            tracing::warn!(
                ip = %ip,
                "Rate limit exceeded on login endpoint"
            );
            too_many_requests(limiter.retry_after())
        }
    }
}

/// Axum middleware: rate-limit registration attempts.
pub async fn rate_limit_register(
    State(limiter): axum::extract::State<Arc<RateLimiter>>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let ip = extract_client_ip(&req);
    match limiter.check_and_increment(&ip) {
        Ok(_) => next.run(req).await,
        Err(()) => {
            tracing::warn!(
                ip = %ip,
                "Rate limit exceeded on register endpoint"
            );
            too_many_requests(limiter.retry_after())
        }
    }
}

/// Axum middleware: rate-limit token refresh attempts.
pub async fn rate_limit_refresh(
    State(limiter): axum::extract::State<Arc<RateLimiter>>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let ip = extract_client_ip(&req);
    match limiter.check_and_increment(&ip) {
        Ok(_) => next.run(req).await,
        Err(()) => {
            tracing::warn!(
                ip = %ip,
                "Rate limit exceeded on refresh endpoint"
            );
            too_many_requests(limiter.retry_after())
        }
    }
}

use axum::extract::State;

#[cfg(test)]
mod tests {
    use super::*;

    fn ip(s: &str) -> Option<IpAddr> {
        s.parse().ok()
    }

    #[test]
    fn per_ip_buckets_are_independent() {
        let limiter = RateLimiter::new(3, 60, 1000);
        for _ in 0..3 {
            assert!(limiter.check_and_increment("203.0.113.1").is_ok());
        }
        // L'attaquant est bloqué…
        assert!(limiter.check_and_increment("203.0.113.1").is_err());
        // … mais un autre client ne l'est pas.
        assert!(limiter.check_and_increment("198.51.100.7").is_ok());
    }

    #[test]
    fn peer_ip_is_used_without_trusted_proxy() {
        let none = TrustedProxies::none();
        assert_eq!(
            client_ip_from(ip("203.0.113.1"), Some("1.2.3.4"), Some("5.6.7.8"), &none),
            "203.0.113.1"
        );
        // Sans pair connu : seau commun.
        assert_eq!(
            client_ip_from(None, Some("1.2.3.4"), None, &none),
            "unknown"
        );
    }

    #[test]
    fn spoofed_forwarded_for_is_ignored_from_untrusted_peer() {
        let trusted = TrustedProxies::loopback();
        assert_eq!(
            client_ip_from(ip("198.51.100.9"), Some("10.9.9.9"), None, &trusted),
            "198.51.100.9"
        );
    }

    #[test]
    fn rightmost_untrusted_forwarded_entry_is_the_client() {
        let trusted = TrustedProxies::parse("127.0.0.1, 10.0.0.0/8");
        // Le client a forgé « 1.1.1.1 » ; le proxy a ajouté l'IP réelle.
        assert_eq!(
            client_ip_from(
                ip("127.0.0.1"),
                Some("1.1.1.1, 203.0.113.5, 10.0.0.3"),
                None,
                &trusted
            ),
            "203.0.113.5"
        );
        // En-tête absent : X-Real-Ip puis pair.
        assert_eq!(
            client_ip_from(ip("127.0.0.1"), None, Some("203.0.113.6"), &trusted),
            "203.0.113.6"
        );
        assert_eq!(
            client_ip_from(ip("127.0.0.1"), None, None, &trusted),
            "127.0.0.1"
        );
        // Entrée illisible : pas de remontée au-delà.
        assert_eq!(
            client_ip_from(ip("127.0.0.1"), Some("1.1.1.1, garbage"), None, &trusted),
            "127.0.0.1"
        );
    }

    #[test]
    fn trusted_proxy_parsing_and_cidr_matching() {
        let t = TrustedProxies::parse("192.168.1.0/24, ::1, bad, 10.0.0.1/40");
        assert!(t.contains(&"192.168.1.200".parse().unwrap()));
        assert!(!t.contains(&"192.168.2.1".parse().unwrap()));
        assert!(t.contains(&"::1".parse().unwrap()));
        assert!(!t.contains(&"10.0.0.1".parse().unwrap()));
        assert!(t.contains(&"::ffff:192.168.1.5".parse().unwrap()));
        assert!(!TrustedProxies::none().contains(&"127.0.0.1".parse().unwrap()));
    }

    #[tokio::test]
    async fn middleware_limits_each_connect_info_ip_separately() {
        use axum::{Router, body::Body, routing::post};
        use tower::ServiceExt;

        let limiter = Arc::new(RateLimiter::new(2, 60, 1000));
        let app = Router::new()
            .route("/login", post(|| async { "ok" }))
            .layer(axum::middleware::from_fn_with_state(
                limiter,
                rate_limit_login,
            ));

        let call = |addr: &str| {
            let mut req = Request::builder()
                .method("POST")
                .uri("/login")
                .header("x-forwarded-for", "9.9.9.9")
                .body(Body::empty())
                .unwrap();
            req.extensions_mut()
                .insert(ConnectInfo(addr.parse::<SocketAddr>().unwrap()));
            app.clone().oneshot(req)
        };

        assert_eq!(
            call("203.0.113.1:1000").await.unwrap().status(),
            StatusCode::OK
        );
        assert_eq!(
            call("203.0.113.1:1001").await.unwrap().status(),
            StatusCode::OK
        );
        assert_eq!(
            call("203.0.113.1:1002").await.unwrap().status(),
            StatusCode::TOO_MANY_REQUESTS
        );
        // Autre IP : non bloquée malgré le même X-Forwarded-For forgé.
        assert_eq!(
            call("198.51.100.2:1000").await.unwrap().status(),
            StatusCode::OK
        );
    }
}
