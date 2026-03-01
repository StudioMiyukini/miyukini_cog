// @id: MGE-ARPG-Combat-Tests @do: unit-tests @role: back-end @layer: 3 @human: miyuk
//! Unit and integration tests for the combat crate.

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use crate::calculator::DamageCalculator;
    use crate::damage::{AttackerStats, DamageType, DefenderStats};
    use crate::events::CombatEvent;
    use crate::hit::HitChance;
    use crate::processor::CombatProcessor;
    use crate::status::StatusEffect;

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
    // Status-effect tests
    // ------------------------------------------------------------------ //

    #[test]
    fn status_poison_tick_decrements() {
        let mut effect = StatusEffect::Poisoned {
            damage_per_tick: 5,
            ticks_remaining: 3,
        };
        let expired = effect.tick();
        assert!(!expired, "Should not expire after first tick (2 remaining)");
        if let StatusEffect::Poisoned {
            ticks_remaining, ..
        } = &effect
        {
            assert_eq!(*ticks_remaining, 2);
        }
    }

    #[test]
    fn status_poison_expires_at_zero() {
        let mut effect = StatusEffect::Poisoned {
            damage_per_tick: 5,
            ticks_remaining: 1,
        };
        let expired = effect.tick();
        assert!(expired, "Should expire when ticks reach 0");
        assert!(effect.is_expired());
    }

    #[test]
    fn status_frozen_tick_decrements() {
        let mut effect = StatusEffect::Frozen {
            ticks_remaining: 2,
        };
        let expired = effect.tick();
        assert!(!expired);
        if let StatusEffect::Frozen { ticks_remaining } = &effect {
            assert_eq!(*ticks_remaining, 1);
        }

        let expired = effect.tick();
        assert!(expired);
        assert!(effect.is_expired());
    }

    #[test]
    fn status_chilled_slow_factor() {
        let effect = StatusEffect::Chilled { slow_factor: 0.5 };
        // Chilled does not expire from ticking.
        assert!(!effect.is_expired());
        if let StatusEffect::Chilled { slow_factor } = &effect {
            assert!((slow_factor - 0.5).abs() < f32::EPSILON);
        }
    }
}
