//! MAIA-LLM — Service d'inférence GGUF local.
//!
//! Usage : `maia-llm serve [--bind 0.0.0.0:11435] [--models-dir ./models] [--config maia-llm.toml]`

mod api;
mod auth;
mod config;
mod engine;
mod store;

use std::sync::Arc;
use std::time::SystemTime;

use engine::LlmEngine;
use store::ModelStore;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

/// État partagé entre tous les handlers axum.
#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<LlmEngine>,
    pub store: Arc<ModelStore>,
    pub config: Arc<config::MaiaLlmConfig>,
    /// Instant de démarrage (pour uptime dans /health).
    pub started_at: Arc<SystemTime>,
}

#[tokio::main]
async fn main() {
    // Logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "maia_llm=info,tower_http=warn".parse().unwrap()),
        )
        .init();

    // Config
    let cfg = parse_args_and_load_config();
    let bind_addr = cfg.bind_addr.clone();
    let models_dir = cfg.models_dir.clone();
    let auto_load = cfg.auto_load;

    tracing::info!("MAIA-LLM démarrage — bind={bind_addr} models={models_dir}");

    // Moteur + store
    let engine = Arc::new(LlmEngine::new((*cfg).clone()));
    let store = Arc::new(ModelStore::new(&models_dir));

    // Auto-load du meilleur modèle si activé
    if auto_load {
        let available_ram_mb = available_ram_mb();
        if let Some(model) = store.recommend_for_ram(available_ram_mb) {
            tracing::info!(
                "Auto-load : {} (RAM disponible ~{} MB)",
                model.filename,
                available_ram_mb
            );
            let eng = engine.clone();
            let path = model.path.clone();
            tokio::task::spawn_blocking(move || match eng.load_model(path) {
                Ok(info) => tracing::info!("Modèle auto-chargé : {}", info.name),
                Err(e) => tracing::warn!("Auto-load échoué : {e}"),
            })
            .await
            .ok();
        } else {
            tracing::info!("Auto-load : aucun modèle disponible dans {models_dir}");
        }
    }

    let state = AppState {
        engine,
        store,
        config: cfg,
        started_at: Arc::new(SystemTime::now()),
    };

    // Router
    // Note: Extension<Option<String>> injectée ici pour le middleware auth
    let app = api::build_router(state.clone())
        .layer(axum::Extension(state.config.auth_key.clone()))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Impossible de binder l'adresse");

    tracing::info!("MAIA-LLM en écoute sur http://{bind_addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Erreur serveur");
}

/// Parse les arguments CLI et charge la config TOML.
/// Priorité : CLI args > TOML > défauts.
fn parse_args_and_load_config() -> Arc<config::MaiaLlmConfig> {
    let args: Vec<String> = std::env::args().collect();
    let mut cfg = config::load_or_create();

    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--bind" | "-b" => {
                if let Some(v) = args.get(i + 1) {
                    cfg.bind_addr = v.clone();
                    i += 2;
                    continue;
                }
            }
            "--models-dir" | "-m" => {
                if let Some(v) = args.get(i + 1) {
                    cfg.models_dir = v.clone();
                    i += 2;
                    continue;
                }
            }
            "--gpu-layers" => {
                if let Some(v) = args.get(i + 1) {
                    if let Ok(n) = v.parse::<i32>() {
                        cfg.gpu_layers = n;
                    }
                    i += 2;
                    continue;
                }
            }
            "--auth-key" => {
                if let Some(v) = args.get(i + 1) {
                    cfg.auth_key = Some(v.clone());
                    i += 2;
                    continue;
                }
            }
            "--no-auto-load" => {
                cfg.auto_load = false;
                i += 1;
                continue;
            }
            _ => {}
        }
        i += 1;
    }

    Arc::new(cfg)
}

/// RAM disponible en Mo (sysinfo — approximatif, pas de dépendance lourde ici).
fn available_ram_mb() -> u64 {
    // Lecture simple via /proc/meminfo sur Linux ou valeur par défaut
    #[cfg(target_os = "linux")]
    {
        if let Ok(s) = std::fs::read_to_string("/proc/meminfo") {
            for line in s.lines() {
                if line.starts_with("MemAvailable:") {
                    if let Some(kb) = line.split_whitespace().nth(1) {
                        if let Ok(n) = kb.parse::<u64>() {
                            return n / 1024;
                        }
                    }
                }
            }
        }
    }
    // Fallback conservateur
    8 * 1024
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Impossible d'installer le handler CTRL+C");
    tracing::info!("Signal d'arrêt reçu — shutdown gracieux...");
}
