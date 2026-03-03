// @id: MGE-UI-Tests-V2 @do: ui-logic-tests-gap04 @role: front-end @layer: 3 @human: miyuk
//! UI logic tests — V2.2 / V2.3 data models (GAP-04 compliance).
//!
//! Covers: `InventoryGrid`, `PaperdollState`, `SkillTreeState`, `ItemTooltipData`,
//! `UiState`, `item_quality_color`, `PotionType`, `HudState`, `color_for_quality`,
//! `compare_stats`, `can_equip`.
//!
//! All tests are pure Rust — no egui `Context`, no wgpu device required.

#[cfg(test)]
mod ui_tests {
    use crate::character::{can_equip, EquipSlot, EquippedItem, PaperdollState};
    use crate::hud::{BeltState, PotionType};
    use crate::inventory::{InventoryError, InventoryGrid, item_size, ItemSize, GRID_COLS, GRID_ROWS};
    use crate::skill_tree::{SkillNodeState, SkillTreeState, SkillTreeTab};
    use crate::tooltip::{
        color_for_quality, compare_stats, unidentified_tooltip,
        COLOR_BETTER, COLOR_WORSE,
    };
    use crate::UiState;

    // -----------------------------------------------------------------------
    // Helper builders
    // -----------------------------------------------------------------------

    fn make_equipped(id: &str, base_type: &str) -> EquippedItem {
        EquippedItem {
            item_id: id.to_string(),
            base_type: base_type.to_string(),
            display_name: id.to_string(),
        }
    }

    fn make_skill_node(skill_id: &str, tab: SkillTreeTab) -> SkillNodeState {
        SkillNodeState {
            skill_id: skill_id.to_string(),
            name: skill_id.to_string(),
            level: 0,
            max_level: 20,
            required_level: 1,
            prereqs_met: true,
            investable: true,
            mana_cost: 4.0,
            tab,
            grid_x: 0,
            grid_y: 0,
            prereq_ids: vec![],
        }
    }

    fn approx_eq_color(a: [f32; 4], b: [f32; 4]) -> bool {
        a.iter().zip(b.iter()).all(|(x, y)| (x - y).abs() < 1e-6)
    }

    // -----------------------------------------------------------------------
    // Inventory Grid tests (T1–T3)
    // -----------------------------------------------------------------------

    /// T1 — fill entire row 0 with 10 × 1x1 rings; all placements must succeed.
    #[test]
    fn inventory_grid_full_row() {
        let mut grid = InventoryGrid::new();
        for col in 0u8..10 {
            grid.place_item(format!("ring-{col}"), "ring", col, 0)
                .unwrap_or_else(|e| panic!("place col {col} failed: {e}"));
        }
        assert_eq!(grid.item_count(), 10, "full row 0 must hold exactly 10 items");
        for col in 0u8..10 {
            assert!(
                grid.item_at(col, 0).is_some(),
                "cell ({col},0) must be occupied after filling row 0"
            );
        }
    }

    /// T2 — place a 2x3 armor at (0,0); all 6 covered cells must resolve to it.
    #[test]
    fn inventory_2x3_armor_placement() {
        let mut grid = InventoryGrid::new();
        // "armor" → ItemSize { width: 2, height: 3 }
        grid.place_item("chest-01", "armor", 0, 0)
            .expect("armor placement must succeed");

        assert_eq!(grid.item_count(), 1);

        for row in 0u8..3 {
            for col in 0u8..2 {
                let slot = grid.item_at(col, row)
                    .unwrap_or_else(|| panic!("cell ({col},{row}) must be occupied by armor"));
                assert_eq!(slot.item_id, "chest-01");
            }
        }
        // Cell just outside the footprint must be free.
        assert!(
            grid.item_at(2, 0).is_none(),
            "cell (2,0) must be empty — armor width is 2"
        );
    }

    /// T3 — place, remove, then re-place at the same spot; second placement must succeed.
    #[test]
    fn inventory_remove_and_reuse_space() {
        let mut grid = InventoryGrid::new();
        let idx = grid.place_item("gem-a", "ring", 3, 2).expect("first place");
        let removed = grid.remove_item(idx).expect("remove must return the slot");
        assert_eq!(removed.item_id, "gem-a");

        grid.place_item("gem-b", "ring", 3, 2)
            .expect("re-place at same spot must succeed after removal");
        let slot = grid.item_at(3, 2).expect("cell (3,2) must be occupied again");
        assert_eq!(slot.item_id, "gem-b");
    }

