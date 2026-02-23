//! @id mge.rpg.combat.v1.systems
//! @role system
//! @layer plugin
//! @domain rpg

use mge_core::{Context, PhaseId, World};
use mge_rpg_stats::{DeadTag, EntityDeathEvent, Health};

use crate::events::{AttackMissEvent, AttackRequestEvent, DamageEvent};
use crate::{Armor, CombatStats};

/// Phase recommandée pour le système de combat (plan MVP).
pub const COMBAT_PHASE: PhaseId = PhaseId(200);

/// Génère un d100 (1-100) avec le RNG du contexte.
#[inline]
fn roll_d100(ctx: &mut Context) -> f64 {
    (1 + (ctx.rng.u32() % 100)) as f64
}

/// Résout les attaques : Phase 1 (toucher atk vs esq), Phase 2 (parade atk vs par), dégâts.
/// Enregistré en Phase 200. Requiert : CombatStats (attaquant + cible), Armor (cible, optionnel),
/// Health (cible), DeadTag.
pub fn combat_resolution_system(world: &mut World, ctx: &mut Context) {
    let events: Vec<AttackRequestEvent> = ctx.events.iter::<AttackRequestEvent>().cloned().collect();

    for event in events {
        if !world.is_alive(event.attacker) || !world.is_alive(event.target) {
            continue;
        }

        let attacker_stats = match world.get::<CombatStats>(event.attacker) {
            Some(s) => s.clone(),
            None => continue,
        };

        let defender_stats = match world.get::<CombatStats>(event.target) {
            Some(s) => s.clone(),
            None => continue,
        };

        // Ignorer les morts
        if world.has_component::<DeadTag>(event.target) {
            continue;
        }

        // Phase 1 : toucher (atk vs esq) — 50% + 1% par point d'écart
        let hit_chance = (50.0 + (attacker_stats.atk - defender_stats.esq)).clamp(1.0, 99.0);
        let roll = roll_d100(ctx);
        if roll > hit_chance {
            ctx.emit(AttackMissEvent {
                attacker: event.attacker,
                target: event.target,
            });
            continue;
        }

        // Critique simplifié : 5% de base (roll <= 5)
        let is_crit = roll <= 5.0;

        // Phase 2 : parade (def par vs atk) — 50% + 1% par point d'écart
        let parry_chance = (50.0 + (defender_stats.par - attacker_stats.atk)).clamp(1.0, 99.0);
        let parry_roll = roll_d100(ctx);
        let parried = parry_roll <= parry_chance;

        // Dégâts
        let mut damage = attacker_stats.damage_base;
        if is_crit {
            damage *= 1.5;
        }

        let ar = world
            .get::<Armor>(event.target)
            .map(|a| a.ar_for_damage_type(attacker_stats.damage_type))
            .unwrap_or(0.0);
        damage *= (1.0 - (ar / 100.0)).max(0.0);

        if parried {
            if let Some(armor) = world.get_mut::<Armor>(event.target) {
                armor.resistance -= damage;
                if armor.resistance < 0.0 {
                    armor.resistance = 0.0;
                }
            }
        } else {
            if let Some(health) = world.get_mut::<Health>(event.target) {
                health.current -= damage;
                if health.current <= 0.0 {
                    health.current = 0.0;
                    world.insert(event.target, DeadTag);
                    ctx.emit(EntityDeathEvent {
                        entity: event.target,
                    });
                }
            }
        }

        ctx.emit(DamageEvent {
            target: event.target,
            amount: damage,
            crit: is_crit,
        });
    }
}
