//! Invocations type Necro D2 — formation idle, chase, anti-stack

use crate::chunk::ChunkManager;
use crate::entity::Entity;
use crate::grid::{GridNode, Vec2};
use crate::pathfinding::{find_path, PathfindingConfig, PathResult};
use std::f32::consts::PI;

pub const INVOCATION_COUNT: usize = 5;
pub const IDLE_RADIUS: f32 = 50.0;
pub const VISION_RANGE: f32 = 150.0;
pub const SEPARATION_DIST: f32 = 15.0;
pub const SEPARATION_STRENGTH: f32 = 1.5;
pub const MAX_SEPARATION_DISPLACEMENT: f32 = 4.0;
pub const FOLLOW_RECALC_INTERVAL: f32 = 0.4;
pub const IDLE_WANDER_RADIUS: f32 = 14.0;   // Rayon max de l'errance autour du slot
pub const IDLE_WANDER_STEP: f32 = 6.0;      // Pas de déplacement aléatoire
pub const IDLE_WANDER_INTERVAL: f32 = 0.35; // Intervalle entre nouveaux déplacements

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvocationState {
    Idle,
    Chase,
    Combat,
    Return,
}

#[derive(Debug)]
pub struct Invocation {
    pub entity: Entity,
    pub slot_index: usize,
    pub state: InvocationState,
    pub recalc_timer: f32,
    pub last_goal: Option<GridNode>,
    /// Offset aléatoire pour l'errance en idle (mouvement naturel)
    pub wander_offset: Vec2,
    pub wander_timer: f32,
}

impl Invocation {
    pub fn new(position: Vec2, slot_index: usize, speed: f32) -> Self {
        Self {
            entity: Entity::new(position, speed),
            slot_index,
            state: InvocationState::Idle,
            recalc_timer: 0.0,
            last_goal: None,
            wander_offset: Vec2::default(),
            wander_timer: 0.0,
        }
    }

    /// Crée une invocation de faction 1 (Empire)
    pub fn new_ally(position: Vec2, slot_index: usize, speed: f32) -> Self {
        Self {
            entity: Entity::new_ally(position, speed),
            slot_index,
            state: InvocationState::Idle,
            recalc_timer: 0.0,
            last_goal: None,
            wander_offset: Vec2::default(),
            wander_timer: (slot_index as f32 * 0.1) % IDLE_WANDER_INTERVAL, // Déphasage par slot
        }
    }

    /// Position cible sur le cercle de formation autour du maître
    pub fn idle_target_position(master_pos: Vec2, slot_index: usize, total_slots: usize) -> Vec2 {
        let angle = (2.0 * PI * slot_index as f32) / total_slots.max(1) as f32;
        let dx = IDLE_RADIUS * angle.cos();
        let dy = IDLE_RADIUS * angle.sin();
        Vec2::new(master_pos.x + dx, master_pos.y + dy)
    }

    /// Applique l'errance aléatoire autour de la position de base (mouvement naturel en idle)
    fn idle_target_with_wander(&mut self, base: Vec2, dt: f32) -> Vec2 {
        self.wander_timer += dt;
        if self.wander_timer >= IDLE_WANDER_INTERVAL {
            self.wander_timer = 0.0;
            let angle = rand::random::<f32>() * 2.0 * PI;
            let step = IDLE_WANDER_STEP * (0.5 + rand::random::<f32>() * 0.5);
            let dx = angle.cos() * step;
            let dy = angle.sin() * step;
            self.wander_offset = self.wander_offset.add(Vec2::new(dx, dy));

            // Limiter l'offset au rayon max (cercle)
            let len = (self.wander_offset.x * self.wander_offset.x
                + self.wander_offset.y * self.wander_offset.y)
                .sqrt();
            if len > IDLE_WANDER_RADIUS && len > 0.001 {
                self.wander_offset = self
                    .wander_offset
                    .scale(IDLE_WANDER_RADIUS / len);
            }
        }
        base.add(self.wander_offset)
    }

    /// Applique la force de séparation par rapport aux autres invocations
    pub fn compute_separation_offset(pos: Vec2, others: &[Vec2], dt: f32) -> Vec2 {
        let mut total = Vec2::default();
        for &other in others {
            let d = pos.distance_to(&other);
            if d < SEPARATION_DIST && d > 0.001 {
                let direction = pos.sub(other).normalize_or_zero();
                let strength = SEPARATION_STRENGTH * (1.0 - d / SEPARATION_DIST);
                total = total.add(direction.scale(strength * dt));
            }
        }
        let len = (total.x * total.x + total.y * total.y).sqrt();
        if len > MAX_SEPARATION_DISPLACEMENT {
            total.scale(MAX_SEPARATION_DISPLACEMENT / len)
        } else {
            total
        }
    }

    /// Met à jour le pathfinding selon l'état (utilise ChunkManager)
    /// `rally_target` : si Some, les followers en Idle/Return se dirigent vers la balise
    pub fn update_path(
        &mut self,
        chunk_mgr: &ChunkManager,
        tile_size: f32,
        path_config: &PathfindingConfig,
        master_pos: Vec2,
        enemy_pos: Vec2,
        enemy_alive: bool,
        rally_target: Option<Vec2>,
        dt: f32,
    ) -> bool {
        if self.entity.is_dead() {
            return false;
        }
        let inv_node = chunk_mgr.world_to_node(self.entity.position);

        let enemy_in_vision =
            enemy_alive && self.entity.position.distance_to(&enemy_pos) <= VISION_RANGE;

        match self.state {
            InvocationState::Combat => {
                if !enemy_alive {
                    self.state = InvocationState::Return;
                }
            }
            InvocationState::Chase => {
                if !enemy_in_vision {
                    self.state = InvocationState::Return;
                }
            }
            InvocationState::Idle | InvocationState::Return => {
                if enemy_in_vision {
                    self.state = InvocationState::Chase;
                }
            }
        }

        self.recalc_timer += dt;
        let goal = match self.state {
            InvocationState::Idle | InvocationState::Return => {
                if let Some(rally_pos) = rally_target {
                    chunk_mgr.world_to_node(rally_pos)
                } else {
                    let base =
                        Self::idle_target_position(master_pos, self.slot_index, INVOCATION_COUNT);
                    let target = self.idle_target_with_wander(base, dt);
                    chunk_mgr.world_to_node(target)
                }
            }
            InvocationState::Chase => chunk_mgr.world_to_node(enemy_pos),
            InvocationState::Combat => return false,
        };

        let goal_changed = self.last_goal != Some(goal);
        let needs_path = self.entity.waypoints.is_empty()
            || (self.recalc_timer >= FOLLOW_RECALC_INTERVAL && goal_changed);

        if needs_path && chunk_mgr.is_walkable(goal.x, goal.y) {
            if goal_changed {
                self.recalc_timer = 0.0;
            }
            self.last_goal = Some(goal);
            if inv_node.x == goal.x && inv_node.y == goal.y {
                self.entity.set_path(PathResult {
                    waypoints: vec![goal.to_vec2(tile_size)],
                    total_cost: 0.0,
                    found: true,
                });
            } else {
                let path = find_path(chunk_mgr, inv_node, goal, tile_size, path_config);
                if path.found {
                    self.entity.set_path(path);
                }
            }
        }
        true
    }
}
