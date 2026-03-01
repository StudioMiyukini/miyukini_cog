// @id: MGE-ARPG-Items-Tests @do: unit-tests @role: back-end @layer: 3 @human: miyuk
//! Unit tests for the mge-arpg-items crate.

#[cfg(test)]
mod tests {
    use crate::equipment::Equipment;
    use crate::instance::ItemInstance;
    use crate::inventory::{Inventory, INV_COLS, INV_ROWS};
    use crate::quality::{Affix, AffixKind, ItemQuality};
    use crate::slot::ItemSlot;

    use rand::SeedableRng;

    /// Helper: deterministic RNG for reproducible tests.
    fn test_rng() -> rand::rngs::StdRng {
        rand::rngs::StdRng::seed_from_u64(42)
    }

    /// Helper: create a basic normal-quality item.
    fn make_item(base_id: &str) -> ItemInstance {
        ItemInstance::new_normal(base_id.to_string(), 10)
    }

    // ========================================================================
    // ItemInstance tests
    // ========================================================================

    #[test]
    fn item_instance_new_normal_is_unidentified() {
        let item = make_item("long_sword");
        assert!(!item.is_identified);
        assert_eq!(item.quality, ItemQuality::Normal);
        assert_eq!(item.ilvl, 10);
        assert!(item.affixes.is_empty());
        assert_eq!(item.stack_size, 1);
    }

    #[test]
    fn item_instance_identify() {
        let mut item = make_item("chain_mail");
        assert!(!item.is_identified);
        item.identify();
        assert!(item.is_identified);
    }

    #[test]
    fn item_instance_apply_damage_reduces_durability() {
        let mut item = make_item("kite_shield");
        assert_eq!(item.durability_current, 100);
        item.apply_damage(30);
        assert_eq!(item.durability_current, 70);
        item.apply_damage(50);
        assert_eq!(item.durability_current, 20);
    }

    #[test]
    fn item_instance_broken_at_zero_durability() {
        let mut item = make_item("short_bow");
        assert!(!item.is_broken());
        item.apply_damage(100);
        assert!(item.is_broken());
        assert_eq!(item.durability_current, 0);
        // Applying more damage does not underflow.
        item.apply_damage(50);
        assert!(item.is_broken());
        assert_eq!(item.durability_current, 0);
    }

    #[test]
    fn item_instance_affix_prefix_filter() {
        let mut item = make_item("ring_gold");
        let mut rng = test_rng();
        item.affixes.push(Affix::roll(
            "p1".to_string(),
            AffixKind::Prefix,
            "max_life".to_string(),
            10,
            20,
            &mut rng,
        ));
        item.affixes.push(Affix::roll(
            "s1".to_string(),
            AffixKind::Suffix,
            "fire_res".to_string(),
            5,
            15,
            &mut rng,
        ));
        item.affixes.push(Affix::roll(
            "p2".to_string(),
            AffixKind::Prefix,
            "min_damage".to_string(),
            1,
            5,
            &mut rng,
        ));
        let prefixes: Vec<_> = item.prefixes().collect();
        assert_eq!(prefixes.len(), 2);
        assert!(prefixes.iter().all(|a| a.kind == AffixKind::Prefix));
    }

    #[test]
    fn item_instance_affix_suffix_filter() {
        let mut item = make_item("amulet_jade");
        let mut rng = test_rng();
        item.affixes.push(Affix::roll(
            "p1".to_string(),
            AffixKind::Prefix,
            "max_mana".to_string(),
            10,
            30,
            &mut rng,
        ));
        item.affixes.push(Affix::roll(
            "s1".to_string(),
            AffixKind::Suffix,
            "cold_res".to_string(),
            5,
            20,
            &mut rng,
        ));
        item.affixes.push(Affix::roll(
            "s2".to_string(),
            AffixKind::Suffix,
            "mf".to_string(),
            10,
            25,
            &mut rng,
        ));
        let suffixes: Vec<_> = item.suffixes().collect();
        assert_eq!(suffixes.len(), 2);
        assert!(suffixes.iter().all(|a| a.kind == AffixKind::Suffix));
    }

