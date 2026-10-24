//! @id     team_auth
//! @do     auth_middleware
//! @role   securite
//! @layer  app
//! @human  team — module « auth ».

//! Middleware d'authentification du hub TEAM (C-22).
//!
//! - Secret configuré → `Authorization: Bearer <secret>` exigé sur la route ;
//!   absent ou invalide → 401. Comparaison en temps constant (`subtle`).
//! - Aucun secret → requête acceptée. Ce cas n'est possible qu'en écoute
//!   loopback : `config::validate_startup` refuse sinon de démarrer.

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use subtle::ConstantTimeEq;

/// Secret attendu (None = pas d'authentification, loopback uniquement).
#[derive(Clone, Default)]
pub struct AuthConfig {
    secret: Option<Arc<str>>,
}

impl AuthConfig {
    pub fn new(secret: Option<&str>) -> Self {
        Self {
            secret: secret.map(Arc::from),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.secret.is_some()
    }
}

/// Middleware axum : vérifie le Bearer token si un secret est configuré.
pub async fn auth_middleware(State(auth): State<AuthConfig>, req: Request, next: Next) -> Response {
    let Some(expected) = auth.secret.as_deref() else {
        // Sans secret (loopback seul) : refuser tout ce qui ne vient pas
        // explicitement de la machine locale, contre le DNS rebinding.
        if requete_locale(&req) {
            return next.run(req).await;
        }
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"error": "forbidden"})),
        )
            .into_response();
    };

    let provided = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    match provided {
        Some(token) if constant_time_eq(token, expected) => next.run(req).await,
        _ => (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "unauthorized"})),
        )
            .into_response(),
    }
}

/// Hôte local exact : `localhost`, `127.0.0.1` ou `[::1]`, port facultatif.
fn hote_local(valeur: &str) -> bool {
    let hote = if let Some(reste) = valeur.strip_prefix('[') {
        match reste.split_once(']') {
            Some((h, apres)) if apres.is_empty() || apres.starts_with(':') => h,
            _ => return false,
        }
    } else {
        match valeur.rsplit_once(':') {
            Some((h, port)) if !port.is_empty() && port.bytes().all(|b| b.is_ascii_digit()) => h,
            Some(_) => return false,
            None => valeur,
        }
    };
    hote.eq_ignore_ascii_case("localhost") || hote == "127.0.0.1" || hote == "::1"
}

/// Requête locale : `Host` local obligatoire ; `Origin`, s'il est présent,
/// doit être `http(s)://` vers un hôte local.
fn requete_locale(req: &Request) -> bool {
    let hote_ok = req
        .headers()
        .get(header::HOST)
        .and_then(|v| v.to_str().ok())
        .is_some_and(hote_local);
    let origine_ok = match req.headers().get(header::ORIGIN) {
        None => true,
        Some(v) => v
            .to_str()
            .ok()
            .and_then(|o| o.strip_prefix("http://").or_else(|| o.strip_prefix("https://")))
            .is_some_and(hote_local),
    };
    hote_ok && origine_ok
}

/// Comparaison en temps constant (seule la longueur peut fuiter).
fn constant_time_eq(a: &str, b: &str) -> bool {
    a.as_bytes().ct_eq(b.as_bytes()).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparaison_constante() {
        assert!(constant_time_eq("abc", "abc"));
        assert!(!constant_time_eq("abc", "abd"));
        assert!(!constant_time_eq("abc", "abcd"));
        assert!(!constant_time_eq("", "abc"));
    }

    #[test]
    fn hote_local_exact() {
        for ok in ["localhost", "localhost:7800", "127.0.0.1", "127.0.0.1:7800", "[::1]", "[::1]:7800", "LOCALHOST"] {
            assert!(hote_local(ok), "{ok}");
        }
        for ko in ["localhost.evil.com", "evil.com", "127.0.0.1.evil.com:80", "127.0.0.2", "localhost:abc", "[::1]x", "", "::1"] {
            assert!(!hote_local(ko), "{ko}");
        }
    }
}
