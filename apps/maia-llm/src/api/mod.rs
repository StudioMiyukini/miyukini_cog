//! Routeur axum de maia-llm.

pub mod chat;
pub mod health;
pub mod models;

use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

use crate::{auth::auth_middleware, AppState};

/// Construit le router principal.
/// L'auth key doit être injectée via `axum::Extension<Option<String>>`
/// au niveau de l'application (voir main.rs).
pub fn build_router(state: AppState) -> Router {
    // Route publique — pas d'auth
    let public = Router::new()
        .route("/maia/llm/health", get(health::health_handler));

    // Routes protégées par Bearer token
    let protected = Router::new()
        // OpenAI-compat
        .route("/v1/chat/completions", post(chat::chat_handler))
        .route("/v1/models", get(models::list_loaded_handler))
        // MAIA-LLM specific
        .route("/maia/llm/models", get(models::list_available_handler))
        .route("/maia/llm/models/load", post(models::load_handler))
        .route("/maia/llm/models/unload", delete(models::unload_handler))
        .route("/maia/llm/models/{id}/hash", post(models::hash_handler))
        // Auth middleware — lit Extension<Option<String>> injectée dans main.rs
        .route_layer(middleware::from_fn(auth_middleware));

    Router::new()
        .merge(public)
        .merge(protected)
        .with_state(state)
}
