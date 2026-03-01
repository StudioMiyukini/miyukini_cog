//! Definitions de zones de jeu.
//!
//! <!-- @id: sd-data-zone-def @do: define @role: arpg @layer: 3 @human: miyuk -->
//!
//! Fichier TOML source : `data/zones/act{N}/{zone}.toml`

use serde::{Deserialize, Serialize};

/// Fichier TOML de zone (wrapper racine).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneFile {
    /// Definition de la zone.
    pub zone: ZoneDef,
}

/// Definition complete d'une zone de jeu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneDef {
    /// Identifiant unique (ex: `"act1/blood_moor"`).
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Numero d'acte (1-5).
    pub act: u8,
    /// Est-ce une ville ?
    pub is_town: bool,
    /// A-t-elle un waypoint ?
    pub has_waypoint: bool,
    /// Piste musicale.
    pub music_track: String,
    /// Piste d'ambiance.
    pub ambient_track: String,
    /// Tileset de la zone.
    pub tileset: String,
    /// Fichier de carte (LDtk).
    pub map_file: String,
    /// Niveaux de zone par difficulte.
    pub area_levels: AreaLevels,
    /// Connexions vers d'autres zones.
    pub connections: ZoneConnections,
    /// Groupes de spawn de monstres.
    #[serde(default)]
    pub spawn_groups: Vec<SpawnGroup>,
    /// Super-uniques presents dans la zone.
    #[serde(default)]
    pub super_uniques: Vec<ZoneSuperUnique>,
    /// Densite de champions.
    pub champion_density: ChampionDensity,
    /// Densite de monstres uniques.
    pub unique_density: UniqueDensity,
}

/// Niveaux de zone par difficulte.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaLevels {
    /// Niveau en Normal.
    pub normal: u8,
    /// Niveau en Nightmare.
    pub nightmare: u8,
    /// Niveau en Hell.
    pub hell: u8,
}

/// Connexions d'une zone vers ses voisines.
///
/// Chaque direction est optionnelle. Les noms correspondent a des IDs de zones.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneConnections {
    /// Zone au nord.
    #[serde(default)]
    pub north: Option<String>,
    /// Zone au sud.
    #[serde(default)]
    pub south: Option<String>,
    /// Zone a l'est.
    #[serde(default)]
    pub east: Option<String>,
    /// Zone a l'ouest.
    #[serde(default)]
    pub west: Option<String>,
    /// Grotte/caverne accessible.
    #[serde(default)]
    pub cave: Option<String>,
    /// Donjon accessible.
    #[serde(default)]
    pub dungeon: Option<String>,
}

/// Groupe de spawn de monstres dans une zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnGroup {
    /// ID du monstre a spawn.
    pub monster_id: String,
    /// Nombre minimum de monstres.
    pub min_count: u32,
    /// Nombre maximum de monstres.
    pub max_count: u32,
    /// Centre de la zone de spawn.
    pub area_center: [f32; 2],
    /// Rayon de la zone de spawn.
    pub area_radius: f32,
}

/// Reference a un super-unique dans une zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneSuperUnique {
    /// ID du super-unique.
    pub su_id: String,
    /// Position de spawn.
    pub position: [f32; 2],
}

/// Configuration de densite de champions dans une zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionDensity {
    /// Nombre de packs par zone.
    pub packs_per_area: u32,
    /// Taille minimale d'un pack.
    pub min_pack_size: u32,
    /// Taille maximale d'un pack.
    pub max_pack_size: u32,
    /// Nombre d'affixes par champion.
    pub affixes_count: u8,
}

/// Configuration de densite de monstres uniques dans une zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueDensity {
    /// Nombre de packs par zone.
    pub packs_per_area: u32,
    /// Nombre minimum de minions.
    pub minions_min: u32,
    /// Nombre maximum de minions.
    pub minions_max: u32,
    /// Nombre minimum d'affixes.
    pub affixes_count_min: u8,
    /// Nombre maximum d'affixes.
    pub affixes_count_max: u8,
}
