//! Tests for the mge-arpg-loot crate.

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use crate::drop::LootGenerator;
    use crate::entry::{DropEntry, TreasureClass};
    use crate::quality::{DropQuality, QualityRoller};
    use crate::registry::TreasureClassRegistry;

    /// Helper: deterministic RNG for reproducible tests.
    fn test_rng() -> rand::rngs::StdRng {
        rand::rngs::StdRng::seed_from_u64(42)
    }

    fn make_entry(item_id: &str, weight: u32, is_tc: bool) -> DropEntry {
        DropEntry {
            item_id: item_id.to_string(),
            weight,
            min_qty: 1,
            max_qty: 1,
            is_treasure_class: is_tc,
        }
    }

    fn make_tc(id: &str, picks: u32, no_drop: u32, entries: Vec<DropEntry>) -> TreasureClass {
        TreasureClass {
            id: id.to_string(),
            picks,
            no_drop,
            entries,
        }
    }

    // -----------------------------------------------------------------------
    // 1. tc_total_weight_sum
    // -----------------------------------------------------------------------
    #[test]
    fn tc_total_weight_sum() {
        let tc = make_tc(
            "test",
            1,
            10,
            vec![make_entry("a", 5, false), make_entry("b", 15, false)],
        );
        // 5 + 15 + 10(no_drop) = 30
        assert_eq!(tc.total_weight(), 30);
    }

    // -----------------------------------------------------------------------
    // 2. tc_registry_register_and_get
    // -----------------------------------------------------------------------
    #[test]
    fn tc_registry_register_and_get() {
        let mut reg = TreasureClassRegistry::new();
        assert!(reg.is_empty());

        let tc = make_tc("weapons_01", 1, 0, vec![make_entry("sword", 10, false)]);
        reg.register(tc);

        assert_eq!(reg.len(), 1);
        let fetched = reg.get("weapons_01");
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().id, "weapons_01");
    }

    // -----------------------------------------------------------------------
    // 3. tc_registry_get_missing
    // -----------------------------------------------------------------------
    #[test]
    fn tc_registry_get_missing() {
        let reg = TreasureClassRegistry::new();
        assert!(reg.get("nonexistent").is_none());
    }

    // -----------------------------------------------------------------------
    // 4. quality_roller_no_mf_usually_normal
    // -----------------------------------------------------------------------
    #[test]
    fn quality_roller_no_mf_usually_normal() {
        let mut rng = test_rng();
        let mut normal_count = 0u32;
        let trials = 500;
        for _ in 0..trials {
            if QualityRoller::roll(1, 0, &mut rng) == DropQuality::Normal {
                normal_count += 1;
            }
        }
        // With MF=0 and ilvl=1, the vast majority should be Normal.
        // We expect > 60% normal.
        assert!(
            normal_count > trials / 2,
            "Expected mostly Normal drops, got {normal_count}/{trials}"
        );
    }

    // -----------------------------------------------------------------------
    // 5. quality_roller_high_mf_improves_chance
    // -----------------------------------------------------------------------
    #[test]
    fn quality_roller_high_mf_improves_chance() {
        let mut rng = test_rng();
        let mut non_normal_low = 0u32;
        let mut non_normal_high = 0u32;
        let trials = 2000;

        for _ in 0..trials {
            if QualityRoller::roll(50, 0, &mut rng) != DropQuality::Normal {
                non_normal_low += 1;
            }
            if QualityRoller::roll(50, 400, &mut rng) != DropQuality::Normal {
                non_normal_high += 1;
            }
        }
        // High MF should produce more non-normal items.
        assert!(
            non_normal_high > non_normal_low,
            "High MF ({non_normal_high}) should beat low MF ({non_normal_low})"
        );
    }

    // -----------------------------------------------------------------------
    // 6. loot_generator_gold_drop_in_range
    // -----------------------------------------------------------------------
    #[test]
    fn loot_generator_gold_drop_in_range() {
        let mut rng = test_rng();
        for _ in 0..100 {
            let drop = LootGenerator::gold_drop(10, 50, &mut rng);
            assert_eq!(drop.item_id, "gold");
            assert_eq!(drop.quality, DropQuality::Normal);
            assert!(
                (10..=50).contains(&drop.quantity),
                "quantity {} out of range 10..=50",
                drop.quantity
            );
        }
    }

    // -----------------------------------------------------------------------
    // 7. loot_generator_simple_tc
    // -----------------------------------------------------------------------
    #[test]
    fn loot_generator_simple_tc() {
        let mut rng = test_rng();
        let mut reg = TreasureClassRegistry::new();

        let tc = make_tc(
            "simple",
            1,
            0, // no NoDrop
            vec![make_entry("sword", 50, false), make_entry("shield", 50, false)],
        );
        reg.register(tc);

        let drops = LootGenerator::generate("simple", &reg, 10, 0, &mut rng);
        assert_eq!(drops.len(), 1, "picks=1 with no_drop=0 should yield exactly 1 drop");
    }

    // -----------------------------------------------------------------------
    // 8. loot_generator_no_drop_weight
    // -----------------------------------------------------------------------
    #[test]
    fn loot_generator_no_drop_weight() {
        let mut rng = test_rng();
        let mut reg = TreasureClassRegistry::new();

        // no_drop = 999, item weight = 1 → almost always NoDrop
        let tc = make_tc("mostly_empty", 1, 999, vec![make_entry("rare_gem", 1, false)]);
        reg.register(tc);

        let mut empty_count = 0u32;
        let trials = 200;
        for _ in 0..trials {
            let drops = LootGenerator::generate("mostly_empty", &reg, 10, 0, &mut rng);
            if drops.is_empty() {
                empty_count += 1;
            }
        }
        // With 999/1000 NoDrop chance, most rolls should be empty.
        assert!(
            empty_count > trials / 2,
            "Expected mostly empty drops, got {empty_count}/{trials} empty"
        );
    }

    // -----------------------------------------------------------------------
    // 9. loot_generator_multi_picks
    // -----------------------------------------------------------------------
    #[test]
    fn loot_generator_multi_picks() {
        let mut rng = test_rng();
        let mut reg = TreasureClassRegistry::new();

        let tc = make_tc(
            "multi",
            3,
            0, // no NoDrop
            vec![make_entry("potion", 100, false)],
        );
        reg.register(tc);

        let drops = LootGenerator::generate("multi", &reg, 10, 0, &mut rng);
        assert_eq!(drops.len(), 3, "picks=3 with no_drop=0 should yield 3 drops");
    }

    // -----------------------------------------------------------------------
    // 10. loot_generator_nested_tc
    // -----------------------------------------------------------------------
    #[test]
    fn loot_generator_nested_tc() {
        let mut rng = test_rng();
        let mut reg = TreasureClassRegistry::new();

        // TC "inner" drops a specific item.
        let inner = make_tc("inner", 1, 0, vec![make_entry("diamond", 100, false)]);
        // TC "outer" always points to "inner".
        let outer = make_tc("outer", 1, 0, vec![make_entry("inner", 100, true)]);

        reg.register(inner);
        reg.register(outer);

        let drops = LootGenerator::generate("outer", &reg, 10, 0, &mut rng);
        assert_eq!(drops.len(), 1);
        assert_eq!(drops[0].item_id, "diamond");
    }

    // -----------------------------------------------------------------------
    // 11. loot_generator_missing_tc_returns_empty
    // -----------------------------------------------------------------------
    #[test]
    fn loot_generator_missing_tc_returns_empty() {
        let mut rng = test_rng();
        let reg = TreasureClassRegistry::new();

        let drops = LootGenerator::generate("does_not_exist", &reg, 10, 0, &mut rng);
        assert!(drops.is_empty(), "Unknown TC should yield empty Vec");
    }

    // -----------------------------------------------------------------------
    // 12. loot_generator_max_recursion_safe
    // -----------------------------------------------------------------------
    #[test]
    fn loot_generator_max_recursion_safe() {
        let mut rng = test_rng();
        let mut reg = TreasureClassRegistry::new();

        // Create a cycle: tc_a -> tc_b -> tc_a (infinite recursion if unchecked).
        let tc_a = make_tc("tc_a", 1, 0, vec![make_entry("tc_b", 100, true)]);
        let tc_b = make_tc("tc_b", 1, 0, vec![make_entry("tc_a", 100, true)]);

        reg.register(tc_a);
        reg.register(tc_b);

        // Should NOT stack-overflow — the recursion limit silently stops.
        let drops = LootGenerator::generate("tc_a", &reg, 10, 0, &mut rng);
        // Result is empty because the cycle never resolves to a real item.
        assert!(
            drops.is_empty(),
            "Cyclic TCs should resolve to empty (recursion cap)"
        );
    }
}
