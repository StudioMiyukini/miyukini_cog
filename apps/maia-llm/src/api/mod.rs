//! @id     maia_llm_api_mod
//! @do     declarer_module_chat
//! @role   orchestration
//! @layer  app
//! @human  maia-llm — module « api mod ».

//! Routeur axum de maia-llm.

pub mod chat;
pub mod health;
pub mod models;

use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

use crate::{
    auth::{auth_middleware, Gardes},
    AppState,
};

/// Construit le router principal. Routes protégées : requête signée par un
/// appelant épinglé pour la famille de la route (voir `auth`).
pub fn build_router(state: AppState, gardes: Gardes) -> Router {
    // Route publique — pas d'auth
    let public = Router::new()
        .route("/maia/llm/health", get(health::health_handler));

    // Routes protégées par requête signée
    let protected = Router::new()
        // OpenAI-compat
        .route("/v1/chat/completions", post(chat::chat_handler))
        .route("/v1/models", get(models::list_loaded_handler))
        // MAIA-LLM specific
        .route("/maia/llm/models", get(models::list_available_handler))
        .route("/maia/llm/models/load", post(models::load_handler))
        .route("/maia/llm/models/unload", delete(models::unload_handler))
        .route("/maia/llm/models/{id}/hash", post(models::hash_handler))
        // Requête signée par un appelant épinglé (fail-closed)
        .route_layer(middleware::from_fn_with_state(gardes, auth_middleware));

    Router::new()
        .merge(public)
        .merge(protected)
        .with_state(state)
}
