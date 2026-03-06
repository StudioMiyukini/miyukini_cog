use serde::{Deserialize, Serialize};

use mge_gameplay::stats::{Modifier, StatKind};

use crate::item::{ItemFamily, ItemKind};

/// Whether an affix occupies a prefix or suffix slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AffixKind {
    Prefix,
    Suffix,
}

/// One stat modifier entry within an affix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffixMod {
    pub stat: StatKind,
    pub modifier: Modifier,
}

/// Static definition of a single affix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffixDef {
    pub id: u32,
    pub name: String,
    pub kind: AffixKind,
    /// Minimum item level for this affix to appear.
    pub min_ilvl: u32,
    /// Item families this affix can spawn on (empty = all families allowed).
    pub allowed_families: Vec<ItemFamily>,
    pub mods: Vec<AffixMod>,
    /// Spawn weight (higher = more common).
    pub weight: u32,
}

impl AffixDef {
    /// Returns true when this affix may appear on the given item kind at `ilvl`.
    pub fn applies_to(&self, kind: ItemKind, ilvl: u32) -> bool {
        if ilvl < self.min_ilvl {
            return false;
        }
        self.allowed_families.is_empty() || self.allowed_families.contains(&kind.family())
    }
}

/// An affix instance stamped onto a specific item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedAffix {
    pub def_id: u32,
    pub kind: AffixKind,
    pub mods: Vec<AffixMod>,
}

/// The full affix set on a magic/rare/crafted item.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemAffixes {
    pub prefixes: Vec<AppliedAffix>,
    pub suffixes: Vec<AppliedAffix>,
}

impl ItemAffixes {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a prefix. Returns false if already at cap (3).
    pub fn add_prefix(&mut self, affix: AppliedAffix) -> bool {
        if self.prefixes.len() >= 3 {
            return false;
        }
        self.prefixes.push(affix);
        true
    }

    /// Add a suffix. Returns false if already at cap (3).
    pub fn add_suffix(&mut self, affix: AppliedAffix) -> bool {
        if self.suffixes.len() >= 3 {
            return false;
        }
        self.suffixes.push(affix);
        true
    }

    pub fn total_count(&self) -> usize {
        self.prefixes.len() + self.suffixes.len()
    }
}

/// Pool of all known affix definitions used for procedural generation.
pub struct AffixPool {
    pub defs: Vec<AffixDef>,
}

impl AffixPool {
    pub fn new(defs: Vec<AffixDef>) -> Self {
        Self { defs }
    }

    /// Select eligible affixes for a given item kind, ilvl, and affix slot.
    pub fn eligible(&self, kind: ItemKind, ilvl: u32, affix_kind: AffixKind) -> Vec<&AffixDef> {
        self.defs.iter().filter(|d| d.kind == affix_kind && d.applies_to(kind, ilvl)).collect()
    }

    /// Pick one affix by weight from an eligible slice, using a `roll` in `[0, total_weight)`.
    pub fn pick<'a>(&'a self, eligible: &[&'a AffixDef], roll: u32) -> Option<&'a AffixDef> {
        let total: u32 = eligible.iter().map(|d| d.weight).sum();
        if total == 0 {
            return None;
        }
        let r = roll % total;
        let mut acc = 0u32;
        for def in eligible {
            acc += def.weight;
            if r < acc {
                return Some(def);
            }
        }
        None
    }
}

/// Minimal MVP affix pool: a handful of prefixes and suffixes.
pub fn mvp_affix_pool() -> AffixPool {
    let defs = vec![
        AffixDef {
            id: 1,
            name: "Sturdy".into(),
            kind: AffixKind::Prefix,
            min_ilvl: 1,
            allowed_families: vec![ItemFamily::Armor],
            mods: vec![AffixMod { stat: StatKind::Defense, modifier: Modifier::Flat(10) }],
            weight: 100,
        },
        AffixDef {
            id: 2,
            name: "Sharp".into(),
            kind: AffixKind::Prefix,
            min_ilvl: 1,
            allowed_families: vec![ItemFamily::Weapon],
            mods: vec![AffixMod { stat: StatKind::DamageMax, modifier: Modifier::Flat(5) }],
            weight: 100,
        },
        AffixDef {
            id: 3,
            name: "Fiery".into(),
            kind: AffixKind::Prefix,
            min_ilvl: 3,
            allowed_families: vec![ItemFamily::Weapon],
            mods: vec![AffixMod { stat: StatKind::DamageMin, modifier: Modifier::Flat(3) }],
            weight: 70,
        },
        AffixDef {
            id: 4,
            name: "of the Ox".into(),
            kind: AffixKind::Suffix,
            min_ilvl: 1,
            allowed_families: vec![],
            mods: vec![AffixMod { stat: StatKind::Life, modifier: Modifier::Flat(15) }],
            weight: 80,
        },
        AffixDef {
            id: 5,
            name: "of Fire".into(),
            kind: AffixKind::Suffix,
            min_ilvl: 5,
            allowed_families: vec![ItemFamily::Weapon],
            mods: vec![AffixMod { stat: StatKind::FireResist, modifier: Modifier::Flat(10) }],
            weight: 60,
        },
    ];
    AffixPool::new(defs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affix_filters_by_family() {
        let pool = mvp_affix_pool();
        let eligible = pool.eligible(ItemKind::Sword, 1, AffixKind::Prefix);
        // Sturdy is Armor-only → absent; Sharp is Weapon → present
        assert!(!eligible.iter().any(|d| d.name == "Sturdy"));
        assert!(eligible.iter().any(|d| d.name == "Sharp"));
    }

    #[test]
    fn affix_filters_by_ilvl() {
        let pool = mvp_affix_pool();
        // "of Fire" requires ilvl 5; at ilvl 3 it must be excluded
        let eligible = pool.eligible(ItemKind::Sword, 3, AffixKind::Suffix);
        assert!(!eligible.iter().any(|d| d.name == "of Fire"));
    }

    #[test]
    fn item_affixes_caps_at_three_prefixes() {
        let mut affixes = ItemAffixes::new();
        let make = || AppliedAffix { def_id: 1, kind: AffixKind::Prefix, mods: vec![] };
        assert!(affixes.add_prefix(make()));
        assert!(affixes.add_prefix(make()));
        assert!(affixes.add_prefix(make()));
        assert!(!affixes.add_prefix(make())); // 4th rejected
        assert_eq!(affixes.prefixes.len(), 3);
    }

    #[test]
    fn weighted_pick_returns_some_when_eligible() {
        let pool = mvp_affix_pool();
        let eligible = pool.eligible(ItemKind::Sword, 10, AffixKind::Prefix);
        assert!(pool.pick(&eligible, 0).is_some());
    }
}