    // -----------------------------------------------------------------------
    // PaperdollState tests (T4–T6)
    // -----------------------------------------------------------------------

    /// T4 — equip one item in each of the 10 paperdoll slots; all must be Some.
    #[test]
    fn paperdoll_all_slots() {
        let mut pd = PaperdollState::new();

        let entries = [
            (EquipSlot::Helm, "helm"),
            (EquipSlot::Armor, "armor"),
            (EquipSlot::Belt, "belt"),
            (EquipSlot::Boots, "boots"),
            (EquipSlot::Gloves, "gloves"),
            (EquipSlot::Amulet, "amulet"),
            (EquipSlot::Ring1, "ring"),
            (EquipSlot::Ring2, "ring"),
            (EquipSlot::Weapon, "sword"),
            (EquipSlot::Shield, "shield"),
        ];

        for (slot, base_type) in entries {
            let item = make_equipped(&format!("{base_type}-01"), base_type);
            pd.equip(slot, item)
                .unwrap_or_else(|e| panic!("equip {slot:?} failed: {e}"));
        }

        for (slot, _) in entries {
            assert!(
                pd.get(slot).is_some(),
                "slot {slot:?} must be occupied after equipping"
            );
        }
    }

    /// T5 — equip a sword, then swap it; first sword is returned, second is equipped.
    #[test]
    fn paperdoll_swap_weapon() {
        let mut pd = PaperdollState::new();

        let sword1 = make_equipped("sword-first", "sword");
        let prev = pd.equip(EquipSlot::Weapon, sword1).expect("first equip");
        assert!(prev.is_none(), "no previous item on fresh paperdoll");

        let sword2 = make_equipped("sword-second", "sword");
        let returned = pd.equip(EquipSlot::Weapon, sword2).expect("second equip");

        let returned_item = returned.expect("equipping over occupied slot must return old item");
        assert_eq!(returned_item.item_id, "sword-first", "first sword must be returned");

        let equipped_now = pd.get(EquipSlot::Weapon).expect("slot must be occupied");
        assert_eq!(equipped_now.item_id, "sword-second", "second sword must now be in Weapon slot");
    }

    /// T6 — equip a ring in Ring1 and Ring2 simultaneously.
    #[test]
    fn paperdoll_ring_both_slots() {
        let mut pd = PaperdollState::new();
        pd.equip(EquipSlot::Ring1, make_equipped("ring-left", "ring"))
            .expect("Ring1 equip");
        pd.equip(EquipSlot::Ring2, make_equipped("ring-right", "ring"))
            .expect("Ring2 equip");

        let r1 = pd.get(EquipSlot::Ring1).expect("Ring1 must be occupied");
        let r2 = pd.get(EquipSlot::Ring2).expect("Ring2 must be occupied");
        assert_ne!(r1.item_id, r2.item_id, "both ring slots must hold different items");
    }

    // -----------------------------------------------------------------------
    // Skill Tree tests (T7–T9)
    // -----------------------------------------------------------------------

    /// T7 — `toggle()` flips `visible` from false to true, then back to false.
    #[test]
    fn skill_tree_toggle_visibility() {
        let mut state = SkillTreeState::new();
        assert!(!state.visible, "fresh state must be invisible");

        state.toggle();
        assert!(state.visible, "first toggle must make state visible");

        state.toggle();
        assert!(!state.visible, "second toggle must hide the state again");
    }

    /// T8 — `set_tab(Curses)` must update `active_tab` to `Curses`.
    #[test]
    fn skill_tree_switch_tabs() {
        let mut state = SkillTreeState::new();
        assert_eq!(state.active_tab, SkillTreeTab::Summoning);

        state.set_tab(SkillTreeTab::Curses);
        assert_eq!(state.active_tab, SkillTreeTab::Curses);

        state.set_tab(SkillTreeTab::PoisonBone);
        assert_eq!(state.active_tab, SkillTreeTab::PoisonBone);
    }

    /// T9 — add 3 nodes per tab; `nodes_for_tab` must return exactly 3 per tab.
    #[test]
    fn skill_tree_nodes_filter_count() {
        let mut state = SkillTreeState::new();

        for i in 0u8..3 {
            state.nodes.push(make_skill_node(&format!("summon-{i}"), SkillTreeTab::Summoning));
            state.nodes.push(make_skill_node(&format!("poison-{i}"), SkillTreeTab::PoisonBone));
            state.nodes.push(make_skill_node(&format!("curse-{i}"), SkillTreeTab::Curses));
        }

        assert_eq!(state.nodes.len(), 9, "total nodes must be 9");
        assert_eq!(state.nodes_for_tab(SkillTreeTab::Summoning).len(), 3);
        assert_eq!(state.nodes_for_tab(SkillTreeTab::PoisonBone).len(), 3);
        assert_eq!(state.nodes_for_tab(SkillTreeTab::Curses).len(), 3);
    }

