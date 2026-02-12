//! # LifeGame World — Opérateur Monde
//!
//! Gestion du terrain, biomes, ressources et génération procédurale.
//!
//! @id: lifegame.operator.world
//! @role: simulation
//! @layer: domain
//! @do: manage_terrain_biomes_resources_generation
//! @human: Opérateur gérant le monde physique : terrain, biomes, ressources naturelles

use serde::{Deserialize, Serialize};

pub mod terrain;
pub mod biome;
pub mod generation;

pub use terrain::{Tile, TerrainType, TerrainGrid};
pub use biome::{Biome, BiomeType};
pub use generation::WorldGenerator;

/// Taille prédéfinie de monde
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WorldSize {
    /// Petit : 256×256
    Small,
    /// Moyen : 512×512
    #[default]
    Medium,
    /// Grand : 1024×1024
    Large,
}

impl WorldSize {
    /// Retourne les dimensions (largeur, hauteur)
    #[must_use]
    pub const fn dimensions(&self) -> (u32, u32) {
        match self {
            Self::Small => (256, 256),
            Self::Medium => (512, 512),
            Self::Large => (1024, 1024),
        }
    }

    /// Retourne la largeur
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.dimensions().0
    }

    /// Retourne la hauteur
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.dimensions().1
    }

    /// Retourne le nombre total de tuiles
    #[must_use]
    pub const fn total_tiles(&self) -> u32 {
        let (w, h) = self.dimensions();
        w * h
    }
}

/// Métadonnées du monde
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMetadata {
    /// Identifiant unique du monde
    pub id: u64,
    /// Nom du monde
    pub name: String,
    /// Taille du monde
    pub size: WorldSize,
    /// Seed de génération
    pub seed: u64,
    /// Timestamp de création
    pub created_at: u64,
    /// Timestamp de dernière sauvegarde
    pub last_saved: u64,
    /// Temps de jeu total (en secondes)
    pub playtime_seconds: u64,
}

impl Default for WorldMetadata {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from("Nouveau Monde"),
            size: WorldSize::Medium,
            seed: rand::random(),
            created_at: 0,
            last_saved: 0,
            playtime_seconds: 0,
        }
    }
}

impl WorldMetadata {
    /// Crée de nouvelles métadonnées pour un monde
    #[must_use]
    pub fn new(name: impl Into<String>, size: WorldSize, seed: u64) -> Self {
        Self {
            id: rand::random(),
            name: name.into(),
            size,
            seed,
            ..Default::default()
        }
    }
}

/// Ressource naturelle sur une tuile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    /// Bois (arbres)
    Wood,
    /// Pierre
    Stone,
    /// Fer
    Iron,
    /// Or
    Gold,
    /// Charbon
    Coal,
    /// Diamant
    Diamond,
    /// Poisson (dans l'eau)
    Fish,
}

impl ResourceType {
    /// Valeur de base de la ressource
    #[must_use]
    pub const fn base_value(&self) -> u32 {
        match self {
            Self::Wood => 1,
            Self::Stone => 2,
            Self::Iron => 5,
            Self::Gold => 20,
            Self::Coal => 3,
            Self::Diamond => 100,
            Self::Fish => 2,
        }
    }
}

/// Dépôt de ressource sur une tuile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDeposit {
    /// Type de ressource
    pub resource_type: ResourceType,
    /// Quantité restante
    pub quantity: u32,
    /// Quantité initiale
    pub initial_quantity: u32,
}

impl ResourceDeposit {
    /// Crée un nouveau dépôt
    #[must_use]
    pub fn new(resource_type: ResourceType, quantity: u32) -> Self {
        Self {
            resource_type,
            quantity,
            initial_quantity: quantity,
        }
    }

    /// Extrait une quantité de la ressource
    pub fn extract(&mut self, amount: u32) -> u32 {
        let extracted = amount.min(self.quantity);
        self.quantity -= extracted;
        extracted
    }

    /// Vérifie si le dépôt est épuisé
    #[must_use]
    pub fn is_depleted(&self) -> bool {
        self.quantity == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_size() {
        assert_eq!(WorldSize::Small.dimensions(), (256, 256));
        assert_eq!(WorldSize::Medium.total_tiles(), 512 * 512);
    }

    #[test]
    fn test_resource_deposit() {
        let mut deposit = ResourceDeposit::new(ResourceType::Iron, 100);
        let extracted = deposit.extract(30);
        assert_eq!(extracted, 30);
        assert_eq!(deposit.quantity, 70);
    }
}
