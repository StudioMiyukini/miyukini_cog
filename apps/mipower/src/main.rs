//! MIPOWER — Miyukini Implementation Protocol Oriented Workflow Editor & Reviewer
//!
//! Serveur HTTP local (127.0.0.1:9765) qui expose :
//!   - GET  /                    -> static/index.html
//!   - GET  /static/*            -> fichiers statiques (JS, CSS)
//!   - GET  /api/sequences       -> liste des sequences MIP (JSON)
//!   - GET  /api/artefact        -> contenu d'un artefact .md (?path=...)
//!   - GET  /api/progress/:slug  -> progression d'une sequence
//!   - POST /api/prompt          -> generation premier prompt MIP
//!   - GET  /sse                 -> Server-Sent Events (suivi live)

use axum::{
    Router,
    routing::get,
    response::{Html, IntoResponse},
    http::StatusCode,
};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod db;
mod models;
mod watcher;

pub const PORT: u16 = 9765;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "mipower=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let static_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("static")))
        .unwrap_or_else(|| std::path::PathBuf::from("apps/mipower/static"));

    let app = Router::new()
        .route("/", get(index_handler))
        .nest("/api", api::router())
        .nest_service("/static", ServeDir::new(&static_dir))
        .fallback(not_found_handler);

    let addr = format!("127.0.0.1:{PORT}");
    tracing::info!("MIPOWER en ligne : http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index_handler() -> impl IntoResponse {
    let html = include_str!("../static/index.html");
    Html(html)
}

async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 — Ressource introuvable")
}
