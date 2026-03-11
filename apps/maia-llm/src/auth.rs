//! Middleware d'authentification Bearer (Authorization: Bearer <key> ou X-MAIA-Key: <key>).
//! La clé est injectée via `axum::Extension<Option<String>>`.
//! Si la clé est None, toutes les requêtes sont acceptées.

use axum::{
    extract::{Extension, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Middleware axum : vérifie le Bearer token si auth_key est configuré.
/// Nécessite `Extension<Option<String>>` injectée au niveau de l'application.
pub async fn auth_middleware(
    Extension(auth_key): Extension<Option<String>>,
    req: Request,
    next: Next,
) -> Response {
    // Pas d'auth configurée → pass-through
    let Some(expected_key) = auth_key else {
        return next.run(req).await;
    };

    // Vérifier Authorization: Bearer <key> OU X-MAIA-Key: <key>
    let provided = extract_bearer(req.headers())
        .or_else(|| {
            req.headers()
                .get("x-maia-key")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
        });

    match provided {
        Some(key) if constant_time_eq(&key, &expected_key) => next.run(req).await,
        _ => (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": {"message": "Unauthorized", "type": "auth_error"}})),
        )
            .into_response(),
    }
}

fn extract_bearer(headers: &axum::http::HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

/// Comparaison à temps constant (anti timing-attack).
fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.bytes().zip(b.bytes()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}
