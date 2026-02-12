//! # Génération procédurale
//!
//! Génération de mondes avec Perlin noise.
//!
//! @id: lifegame.world.generation
//! @role: simulation
//! @layer: domain
//! @do: procedurally_generate_terrain_with_noise
//! @human: Générateur de monde utilisant le bruit de Perlin

use noise::{NoiseFn, Perlin};
use crate::{TerrainGrid, TerrainType, WorldSize, ResourceDeposit, ResourceType};

/// Générateur de monde procédural
///
/// @id: lifegame.world.generator
/// @do: generate_terrain_biomes_resources
pub struct WorldGenerator {
    /// Seed de génération
    seed: u64,
    /// Générateur Perlin pour l'élévation
    elevation_noise: Perlin,
    /// Générateur Perlin pour l'humidité
    moisture_noise: Perlin,
    /// Générateur Perlin pour la température
    temperature_noise: Perlin,
}

impl WorldGenerator {
    /// Crée un nouveau générateur avec une seed
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            elevation_noise: Perlin::new(seed as u32),
            moisture_noise: Perlin::new(seed.wrapping_add(1) as u32),
            temperature_noise: Perlin::new(seed.wrapping_add(2) as u32),
        }
    }

    /// Génère une grille de terrain complète
    #[must_use]
    pub fn generate(&self, size: WorldSize) -> TerrainGrid {
        let (width, height) = size.dimensions();
        let mut grid = TerrainGrid::new(width, height);

        // Échelles pour le bruit (plus grand = plus doux)
        let elevation_scale = 100.0;
        let moisture_scale = 80.0;
        let temperature_scale = 150.0;

        for y in 0..height {
            for x in 0..width {
                let fx = f64::from(x) / elevation_scale;
                let fy = f64::from(y) / elevation_scale;

                // Élévation avec plusieurs octaves
                let elevation = self.sample_elevation(fx, fy);
                
                // Humidité
                let moisture = self.sample_moisture(
                    f64::from(x) / moisture_scale,
                    f64::from(y) / moisture_scale,
                );
                
                // Température (varie avec latitude)
                let latitude_factor = (f64::from(y) / f64::from(height) - 0.5).abs() * 2.0;
                let temperature = self.sample_temperature(
                    f64::from(x) / temperature_scale,
                    f64::from(y) / temperature_scale,
                    latitude_factor,
                );

                // Détermine le type de terrain
                let terrain_type = self.elevation_to_terrain(elevation, moisture, temperature);

                if let Some(tile) = grid.get_mut(x, y) {
                    tile.terrain_type = terrain_type;
                    tile.elevation = ((elevation + 1.0) * 0.5 * 255.0) as u8;
                    tile.moisture = (moisture * 100.0) as u8;
                    tile.temperature = (temperature * 50.0) as i8;
                    
                    // Ajoute des arbres dans les forêts
                    if terrain_type == TerrainType::Forest {
                        tile.has_tree = rand::random::<f32>() < 0.6;
                    }
                }
            }
        }

        // Ajoute des ressources
        self.scatter_resources(&mut grid);

        grid
    }

    /// Échantillonne l'élévation avec plusieurs octaves
    fn sample_elevation(&self, x: f64, y: f64) -> f64 {
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;
        let octaves = 4;

        for _ in 0..octaves {
            value += self.elevation_noise.get([x * frequency, y * frequency]) * amplitude;
            amplitude *= 0.5;
            frequency *= 2.0;
        }

        value
    }

    /// Échantillonne l'humidité
    fn sample_moisture(&self, x: f64, y: f64) -> f64 {
        let value = self.moisture_noise.get([x, y]);
        (value + 1.0) * 0.5 // Normalise à [0, 1]
    }

    /// Échantillonne la température
    fn sample_temperature(&self, x: f64, y: f64, latitude_factor: f64) -> f64 {
        let base = self.temperature_noise.get([x, y]);
        // Plus froid vers les pôles (haut et bas de la carte)
        let adjusted = base - latitude_factor * 0.8;
        adjusted.clamp(-1.0, 1.0)
    }

    /// Convertit les valeurs de bruit en type de terrain
    fn elevation_to_terrain(&self, elevation: f64, moisture: f64, temperature: f64) -> TerrainType {
        // Eau profonde
        if elevation < -0.3 {
            return TerrainType::DeepWater;
        }

        // Eau peu profonde
        if elevation < -0.1 {
            return TerrainType::ShallowWater;
        }

        // Plage
        if elevation < 0.0 {
            return TerrainType::Beach;
        }

        // Montagnes
        if elevation > 0.7 {
            if temperature < -0.3 {
                return TerrainType::Snow;
            }
            return TerrainType::Mountain;
        }

        // Collines
        if elevation > 0.5 {
            if temperature < -0.2 {
                return TerrainType::Snow;
            }
            return TerrainType::Hill;
        }

        // Zones basses et moyennes
        // Température très basse = neige
        if temperature < -0.4 {
            return TerrainType::Snow;
        }

        // Très sec = désert
        if moisture < 0.2 && temperature > 0.2 {
            return TerrainType::Desert;
        }

        // Très humide et chaud = marais
        if moisture > 0.8 && temperature > 0.0 && elevation < 0.2 {
            return TerrainType::Swamp;
        }

        // Humide = forêt
        if moisture > 0.5 {
            return TerrainType::Forest;
        }

        // Par défaut = plaines
        TerrainType::Grass
    }

    /// Disperse des ressources sur la carte
    fn scatter_resources(&self, grid: &mut TerrainGrid) {
        let resource_noise = Perlin::new(self.seed.wrapping_add(100) as u32);
        
        for y in 0..grid.height {
            for x in 0..grid.width {
                let fx = f64::from(x) / 50.0;
                let fy = f64::from(y) / 50.0;
                let value = resource_noise.get([fx, fy]);

                if let Some(tile) = grid.get_mut(x, y) {
                    // Minerais dans les montagnes et collines
                    if matches!(tile.terrain_type, TerrainType::Mountain | TerrainType::Hill) {
                        if value > 0.7 {
                            tile.resource = Some(ResourceDeposit::new(ResourceType::Gold, 50));
                        } else if value > 0.5 {
                            tile.resource = Some(ResourceDeposit::new(ResourceType::Iron, 100));
                        } else if value > 0.3 {
                            tile.resource = Some(ResourceDeposit::new(ResourceType::Stone, 200));
                        }
                    }

                    // Poissons dans l'eau
                    if matches!(tile.terrain_type, TerrainType::ShallowWater) && value > 0.6 {
                        tile.resource = Some(ResourceDeposit::new(ResourceType::Fish, 150));
                    }
                }
            }
        }
    }
}