    // -----------------------------------------------------------------------
    // Tooltip tests (T10–T11)
    // -----------------------------------------------------------------------

    /// T10 — tooltip for an unidentified item must have empty comparison and `is_identified = false`.
    #[test]
    fn tooltip_empty_comparison() {
        let data = unidentified_tooltip("sword", "normal");
        assert!(
            data.comparison.is_empty(),
            "unidentified tooltip must have empty comparison"
        );
        assert!(
            !data.is_identified,
            "unidentified tooltip must have is_identified = false"
        );
    }

    /// T11 — `color_for_quality` must return a distinct value for each of the 5 quality strings.
    #[test]
    fn tooltip_all_quality_colors() {
        let qualities = ["normal", "magic", "rare", "set", "unique"];
        let colors: Vec<[f32; 4]> = qualities.iter().map(|q| color_for_quality(*q)).collect();

        // All consecutive pairs must differ.
        for i in 0..colors.len() - 1 {
            assert!(
                !approx_eq_color(colors[i], colors[i + 1]),
                "quality '{}' and '{}' must produce different colors",
                qualities[i],
                qualities[i + 1]
            );
        }
    }

    // -----------------------------------------------------------------------
    // Integration-style tests (T12–T15)
    // -----------------------------------------------------------------------

    /// T12 — fill all 40 cells of the 10×4 grid; a 41st placement must fail.
    #[test]
    fn inventory_max_capacity() {
        let mut grid = InventoryGrid::new();

        for row in 0u8..GRID_ROWS as u8 {
            for col in 0u8..GRID_COLS as u8 {
                grid.place_item(format!("ring-{row}-{col}"), "ring", col, row)
                    .expect("placement must succeed while grid has space");
            }
        }

        assert_eq!(grid.item_count(), 40, "grid must report 40 items");

        let result = grid.place_item("ring-extra", "ring", 0, 0);
        assert!(
            matches!(result, Err(InventoryError::Overlap { .. })),
            "placing a 41st item at (0,0) must return Overlap, got {result:?}"
        );
    }

    /// T13 — equip then unequip; item data must be preserved intact.
    #[test]
    fn paperdoll_unequip_returns_item() {
        let mut pd = PaperdollState::new();
        pd.equip(EquipSlot::Armor, make_equipped("unique-armor-of-doom", "armor"))
            .expect("equip");

        let unequipped = pd.unequip(EquipSlot::Armor).expect("unequip must return item");
        assert_eq!(unequipped.item_id, "unique-armor-of-doom");
        assert!(pd.get(EquipSlot::Armor).is_none(), "slot must be empty after unequip");
    }

    /// T14 — investing in a skill increments `level` and decrements `available_points`.
    #[test]
    fn skill_tree_invest_updates_level() {
        let mut state = SkillTreeState::new();
        state.available_points = 3;
        state.nodes.push(make_skill_node("raise_skeleton", SkillTreeTab::Summoning));

        let ok = state.invest("raise_skeleton");
        assert!(ok, "invest must return true for an investable node");

        let node = state.nodes.iter().find(|n| n.skill_id == "raise_skeleton").unwrap();
        assert_eq!(node.level, 1, "level must be 1 after first investment");
        assert_eq!(state.available_points, 2, "available_points must decrease by 1");

        // Second investment.
        state.nodes[0].investable = true;
        let ok2 = state.invest("raise_skeleton");
        assert!(ok2, "second invest must succeed");
        assert_eq!(state.nodes[0].level, 2);
        assert_eq!(state.available_points, 1);
    }

    /// T15 — `BeltState` is independent of `InventoryGrid`; modifications are isolated.
    #[test]
    fn belt_and_inventory_independent() {
        let mut grid = InventoryGrid::new();
        let mut belt = BeltState::new();

        grid.place_item("ring-01", "ring", 0, 0).expect("grid place");
        belt.set_slot(0, Some(PotionType::HpSmall));

        // Remove from inventory.
        grid.remove_item(0);
        assert_eq!(grid.item_count(), 0, "grid must be empty after remove");

        // Belt must be unaffected.
        assert_eq!(
            belt.slots[0],
            Some(PotionType::HpSmall),
            "belt slot must still hold HpSmall after inventory change"
        );

        belt.use_slot(0);
        assert_eq!(belt.slots[0], None, "belt slot must be empty after use");
        assert_eq!(grid.item_count(), 0, "grid must still be empty");
    }

