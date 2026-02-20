//! Données hardcodées des chunks — MGE Démo Pathfinding
//! Monde 4x4 chunks (256x256 tiles) avec terrains prédéfinis.

use crate::chunk::{Chunk, ChunkCoord, MonsterType, Rect, SpawnZone, CHUNK_SIZE};
use crate::faction::AggroBehavior;
use crate::grid::Terrain;

/// Registre des templates de chunks (monde non-procédural)
pub struct ChunkRegistry;

impl ChunkRegistry {
    /// Génère un chunk à partir des données hardcodées
    pub fn create_chunk(coord: ChunkCoord) -> Option<Chunk> {
        if coord.x < 0 || coord.x >= 4 || coord.y < 0 || coord.y >= 4 {
            return None;
        }

        let mut chunk = Chunk::new(coord);

        match (coord.x, coord.y) {
            (0, 0) => Self::fill_forest(&mut chunk),
            (1, 0) => Self::fill_plains(&mut chunk),
            (2, 0) => Self::fill_plains_road(&mut chunk),
            (3, 0) => Self::fill_mountain(&mut chunk),
            (0, 1) => Self::fill_plains(&mut chunk),
            (1, 1) => Self::fill_village(&mut chunk),
            (2, 1) => Self::fill_crossroad(&mut chunk),
            (3, 1) => Self::fill_desert(&mut chunk),
            (0, 2) => Self::fill_swamp(&mut chunk),
            (1, 2) => Self::fill_road_vertical(&mut chunk),
            (2, 2) => Self::fill_spawn_area(&mut chunk),
            (3, 2) => Self::fill_ruins(&mut chunk),
            (0, 3) => Self::fill_lake(&mut chunk),
            (1, 3) => Self::fill_plains(&mut chunk),
            (2, 3) => Self::fill_plains(&mut chunk),
            (3, 3) => Self::fill_dungeon_entrance(&mut chunk),
            _ => {}
        }

        // Bordures du monde (bords extérieurs)
        Self::add_world_borders(&mut chunk);

        Some(chunk)
    }

    fn add_world_borders(chunk: &mut Chunk) {
        let c = chunk.coord;
        // Bord gauche du monde
        if c.x == 0 {
            for y in 0..CHUNK_SIZE {
                chunk.set_terrain(0, y, Terrain::Obstacle);
            }
        }
        // Bord droit du monde
        if c.x == 3 {
            for y in 0..CHUNK_SIZE {
                chunk.set_terrain(CHUNK_SIZE - 1, y, Terrain::Obstacle);
            }
        }
        // Bord haut du monde
        if c.y == 0 {
            for x in 0..CHUNK_SIZE {
                chunk.set_terrain(x, 0, Terrain::Obstacle);
            }
        }
        // Bord bas du monde
        if c.y == 3 {
            for x in 0..CHUNK_SIZE {
                chunk.set_terrain(x, CHUNK_SIZE - 1, Terrain::Obstacle);
            }
        }
    }

