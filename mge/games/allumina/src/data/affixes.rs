//! Allumina's procedural affix pool, Diablo-2 style.
//!
//! Each affix is a *prefix* or a *suffix*, belongs to a *family* (no two affixes
//! of the same family may roll on one item), carries a *weight* for the weighted
//! draw, and scales through level-gated *tiers*. Magic items take 1-2 affixes
//! (a prefix and/or a suffix); rare items take 3-6 (max 3 of each slot).
//!
//! `mge-gameplay` has a fixed `StatKind` set (no damage-%, life-leech-%, regen,
//! or thorns stat), so a few affixes map onto the nearest available stat:
//! "+% Damage" -> `DamageMax` %, leech/regen -> `LifeOnHit`, dodge -> `BlockChance`,
//! thorns -> flat `Defense`.

use mge_gameplay::stats::{Modifier, StatKind};
use mge_items::affixes::AffixMod;

/// Which half of the name an affix occupies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AffixSlot {
    Prefix,
    Suffix,
}

/// Whether a rolled value is an absolute amount or a percentage.
#[derive(Debug, Clone, Copy)]
pub enum ModKind {
    Flat,
    Percent,
}

/// How an affix turns one rolled value into concrete stat modifiers.
#[derive(Debug, Clone, Copy)]
pub enum StatMod {
    /// A single stat.
    Single(StatKind, ModKind),
    /// Two stats, each granted the rolled value (e.g. Soldier's: +Str +Vit).
    Dual(StatKind, StatKind, ModKind),
    /// All four elemental resistances (e.g. of Warding).
    AllResist(ModKind),
}

impl StatMod {
    fn mk(kind: ModKind, value: i32) -> Modifier {
        match kind {
            ModKind::Flat => Modifier::Flat(value),
            ModKind::Percent => Modifier::Percentage(value),
        }
    }

    /// Concrete modifiers granted by this affix for a given rolled `value`.
    #[must_use]
    pub fn to_mods(self, value: i32) -> Vec<AffixMod> {
        match self {
            StatMod::Single(stat, k) => vec![AffixMod { stat, modifier: Self::mk(k, value) }],
            StatMod::Dual(a, b, k) => vec![
                AffixMod { stat: a, modifier: Self::mk(k, value) },
                AffixMod { stat: b, modifier: Self::mk(k, value) },
            ],
            StatMod::AllResist(k) => [
                StatKind::FireResist,
                StatKind::ColdResist,
                StatKind::LightningResist,
                StatKind::PoisonResist,
            ]
            .into_iter()
            .map(|stat| AffixMod { stat, modifier: Self::mk(k, value) })
            .collect(),
        }
    }
}

/// One level-gated value band for an affix.
#[derive(Debug, Clone, Copy)]
pub struct AffixTier {
    pub min_level: u32,
    pub min_value: f64,
    pub max_value: f64,
}

/// A single affix definition (prefix or suffix).
#[derive(Debug, Clone)]
pub struct AffixDef {
    pub id: u32,
    pub name: &'static str,
    pub slot: AffixSlot,
    pub stat: StatMod,
    pub tiers: Vec<AffixTier>,
    pub weight: u32,
    pub family: &'static str,
}

impl AffixDef {
    /// Highest tier unlocked at `level` (tiers must be ordered by `min_level`).
    #[must_use]
    pub fn tier_for(&self, level: u32) -> Option<&AffixTier> {
        self.tiers.iter().filter(|t| t.min_level <= level).last()
    }
}

/// The full affix pool used for magic/rare generation.
pub struct AffixPool {
    defs: Vec<AffixDef>,
}

impl AffixPool {
    #[must_use]
    pub fn new(defs: Vec<AffixDef>) -> Self {
        Self { defs }
    }

    #[must_use]
    pub fn defs(&self) -> &[AffixDef] {
        &self.defs
    }

    /// Definitions of `slot` whose first tier is reachable at `level`.
    #[must_use]
    pub fn eligible(&self, slot: AffixSlot, level: u32) -> Vec<&AffixDef> {
        self.defs
            .iter()
            .filter(|d| d.slot == slot)
            .filter(|d| d.tiers.first().map_or(false, |t| t.min_level <= level))
            .collect()
    }
}

fn t(min_level: u32, min_value: f64, max_value: f64) -> AffixTier {
    AffixTier { min_level, min_value, max_value }
}

fn prefix(
    id: u32,
    name: &'static str,
    stat: StatMod,
    weight: u32,
    family: &'static str,
    tiers: Vec<AffixTier>,
) -> AffixDef {
    AffixDef { id, name, slot: AffixSlot::Prefix, stat, tiers, weight, family }
}

fn suffix(
    id: u32,
    name: &'static str,
    stat: StatMod,
    weight: u32,
    family: &'static str,
    tiers: Vec<AffixTier>,
) -> AffixDef {
    AffixDef { id, name, slot: AffixSlot::Suffix, stat, tiers, weight, family }
}

