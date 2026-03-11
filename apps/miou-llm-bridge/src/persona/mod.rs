//! Système de personas TOML pour MAIA.
//!
//! Chaque persona définit le comportement complet de l'agent : modèle LLM préféré,
//! voix STT/TTS, mot de réveil, mode (standalone/pro), et politique de fallback.
//!
//! Profils embarqués (dans le binaire) : alicia, miou, sophie-guide.
//! Profil custom : fichier TOML sur disque chargé via --profile <path>.

use serde::{Deserialize, Serialize};

// ── Profils embarqués ─────────────────────────────────────────────────────

const PROFILE_ALICIA: &str = include_str!("profiles/alicia.toml");
const PROFILE_MIOU: &str = include_str!("profiles/miou.toml");
const PROFILE_SOPHIE_GUIDE: &str = include_str!("profiles/sophie-guide.toml");

// ── Types ────────────────────────────────────────────────────────────────

/// Configuration complète d'une persona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaConfig {
    /// Identifiant interne (ex: "alicia").
    pub name: String,
    /// Nom affiché (ex: "Alicia").
    pub display_name: String,
    /// Description courte.
    #[serde(default)]
    pub description: String,
    /// Mode de fonctionnement.
    pub mode: PersonaMode,
    /// Paramètres LLM.
    pub llm: PersonaLlm,
    /// Paramètres STT (Speech-to-Text).
    pub stt: PersonaStt,
    /// Paramètres TTS (Text-to-Speech).
    pub tts: PersonaTts,
    /// Mot de réveil.
    pub wake_word: PersonaWakeWord,
    /// Politique de fallback.
    pub fallback: PersonaFallback,
}

/// Mode de fonctionnement de la persona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaMode {
    /// "standalone" (isolation) ou "pro" (nœud TEAM).
    pub kind: PersonaModeKind,
    /// Mode isolation stricte — panic au démarrage si réseau configuré.
    #[serde(default)]
    pub isolated: bool,
}

/// Type de mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PersonaModeKind {
    /// Aucune connexion TEAM. Fonctionne en totale autonomie.
    Standalone,
    /// Enregistré auprès d'un hub TEAM — reçoit des tâches orchestrées.
    Pro,
}

impl std::fmt::Display for PersonaModeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standalone => write!(f, "standalone"),
            Self::Pro => write!(f, "pro"),
        }
    }
}

/// Paramètres LLM de la persona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaLlm {
    /// Patterns de modèles préférés (ordre de priorité décroissant).
    pub preferred_patterns: Vec<String>,
    /// Prompt système injecté dans chaque conversation.
    pub system_prompt: String,
    /// Température par défaut.
    #[serde(default = "default_temperature")]
    pub temperature: f64,
    /// Tokens maximum par défaut.
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    /// Active le thinking-mode Qwen3 (budget tokens).
    #[serde(default)]
    pub thinking_mode: bool,
}

fn default_temperature() -> f64 {
    0.7
}
fn default_max_tokens() -> u32 {
    2048
}

/// Paramètres STT (Speech-to-Text Whisper).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaStt {
    /// Code langue pour Whisper (ex: "fr", "en").
    #[serde(default = "default_language")]
    pub language: String,
    /// Prompt initial pour guider la transcription (vocabulaire COG).
    #[serde(default)]
    pub initial_prompt: String,
    /// Seuil de confiance minimum (0.0–1.0).
    /// En dessous → REJECT (transcription ignorée).
    #[serde(default = "default_confidence")]
    pub confidence_threshold: f64,
    /// Modèle Whisper à utiliser (ex: "whisper-small", "whisper-medium").
    #[serde(default = "default_whisper_model")]
    pub model: String,
    /// Beam size pour le décodage Whisper.
    #[serde(default = "default_beam_size")]
    pub beam_size: u32,
    /// Durée de silence pour couper l'enregistrement (ms).
    #[serde(default = "default_vad_silence_ms")]
    pub vad_silence_ms: u64,
    /// Padding audio avant/après la parole (ms).
    #[serde(default = "default_vad_padding_ms")]
    pub vad_padding_ms: u64,
}

fn default_language() -> String {
    "fr".into()
}
fn default_confidence() -> f64 {
    0.65
}
fn default_whisper_model() -> String {
    "whisper-medium".into()
}
fn default_beam_size() -> u32 {
    5
}
fn default_vad_silence_ms() -> u64 {
    800
}
fn default_vad_padding_ms() -> u64 {
    200
}

