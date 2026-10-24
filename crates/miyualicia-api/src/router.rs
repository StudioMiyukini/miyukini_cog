//! Definition des routes axum et middlewares.

// @id service.alicia.api.router
// @role route_definition
// @layer 7
// @do define_api_routes
// @human Routeur axum complet avec middlewares : JWT, CORS, rate limit, trace.

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
    /// Limiteur de debit en memoire (par IP, `rate_limit_per_min`).
    pub rate_limiter: crate::security::RateLimiter,
}

/// Middleware de limitation de debit par IP (fenetre fixe 60 s, en memoire).
async fn rate_limit_mw(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    request: axum::http::Request<axum::body::Body>,
    next: middleware::Next,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let ip = request
        .extensions()
        .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
        .map(|ci| ci.0.ip());
    match state.rate_limiter.check(ip) {
        Ok(()) => next.run(request).await,
        Err(retry_after_secs) => {
            crate::errors::ApiError::TooManyRequests { retry_after_secs }.into_response()
        }
    }
}

/// Couche CORS a liste explicite d'origines (aucun joker). `None` si CORS desactive
/// ou liste vide (alors aucune origine croisee n'est autorisee).
fn build_cors(config: &ApiConfig) -> Option<tower_http::cors::CorsLayer> {
    if !config.cors_enabled {
        return None;
    }
    let origins: Vec<axum::http::HeaderValue> = config
        .cors_origins
        .iter()
        .filter(|o| o.trim() != "*")
        .filter_map(|o| o.parse().ok())
        .collect();
    if origins.is_empty() {
        return None;
    }
    Some(
        tower_http::cors::CorsLayer::new()
            .allow_origin(origins)
            .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
            .allow_headers([
                axum::http::header::AUTHORIZATION,
                axum::http::header::CONTENT_TYPE,
            ]),
    )
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
        rate_limiter: crate::security::RateLimiter::new(config.rate_limit_per_min),
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
        .route("/history", get(handlers::history::list))
        // ----- Capabilities (P1-P7) -----
        .route("/capabilities/tools", get(handlers::capabilities::tools))
        .route("/capabilities/services", get(handlers::capabilities::services))
        .route("/capabilities/dispatch", post(handlers::capabilities::dispatch))
        .route(
            "/capabilities/dispatch/confirm",
            post(handlers::capabilities::dispatch_confirm),
        )
        .route(
            "/capabilities/dispatch/cancel",
            post(handlers::capabilities::dispatch_cancel),
        )
        .route(
            "/capabilities/learnings/pending/{persona}",
            get(handlers::capabilities::learnings_pending),
        )
        .route(
            "/capabilities/learnings/active/{persona}",
            get(handlers::capabilities::learnings_active),
        )
        .route(
            "/capabilities/learnings/{id}/accept",
            post(handlers::capabilities::learnings_accept),
        )
        .route(
            "/capabilities/learnings/{id}/reject",
            post(handlers::capabilities::learnings_reject),
        )
        .route(
            "/capabilities/learnings/detect/{persona}",
            post(handlers::capabilities::learnings_detect),
        )
        .route(
            "/capabilities/persona/active",
            get(handlers::capabilities::persona_active),
        )
        .route(
            "/capabilities/persona/{id}/activate",
            post(handlers::capabilities::persona_activate),
        )
        .route(
            "/capabilities/manifest/reload",
            post(handlers::capabilities::manifest_reload),
        )
        .route(
            "/capabilities/episodes/recent",
            get(handlers::capabilities::episodes_recent),
        )
        // ----- Queue async des PendingConfirmation -----
        .route(
            "/capabilities/dispatch_and_queue",
            post(handlers::capabilities::dispatch_and_queue),
        )
        .route(
            "/capabilities/pending_confirmations",
            get(handlers::capabilities::pending_list),
        )
        .route(
            "/capabilities/pending_confirmations/{id}/confirm",
            post(handlers::capabilities::pending_confirm),
        )
        .route(
            "/capabilities/pending_confirmations/{id}/cancel",
            post(handlers::capabilities::pending_cancel),
        );

    // Combinaison
    let api_routes = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            inject_jwt_secret,
        ));

    // Routeur final avec prefixe et middlewares globaux
    let mut router = Router::new()
        .nest("/api/v1/alicia", api_routes)
        .layer(RequestBodyLimitLayer::new(65_536)) // 64 Ko max
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            rate_limit_mw,
        ));
    if let Some(cors) = build_cors(&config) {
        router = router.layer(cors);
    }
    router
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}

/// POST /api/v1/alicia/auth/token
///
/// Genere un token JWT pour un client authentifie. Le secret est verifie (Argon2id, temps
/// constant) contre le registre `clients` de la config ; registre vide => refus (fail-closed).
/// Les scopes viennent de la config du client, jamais du nom du `client_id`.
async fn auth_token_handler(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    axum::Json(req): axum::Json<crate::dto::TokenRequest>,
) -> Result<axum::Json<crate::dto::TokenResponse>, crate::errors::ApiError> {
    if state.config.clients.is_empty() {
        tracing::warn!("/auth/token refuse : aucun client configure (api.clients vide)");
        return Err(crate::errors::ApiError::Unauthorized);
    }
    let config = state.config.clone();
    let (cid, secret) = (req.client_id.clone(), req.secret);
    // Argon2 est couteux : hors du runtime async.
    let scopes = tokio::task::spawn_blocking(move || {
        crate::security::authenticate_client(&config, &cid, &secret)
    })
    .await
    .map_err(|e| crate::errors::ApiError::InternalError(format!("auth: {e}")))?
    .ok_or(crate::errors::ApiError::Unauthorized)?;

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
