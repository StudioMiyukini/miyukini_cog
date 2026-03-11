//! MAIA — Miyukini AI Agents — Service IA local pour l'écosystème COG.
//!
//! Orchestrateur d'agents IA — délègue l'inférence GGUF à maia-llm (HTTP).
//! Expose une API OpenAI-compatible enrichie avec :
//! - Client HTTP vers maia-llm (GGUF local) — moteur principal
//! - Proxy HTTP vers LM Studio / Ollama — backup
//! - 17 agents spécialisés avec tool calling
//! - Système de skills (fichiers, shell, web, services COG)
//! - Bases de contexte pour enrichir les prompts
//! - Dégradation graduée (maia-llm → upstream → proto-IA)
//!
//! Usage :
//!   maia                                # config depuis bridge.toml ou défauts
//!   MIYUKINI_DATA_DIR=./data maia

mod agents;
mod catalog;
mod config;
mod context;
mod fallback;
mod hardware;
mod health;
mod inference;
mod llm_api;
mod mode;
mod model_manager;
mod persona;
mod proxy;
mod recommend;
mod security;
mod skills;
mod team_client;
mod tools;
mod voice;

use agents::AgentRegistry;
use context::ContextRegistry;
use inference::{InferenceRouter, MaiaLlmClient, UpstreamBackend};
use model_manager::ModelManager;
use proxy::{proxy_router, ProxyState};
use security::{AuditLog, RateLimitConfig, RateLimiter, SecurityAuditLog};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "maia=info,tower_http=info".into()),
        )
        .init();

    let cfg = config::load_or_create();

    // Timestamp de démarrage (pour uptime)
    let started_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Chargement persona (--profile <nom|chemin.toml>)
    let active_persona = parse_profile_arg();

    // Parse et valide le mode (--mode standalone|pro, --isolated, --team <url>)
    let mode_config = mode::parse_mode_args();
    mode::validate_mode(&mode_config);
    let maia_mode = mode_config.mode;

    // Validation --isolated : log et vérifier cohérence persona
    if mode_config.isolated {
        tracing::info!("Mode isolation strict activé — aucun socket réseau sortant");
        if let Some(ref p) = active_persona {
            if !p.mode.isolated {
                tracing::warn!(
                    "Persona '{}' n'est pas configurée en mode isolated mais --isolated est actif",
                    p.name
                );
            }
        }
    }

    // Détection hardware au démarrage
    let hw = hardware::detect_hardware();

    // Initialisation des registres
    let agent_registry = AgentRegistry::new();
    let context_registry = ContextRegistry::new();
    let rate_limiter = RateLimiter::new(RateLimitConfig::default());
    let audit_log = AuditLog::new(10_000);
    let security_audit = SecurityAuditLog::new(5_000);

    // Client HTTP partagé (maia-llm + upstream)
    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .expect("reqwest client");

    // Client maia-llm (GGUF local — remplace NativeBackend llama-cpp-2)
    // maia-llm tourne en parallèle sur le port 11435 par défaut.
    let maia_llm_url = std::env::var("MAIA_LLM_URL")
        .unwrap_or_else(|_| "http://localhost:11435".into());
    let maia_client = MaiaLlmClient::new(
        maia_llm_url.clone(),
        cfg.auth_token.clone(),
        http_client.clone(),
    );
    let model_manager = ModelManager::new(&cfg.native.models_dir);

    // Backend upstream (LM Studio / Ollama — backup)
    let upstream_backend = UpstreamBackend::new(http_client.clone(), cfg.upstream_url.clone());
    let inference_router =
        InferenceRouter::new(maia_client, upstream_backend, cfg.native.prefer_native);

    // Auto-load : demander à maia-llm de charger le meilleur modèle disponible
    if cfg.native.auto_load {
        let local_models = model_manager.scan_models();
        if !local_models.is_empty() {
            let model_to_load = if let Some(ref default) = cfg.default_model {
                model_manager.find_model(default)
            } else {
                model_manager.recommend_model(hw.ram_total_mb)
            };

            if let Some(model) = model_to_load {
                tracing::info!("Auto-chargement via maia-llm : {}", model.filename);
                match inference_router.native.load_model(model.path.clone()).await {
                    Ok(name) => tracing::info!("Modèle chargé : {name}"),
                    Err(e) => tracing::warn!(
                        "Échec du chargement auto (maia-llm disponible ?) : {e}"
                    ),
                }
            }
        }
    }

    let agent_count = agent_registry.list_agents().len();
    let local_model_count = model_manager.scan_models().len();
    let native_model = inference_router.native.loaded_model_name();

    if let Some(ref p) = active_persona {
        tracing::info!(
            "Persona active : {} ({}, thinking={})",
            p.display_name,
            p.mode.kind,
            p.llm.thinking_mode
        );
    }

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
    if let Some(ref p) = active_persona {
        let mode_label = if p.mode.isolated {
            format!("{} (isolated)", p.mode.kind)
        } else {
            p.mode.kind.to_string()
        };
        tracing::info!("╠══════════════════════════════════════════════════╣");
        tracing::info!("║  Persona  : {:<37}║", p.display_name);
        tracing::info!("║  Mode     : {:<37}║", mode_label);
        tracing::info!("║  Wake     : {:<37}║", p.wake_word.word);
        tracing::info!("║  LLM pref : {:<37}║", p.llm.preferred_patterns.first().map(|s| s.as_str()).unwrap_or("-"));
    }
    tracing::info!("╚══════════════════════════════════════════════════╝");

    // Enregistrement TEAM (mode Pro uniquement)
    let mut team_registered = false;
    if maia_mode == mode::MaiaMode::Pro {
        if let Some(team_url) = mode::resolve_team_url(&mode_config) {
            let persona_name = active_persona.as_ref().map(|p| p.name.clone());
            let node = team_client::NodeInfo::new(
                hostname::get()
                    .map(|h| h.to_string_lossy().to_string())
                    .unwrap_or_else(|_| "maia-node".into()),
                format!("http://localhost:{}", cfg.bind_addr.split(':').last().unwrap_or("11434")),
                &hw,
                maia_mode,
                inference_router.native.loaded_model_name(),
                persona_name,
            );
            let client = team_client::TeamClient::new(http_client.clone(), team_url, node);
            team_registered = client.register().await;
        }
    }

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
        active_persona,
        maia_mode,
        started_at,
        team_registered,
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

/// Parse l'argument `--profile <nom|chemin>` depuis les args CLI.
/// Retourne None si absent, ou panic si le profil est invalide.
fn parse_profile_arg() -> Option<persona::PersonaConfig> {
    let args: Vec<String> = std::env::args().collect();
    let pos = args.iter().position(|a| a == "--profile")?;
    let name_or_path = args.get(pos + 1).unwrap_or_else(|| {
        eprintln!("Erreur : --profile requiert un argument (ex: alicia, miou, sophie-guide)");
        std::process::exit(1);
    });

    match persona::load_persona(name_or_path) {
        Ok(p) => {
            tracing::info!(
                "Persona chargée : {} ({})",
                p.display_name,
                name_or_path
            );
            Some(p)
        }
        Err(e) => {
            eprintln!("Erreur chargement persona '{}': {e}", name_or_path);
            eprintln!(
                "Personas disponibles : {}",
                persona::builtin_personas().join(", ")
            );
            std::process::exit(1);
        }
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    tracing::info!("Signal d'arrêt reçu");
}
