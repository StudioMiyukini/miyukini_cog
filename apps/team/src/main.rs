//! TEAM — Hub d'orchestration MAIA.
//!
//! Centralise les nœuds MAIA, orchestre les tâches, expose la MWS gateway.
//!
//! Usage :
//!   team serve --port 7800
//!   TEAM_CONFIG=team.toml team serve

mod api;
mod config;
mod mipower;
mod mws;
mod nodes;
mod orchestrator;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use nodes::registry::NodeRegistry;
use orchestrator::scheduler::TaskScheduler;

/// État partagé du hub TEAM.
#[derive(Clone)]
pub struct AppState {
    pub registry: NodeRegistry,
    pub scheduler: TaskScheduler,
    pub sequences: Arc<Vec<mipower::MiPowerSequence>>,
    pub config: Arc<config::TeamConfig>,
    pub client: reqwest::Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "team=info,tower_http=info".into()),
        )
        .init();

    let cfg = config::load_or_create();

    tracing::info!("╔══════════════════════════════════════════════════╗");
    tracing::info!(
        "║     TEAM Hub v{}                          ║",
        env!("CARGO_PKG_VERSION")
    );
    tracing::info!("╠══════════════════════════════════════════════════╣");
    tracing::info!("║  Listen     : {:<35}║", cfg.bind_addr);
    tracing::info!("║  Isolation  : {:<35}║", cfg.isolation_policy);
    tracing::info!(
        "║  Heartbeat  : {:<35}║",
        format!("{}s (stale: {}s)", cfg.health_interval_secs, cfg.stale_threshold_secs)
    );

    // Charger les séquences MIPOWER
    let sequences_dir = std::path::Path::new(".mip/sequences");
    let sequences = mipower::load_sequences(sequences_dir);
    tracing::info!("║  Séquences  : {:<35}║", format!("{} chargées", sequences.len()));

    tracing::info!("╚══════════════════════════════════════════════════╝");

    let registry = NodeRegistry::new();
    let scheduler = TaskScheduler::new();

    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("reqwest client");

    let state = AppState {
        registry: registry.clone(),
        scheduler,
        sequences: Arc::new(sequences),
        config: Arc::new(cfg.clone()),
        client: http_client.clone(),
    };

    // Démarrer la boucle heartbeat
    {
        let registry = registry.clone();
        let client = http_client.clone();
        let interval = cfg.health_interval_secs;
        let stale = cfg.stale_threshold_secs;
        tokio::spawn(async move {
            nodes::heartbeat::run_heartbeat_loop(registry, client, interval, stale).await;
        });
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // ── Enregistrement MAIA ────────────────────────────────────
        .route("/team/register", post(api::register_node))
        .route("/team/heartbeat", post(api::node_heartbeat))
        .route("/team/nodes/{id}", delete(api::unregister_node))
        .route("/team/nodes", get(api::list_nodes_internal))
        // ── MWS Gateway ────────────────────────────────────────────
        .route("/api/task", post(mws::routes::submit_task))
        .route("/api/task/{id}", get(mws::routes::get_task))
        .route("/api/nodes", get(mws::routes::list_nodes))
        .route("/api/sequences", get(mws::routes::list_sequences))
        .route("/api/status", get(mws::routes::team_status))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&cfg.bind_addr)
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Impossible d'écouter sur {} : {e}", cfg.bind_addr);
            std::process::exit(1);
        });

    tracing::info!("TEAM Hub prêt — Ctrl+C pour arrêter");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|e| tracing::error!("Erreur serveur : {e}"));

    tracing::info!("TEAM Hub arrêté");
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
}
