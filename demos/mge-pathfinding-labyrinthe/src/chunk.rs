//! Système de chunks — MGE Démo Pathfinding
//! Gestion du monde par chunks 64x64 avec chargement/déchargement dynamique.

use crate::faction::AggroBehavior;
use crate::grid::{GridNode, Terrain, Vec2};
use std::collections::HashMap;

pub const CHUNK_SIZE: i32 = 64;
pub const TILE_SIZE: f32 = 18.0;
pub const CHUNK_PIXELS: f32 = CHUNK_SIZE as f32 * TILE_SIZE;

/// Coordonnées d'un chunk dans le monde (unité = chunk)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

impl ChunkCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Convertit une position monde en coordonnées de chunk
    pub fn from_world_pos(pos: Vec2) -> Self {
        Self {
            x: (pos.x / CHUNK_PIXELS).floor() as i32,
            y: (pos.y / CHUNK_PIXELS).floor() as i32,
        }
    }

    /// Position monde du coin supérieur gauche du chunk
    pub fn world_origin(&self) -> Vec2 {
        Vec2::new(self.x as f32 * CHUNK_PIXELS, self.y as f32 * CHUNK_PIXELS)
    }

    /// Distance de Chebyshev (max des distances sur chaque axe)
    pub fn chebyshev_distance(&self, other: &ChunkCoord) -> i32 {
        (self.x - other.x).abs().max((self.y - other.y).abs())
    }
}

/// Type de monstre pour le spawn
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterType {
    Skeleton,
    Zombie,
    Ghost,
}

/// Rectangle local au chunk (coordonnées en tiles)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

/// Zone de spawn de monstres dans un chunk — spawn aléatoire dans la zone jusqu'au max
#[derive(Debug, Clone)]
pub struct SpawnZone {
    pub rect: Rect,
    pub creature_type: MonsterType,
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
        creature_type: MonsterType,
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

    /// Position monde aléatoire dans la zone (centre de tile)
    pub fn random_world_pos(&self, chunk_coord: ChunkCoord) -> Vec2 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(self.rect.x1..=self.rect.x2);
        let y = rng.gen_range(self.rect.y1..=self.rect.y2);
        let origin = chunk_coord.world_origin();
        Vec2::new(
            origin.x + (x as f32 + 0.5) * TILE_SIZE,
            origin.y + (y as f32 + 0.5) * TILE_SIZE,
        )
    }

    /// Vérifie si une position monde (pixels) est dans la zone
    pub fn contains_world_pos(&self, chunk_coord: ChunkCoord, pos: Vec2) -> bool {
        let origin = chunk_coord.world_origin();
        let local_x = ((pos.x - origin.x) / TILE_SIZE).floor() as i32;
        let local_y = ((pos.y - origin.y) / TILE_SIZE).floor() as i32;
        local_x >= self.rect.x1
            && local_x <= self.rect.x2
            && local_y >= self.rect.y1
            && local_y <= self.rect.y2
    }

    /// Appelé quand une créature de cette zone meurt (décrémente le compteur)
    pub fn on_creature_died(&mut self) {
        if self.current_count > 0 {
            self.current_count -= 1;
        }
    }

    /// Tente un spawn si possible (current_count < max_count et spawn_timer <= 0).
    /// Retourne Some((pos, aggro)) si une position walkable est trouvée dans la zone.
    /// `is_walkable` prend (world_x, world_y) en coordonnées tuiles.
    pub fn try_spawn(
        &mut self,
        dt: f32,
        chunk_coord: ChunkCoord,
        is_walkable: impl Fn(i32, i32) -> bool,
    ) -> Option<(Vec2, AggroBehavior)> {
        if self.current_count >= self.max_count {
            return None;
        }
        self.spawn_timer -= dt;
        if self.spawn_timer > 0.0 {
            return None;
        }
        self.spawn_timer = self.spawn_interval;

        use rand::Rng;
        let mut rng = rand::thread_rng();
        const MAX_ATTEMPTS: usize = 20;
        for _ in 0..MAX_ATTEMPTS {
            let x = rng.gen_range(self.rect.x1..=self.rect.x2);
            let y = rng.gen_range(self.rect.y1..=self.rect.y2);
            let world_x = chunk_coord.x * CHUNK_SIZE + x;
            let world_y = chunk_coord.y * CHUNK_SIZE + y;
            if is_walkable(world_x, world_y) {
                self.current_count += 1;
                let origin = chunk_coord.world_origin();
                let pos = Vec2::new(
                    origin.x + (x as f32 + 0.5) * TILE_SIZE,
                    origin.y + (y as f32 + 0.5) * TILE_SIZE,
                );
                return Some((pos, self.aggro));
            }
        }
        None
    }
}

/// Chunk de terrain 64x64
#[derive(Debug, Clone)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub terrain: Vec<Terrain>,
    pub spawn_zones: Vec<SpawnZone>,
}

impl Chunk {
    pub fn new(coord: ChunkCoord) -> Self {
        Self {
            coord,
            terrain: vec![Terrain::Grass; (CHUNK_SIZE * CHUNK_SIZE) as usize],
            spawn_zones: Vec::new(),
        }
    }

