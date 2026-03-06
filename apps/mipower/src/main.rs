//! MIPOWER — Miyukini Implementation Protocol Oriented Workflow Editor & Reviewer
//!
//! Serveur HTTP local (127.0.0.1:9765) qui expose :
//!   GET  /                        -> static/index.html
//!   GET  /static/*                -> fichiers statiques
//!   GET  /api/sequences           -> liste des sequences MIP (JSON)
//!   GET  /api/artefact?path=...   -> contenu d'un artefact .md
//!   GET  /api/progress/:slug      -> progression d'une sequence
//!   POST /api/prompt              -> generation premier prompt MIP
//!   POST /api/settings            -> mise a jour mip_root
//!   GET  /sse                     -> Server-Sent Events (suivi live)

use std::sync::{Arc, Mutex};
use axum::{
    Router,
    routing::get,
    response::{Html, IntoResponse},
    http::StatusCode,
};
use api::sse_handler;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod db;
mod models;
mod watcher;

pub const PORT: u16 = 9765;

pub struct AppState {
    pub db:       Mutex<rusqlite::Connection>,
    pub mip_root: Mutex<String>,
    pub events:   broadcast::Sender<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "mipower=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_path = dirs_or_local();
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let conn = db::open(&db_path)?;
    tracing::info!("DB : {}", db_path.display());

    let default_root = std::env::current_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let (events_tx, _) = broadcast::channel::<String>(64);

    let state = Arc::new(AppState {
        db:       Mutex::new(conn),
        mip_root: Mutex::new(default_root.clone()),
        events:   events_tx.clone(),
    });

    // Lancer le file watcher sur sequences/
    let seq_dir = {
        let candidates = [
            std::path::PathBuf::from(&default_root).join(".mip").join("sequences"),
            std::path::PathBuf::from(&default_root).join("sequences"),
        ];
        candidates.into_iter().find(|p| p.exists())
    };

    if let Some(dir) = seq_dir {
        if let Err(e) = watcher::start_watcher(&dir, events_tx) {
            tracing::warn!("File watcher non demarre : {e}");
        }
    } else {
        tracing::info!("sequences/ introuvable au demarrage — watcher desactive. Configurez mip_root.");
    }

    let static_dir = static_dir_path();

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/sse", get(sse_handler))
        .nest("/api", api::router())
        .nest_service("/static", ServeDir::new(&static_dir))
        .fallback(not_found_handler)
        .with_state(state);

    let addr = format!("127.0.0.1:{PORT}");
    tracing::info!("MIPOWER en ligne : http://{addr}");
    println!("MIPOWER en ligne : http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn dirs_or_local() -> std::path::PathBuf {
    if let Some(base) = std::env::var_os("APPDATA") {
        return std::path::PathBuf::from(base).join("mipower").join("mipower.db");
    }
    std::path::PathBuf::from("mipower.db")
}

fn static_dir_path() -> std::path::PathBuf {
    let candidates = [
        std::path::PathBuf::from("apps/mipower/static"),
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join("static")))
            .unwrap_or_default(),
    ];
    candidates.into_iter().find(|p| p.exists())
        .unwrap_or_else(|| std::path::PathBuf::from("static"))
}

async fn index_handler() -> impl IntoResponse {
    let html = include_str!("../static/index.html");
    Html(html)
}

async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 — Ressource introuvable")
}