/// Build the complete affix pool (14 prefixes, 12 suffixes).
#[must_use]
pub fn affix_pool() -> AffixPool {
    use ModKind::{Flat, Percent};
    use StatKind::{
        AttackSpeed, BlockChance, CritChance, DamageMax, DamageMin, Defense, Dexterity, Energy,
        Life, LifeOnHit, ManaOnKill, Mana, MovementSpeed, Strength, Vitality,
    };

    let defs = vec![
        // ── Prefixes (offensive / utility) ──────────────────────────────────
        prefix(100, "Jagged", StatMod::Single(DamageMin, Flat), 100, "dmg_min",
            vec![t(1, 1.0, 3.0), t(10, 3.0, 6.0), t(20, 6.0, 10.0), t(35, 10.0, 15.0)]),
        prefix(101, "Sharp", StatMod::Single(DamageMax, Flat), 100, "dmg_max",
            vec![t(1, 2.0, 4.0), t(10, 4.0, 8.0), t(20, 8.0, 14.0), t(35, 14.0, 20.0)]),
        prefix(102, "Fine", StatMod::Single(AttackSpeed, Percent), 60, "atk_speed",
            vec![t(1, 5.0, 10.0), t(10, 10.0, 15.0), t(20, 15.0, 20.0), t(35, 20.0, 30.0)]),
        prefix(103, "Sturdy", StatMod::Single(Defense, Flat), 100, "defense",
            vec![t(1, 5.0, 10.0), t(10, 10.0, 20.0), t(20, 20.0, 35.0), t(35, 35.0, 50.0)]),
        prefix(104, "Stout", StatMod::Single(Life, Flat), 90, "life",
            vec![t(1, 5.0, 15.0), t(10, 15.0, 30.0), t(20, 30.0, 50.0), t(35, 50.0, 80.0)]),
        prefix(105, "Glowing", StatMod::Single(Mana, Flat), 80, "mana",
            vec![t(1, 5.0, 10.0), t(10, 10.0, 20.0), t(20, 20.0, 35.0), t(35, 35.0, 50.0)]),
        prefix(106, "Cruel", StatMod::Single(DamageMax, Percent), 50, "dmg_pct",
            vec![t(1, 10.0, 20.0), t(10, 20.0, 40.0), t(20, 40.0, 60.0), t(35, 60.0, 100.0)]),
        prefix(107, "Fleet", StatMod::Single(MovementSpeed, Percent), 50, "move_speed",
            vec![t(1, 5.0, 10.0), t(10, 10.0, 15.0), t(20, 15.0, 20.0), t(35, 20.0, 25.0)]),
        prefix(108, "Rugged", StatMod::Single(Strength, Flat), 70, "strength",
            vec![t(1, 1.0, 3.0), t(10, 3.0, 5.0), t(20, 5.0, 8.0), t(35, 8.0, 12.0)]),
        prefix(109, "Nimble", StatMod::Single(Dexterity, Flat), 70, "dexterity",
            vec![t(1, 1.0, 3.0), t(10, 3.0, 5.0), t(20, 5.0, 8.0), t(35, 8.0, 12.0)]),
        prefix(110, "Sage", StatMod::Single(Energy, Flat), 70, "energy",
            vec![t(1, 1.0, 3.0), t(10, 3.0, 5.0), t(20, 5.0, 8.0), t(35, 8.0, 12.0)]),
        prefix(111, "Hardy", StatMod::Single(Vitality, Flat), 70, "vitality",
            vec![t(1, 1.0, 3.0), t(10, 3.0, 5.0), t(20, 5.0, 8.0), t(35, 8.0, 12.0)]),
        prefix(112, "Soldier's", StatMod::Dual(Strength, Vitality, Flat), 40, "str_vit",
            vec![t(1, 1.0, 2.0), t(10, 2.0, 3.0), t(20, 3.0, 5.0), t(35, 5.0, 8.0)]),
        prefix(113, "Knight's", StatMod::Single(CritChance, Percent), 40, "crit",
            vec![t(1, 1.0, 2.0), t(10, 2.0, 4.0), t(20, 4.0, 6.0), t(35, 6.0, 10.0)]),
        // ── Suffixes (defensive / resist / special) ─────────────────────────
        suffix(200, "of Fire", StatMod::Single(StatKind::FireResist, Percent), 70, "res_fire",
            vec![t(1, 5.0, 10.0), t(10, 10.0, 20.0), t(20, 20.0, 30.0), t(35, 30.0, 40.0)]),
        suffix(201, "of Frost", StatMod::Single(StatKind::ColdResist, Percent), 70, "res_cold",
            vec![t(1, 5.0, 10.0), t(10, 10.0, 20.0), t(20, 20.0, 30.0), t(35, 30.0, 40.0)]),
        suffix(202, "of Lightning", StatMod::Single(StatKind::LightningResist, Percent), 70, "res_light",
            vec![t(1, 5.0, 10.0), t(10, 10.0, 20.0), t(20, 20.0, 30.0), t(35, 30.0, 40.0)]),
        suffix(203, "of Venom", StatMod::Single(StatKind::PoisonResist, Percent), 70, "res_poison",
            vec![t(1, 5.0, 10.0), t(10, 10.0, 20.0), t(20, 20.0, 30.0), t(35, 30.0, 40.0)]),
        suffix(204, "of the Colossus", StatMod::Single(Life, Flat), 60, "life",
            vec![t(1, 10.0, 20.0), t(10, 20.0, 40.0), t(20, 40.0, 65.0), t(35, 65.0, 100.0)]),
        // No life-leech-% stat: approximate with flat life-on-hit.
        suffix(205, "of the Leech", StatMod::Single(LifeOnHit, Flat), 35, "life_steal",
            vec![t(1, 1.0, 2.0), t(10, 2.0, 3.0), t(20, 3.0, 5.0), t(35, 5.0, 7.0)]),
        // No mana-leech stat: approximate with mana-on-kill.
        suffix(206, "of the Bat", StatMod::Single(ManaOnKill, Flat), 35, "mana_steal",
            vec![t(1, 1.0, 2.0), t(10, 2.0, 3.0), t(20, 3.0, 5.0), t(35, 5.0, 7.0)]),
        // No regen stat: approximate with flat life-on-hit (distinct family).
        suffix(207, "of Regrowth", StatMod::Single(LifeOnHit, Flat), 45, "life_regen",
            vec![t(1, 1.0, 2.0), t(10, 2.0, 4.0), t(20, 4.0, 7.0), t(35, 7.0, 10.0)]),
        suffix(208, "of Warding", StatMod::AllResist(Percent), 40, "all_res",
            vec![t(1, 3.0, 5.0), t(10, 5.0, 10.0), t(20, 10.0, 15.0), t(35, 15.0, 20.0)]),
        // No thorns/reflect stat: approximate with flat defense (distinct family).
        suffix(209, "of Thorns", StatMod::Single(Defense, Flat), 40, "thorns",
            vec![t(1, 1.0, 3.0), t(10, 3.0, 6.0), t(20, 6.0, 10.0), t(35, 10.0, 15.0)]),
        // No dodge stat: approximate with block chance.
        suffix(210, "of Balance", StatMod::Single(BlockChance, Percent), 40, "dodge",
            vec![t(1, 2.0, 4.0), t(10, 4.0, 8.0), t(20, 8.0, 12.0), t(35, 12.0, 18.0)]),
        suffix(211, "of the Mammoth", StatMod::Single(Defense, Flat), 55, "defense",
            vec![t(1, 10.0, 20.0), t(10, 20.0, 35.0), t(20, 35.0, 50.0), t(35, 50.0, 75.0)]),
    ];
    AffixPool::new(defs)
}

