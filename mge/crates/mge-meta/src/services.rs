use serde::{Deserialize, Serialize};

/// A logical MMO-ready service boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceKind {
    /// Entry point: authentication, session, routing.
    Gateway,
    /// Realm state: character lists, world map, ladder.
    Realm,
    /// Live zone simulation: entities, physics, AI.
    Zone,
    /// Social graph: friends, guilds, messages, party.
    Social,
    /// Durable storage: characters, inventory, quests, config.
    Persistence,
}

impl ServiceKind {
    /// Human-readable description of the service's responsibility.
    #[must_use]
    pub fn description(self) -> &'static str {
        match self {
            Self::Gateway => "Authenticates players and routes requests to realm.",
            Self::Realm => "Manages character lists, world state, and ladder.",
            Self::Zone => "Runs zone simulation: entity state, AI, loot, transitions.",
            Self::Social => "Friends, guilds, party, chat, and social presence.",
            Self::Persistence => "Durable save for characters, inventories, quests, config.",
        }
    }

    /// Whether this service can run in a single-player embedded mode.
    #[must_use]
    pub fn embeddable(self) -> bool {
        matches!(self, Self::Zone | Self::Persistence)
    }
}

/// A declared service boundary with scaling metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBoundary {
    pub kind: ServiceKind,
    pub min_instances: u32,
    pub max_instances: u32,
    pub stateful: bool,
}

impl ServiceBoundary {
    #[must_use]
    pub fn new(kind: ServiceKind, stateful: bool) -> Self {
        Self { kind, min_instances: 1, max_instances: 1, stateful }
    }

    #[must_use]
    pub fn with_scale(mut self, min: u32, max: u32) -> Self {
        self.min_instances = min;
        self.max_instances = max;
        self
    }
}

/// Full MMO-ready service topology.
#[must_use]
pub fn mmo_service_topology() -> Vec<ServiceBoundary> {
    vec![
        ServiceBoundary::new(ServiceKind::Gateway, false).with_scale(2, 8),
        ServiceBoundary::new(ServiceKind::Realm, true).with_scale(1, 4),
        ServiceBoundary::new(ServiceKind::Zone, true).with_scale(1, 64),
        ServiceBoundary::new(ServiceKind::Social, false).with_scale(1, 4),
        ServiceBoundary::new(ServiceKind::Persistence, true).with_scale(1, 4),
    ]
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topology_has_five_services() {
        assert_eq!(mmo_service_topology().len(), 5);
    }

    #[test]
    fn zone_is_embeddable() {
        assert!(ServiceKind::Zone.embeddable());
        assert!(!ServiceKind::Gateway.embeddable());
    }

    #[test]
    fn gateway_scales_up() {
        let topo = mmo_service_topology();
        let gw = topo.iter().find(|s| s.kind == ServiceKind::Gateway).unwrap();
        assert!(gw.max_instances > 1);
    }

    #[test]
    fn all_services_have_descriptions() {
        for kind in [
            ServiceKind::Gateway,
            ServiceKind::Realm,
            ServiceKind::Zone,
            ServiceKind::Social,
            ServiceKind::Persistence,
        ] {
            assert!(!kind.description().is_empty());
        }
    }
}
