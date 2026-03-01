//! Système de Skills — capacités exécutables par les agents.
//!
//! Chaque skill est un outil que les agents peuvent invoquer pour interagir
//! avec le monde réel : fichiers, commandes, web, services COG.

pub mod cog_services;
pub mod file_ops;
pub mod shell_exec;
pub mod web_fetch;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Types ────────────────────────────────────────────────────────────

/// Résultat d'exécution d'un skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillResult {
    /// Succès ou échec.
    pub success: bool,
    /// Données de sortie (format dépend du skill).
    pub output: serde_json::Value,
    /// Message d'erreur si échec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Durée d'exécution en ms.
    pub duration_ms: u64,
}

/// Métadonnées d'un skill (pour l'API GET /v1/skills).
#[derive(Debug, Clone, Serialize)]
pub struct SkillInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    /// Paramètres acceptés (nom → description).
    pub parameters: HashMap<String, String>,
    /// Niveau de sécurité minimum requis.
    pub min_security_level: &'static str,
}

/// Requête d'exécution d'un skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExecRequest {
    /// Paramètres du skill (clé-valeur).
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
    /// ID de l'agent qui exécute (pour vérifier les permissions).
    #[serde(default)]
    pub agent_id: Option<String>,
}

// ── Registre ─────────────────────────────────────────────────────────

/// Registre central des skills disponibles.
pub struct SkillRegistry;

impl SkillRegistry {
    /// Liste tous les skills disponibles avec leurs métadonnées.
    pub fn list_skills() -> Vec<SkillInfo> {
        vec![
            file_ops::info(),
            shell_exec::info(),
            web_fetch::info(),
            cog_services::info(),
        ]
    }

    /// Exécute un skill par son ID.
    pub async fn execute(skill_id: &str, params: &HashMap<String, serde_json::Value>) -> SkillResult {
        let start = std::time::Instant::now();

        let result = match skill_id {
            "file_ops" => file_ops::execute(params).await,
            "shell_exec" => shell_exec::execute(params).await,
            "web_fetch" => web_fetch::execute(params).await,
            "cog_services" => cog_services::execute(params).await,
            _ => SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(format!("Skill inconnu : {skill_id}")),
                duration_ms: 0,
            },
        };

        let duration_ms = start.elapsed().as_millis() as u64;
        SkillResult { duration_ms, ..result }
    }
}
