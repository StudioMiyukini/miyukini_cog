//! @id     maia_llm_main
//! @do     declarer_module_api
//! @role   orchestration
//! @layer  app
//! @human  maia-llm — module « main ».

//! MAIA-LLM — Service d'inférence GGUF local.
//!
//! Usage : `maia-llm serve [--bind 127.0.0.1:11435] [--models-dir ./models] [--config maia-llm.toml]`
//!
//! EX-26 : seuls les modèles accompagnés d'un manifeste signé `X.gguf.sceau`
//! (ancres `model_trust_anchors`) sont chargés — voir la crate `maia-modeles`.
//! Outillage : `maia-llm cle-dev --sortie <f>`, `maia-llm sceller-modele …`,
//! `maia-llm emballer-modele …` (artefact `MMIA`), `maia-llm installer-modele …`
//! (installateur du canal) et `maia-llm banc …` (banc d'essai).

// C-10 : aucun unwrap/expect/panic hors tests dans le code exposé au réseau.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

mod api;
mod auth;
mod banc;
mod config;
mod engine;
mod materiel;
mod store;

use std::sync::Arc;
use std::time::SystemTime;

use engine::LlmEngine;
use maia_modeles as provenance;
use store::ModelStore;
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
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("maia_llm=info,tower_http=warn")),
        )
        .init();

    // Outillage de signature des modèles (EX-26) : sous-commandes hors service.
    let args: Vec<String> = std::env::args().collect();
    if let Some(r) = provenance::executer_outil(&args) {
        match r {
            Ok(msg) => {
                println!("{msg}");
                return;
            }
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(2);
            }
        }
    }

    // Banc d'essai (au premier plan, hors service).
    if let Some(r) = banc::executer(&args, derogation_du_binaire(&args)) {
        match r {
            Ok(json) => {
                println!("{json}");
                return;
            }
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(2);
            }
        }
    }

    // Config
    let cfg = parse_args_and_load_config();
    // Fail-closed : clés épinglées bien formées, pas de bind hors loopback sans appelant.
    if let Err(msg) = config::validate_bind(&cfg) {
        tracing::error!("{msg}");
        eprintln!("{msg}");
        std::process::exit(1);
    }
    if cfg.auth_key.is_some() {
        tracing::warn!("`auth_key` est obsolète et ignorée : épinglez les clés des appelants (appelants_inference / appelants_modeles)");
    }
    let gardes = match config::cles_appelants(&cfg) {
        Ok((inference, modeles)) => {
            if inference.is_empty() {
                tracing::warn!("Aucun appelant épinglé pour /v1/* : requêtes refusées (503)");
            }
            if modeles.is_empty() {
                tracing::warn!("Aucun appelant épinglé pour /maia/llm/models* : requêtes refusées (503)");
            }
            auth::Gardes::new(inference, modeles)
        }
        Err(e) => {
            eprintln!("Refus de démarrer : {e}");
            std::process::exit(1);
        }
    };
    let bind_addr = cfg.bind_addr.clone();
    let models_dir = cfg.models_dir.clone();
    let auto_load = cfg.auto_load;

    tracing::info!("MAIA-LLM démarrage — bind={bind_addr} models={models_dir}");

    // Politique de provenance des modèles (EX-26) — fail-closed : une ancre
    // mal formée empêche le démarrage.
    let ancres = match provenance::ancres_depuis_hex(&cfg.model_trust_anchors) {
        Ok(a) => a,
        Err(e) => {
            tracing::error!("{e}");
            eprintln!("Refus de démarrer : {e}");
            std::process::exit(1);
        }
    };
    let politique = provenance::PolitiqueModeles::new(ancres, derogation_du_binaire(&args));

    // Moteur + store
    let engine = match LlmEngine::new((*cfg).clone(), politique) {
        Ok(e) => Arc::new(e),
        Err(e) => {
            tracing::error!("{e}");
            eprintln!("Refus de démarrer : {e}");
            std::process::exit(1);
        }
    };
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

    // Router (routes protégées : requêtes signées par un appelant épinglé)
    let app = api::build_router(state.clone(), gardes)
        .layer(config::build_cors(&state.config.cors_allowed_origins))
        .layer(TraceLayer::new_for_http());

    let listener = match tokio::net::TcpListener::bind(&bind_addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("Impossible de binder l'adresse {bind_addr} : {e}");
            eprintln!("Refus de démarrer : impossible de binder {bind_addr} : {e}");
            std::process::exit(1);
        }
    };

    tracing::info!("MAIA-LLM en écoute sur http://{bind_addr}");

    if let Err(e) = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        tracing::error!("Erreur serveur : {e}");
        std::process::exit(1);
    }
}

/// Dérogation « modèles non signés » (EX-26) : décidée **ici**, avec le profil
/// de compilation de ce binaire (`cfg!(debug_assertions)`), jamais par la
/// bibliothèque `maia-modeles`. Ignorée (et journalisée) dans un binaire de release.
fn derogation_du_binaire(args: &[String]) -> bool {
    provenance::derogation_effective(provenance::derogation_demandee(args), cfg!(debug_assertions))
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
                    cfg.bind_addr.clone_from(v);
                    i += 2;
                    continue;
                }
            }
            "--models-dir" | "-m" => {
                if let Some(v) = args.get(i + 1) {
                    cfg.models_dir.clone_from(v);
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
                eprintln!(
                    "--auth-key est retirée (lot 2 : plus de secret partagé). Épinglez la clé \
                     publique de chaque appelant dans `appelants_inference` / `appelants_modeles`."
                );
                std::process::exit(2);
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
    if let Err(e) = tokio::signal::ctrl_c().await {
        // Pas de panique (C-10) : sans gestionnaire, l'arrêt gracieux est
        // indisponible mais le signal par défaut du système arrête le processus.
        tracing::error!("Impossible d'installer le handler CTRL+C : {e}");
        std::future::pending::<()>().await;
    }
    tracing::info!("Signal d'arrêt reçu — shutdown gracieux...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derogation_seulement_dans_un_binaire_de_debug() {
        let args = vec!["maia-llm".to_string(), provenance::OPTION_DEROGATION.to_string()];
        assert_eq!(derogation_du_binaire(&args), cfg!(debug_assertions));
    }
}
