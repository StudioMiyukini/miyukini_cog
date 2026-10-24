//! @id     team_main
//! @do     declarer_module_api
//! @role   orchestration
//! @layer  app
//! @human  team — module « main ».

//! TEAM — Hub d'orchestration MAIA.
//!
//! Centralise les nœuds MAIA, orchestre les tâches, expose la MWS gateway.
//!
//! Usage :
//!   team                                  (écoute 127.0.0.1:7800)
//!   TEAM_CONFIG=team.toml team
//!   TEAM_HMAC_SECRET=<secret> team        (Bearer exigé ; requis hors loopback)

mod api;
mod app;
mod auth;
mod config;
mod mipower;
mod mws;
mod nodes;
mod orchestrator;

use std::sync::Arc;

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

    // Fail-closed (C-21/C-22) : pas d'écoute hors loopback sans secret.
    if let Err(e) = config::validate_startup(&cfg) {
        tracing::error!("{e}");
        eprintln!("{e}");
        std::process::exit(1);
    }
    if cfg.secret().is_none() {
        tracing::warn!(
            "Aucun hmac_secret configuré : routes ouvertes aux processus locaux (loopback uniquement)"
        );
    }

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

    // Pas de suivi de redirection : une URL de nœud validée ne doit pas
    // pouvoir rediriger le hub vers une destination arbitraire (SSRF).
    let http_client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::none())
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Client HTTP impossible à construire : {e}");
            std::process::exit(1);
        }
    };

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
        let config = Arc::clone(&state.config);
        tokio::spawn(async move {
            nodes::heartbeat::run_heartbeat_loop(registry, client, config).await;
        });
    }

    let app = app::build_router(state);

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
    if let Err(e) = tokio::signal::ctrl_c().await {
        tracing::error!("Impossible d'installer le gestionnaire Ctrl+C : {e}");
        // Sans gestionnaire, on ne déclenche pas d'arrêt : attente indéfinie.
        std::future::pending::<()>().await;
    }
}
