//! @id     miyucloud_interfaces_middleware_csrf
//! @do     definir_safe_methods
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « interfaces middleware csrf ».

//! CSRF double-submit cookie middleware.
//!
//! State-changing requests (`POST`, `PUT`, `DELETE`, `PATCH`) that were
//! authenticated via an HttpOnly cookie (i.e. browser sessions) **must**
//! include an `X-CSRF-Token` header whose value matches the `miyucloud_csrf`
//! cookie.  Requests authenticated via `Bearer` or `Basic` headers are
//! exempt because they are not vulnerable to CSRF — the browser never
//! attaches those automatically.
//!
//! Safe methods (`GET`, `HEAD`, `OPTIONS`) are always allowed through.

use axum::{
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::interfaces::api::cookie_auth;
use crate::interfaces::middleware::auth::CookieAuthenticated;

/// Methods considered safe (no side-effects) — CSRF check is skipped.
const SAFE_METHODS: [Method; 3] = [Method::GET, Method::HEAD, Method::OPTIONS];

/// Middleware that enforces CSRF protection for cookie-authenticated browser
/// sessions using the **double-submit cookie** pattern.
///
/// Must be applied **after** `auth_middleware` so that the
/// `CookieAuthenticated` marker is available in extensions.
#[allow(clippy::result_large_err)] // Err = `Response` axum, convention des handlers.
pub async fn csrf_middleware(request: Request, next: Next) -> Result<Response, Response> {
    // Safe methods never need CSRF validation.
    if SAFE_METHODS.contains(request.method()) {
        return Ok(next.run(request).await);
    }

    // Only enforce for cookie-authenticated sessions.
    let is_cookie_auth = request.extensions().get::<CookieAuthenticated>().is_some();
    if !is_cookie_auth {
        return Ok(next.run(request).await);
    }

    // Extract the CSRF token from the cookie.
    let cookie_token =
        cookie_auth::extract_cookie_value(request.headers(), cookie_auth::CSRF_COOKIE);

    // Extract the CSRF token from the request header.
    let header_token = request
        .headers()
        .get(cookie_auth::CSRF_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    match (cookie_token, header_token) {
        (Some(c), Some(h)) if !c.is_empty() && c == h => {
            // Tokens match — allow the request through.
            Ok(next.run(request).await)
        }
        _ => {
            tracing::warn!(
                method = %request.method(),
                uri = %request.uri(),
                "CSRF validation failed: missing or mismatched token"
            );
            Err((
                StatusCode::FORBIDDEN,
                axum::Json(serde_json::json!({
                    "error": "CSRF token missing or invalid"
                })),
            )
                .into_response())
        }
    }
}

/// Hôte (`hôte[:port]`, minuscules) d'une valeur `Origin` (`schéma://hôte[:port]`).
fn hote_origine(origine: &str) -> Option<String> {
    let (_, reste) = origine.trim().split_once("://")?;
    let hote = reste.split(['/', '?', '#']).next()?;
    (!hote.is_empty()).then(|| hote.to_ascii_lowercase())
}

/// Vrai si une requête de navigateur vient d'une autre origine : `Sec-Fetch-Site`
/// autre que `same-origin` / `none`, ou `Origin` (présent) d'un autre hôte que
/// `X-Forwarded-Host` (mandataire) ou `Host`. `Origin: null` : autre origine.
pub fn origine_etrangere(headers: &axum::http::HeaderMap) -> bool {
    let lire = |nom: &str| headers.get(nom).and_then(|v| v.to_str().ok());
    if let Some(site) = lire("sec-fetch-site")
        && !matches!(site.trim(), "same-origin" | "none")
    {
        return true;
    }
    let Some(origine) = lire("origin") else {
        return false;
    };
    let Some(source) = hote_origine(origine) else {
        return true;
    };
    let cible = lire("x-forwarded-host")
        .and_then(|h| h.split(',').next())
        .or_else(|| lire("host"))
        .map(|h| h.trim().to_ascii_lowercase());
    cible.is_none_or(|c| c != source)
}

/// Défense en profondeur des routes de gestion des titres (liaisons
/// d'identité et de clés d'appareil, I6) : une requête modifiante d'une
/// **session de navigateur** venue d'une autre origine est refusée (403),
/// en plus du jeton CSRF. À appliquer **après** `auth_middleware`.
#[allow(clippy::result_large_err)] // Err = `Response` axum, convention des handlers.
pub async fn same_origin_middleware(request: Request, next: Next) -> Result<Response, Response> {
    if SAFE_METHODS.contains(request.method())
        || request.extensions().get::<CookieAuthenticated>().is_none()
        || !origine_etrangere(request.headers())
    {
        return Ok(next.run(request).await);
    }
    tracing::warn!(
        method = %request.method(),
        uri = %request.uri(),
        "Cross-origin browser request refused"
    );
    Err((
        StatusCode::FORBIDDEN,
        axum::Json(serde_json::json!({ "error": "Cross-origin request refused" })),
    )
        .into_response())
}

#[cfg(test)]
mod origine_tests {
    use super::*;
    use axum::http::HeaderMap;

    fn h(paires: &[(&'static str, &str)]) -> HeaderMap {
        let mut m = HeaderMap::new();
        for (k, v) in paires {
            m.insert(*k, v.parse().unwrap());
        }
        m
    }

    #[test]
    fn same_origin_requests_pass() {
        assert!(!origine_etrangere(&h(&[])));
        assert!(!origine_etrangere(&h(&[("host", "cloud.local:8086")])));
        assert!(!origine_etrangere(&h(&[
            ("host", "Cloud.Local:8086"),
            ("origin", "https://cloud.local:8086"),
            ("sec-fetch-site", "same-origin"),
        ])));
        // Derrière un mandataire : X-Forwarded-Host prime.
        assert!(!origine_etrangere(&h(&[
            ("host", "127.0.0.1:8086"),
            ("x-forwarded-host", "cloud.example"),
            ("origin", "https://cloud.example"),
        ])));
    }

    #[test]
    fn foreign_origin_is_detected() {
        assert!(origine_etrangere(&h(&[("sec-fetch-site", "cross-site")])));
        assert!(origine_etrangere(&h(&[("sec-fetch-site", "same-site")])));
        assert!(origine_etrangere(&h(&[
            ("host", "cloud.local"),
            ("origin", "https://evil.example"),
        ])));
        assert!(origine_etrangere(&h(&[
            ("host", "cloud.local"),
            ("origin", "null")
        ])));
        assert!(origine_etrangere(&h(&[("origin", "https://cloud.local")])));
    }
}
