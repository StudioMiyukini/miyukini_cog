//! Système d'agents IA — Miyukini AI Studio.
//!
//! Chaque agent possède :
//! - Un identifiant unique + nom personnalisable
//! - Un system prompt spécialisé (rôle, personnalité, consignes)
//! - Un skill set (outils autorisés)
//! - Un niveau de sécurité
//! - Une préférence de modèle (rapide, créatif, analytique)

pub mod builtin;
pub mod custom;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// ── Types ────────────────────────────────────────────────────────────

/// Préférence de modèle pour un agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelPreference {
    /// Modèle rapide, petite taille — réponses courtes.
    Fast,
    /// Modèle équilibré — usage général.
    Balanced,
    /// Modèle analytique — raisonnement, code, données.
    Analytical,
    /// Modèle créatif — rédaction, marketing, communication.
    Creative,
    /// Le meilleur modèle disponible — tâches critiques.
    Best,
}

/// Niveau de sécurité d'un agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityLevel {
    /// Lecture seule, pas d'accès fichier/shell.
    ReadOnly = 0,
    /// Lecture + écriture fichiers dans le sandbox.
    Sandboxed = 1,
    /// Accès fichiers étendu + commandes whitelistées.
    Extended = 2,
    /// Accès complet (supervisé par Arianne).
    Full = 3,
}

/// Définition d'un agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDef {
    /// Identifiant unique (snake_case, ex: "alicia", "denis").
    pub id: String,
    /// Nom affiché (personnalisable par l'utilisateur).
    pub display_name: String,
    /// Rôle court (titre).
    pub role: String,
    /// Description détaillée du rôle.
    pub description: String,
    /// System prompt complet envoyé au LLM.
    pub system_prompt: String,
    /// Skills autorisés (IDs des skills).
    pub skill_ids: Vec<String>,
    /// Préférence de modèle.
    pub model_preference: ModelPreference,
    /// Niveau de sécurité.
    pub security_level: SecurityLevel,
    /// Est un agent built-in (non supprimable).
    pub is_builtin: bool,
    /// Icône/emoji.
    pub icon: String,
    /// Peut dispatcher vers d'autres agents.
    pub can_dispatch: bool,
}

/// Résumé public d'un agent (pour les endpoints GET).
#[derive(Debug, Clone, Serialize)]
pub struct AgentSummary {
    pub id: String,
    pub display_name: String,
    pub role: String,
    pub description: String,
    pub icon: String,
    pub skill_ids: Vec<String>,
    pub model_preference: ModelPreference,
    pub security_level: SecurityLevel,
    pub is_builtin: bool,
    pub can_dispatch: bool,
}

impl From<&AgentDef> for AgentSummary {
    fn from(agent: &AgentDef) -> Self {
        Self {
            id: agent.id.clone(),
            display_name: agent.display_name.clone(),
            role: agent.role.clone(),
            description: agent.description.clone(),
            icon: agent.icon.clone(),
            skill_ids: agent.skill_ids.clone(),
            model_preference: agent.model_preference,
            security_level: agent.security_level,
            is_builtin: agent.is_builtin,
            can_dispatch: agent.can_dispatch,
        }
    }
}

// ── Équipe d'agents ──────────────────────────────────────────────────

/// Une équipe d'agents assignée à un projet/tâche.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTeam {
    pub id: String,
    pub name: String,
    pub description: String,
    /// IDs des agents dans l'équipe.
    pub agent_ids: Vec<String>,
    /// ID de l'agent coordinateur (ex: "maria" ou "arianne").
    pub coordinator_id: String,
    /// Créé le.
    pub created_at: String,
}

/// Tâche assignée à une équipe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamTask {
    pub description: String,
    /// Contextes à inclure (IDs des ContextBase).
    #[serde(default)]
    pub context_ids: Vec<String>,
    /// Agent spécifique à qui assigner (optionnel, sinon le coordinateur dispatche).
    #[serde(default)]
    pub target_agent_id: Option<String>,
}

// ── Registre ─────────────────────────────────────────────────────────

/// Registre central des agents — contient built-in + custom.
#[derive(Debug, Clone)]
pub struct AgentRegistry {
    agents: Arc<RwLock<HashMap<String, AgentDef>>>,
    teams: Arc<RwLock<HashMap<String, AgentTeam>>>,
}

impl AgentRegistry {
    /// Crée un registre avec les agents built-in pré-chargés.
    pub fn new() -> Self {
        let mut agents = HashMap::new();
        for agent in builtin::builtin_agents() {
            agents.insert(agent.id.clone(), agent);
        }

        Self {
            agents: Arc::new(RwLock::new(agents)),
            teams: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Liste tous les agents (résumés).
    pub fn list_agents(&self) -> Vec<AgentSummary> {
        let agents = self.agents.read().unwrap();
        let mut list: Vec<AgentSummary> = agents.values().map(AgentSummary::from).collect();
        list.sort_by(|a, b| a.id.cmp(&b.id));
        list
    }

    /// Récupère un agent par ID.
    pub fn get_agent(&self, id: &str) -> Option<AgentDef> {
        let agents = self.agents.read().unwrap();
        agents.get(id).cloned()
    }

    /// Ajoute ou met à jour un agent custom.
    pub fn upsert_agent(&self, agent: AgentDef) {
        let mut agents = self.agents.write().unwrap();
        agents.insert(agent.id.clone(), agent);
    }

    /// Supprime un agent custom (refuse de supprimer un built-in).
    pub fn delete_agent(&self, id: &str) -> Result<(), &'static str> {
        let mut agents = self.agents.write().unwrap();
        if let Some(agent) = agents.get(id) {
            if agent.is_builtin {
                return Err("Impossible de supprimer un agent built-in");
            }
        }
        agents.remove(id);
        Ok(())
    }

    /// Renomme un agent (built-in ou custom).
    pub fn rename_agent(&self, id: &str, new_name: &str) -> Result<(), &'static str> {
        let mut agents = self.agents.write().unwrap();
        if let Some(agent) = agents.get_mut(id) {
            agent.display_name = new_name.to_string();
            Ok(())
        } else {
            Err("Agent introuvable")
        }
    }

    // ── Équipes ──────────────────────────────────────────────────────

    /// Crée une équipe.
    pub fn create_team(&self, team: AgentTeam) {
        let mut teams = self.teams.write().unwrap();
        teams.insert(team.id.clone(), team);
    }

    /// Liste les équipes.
    pub fn list_teams(&self) -> Vec<AgentTeam> {
        let teams = self.teams.read().unwrap();
        teams.values().cloned().collect()
    }

    /// Récupère une équipe.
    pub fn get_team(&self, id: &str) -> Option<AgentTeam> {
        let teams = self.teams.read().unwrap();
        teams.get(id).cloned()
    }

    /// Supprime une équipe.
    pub fn delete_team(&self, id: &str) {
        let mut teams = self.teams.write().unwrap();
        teams.remove(id);
    }
}
