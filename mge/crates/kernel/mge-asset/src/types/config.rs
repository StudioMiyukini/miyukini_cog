//! Definitions de configuration globale : difficulte, breakpoints, experience, shrines.
//!
//! <!-- @id: sd-data-config @do: define @role: arpg @layer: 3 @human: miyuk -->
//!
//! Fichiers TOML source :
//! - `data/config/difficulty.toml`
//! - `data/config/breakpoints.toml`
//! - `data/config/experience.toml`
//! - `data/config/shrines.toml`

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// =========================================================================
// Difficulte
// =========================================================================

/// Configuration des trois niveaux de difficulte.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyConfig {
    /// Parametres de la difficulte Normal.
    pub normal: DifficultyLevel,
    /// Parametres de la difficulte Nightmare.
    pub nightmare: DifficultyLevel,
    /// Parametres de la difficulte Hell.
    pub hell: DifficultyLevel,
}

/// Parametres d'un niveau de difficulte.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyLevel {
    /// Penalite de resistance au feu.
    pub fire_resist_penalty: i32,
    /// Penalite de resistance au froid.
    pub cold_resist_penalty: i32,
    /// Penalite de resistance a la foudre.
    pub lightning_resist_penalty: i32,
    /// Penalite de resistance au poison.
    pub poison_resist_penalty: i32,
    /// Multiplicateur d'experience.
    pub experience_multiplier: f32,
    /// Multiplicateur d'or.
    pub gold_multiplier: f32,
}

// =========================================================================
// Breakpoints
// =========================================================================

/// Tables de breakpoints par classe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointsFile {
    /// Faster Cast Rate par classe.
    #[serde(default)]
    pub fcr: HashMap<String, BreakpointTable>,
    /// Faster Hit Recovery par classe.
    #[serde(default)]
    pub fhr: HashMap<String, BreakpointTable>,
    /// Faster Block Rate par classe.
    #[serde(default)]
    pub fbr: HashMap<String, BreakpointTable>,
    /// Increased Attack Speed (calcule differemment, voir doc).
    #[serde(default)]
    pub ias: HashMap<String, BreakpointTable>,
}

/// Table de breakpoints pour une classe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointTable {
    /// Seuils de breakpoint (valeurs de stat).
    pub breakpoints: Vec<i32>,
    /// Nombre de frames correspondant a chaque seuil.
    pub frames: Vec<u32>,
}

// =========================================================================
// Experience
// =========================================================================

/// Fichier TOML de table d'experience.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceFile {
    /// Experience requise par niveau (index = niveau - 1).
    pub experience_table: Vec<u64>,
}

// =========================================================================
// Shrines
// =========================================================================

/// Fichier TOML de shrines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShrinesFile {
    /// Liste des shrines.
    pub shrines: Vec<ShrineDef>,
}

/// Definition d'un shrine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShrineDef {
    /// Identifiant unique.
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Type d'effet.
    pub effect: String,
    /// Duree de l'effet en secondes.
    pub duration_seconds: f32,
    /// Valeur de l'effet.
    pub value: i32,
}
