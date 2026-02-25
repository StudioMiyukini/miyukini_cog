//! @id allumina.prototype.combat
//! @role system
//! @layer application
//! @domain allumina
//! @do combat_resolution_allumina_spec
//!
//! Résolution de combat Allumina — conforme à la spec section 5-6 :
//!
//! Séquence d'attaque mêlée (section 6) :
//!   Phase 0 : Crit (Luk) — jet ≤ Luk → critik (150% dmg, ignore esq/par)
//!   Phase 1 : Toucher — atk (assaillant) vs esq (défenseur)
//!             50% + 1%/point d'écart ; si échec → manqué
//!   Phase 2 : Parade — atk (assaillant) vs par (défenseur)
//!             si défenseur réussit → bloqué (pas de PV perdus)
//!   Sinon   : Dégâts = For × 3 appliqués
//!
//! Phase 80 : appliquer PendingDamage
//! Phase 85 : attaque joueur (résolution complète 2 phases + crit)

use mge_plugin_spatial::Position2D;

use crate::ai::MonsterAI;
use crate::components::PlayerMarker;
use crate::stats::{CharacterStats, CombatStats, Dead, PendingDamage, PlayerAttack};

// ── RNG déterministe (LCG) ────────────────────────────────────────────────────

fn lcg_next(seed: &mut u64) -> u64 {
    *seed = seed
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *seed
}

/// Jet d100 (1-100)
fn roll_d100(seed: &mut u64) -> i32 {
    ((lcg_next(seed) >> 33) % 100 + 1) as i32
}

// ── Résolutions de jets (spec section 5.1) ───────────────────────────────────

/// Jet d'opposition Allumina : 50% + 1%/pt d'écart.
/// Retourne true si `attacker_apt` réussit.
fn roll_opposition(attacker_apt: i32, defender_apt: i32, rng: &mut u64) -> bool {
    let chance = (50 + (attacker_apt - defender_apt).clamp(-50, 50)) as i32;
    roll_d100(rng) <= chance
}

/// Vérification critique Luk : jet naturel ≤ Luk → critik.
fn roll_crit(luk: u8, rng: &mut u64) -> bool {
    if luk == 0 { return false; }
    roll_d100(rng) <= luk as i32
}

// ── Phase 80 : appliquer PendingDamage ───────────────────────────────────────

/// Phase 80 : applique tous les PendingDamage accumulés dans le tick.
pub fn combat_apply_system(world: &mut mge_ecs::World, _ctx: &mut mge_core::Context) {
    let damaged: Vec<(mge_ecs::EntityId, i32)> = world
        .iter1::<PendingDamage>()
        .filter(|(_, pd)| pd.amount > 0)
        .map(|(id, pd)| (id, pd.amount))
        .collect();

    for (id, raw_dmg) in damaged {
        if let Some(pd) = world.get_mut::<PendingDamage>(id) {
            pd.amount = 0;
        }

        let is_player = world.has_component::<PlayerMarker>(id);

        let mut should_die = false;
        if let Some(stats) = world.get_mut::<CombatStats>(id) {
            // Dégâts directs (l'opposition a déjà été résolue en amont)
            stats.hp = (stats.hp - raw_dmg).max(0);
            should_die = stats.hp <= 0;
        }

        if should_die && !is_player {
            world.insert(id, Dead);
        }
    }
}

// ── Phase 85 : attaque joueur (2 phases + crit Luk) ──────────────────────────

