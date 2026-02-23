//! @id allumina.prototype.ai
//! @role system
//! @layer application
//! @domain allumina
//! @do monster_ai_fsm
//!
//! IA des monstres — FSM table-driven inspirée D2 (OpenDiablo2 extraction).
//! États : Idle (errance) → Chase (poursuite) → Attack (frappe) → Dead.
//! Phase 40 — avant l'input joueur.

use mge_ecs::Component;
use mge_plugin_spatial::Position2D;

use crate::components::{MoveTarget, PlayerMarker};
use crate::stats::{CombatStats, PendingDamage};

/// Vitesse de déplacement des monstres (tiles/sec)
pub const MOB_SPEED: f32 = 2.8;

// ── FSM States ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiState {
    /// Errance autour du point de spawn
    Idle,
    /// Poursuite active du joueur
    Chase,
    /// À portée — frappe en cours
    Attack,
    /// Mort — en attente de despawn
    Dead,
}

// ── Composant ─────────────────────────────────────────────────────────────────

pub struct MonsterAI {
    pub state: AiState,
    /// Rayon de détection (aggro)
    pub aggro_range: f32,
    /// Rayon de laisse — le monstre retourne au patrol si dépassé
    pub leash_range: f32,
    /// Portée de frappe (melee)
    pub attack_range: f32,
    /// Cooldown entre deux frappes (secondes)
    pub attack_cooldown: f32,
    /// Temps restant avant la prochaine frappe
    pub attack_timer: f32,
    /// Point de spawn/patrol
    pub patrol_x: f32,
    pub patrol_y: f32,
    /// Cible d'errance courante
    pub wander_target_x: f32,
    pub wander_target_y: f32,
    /// Timer avant de choisir une nouvelle cible d'errance
    pub wander_timer: f32,
    /// Graine RNG déterministe propre à cette entité
    pub rng_seed: u64,
}

impl Component for MonsterAI {}

impl MonsterAI {
    pub fn new(spawn_x: f32, spawn_y: f32, rng_seed: u64) -> Self {
        Self {
            state: AiState::Idle,
            aggro_range: 6.0,
            leash_range: 14.0,
            attack_range: 1.4,
            attack_cooldown: 1.2,
            attack_timer: 0.0,
            patrol_x: spawn_x,
            patrol_y: spawn_y,
            wander_target_x: spawn_x,
            wander_target_y: spawn_y,
            wander_timer: 0.0,
            rng_seed,
        }
    }
}

// ── RNG déterministe (LCG) ────────────────────────────────────────────────────

fn rng_f32(seed: &mut u64) -> f32 {
    *seed = seed
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    (*seed >> 33) as f32 / u32::MAX as f32
}

// ── Snapshot pour éviter les conflits de borrow ───────────────────────────────

struct AiSnap {
    id: mge_ecs::EntityId,
    px: f32,
    py: f32,
    state: AiState,
    aggro_range: f32,
    leash_range: f32,
    attack_range: f32,
    attack_cooldown: f32,
    attack_timer: f32,
    patrol_x: f32,
    patrol_y: f32,
    wander_target_x: f32,
    wander_target_y: f32,
    wander_timer: f32,
    rng_seed: u64,
    hp: i32,
}

// ── Système ───────────────────────────────────────────────────────────────────

