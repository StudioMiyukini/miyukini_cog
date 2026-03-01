// @id: MGE-ARPG-Combat-Processor @do: attack-pipeline @role: back-end @layer: 3 @human: miyuk
//! High-level combat processor that orchestrates hit-check, damage, and events.

use rand::Rng;

use crate::calculator::DamageCalculator;
use crate::damage::{AttackerStats, DefenderStats};
use crate::events::{CombatEvent, CombatResult};
use crate::hit::HitChance;

/// Stateless combat processor.
///
/// Resolves a single attack between an attacker and a defender, producing
/// a [`CombatResult`] that contains all generated events.
pub struct CombatProcessor;

impl CombatProcessor {
    /// Resolve one attack.
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
}
