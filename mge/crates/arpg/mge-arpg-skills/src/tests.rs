// @id: MGE-ARPG-SkillTests @do: tests @role: back-end @layer: 3 @human: miyuk
//! Unit and integration tests for the mge-arpg-skills crate.

#[cfg(test)]
mod tests {
    use mge_arpg_combat::DamageType;

    use crate::{
        can_use_skill, necro, Cooldown, SkillBook, SkillCooldownTracker, SkillDef, SkillError,
        SkillId, SkillKind, SkillRegistry, SynergyCalculator, SynergyDef,
    };

    // ── Helpers ──────────────────────────────────────────────────────────

    fn make_basic_def(id: &str) -> SkillDef {
        SkillDef {
            id: SkillId::new(id),
            name: id.to_string(),
            max_level: 20,
            prerequisites: Vec::new(),
            synergies: Vec::new(),
            mana_cost_base: 5.0,
            mana_cost_per_level: 1.5,
            cooldown_ms: 0,
            tree: 0,
            damage_type: DamageType::Physical,
            kind: SkillKind::Projectile,
            base_damage_min: 10,
            base_damage_max: 15,
            damage_per_level: 3,
            synergy_ids: Vec::new(),
        }
    }

    fn make_def_with_prereq(id: &str, prereqs: &[&str]) -> SkillDef {
        SkillDef {
            prerequisites: prereqs.iter().map(|p| SkillId::new(*p)).collect(),
            ..make_basic_def(id)
        }
    }

    fn make_def_with_synergy(id: &str, source: &str, bonus: f32, stat: &str) -> SkillDef {
        SkillDef {
            synergies: vec![SynergyDef {
                source_skill: SkillId::new(source),
                bonus_per_level: bonus,
                stat: stat.to_string(),
            }],
            ..make_basic_def(id)
        }
    }

    // ── SkillId tests ───────────────────────────────────────────────────

    #[test]
    fn skill_id_as_str() {
        let id = SkillId::new("fireball");
        assert_eq!(id.as_str(), "fireball");
    }

    #[test]
    fn skill_id_display() {
        let id = SkillId::new("frozen_orb");
        assert_eq!(format!("{id}"), "frozen_orb");
    }

    // ── SkillDef mana cost ──────────────────────────────────────────────

    #[test]
    fn skill_def_mana_cost_level_1() {
        let def = make_basic_def("fireball");
        // base=5.0, per_level=1.5 => 5.0 + 1.5*1 = 6.5
        let cost = def.mana_cost(1);
        assert!((cost - 6.5).abs() < f32::EPSILON);
    }

    #[test]
    fn skill_def_mana_cost_level_5() {
        let def = make_basic_def("fireball");
        // base=5.0, per_level=1.5 => 5.0 + 1.5*5 = 12.5
        let cost = def.mana_cost(5);
        assert!((cost - 12.5).abs() < f32::EPSILON);
    }

    // ── SkillRegistry ───────────────────────────────────────────────────

