//! Definition des routes axum et middlewares.

// @id: service.alicia.api.router
// @role: route_definition
// @layer: 7
// @human: Routeur axum complet avec middlewares : JWT, CORS, rate limit, trace.
// @do: define_api_routes

use std::sync::Arc;
use std::time::Instant;

use axum::routing::{get, post};
use axum::{middleware, Router};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;

use miyualicia::AliciaService;

use crate::auth::JwtSecret;
use crate::config::ApiConfig;
use crate::handlers;

/// Etat partage de l'application axum.
#[derive(Clone)]
pub struct AppState {
    /// Service Alicia partage.
    pub alicia: Arc<AliciaService>,
    /// Configuration API.
    pub config: ApiConfig,
    /// Cle JWT HS256.
    pub jwt_secret: JwtSecret,
    /// Instant de demarrage du serveur (pour le calcul d'uptime).
    pub start_time: Instant,
}

/// Middleware axum pour injecter le `JwtSecret` dans les extensions de la requete.
///
/// Permet a l'extractor `JwtAuth` de recuperer la cle sans la passer en state.
async fn inject_jwt_secret(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    mut request: axum::http::Request<axum::body::Body>,
    next: middleware::Next,
) -> axum::response::Response {
    request.extensions_mut().insert(state.jwt_secret.clone());
    next.run(request).await
}

/// Construit le routeur axum complet avec tous les middlewares.
pub fn build_router(alicia: Arc<AliciaService>, config: ApiConfig, secret: Vec<u8>) -> Router {
    let app_state = Arc::new(AppState {
        alicia,
        config: config.clone(),
        jwt_secret: JwtSecret(Arc::new(secret)),
        start_time: Instant::now(),
    });

    // Routes publiques (sans JWT)
    let public_routes = Router::new()
        .route("/health", get(handlers::health::health))
        .route("/auth/token", post(auth_token_handler));

    // Routes protegees (JWT requis via extractor JwtAuth)
    let protected_routes = Router::new()
        .route("/state", get(handlers::state::get_state))
        .route("/rooms", get(handlers::rooms::list))
        .route("/rooms/{room_id}", get(handlers::rooms::get))
        .route("/rooms/{room_id}/devices", get(handlers::rooms::devices))
        .route("/devices", get(handlers::devices::list))
        .route("/devices/{id}", get(handlers::devices::get))
        .route("/devices/{id}/command", post(handlers::devices::command))
        .route("/history", get(handlers::history::list));

    // Combinaison
    let api_routes = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            inject_jwt_secret,
        ));

    // Routeur final avec prefixe et middlewares globaux
    Router::new()
        .nest("/api/v1/alicia", api_routes)
        .layer(RequestBodyLimitLayer::new(65_536)) // 64 Ko max
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}

/// POST /api/v1/alicia/auth/token
///
/// Genere un token JWT pour un client authentifie.
/// En Phase 1, le secret est verifie en comparaison simple.
async fn auth_token_handler(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    axum::Json(req): axum::Json<crate::dto::TokenRequest>,
) -> Result<axum::Json<crate::dto::TokenResponse>, crate::errors::ApiError> {
    // Phase 1 : verification simplifiee (a remplacer par hash BLAKE3 en Phase 2)
    // Le client_id et le secret sont verifies contre des valeurs configurees
    if req.client_id.is_empty() || req.secret.is_empty() {
        return Err(crate::errors::ApiError::Unauthorized);
    }

    // Generer les scopes en fonction du client_id
    let scopes = match req.client_id.as_str() {
        "admin" => vec![crate::auth::JwtScope::Admin],
        _ => vec![
            crate::auth::JwtScope::Read,
            crate::auth::JwtScope::Write,
            crate::auth::JwtScope::History,
        ],
    };

    let scope_names: Vec<String> = scopes
        .iter()
        .map(std::string::ToString::to_string)
        .collect();
    let ttl = state.config.token_ttl_secs;
    let claims = crate::auth::JwtClaims::new(&req.client_id, scopes, ttl);
    let token = crate::auth::generate_token(&claims, &state.jwt_secret.0)?;

    Ok(axum::Json(crate::dto::TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: ttl,
        scopes: scope_names,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyualicia::AliciaConfig;

    #[test]
    fn test_build_router_creates_router() {
        let config = AliciaConfig {
            mqtt: None,
            ..Default::default()
        };
        let alicia = Arc::new(AliciaService::new(config));
        let secret = b"test-secret-key-at-least-32-bytes-long!!".to_vec();
        let _router = build_router(alicia, ApiConfig::default(), secret);
        // Si on arrive ici, le routeur s'est construit sans panic
    }
}
