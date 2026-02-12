//! # Terrain
//!
//! Gestion des tuiles et de la grille de terrain.
//!
//! @id: lifegame.world.terrain
//! @role: simulation
//! @layer: domain
//! @do: manage_terrain_tiles_and_grid
//! @human: Grille de terrain avec types de tuiles et propriétés

use serde::{Deserialize, Serialize};
use crate::ResourceDeposit;

/// Type de terrain pour une tuile
///
/// @id: lifegame.world.terrain_type
/// @do: enumerate_terrain_types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TerrainType {
    /// Eau profonde — navigation bateau uniquement
    DeepWater,
    /// Eau peu profonde — peut patauger
    ShallowWater,
    /// Plage — constructible limité
    Beach,
    /// Plaines — terrain idéal
    #[default]
    Grass,
    /// Forêt — doit défricher avant construction
    Forest,
    /// Colline — construction lente
    Hill,
    /// Montagne — non traversable
    Mountain,
    /// Neige — très faible fertilité
    Snow,
    /// Désert — faible fertilité
    Desert,
    /// Marais — malsain, ralentit
    Swamp,
    /// Lave — mortel
    Lava,
}

impl TerrainType {
    /// Peut-on construire sur ce terrain ?
    #[must_use]
    pub const fn is_buildable(&self) -> bool {
        matches!(
            self,
            Self::Beach | Self::Grass | Self::Hill | Self::Snow | Self::Desert
        )
    }

    /// Peut-on marcher sur ce terrain ?
    #[must_use]
    pub const fn is_walkable(&self) -> bool {
        !matches!(self, Self::DeepWater | Self::Mountain | Self::Lava)
    }

    /// Peut-on naviguer sur ce terrain (bateau) ?
    #[must_use]
    pub const fn is_navigable(&self) -> bool {
        matches!(self, Self::DeepWater | Self::ShallowWater)
    }

    /// Fertilité du terrain (0.0 - 1.0)
    #[must_use]
    pub const fn fertility(&self) -> f32 {
        match self {
            Self::DeepWater | Self::Mountain | Self::Lava => 0.0,
            Self::ShallowWater => 0.1,
            Self::Beach | Self::Desert => 0.2,
            Self::Snow => 0.1,
            Self::Hill => 0.4,
            Self::Swamp => 0.5,
            Self::Forest => 0.6,
            Self::Grass => 1.0,
        }
    }

    /// Modificateur de vitesse (1.0 = normal)
    #[must_use]
    pub const fn speed_modifier(&self) -> f32 {
        match self {
            Self::DeepWater | Self::Mountain | Self::Lava => 0.0,
            Self::ShallowWater => 0.5,
            Self::Swamp => 0.4,
            Self::Forest | Self::Hill => 0.7,
            Self::Snow | Self::Desert => 0.8,
            Self::Beach | Self::Grass => 1.0,
        }
    }

    /// Couleur RGB pour le rendu
    #[must_use]
    pub const fn color(&self) -> (u8, u8, u8) {
        match self {
            Self::DeepWater => (30, 60, 150),
            Self::ShallowWater => (70, 130, 180),
            Self::Beach => (238, 214, 175),
            Self::Grass => (86, 176, 76),
            Self::Forest => (34, 139, 34),
            Self::Hill => (139, 119, 101),
            Self::Mountain => (128, 128, 128),
            Self::Snow => (255, 250, 250),
            Self::Desert => (210, 180, 140),
            Self::Swamp => (85, 107, 47),
            Self::Lava => (255, 69, 0),
        }
    }
}

/// Une tuile du monde
///
/// @id: lifegame.world.tile
/// @do: represent_single_terrain_tile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    /// Type de terrain
    pub terrain_type: TerrainType,
    /// Élévation (0-255)
    pub elevation: u8,
    /// Température (-50 à +50)
    pub temperature: i8,
    /// Humidité (0-100)
    pub moisture: u8,
    /// Ressource présente (si applicable)
    pub resource: Option<ResourceDeposit>,
    /// Y a-t-il un arbre sur cette tuile ?
    pub has_tree: bool,
    /// Effet actif (feu, corruption, etc.)
    pub effect: Option<TileEffect>,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            terrain_type: TerrainType::Grass,
            elevation: 128,
            temperature: 20,
            moisture: 50,
            resource: None,
            has_tree: false,
            effect: None,
        }
    }
}

impl Tile {
    /// Crée une tuile d'un type donné
    #[must_use]
    pub fn new(terrain_type: TerrainType) -> Self {
        Self {
            terrain_type,
            ..Default::default()
        }
    }

    /// Crée une tuile d'eau
    #[must_use]
    pub fn water() -> Self {
        Self::new(TerrainType::DeepWater)
    }

    /// Crée une tuile de terre
    #[must_use]
    pub fn land() -> Self {
        Self::new(TerrainType::Grass)
    }

    /// Vérifie si la tuile est traversable à pied
    #[must_use]
    pub fn is_walkable(&self) -> bool {
        self.terrain_type.is_walkable() && !matches!(self.effect, Some(TileEffect::Fire))
    }

    /// Vérifie si on peut construire
    #[must_use]
    pub fn is_buildable(&self) -> bool {
        self.terrain_type.is_buildable() && !self.has_tree && self.effect.is_none()
    }
}

