use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/sequences", get(sequences_handler))
        .route("/artefact", get(artefact_handler))
        .route("/health", get(health_handler))
}

async fn health_handler() -> &'static str {
    "ok"
}

// Stubs E00 — implémentés en E01
async fn sequences_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "sequences": [], "status": "stub_e00" }))
}

async fn artefact_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "content": "", "status": "stub_e00" }))
}
