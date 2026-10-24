//! @id     team_nodes_registry
//! @do     definir_node_mode
//! @role   orchestration
//! @layer  app
//! @human  team — module « nodes registry ».

//! Registre des nœuds MAIA — stocke l'état de chaque nœud enregistré.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

/// Mode de fonctionnement d'un nœud MAIA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeMode {
    Standalone,
    Pro,
}

/// Informations de base d'un nœud MAIA.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: String,
    pub name: String,
    pub maia_url: String,
    pub version: String,
    pub mode: NodeMode,
    pub hardware_tier: u8,
    pub cpu: String,
    pub ram_mb: u64,
    pub gpu: Option<String>,
    pub model_active: Option<String>,
    pub persona: Option<String>,
}

/// État de santé d'un nœud (réponse de GET /maia/health).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHealth {
    pub model: Option<String>,
    pub backend: Option<String>,
    pub hardware_tier: String,
    pub uptime_secs: u64,
    pub capabilities_agents: usize,
    pub capabilities_tts: bool,
}

/// Statut de disponibilité d'un nœud.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeStatus {
    /// Nœud actif et répondant.
    Online,
    /// Nœud n'a pas répondu depuis > stale_threshold.
    Stale,
    /// Nœud s'est désenregistré proprement.
    Offline,
}

impl std::fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Online => write!(f, "online"),
            Self::Stale => write!(f, "stale"),
            Self::Offline => write!(f, "offline"),
        }
    }
}

/// Entrée complète d'un nœud dans le registre.
#[derive(Debug, Clone)]
pub struct NodeEntry {
    pub info: NodeInfo,
    pub health: Option<NodeHealth>,
    pub status: NodeStatus,
    pub registered_at: Instant,
    pub last_seen: Instant,
    /// Clé publique qui a enregistré le nœud (requête signée) : seule autorisée
    /// à le mettre à jour ou le retirer. `None` : entrée interne (tests).
    pub signataire: Option<[u8; 32]>,
}

/// Refus d'enregistrement d'un nœud.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefusEnregistrement {
    /// Registre plein (C-13).
    Plein,
    /// Identifiant déjà enregistré par une autre clé.
    AutreSignataire,
}

impl NodeEntry {
    pub fn new(info: NodeInfo) -> Self {
        let now = Instant::now();
        Self {
            info,
            health: None,
            status: NodeStatus::Online,
            registered_at: now,
            last_seen: now,
            signataire: None,
        }
    }

    pub fn age(&self) -> Duration {
        self.last_seen.elapsed()
    }

    pub fn update_health(&mut self, health: NodeHealth) {
        self.health = Some(health);
        self.last_seen = Instant::now();
        self.status = NodeStatus::Online;
    }

    pub fn mark_stale(&mut self) {
        self.status = NodeStatus::Stale;
    }

    pub fn mark_offline(&mut self) {
        self.status = NodeStatus::Offline;
    }
}

/// Registre partagé des nœuds MAIA.
#[derive(Clone, Default)]
pub struct NodeRegistry {
    nodes: Arc<RwLock<HashMap<String, NodeEntry>>>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enregistre un nouveau nœud (ou met à jour si déjà présent).
    pub fn register(&self, info: NodeInfo) {
        let id = info.id.clone();
        let mut nodes = self.nodes.write().unwrap_or_else(std::sync::PoisonError::into_inner);
        let entry = nodes.entry(id).or_insert_with(|| NodeEntry::new(info.clone()));
        entry.info = info;
        entry.last_seen = Instant::now();
        entry.status = NodeStatus::Online;
    }

    /// Enregistre (ou met à jour) un nœud au nom de `signataire` : un identifiant
    /// déjà lié à une autre clé est refusé, comme un nouveau nœud au-delà de
    /// `max_nodes` (C-13).
    pub fn register_signed(
        &self,
        info: NodeInfo,
        max_nodes: usize,
        signataire: [u8; 32],
    ) -> Result<(), RefusEnregistrement> {
        let mut nodes = self.nodes.write().unwrap_or_else(std::sync::PoisonError::into_inner);
        match nodes.get(&info.id) {
            Some(e) if e.signataire != Some(signataire) => return Err(RefusEnregistrement::AutreSignataire),
            None if nodes.len() >= max_nodes => return Err(RefusEnregistrement::Plein),
            _ => {}
        }
        let id = info.id.clone();
        let entry = nodes.entry(id).or_insert_with(|| NodeEntry::new(info.clone()));
        entry.info = info;
        entry.signataire = Some(signataire);
        entry.last_seen = Instant::now();
        entry.status = NodeStatus::Online;
        Ok(())
    }