/// Génère un monde simple pour les tests
#[must_use]
pub fn generate_simple_world(size: WorldSize) -> TerrainGrid {
    let (width, height) = size.dimensions();
    let mut grid = TerrainGrid::new_ocean(width, height);

    // Crée un continent central
    let cx = width / 2;
    let cy = height / 2;
    let radius = width / 3;

    grid.paint_disk(cx, cy, radius, TerrainType::Grass);

    // Ajoute des forêts
    let forest_radius = radius / 3;
    grid.paint_disk(cx - radius / 2, cy, forest_radius, TerrainType::Forest);
    grid.paint_disk(cx + radius / 2, cy, forest_radius, TerrainType::Forest);

    // Ajoute des montagnes
    let mountain_radius = radius / 4;
    grid.paint_disk(cx, cy - radius / 2, mountain_radius, TerrainType::Mountain);

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_generator() {
        let generator = WorldGenerator::new(12345);
        let grid = generator.generate(WorldSize::Small);
        
        assert_eq!(grid.width, 256);
        assert_eq!(grid.height, 256);
        
        // Vérifie qu'il y a de la variété
        let grass_count = grid.count_terrain_type(TerrainType::Grass);
        let water_count = grid.count_terrain_type(TerrainType::DeepWater);
        
        assert!(grass_count > 0);
        assert!(water_count > 0);
    }

    #[test]
    fn test_simple_world() {
        let grid = generate_simple_world(WorldSize::Small);
        
        // Le centre devrait être de l'herbe
        assert_eq!(grid.get(128, 128).unwrap().terrain_type, TerrainType::Grass);
    }
}
