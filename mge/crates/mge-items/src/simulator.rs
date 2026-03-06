//! Drop and economy simulator for offline balance analysis.

use serde::{Deserialize, Serialize};

use crate::loot::LootTable;
use crate::rarity::Rarity;

/// Aggregated statistics from one simulation run.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DropStats {
    pub total_kills: u64,
    pub total_drops: u64,
    pub total_gold: u64,
    pub unique_drops: u64,
    pub rare_drops: u64,
    pub magic_drops: u64,
    pub normal_drops: u64,
}

impl DropStats {
    /// Fraction of kills that produce an item drop.
    #[allow(clippy::cast_precision_loss)]
    pub fn drop_rate(&self) -> f64 {
        if self.total_kills == 0 {
            return 0.0;
        }
        self.total_drops as f64 / self.total_kills as f64
    }

    /// Fraction of kills that produce a unique item.
    #[allow(clippy::cast_precision_loss)]
    pub fn unique_rate(&self) -> f64 {
        if self.total_kills == 0 {
            return 0.0;
        }
        self.unique_drops as f64 / self.total_kills as f64
    }
}

/// Simulate `kills` encounters against a loot table at `area_level`.
/// Uses a deterministic LCG keyed by `seed`.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn simulate_drops(table: &LootTable, area_level: u32, kills: u64, seed: u64) -> DropStats {
    let mut stats = DropStats { total_kills: kills, ..DropStats::default() };
    let mut rng = seed;

    for _ in 0..kills {
        rng = lcg_next(rng);
        let item_roll = (rng % 100) as u32;
        rng = lcg_next(rng);
        let entry_roll = (rng % 10_000) as u32;
        rng = lcg_next(rng);
        let gold_roll = (rng % 1_000) as f32 / 1_000.0;

        if let Some(entry) = table.roll_drop(area_level, item_roll, entry_roll) {
            stats.total_drops += 1;
            match entry.rarity {
                Rarity::Unique => stats.unique_drops += 1,
                Rarity::Rare => stats.rare_drops += 1,
                Rarity::Magic | Rarity::Crafted => stats.magic_drops += 1,
                Rarity::Normal | Rarity::Set => stats.normal_drops += 1,
            }
        }
        stats.total_gold += table.roll_gold(gold_roll);
    }
    stats
}

/// Knuth multiplicative LCG.
fn lcg_next(state: u64) -> u64 {
    state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407)
}

/// Economy snapshot derived from simulation statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomySnapshot {
    pub stats: DropStats,
    pub avg_gold_per_kill: f64,
    pub drop_rate_pct: f64,
}

impl EconomySnapshot {
    #[allow(clippy::cast_precision_loss)]
    pub fn from_stats(stats: DropStats) -> Self {
        let avg_gold = if stats.total_kills == 0 {
            0.0
        } else {
            stats.total_gold as f64 / stats.total_kills as f64
        };
        let drop_rate_pct = stats.drop_rate() * 100.0;
        Self { stats, avg_gold_per_kill: avg_gold, drop_rate_pct }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loot::mvp_fallen_loot;

    #[test]
    fn simulate_drops_in_valid_range() {
        let table = mvp_fallen_loot();
        let stats = simulate_drops(&table, 5, 1_000, 42);
        assert!(stats.total_drops <= stats.total_kills);
        assert!(stats.drop_rate() <= 1.0);
    }

    #[test]
    fn simulate_generates_gold() {
        let table = mvp_fallen_loot();
        let stats = simulate_drops(&table, 5, 1_000, 7);
        // gold_min=1 so at least some gold must appear over 1000 kills
        assert!(stats.total_gold > 0);
    }

    #[test]
    fn economy_snapshot_rates_are_non_negative() {
        let table = mvp_fallen_loot();
        let stats = simulate_drops(&table, 5, 100, 13);
        let snap = EconomySnapshot::from_stats(stats);
        assert!(snap.avg_gold_per_kill >= 0.0);
        assert!(snap.drop_rate_pct >= 0.0 && snap.drop_rate_pct <= 100.0);
    }

    #[test]
    fn different_seeds_differ() {
        let table = mvp_fallen_loot();
        let s1 = simulate_drops(&table, 5, 500, 1);
        let s2 = simulate_drops(&table, 5, 500, 999_999);
        // Vanishingly unlikely to collide over 500 iterations
        assert_ne!(s1.total_gold, s2.total_gold);
    }
}