    /// Heartbeat d'un nœud au nom de `signataire`. `false` si le nœud est
    /// inconnu ou lié à une autre clé.
    pub fn heartbeat_signed(
        &self,
        id: &str,
        signataire: [u8; 32],
        health: NodeHealth,
        model_active: Option<String>,
        persona: Option<String>,
    ) -> bool {
        let mut nodes = self.nodes.write().unwrap_or_else(std::sync::PoisonError::into_inner);
        match nodes.get_mut(id) {
            Some(entry) if entry.signataire == Some(signataire) => {
                entry.update_health(health);
                entry.info.model_active = model_active;
                entry.info.persona = persona;
                true
            }
            _ => false,
        }
    }

    /// Retrait d'un nœud au nom de `signataire`. `false` si le nœud est inconnu
    /// ou lié à une autre clé.
    pub fn unregister_signed(&self, id: &str, signataire: [u8; 32]) -> bool {
        let mut nodes = self.nodes.write().unwrap_or_else(std::sync::PoisonError::into_inner);
        match nodes.get_mut(id) {
            Some(entry) if entry.signataire == Some(signataire) => {
                entry.mark_offline();
                true
            }
            _ => false,
        }
    }

    /// Met à jour le dernier heartbeat + health d'un nœud.
    pub fn heartbeat(&self, id: &str, health: NodeHealth, model_active: Option<String>, persona: Option<String>) {
        let mut nodes = self.nodes.write().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(entry) = nodes.get_mut(id) {
            entry.update_health(health);
            entry.info.model_active = model_active;
            entry.info.persona = persona;
        }
    }

    /// Marque un nœud comme offline (désenregistrement propre).
    pub fn unregister(&self, id: &str) {
        let mut nodes = self.nodes.write().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(entry) = nodes.get_mut(id) {
            entry.mark_offline();
        }
    }

    /// Marque les nœuds stale (pas vu depuis > threshold).
    pub fn mark_stale_nodes(&self, stale_threshold: Duration) {
        let mut nodes = self.nodes.write().unwrap_or_else(std::sync::PoisonError::into_inner);
        for entry in nodes.values_mut() {
            if entry.status == NodeStatus::Online && entry.age() > stale_threshold {
                entry.mark_stale();
            }
        }
    }

    /// Retourne tous les nœuds online (non-stale, non-offline).
    pub fn healthy_nodes(&self) -> Vec<NodeEntry> {
        self.nodes
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .values()
            .filter(|e| e.status == NodeStatus::Online)
            .cloned()
            .collect()
    }

    /// Retourne tous les nœuds (tous statuts).
    pub fn all_nodes(&self) -> Vec<NodeEntry> {
        self.nodes.read().unwrap_or_else(std::sync::PoisonError::into_inner).values().cloned().collect()
    }

    /// Retourne un nœud par ID.
    pub fn get(&self, id: &str) -> Option<NodeEntry> {
        self.nodes.read().unwrap_or_else(std::sync::PoisonError::into_inner).get(id).cloned()
    }

    /// Supprime les nœuds stale/offline non vus depuis > reap_threshold.
    /// Retourne le nombre de nœuds supprimés.
    pub fn reap_dead_nodes(&self, reap_threshold: Duration) -> usize {
        let mut nodes = self.nodes.write().unwrap_or_else(std::sync::PoisonError::into_inner);
        let before = nodes.len();
        nodes.retain(|_, entry| {
            if entry.status == NodeStatus::Online {
                return true;
            }
            // Garder les nœuds stale/offline récents (ex: pourrait revenir)
            entry.age() < reap_threshold
        });
        before - nodes.len()
    }

    /// Nombre de nœuds online.
    pub fn online_count(&self) -> usize {
        self.nodes
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .values()
            .filter(|e| e.status == NodeStatus::Online)
            .count()
    }