    fn index(&self, local_x: i32, local_y: i32) -> Option<usize> {
        if local_x >= 0 && local_x < CHUNK_SIZE && local_y >= 0 && local_y < CHUNK_SIZE {
            Some((local_y * CHUNK_SIZE + local_x) as usize)
        } else {
            None
        }
    }

    pub fn terrain_at(&self, local_x: i32, local_y: i32) -> Terrain {
        self.index(local_x, local_y)
            .map(|i| self.terrain[i])
            .unwrap_or(Terrain::Obstacle)
    }

    pub fn set_terrain(&mut self, local_x: i32, local_y: i32, terrain: Terrain) {
        if let Some(i) = self.index(local_x, local_y) {
            self.terrain[i] = terrain;
        }
    }

    pub fn is_walkable(&self, local_x: i32, local_y: i32) -> bool {
        self.terrain_at(local_x, local_y).is_walkable()
    }
}

/// Gestionnaire de chunks avec chargement/déchargement dynamique
pub struct ChunkManager {
    pub active_chunks: HashMap<ChunkCoord, Chunk>,
    pub world_width: i32,
    pub world_height: i32,
}

impl ChunkManager {
    pub fn new(world_width: i32, world_height: i32) -> Self {
        Self {
            active_chunks: HashMap::new(),
            world_width,
            world_height,
        }
    }

    /// Convertit des coordonnées monde en coordonnées locales de chunk
    pub fn world_to_local(world_x: i32, world_y: i32) -> (ChunkCoord, i32, i32) {
        let chunk_x = world_x.div_euclid(CHUNK_SIZE);
        let chunk_y = world_y.div_euclid(CHUNK_SIZE);
        let local_x = world_x.rem_euclid(CHUNK_SIZE);
        let local_y = world_y.rem_euclid(CHUNK_SIZE);
        (ChunkCoord::new(chunk_x, chunk_y), local_x, local_y)
    }

    /// Terrain à une position monde (en tiles)
    pub fn terrain_at(&self, world_x: i32, world_y: i32) -> Terrain {
        let (coord, local_x, local_y) = Self::world_to_local(world_x, world_y);
        self.active_chunks
            .get(&coord)
            .map(|c| c.terrain_at(local_x, local_y))
            .unwrap_or(Terrain::Obstacle)
    }

    /// Vérifie si une position monde est traversable
    pub fn is_walkable(&self, world_x: i32, world_y: i32) -> bool {
        self.terrain_at(world_x, world_y).is_walkable()
    }

    /// Coût de déplacement à une position monde
    pub fn movement_cost_at(&self, world_x: i32, world_y: i32) -> f32 {
        self.terrain_at(world_x, world_y).movement_cost()
    }

    /// Convertit une position monde (pixels) en GridNode
    pub fn world_to_node(&self, pos: Vec2) -> GridNode {
        GridNode::new(
            (pos.x / TILE_SIZE).floor() as i32,
            (pos.y / TILE_SIZE).floor() as i32,
        )
    }

    /// Convertit un GridNode en position monde (centre de la tile)
    pub fn node_to_world(&self, node: GridNode) -> Vec2 {
        Vec2::new(
            (node.x as f32 + 0.5) * TILE_SIZE,
            (node.y as f32 + 0.5) * TILE_SIZE,
        )
    }

    /// Met à jour les chunks chargés selon la position du joueur
    /// Charge les 9 chunks autour du joueur, décharge ceux à distance > 1
    pub fn update_loaded_chunks<F>(&mut self, player_pos: Vec2, mut load_chunk: F)
    where
        F: FnMut(ChunkCoord) -> Option<Chunk>,
    {
        let player_chunk = ChunkCoord::from_world_pos(player_pos);

        // Chunks à charger (3x3 autour du joueur)
        let mut to_load = Vec::new();
        for dy in -1..=1 {
            for dx in -1..=1 {
                let coord = ChunkCoord::new(player_chunk.x + dx, player_chunk.y + dy);
                if coord.x >= 0
                    && coord.x < self.world_width
                    && coord.y >= 0
                    && coord.y < self.world_height
                    && !self.active_chunks.contains_key(&coord)
                {
                    to_load.push(coord);
                }
            }
        }

        // Charger les nouveaux chunks
        for coord in to_load {
            if let Some(chunk) = load_chunk(coord) {
                self.active_chunks.insert(coord, chunk);
            }
        }

        // Décharger les chunks trop loin (distance Chebyshev > 1)
        let to_unload: Vec<ChunkCoord> = self
            .active_chunks
            .keys()
            .filter(|c| c.chebyshev_distance(&player_chunk) > 1)
            .copied()
            .collect();

        for coord in to_unload {
            self.active_chunks.remove(&coord);
        }
    }

    /// Retourne tous les chunks actifs pour le rendu
    pub fn active_chunks_iter(&self) -> impl Iterator<Item = &Chunk> {
        self.active_chunks.values()
    }

    /// Retourne tous les chunks actifs de façon mutable (pour spawn, etc.)
    pub fn active_chunks_iter_mut(&mut self) -> impl Iterator<Item = &mut Chunk> {
        self.active_chunks.values_mut()
    }
}

impl crate::pathfinding::TerrainProvider for ChunkManager {
    fn is_walkable(&self, x: i32, y: i32) -> bool {
        self.is_walkable(x, y)
    }
    fn movement_cost_at(&self, x: i32, y: i32) -> f32 {
        self.movement_cost_at(x, y)
    }
}
