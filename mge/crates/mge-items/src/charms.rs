use serde::{Deserialize, Serialize};

use mge_gameplay::stats::{Modifier, StatKind};

/// Charm inventory size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharmSize {
    /// 1×1 inventory cells.
    Small,
    /// 1×2 inventory cells.
    Large,
    /// 1×3 inventory cells.
    Grand,
}

impl CharmSize {
    /// Inventory grid dimensions (columns, rows).
    pub fn cells(self) -> (u8, u8) {
        match self {
            Self::Small => (1, 1),
            Self::Large => (1, 2),
            Self::Grand => (1, 3),
        }
    }

    /// Maximum number of random modifiers this charm size may carry.
    pub fn max_mods(self) -> u8 {
        match self {
            Self::Small => 1,
            Self::Large => 2,
            Self::Grand => 3,
        }
    }
}

/// A passive charm that is active while kept in the main inventory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charm {
    pub uid: u64,
    pub size: CharmSize,
    pub name: String,
    /// Passive stat modifiers active while this charm is in the inventory.
    pub mods: Vec<(StatKind, Modifier)>,
}

impl Charm {
    pub fn new(uid: u64, size: CharmSize, name: impl Into<String>) -> Self {
        Self { uid, size, name: name.into(), mods: Vec::new() }
    }

    /// Add a mod. Returns false if the charm is already at capacity.
    pub fn add_mod(&mut self, stat: StatKind, modifier: Modifier) -> bool {
        if self.mods.len() >= usize::from(self.size.max_mods()) {
            return false;
        }
        self.mods.push((stat, modifier));
        true
    }
}

/// Tracks charms in the active inventory zone (mods from these charms are applied).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharmBag {
    pub charms: Vec<Charm>,
}

impl CharmBag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, charm: Charm) {
        self.charms.push(charm);
    }

    pub fn remove(&mut self, uid: u64) {
        self.charms.retain(|c| c.uid != uid);
    }

    /// Collect all active modifier pairs from every charm in the bag.
    pub fn active_mods(&self) -> Vec<(StatKind, Modifier)> {
        self.charms.iter().flat_map(|c| c.mods.iter().copied()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_charm_max_one_mod() {
        let mut charm = Charm::new(1, CharmSize::Small, "Small Charm");
        assert!(charm.add_mod(StatKind::Life, Modifier::Flat(7)));
        assert!(!charm.add_mod(StatKind::Mana, Modifier::Flat(5))); // rejected
        assert_eq!(charm.mods.len(), 1);
    }

    #[test]
    fn grand_charm_max_three_mods() {
        let mut charm = Charm::new(2, CharmSize::Grand, "Grand Charm");
        assert!(charm.add_mod(StatKind::Life, Modifier::Flat(20)));
        assert!(charm.add_mod(StatKind::Mana, Modifier::Flat(10)));
        assert!(charm.add_mod(StatKind::FireResist, Modifier::Flat(10)));
        assert!(!charm.add_mod(StatKind::Defense, Modifier::Flat(5)));
    }

    #[test]
    fn charm_bag_collects_active_mods() {
        let mut bag = CharmBag::new();
        let mut charm = Charm::new(1, CharmSize::Small, "Test");
        charm.add_mod(StatKind::Life, Modifier::Flat(10));
        bag.add(charm);
        assert_eq!(bag.active_mods().len(), 1);
    }

    #[test]
    fn charm_bag_remove_by_uid() {
        let mut bag = CharmBag::new();
        bag.add(Charm::new(1, CharmSize::Small, "A"));
        bag.add(Charm::new(2, CharmSize::Small, "B"));
        bag.remove(1);
        assert_eq!(bag.charms.len(), 1);
        assert_eq!(bag.charms[0].uid, 2);
    }
}