    // -----------------------------------------------------------------------
    // Bonus tests (T16–T19) — ItemSize, can_equip, UiState, compare_stats
    // -----------------------------------------------------------------------

    /// T16 — `item_size` must return the canonical footprint for each base type category.
    #[test]
    fn item_size_canonical_footprints() {
        assert_eq!(item_size("ring"), ItemSize { width: 1, height: 1 }, "ring must be 1x1");
        assert_eq!(item_size("amulet"), ItemSize { width: 1, height: 1 }, "amulet must be 1x1");
        assert_eq!(item_size("helm"), ItemSize { width: 2, height: 2 }, "helm must be 2x2");
        assert_eq!(item_size("sword"), ItemSize { width: 1, height: 3 }, "sword must be 1x3");
        assert_eq!(item_size("bow"), ItemSize { width: 2, height: 4 }, "bow must be 2x4");
        assert_eq!(item_size("armor"), ItemSize { width: 2, height: 3 }, "armor must be 2x3");
        assert_eq!(
            item_size("unknown_xyz"),
            ItemSize { width: 1, height: 1 },
            "unknown type must default to 1x1"
        );
    }

    /// T17 — `can_equip` must enforce correct slot/type pairings.
    #[test]
    fn can_equip_correct_and_wrong_slots() {
        // Valid pairings.
        assert!(can_equip(EquipSlot::Helm, "helm"), "helm type goes in Helm slot");
        assert!(can_equip(EquipSlot::Ring1, "ring"), "ring type goes in Ring1");
        assert!(can_equip(EquipSlot::Ring2, "ring"), "ring type goes in Ring2");
        assert!(can_equip(EquipSlot::Weapon, "sword"), "sword goes in Weapon");
        assert!(can_equip(EquipSlot::Shield, "buckler"), "buckler goes in Shield");

        // Invalid pairings.
        assert!(!can_equip(EquipSlot::Boots, "helm"), "helm must not go in Boots");
        assert!(!can_equip(EquipSlot::Weapon, "ring"), "ring must not go in Weapon");
        assert!(!can_equip(EquipSlot::Helm, "armor"), "armor must not go in Helm");
    }

    /// T18 — `UiState` toggle helpers must flip the correct panel flags independently.
    #[test]
    fn ui_state_toggle_panels() {
        let mut state = UiState::new();
        assert!(!state.panels.inventory);
        assert!(!state.panels.character);
        assert!(!state.panels.skill_tree);

        state.toggle_inventory();
        assert!(state.panels.inventory);
        assert!(!state.panels.character, "character must remain closed");
        assert!(!state.panels.skill_tree, "skill_tree must remain closed");

        state.toggle_character();
        assert!(state.panels.character);

        state.toggle_skill_tree();
        assert!(state.panels.skill_tree);

        // Toggle all back.
        state.toggle_inventory();
        state.toggle_character();
        state.toggle_skill_tree();
        assert!(!state.panels.inventory);
        assert!(!state.panels.character);
        assert!(!state.panels.skill_tree);
    }

    /// T19 — `compare_stats`: new stat in current but not in equipped → green "+value" line.
    ///        Lost stat in equipped but not in current → red "-value" line.
    #[test]
    fn compare_stats_new_and_lost() {
        let current = vec![("poison_resist".to_string(), 25_i32)];
        let equipped = vec![("fire_resist".to_string(), 30_i32)];

        let lines = compare_stats(&current, &equipped);

        // Expect two lines: one gain (poison_resist), one loss (fire_resist).
        assert_eq!(lines.len(), 2, "must produce 2 comparison lines");

        let gain = lines.iter().find(|l| l.text.contains("poison_resist"))
            .expect("poison_resist gain line must exist");
        assert!(
            approx_eq_color(gain.color, COLOR_BETTER),
            "new stat must be green (BETTER)"
        );
        assert!(gain.text.contains("+25"), "gain line must show +25");

        let loss = lines.iter().find(|l| l.text.contains("fire_resist"))
            .expect("fire_resist loss line must exist");
        assert!(
            approx_eq_color(loss.color, COLOR_WORSE),
            "lost stat must be red (WORSE)"
        );
        assert!(loss.text.contains("-30"), "loss line must show -30");
    }
}
