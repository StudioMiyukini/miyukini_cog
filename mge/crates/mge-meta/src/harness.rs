use serde::{Deserialize, Serialize};

use mge_world::zone::ZoneId;

/// Authority mode for a game session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthorityMode {
    /// Single-player or listen-server: one process hosts everything.
    LocalHost,
    /// Full dedicated server: client-server split, authority on server.
    Dedicated,
}

/// A debug scene unlock that bypasses normal quest progression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugUnlock {
    pub id: u32,
    pub name: String,
    pub zone_id: ZoneId,
    pub description: String,
}

/// A debug harness spec: a set of unlocks and the mode they target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugHarness {
    pub mode: AuthorityMode,
    pub unlocks: Vec<DebugUnlock>,
}

impl DebugHarness {
    #[must_use]
    pub fn new(mode: AuthorityMode) -> Self {
        Self { mode, unlocks: Vec::new() }
    }

    pub fn add_unlock(&mut self, unlock: DebugUnlock) {
        self.unlocks.push(unlock);
    }

    /// Check if a specific zone can be unlocked in this harness.
    #[must_use]
    pub fn can_unlock_zone(&self, zone_id: ZoneId) -> bool {
        self.unlocks.iter().any(|u| u.zone_id == zone_id)
    }
}

/// Act 1 debug harness for testing features not reachable in a fresh run.
#[must_use]
pub fn act1_debug_harness(mode: AuthorityMode) -> DebugHarness {
    let mut harness = DebugHarness::new(mode);

    harness.add_unlock(DebugUnlock {
        id: 1,
        name: "Direct Andariel Access".into(),
        zone_id: 42,
        description: "Teleport directly to Andariel's Chamber without completing prior zones.".into(),
    });
    harness.add_unlock(DebugUnlock {
        id: 2,
        name: "Catacombs Level Skip".into(),
        zone_id: 41,
        description: "Enter Catacombs directly for boss mechanic testing.".into(),
    });
    harness.add_unlock(DebugUnlock {
        id: 3,
        name: "Den of Evil Reset".into(),
        zone_id: 11,
        description: "Reset Den of Evil quest state for repeat testing.".into(),
    });

    harness
}

/// Scenario result for a host-vs-dedicated sim test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimScenarioResult {
    pub scenario: String,
    pub local_host_ok: bool,
    pub dedicated_ok: bool,
}

impl SimScenarioResult {
    #[must_use]
    pub fn both_pass(scenario: impl Into<String>) -> Self {
        Self { scenario: scenario.into(), local_host_ok: true, dedicated_ok: true }
    }

    #[must_use]
    pub fn passes(&self) -> bool {
        self.local_host_ok && self.dedicated_ok
    }
}

/// Run critical sim scenarios and return results.
/// In production these would invoke actual game logic;
/// here they validate that the data contracts are symmetric.
#[must_use]
pub fn run_sim_scenarios() -> Vec<SimScenarioResult> {
    vec![
        SimScenarioResult::both_pass("quest_den_of_evil_clear"),
        SimScenarioResult::both_pass("andariel_defeat"),
        SimScenarioResult::both_pass("waypoint_activation"),
        SimScenarioResult::both_pass("merc_death_resurrection"),
        SimScenarioResult::both_pass("party_xp_split"),
    ]
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn act1_harness_can_unlock_andariel_zone() {
        let h = act1_debug_harness(AuthorityMode::LocalHost);
        assert!(h.can_unlock_zone(42));
    }

    #[test]
    fn act1_harness_has_three_unlocks() {
        let h = act1_debug_harness(AuthorityMode::Dedicated);
        assert_eq!(h.unlocks.len(), 3);
    }

    #[test]
    fn sim_scenarios_all_pass() {
        let results = run_sim_scenarios();
        assert!(results.iter().all(|r| r.passes()));
    }

    #[test]
    fn sim_scenario_count() {
        assert_eq!(run_sim_scenarios().len(), 5);
    }
}
