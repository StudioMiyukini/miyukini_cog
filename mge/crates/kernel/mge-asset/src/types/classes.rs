//! Definitions de classes de personnages.
//!
//! <!-- @id: sd-data-class-def @do: define @role: arpg @layer: 3 @human: miyuk -->
//!
//! Fichier TOML source : `data/classes/{classe}.toml`

use serde::{Deserialize, Serialize};

/// Fichier TOML de classe (wrapper racine).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassFile {
    /// Definition de la classe.
    pub class: ClassDef,
}

/// Definition complete d'une classe de personnage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDef {
    /// Identifiant unique de la classe (ex: `"mortecian"`).
    pub id: String,
    /// Nom affiche de la classe (ex: `"Mortecian"`).
    pub name: String,
    /// Nom de reference D2 (ex: `"Necromancer"`).
    pub d2_name: String,
    /// Stats de base au niveau 1.
    pub base_stats: ClassBaseStats,
    /// Gains par niveau.
    pub per_level: ClassPerLevel,
    /// Parametres de combat.
    pub combat: ClassCombat,
    /// Configuration des arbres de competences.
    pub skill_trees: ClassSkillTrees,
    /// Sprites de la classe.
    pub sprite: ClassSprite,
}

/// Stats de base d'une classe au niveau 1.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassBaseStats {
    /// Force de base.
    pub strength: i32,
    /// Dexterite de base.
    pub dexterity: i32,
    /// Vitalite de base.
    pub vitality: i32,
    /// Energie de base.
    pub energy: i32,
    /// Points de vie de base.
    pub life: i32,
    /// Points de mana de base.
    pub mana: i32,
    /// Endurance de base.
    pub stamina: i32,
}

/// Gains de stats par niveau pour une classe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassPerLevel {
    /// Vie gagnee par niveau.
    pub life_per_level: f32,
    /// Mana gagne par niveau.
    pub mana_per_level: f32,
    /// Endurance gagnee par niveau.
    pub stamina_per_level: f32,
    /// Vie gagnee par point de vitalite.
    pub life_per_vitality: i32,
    /// Mana gagne par point d'energie.
    pub mana_per_energy: f32,
    /// Endurance gagnee par point de vitalite.
    pub stamina_per_vitality: i32,
}

/// Parametres de combat specifiques a la classe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassCombat {
    /// Attack Rating de base de la classe.
    pub class_base_ar: i32,
    /// Nombre de frames de base pour le block.
    pub base_block_frames: u32,
}

/// Configuration des arbres de competences de la classe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassSkillTrees {
    /// Noms des arbres (ex: `["Summoning", "PoisonBone", "Curses"]`).
    pub trees: Vec<String>,
    /// Nombre de skills par arbre.
    pub skills_per_tree: u8,
    /// Nombre total de skills.
    pub total_skills: u8,
}

/// Sprites de la classe pour l'animation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassSprite {
    /// Sprite de base (idle/walk).
    pub base_sprite: String,
    /// Sprite d'attaque.
    pub attack_sprite: String,
    /// Sprite de cast.
    pub cast_sprite: String,
}