/// Phase 85 : attaque automatique du joueur vers le mob le plus proche.
/// Résolution conforme spec Allumina :
///   Phase 0 : Luk crit → critik (150% dmg)
///   Phase 1 : atk vs esq → manqué si raté
///   Phase 2 : atk vs par → bloqué si défenseur réussit
pub fn player_attack_system(world: &mut mge_ecs::World, ctx: &mut mge_core::Context) {
    let dt = ctx.delta_secs();

    let player_id = match world.iter1::<PlayerMarker>().next().map(|(id, _)| id) {
        Some(id) => id,
        None => return,
    };

    // Réduire le cooldown
    if let Some(atk) = world.get_mut::<PlayerAttack>(player_id) {
        atk.cooldown = (atk.cooldown - dt).max(0.0);
    }

    let can_attack = world.get::<PlayerAttack>(player_id)
        .map(|a| a.cooldown <= 0.0)
        .unwrap_or(false);
    if !can_attack { return; }

    // Copier données joueur (shared borrows → Copy)
    let px          = world.get::<Position2D>(player_id).map(|p| p.x).unwrap_or(0.0);
    let py          = world.get::<Position2D>(player_id).map(|p| p.y).unwrap_or(0.0);
    let atk_apt     = world.get::<CharacterStats>(player_id).map(|c| c.atk_apt()).unwrap_or(30);
    let base_dmg    = world.get::<CharacterStats>(player_id).map(|c| c.melee_damage()).unwrap_or(3);
    let luk         = world.get::<CharacterStats>(player_id).map(|c| c.luk).unwrap_or(0);
    let range       = world.get::<PlayerAttack>(player_id).map(|a| a.range).unwrap_or(1.5);
    let cooldown_max= world.get::<PlayerAttack>(player_id).map(|a| a.cooldown_max).unwrap_or(1.0);
    let mut rng     = world.get::<PlayerAttack>(player_id).map(|a| a.rng_seed).unwrap_or(1);

    // Collecter les ids de mobs (shared borrow libéré après collect)
    let mob_ids: Vec<mge_ecs::EntityId> = world
        .iter1::<MonsterAI>()
        .map(|(id, _)| id)
        .collect();

    // Mob non-mort le plus proche dans la portée
    let mut nearest_mob: Option<mge_ecs::EntityId> = None;
    let mut nearest_d2 = range * range;
    for mid in &mob_ids {
        if world.has_component::<Dead>(*mid) { continue; }
        let (mx, my) = match world.get::<Position2D>(*mid) {
            Some(pos) => (pos.x, pos.y),
            None => continue,
        };
        let dx = mx - px;
        let dy = my - py;
        let d2 = dx * dx + dy * dy;
        if d2 < nearest_d2 {
            nearest_d2 = d2;
            nearest_mob = Some(*mid);
        }
    }

    let Some(target_id) = nearest_mob else {
        // Pas de cible, sauvegarder le rng quand même
        if let Some(atk) = world.get_mut::<PlayerAttack>(player_id) { atk.rng_seed = rng; }
        return;
    };

    // Aptitudes défensives de la cible (shared borrows)
    let target_esq = world.get::<CharacterStats>(target_id).map(|c| c.esq_apt()).unwrap_or(30);
    let target_par = world.get::<CharacterStats>(target_id).map(|c| c.par_apt()).unwrap_or(30);

    // ── Résolution combat (spec sections 5.3 + 6) ────────────────────────────

    // Phase 0 : Crit (Luk) — jet ≤ Luk → critik, dégâts 150%, ignore esq/par
    let damage = if roll_crit(luk, &mut rng) {
        base_dmg * 3 / 2   // 150%
    } else {
        // Phase 1 : atk vs esq — 50% + 1%/point d'écart
        if !roll_opposition(atk_apt, target_esq, &mut rng) {
            // Manqué
            if let Some(atk) = world.get_mut::<PlayerAttack>(player_id) {
                atk.rng_seed = rng;
                atk.cooldown = cooldown_max;
            }
            return;
        }
        // Phase 2 : par vs atk — si défenseur réussit → bloqué
        if roll_opposition(target_par, atk_apt, &mut rng) {
            // Paré
            if let Some(atk) = world.get_mut::<PlayerAttack>(player_id) {
                atk.rng_seed = rng;
                atk.cooldown = cooldown_max;
            }
            return;
        }
        base_dmg.max(1)
    };

    // Sauvegarder le rng et déclencher le cooldown
    if let Some(atk) = world.get_mut::<PlayerAttack>(player_id) {
        atk.rng_seed = rng;
        atk.cooldown = cooldown_max;
    }

    // Appliquer PendingDamage sur la cible
    if let Some(pd) = world.get_mut::<PendingDamage>(target_id) {
        pd.amount += damage;
    } else {
        world.insert(target_id, PendingDamage { amount: damage });
    }
}