/// Paramètres TTS (Text-to-Speech Piper).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaTts {
    /// Identifiant de voix Piper (ex: "fr-siwis-medium").
    pub voice: String,
    /// Vitesse de synthèse (1.0 = normal).
    #[serde(default = "default_speed")]
    pub speed: f64,
    /// Taux d'échantillonnage audio (Hz).
    #[serde(default = "default_sample_rate")]
    pub sample_rate: u32,
}

fn default_speed() -> f64 {
    1.0
}
fn default_sample_rate() -> u32 {
    22050
}

/// Configuration du mot de réveil (wake word).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaWakeWord {
    /// Mot de réveil (ex: "Alicia").
    pub word: String,
    /// Sensibilité (0.0–1.0).
    #[serde(default = "default_sensitivity")]
    pub sensitivity: f64,
    /// Délai avant ré-activation après réponse (ms).
    #[serde(default = "default_reactivation_delay")]
    pub reactivation_delay_ms: u64,
}

fn default_sensitivity() -> f64 {
    0.5
}
fn default_reactivation_delay() -> u64 {
    1500
}

/// Politique de fallback si maia-llm est indisponible.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaFallback {
    /// Activer la réponse proto-IA minimale si aucun backend disponible.
    #[serde(default)]
    pub proto_ia_enabled: bool,
    /// Message de fallback affiché/lu si proto_ia_enabled.
    #[serde(default)]
    pub proto_ia_max_response: String,
}

// ── Chargement ───────────────────────────────────────────────────────────

/// Erreur de chargement de persona.
#[derive(Debug)]
pub enum PersonaError {
    NotFound(String),
    ParseError(String),
    IoError(String),
}

impl std::fmt::Display for PersonaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(n) => write!(f, "Persona '{n}' inconnue"),
            Self::ParseError(e) => write!(f, "Erreur de parse TOML : {e}"),
            Self::IoError(e) => write!(f, "Erreur lecture fichier : {e}"),
        }
    }
}

/// Charge une persona par nom (builtin) ou par chemin de fichier TOML.
///
/// Noms builtins : "alicia", "miou", "sophie-guide", "sophie_guide".
/// Si le nom ressemble à un chemin (contient '/' ou '\' ou finit par ".toml"),
/// charge le fichier directement.
pub fn load_persona(name_or_path: &str) -> Result<PersonaConfig, PersonaError> {
    // Détection chemin de fichier
    if name_or_path.ends_with(".toml")
        || name_or_path.contains('/')
        || name_or_path.contains('\\')
    {
        let content = std::fs::read_to_string(name_or_path)
            .map_err(|e| PersonaError::IoError(format!("{name_or_path}: {e}")))?;
        return parse_persona(&content);
    }

    // Profils builtins
    let toml_str = match name_or_path.to_lowercase().replace('_', "-").as_str() {
        "alicia" => PROFILE_ALICIA,
        "miou" => PROFILE_MIOU,
        "sophie-guide" | "sophie" => PROFILE_SOPHIE_GUIDE,
        other => return Err(PersonaError::NotFound(other.to_string())),
    };

    parse_persona(toml_str)
}

/// Parse un contenu TOML en PersonaConfig.
fn parse_persona(toml_str: &str) -> Result<PersonaConfig, PersonaError> {
    toml::from_str(toml_str).map_err(|e| PersonaError::ParseError(e.to_string()))
}

/// Retourne la liste des personas builtins disponibles.
pub fn builtin_personas() -> &'static [&'static str] {
    &["alicia", "miou", "sophie-guide"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_alicia() {
        let p = load_persona("alicia").expect("alicia parse");
        assert_eq!(p.name, "alicia");
        assert_eq!(p.mode.kind, PersonaModeKind::Standalone);
        assert!(!p.llm.preferred_patterns.is_empty());
        assert!(!p.llm.system_prompt.is_empty());
        assert_eq!(p.stt.language, "fr");
    }

    #[test]
    fn test_load_miou() {
        let p = load_persona("miou").expect("miou parse");
        assert_eq!(p.mode.kind, PersonaModeKind::Pro);
        assert!(p.llm.thinking_mode);
        assert!(!p.fallback.proto_ia_enabled);
    }

    #[test]
    fn test_load_sophie_guide() {
        let p = load_persona("sophie-guide").expect("sophie-guide parse");
        assert!(p.mode.isolated);
        assert_eq!(p.llm.max_tokens, 512);
        assert_eq!(p.stt.model, "whisper-small");
    }

    #[test]
    fn test_unknown_persona() {
        assert!(matches!(
            load_persona("unknown-persona"),
            Err(PersonaError::NotFound(_))
        ));
    }
}