/// Effet temporaire sur une tuile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileEffect {
    /// En feu
    Fire,
    /// Inondée
    Flooded,
    /// Corrompue (tumeur)
    Corrupted,
    /// Bénie (terre sacrée)
    Blessed,
    /// Gelée
    Frozen,
}

/// Grille de terrain du monde
///
/// @id: lifegame.world.terrain_grid
/// @do: manage_2d_grid_of_tiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainGrid {
    /// Largeur en tuiles
    pub width: u32,
    /// Hauteur en tuiles
    pub height: u32,
    /// Grille de tuiles [y][x]
    tiles: Vec<Vec<Tile>>,
}

impl TerrainGrid {
    /// Crée une nouvelle grille vide (tout en herbe)
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        let tiles = vec![vec![Tile::default(); width as usize]; height as usize];
        Self { width, height, tiles }
    }

    /// Crée une nouvelle grille remplie d'eau
    #[must_use]
    pub fn new_ocean(width: u32, height: u32) -> Self {
        let tiles = vec![vec![Tile::water(); width as usize]; height as usize];
        Self { width, height, tiles }
    }

    /// Récupère une tuile (immutable)
    #[must_use]
    pub fn get(&self, x: u32, y: u32) -> Option<&Tile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y as usize][x as usize])
        } else {
            None
        }
    }

    /// Récupère une tuile (mutable)
    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Tile> {
        if x < self.width && y < self.height {
            Some(&mut self.tiles[y as usize][x as usize])
        } else {
            None
        }
    }

    /// Définit le type de terrain à une position
    pub fn set_terrain(&mut self, x: u32, y: u32, terrain_type: TerrainType) {
        if let Some(tile) = self.get_mut(x, y) {
            tile.terrain_type = terrain_type;
        }
    }

    /// Place un arbre à une position
    pub fn add_tree(&mut self, x: u32, y: u32) {
        if let Some(tile) = self.get_mut(x, y) {
            if tile.terrain_type == TerrainType::Grass || tile.terrain_type == TerrainType::Forest {
                tile.has_tree = true;
            }
        }
    }

    /// Retire un arbre d'une position
    pub fn remove_tree(&mut self, x: u32, y: u32) {
        if let Some(tile) = self.get_mut(x, y) {
            tile.has_tree = false;
        }
    }

    /// Applique un effet à une zone circulaire
    pub fn apply_effect_radius(&mut self, center_x: u32, center_y: u32, radius: u32, effect: TileEffect) {
        let cx = center_x as i32;
        let cy = center_y as i32;
        let r = radius as i32;

        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    let x = (cx + dx) as u32;
                    let y = (cy + dy) as u32;
                    if let Some(tile) = self.get_mut(x, y) {
                        tile.effect = Some(effect);
                    }
                }
            }
        }
    }

    /// Supprime les effets d'une zone
    pub fn clear_effects_radius(&mut self, center_x: u32, center_y: u32, radius: u32) {
        let cx = center_x as i32;
        let cy = center_y as i32;
        let r = radius as i32;

        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    let x = (cx + dx) as u32;
                    let y = (cy + dy) as u32;
                    if let Some(tile) = self.get_mut(x, y) {
                        tile.effect = None;
                    }
                }
            }
        }
    }

    /// Transforme un disque en un type de terrain
    pub fn paint_disk(&mut self, center_x: u32, center_y: u32, radius: u32, terrain_type: TerrainType) {
        let cx = center_x as i32;
        let cy = center_y as i32;
        let r = radius as i32;

        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    let x = (cx + dx) as u32;
                    let y = (cy + dy) as u32;
                    self.set_terrain(x, y, terrain_type);
                }
            }
        }
    }

    /// Compte le nombre de tuiles d'un type donné
    #[must_use]
    pub fn count_terrain_type(&self, terrain_type: TerrainType) -> usize {
        self.tiles
            .iter()
            .flat_map(|row| row.iter())
            .filter(|t| t.terrain_type == terrain_type)
            .count()
    }

    /// Vérifie si une position est valide
    #[must_use]
    pub fn is_valid(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain_type_properties() {
        assert!(TerrainType::Grass.is_buildable());
        assert!(TerrainType::Grass.is_walkable());
        assert!(!TerrainType::DeepWater.is_walkable());
        assert!(TerrainType::DeepWater.is_navigable());
    }

    #[test]
    fn test_terrain_grid_creation() {
        let grid = TerrainGrid::new(256, 256);
        assert_eq!(grid.width, 256);
        assert_eq!(grid.height, 256);
    }

    #[test]
    fn test_terrain_grid_access() {
        let mut grid = TerrainGrid::new(10, 10);
        grid.set_terrain(5, 5, TerrainType::Forest);
        assert_eq!(grid.get(5, 5).unwrap().terrain_type, TerrainType::Forest);
    }

    #[test]
    fn test_paint_disk() {
        let mut grid = TerrainGrid::new_ocean(100, 100);
        grid.paint_disk(50, 50, 10, TerrainType::Grass);
        assert_eq!(grid.get(50, 50).unwrap().terrain_type, TerrainType::Grass);
    }
}
