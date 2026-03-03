// @id: Sodomight-Content-Loot @do: act1-treasure-class-definitions @role: back-end @layer: 4 @human: miyuk
//! Act 1 treasure class definitions for the loot generation system.
//!
//! Each monster references one of these TCs via its `tc_id` field.
//! Drop weights and NoDrop values are tuned for early-game pacing.
#![allow(clippy::too_many_lines)]

use mge_arpg_loot::{DropEntry, TreasureClass};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Convenience helper to build a non-TC `DropEntry`.
fn drop(item_id: &str, weight: u32, min_qty: u32, max_qty: u32) -> DropEntry {
    DropEntry {
        item_id: item_id.into(),
        weight,
        min_qty,
        max_qty,
        is_treasure_class: false,
    }
}

// ---------------------------------------------------------------------------
// Treasure class definitions
// ---------------------------------------------------------------------------

/// Returns Act 1 treasure class definitions.
///
/// Each monster references one of these TCs via its `tc_id` field.
/// Drop weights and NoDrop values are tuned for early-game pacing.
#[must_use]
pub fn act1_treasure_classes() -> Vec<TreasureClass> {
    vec![
        TreasureClass {
            id: "tc_fallen".into(),
            picks: 1,
            no_drop: 50,
            entries: vec![
                drop("gold", 40, 1, 5),
                drop("minor_health_potion", 10, 1, 1),
            ],
        },
        TreasureClass {
            id: "tc_zombie".into(),
            picks: 1,
            no_drop: 45,
            entries: vec![
                drop("gold", 35, 2, 8),
                drop("minor_health_potion", 15, 1, 1),
                drop("short_sword", 5, 1, 1),
            ],
        },
        TreasureClass {
            id: "tc_skeleton".into(),
            picks: 1,
            no_drop: 40,
            entries: vec![
                drop("gold", 30, 3, 10),
                drop("minor_health_potion", 12, 1, 1),
                drop("buckler", 5, 1, 1),
                drop("cap", 5, 1, 1),
            ],
        },
        // Wendigo: mid-tier loot, occasional armor.
        TreasureClass {
            id: "tc_wendigo".into(),
            picks: 1,
            no_drop: 35,
            entries: vec![
                drop("gold", 30, 4, 15),
                drop("minor_health_potion", 15, 1, 1),
                drop("quilted_armor", 8, 1, 1),
                drop("leather_boots", 7, 1, 1),
            ],
        },
        // Corrupted Rogue: balanced table, weapon-heavy.
        TreasureClass {
            id: "tc_rogue".into(),
            picks: 1,
            no_drop: 38,
            entries: vec![
                drop("gold", 28, 3, 12),
                drop("minor_health_potion", 12, 1, 1),
                drop("short_sword", 8, 1, 1),
                drop("buckler", 6, 1, 1),
            ],
        },
        // Goatman: mid-tier table, armor bias.
        TreasureClass {
            id: "tc_goatman".into(),
            picks: 1,
            no_drop: 36,
            entries: vec![
                drop("gold", 28, 4, 14),
                drop("minor_health_potion", 12, 1, 1),
                drop("cap", 8, 1, 1),
                drop("hand_axe", 8, 1, 1),
            ],
        },
        // Brute: high-tier table, better drops for the HP wall.
        TreasureClass {
            id: "tc_brute".into(),
            picks: 1,
            no_drop: 30,
            entries: vec![
                drop("gold", 25, 5, 18),
                drop("minor_health_potion", 15, 1, 1),
                drop("leather_armor", 10, 1, 1),
                drop("hand_axe", 10, 1, 1),
            ],
        },
        // Tainted: physical fighter, balanced table.
        TreasureClass {
            id: "tc_tainted".into(),
            picks: 1,
            no_drop: 35,
            entries: vec![
                drop("gold", 28, 4, 14),
                drop("minor_health_potion", 14, 1, 1),
                drop("leather_gloves", 8, 1, 1),
                drop("sash", 7, 1, 1),
            ],
        },
        // Ghoul: poison-themed, potion-heavy.
        TreasureClass {
            id: "tc_ghoul".into(),
            picks: 1,
            no_drop: 38,
            entries: vec![
                drop("gold", 25, 3, 12),
                drop("minor_health_potion", 18, 1, 1),
                drop("minor_mana_potion", 10, 1, 1),
            ],
        },
        // Dark Hunter / Vile Hunter: high-tier ranged.
        TreasureClass {
            id: "tc_hunter".into(),
            picks: 1,
            no_drop: 32,
            entries: vec![
                drop("gold", 28, 5, 16),
                drop("minor_health_potion", 12, 1, 1),
                drop("leather_armor", 10, 1, 1),
                drop("leather_boots", 8, 1, 1),
            ],
        },
        TreasureClass {
            id: "tc_blood_raven".into(),
            picks: 3,
            no_drop: 10,
            entries: vec![
                drop("gold", 20, 10, 30),
                drop("hand_axe", 15, 1, 1),
                drop("leather_armor", 15, 1, 1),
                drop("minor_health_potion", 10, 1, 1),
            ],
        },
        TreasureClass {
            id: "tc_andariel".into(),
            picks: 5,
            no_drop: 5,
            entries: vec![
                drop("gold", 10, 30, 100),
                drop("quilted_armor", 10, 1, 1),
                drop("leather_armor", 10, 1, 1),
                drop("hand_axe", 10, 1, 1),
                drop("short_sword", 10, 1, 1),
            ],
        },
    ]
}

