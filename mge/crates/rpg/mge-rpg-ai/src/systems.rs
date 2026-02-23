//! @id mge.rpg.ai.v1.systems
//! @role system
//! @layer plugin
//! @domain rpg

use mge_core::Context;
use mge_ecs::{EntityId, World};
use mge_plugin_spatial::{Position2D, Velocity2D};
use mge_rpg_stats::Health;

use crate::components::{AiTargetable, AIState, CreatureState};
use crate::events::AiAttackRequestEvent;

/// @id mge.rpg.ai.v1.fn.ai_tick_system
/// @role system
/// @layer plugin
/// @do FSM 5 etats (Idle, Chase, Attack, Return)
/// @requires AIState,Position2D
/// @writes AIState,Velocity2D
/// @emits AiAttackRequestEvent
/// @phase 500
/// @complexity O(n*m)
pub fn ai_tick_system(world: &mut World, ctx: &mut Context) {
    let ids: Vec<EntityId> = world
        .iter2::<AIState, Position2D>()
        .map(|(id, _, _)| id)
        .collect();
    for entity_id in ids {
        if !world.is_alive(entity_id) {
            continue;
        }
        let vel = run_fsm_read_phase(world, entity_id);
        let (new_state, new_target, should_attack) = vel.decision;
        {
            let ai = match world.get_mut::<AIState>(entity_id) {
                Some(a) => a,
                None => continue,
            };
            ai.state = new_state;
            ai.target = new_target;
        }
        if should_attack {
            if let Some(tid) = new_target {
                ctx.emit(AiAttackRequestEvent {
                    attacker: entity_id,
                    target: tid,
                });
            }
        }
        set_velocity(world, entity_id, vel.vx, vel.vy);
    }
}

struct FsmOutput {
    decision: (CreatureState, Option<EntityId>, bool),
    vx: f32,
    vy: f32,
}

fn run_fsm_read_phase(world: &World, entity_id: EntityId) -> FsmOutput {
    let pos = match world.get::<Position2D>(entity_id) {
        Some(p) => (p.x, p.y),
        None => return FsmOutput { decision: (CreatureState::Idle, None, false), vx: 0.0, vy: 0.0 },
    };
    let ai = match world.get::<AIState>(entity_id) {
        Some(a) => a,
        None => return FsmOutput { decision: (CreatureState::Idle, None, false), vx: 0.0, vy: 0.0 },
    };
    let spawn = ai.spawn_point;
    let dist_spawn = dist(pos.0, pos.1, spawn.0, spawn.1);

    let (vx_ret, vy_ret) = move_toward_vel(pos, spawn);
    let (state, target, attack, vx, vy) = if dist_spawn > ai.leash_radius && ai.state != CreatureState::Return {
        (CreatureState::Return, None, false, vx_ret, vy_ret)
    } else {
        match ai.state {
            CreatureState::Idle => {
                if let Some(tid) = find_target(world, pos, ai.aggro_radius) {
                    let (vx, vy) = move_toward_vel(pos, target_pos(world, tid).unwrap_or(spawn));
                    (CreatureState::Chase, Some(tid), false, vx, vy)
                } else {
                    (CreatureState::Idle, None, false, 0.0, 0.0)
                }
            }
            CreatureState::Chase => chase_decide(world, ai, pos),
            CreatureState::Attack => attack_decide(world, ai, pos),
            CreatureState::Return => {
                if dist_spawn < 0.5 {
                    (CreatureState::Idle, None, false, 0.0, 0.0)
                } else {
                    let (vx, vy) = move_toward_vel(pos, spawn);
                    (CreatureState::Return, None, false, vx, vy)
                }
            }
        }
    };

    FsmOutput {
        decision: (state, target, attack),
        vx,
        vy,
    }
}

fn chase_decide(
    world: &World,
    ai: &AIState,
    pos: (f32, f32),
) -> (CreatureState, Option<EntityId>, bool, f32, f32) {
    let tid = match ai.target {
        Some(t) => t,
        None => return (CreatureState::Idle, None, false, 0.0, 0.0),
    };
    if !world.is_alive(tid) || is_dead(world, tid) {
        return (CreatureState::Idle, None, false, 0.0, 0.0);
    }
    let tpos = match target_pos(world, tid) {
        Some(p) => p,
        None => return (CreatureState::Idle, None, false, 0.0, 0.0),
    };
    let dist_target = dist(pos.0, pos.1, tpos.0, tpos.1);
    if dist_target <= ai.attack_range {
        (CreatureState::Attack, Some(tid), true, 0.0, 0.0)
    } else {
        let (vx, vy) = move_toward_vel(pos, tpos);
        (CreatureState::Chase, Some(tid), false, vx, vy)
    }
}

fn attack_decide(
    world: &World,
    ai: &AIState,
    pos: (f32, f32),
) -> (CreatureState, Option<EntityId>, bool, f32, f32) {
    let tid = match ai.target {
        Some(t) => t,
        None => return (CreatureState::Idle, None, false, 0.0, 0.0),
    };
    if !world.is_alive(tid) || is_dead(world, tid) {
        return (CreatureState::Idle, None, false, 0.0, 0.0);
    }
    let tpos = match target_pos(world, tid) {
        Some(p) => p,
        None => return (CreatureState::Idle, None, false, 0.0, 0.0),
    };
    let dist_target = dist(pos.0, pos.1, tpos.0, tpos.1);
    if dist_target > ai.attack_range {
        let (vx, vy) = move_toward_vel(pos, tpos);
        (CreatureState::Chase, Some(tid), false, vx, vy)
    } else {
        (CreatureState::Attack, Some(tid), true, 0.0, 0.0)
    }
}

fn dist(ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
    let dx = bx - ax;
    let dy = by - ay;
    (dx * dx + dy * dy).sqrt()
}

fn is_dead(world: &World, id: EntityId) -> bool {
    world
        .get::<Health>(id)
        .map(|h| h.current <= 0.0)
        .unwrap_or(false)
}

fn target_pos(world: &World, id: EntityId) -> Option<(f32, f32)> {
    world.get::<Position2D>(id).map(|p| (p.x, p.y))
}

fn find_target(world: &World, pos: (f32, f32), radius: f32) -> Option<EntityId> {
    let mut best: Option<(EntityId, f32)> = None;
    for (tid, _pos, _) in world.iter2::<Position2D, AiTargetable>() {
        if is_dead(world, tid) {
            continue;
        }
        if let Some(p) = world.get::<Position2D>(tid) {
            let d = dist(pos.0, pos.1, p.x, p.y);
            if d <= radius && best.as_ref().map(|(_, bd)| d < *bd).unwrap_or(true) {
                best = Some((tid, d));
            }
        }
    }
    best.map(|(id, _)| id)
}

fn move_toward_vel(from: (f32, f32), to: (f32, f32)) -> (f32, f32) {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let len = (dx * dx + dy * dy).sqrt();
    let speed = 50.0;
    if len > 0.001 {
        (dx / len * speed, dy / len * speed)
    } else {
        (0.0, 0.0)
    }
}

fn set_velocity(world: &mut World, entity_id: EntityId, vx: f32, vy: f32) {
    if world.has_component::<Velocity2D>(entity_id) {
        if let Some(v) = world.get_mut::<Velocity2D>(entity_id) {
            v.x = vx;
            v.y = vy;
        }
    } else {
        world.insert(entity_id, Velocity2D { x: vx, y: vy });
    }
}
