use serde::{Deserialize, Serialize};

/// Mercenary archetype available for hire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MercType {
    RogueScout,    // Ranged, Act 1
    DesertWarrior, // Melee, Act 2
    IronWolf,      // Caster, Act 3
    BarbarianClan, // Melee, Act 5
}

/// Static definition of a hireable mercenary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MercDef {
    pub id: u32,
    pub name: String,
    pub merc_type: MercType,
    pub hire_cost: u32,
    pub base_hp: u32,
    pub base_damage: u32,
    /// Resurrection gold cost (scales with level).
    pub res_base_cost: u32,
}

/// Runtime state of an active mercenary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MercStatus {
    Alive,
    /// Merc died but can be resurrected.
    Ghost,
    /// Not hired.
    None,
}

/// A player's active mercenary slot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MercSlot {
    pub def: Option<MercDef>,
    pub status: MercStatus,
    pub level: u32,
    pub current_hp: u32,
}

impl MercSlot {
    #[must_use]
    pub fn empty() -> Self {
        Self { def: None, status: MercStatus::None, level: 0, current_hp: 0 }
    }

    /// Hire the given merc definition.
    pub fn hire(&mut self, def: MercDef) {
        self.current_hp = def.base_hp;
        self.level = 1;
        self.status = MercStatus::Alive;
        self.def = Some(def);
    }

    /// Kill the active merc (transition to Ghost).
    pub fn kill(&mut self) {
        if self.status == MercStatus::Alive {
            self.status = MercStatus::Ghost;
            self.current_hp = 0;
        }
    }

    /// Resurrect the ghost merc. Returns the gold cost paid.
    pub fn resurrect(&mut self) -> Option<u32> {
        if self.status != MercStatus::Ghost {
            return None;
        }
        let cost = self
            .def
            .as_ref()
            .map_or(0, |d| d.res_base_cost.saturating_mul(self.level));
        self.status = MercStatus::Alive;
        self.current_hp = self.def.as_ref().map_or(0, |d| d.base_hp);
        Some(cost)
    }

    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.status == MercStatus::Alive
    }
}

/// Act 1 available mercenaries.
#[must_use]
pub fn act1_mercs() -> Vec<MercDef> {
    vec![
        MercDef {
            id: 1,
            name: "Flavie".into(),
            merc_type: MercType::RogueScout,
            hire_cost: 200,
            base_hp: 150,
            base_damage: 20,
            res_base_cost: 100,
        },
        MercDef {
            id: 2,
            name: "Lysa".into(),
            merc_type: MercType::RogueScout,
            hire_cost: 250,
            base_hp: 180,
            base_damage: 25,
            res_base_cost: 120,
        },
    ]
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hire_sets_alive_status() {
        let mut slot = MercSlot::empty();
        slot.hire(act1_mercs().remove(0));
        assert!(slot.is_alive());
    }

    #[test]
    fn kill_transitions_to_ghost() {
        let mut slot = MercSlot::empty();
        slot.hire(act1_mercs().remove(0));
        slot.kill();
        assert_eq!(slot.status, MercStatus::Ghost);
    }

    #[test]
    fn resurrect_returns_cost_and_revives() {
        let mut slot = MercSlot::empty();
        slot.hire(act1_mercs().remove(0));
        slot.kill();
        let cost = slot.resurrect();
        assert!(cost.is_some());
        assert!(slot.is_alive());
    }

    #[test]
    fn resurrect_alive_merc_returns_none() {
        let mut slot = MercSlot::empty();
        slot.hire(act1_mercs().remove(0));
        assert!(slot.resurrect().is_none());
    }
}
