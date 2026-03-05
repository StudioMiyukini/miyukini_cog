//! Miyukini AI Studio — Service IA local pour l'écosystème COG.
//!
//! Moteur d'inférence GGUF natif + orchestrateur d'agents IA.
//! Expose une API OpenAI-compatible enrichie avec :
//! - Inférence GGUF native (llama-cpp-2) — moteur principal
//! - Proxy HTTP vers LM Studio / Ollama — backup
//! - 17 agents spécialisés avec tool calling
//! - Système de skills (fichiers, shell, web, services COG)
//! - Bases de contexte pour enrichir les prompts
//! - Dégradation graduée (natif → upstream → proto-IA)
//!
//! Usage :
//!   miou-llm-bridge                     # config depuis bridge.toml ou défauts
//!   MIYUKINI_DATA_DIR=./data miou-llm-bridge

mod agents;
mod catalog;
mod config;
mod context;
mod fallback;
mod hardware;
mod inference;
mod llm_api;
mod model_manager;
mod proxy;
mod recommend;
mod security;
mod skills;
mod tools;

use agents::AgentRegistry;
use context::ContextRegistry;
use inference::{InferenceRouter, NativeBackend, UpstreamBackend};
use model_manager::ModelManager;
use proxy::{proxy_router, ProxyState};
use security::{AuditLog, RateLimitConfig, RateLimiter, SecurityAuditLog};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "miou_llm_bridge=info,tower_http=info".into()),
        )
        .init();

    let cfg = config::load_or_create();

    // Détection hardware au démarrage
    let hw = hardware::detect_hardware();

    // Initialisation des registres
    let agent_registry = AgentRegistry::new();
    let context_registry = ContextRegistry::new();
    let rate_limiter = RateLimiter::new(RateLimitConfig::default());
    let audit_log = AuditLog::new(10_000);
    let security_audit = SecurityAuditLog::new(5_000);

    // Initialisation du moteur d'inférence natif
    let native_backend = NativeBackend::new(cfg.native.clone());
    let model_manager = ModelManager::new(&cfg.native.models_dir);

    // Client HTTP pour l'upstream (backup)
    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .expect("reqwest client");

    let upstream_backend = UpstreamBackend::new(http_client.clone(), cfg.upstream_url.clone());
    let inference_router =
        InferenceRouter::new(native_backend, upstream_backend, cfg.native.prefer_native);

    // Auto-load : charger le meilleur modèle GGUF disponible
    if cfg.native.auto_load {
        let local_models = model_manager.scan_models();
        if !local_models.is_empty() {
            // Préférer le modèle par défaut de la config, sinon le recommandé
            let model_to_load = if let Some(ref default) = cfg.default_model {
                model_manager.find_model(default)
            } else {
                model_manager.recommend_model(hw.ram_total_mb)
            };

            if let Some(model) = model_to_load {
                tracing::info!("Auto-chargement du modèle : {}", model.filename);
                match inference_router.native.load_model(model.path.clone()) {
                    Ok(name) => tracing::info!("Modèle chargé : {name}"),
                    Err(e) => tracing::warn!("Échec du chargement auto : {e}"),
                }
            }
        }
    }

    let agent_count = agent_registry.list_agents().len();
    let local_model_count = model_manager.scan_models().len();
    let native_model = inference_router.native.loaded_model_name();

    tracing::info!("╔══════════════════════════════════════════════════╗");
    tracing::info!(
        "║     Miyukini AI Studio v{}                ║",
        env!("CARGO_PKG_VERSION")
    );
    tracing::info!("╠══════════════════════════════════════════════════╣");
    tracing::info!("║  Listen   : {:<37}║", cfg.bind_addr);
    tracing::info!(
        "║  Upstream : {:<37}║",
        format!("{} (backup)", cfg.upstream_url)
    );
    if let Some(ref name) = native_model {
        tracing::info!("║  Natif    : {:<37}║", name);
    } else {
        tracing::info!(
            "║  Natif    : {:<37}║",
            format!("{local_model_count} GGUF disponibles (aucun chargé)")
        );
    }
    if cfg.auth_token.is_some() {
        tracing::info!("║  Auth     : {:<37}║", "Bearer token activé");
    }
    tracing::info!("╠══════════════════════════════════════════════════╣");
    tracing::info!("║  CPU      : {:<37}║", hw.cpu_name);
    tracing::info!(
        "║  Cores    : {:<37}║",
        format!("{} cores / {} threads", hw.cpu_cores, hw.cpu_threads)
    );
    tracing::info!(
        "║  RAM      : {:<37}║",
        format!("{} GB total", hw.ram_total_mb / 1024)
    );
    if let Some(ref gpu) = hw.gpu {
        tracing::info!("║  GPU      : {:<37}║", gpu.name);
        tracing::info!("║  VRAM     : {:<37}║", format!("{} MB", gpu.vram_mb));
    } else {
        tracing::info!("║  GPU      : {:<37}║", "None (CPU-only)");
    }
    tracing::info!("║  Tier     : {:<37}║", hw.tier.label());
    tracing::info!("╠══════════════════════════════════════════════════╣");
    tracing::info!(
        "║  Agents   : {:<37}║",
        format!("{agent_count} agents chargés")
    );
    tracing::info!(
        "║  Skills   : {:<37}║",
        format!(
            "{} skills disponibles",
            skills::SkillRegistry::list_skills().len()
        )
    );
    tracing::info!(
        "║  Mode     : {:<37}║",
        if native_model.is_some() {
            "Natif (GGUF)"
        } else {
            "Upstream (backup)"
        }
    );
    tracing::info!("╠══════════════════════════════════════════════════╣");
    let sec = &cfg.security;
    let origin_status = if !sec.allowed_origins.is_empty() {
        format!("{} origines", sec.allowed_origins.len())
    } else if sec.strict_user_agent {
        "User-Agent COG requis".into()
    } else {
        "localhost only (défaut)".into()
    };
    tracing::info!("║  Origins  : {:<37}║", origin_status);
    tracing::info!(
        "║  HMAC     : {:<37}║",
        if sec.hmac_secret.is_some() {
            "SHA-256 activé (anti-MITM)"
        } else {
            "désactivé"
        }
    );
    tracing::info!(
        "║  Encrypt  : {:<37}║",
        if sec.encryption_key.is_some() {
            "AES-256-GCM activé"
        } else {
            "désactivé"
        }
    );
    tracing::info!(
        "║  Auth all : {:<37}║",
        if sec.require_auth_all {
            "oui (Bearer requis partout)"
        } else {
            "non (proxy seulement)"
        }
    );
    tracing::info!("╚══════════════════════════════════════════════════╝");

    let state = ProxyState {
        client: http_client,
        upstream_url: cfg.upstream_url,
        auth_token: cfg.auth_token,
        log_requests: cfg.log_requests,
        hardware: hw,
        agent_registry,
        context_registry,
        rate_limiter,
        audit_log,
        inference: inference_router,
        model_manager,
        security_config: cfg.security,
        security_audit,
    };

    let app = proxy_router(state);

    let listener = tokio::net::TcpListener::bind(&cfg.bind_addr)
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Impossible d'écouter sur {} : {e}", cfg.bind_addr);
            std::process::exit(1);
        });

    tracing::info!("AI Studio prêt — Ctrl+C pour arrêter");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Erreur serveur : {e}");
        });

    tracing::info!("AI Studio arrêté");
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    tracing::info!("Signal d'arrêt reçu");
}