    // ========================================================================
    // Affix tests
    // ========================================================================

    #[test]
    fn affix_roll_within_range() {
        let mut rng = test_rng();
        for _ in 0..100 {
            let affix = Affix::roll(
                "test".to_string(),
                AffixKind::Prefix,
                "stat".to_string(),
                10,
                50,
                &mut rng,
            );
            assert!(
                affix.rolled_value >= 10 && affix.rolled_value <= 50,
                "rolled_value {} out of range [10, 50]",
                affix.rolled_value
            );
        }
    }

    // ========================================================================
    // Inventory tests
    // ========================================================================

    #[test]
    fn inventory_new_is_empty() {
        let inv = Inventory::new();
        assert_eq!(inv.item_count(), 0);
        assert!(!inv.is_full());
    }

    #[test]
    fn inventory_place_and_get() {
        let mut inv = Inventory::new();
        let item = make_item("cap");
        let item_id = item.instance_id;
        inv.try_place(item, 0, 0).unwrap();
        let stored = inv.get(0, 0).unwrap();
        assert_eq!(stored.instance_id, item_id);
        assert_eq!(inv.item_count(), 1);
    }

    #[test]
    fn inventory_place_out_of_bounds_error() {
        let mut inv = Inventory::new();
        let item = make_item("scroll");
        let result = inv.try_place(item, INV_COLS, INV_ROWS);
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("out of bounds"));
    }

    #[test]
    fn inventory_place_occupied_error() {
        let mut inv = Inventory::new();
        inv.try_place(make_item("potion_hp"), 3, 2).unwrap();
        let result = inv.try_place(make_item("potion_mp"), 3, 2);
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("already occupied"));
    }

    #[test]
    fn inventory_remove() {
        let mut inv = Inventory::new();
        let item = make_item("key");
        let item_id = item.instance_id;
        inv.try_place(item, 5, 1).unwrap();
        let removed = inv.remove(5, 1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().instance_id, item_id);
        assert!(inv.get(5, 1).is_none());
        assert_eq!(inv.item_count(), 0);
    }

    #[test]
    fn inventory_find_free_slot() {
        let inv = Inventory::new();
        assert_eq!(inv.find_free_slot(), Some((0, 0)));

        let mut inv2 = Inventory::new();
        inv2.try_place(make_item("a"), 0, 0).unwrap();
        assert_eq!(inv2.find_free_slot(), Some((1, 0)));
    }

    #[test]
    fn inventory_is_full() {
        let mut inv = Inventory::new();
        for row in 0..INV_ROWS {
            for col in 0..INV_COLS {
                inv.try_place(make_item("filler"), col, row).unwrap();
            }
        }
        assert!(inv.is_full());
        assert_eq!(inv.item_count(), INV_COLS * INV_ROWS);
        assert!(inv.find_free_slot().is_none());
    }

    // ========================================================================
    // Equipment tests
    // ========================================================================

    #[test]
    fn equipment_equip_and_get() {
        let mut eq = Equipment::new();
        let item = make_item("plate_mail");
        let item_id = item.instance_id;
        let prev = eq.equip(ItemSlot::Armor, item);
        assert!(prev.is_none());
        let got = eq.get(ItemSlot::Armor).unwrap();
        assert_eq!(got.instance_id, item_id);
    }

    #[test]
    fn equipment_equip_returns_previous() {
        let mut eq = Equipment::new();
        let first = make_item("cap");
        let first_id = first.instance_id;
        eq.equip(ItemSlot::Helm, first);
        let second = make_item("crown");
        let prev = eq.equip(ItemSlot::Helm, second);
        assert!(prev.is_some());
        assert_eq!(prev.unwrap().instance_id, first_id);
    }

    #[test]
    fn equipment_unequip_empty() {
        let mut eq = Equipment::new();
        let result = eq.unequip(ItemSlot::Boots);
        assert!(result.is_none());
    }
}
