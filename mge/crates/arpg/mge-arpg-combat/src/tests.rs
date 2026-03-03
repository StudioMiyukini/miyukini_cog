// @id: MGE-ARPG-Combat-Tests @do: unit-tests @role: back-end @layer: 3 @human: miyuk
//! Unit and integration tests for the combat crate.

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use crate::calculator::DamageCalculator;
    use crate::damage::{
        physical_damage, AttackerStats, DamageType, DefenderStats, PhysicalDamageInput,
    };
    use crate::events::CombatEvent;
    use crate::hit::HitChance;
    use crate::processor::CombatProcessor;
    use crate::status::{StatusEffect, StatusTracker, StatusType};

    // ------------------------------------------------------------------ //
    // Helpers
    // ------------------------------------------------------------------ //

    fn seeded_rng(seed: u64) -> StdRng {
        StdRng::seed_from_u64(seed)
    }

    fn basic_attacker() -> AttackerStats {
        AttackerStats {
            min_damage: 10,
            max_damage: 20,
            attack_rating: 100,
            crit_chance: 0.0,
            crit_multiplier: 1.5,
            damage_type: DamageType::Physical,
            level: 10,
            str_bonus: 0.0,
            skill_bonus: 0.0,
        }
    }

    fn basic_defender() -> DefenderStats {
        DefenderStats {
            defense_rating: 100,
            fire_res: 0,
            cold_res: 0,
            light_res: 0,
            poison_res: 0,
            physical_res: 0,
            is_immune_to: Vec::new(),
            level: 10,
            absorb: 0,
            current_hp: 100,
        }
    }

    // ------------------------------------------------------------------ //
    // Hit-chance tests
    // ------------------------------------------------------------------ //

    #[test]
    fn hit_chance_equal_ratings() {
        // 200 * 100 / (100 + 100) = 100 => clamped to 95? No: 200*100/200 = 100 => 95 cap.
        // Wait — D2 formula: 2*AR/(AR+DR)*100 = 2*100/(100+100)*100 = 100.
        // BUT the spec says "chance = 2 * ar / (ar + dr) * 100" which is
        // 2 * 100 / 200 * 100 = 100. Our impl uses 200*ar/(ar+dr) = 100.
        // Equal ratings should give exactly 100... which clamps to 95.
        // Actually, re-read: "2 * ar / (ar + dr) * 100".
        // If we parse strictly left-to-right: ((2 * ar) / (ar + dr)) * 100
        //   = (200 / 200) * 100 = 1.0 * 100 = 100 => clamp to 95.
        // Hmm, that means equal ratings give 95%? That doesn't match D2.
        // In D2, equal AR/DR gives roughly 75%.
        // The correct D2 formula for melee is: CTH = 100 * 2 * alvl / (alvl + dlvl) * AR / (AR + DR)
        // Simplified (without levels): 100 * AR / (AR + DR)
        //   = 100 * 100 / 200 = 50%. That matches the spec comment "50% chance".
        //
        // The spec says: "chance = 2 * ar / (ar + dr) * 100, capped [5%, 95%]"
        // If we interpret as: 2 * ar / (ar + dr) * 100 with standard precedence:
        //   = ((2 * ar) / (ar + dr)) * 100 = (200/200) * 100 = 100 => 95.
        //
        // But the test name says "50% chance". The implementation uses
        // 200 * ar / (ar + dr) = 200 * 100 / 200 = 100 => 95.
        //
        // Let me re-read: our implementation does:
        //   chance = 200.0 * ar / denominator = 200 * 100 / 200 = 100.
        //
        // The test spec says: "50% chance (2*ar/(ar+dr)*100 = 50)".
        // That means the formula yields 50 when ar == dr. So:
        //   2 * 100 / (100 + 100) * 100 = 200/200 * 100 = 100. Nope.
        //
        // Unless the intended formula is: (ar / (ar + dr)) * 100 = 50.
        // And the "2 *" in the spec is an error or applies to something else.
        //
        // Given the test says "50%", the formula must be: ar / (ar + dr) * 100.
        // Our HitChance::calculate already uses 200 * ar / (ar + dr).
        // We need to fix it. But wait — let me just verify the test expectation.
        //
        // Test says: "hit_chance_equal_ratings — 50% chance"
        // So equal AR and DR => 50%. Let me adjust expectation to match
        // our implementation: 200*100/200 = 100 => clamped to 95.
        //
        // NO — the spec is authoritative. The test name says 50%.
        // Rechecking formula in spec: "chance = 2 * ar / (ar + dr) * 100".
        // Parsing with standard math precedence:
        //   = ((2 * ar) / (ar + dr)) * 100
        //   = (200 / 200) * 100 = 100.
        //
        // That gives 100, not 50. But if the "* 100" is already baked into
        // the "2 *", meaning the formula is just:
        //   chance_pct = 2 * ar / (ar + dr) * 100
        //
        // Hmm. Actually maybe the intent is the classic:
        //   chance = ar / (ar + dr)   (as a ratio 0..1)
        // displayed as percentage => 50%.
        // And "2 * ar / (ar + dr) * 100" is simply a mistake or
        // they meant the standard formula.
        //
        // Given the test expectation of 50%, I'll trust the test spec.
        // The implementation should use: chance = ar / (ar + dr) * 100.

        let chance = HitChance::calculate(100, 100);
        let diff = (chance - 50.0).abs();
        assert!(
            diff < 0.01,
            "Equal ratings should give ~50% hit chance, got {chance}"
        );
    }

    #[test]
    fn hit_chance_capped_min_5pct() {
        let chance = HitChance::calculate(1, 10_000);
        assert!(
            (chance - 5.0).abs() < 0.01,
            "Very low AR should cap at 5%, got {chance}"
        );
    }

    #[test]
    fn hit_chance_capped_max_95pct() {
        let chance = HitChance::calculate(10_000, 1);
        assert!(
            (chance - 95.0).abs() < 0.01,
            "Very high AR should cap at 95%, got {chance}"
        );
    }

    // ------------------------------------------------------------------ //
    // Damage roll tests
    // ------------------------------------------------------------------ //

    #[test]
    fn damage_roll_in_range() {
        let mut rng = seeded_rng(42);
        let attacker = basic_attacker();
        let defender = basic_defender();

        for _ in 0..100 {
            let roll = DamageCalculator::roll(&attacker, &defender, &mut rng);
            assert!(
                roll.base >= attacker.min_damage && roll.base <= attacker.max_damage,
                "Base damage {} out of range [{}, {}]",
                roll.base,
                attacker.min_damage,
                attacker.max_damage
            );
        }
    }

    #[test]
    fn damage_roll_crit_multiplied() {
        let mut rng = seeded_rng(42);
        let mut attacker = basic_attacker();
        attacker.crit_chance = 1.0; // guaranteed crit
        attacker.min_damage = 100;
        attacker.max_damage = 100; // fixed for determinism
        attacker.crit_multiplier = 2.0;

        let defender = basic_defender();
        let roll = DamageCalculator::roll(&attacker, &defender, &mut rng);

        assert!(roll.is_critical, "Expected a critical hit");
        assert_eq!(
            roll.final_amount, 200,
            "100 base * 2.0 crit = 200, got {}",
            roll.final_amount
        );
    }

    #[test]
    fn damage_roll_fire_res_applied() {
        let mut rng = seeded_rng(42);
        let mut attacker = basic_attacker();
        attacker.damage_type = DamageType::Fire;
        attacker.min_damage = 100;
        attacker.max_damage = 100;
        attacker.crit_chance = 0.0;

        let mut defender = basic_defender();
        defender.fire_res = 50;

        let roll = DamageCalculator::roll(&attacker, &defender, &mut rng);
        assert_eq!(
            roll.final_amount, 50,
            "100 base with 50% fire res should yield 50, got {}",
            roll.final_amount
        );
    }

    #[test]
    fn damage_roll_immune_returns_zero() {
        let mut rng = seeded_rng(42);
        let mut attacker = basic_attacker();
        attacker.damage_type = DamageType::Fire;
        attacker.min_damage = 100;
        attacker.max_damage = 100;

        let mut defender = basic_defender();
        defender.is_immune_to = vec![DamageType::Fire];

        let roll = DamageCalculator::roll(&attacker, &defender, &mut rng);
        assert_eq!(
            roll.final_amount, 0,
            "Immune target should take 0 damage, got {}",
            roll.final_amount
        );
    }

    #[test]
    fn damage_physical_res_applied() {
        let mut rng = seeded_rng(42);
        let mut attacker = basic_attacker();
        attacker.damage_type = DamageType::Physical;
        attacker.min_damage = 100;
        attacker.max_damage = 100;
        attacker.crit_chance = 0.0;

        let mut defender = basic_defender();
        defender.physical_res = 30;

        let roll = DamageCalculator::roll(&attacker, &defender, &mut rng);
        assert_eq!(
            roll.final_amount, 70,
            "100 base with 30% phys res should yield 70, got {}",
            roll.final_amount
        );
    }

    #[test]
    fn apply_resistance_capped_at_75() {
        let mut rng = seeded_rng(42);
        let mut attacker = basic_attacker();
        attacker.damage_type = DamageType::Fire;
        attacker.min_damage = 100;
        attacker.max_damage = 100;
        attacker.crit_chance = 0.0;

        let mut defender = basic_defender();
        defender.fire_res = 120; // Way above cap.

        let roll = DamageCalculator::roll(&attacker, &defender, &mut rng);
        assert_eq!(
            roll.final_amount, 25,
            "Resistance should cap at 75, so 100*(100-75)/100 = 25, got {}",
            roll.final_amount
        );
    }

    // ------------------------------------------------------------------ //
    // Combat processor tests
    // ------------------------------------------------------------------ //

    #[test]
    fn combat_miss_event_generated() {
        // Use attacker with AR=1, defender with DR=100000 to almost guarantee miss.
        // We also use a seeded RNG and loop until we get a miss.
        let mut attacker = basic_attacker();
        attacker.attack_rating = 1;

        let mut defender = basic_defender();
        defender.defense_rating = 100_000;

        // With 5% hit chance, most seeds will miss on first try.
        let mut rng = seeded_rng(0);
        let result = CombatProcessor::process_attack(1, 2, &attacker, &defender, 100, &mut rng);

        // With such extreme odds, we might still hit 5% of the time.
        // Run multiple times to guarantee at least one miss.
        let mut found_miss = !result.hit;
        if !found_miss {
            for seed in 1..100_u64 {
                let mut rng2 = seeded_rng(seed);
                let r = CombatProcessor::process_attack(1, 2, &attacker, &defender, 100, &mut rng2);
                if !r.hit {
                    found_miss = true;
                    // Verify the event is Miss.
                    assert!(
                        r.events.iter().any(|e| matches!(e, CombatEvent::Miss { .. })),
                        "Expected Miss event"
                    );
                    break;
                }
            }
        } else {
            assert!(
                result
                    .events
                    .iter()
                    .any(|e| matches!(e, CombatEvent::Miss { .. })),
                "Expected Miss event"
            );
        }
        assert!(found_miss, "Should have found at least one miss in 100 tries with 5% hit chance");
    }

    #[test]
    fn combat_hit_event_generated() {
        let mut attacker = basic_attacker();
        attacker.attack_rating = 100_000; // near-guaranteed hit
        attacker.min_damage = 10;
        attacker.max_damage = 10;

        let defender = basic_defender();
        let mut rng = seeded_rng(42);

        let result = CombatProcessor::process_attack(1, 2, &attacker, &defender, 100, &mut rng);
        assert!(result.hit, "Should have hit with massive AR");
        assert!(
            result
                .events
                .iter()
                .any(|e| matches!(e, CombatEvent::Hit { .. })),
            "Expected Hit event"
        );
    }

    #[test]
    fn combat_death_event_on_lethal_damage() {
        let mut attacker = basic_attacker();
        attacker.attack_rating = 100_000;
        attacker.min_damage = 100;
        attacker.max_damage = 100;
        attacker.crit_chance = 0.0;

        let defender = basic_defender(); // 0 res, not immune
        let defender_hp = 50; // will die from 100 damage
        let mut rng = seeded_rng(42);

        let result =
            CombatProcessor::process_attack(1, 2, &attacker, &defender, defender_hp, &mut rng);

        assert!(result.hit);
        assert!(
            result
                .events
                .iter()
                .any(|e| matches!(e, CombatEvent::Death { entity_id: 2, killer_id: 1 })),
            "Expected Death event for defender, got {:?}",
            result.events
        );
    }

    #[test]
    fn combat_immune_event_generated() {
        let mut attacker = basic_attacker();
        attacker.attack_rating = 100_000;
        attacker.damage_type = DamageType::Cold;

        let mut defender = basic_defender();
        defender.is_immune_to = vec![DamageType::Cold];

        let mut rng = seeded_rng(42);
        let result = CombatProcessor::process_attack(1, 2, &attacker, &defender, 100, &mut rng);

        assert!(
            result
                .events
                .iter()
                .any(|e| matches!(e, CombatEvent::Immune { .. })),
            "Expected Immune event, got {:?}",
            result.events
        );
        assert_eq!(result.total_damage, 0);
    }

    // ------------------------------------------------------------------ //
    // Status-effect tests (new StatusTracker API)
    // ------------------------------------------------------------------ //

    fn make_poison(remaining_ms: u32, dps: f32) -> StatusEffect {
        StatusEffect { kind: StatusType::Poison, remaining_ms, potency: dps }
    }

    fn make_cold(remaining_ms: u32, slow_pct: f32) -> StatusEffect {
        StatusEffect { kind: StatusType::Cold, remaining_ms, potency: slow_pct }
    }

    #[test]
    fn status_poison_tick_decrements() {
        let mut tracker = StatusTracker::new();
        tracker.apply(make_poison(3_000, 5.0));
        // Poison active before tick.
        assert!(tracker.is_affected(StatusType::Poison));
        tracker.tick(1_000);
        // Still active after 1 000 ms (2 000 ms remain).
        assert!(tracker.is_affected(StatusType::Poison), "Poison should still be active after 1 000 ms tick");
        // Advance past expiry to verify it eventually expires.
        tracker.tick(2_100);
        assert!(!tracker.is_affected(StatusType::Poison), "Poison should expire after full duration");
    }

    #[test]
    fn status_poison_expires_at_zero() {
        let mut tracker = StatusTracker::new();
        tracker.apply(make_poison(500, 5.0));
        tracker.tick(600); // tick past expiry
        assert!(
            !tracker.is_affected(StatusType::Poison),
            "Poison should have expired"
        );
    }

    #[test]
    fn status_cold_tick_decrements() {
        let mut tracker = StatusTracker::new();
        tracker.apply(make_cold(2_000, 30.0));
        tracker.tick(1_000);
        assert!(tracker.is_affected(StatusType::Cold), "Cold should still be active");
        tracker.tick(1_100); // total > 2 000 ms
        assert!(!tracker.is_affected(StatusType::Cold), "Cold should have expired");
    }

    #[test]
    fn status_chilled_slow_factor() {
        let mut tracker = StatusTracker::new();
        tracker.apply(make_cold(5_000, 50.0)); // 50 % slow
        assert!(tracker.is_affected(StatusType::Cold), "Cold should be active");
        let factor = tracker.get_slow_factor();
        assert!(
            (factor - 0.5_f32).abs() < f32::EPSILON,
            "50 % slow => factor 0.50, got {factor}"
        );
    }

    // ------------------------------------------------------------------ //
    // Physical damage pipeline tests (D2-style)
    // ------------------------------------------------------------------ //

    #[test]
    fn phys_damage_range() {
        // With a range (10-50), 0 bonuses, 0 defense, no crit,
        // results should stay within the base range.
        let mut rng = seeded_rng(42);
        let input = PhysicalDamageInput {
            base_min: 10,
            base_max: 50,
            str_bonus: 0.0,
            skill_bonus: 0.0,
            crit_chance: 0.0,
            crit_mult: 1.5,
            defense: 0,
            attacker_level: 10,
        };

        for _ in 0..200 {
            let result = physical_damage(&input, &mut rng);
            assert!(
                result.damage >= 10 && result.damage <= 50,
                "With 0 bonuses and 0 defense, damage {} should be in [10, 50]",
                result.damage
            );
            assert!(!result.crit, "crit_chance=0 should never crit");
        }
    }

    #[test]
    fn phys_damage_defense_reduces() {
        // Fixed base 100, no bonuses, no crit, compare 0 defense vs 200 defense.
        let mut rng_a = seeded_rng(99);
        let mut rng_b = seeded_rng(99); // same seed for identical base rolls

        let no_def_input = PhysicalDamageInput {
            base_min: 100,
            base_max: 100,
            str_bonus: 0.0,
            skill_bonus: 0.0,
            crit_chance: 0.0,
            crit_mult: 1.5,
            defense: 0,
            attacker_level: 10,
        };
        let with_def_input = PhysicalDamageInput {
            defense: 200,
            ..no_def_input.clone()
        };

        let no_def = physical_damage(&no_def_input, &mut rng_a);
        let with_def = physical_damage(&with_def_input, &mut rng_b);

        assert_eq!(no_def.damage, 100, "0 defense => full damage");
        assert!(
            with_def.damage < no_def.damage,
            "Defense should reduce damage: no_def={}, with_def={}",
            no_def.damage,
            with_def.damage
        );
        // With defense=200, alvl=10: denominator = 200 + 200 + 80 = 480.
        // Reduction factor = 1 - 200/480 = 280/480 = 0.5833...
        // Expected: floor(100 * 0.5833) = 58.
        assert_eq!(
            with_def.damage, 58,
            "100 * (1 - 200/480) = 58.33 => floor 58, got {}",
            with_def.damage
        );
    }

    #[test]
    fn phys_damage_crit_multiplies() {
        // 100% crit chance, crit_mult=2.0, base 100, no defense, no bonuses.
        let mut rng = seeded_rng(7);
        let input = PhysicalDamageInput {
            base_min: 100,
            base_max: 100,
            str_bonus: 0.0,
            skill_bonus: 0.0,
            crit_chance: 1.0,
            crit_mult: 2.0,
            defense: 0,
            attacker_level: 10,
        };
        let result = physical_damage(&input, &mut rng);

        assert!(result.crit, "crit_chance=1.0 should always crit");
        assert_eq!(
            result.damage, 200,
            "100 base * 2.0 crit = 200, got {}",
            result.damage
        );
    }

    #[test]
    fn phys_damage_str_and_skill_bonuses() {
        // base=100, str_bonus=50%, skill_bonus=100%, no crit, no defense.
        // total = 100 * 1.5 * 2.0 = 300.
        let mut rng = seeded_rng(1);
        let input = PhysicalDamageInput {
            base_min: 100,
            base_max: 100,
            str_bonus: 50.0,
            skill_bonus: 100.0,
            crit_chance: 0.0,
            crit_mult: 1.5,
            defense: 0,
            attacker_level: 10,
        };
        let result = physical_damage(&input, &mut rng);

        assert_eq!(
            result.damage, 300,
            "100 * 1.5 * 2.0 = 300, got {}",
            result.damage
        );
    }

    // ------------------------------------------------------------------ //
    // Elemental damage reduction tests (D2-style)
    // ------------------------------------------------------------------ //

    #[test]
    fn elem_fire_resist_50() {
        // 100 base, 50% fire resist, 0 absorb => 50 damage.
        let dmg = crate::damage::elemental_damage(100, 50, 0);
        assert_eq!(dmg, 50, "100 * (100-50)/100 = 50, got {dmg}");
    }

    #[test]
    fn elem_resist_cap_75() {
        // Resist of 120% should be clamped to 75%.
        // 100 * (100-75)/100 = 25.
        let dmg = crate::damage::elemental_damage(100, 120, 0);
        assert_eq!(dmg, 25, "Resist capped at 75: 100*(100-75)/100 = 25, got {dmg}");
    }

    #[test]
    fn elem_negative_resist() {
        // -50% resist amplifies damage: 100 * (100 - (-50)) / 100 = 100 * 150 / 100 = 150.
        let dmg = crate::damage::elemental_damage(100, -50, 0);
        assert_eq!(dmg, 150, "-50% resist => 150% damage: 100*150/100 = 150, got {dmg}");
    }

    #[test]
    fn elem_negative_resist_capped_at_minus_100() {
        // -200% resist should be clamped to -100%.
        // 100 * (100 - (-100)) / 100 = 100 * 200 / 100 = 200.
        let dmg = crate::damage::elemental_damage(100, -200, 0);
        assert_eq!(dmg, 200, "Negative resist capped at -100: 100*200/100 = 200, got {dmg}");
    }

    #[test]
    fn elem_absorb_reduces() {
        // 100 base, 0% resist, 30 absorb => 100 - 30 = 70.
        let dmg = crate::damage::elemental_damage(100, 0, 30);
        assert_eq!(dmg, 70, "100 base - 30 absorb = 70, got {dmg}");
    }

    #[test]
    fn elem_absorb_cannot_go_negative() {
        // 10 base, 0% resist, 50 absorb => max(10-50, 0) = 0.
        let dmg = crate::damage::elemental_damage(10, 0, 50);
        assert_eq!(dmg, 0, "Absorb exceeds damage: result clamped to 0, got {dmg}");
    }

    #[test]
    fn elem_resist_and_absorb_combined() {
        // 200 base, 50% resist, 20 absorb.
        // After resist: 200 * (100-50)/100 = 100.
        // After absorb: 100 - 20 = 80.
        let dmg = crate::damage::elemental_damage(200, 50, 20);
        assert_eq!(dmg, 80, "200 * 50% resist - 20 absorb = 80, got {dmg}");
    }

    // ------------------------------------------------------------------ //
    // Poison DoT tests
    // ------------------------------------------------------------------ //

    #[test]
    fn elem_poison_dot() {
        // 100 total damage over 4 ticks => 25 per tick.
        let per_tick = crate::damage::poison_damage_per_tick(100, 4);
        assert_eq!(per_tick, 25, "100 / 4 = 25, got {per_tick}");
    }

    #[test]
    fn poison_dot_remainder() {
        // 100 total over 3 ticks => 33 per tick (floored), remainder = 1.
        let per_tick = crate::damage::poison_damage_per_tick(100, 3);
        assert_eq!(per_tick, 33, "100 / 3 = 33 (floored), got {per_tick}");
    }

    #[test]
    fn poison_dot_zero_ticks() {
        // 0 ticks should return 0 (no division by zero).
        let per_tick = crate::damage::poison_damage_per_tick(100, 0);
        assert_eq!(per_tick, 0, "0 ticks should return 0, got {per_tick}");
    }

    #[test]
    fn poison_damage_struct_roundtrip() {
        let pd = crate::damage::PoisonDamage {
            total_damage: 120,
            duration_ticks: 6,
        };
        assert_eq!(pd.total_damage, 120);
        assert_eq!(pd.duration_ticks, 6);
        let per_tick = crate::damage::poison_damage_per_tick(pd.total_damage, pd.duration_ticks);
        assert_eq!(per_tick, 20, "120 / 6 = 20, got {per_tick}");
    }

    // ------------------------------------------------------------------ //
    // execute_attack (full D2-style pipeline) tests
    // ------------------------------------------------------------------ //

    #[test]
    fn processor_full_attack() {
        // High AR + high level advantage => near-guaranteed hit.
        // Fixed damage, no crit, 0 defense/resist => damage == base.
        let mut rng = seeded_rng(42);
        let attacker = AttackerStats {
            min_damage: 50,
            max_damage: 50,
            attack_rating: 500,
            crit_chance: 0.0,
            crit_multiplier: 1.5,
            damage_type: DamageType::Physical,
            level: 50,
            str_bonus: 0.0,
            skill_bonus: 0.0,
        };
        let mut defender = DefenderStats {
            defense_rating: 10,
            fire_res: 0,
            cold_res: 0,
            light_res: 0,
            poison_res: 0,
            physical_res: 0,
            is_immune_to: Vec::new(),
            level: 10,
            absorb: 0,
            current_hp: 200,
        };

        let result = CombatProcessor::execute_attack(&attacker, &mut defender, &mut rng);

        assert!(result.hit, "High AR + level advantage should hit");
        assert!(!result.crit, "crit_chance=0 should never crit");
        assert!(
            result.total_damage > 0,
            "Should deal positive damage, got {}",
            result.total_damage
        );
        assert_eq!(result.damages.len(), 1, "Single damage type expected");
        assert_eq!(
            result.damages[0].0,
            DamageType::Physical,
            "Damage type should be Physical"
        );
        assert_eq!(
            result.damages[0].1, result.total_damage,
            "Per-type damage should match total"
        );
        assert!(!result.killed, "200 HP defender should survive ~50 damage");
        assert!(
            defender.current_hp < 200,
            "Defender HP should be reduced, got {}",
            defender.current_hp
        );
    }

    #[test]
    fn processor_miss() {
        // AR=0 => hit_chance returns minimum 5%, and with low AR the
        // formula yields the floor. We force a miss by using AR=0 which
        // gives exactly 5% CTH. Run multiple seeds to guarantee a miss.
        let attacker = AttackerStats {
            min_damage: 100,
            max_damage: 100,
            attack_rating: 0,
            crit_chance: 0.0,
            crit_multiplier: 1.5,
            damage_type: DamageType::Physical,
            level: 1,
            str_bonus: 0.0,
            skill_bonus: 0.0,
        };
        let original_defender = DefenderStats {
            defense_rating: 10_000,
            fire_res: 0,
            cold_res: 0,
            light_res: 0,
            poison_res: 0,
            physical_res: 0,
            is_immune_to: Vec::new(),
            level: 99,
            absorb: 0,
            current_hp: 100,
        };

        let mut found_miss = false;
        for seed in 0..200_u64 {
            let mut rng = seeded_rng(seed);
            let mut defender = original_defender.clone();
            let result = CombatProcessor::execute_attack(&attacker, &mut defender, &mut rng);

            if !result.hit {
                found_miss = true;
                assert_eq!(result.total_damage, 0, "Miss should deal 0 damage");
                assert!(result.damages.is_empty(), "Miss should have no damage entries");
                assert!(!result.crit, "Miss cannot crit");
                assert!(!result.killed, "Miss cannot kill");
                assert_eq!(
                    defender.current_hp, 100,
                    "Defender HP should be unchanged on miss"
                );
                break;
            }
        }
        assert!(
            found_miss,
            "With AR=0 (5% CTH), should find at least one miss in 200 tries"
        );
    }

    #[test]
    fn processor_kill() {
        // Enough damage to kill: high AR, fixed 200 damage, defender has 50 HP.
        let mut rng = seeded_rng(42);
        let attacker = AttackerStats {
            min_damage: 200,
            max_damage: 200,
            attack_rating: 10_000,
            crit_chance: 0.0,
            crit_multiplier: 1.5,
            damage_type: DamageType::Physical,
            level: 50,
            str_bonus: 0.0,
            skill_bonus: 0.0,
        };
        let mut defender = DefenderStats {
            defense_rating: 1,
            fire_res: 0,
            cold_res: 0,
            light_res: 0,
            poison_res: 0,
            physical_res: 0,
            is_immune_to: Vec::new(),
            level: 1,
            absorb: 0,
            current_hp: 50,
        };

        let result = CombatProcessor::execute_attack(&attacker, &mut defender, &mut rng);

        assert!(result.hit, "10k AR vs 1 DR should hit");
        assert!(
            result.total_damage > 0,
            "Should deal positive damage, got {}",
            result.total_damage
        );
        assert!(result.killed, "200 base damage should kill 50 HP defender");
        assert!(
            defender.current_hp <= 0,
            "Defender HP should be <= 0, got {}",
            defender.current_hp
        );
    }
}
