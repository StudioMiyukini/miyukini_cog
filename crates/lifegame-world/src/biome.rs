//! # Biomes
//!
//! Gestion des zones climatiques et biomes.
//!
//! @id: lifegame.world.biome
//! @role: simulation
//! @layer: domain
//! @do: manage_climate_zones_and_biomes
//! @human: Définition des biomes et leurs caractéristiques

use serde::{Deserialize, Serialize};

/// Type de biome
///
/// @id: lifegame.world.biome_type
/// @do: enumerate_biome_types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BiomeType {
    /// Océan
    Ocean,
    /// Plaines tempérées — équilibré, idéal
    #[default]
    TemperatePlains,
    /// Forêt dense — favorise les Elfes
    DenseForest,
    /// Montagnes — favorise les Nains
    Mountains,
    /// Désert — difficile
    Desert,
    /// Toundra — froid, croissance lente
    Tundra,
    /// Jungle — dense, maladies fréquentes
    Jungle,
    /// Marais — malsain mais riche
    Swamp,
    /// Volcanique — dangereux mais minerais
    Volcanic,
    /// Arctique — très froid, survie difficile
    Arctic,
    /// Savane — sec, faune abondante
    Savanna,
}

impl BiomeType {
    /// Température moyenne du biome
    #[must_use]
    pub const fn average_temperature(&self) -> i8 {
        match self {
            Self::Ocean => 15,
            Self::TemperatePlains => 20,
            Self::DenseForest => 18,
            Self::Mountains => 5,
            Self::Desert => 40,
            Self::Tundra => -10,
            Self::Jungle => 30,
            Self::Swamp => 25,
            Self::Volcanic => 35,
            Self::Arctic => -30,
            Self::Savanna => 28,
        }
    }

    /// Humidité moyenne (0-100)
    #[must_use]
    pub const fn average_moisture(&self) -> u8 {
        match self {
            Self::Ocean => 100,
            Self::TemperatePlains => 50,
            Self::DenseForest => 70,
            Self::Mountains => 40,
            Self::Desert => 10,
            Self::Tundra => 30,
            Self::Jungle => 90,
            Self::Swamp => 95,
            Self::Volcanic => 20,
            Self::Arctic => 40,
            Self::Savanna => 25,
        }
    }

    /// Multiplicateur de fertilité
    #[must_use]
    pub const fn fertility_modifier(&self) -> f32 {
        match self {
            Self::Ocean => 0.0,
            Self::TemperatePlains => 1.0,
            Self::DenseForest => 0.7,
            Self::Mountains => 0.2,
            Self::Desert => 0.1,
            Self::Tundra => 0.2,
            Self::Jungle => 0.8,
            Self::Swamp => 0.5,
            Self::Volcanic => 0.3,
            Self::Arctic => 0.05,
            Self::Savanna => 0.4,
        }
    }

    /// Taux de spawn d'animaux
    #[must_use]
    pub const fn wildlife_spawn_rate(&self) -> f32 {
        match self {
            Self::Ocean => 0.5,  // Poissons
            Self::TemperatePlains => 1.0,
            Self::DenseForest => 1.5,
            Self::Mountains => 0.3,
            Self::Desert => 0.2,
            Self::Tundra => 0.4,
            Self::Jungle => 2.0,
            Self::Swamp => 0.8,
            Self::Volcanic => 0.1,
            Self::Arctic => 0.3,
            Self::Savanna => 1.2,
        }
    }

    /// Risque de maladie (0.0 - 1.0)
    #[must_use]
    pub const fn disease_risk(&self) -> f32 {
        match self {
            Self::Ocean => 0.0,
            Self::TemperatePlains => 0.1,
            Self::DenseForest => 0.15,
            Self::Mountains => 0.05,
            Self::Desert => 0.05,
            Self::Tundra => 0.1,
            Self::Jungle => 0.4,
            Self::Swamp => 0.5,
            Self::Volcanic => 0.2,
            Self::Arctic => 0.15,
            Self::Savanna => 0.1,
        }
    }

    /// Couleur de base du biome pour la minimap
    #[must_use]
    pub const fn map_color(&self) -> (u8, u8, u8) {
        match self {
            Self::Ocean => (30, 60, 150),
            Self::TemperatePlains => (86, 176, 76),
            Self::DenseForest => (34, 139, 34),
            Self::Mountains => (128, 128, 128),
            Self::Desert => (210, 180, 140),
            Self::Tundra => (200, 200, 200),
            Self::Jungle => (0, 100, 0),
            Self::Swamp => (85, 107, 47),
            Self::Volcanic => (139, 69, 19),
            Self::Arctic => (240, 248, 255),
            Self::Savanna => (218, 165, 32),
        }
    }
}

/// Un biome dans le monde
///
/// @id: lifegame.world.biome_struct
/// @do: represent_biome_region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biome {
    /// Type de biome
    pub biome_type: BiomeType,
    /// Centre du biome (x, y)
    pub center: (u32, u32),
    /// Rayon approximatif
    pub radius: u32,
    /// Température locale (peut varier)
    pub temperature: i8,
    /// Humidité locale
    pub moisture: u8,
}

impl Biome {
    /// Crée un nouveau biome
    #[must_use]
    pub fn new(biome_type: BiomeType, center: (u32, u32), radius: u32) -> Self {
        Self {
            biome_type,
            center,
            radius,
            temperature: biome_type.average_temperature(),
            moisture: biome_type.average_moisture(),
        }
    }

    /// Vérifie si un point est dans ce biome (approximatif)
    #[must_use]
    pub fn contains(&self, x: u32, y: u32) -> bool {
        let dx = x as i32 - self.center.0 as i32;
        let dy = y as i32 - self.center.1 as i32;
        let dist_sq = (dx * dx + dy * dy) as u32;
        dist_sq <= self.radius * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biome_type_properties() {
        assert!(BiomeType::Jungle.average_moisture() > BiomeType::Desert.average_moisture());
        assert!(BiomeType::Arctic.average_temperature() < BiomeType::Desert.average_temperature());
    }

    #[test]
    fn test_biome_contains() {
        let biome = Biome::new(BiomeType::TemperatePlains, (100, 100), 50);
        assert!(biome.contains(100, 100));
        assert!(biome.contains(120, 100));
        assert!(!biome.contains(200, 200));
    }
}
