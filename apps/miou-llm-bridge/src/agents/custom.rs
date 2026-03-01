//! Agents personnalisés — CRUD d'agents définis par l'utilisateur.
//!
//! Persistés en JSON dans le répertoire de données du bridge.

use serde::{Deserialize, Serialize};

use super::{AgentDef, ModelPreference, SecurityLevel};

/// Requête de création d'un agent custom.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    /// Identifiant unique (snake_case). Généré si absent.
    #[serde(default)]
    pub id: String,
    /// Nom affiché.
    pub display_name: String,
    /// Rôle court.
    pub role: String,
    /// Description détaillée.
    #[serde(default)]
    pub description: String,
    /// System prompt.
    pub system_prompt: String,
    /// Skills autorisés.
    #[serde(default)]
    pub skill_ids: Vec<String>,
    /// Préférence de modèle.
    #[serde(default = "default_model_pref")]
    pub model_preference: ModelPreference,
    /// Niveau de sécurité.
    #[serde(default = "default_security")]
    pub security_level: SecurityLevel,
    /// Icône/emoji.
    #[serde(default = "default_icon")]
    pub icon: String,
    /// Peut dispatcher vers d'autres agents.
    #[serde(default)]
    pub can_dispatch: bool,
}

fn default_model_pref() -> ModelPreference {
    ModelPreference::Balanced
}
fn default_security() -> SecurityLevel {
    SecurityLevel::Sandboxed
}
fn default_icon() -> String {
    "🤖".into()
}

/// Requête de mise à jour partielle d'un agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAgentRequest {
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub system_prompt: Option<String>,
    #[serde(default)]
    pub skill_ids: Option<Vec<String>>,
    #[serde(default)]
    pub model_preference: Option<ModelPreference>,
    #[serde(default)]
    pub security_level: Option<SecurityLevel>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub can_dispatch: Option<bool>,
}

impl CreateAgentRequest {
    /// Convertit en `AgentDef`.
    pub fn into_agent_def(self) -> AgentDef {
        let id = if self.id.is_empty() {
            slugify(&self.display_name)
        } else {
            self.id
        };

        AgentDef {
            id,
            display_name: self.display_name,
            role: self.role,
            description: self.description,
            system_prompt: self.system_prompt,
            skill_ids: self.skill_ids,
            model_preference: self.model_preference,
            security_level: self.security_level,
            is_builtin: false,
            icon: self.icon,
            can_dispatch: self.can_dispatch,
        }
    }
}

impl UpdateAgentRequest {
    /// Applique les modifications sur un `AgentDef` existant.
    pub fn apply_to(self, agent: &mut AgentDef) {
        if let Some(name) = self.display_name {
            agent.display_name = name;
        }
        if let Some(role) = self.role {
            agent.role = role;
        }
        if let Some(desc) = self.description {
            agent.description = desc;
        }
        if let Some(prompt) = self.system_prompt {
            agent.system_prompt = prompt;
        }
        if let Some(skills) = self.skill_ids {
            agent.skill_ids = skills;
        }
        if let Some(pref) = self.model_preference {
            agent.model_preference = pref;
        }
        if let Some(sec) = self.security_level {
            agent.security_level = sec;
        }
        if let Some(icon) = self.icon {
            agent.icon = icon;
        }
        if let Some(dispatch) = self.can_dispatch {
            agent.can_dispatch = dispatch;
        }
    }
}

/// Convertit un nom en slug snake_case.
fn slugify(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_lowercase().next().unwrap_or('_')
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}
