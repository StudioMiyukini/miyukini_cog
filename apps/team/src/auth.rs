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
        return next.run(req).await;
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
}