    #[test]
    fn registry_register_and_get() {
        let mut reg = SkillRegistry::new();
        let def = make_basic_def("fireball");
        reg.register(def);
        assert_eq!(reg.len(), 1);

        let retrieved = reg.get(&SkillId::new("fireball"));
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "fireball");
    }

    #[test]
    fn registry_get_missing() {
        let reg = SkillRegistry::new();
        assert!(reg.get(&SkillId::new("nonexistent")).is_none());
    }

    // ── SkillBook ───────────────────────────────────────────────────────

    #[test]
    fn skill_book_invest_ok() {
        let mut reg = SkillRegistry::new();
        reg.register(make_basic_def("fireball"));

        let mut book = SkillBook::new(5);
        let result = book.invest(&SkillId::new("fireball"), &reg);
        assert!(result.is_ok());
        assert_eq!(book.level_of(&SkillId::new("fireball")), 1);
        assert_eq!(book.available_points(), 4);
    }

    #[test]
    fn skill_book_invest_no_points() {
        let mut reg = SkillRegistry::new();
        reg.register(make_basic_def("fireball"));

        let mut book = SkillBook::new(0);
        let result = book.invest(&SkillId::new("fireball"), &reg);
        assert!(matches!(result, Err(SkillError::NoPointsAvailable)));
    }

    #[test]
    fn skill_book_invest_missing_skill() {
        let reg = SkillRegistry::new();
        let mut book = SkillBook::new(5);
        let result = book.invest(&SkillId::new("nonexistent"), &reg);
        assert!(matches!(result, Err(SkillError::NotFound { .. })));
    }

    #[test]
    fn skill_book_invest_prerequisites_not_met() {
        let mut reg = SkillRegistry::new();
        reg.register(make_basic_def("fire_bolt"));
        reg.register(make_def_with_prereq("fireball", &["fire_bolt"]));

        let mut book = SkillBook::new(5);
        // Try to invest in fireball without having fire_bolt
        let result = book.invest(&SkillId::new("fireball"), &reg);
        assert!(matches!(result, Err(SkillError::PrerequisitesNotMet { .. })));

        // Now invest in fire_bolt first
        book.invest(&SkillId::new("fire_bolt"), &reg).unwrap();
        let result = book.invest(&SkillId::new("fireball"), &reg);
        assert!(result.is_ok());
    }

    #[test]
    fn skill_book_invest_max_level() {
        let mut reg = SkillRegistry::new();
        let mut def = make_basic_def("fireball");
        def.max_level = 2;
        reg.register(def);

        let mut book = SkillBook::new(10);
        book.invest(&SkillId::new("fireball"), &reg).unwrap();
        book.invest(&SkillId::new("fireball"), &reg).unwrap();
        assert_eq!(book.level_of(&SkillId::new("fireball")), 2);

        let result = book.invest(&SkillId::new("fireball"), &reg);
        assert!(matches!(result, Err(SkillError::MaxLevelReached { max: 2, .. })));
    }

    // ── Cooldown ────────────────────────────────────────────────────────

    #[test]
    fn cooldown_initially_ready() {
        let cd = Cooldown::new(1000);
        assert!(cd.is_ready());
        assert_eq!(cd.remaining_ms(), 0);
    }

    #[test]
    fn cooldown_trigger_not_ready() {
        let mut cd = Cooldown::new(1000);
        cd.trigger();
        assert!(!cd.is_ready());
        assert_eq!(cd.remaining_ms(), 1000);
    }

    #[test]
    fn cooldown_tick_decrements() {
        let mut cd = Cooldown::new(1000);
        cd.trigger();
        cd.tick(300);
        assert_eq!(cd.remaining_ms(), 700);
        assert!(!cd.is_ready());
    }

    #[test]
    fn cooldown_tick_to_zero_ready() {
        let mut cd = Cooldown::new(500);
        cd.trigger();
        cd.tick(500);
        assert!(cd.is_ready());
        assert_eq!(cd.remaining_ms(), 0);
    }

    #[test]
    fn cooldown_tracker_tick_all() {
        let mut tracker = SkillCooldownTracker::new();
        let id_a = SkillId::new("fireball");
        let id_b = SkillId::new("ice_bolt");

        tracker.register(id_a.clone(), 1000);
        tracker.register(id_b.clone(), 500);

        tracker.trigger(&id_a);
        tracker.trigger(&id_b);

        assert!(!tracker.is_ready(&id_a));
        assert!(!tracker.is_ready(&id_b));

        tracker.tick_all(500);

        assert!(!tracker.is_ready(&id_a)); // 500 remaining
        assert!(tracker.is_ready(&id_b)); // 0 remaining

        tracker.tick_all(500);
        assert!(tracker.is_ready(&id_a)); // now ready
    }

    // ── SynergyCalculator ───────────────────────────────────────────────

    #[test]
    fn synergy_calculator_no_synergy() {
        let mut reg = SkillRegistry::new();
        reg.register(make_basic_def("fireball"));

        let book = SkillBook::new(0);
        let bonus = SynergyCalculator::calculate_bonus(
            &SkillId::new("fireball"),
            "damage",
            &reg,
            &book,
        );
        assert!((bonus - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn synergy_calculator_with_levels() {
        let mut reg = SkillRegistry::new();
        reg.register(make_basic_def("fire_bolt"));
        reg.register(make_def_with_synergy("fireball", "fire_bolt", 0.06, "damage"));

        let mut book = SkillBook::new(10);
        // Invest 5 levels in fire_bolt
        for _ in 0..5 {
            book.invest(&SkillId::new("fire_bolt"), &reg).unwrap();
        }

        let bonus = SynergyCalculator::calculate_bonus(
            &SkillId::new("fireball"),
            "damage",
            &reg,
            &book,
        );
        // 0.06 * 5 = 0.30
        assert!((bonus - 0.30).abs() < f32::EPSILON);
    }

    // ══════════════════════════════════════════════════════════════════════
    // TASK-040 : Necromancer skill definitions
    // ══════════════════════════════════════════════════════════════════════

    #[test]
    fn bone_spear_def() {
        let def = necro::bone_spear();
        assert_eq!(def.id.as_str(), necro::BONE_SPEAR_ID);
        assert_eq!(def.name, "Bone Spear");
        assert_eq!(def.damage_type, DamageType::Magic);
        assert_eq!(def.kind, SkillKind::Projectile);
        assert!((def.mana_cost_base - 7.0).abs() < f32::EPSILON);
        assert_eq!(def.base_damage_min, 15);
        assert_eq!(def.base_damage_max, 17);
        assert_eq!(def.damage_per_level, 5);
        // At level 5: min=15+25=40, max=17+25=42
        let (dmin, dmax) = def.damage_range(5);
        assert_eq!(dmin, 40);
        assert_eq!(dmax, 42);
    }

    #[test]
    fn raise_skeleton_max_count() {
        // level 0 -> 2, level 1 -> 2, level 3 -> 3, level 6 -> 4,
        // level 9 -> 5, level 12 -> 6, level 18 -> 8, level 20 -> 8
        assert_eq!(necro::raise_skeleton_max_count(0), 2);
        assert_eq!(necro::raise_skeleton_max_count(1), 2);
        assert_eq!(necro::raise_skeleton_max_count(3), 3);
        assert_eq!(necro::raise_skeleton_max_count(6), 4);
        assert_eq!(necro::raise_skeleton_max_count(9), 5);
        assert_eq!(necro::raise_skeleton_max_count(12), 6);
        assert_eq!(necro::raise_skeleton_max_count(18), 8);
        assert_eq!(necro::raise_skeleton_max_count(20), 8);
    }

    #[test]
    fn registry_get_necro() {
        let mut reg = SkillRegistry::new();
        necro::register_all(&mut reg);
        assert_eq!(reg.len(), 3);

        let bs = reg.get(&SkillId::new(necro::BONE_SPEAR_ID));
        assert!(bs.is_some());
        assert_eq!(bs.unwrap().name, "Bone Spear");

        let rs = reg.get(&SkillId::new(necro::RAISE_SKELETON_ID));
        assert!(rs.is_some());
        assert_eq!(rs.unwrap().name, "Raise Skeleton");

        let ce = reg.get(&SkillId::new(necro::CORPSE_EXPLOSION_ID));
        assert!(ce.is_some());
        assert_eq!(ce.unwrap().name, "Corpse Explosion");
    }

    // ══════════════════════════════════════════════════════════════════════
    // TASK-041 : Skill activation (mana + cooldown + level)
    // ══════════════════════════════════════════════════════════════════════

    /// Helper: set up a registry with necro skills and a book with Bone Spear at level 1.
    fn necro_setup() -> (SkillRegistry, SkillBook, SkillCooldownTracker) {
        let mut reg = SkillRegistry::new();
        necro::register_all(&mut reg);

        let mut book = SkillBook::new(5);
        let bs_id = SkillId::new(necro::BONE_SPEAR_ID);
        book.invest(&bs_id, &reg).unwrap();

        let mut cooldowns = SkillCooldownTracker::new();
        // Bone Spear has 0 ms cooldown, but register it anyway for completeness.
        cooldowns.register(bs_id, 0);

        (reg, book, cooldowns)
    }

    #[test]
    fn activate_mana_check() {
        let (reg, book, cooldowns) = necro_setup();
        let bs_id = SkillId::new(necro::BONE_SPEAR_ID);
        // Bone Spear costs 7.0 mana. Give the character only 3.0 mana.
        assert!(!can_use_skill(&bs_id, &book, 3.0, &cooldowns, &reg));
    }

    #[test]
    fn activate_cooldown() {
        let mut reg = SkillRegistry::new();
        necro::register_all(&mut reg);

        let mut book = SkillBook::new(5);
        let rs_id = SkillId::new(necro::RAISE_SKELETON_ID);
        book.invest(&rs_id, &reg).unwrap();

        let mut cooldowns = SkillCooldownTracker::new();
        // Raise Skeleton has 500 ms cooldown.
        cooldowns.register(rs_id.clone(), 500);
        cooldowns.trigger(&rs_id);

        // Plenty of mana (100) but cooldown is active => cannot use.
        assert!(!can_use_skill(&rs_id, &book, 100.0, &cooldowns, &reg));

        // Tick the cooldown to completion.
        cooldowns.tick_all(500);
        assert!(can_use_skill(&rs_id, &book, 100.0, &cooldowns, &reg));
    }

    #[test]
    fn activate_success() {
        let (reg, book, cooldowns) = necro_setup();
        let bs_id = SkillId::new(necro::BONE_SPEAR_ID);
        // Bone Spear: level 1, mana cost 7.0, no cooldown.
        // Give character 50.0 mana => should succeed.
        assert!(can_use_skill(&bs_id, &book, 50.0, &cooldowns, &reg));
    }
}
