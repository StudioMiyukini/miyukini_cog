//! Grille 2D pour le labyrinthe — MGE Démo Pathfinding
//! Terrain avec coûts de déplacement : route, herbe, sable, forêt, obstacle.

/// Type de terrain avec coût de déplacement (× sur base 1.0)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Terrain {
    /// Obstacle infranchissable
    Obstacle,
    /// Route — coût 0.8
    Road,
    /// Herbe, sol — coût 1.0
    #[default]
    Grass,
    /// Sable, gravier — coût 1.5
    Sand,
    /// Forêt — coût 2.0
    Forest,
}

impl Terrain {
    /// Coût multiplicatif du terrain (1.0 = neutre). Obstacle = f32::INFINITY.
    pub fn movement_cost(&self) -> f32 {
        match self {
            Terrain::Obstacle => f32::INFINITY,
            Terrain::Road => 0.8,
            Terrain::Grass => 1.0,
            Terrain::Sand => 1.5,
            Terrain::Forest => 2.0,
        }
    }

    pub fn is_walkable(&self) -> bool {
        !self.movement_cost().is_infinite()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridNode {
    pub x: i32,
    pub y: i32,
}

impl GridNode {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_vec2(&self, tile_size: f32) -> Vec2 {
        Vec2 {
            x: (self.x as f32 + 0.5) * tile_size,
            y: (self.y as f32 + 0.5) * tile_size,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Vec2) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn sub(&self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }

    pub fn add(&self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }

    pub fn scale(&self, s: f32) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }

    /// Normalise le vecteur. Retourne (0,0) si longueur ≈ 0.
    pub fn normalize_or_zero(self) -> Vec2 {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        if len < 1e-6 {
            Vec2::default()
        } else {
            self.scale(1.0 / len)
        }
    }
}

/// Grille 2D avec terrain et coûts de déplacement
#[derive(Debug, Clone)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    cells: Vec<Terrain>,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            cells: vec![Terrain::Grass; size],
        }
    }

    fn index(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some((y * self.width + x) as usize)
        } else {
            None
        }
    }

    pub fn terrain_at(&self, x: i32, y: i32) -> Terrain {
        self.index(x, y)
            .map(|i| self.cells[i])
            .unwrap_or(Terrain::Obstacle)
    }

    pub fn set_terrain(&mut self, x: i32, y: i32, terrain: Terrain) {
        if let Some(i) = self.index(x, y) {
            self.cells[i] = terrain;
        }
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        self.terrain_at(x, y).is_walkable()
    }

    /// Coût de la cellule (pour le pathfinding). Obstacle = f32::INFINITY.
    pub fn movement_cost_at(&self, x: i32, y: i32) -> f32 {
        self.terrain_at(x, y).movement_cost()
    }

    pub fn set_obstacle(&mut self, x: i32, y: i32, obstacle: bool) {
        self.set_terrain(
            x,
            y,
            if obstacle { Terrain::Obstacle } else { Terrain::Grass },
        );
    }

    pub fn node_to_world(&self, node: GridNode, tile_size: f32) -> Vec2 {
        node.to_vec2(tile_size)
    }

    pub fn world_to_node(&self, pos: Vec2, tile_size: f32) -> GridNode {
        let x = (pos.x / tile_size).floor() as i32;
        let y = (pos.y / tile_size).floor() as i32;
        GridNode::new(
            x.clamp(0, self.width - 1),
            y.clamp(0, self.height - 1),
        )
    }

    /// Génère un labyrinthe de démo (murs, obstacles, zones : route, sable, forêt)
    pub fn demo_maze() -> Self {
        let mut grid = Self::new(32, 32);

        // Bords (obstacles)
        for x in 0..32 {
            grid.set_terrain(x, 0, Terrain::Obstacle);
            grid.set_terrain(x, 31, Terrain::Obstacle);
        }
        for y in 0..32 {
            grid.set_terrain(0, y, Terrain::Obstacle);
            grid.set_terrain(31, y, Terrain::Obstacle);
        }

        // Obstacles internes (couloirs)
        for x in 10..22 {
            grid.set_terrain(x, 10, Terrain::Obstacle);
            grid.set_terrain(x, 11, Terrain::Obstacle);
        }
        for y in 10..22 {
            grid.set_terrain(10, y, Terrain::Obstacle);
            grid.set_terrain(11, y, Terrain::Obstacle);
        }
        for x in 10..22 {
            grid.set_terrain(x, 21, Terrain::Obstacle);
            grid.set_terrain(x, 20, Terrain::Obstacle);
        }
        for y in 10..22 {
            grid.set_terrain(21, y, Terrain::Obstacle);
            grid.set_terrain(20, y, Terrain::Obstacle);
        }

        // Ouvertures
        grid.set_terrain(15, 10, Terrain::Grass);
        grid.set_terrain(16, 10, Terrain::Grass);
        grid.set_terrain(15, 21, Terrain::Grass);
        grid.set_terrain(16, 21, Terrain::Grass);
        grid.set_terrain(10, 15, Terrain::Grass);
        grid.set_terrain(10, 16, Terrain::Grass);
        grid.set_terrain(21, 15, Terrain::Grass);
        grid.set_terrain(21, 16, Terrain::Grass);

        // Zone sable (bas-gauche) — coût 1.5
        for y in 22..30 {
            for x in 2..10 {
                grid.set_terrain(x, y, Terrain::Sand);
            }
        }

        // Zone forêt (haut-droite) — coût 2.0
        for y in 2..10 {
            for x in 20..30 {
                grid.set_terrain(x, y, Terrain::Forest);
            }
        }

        // Route (ligne horizontale traversant le centre) — coût 0.8
        for x in 2..30 {
            grid.set_terrain(x, 16, Terrain::Road);
            grid.set_terrain(x, 15, Terrain::Road);
        }

        // Route verticale (rejoignant la zone sable)
        for y in 18..24 {
            grid.set_terrain(5, y, Terrain::Road);
            grid.set_terrain(6, y, Terrain::Road);
        }

        grid
    }
}
