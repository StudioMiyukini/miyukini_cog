// @id: MGE-ARPG-Stats-Tests @do: tests @role: back-end @layer: 3 @human: miyuk
//! Unit tests for the mge-arpg-stats crate.

#[cfg(test)]
mod tests {
    use crate::base::BaseStats;
    use crate::block::StatBlock;
    use crate::derived::DerivedStats;
    use crate::level::{CharacterLevel, ExpTable};
    use crate::stat_value::StatValue;

    // ---- StatValue ----

    #[test]
    fn stat_value_effective_no_modifier() {
        let sv = StatValue::new(25);
        assert_eq!(sv.effective(), 25);
    }

    #[test]
    fn stat_value_effective_with_add() {
        let mut sv = StatValue::new(10);
        sv.add(20);
        // (10 + 20) * 1.0 = 30
        assert_eq!(sv.effective(), 30);
    }

    #[test]
    fn stat_value_effective_with_multiply() {
        let mut sv = StatValue::new(100);
        sv.multiply(1.5);
        // (100 + 0) * 1.5 = 150
        assert_eq!(sv.effective(), 150);
    }

    #[test]
    fn stat_value_effective_combined() {
        let mut sv = StatValue::new(50);
        sv.add(10);
        sv.multiply(1.2);
        // floor((50 + 10) * 1.2) = floor(72.0) = 72
        assert_eq!(sv.effective(), 72);
    }

    // ---- DerivedStats ----

    #[test]
    fn derived_stats_life_from_vitality() {
        let base = BaseStats::new(10, 10, 30, 10);
        let derived = DerivedStats::from_base(&base, "barbarian");
        // max_life = 30 * 4 + 50 = 170
        assert_eq!(derived.max_life, 170);
    }

    #[test]
    fn derived_stats_mana_from_energy() {
        let base = BaseStats::new(10, 10, 10, 50);
        let derived = DerivedStats::from_base(&base, "sorceress");
        // max_mana = 50 * 2 + 10 = 110
        assert_eq!(derived.max_mana, 110);
    }

    #[test]
    fn derived_stats_resistance_cap() {
        let base = BaseStats::new(10, 10, 10, 10);
        let mut derived = DerivedStats::from_base(&base, "paladin");
        derived.fire_res = 100;
        derived.cold_res = 80;
        derived.light_res = 75;
        derived.poison_res = 50;
        derived.apply_resistance_cap();
        assert_eq!(derived.fire_res, 75);
        assert_eq!(derived.cold_res, 75);
        assert_eq!(derived.light_res, 75);
        assert_eq!(derived.poison_res, 50);
    }

    // ---- ExpTable ----

    #[test]
    fn exp_table_level_1_requires_0_xp() {
        let table = ExpTable::d2_standard();
        assert_eq!(table.xp_for_level(1), Some(0));
    }

    #[test]
    fn exp_table_level_2_requires_500_xp() {
        let table = ExpTable::d2_standard();
        assert_eq!(table.xp_for_level(2), Some(500));
    }

    #[test]
    fn exp_table_level_for_xp_correct() {
        let table = ExpTable::d2_standard();
        // 0 XP -> level 1
        assert_eq!(table.level_for_xp(0), 1);
        // 499 XP -> still level 1
        assert_eq!(table.level_for_xp(499), 1);
        // 500 XP -> level 2
        assert_eq!(table.level_for_xp(500), 2);
        // Way past level 2 threshold but below level 3
        // Level 3 threshold = round(500 * 1.15) = 575
        assert_eq!(table.level_for_xp(574), 2);
        assert_eq!(table.level_for_xp(575), 3);
    }

    // ---- CharacterLevel ----

    #[test]
    fn char_level_gain_xp_levels_up() {
        let table = ExpTable::d2_standard();
        let mut cl = CharacterLevel::new();
        assert_eq!(cl.level, 1);
        // Give enough XP to reach level 2
        cl.add_experience(500, &table);
        assert_eq!(cl.level, 2);
        assert_eq!(cl.stat_points, 5);
        assert_eq!(cl.skill_points, 1);
    }

    #[test]
    fn char_level_spend_stat_point() {
        let table = ExpTable::d2_standard();
        let mut cl = CharacterLevel::new();
        cl.add_experience(500, &table); // level 2 -> 5 stat points
        assert!(cl.spend_stat_point());
        assert_eq!(cl.stat_points, 4);
    }

    #[test]
    fn char_level_spend_skill_point_empty() {
        let cl = &mut CharacterLevel::new();
        // Level 1, no skill points
        assert!(!cl.spend_skill_point());
    }

    // ---- StatBlock ----

    #[test]
    fn stat_block_take_damage_reduces_life() {
        let base = BaseStats::new(10, 10, 20, 10);
        let mut block = StatBlock::new(base, "amazon");
        let initial = block.current_life;
        let dealt = block.take_damage(30);
        assert_eq!(dealt, 30);
        assert_eq!(block.current_life, initial - 30);
    }

    #[test]
    fn stat_block_take_damage_not_below_zero() {
        let base = BaseStats::new(10, 10, 10, 10);
        let mut block = StatBlock::new(base, "necromancer");
        let max = block.derived.max_life;
        let dealt = block.take_damage(max + 999);
        assert_eq!(dealt, max);
        assert_eq!(block.current_life, 0);
    }

    #[test]
    fn stat_block_is_alive_dead_at_zero() {
        let base = BaseStats::new(10, 10, 10, 10);
        let mut block = StatBlock::new(base, "druid");
        assert!(block.is_alive());
        block.take_damage(block.derived.max_life);
        assert!(!block.is_alive());
    }

    #[test]
    fn stat_block_restore_life_caps_at_max() {
        let base = BaseStats::new(10, 10, 20, 10);
        let mut block = StatBlock::new(base, "paladin");
        let max_life = block.derived.max_life;
        block.take_damage(50);
        block.restore_life(9999);
        assert_eq!(block.current_life, max_life);
    }
}