// ---------------------------------------------------------------------------
// Act 1 canonical treasure class definitions (TC1-TC4)
// ---------------------------------------------------------------------------

/// Returns the four canonical Act 1 treasure classes used for unit testing
/// and registry seeding.
#[cfg(test)]
///
/// These TCs represent the canonical D2-style progression:
/// - TC1 `tc_fallen`  : high NoDrop (60), light loot, early zone.
/// - TC2 `tc_zombie`  : medium NoDrop (40), balanced loot.
/// - TC3 `tc_skeleton`: lower NoDrop (30), higher item chance.
/// - TC4 `tc_boss`    : NoDrop=0 (guaranteed drop), Andariel-tier.
///
/// Drop entry layout per TC:
/// `NoDrop | Gold(min-max) | Item` -- only the NoDrop weight lives in
/// `TreasureClass::no_drop`; Gold and Item are `DropEntry` rows.
fn act1_treasure_classes_canonical() -> Vec<TreasureClass> {
    vec![
        // TC1 -- Fallen: very likely to drop nothing, small gold or potion.
        TreasureClass {
            id: "tc_fallen".into(),
            picks: 1,
            no_drop: 60,
            entries: vec![
                drop("gold", 25, 1, 5),
                drop("minor_health_potion", 15, 1, 1),
            ],
        },
        // TC2 -- Zombie: balanced early-game table.
        TreasureClass {
            id: "tc_zombie".into(),
            picks: 1,
            no_drop: 40,
            entries: vec![
                drop("gold", 30, 2, 10),
                drop("minor_health_potion", 30, 1, 1),
            ],
        },
        // TC3 -- Skeleton: item drop more common than NoDrop.
        TreasureClass {
            id: "tc_skeleton".into(),
            picks: 1,
            no_drop: 30,
            entries: vec![
                drop("gold", 35, 3, 12),
                drop("buckler", 35, 1, 1),
            ],
        },
        // TC4 -- Boss (Andariel): guaranteed drop (NoDrop=0), rich loot.
        TreasureClass {
            id: "tc_boss".into(),
            picks: 1,
            no_drop: 0,
            entries: vec![
                drop("gold", 40, 10, 50),
                drop("leather_armor", 60, 1, 1),
            ],
        },
    ]
}

// ---------------------------------------------------------------------------
// Lookup helpers
// ---------------------------------------------------------------------------

/// Find a treasure class by its id.
#[must_use]
pub fn find_treasure_class(id: &str) -> Option<TreasureClass> {
    act1_treasure_classes()
        .into_iter()
        .find(|tc| tc.id == id)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_act1_treasure_classes_count() {
        let tcs = act1_treasure_classes();
        // 5 original + 7 bestiary TCs = 12
        assert_eq!(tcs.len(), 12);
    }

    #[test]
    fn test_tc_fallen_no_drop_weight() {
        let tc = find_treasure_class("tc_fallen").expect("tc_fallen must exist");
        assert_eq!(tc.no_drop, 50);
        assert_eq!(tc.picks, 1);
        assert_eq!(tc.entries.len(), 2);
    }

    #[test]
    fn test_tc_andariel_picks() {
        let tc = find_treasure_class("tc_andariel").expect("tc_andariel must exist");
        assert_eq!(tc.picks, 5);
        assert_eq!(tc.no_drop, 5);
    }

    // -- Canonical TC1-TC4 (act1_treasure_classes_canonical) --------------

    /// TC1 (tc_fallen): NoDrop weight must be > 50 % of total weight.
    ///
    /// Spec: NoDrop=60, Gold weight=25, Item weight=15 -> total=100.
    /// NoDrop fraction = 60/100 = 60 % > 50 %.
    #[test]
    fn tc1_no_drop_common() {
        let tcs = act1_treasure_classes_canonical();
        let tc1 = tcs.iter().find(|tc| tc.id == "tc_fallen").expect("tc_fallen must exist");
        let total = tc1.total_weight();
        // NoDrop must represent more than half of the total weight.
        assert!(
            tc1.no_drop * 2 > total,
            "tc_fallen NoDrop ({}) should be > 50% of total ({})",
            tc1.no_drop,
            total,
        );
    }

    /// TC4 (tc_boss/Andariel): NoDrop weight must be exactly 0.
    ///
    /// Spec: NoDrop=0 -> every pick is guaranteed to yield an item.
    #[test]
    fn tc4_guaranteed_drop() {
        let tcs = act1_treasure_classes_canonical();
        let tc4 = tcs.iter().find(|tc| tc.id == "tc_boss").expect("tc_boss must exist");
        assert_eq!(
            tc4.no_drop, 0,
            "tc_boss NoDrop must be 0 (guaranteed drop), got {}",
            tc4.no_drop,
        );
    }

    /// act1_treasure_classes_canonical must return exactly 4 TCs.
    #[test]
    fn tc_count() {
        let tcs = act1_treasure_classes_canonical();
        assert_eq!(
            tcs.len(),
            4,
            "act1_treasure_classes_canonical must contain exactly 4 TCs, got {}",
            tcs.len(),
        );
    }
}
