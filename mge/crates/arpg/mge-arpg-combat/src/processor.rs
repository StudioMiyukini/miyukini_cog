// @id: MGE-ARPG-Combat-Processor @do: attack-pipeline @role: back-end @layer: 3 @human: miyuk
//! High-level combat processor that orchestrates hit-check, damage, and events.

use rand::Rng;

use crate::calculator::DamageCalculator;
use crate::damage::{
    elemental_damage, physical_damage, AttackerStats, DamageType, DefenderStats,
    PhysicalDamageInput,
};
use crate::events::{CombatEvent, CombatResult};
use crate::hit::{hit_chance, roll_hit, HitChance};

// ------------------------------------------------------------------ //
// AttackResult — returned by the full D2-style execute_attack pipeline
// ------------------------------------------------------------------ //

/// Result of a full D2-style attack resolution.
///
/// Unlike [`CombatResult`] (event-based), this struct provides a flat,
/// query-friendly view of the outcome: whether the attack hit, per-type
/// damage breakdown, crit status, and kill status.
#[derive(Debug, Clone)]
pub struct AttackResult {
    /// Whether the attack connected.
    pub hit: bool,
    /// Per-type damage breakdown (after defence and resistances).
    pub damages: Vec<(DamageType, i32)>,
    /// Sum of all damage dealt.
    pub total_damage: i32,
    /// Whether the hit was a critical strike.
    pub crit: bool,
    /// Whether the defender was killed by this attack.
    pub killed: bool,
}

/// Stateless combat processor.
///
/// Resolves a single attack between an attacker and a defender, producing
/// a [`CombatResult`] that contains all generated events.
pub struct CombatProcessor;

impl CombatProcessor {
    /// Resolve one attack (simplified — no level scaling).
    ///
    /// # Flow
    /// 1. Roll hit chance.
    /// 2. If miss, emit `Miss` event and return.
    /// 3. Roll damage (includes crit and resistance).
    /// 4. If immune (final == 0 and immune list matches), emit `Immune`.
    /// 5. Emit `Hit`.
    /// 6. If lethal (`defender_current_hp - final <= 0`), emit `Death`.
    pub fn process_attack(
        attacker_id: u32,
        defender_id: u32,
        attacker: &AttackerStats,
        defender: &DefenderStats,
        defender_current_hp: i32,
        rng: &mut impl Rng,
    ) -> CombatResult {
        let mut events = Vec::new();

        // 1. Hit check.
        let hit = HitChance::roll(attacker.attack_rating, defender.defense_rating, rng);

        if !hit {
            events.push(CombatEvent::Miss {
                attacker_id,
                defender_id,
            });
            return CombatResult {
                events,
                total_damage: 0,
                hit: false,
            };
        }

        // 2. Damage roll.
        let roll = DamageCalculator::roll(attacker, defender, rng);

        // 3. Immunity check (damage calculator already zeroed out the amount).
        if defender.is_immune_to.contains(&attacker.damage_type) {
            events.push(CombatEvent::Immune {
                defender_id,
                dtype: attacker.damage_type,
            });
            return CombatResult {
                events,
                total_damage: 0,
                hit: true,
            };
        }

        let dmg = roll.final_amount;

        events.push(CombatEvent::Hit {
            attacker_id,
            defender_id,
            damage: roll,
        });

        // 4. Death check.
        if defender_current_hp - dmg <= 0 {
            events.push(CombatEvent::Death {
                entity_id: defender_id,
                killer_id: attacker_id,
            });
        }

        CombatResult {
            events,
            total_damage: dmg,
            hit: true,
        }
    }

    // ------------------------------------------------------------------ //
    // Full D2-style attack pipeline (with levels)
    // ------------------------------------------------------------------ //

    /// Execute a full Diablo 2-style attack pipeline.
    ///
    /// Uses the level-aware [`hit_chance`] free function and the D2-style
    /// [`physical_damage`] / [`elemental_damage`] free functions for a
    /// complete attack resolution.
    ///
    /// # Pipeline
    /// 1. Compute CTH via D2 formula with attacker/defender levels, then
    ///    roll hit.
    /// 2. If miss, return early with `hit = false`.
    /// 3. Compute damage per type:
    ///    - **Physical**: uses [`physical_damage`] (str/skill bonuses,
    ///      defence reduction, crit).
    ///    - **Elemental / other**: uses [`elemental_damage`] (resist +
    ///      absorb).
    /// 4. Deduct HP from defender.
    /// 5. Return [`AttackResult`] with per-type breakdown, crit, and kill.
    pub fn execute_attack(
        attacker: &AttackerStats,
        defender: &mut DefenderStats,
        rng: &mut impl Rng,
    ) -> AttackResult {
        // 1. Hit check — D2 formula with levels.
        let cth = hit_chance(
            attacker.attack_rating,
            defender.defense_rating,
            attacker.level,
            defender.level,
        );
        if !roll_hit(rng, cth) {
            return AttackResult {
                hit: false,
                damages: Vec::new(),
                total_damage: 0,
                crit: false,
                killed: false,
            };
        }

        // 2. Immunity check.
        if defender.is_immune_to.contains(&attacker.damage_type) {
            return AttackResult {
                hit: true,
                damages: vec![(attacker.damage_type, 0)],
                total_damage: 0,
                crit: false,
                killed: false,
            };
        }

        // 3. Compute damage based on type.
        let dtype = attacker.damage_type;
        let (final_dmg, crit) = if dtype == DamageType::Physical {
            let input = PhysicalDamageInput {
                base_min: attacker.min_damage,
                base_max: attacker.max_damage,
                str_bonus: attacker.str_bonus,
                skill_bonus: attacker.skill_bonus,
                crit_chance: attacker.crit_chance,
                crit_mult: attacker.crit_multiplier,
                defense: defender.defense_rating,
                attacker_level: attacker.level,
            };
            let result = physical_damage(&input, rng);
            (result.damage, result.crit)
        } else {
            // Elemental / magic / poison — roll base, apply crit, then
            // resistance + absorb via elemental_damage().
            let base = if attacker.min_damage >= attacker.max_damage {
                attacker.min_damage
            } else {
                rng.gen_range(attacker.min_damage..=attacker.max_damage)
            };

            // Crit check.
            let crit_roll: f32 = rng.gen();
            let is_crit = crit_roll < attacker.crit_chance;

            #[allow(clippy::cast_possible_truncation)]
            let after_crit = if is_crit {
                #[allow(clippy::cast_precision_loss)]
                let multiplied = (base as f32 * attacker.crit_multiplier).round() as i32;
                multiplied
            } else {
                base
            };

            let resist = defender.resistance_for(dtype);
            let dmg = elemental_damage(after_crit, resist, defender.absorb);
            (dmg, is_crit)
        };

        let damages = vec![(dtype, final_dmg)];

        // 4. Deduct HP.
        defender.current_hp -= final_dmg;
        let killed = defender.current_hp <= 0;

        // 5. Return result.
        AttackResult {
            hit: true,
            damages,
            total_damage: final_dmg,
            crit,
            killed,
        }
    }
}
