//! Sélection de nœud pour une tâche — routage vers le meilleur nœud disponible.

use crate::nodes::registry::{NodeEntry, NodeRegistry, NodeStatus};

/// Critères de sélection d'un nœud.
#[derive(Debug, Clone, Default)]
pub struct NodeSelector {
    /// Tier hardware minimum requis (1-5).
    pub min_tier: Option<u8>,
    /// Pattern de modèle attendu (ex: "qwen3-4b").
    pub model_pattern: Option<String>,
    /// Persona attendue.
    pub persona: Option<String>,
}

impl NodeSelector {
    pub fn any() -> Self {
        Self::default()
    }

    pub fn with_min_tier(tier: u8) -> Self {
        Self {
            min_tier: Some(tier),
            ..Default::default()
        }
    }

    pub fn with_model(pattern: impl Into<String>) -> Self {
        Self {
            model_pattern: Some(pattern.into()),
            ..Default::default()
        }
    }
}

/// Sélectionne le meilleur nœud pour les critères donnés.
///
/// Stratégie :
/// 1. Filtrer les nœuds Online.
/// 2. Appliquer les critères (tier, modèle).
/// 3. Préférer les nœuds avec le modèle déjà chargé.
/// 4. En cas d'égalité, préférer le tier le plus élevé.
pub fn select_node(registry: &NodeRegistry, selector: &NodeSelector) -> Option<NodeEntry> {
    let mut candidates: Vec<NodeEntry> = registry
        .healthy_nodes()
        .into_iter()
        .filter(|n| {
            // Filtre tier minimum
            if let Some(min_tier) = selector.min_tier {
                if n.info.hardware_tier < min_tier {
                    return false;
                }
            }
            // Filtre modèle
            if let Some(ref pattern) = selector.model_pattern {
                let model_match = n
                    .info
                    .model_active
                    .as_deref()
                    .map(|m| m.to_lowercase().contains(&pattern.to_lowercase()))
                    .unwrap_or(false);
                if !model_match {
                    // Accepter quand même — le modèle pourra être chargé
                    // mais on lui donnera une priorité plus basse
                }
            }
            true
        })
        .collect();

    if candidates.is_empty() {
        return None;
    }

    // Trier : modèle chargé > tier élevé
    candidates.sort_by(|a, b| {
        let a_model_match = selector.model_pattern.as_ref().map(|p| {
            a.info
                .model_active
                .as_deref()
                .map(|m| m.to_lowercase().contains(&p.to_lowercase()))
                .unwrap_or(false)
        }).unwrap_or(true);

        let b_model_match = selector.model_pattern.as_ref().map(|p| {
            b.info
                .model_active
                .as_deref()
                .map(|m| m.to_lowercase().contains(&p.to_lowercase()))
                .unwrap_or(false)
        }).unwrap_or(true);

        // Modèle chargé en premier
        b_model_match.cmp(&a_model_match)
            // Puis tier le plus élevé
            .then(b.info.hardware_tier.cmp(&a.info.hardware_tier))
    });

    candidates.into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::registry::{NodeInfo, NodeMode};

    fn make_node(id: &str, tier: u8, model: Option<&str>) -> NodeInfo {
        NodeInfo {
            id: id.into(),
            name: id.into(),
            maia_url: format!("http://192.168.1.{id}:11434"),
            version: "0.1.0".into(),
            mode: NodeMode::Pro,
            hardware_tier: tier,
            cpu: "CPU".into(),
            ram_mb: 16_384,
            gpu: None,
            model_active: model.map(|m| m.into()),
            persona: None,
        }
    }

    #[test]
    fn test_select_any_node() {
        let registry = NodeRegistry::new();
        registry.register(make_node("1", 2, Some("qwen3-4b")));
        registry.register(make_node("2", 3, None));

        let selected = select_node(&registry, &NodeSelector::any());
        assert!(selected.is_some());
    }

    #[test]
    fn test_select_min_tier() {
        let registry = NodeRegistry::new();
        registry.register(make_node("low", 1, None));
        registry.register(make_node("high", 4, Some("qwen3-14b")));

        let selected = select_node(&registry, &NodeSelector::with_min_tier(3));
        assert!(selected.is_some());
        assert!(selected.unwrap().info.hardware_tier >= 3);
    }

    #[test]
    fn test_select_prefers_loaded_model() {
        let registry = NodeRegistry::new();
        registry.register(make_node("no-model", 3, None));
        registry.register(make_node("has-model", 3, Some("qwen3-4b")));

        let selected = select_node(&registry, &NodeSelector::with_model("qwen3-4b"));
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().info.id, "has-model");
    }

    #[test]
    fn test_select_no_nodes() {
        let registry = NodeRegistry::new();
        let selected = select_node(&registry, &NodeSelector::any());
        assert!(selected.is_none());
    }
}