    /// Nombre total de nœuds dans le registre.
    pub fn total_count(&self) -> usize {
        self.nodes.read().unwrap_or_else(std::sync::PoisonError::into_inner).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(id: &str) -> NodeInfo {
        NodeInfo {
            id: id.into(),
            name: format!("Node {id}"),
            maia_url: format!("http://192.168.1.{id}:11434"),
            version: "0.1.0".into(),
            mode: NodeMode::Pro,
            hardware_tier: 2,
            cpu: "Test CPU".into(),
            ram_mb: 16_384,
            gpu: None,
            model_active: Some("qwen3-4b".into()),
            persona: Some("alicia".into()),
        }
    }

    #[test]
    fn test_register_and_list() {
        let registry = NodeRegistry::new();
        registry.register(make_node("1"));
        registry.register(make_node("2"));
        assert_eq!(registry.online_count(), 2);
    }

    #[test]
    fn test_unregister() {
        let registry = NodeRegistry::new();
        registry.register(make_node("1"));
        registry.unregister("1");
        let node = registry.get("1").unwrap();
        assert_eq!(node.status, NodeStatus::Offline);
    }

    #[test]
    fn test_mark_stale() {
        let registry = NodeRegistry::new();
        registry.register(make_node("1"));
        // Marquer stale avec seuil zéro (tous deviennent stale immédiatement)
        registry.mark_stale_nodes(Duration::from_millis(0));
        let node = registry.get("1").unwrap();
        assert_eq!(node.status, NodeStatus::Stale);
    }

    #[test]
    fn test_reap_dead_nodes() {
        let registry = NodeRegistry::new();
        registry.register(make_node("1"));
        registry.register(make_node("2"));
        // Marquer stale avec seuil zéro puis reap immédiatement
        registry.mark_stale_nodes(Duration::from_millis(0));
        let reaped = registry.reap_dead_nodes(Duration::from_millis(0));
        assert_eq!(reaped, 2);
        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn test_reap_keeps_recent_stale() {
        let registry = NodeRegistry::new();
        registry.register(make_node("1"));
        registry.mark_stale_nodes(Duration::from_millis(0));
        // Reap avec seuil très long — ne devrait rien supprimer
        let reaped = registry.reap_dead_nodes(Duration::from_secs(3600));
        assert_eq!(reaped, 0);
        assert_eq!(registry.total_count(), 1);
    }

    #[test]
    fn test_register_signed_bounded_and_bound_to_key() {
        let registry = NodeRegistry::new();
        let (a, b) = ([1_u8; 32], [2_u8; 32]);
        assert_eq!(registry.register_signed(make_node("1"), 1, a), Ok(()));
        // Registre plein : nouveau nœud refusé
        assert_eq!(registry.register_signed(make_node("2"), 1, a), Err(RefusEnregistrement::Plein));
        // Mise à jour par la même clé : acceptée ; par une autre : refusée
        assert_eq!(registry.register_signed(make_node("1"), 1, a), Ok(()));
        assert_eq!(registry.register_signed(make_node("1"), 1, b), Err(RefusEnregistrement::AutreSignataire));
        assert_eq!(registry.total_count(), 1);
        let health = || NodeHealth {
            model: None,
            backend: None,
            hardware_tier: String::new(),
            uptime_secs: 0,
            capabilities_agents: 0,
            capabilities_tts: false,
        };
        assert!(!registry.heartbeat_signed("1", b, health(), None, None));
        assert!(registry.heartbeat_signed("1", a, health(), None, None));
        assert!(!registry.unregister_signed("1", b));
        assert_eq!(registry.get("1").unwrap().status, NodeStatus::Online);
        assert!(registry.unregister_signed("1", a));
        assert_eq!(registry.get("1").unwrap().status, NodeStatus::Offline);
    }

    #[test]
    fn test_heartbeat_update() {
        let registry = NodeRegistry::new();
        registry.register(make_node("1"));
        let health = NodeHealth {
            model: Some("qwen3-8b".into()),
            backend: Some("native".into()),
            hardware_tier: "Light (Tier 2)".into(),
            uptime_secs: 120,
            capabilities_agents: 17,
            capabilities_tts: true,
        };
        registry.heartbeat("1", health, Some("qwen3-8b".into()), None);
        let node = registry.get("1").unwrap();
        assert_eq!(node.status, NodeStatus::Online);
        assert_eq!(node.info.model_active.as_deref(), Some("qwen3-8b"));
    }
}
