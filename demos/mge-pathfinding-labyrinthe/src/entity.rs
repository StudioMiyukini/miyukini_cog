//! Entité et locomotion — MGE Démo Pathfinding
//! Suit les waypoints avec vitesse constante, combat, mort, respawn

use crate::combat::CombatStats;
use crate::faction::{AggroBehavior, Faction};
use crate::grid::Vec2;
use crate::pathfinding::PathResult;

const WAYPOINT_REACHED_DIST: f32 = 4.0;

#[derive(Debug, Clone)]
pub struct Entity {
    pub position: Vec2,
    pub spawn_pos: Vec2,
    pub waypoints: Vec<Vec2>,
    pub waypoint_index: usize,
    pub speed: f32,
    pub combat: CombatStats,
    /// Temps restant avant respawn (si mort)
    pub respawn_timer: f32,
    /// Faction de l'entité (Neutral, Empire, Alliance, Horde)
    pub faction: Faction,
    /// Comportement d'aggro : Aggressive, Passive, Neutral
    pub aggro_behavior: AggroBehavior,
    /// ID du groupe (pour formation de monstres)
    pub group_id: Option<u32>,
    /// Indice de l'entité qui m'a attaqué (pour riposte)
    pub aggressor: Option<usize>,
    /// Les monstres vivants bloquent le mouvement du joueur
    pub is_obstacle: bool,
    /// Multiplicateur de vitesse (1.0 = normal, 0.6 = errance)
    pub speed_multiplier: f32,
}

impl Entity {
    /// Crée une entité avec les valeurs par défaut pour monstres (Neutral, Aggressive, obstacle).
    pub fn new(position: Vec2, speed: f32) -> Self {
        Self {
            position,
            spawn_pos: position,
            waypoints: Vec::new(),
            waypoint_index: 0,
            speed,
            combat: CombatStats::new(),
            respawn_timer: 0.0,
            faction: Faction::Neutral,
            aggro_behavior: AggroBehavior::Aggressive,
            group_id: None,
            aggressor: None,
            is_obstacle: true,
            speed_multiplier: 1.0,
        }
    }

    /// Crée une entité de faction Empire (joueur, invocations) — non obstacle.
    pub fn new_ally(position: Vec2, speed: f32) -> Self {
        Self {
            position,
            spawn_pos: position,
            waypoints: Vec::new(),
            waypoint_index: 0,
            speed,
            combat: CombatStats::new(),
            respawn_timer: 0.0,
            faction: Faction::Empire,
            aggro_behavior: AggroBehavior::Passive,
            group_id: None,
            aggressor: None,
            is_obstacle: false,
            speed_multiplier: 1.0,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.combat.is_alive()
    }

    pub fn is_dead(&self) -> bool {
        !self.is_alive()
    }

    pub fn clear_path(&mut self) {
        self.waypoints.clear();
        self.waypoint_index = 0;
    }

    pub fn set_path(&mut self, result: PathResult) {
        if result.found {
            self.waypoints = result.waypoints.clone();
            self.waypoint_index = 0;
        }
    }

    pub fn current_waypoint(&self) -> Option<Vec2> {
        if self.waypoint_index < self.waypoints.len() {
            Some(self.waypoints[self.waypoint_index])
        } else {
            None
        }
    }

    /// Met à jour la position vers le prochain waypoint.
    /// Ne fait rien si mort. Retourne true si l'entité a bougé.
    pub fn update_movement(&mut self, dt: f32) -> bool {
        if self.is_dead() {
            return false;
        }
        let Some(target) = self.current_waypoint() else {
            return false;
        };

        let dx = target.x - self.position.x;
        let dy = target.y - self.position.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < WAYPOINT_REACHED_DIST {
            self.waypoint_index += 1;
            return self.update_movement(dt);
        }

        let move_dist = self.speed * self.speed_multiplier * dt;
        if move_dist >= dist {
            self.position = target;
            self.waypoint_index += 1;
            true
        } else {
            let t = move_dist / dist;
            self.position.x += dx * t;
            self.position.y += dy * t;
            true
        }
    }

    /// Met à jour le respawn si mort.
    pub fn update_respawn(&mut self, dt: f32) {
        if self.is_dead() && self.respawn_timer > 0.0 {
            self.respawn_timer -= dt;
            if self.respawn_timer <= 0.0 {
                self.position = self.spawn_pos;
                self.combat.reset_for_respawn();
                self.clear_path();
                self.aggressor = None;
                self.group_id = None;
                self.speed_multiplier = 1.0;
            }
        }
    }

    /// Effectue une attaque contre la cible.
    /// Retourne Dodge(pos) si esquive, Hit(pos, dmg) si dégâts appliqués, None sinon.
    pub fn try_attack(&mut self, target: &mut Entity, dt: f32) -> Option<AttackResult> {
        if self.is_dead() || target.is_dead() {
            return None;
        }
        self.combat.atk_cooldown -= dt;
        if self.combat.atk_cooldown <= 0.0 {
            self.combat.atk_cooldown = 1.0 / self.combat.atk_speed;
            if rand::random::<u8>() % 3 == 0 {
                return Some(AttackResult::Dodge(target.position));
            }
            let dmg = self.combat.deal_damage(&mut target.combat);
            if target.is_dead() {
                target.respawn_timer = 6.0;
            }
            return Some(AttackResult::Hit(target.position, dmg));
        }
        None
    }
}

#[derive(Debug, Clone)]
pub enum AttackResult {
    Dodge(Vec2),
    Hit(Vec2, i32),
}