    fn fill_forest(chunk: &mut Chunk) {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Forest);
            }
        }
        // Clairière au centre
        for y in 25..40 {
            for x in 25..40 {
                chunk.set_terrain(x, y, Terrain::Grass);
            }
        }
        // Chemin vers le sud
        for y in 40..CHUNK_SIZE {
            for x in 30..34 {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        // Chemin vers l'est
        for y in 30..34 {
            for x in 40..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        // Zones de spawn (clairière)
        chunk.spawn_zones.push(SpawnZone::new(
            Rect {
                x1: 25,
                y1: 25,
                x2: 39,
                y2: 39,
            },
            MonsterType::Skeleton,
            AggroBehavior::Aggressive,
            25,
            5.0,
        ));
    }

    fn fill_plains(chunk: &mut Chunk) {
        // Déjà Grass par défaut, ajouter quelques arbres dispersés
        for i in 0..15 {
            let x = (i * 7 + 3) % CHUNK_SIZE;
            let y = (i * 11 + 5) % CHUNK_SIZE;
            chunk.set_terrain(x, y, Terrain::Forest);
        }
        // Zones de spawn
        chunk.spawn_zones.push(SpawnZone::new(
            Rect {
                x1: 15,
                y1: 15,
                x2: 50,
                y2: 45,
            },
            MonsterType::Zombie,
            AggroBehavior::Passive,
            20,
            5.0,
        ));
    }

    fn fill_plains_road(chunk: &mut Chunk) {
        Self::fill_plains(chunk);
        // Route horizontale
        for y in 30..34 {
            for x in 0..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
    }

    fn fill_mountain(chunk: &mut Chunk) {
        // Base herbe avec beaucoup de rochers
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                if (x + y) % 5 == 0 || (x * y) % 7 == 0 {
                    chunk.set_terrain(x, y, Terrain::Obstacle);
                }
            }
        }
        // Passage au sud
        for y in CHUNK_SIZE - 10..CHUNK_SIZE {
            for x in 25..40 {
                chunk.set_terrain(x, y, Terrain::Grass);
            }
        }
        // Passage à l'ouest
        for y in 28..36 {
            for x in 0..20 {
                chunk.set_terrain(x, y, Terrain::Grass);
            }
        }
        chunk.spawn_zones.push(SpawnZone::new(
            Rect {
                x1: 25,
                y1: 45,
                x2: 40,
                y2: 60,
            },
            MonsterType::Ghost,
            AggroBehavior::Aggressive,
            20,
            5.0,
        ));
    }

    fn fill_village(chunk: &mut Chunk) {
        // Place centrale
        for y in 20..45 {
            for x in 20..45 {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        // Maisons (obstacles)
        for house_y in [22, 35] {
            for house_x in [22, 35] {
                for dy in 0..8 {
                    for dx in 0..8 {
                        chunk.set_terrain(house_x + dx, house_y + dy, Terrain::Obstacle);
                    }
                }
            }
        }
        // Routes vers les bords
        for y in 30..34 {
            for x in 0..20 {
                chunk.set_terrain(x, y, Terrain::Road);
            }
            for x in 45..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        for x in 30..34 {
            for y in 0..20 {
                chunk.set_terrain(x, y, Terrain::Road);
            }
            for y in 45..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
    }

    fn fill_crossroad(chunk: &mut Chunk) {
        // Route horizontale
        for y in 30..34 {
            for x in 0..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        // Route verticale
        for x in 30..34 {
            for y in 0..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        chunk.spawn_zones.push(SpawnZone::new(
            Rect {
                x1: 10,
                y1: 10,
                x2: 55,
                y2: 55,
            },
            MonsterType::Skeleton,
            AggroBehavior::Aggressive,
            30,
            5.0,
        ));
    }

    fn fill_desert(chunk: &mut Chunk) {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Sand);
            }
        }
        // Quelques rochers
        for i in 0..20 {
            let x = (i * 13 + 7) % CHUNK_SIZE;
            let y = (i * 17 + 11) % CHUNK_SIZE;
            chunk.set_terrain(x, y, Terrain::Obstacle);
        }
        // Passage ouest
        for y in 28..36 {
            for x in 0..15 {
                chunk.set_terrain(x, y, Terrain::Sand);
            }
        }
        chunk.spawn_zones.push(SpawnZone::new(
            Rect {
                x1: 15,
                y1: 40,
                x2: 55,
                y2: 60,
            },
            MonsterType::Zombie,
            AggroBehavior::Passive,
            25,
            5.0,
        ));
    }

    fn fill_swamp(chunk: &mut Chunk) {
        // Base marécageuse (sable = boue)
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                if (x + y) % 3 == 0 {
                    chunk.set_terrain(x, y, Terrain::Sand);
                } else {
                    chunk.set_terrain(x, y, Terrain::Forest);
                }
            }
        }
        // Chemin surélevé
        for y in 25..40 {
            for x in 25..40 {
                chunk.set_terrain(x, y, Terrain::Grass);
            }
        }
        // Passage nord
        for y in 0..25 {
            for x in 30..34 {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        // Passage est
        for y in 30..34 {
            for x in 40..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        chunk.spawn_zones.push(SpawnZone::new(
            Rect {
                x1: 25,
                y1: 25,
                x2: 40,
                y2: 40,
            },
            MonsterType::Ghost,
            AggroBehavior::Neutral,
            20,
            5.0,
        ));
    }

    fn fill_road_vertical(chunk: &mut Chunk) {
        // Route verticale principale
        for x in 30..34 {
            for y in 0..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        // Route horizontale
        for y in 30..34 {
            for x in 0..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        chunk.spawn_zones.push(SpawnZone::new(
            Rect {
                x1: 5,
                y1: 5,
                x2: 55,
                y2: 55,
            },
            MonsterType::Skeleton,
            AggroBehavior::Aggressive,
            30,
            5.0,
        ));
        chunk.spawn_zones.push(SpawnZone::new(
            Rect {
                x1: 45,
                y1: 45,
                x2: 60,
                y2: 60,
            },
            MonsterType::Zombie,
            AggroBehavior::Passive,
            15,
            5.0,
        ));
    }

    fn fill_spawn_area(chunk: &mut Chunk) {
        // Zone de spawn du joueur — terrain ouvert avec routes
        // Place centrale
        for y in 20..45 {
            for x in 20..45 {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        // Routes vers tous les bords
        for y in 30..34 {
            for x in 0..20 {
                chunk.set_terrain(x, y, Terrain::Road);
            }
            for x in 45..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        for x in 30..34 {
            for y in 0..20 {
                chunk.set_terrain(x, y, Terrain::Road);
            }
            for y in 45..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        // Pas de spawns dans la zone de départ
    }

    fn fill_ruins(chunk: &mut Chunk) {
        // Murs en ruine
        for wall_y in [10, 30, 50] {
            for x in 5..55 {
                if x % 4 != 0 {
                    chunk.set_terrain(x, wall_y, Terrain::Obstacle);
                }
            }
        }
        for wall_x in [10, 30, 50] {
            for y in 5..55 {
                if y % 4 != 0 {
                    chunk.set_terrain(wall_x, y, Terrain::Obstacle);
                }
            }
        }
        // Entrée ouest
        for y in 28..36 {
            for x in 0..10 {
                chunk.set_terrain(x, y, Terrain::Grass);
            }
        }
        chunk.spawn_zones.push(SpawnZone::new(
            Rect { x1: 15, y1: 15, x2: 30, y2: 30 },
            MonsterType::Ghost,
            AggroBehavior::Neutral,
            15,
            5.0,
        ));
        chunk.spawn_zones.push(SpawnZone::new(
            Rect { x1: 35, y1: 35, x2: 50, y2: 50 },
            MonsterType::Ghost,
            AggroBehavior::Neutral,
            15,
            5.0,
        ));
        chunk.spawn_zones.push(SpawnZone::new(
            Rect { x1: 15, y1: 35, x2: 30, y2: 50 },
            MonsterType::Skeleton,
            AggroBehavior::Aggressive,
            15,
            5.0,
        ));
    }

    fn fill_lake(chunk: &mut Chunk) {
        // Lac au centre (obstacle = eau profonde)
        for y in 10..55 {
            for x in 10..55 {
                let dx = x - 32;
                let dy = y - 32;
                if dx * dx + dy * dy < 400 {
                    chunk.set_terrain(x, y, Terrain::Obstacle);
                } else if dx * dx + dy * dy < 600 {
                    chunk.set_terrain(x, y, Terrain::Sand);
                }
            }
        }
        // Passage nord
        for y in 0..10 {
            for x in 28..36 {
                chunk.set_terrain(x, y, Terrain::Grass);
            }
        }
        // Passage est
        for y in 28..36 {
            for x in 55..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Grass);
            }
        }
    }

    fn fill_dungeon_entrance(chunk: &mut Chunk) {
        // Entrée de donjon — terrain rocheux avec couloir
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                chunk.set_terrain(x, y, Terrain::Obstacle);
            }
        }
        // Couloir d'entrée depuis l'ouest
        for y in 25..40 {
            for x in 0..40 {
                chunk.set_terrain(x, y, Terrain::Grass);
            }
        }
        // Salle du donjon
        for y in 15..50 {
            for x in 30..55 {
                chunk.set_terrain(x, y, Terrain::Road);
            }
        }
        chunk.spawn_zones.push(SpawnZone::new(
            Rect { x1: 42, y1: 28, x2: 50, y2: 38 },
            MonsterType::Ghost,
            AggroBehavior::Neutral,
            10,
            5.0,
        ));
        chunk.spawn_zones.push(SpawnZone::new(
            Rect { x1: 35, y1: 22, x2: 45, y2: 32 },
            MonsterType::Skeleton,
            AggroBehavior::Aggressive,
            12,
            5.0,
        ));
    }
}
