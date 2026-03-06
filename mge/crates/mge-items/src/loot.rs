use serde::{Deserialize, Serialize};

use crate::item::ItemKind;
use crate::rarity::Rarity;

/// One weighted entry in a loot table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootEntry {
    pub kind: ItemKind,
    pub rarity: Rarity,
    /// Spawn weight (higher = more common).
    pub weight: u32,
    /// Minimum area level for this entry to become eligible.
    pub min_area_level: u32,
}

/// Procedural loot table for a monster category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTable {
    pub name: String,
    pub entries: Vec<LootEntry>,
    /// Chance (0–100) of no item drop.
    pub no_drop_chance: u32,
    pub gold_min: u64,
    pub gold_max: u64,
}

impl LootTable {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            entries: Vec::new(),
            no_drop_chance: 50,
            gold_min: 0,
            gold_max: 0,
        }
    }

    pub fn add_entry(&mut self, entry: LootEntry) {
        self.entries.push(entry);
    }

    /// Total eligible weight at the given area level.
    pub fn total_weight(&self, area_level: u32) -> u32 {
        self.entries.iter().filter(|e| e.min_area_level <= area_level).map(|e| e.weight).sum()
    }

    /// Roll a drop. `item_roll` in `[0, 100)` checks no-drop; `entry_roll` selects the entry.
    pub fn roll_drop(
        &self,
        area_level: u32,
        item_roll: u32,
        entry_roll: u32,
    ) -> Option<&LootEntry> {
        if item_roll < self.no_drop_chance {
            return None;
        }
        let eligible: Vec<&LootEntry> =
            self.entries.iter().filter(|e| e.min_area_level <= area_level).collect();
        if eligible.is_empty() {
            return None;
        }
        let total: u32 = eligible.iter().map(|e| e.weight).sum();
        if total == 0 {
            return None;
        }
        let r = entry_roll % total;
        let mut acc = 0u32;
        for entry in &eligible {
            acc += entry.weight;
            if r < acc {
                return Some(entry);
            }
        }
        None
    }

    /// Gold drop for a given roll in `[0.0, 1.0)`.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn roll_gold(&self, roll: f32) -> u64 {
        if self.gold_max <= self.gold_min {
            return self.gold_min;
        }
        self.gold_min + ((self.gold_max - self.gold_min) as f32 * roll) as u64
    }
}

/// Boss loot table with guaranteed drops plus random ones.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossLootTable {
    pub name: String,
    pub guaranteed: Vec<LootEntry>,
    pub random: LootTable,
}

// ─── MVP tables ──────────────────────────────────────────────────────────────

pub fn mvp_fallen_loot() -> LootTable {
    let mut t = LootTable::new("Fallen");
    t.no_drop_chance = 60;
    t.gold_min = 1;
    t.gold_max = 30;
    t.add_entry(LootEntry {
        kind: ItemKind::Sword,
        rarity: Rarity::Normal,
        weight: 80,
        min_area_level: 1,
    });
    t.add_entry(LootEntry {
        kind: ItemKind::Helm,
        rarity: Rarity::Normal,
        weight: 70,
        min_area_level: 1,
    });
    t.add_entry(LootEntry {
        kind: ItemKind::Sword,
        rarity: Rarity::Magic,
        weight: 20,
        min_area_level: 2,
    });
    t.add_entry(LootEntry {
        kind: ItemKind::Ring,
        rarity: Rarity::Magic,
        weight: 5,
        min_area_level: 5,
    });
    t
}

pub fn mvp_andariel_loot() -> BossLootTable {
    let mut random = LootTable::new("Andariel Random");
    random.no_drop_chance = 0;
    random.gold_min = 500;
    random.gold_max = 2_000;
    random.add_entry(LootEntry {
        kind: ItemKind::Amulet,
        rarity: Rarity::Unique,
        weight: 10,
        min_area_level: 1,
    });
    random.add_entry(LootEntry {
        kind: ItemKind::BodyArmor,
        rarity: Rarity::Rare,
        weight: 60,
        min_area_level: 1,
    });
    random.add_entry(LootEntry {
        kind: ItemKind::Ring,
        rarity: Rarity::Rare,
        weight: 30,
        min_area_level: 1,
    });
    BossLootTable {
        name: "Andariel".into(),
        guaranteed: vec![LootEntry {
            kind: ItemKind::TownPortalScroll,
            rarity: Rarity::Normal,
            weight: 1,
            min_area_level: 1,
        }],
        random,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallen_table_no_drop_below_threshold() {
        let table = mvp_fallen_loot();
        // item_roll=50 < no_drop_chance=60 → no item
        assert!(table.roll_drop(1, 50, 0).is_none());
    }

    #[test]
    fn fallen_table_drops_above_threshold() {
        let table = mvp_fallen_loot();
        // item_roll=99 >= no_drop_chance=60 → item
        assert!(table.roll_drop(1, 99, 0).is_some());
    }

    #[test]
    fn fallen_table_weight_grows_with_area_level() {
        let table = mvp_fallen_loot();
        // Magic ring requires area_level ≥ 5
        assert!(table.total_weight(5) > table.total_weight(1));
    }

    #[test]
    fn boss_table_has_guaranteed_drops() {
        let boss = mvp_andariel_loot();
        assert!(!boss.guaranteed.is_empty());
    }

    #[test]
    fn gold_roll_within_range() {
        let table = mvp_fallen_loot();
        let gold = table.roll_gold(0.5);
        assert!(gold >= table.gold_min && gold <= table.gold_max);
    }
}
