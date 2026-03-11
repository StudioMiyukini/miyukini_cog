//! Endpoint GET /maia/health — état complet du service MAIA.
//!
//! Retourne :
//! - Modèle LLM actif (nom, backend)
//! - Tier hardware détecté
//! - Persona active (si configurée)
//! - Mode de fonctionnement (standalone/pro)
//! - Capacités disponibles (STT, TTS, voice, agents)
//! - Uptime depuis le démarrage

use axum::{extract::State, response::IntoResponse, Json};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::mode::MaiaMode;
use crate::proxy::ProxyState;

/// Réponse de GET /maia/health.
#[derive(Debug, Serialize)]
pub struct MaiaHealth {
    /// Nom du service.
    pub service: &'static str,
    /// Version du binaire.
    pub version: &'static str,
    /// Mode de fonctionnement.
    pub mode: String,
    /// LLM actif.
    pub llm: LlmStatus,
    /// Tier hardware.
    pub hardware_tier: String,
    /// Numéro de tier (1-5).
    pub hardware_tier_number: u8,
    /// Persona active (None si aucune).
    pub persona: Option<PersonaStatus>,
    /// Capacités disponibles.
    pub capabilities: Capabilities,
    /// Uptime en secondes.
    pub uptime_secs: u64,
    /// Timestamp UTC (epoch secondes).
    pub timestamp: u64,
}

#[derive(Debug, Serialize)]
pub struct LlmStatus {
    /// Modèle actif dans maia-llm (None si aucun).
    pub model: Option<String>,
    /// Backend actif ("native" | "upstream" | None).
    pub backend: Option<String>,
    /// maia-llm joignable avec modèle chargé.
    pub model_loaded: bool,
    /// Nom du modèle (alias de model pour le frontend).
    pub model_name: Option<String>,
    /// Upstream (LM Studio / Ollama) joignable en ce moment.
    pub upstream_available: bool,
    /// URL de maia-llm.
    pub maia_llm_url: String,
    /// URL upstream backup.
    pub upstream_url: String,
}

#[derive(Debug, Serialize)]
pub struct PersonaStatus {
    pub name: String,
    pub display_name: String,
    pub mode: String,
    pub wake_word: String,
    pub llm_preferred: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Capabilities {
    /// STT disponible (maia-stt compilé avec feature whisper).
    pub stt: bool,
    /// TTS disponible (piper dans le PATH).
    pub tts: bool,
    /// Pipeline voix actif.
    pub voice: bool,
    /// Nombre d'agents chargés.
    pub agents: usize,
    /// Nombre de skills disponibles.
    pub skills: usize,
    /// Enregistré auprès du hub TEAM (mode pro uniquement).
    pub team_registered: bool,
}

/// Handler GET /maia/health.
pub async fn health_handler(State(state): State<ProxyState>) -> impl IntoResponse {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let uptime_secs = now.saturating_sub(state.started_at);

    let native_model = state.inference.native.loaded_model_name();
    let upstream_available = state.inference.upstream.is_available().await;
    let backend = if native_model.is_some() {
        Some("native".to_string())
    } else if upstream_available {
        Some("upstream".to_string())
    } else {
        None
    };

    let persona = state.active_persona.as_ref().map(|p| PersonaStatus {
        name: p.name.clone(),
        display_name: p.display_name.clone(),
        mode: p.mode.kind.to_string(),
        wake_word: p.wake_word.word.clone(),
        llm_preferred: p.llm.preferred_patterns.first().cloned(),
    });

    let agents_count = state.agent_registry.list_agents().len();
    let skills_count = crate::skills::SkillRegistry::list_skills().len();

    let tts_available = check_piper_available();

    let health = MaiaHealth {
        service: "maia",
        version: env!("CARGO_PKG_VERSION"),
        mode: state.maia_mode.to_string(),
        llm: LlmStatus {
            model_loaded: native_model.is_some(),
            model_name: native_model.clone(),
            model: native_model,
            backend,
            upstream_available,
            maia_llm_url: state.inference.native.base_url.clone(),
            upstream_url: state.upstream_url.clone(),
        },
        hardware_tier: state.hardware.tier.label().to_string(),
        hardware_tier_number: state.hardware.tier.number(),
        persona,
        capabilities: Capabilities {
            stt: false, // activé en S1-E08 quand maia-stt est intégré
            tts: tts_available,
            voice: false, // activé quand VoicePipeline est démarré
            agents: agents_count,
            skills: skills_count,
            team_registered: state.team_registered,
        },
        uptime_secs,
        timestamp: now,
    };

    Json(health)
}

/// Vérifie si le binaire `piper` est disponible dans le PATH.
fn check_piper_available() -> bool {
    std::process::Command::new("piper")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