/// Phase 40 : tick IA de tous les monstres.
pub fn ai_system(world: &mut mge_ecs::World, ctx: &mut mge_core::Context) {
    let dt = ctx.delta_secs();

    // Position du joueur (shared borrow → collect)
    let player_info = world
        .iter2::<PlayerMarker, Position2D>()
        .next()
        .map(|(pid, _, pos)| (pid, pos.x, pos.y));

    let Some((player_id, ppx, ppy)) = player_info else { return };

    // Snapshot tous les monstres avec IA + Stats (shared borrow)
    let snaps: Vec<AiSnap> = world
        .iter3::<Position2D, MonsterAI, CombatStats>()
        .map(|(id, pos, ai, stats)| AiSnap {
            id,
            px: pos.x,
            py: pos.y,
            state: ai.state,
            aggro_range: ai.aggro_range,
            leash_range: ai.leash_range,
            attack_range: ai.attack_range,
            attack_cooldown: ai.attack_cooldown,
            attack_timer: ai.attack_timer,
            patrol_x: ai.patrol_x,
            patrol_y: ai.patrol_y,
            wander_target_x: ai.wander_target_x,
            wander_target_y: ai.wander_target_y,
            wander_timer: ai.wander_timer,
            rng_seed: ai.rng_seed,
            hp: stats.hp,
        })
        .collect();

    for mut snap in snaps {
        // Mort : ne pas traiter
        if snap.hp <= 0 || snap.state == AiState::Dead {
            continue;
        }

        let dx_pl = ppx - snap.px;
        let dy_pl = ppy - snap.py;
        let dist_pl = (dx_pl * dx_pl + dy_pl * dy_pl).sqrt();

        let dx_pt = snap.patrol_x - snap.px;
        let dy_pt = snap.patrol_y - snap.py;
        let dist_pt = (dx_pt * dx_pt + dy_pt * dy_pt).sqrt();

        // Mise à jour des timers
        let new_attack_timer = (snap.attack_timer - dt).max(0.0);
        let new_wander_timer = (snap.wander_timer - dt).max(0.0);

        let mut new_state = snap.state;
        let mut move_to: Option<(f32, f32)> = None;
        let mut deal_damage = false;
        let mut new_wander_target = (snap.wander_target_x, snap.wander_target_y);
        let mut reset_wander_timer: Option<f32> = None;

        match snap.state {
            AiState::Idle => {
                if dist_pl <= snap.aggro_range {
                    // Aggro : passer en Chase
                    new_state = AiState::Chase;
                    move_to = Some((ppx, ppy));
                } else if new_wander_timer <= 0.0 {
                    // Choisir une nouvelle cible d'errance
                    let angle = rng_f32(&mut snap.rng_seed) * std::f32::consts::TAU;
                    let radius = 1.0 + rng_f32(&mut snap.rng_seed) * 2.5;
                    let wx = (snap.patrol_x + angle.cos() * radius).clamp(1.0, 62.0);
                    let wy = (snap.patrol_y + angle.sin() * radius).clamp(1.0, 62.0);
                    new_wander_target = (wx, wy);
                    reset_wander_timer = Some(2.5 + rng_f32(&mut snap.rng_seed) * 2.0);
                    move_to = Some(new_wander_target);
                } else {
                    // Continuer vers la cible de wander
                    let wdx = snap.wander_target_x - snap.px;
                    let wdy = snap.wander_target_y - snap.py;
                    if wdx * wdx + wdy * wdy > 0.09 {
                        move_to = Some((snap.wander_target_x, snap.wander_target_y));
                    }
                }
            }

            AiState::Chase => {
                if dist_pl > snap.leash_range {
                    // Laisse dépassée — retour au patrol
                    new_state = AiState::Idle;
                    if dist_pt > 0.3 {
                        move_to = Some((snap.patrol_x, snap.patrol_y));
                    }
                } else if dist_pl <= snap.attack_range {
                    // À portée de frappe
                    new_state = AiState::Attack;
                    // Rester sur place pour attaquer
                } else {
                    // Continuer la poursuite
                    move_to = Some((ppx, ppy));
                }
            }

            AiState::Attack => {
                if dist_pl > snap.attack_range * 1.6 {
                    // Joueur s'est éloigné → repoursuite
                    new_state = AiState::Chase;
                    move_to = Some((ppx, ppy));
                } else if new_attack_timer <= 0.0 {
                    // Frappe !
                    deal_damage = true;
                }
            }

            AiState::Dead => {}
        }

        // ── Appliquer les changements (mutations séquentielles) ──────────────

        if let Some(ai) = world.get_mut::<MonsterAI>(snap.id) {
            ai.state = new_state;
            ai.attack_timer = if deal_damage { snap.attack_cooldown } else { new_attack_timer };
            ai.wander_timer = reset_wander_timer.unwrap_or(new_wander_timer);
            ai.wander_target_x = new_wander_target.0;
            ai.wander_target_y = new_wander_target.1;
            ai.rng_seed = snap.rng_seed;
        }

        if let Some((tx, ty)) = move_to {
            world.insert(snap.id, MoveTarget { x: tx, y: ty });
        }

        if deal_damage {
            let dmg = world
                .get::<CombatStats>(snap.id)
                .map(|s| s.attack)
                .unwrap_or(5);
            if let Some(pd) = world.get_mut::<PendingDamage>(player_id) {
                pd.amount += dmg;
            } else {
                world.insert(player_id, PendingDamage { amount: dmg });
            }
        }
    }
}