/// First word of a rare item's invented name.
pub const RARE_FIRST_WORDS: [&str; 40] = [
    "Dread", "Shadow", "Storm", "Blood", "Iron", "Doom", "Soul", "Dark", "War", "Death", "Rune",
    "Grim", "Night", "Bone", "Fury", "Plague", "Ember", "Frost", "Thunder", "Ash", "Void",
    "Blight", "Havoc", "Chaos", "Wrath", "Spirit", "Dragon", "Demon", "Ancient", "Savage", "Viper",
    "Wolf", "Raven", "Eagle", "Serpent", "Flame", "Stone", "Steel", "Thorn", "Mist",
];

/// Second word of a rare item's invented name.
pub const RARE_SECOND_WORDS: [&str; 40] = [
    "Bane", "Edge", "Mark", "Fang", "Claw", "Gore", "Grip", "Song", "Crown", "Heart", "Ward",
    "Star", "Strike", "Shard", "Gaze", "Bloom", "Bite", "Brow", "Spine", "Veil", "Roar", "Wing",
    "Weave", "Break", "Guard", "Keep", "Touch", "Cry", "Wind", "Tide", "Reign", "Forge", "Might",
    "Wail", "Brand", "Grasp", "Haze", "Tusk", "Horn", "Pact",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_has_expected_slot_counts() {
        let pool = affix_pool();
        assert_eq!(pool.eligible(AffixSlot::Prefix, 99).len(), 14);
        assert_eq!(pool.eligible(AffixSlot::Suffix, 99).len(), 12);
    }

    #[test]
    fn tier_scales_with_level() {
        let pool = affix_pool();
        let jagged = pool.defs().iter().find(|d| d.name == "Jagged").unwrap();
        assert_eq!(jagged.tier_for(1).unwrap().min_level, 1);
        assert_eq!(jagged.tier_for(9).unwrap().min_level, 1);
        assert_eq!(jagged.tier_for(15).unwrap().min_level, 10);
        assert_eq!(jagged.tier_for(40).unwrap().min_level, 35);
    }

    #[test]
    fn low_level_only_unlocks_first_tier() {
        let pool = affix_pool();
        for d in pool.defs() {
            assert_eq!(d.tier_for(1).unwrap().min_level, 1, "{} tier@1", d.name);
        }
    }

    #[test]
    fn all_resist_expands_to_four_mods() {
        let mods = StatMod::AllResist(ModKind::Percent).to_mods(10);
        assert_eq!(mods.len(), 4);
    }

    #[test]
    fn rare_word_tables_are_full() {
        assert_eq!(RARE_FIRST_WORDS.len(), 40);
        assert_eq!(RARE_SECOND_WORDS.len(), 40);
    }
}
