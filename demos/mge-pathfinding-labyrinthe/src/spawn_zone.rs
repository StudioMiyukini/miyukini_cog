//! Zones de spawn — MGE Pathfinding
//! SpawnZone avec rectangles, max_count, spawn aléatoire dans la zone.

use crate::faction::AggroBehavior;
use crate::grid::Vec2;
use rand::Rng;

/// Rectangle en coordonnées locales du chunk (en tiles)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self {
            x1: x1.min(x2),
            y1: y1.min(y2),
            x2: x1.max(x2),
            y2: y1.max(y2),
        }
    }

    pub fn width(&self) -> i32 {
        (self.x2 - self.x1).max(1)
    }

    pub fn height(&self) -> i32 {
        (self.y2 - self.y1).max(1)
    }

    /// Retourne une position aléatoire dans le rectangle (centre de tile)
    pub fn random_local_pos(&self, rng: &mut impl Rng) -> (i32, i32) {
        let x = rng.gen_range(self.x1..self.x2);
        let y = rng.gen_range(self.y1..self.y2);
        (x, y)
    }
}

/// Type de créature (monstres, animaux)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreatureType {
    Wolf,
    Skeleton,
    Zombie,
    Ghost,
}

/// Zone de spawn dans un chunk — plusieurs zones par chunk, spawn aléatoire jusqu'au max
#[derive(Debug, Clone)]
pub struct SpawnZone {
    pub rect: Rect,
    pub creature_type: CreatureType,
    pub aggro: AggroBehavior,
    pub max_count: u32,
    pub current_count: u32,
    pub spawn_interval: f32,
    /// Temps restant avant le prochain spawn possible
    pub spawn_timer: f32,
}

impl SpawnZone {
    pub fn new(
        rect: Rect,
        creature_type: CreatureType,
        aggro: AggroBehavior,
        max_count: u32,
        spawn_interval: f32,
    ) -> Self {
        Self {
            rect,
            creature_type,
            aggro,
            max_count,
            current_count: 0,
            spawn_interval,
            spawn_timer: 0.0,
        }
    }

    /// Met à jour le timer et tente un spawn si possible.
    /// `world_origin` : position monde du coin supérieur gauche du chunk (pixels).
    /// `tile_size` : taille d'une tile en pixels.
    /// `world_tile_offset` : (chunk_x * chunk_size, chunk_y * chunk_size) pour les coordonnées monde.
    /// `is_walkable(world_x, world_y)` : coordonnées monde en tiles.
    /// Retourne `Some(pos_world)` si une créature a été spawnée.
    pub fn try_spawn<F>(
        &mut self,
        dt: f32,
        world_origin: Vec2,
        tile_size: f32,
        world_tile_offset: (i32, i32),
        is_walkable: F,
        rng: &mut impl Rng,
    ) -> Option<Vec2>
    where
        F: Fn(i32, i32) -> bool,
    {
        if self.current_count >= self.max_count {
            return None;
        }

        self.spawn_timer -= dt;
        if self.spawn_timer > 0.0 {
            return None;
        }

        // Réinitialiser le timer pour le prochain spawn
        self.spawn_timer = self.spawn_interval;

        const MAX_ATTEMPTS: usize = 20;
        let (ox, oy) = world_tile_offset;

        for _ in 0..MAX_ATTEMPTS {
            let (local_x, local_y) = self.rect.random_local_pos(rng);
            let world_x = ox + local_x;
            let world_y = oy + local_y;

            if is_walkable(world_x, world_y) {
                self.current_count += 1;
                let pos = Vec2::new(
                    world_origin.x + (local_x as f32 + 0.5) * tile_size,
                    world_origin.y + (local_y as f32 + 0.5) * tile_size,
                );
                return Some(pos);
            }
        }

        None
    }

    /// Appelé quand une créature de cette zone meurt (décrémente le compteur)
    pub fn on_creature_died(&mut self) {
        if self.current_count > 0 {
            self.current_count -= 1;
        }
    }

    /// Vérifie si la zone peut encore spawner
    pub fn can_spawn(&self) -> bool {
        self.current_count < self.max_count
    }
}
