// @id: Sodomight-DataLoader @do: data-loading @role: back-end @layer: 4 @human: miyuk
//! TOML data loader for Sodomight game content.
//!
//! Provides typed deserialization of game data from the `data/` directory tree.
//! All loading functions accept a `data_dir` path, making them testable without
//! relying on the filesystem by using inline TOML strings in unit tests.

use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors that can occur when loading game data from TOML files.
#[derive(Debug, thiserror::Error)]
pub enum DataLoadError {
    /// The file could not be read from disk.
    #[error("Failed to read {path}: {source}")]
    Io {
        /// Path that was attempted.
        path: PathBuf,
        /// Underlying I/O error.
        source: std::io::Error,
    },
    /// The file contents could not be parsed as TOML.
    #[error("Failed to parse {path}: {message}")]
    Parse {
        /// Path that was attempted.
        path: PathBuf,
        /// TOML parse error message.
        message: String,
    },
}

// ---------------------------------------------------------------------------
// Game config structs
// ---------------------------------------------------------------------------

/// Top-level game configuration loaded from `config/game.toml`.
#[derive(Debug, serde::Deserialize)]
pub struct GameConfig {
    /// Core game settings.
    pub game: GameSettings,
    /// Per-difficulty modifier settings.
    pub difficulty: DifficultySettings,
}

/// Core game settings (title, version, initial values).
#[derive(Debug, serde::Deserialize)]
pub struct GameSettings {
    /// Game title shown in the main menu.
    pub title: String,
    /// Game data version string.
    pub version: String,
    /// Maximum character level cap.
    pub max_level: u32,
    /// Gold amount granted at character creation.
    pub starting_gold: i64,
    /// Act number where a new character begins.
    pub starting_act: u8,
}

/// Resistance penalty applied per difficulty level.
#[derive(Debug, serde::Deserialize)]
pub struct DifficultySettings {
    /// Resistance penalty on Normal (typically 0).
    pub normal_resist_penalty: i32,
    /// Resistance penalty on Nightmare.
    pub nightmare_resist_penalty: i32,
    /// Resistance penalty on Hell.
    pub hell_resist_penalty: i32,
}

// ---------------------------------------------------------------------------
// Class structs
// ---------------------------------------------------------------------------

/// Class data loaded from `classes/<name>.toml`.
#[derive(Debug, serde::Deserialize)]
pub struct ClassData {
    /// Base attribute and derived stats for this class.
    pub base_stats: BaseStats,
}

/// Base stats for a playable class at level 1.
#[derive(Debug, serde::Deserialize)]
pub struct BaseStats {
    /// Base strength attribute.
    pub strength: u32,
    /// Base dexterity attribute.
    pub dexterity: u32,
    /// Base vitality attribute.
    pub vitality: u32,
    /// Base energy attribute.
    pub energy: u32,
    /// Starting life points.
    pub life: u32,
    /// Starting mana points.
    pub mana: u32,
    /// Starting stamina points.
    pub stamina: u32,
    /// Life gained per vitality point.
    pub life_per_vitality: f32,
    /// Mana gained per energy point.
    pub mana_per_energy: f32,
    /// Life gained per character level.
    pub life_per_level: f32,
    /// Mana gained per character level.
    pub mana_per_level: f32,
    /// Stamina gained per vitality point.
    pub stamina_per_vitality: f32,
}

// ---------------------------------------------------------------------------
// Skill structs
// ---------------------------------------------------------------------------

/// Outer wrapper for a skill TOML file (`[skill]` section).
#[derive(Debug, serde::Deserialize)]
pub struct SkillToml {
    /// Skill definition data.
    pub skill: SkillData,
}

/// Skill definition loaded from a skill TOML file.
#[derive(Debug, serde::Deserialize)]
pub struct SkillData {
    /// Unique skill identifier (e.g. `"necro_raise_skeleton"`).
    pub id: String,
    /// Display name shown in the skill tree UI.
    pub name: String,
    /// Skill tree index (0 = Summoning, 1 = Bone, 2 = Curses for Necromancer).
    pub tree: u8,
    /// Skill kind tag (e.g. `"Summon"`, `"Projectile"`, `"Curse"`).
    pub kind: String,
    /// Damage type tag (e.g. `"Physical"`, `"Fire"`, `"Cold"`).
    pub damage_type: String,
    /// Maximum learnable skill level.
    pub max_level: u32,
    /// Mana cost at skill level 1.
    pub mana_cost_base: f32,
    /// Additional mana cost per skill level above 1.
    pub mana_cost_per_level: f32,
    /// Cooldown in milliseconds (0 = no cooldown).
    pub cooldown_ms: u32,
    /// Minimum damage at level 1 (0 for non-damaging skills).
    pub base_damage_min: i32,
    /// Maximum damage at level 1 (0 for non-damaging skills).
    pub base_damage_max: i32,
    /// Flat damage added per skill level.
    pub damage_per_level: i32,
    /// IDs of skills required before this one can be learned.
    pub prerequisites: Vec<String>,
}

// ---------------------------------------------------------------------------
// Monster structs
// ---------------------------------------------------------------------------

/// Outer wrapper for a monster TOML file (`[monster]` section).
#[derive(Debug, serde::Deserialize)]
pub struct MonsterToml {
    /// Monster definition data.
    pub monster: MonsterData,
}

/// Monster definition loaded from a monster TOML file.
#[derive(Debug, serde::Deserialize)]
pub struct MonsterData {
    /// Unique monster identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Monster level used for difficulty scaling.
    pub level: u32,
    /// Base hit points.
    pub base_hp: i32,
    /// Minimum physical damage per hit.
    pub base_damage_min: i32,
    /// Maximum physical damage per hit.
    pub base_damage_max: i32,
    /// Defense rating.
    pub defense: i32,
    /// Attack rating.
    pub attack_rating: i32,
    /// Experience awarded on kill.
    pub experience: i64,
}

// ---------------------------------------------------------------------------
// Loot structs
// ---------------------------------------------------------------------------

/// Loot file containing one or more treasure class entries.
#[derive(Debug, serde::Deserialize)]
pub struct LootFile {
    /// Array of treasure class definitions.
    pub treasure_class: Vec<TreasureClassData>,
}

/// Single treasure class entry from the loot TOML.
#[derive(Debug, serde::Deserialize)]
pub struct TreasureClassData {
    /// Unique treasure class identifier.
    pub id: String,
    /// Monster that references this TC.
    pub monster_id: String,
    /// Item IDs that can drop.
    pub items: Vec<String>,
    /// Drop chance percentage for each item (must match `items` length).
    pub chances: Vec<u32>,
}

// ---------------------------------------------------------------------------
// Loading functions
// ---------------------------------------------------------------------------

/// Loads the main game configuration from `<data_dir>/config/game.toml`.
///
/// # Errors
///
/// Returns [`DataLoadError::Io`] if the file cannot be read, or
/// [`DataLoadError::Parse`] if the TOML is malformed.
pub fn load_game_config(data_dir: &Path) -> Result<GameConfig, DataLoadError> {
    let path = data_dir.join("config/game.toml");
    let content = std::fs::read_to_string(&path).map_err(|e| DataLoadError::Io {
        path: path.clone(),
        source: e,
    })?;
    toml::from_str(&content).map_err(|e| DataLoadError::Parse {
        path,
        message: e.to_string(),
    })
}

/// Loads class data from `<data_dir>/classes/<class_name>.toml`.
///
/// # Errors
///
/// Returns [`DataLoadError::Io`] if the file cannot be read, or
/// [`DataLoadError::Parse`] if the TOML is malformed.
pub fn load_class_data(data_dir: &Path, class_name: &str) -> Result<ClassData, DataLoadError> {
    let path = data_dir.join(format!("classes/{class_name}.toml"));
    let content = std::fs::read_to_string(&path).map_err(|e| DataLoadError::Io {
        path: path.clone(),
        source: e,
    })?;
    toml::from_str(&content).map_err(|e| DataLoadError::Parse {
        path,
        message: e.to_string(),
    })
}

/// Loads a skill definition from `<data_dir>/skills/<tree>/<skill_file>.toml`.
///
/// # Errors
///
/// Returns [`DataLoadError::Io`] if the file cannot be read, or
/// [`DataLoadError::Parse`] if the TOML is malformed.
pub fn load_skill(
    data_dir: &Path,
    tree: &str,
    skill_file: &str,
) -> Result<SkillToml, DataLoadError> {
    let path = data_dir.join(format!("skills/{tree}/{skill_file}.toml"));
    let content = std::fs::read_to_string(&path).map_err(|e| DataLoadError::Io {
        path: path.clone(),
        source: e,
    })?;
    toml::from_str(&content).map_err(|e| DataLoadError::Parse {
        path,
        message: e.to_string(),
    })
}

/// Loads a monster definition from `<data_dir>/monsters/<act>/<monster_file>.toml`.
///
/// # Errors
///
/// Returns [`DataLoadError::Io`] if the file cannot be read, or
/// [`DataLoadError::Parse`] if the TOML is malformed.
pub fn load_monster(
    data_dir: &Path,
    act: &str,
    monster_file: &str,
) -> Result<MonsterToml, DataLoadError> {
    let path = data_dir.join(format!("monsters/{act}/{monster_file}.toml"));
    let content = std::fs::read_to_string(&path).map_err(|e| DataLoadError::Io {
        path: path.clone(),
        source: e,
    })?;
    toml::from_str(&content).map_err(|e| DataLoadError::Parse {
        path,
        message: e.to_string(),
    })
}

/// Loads a loot table from `<data_dir>/loot/<loot_file>.toml`.
///
/// # Errors
///
/// Returns [`DataLoadError::Io`] if the file cannot be read, or
/// [`DataLoadError::Parse`] if the TOML is malformed.
pub fn load_loot(data_dir: &Path, loot_file: &str) -> Result<LootFile, DataLoadError> {
    let path = data_dir.join(format!("loot/{loot_file}.toml"));
    let content = std::fs::read_to_string(&path).map_err(|e| DataLoadError::Io {
        path: path.clone(),
        source: e,
    })?;
    toml::from_str(&content).map_err(|e| DataLoadError::Parse {
        path,
        message: e.to_string(),
    })
}

// ---------------------------------------------------------------------------
// Unit tests (inline TOML — no file I/O)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Parses the game config from an inline TOML string.
    #[test]
    fn parse_game_config() {
        let raw = r#"
            [game]
            title = "Sodomight"
            version = "0.1.0"
            max_level = 99
            starting_gold = 0
            starting_act = 1

            [difficulty]
            normal_resist_penalty = 0
            nightmare_resist_penalty = -40
            hell_resist_penalty = -100
        "#;

        let config: GameConfig = toml::from_str(raw).expect("GameConfig parse failed");
        assert_eq!(config.game.title, "Sodomight");
        assert_eq!(config.game.max_level, 99);
        assert_eq!(config.game.starting_act, 1);
        assert_eq!(config.difficulty.hell_resist_penalty, -100);
        assert_eq!(config.difficulty.nightmare_resist_penalty, -40);
        assert_eq!(config.difficulty.normal_resist_penalty, 0);
    }

    /// Parses necromancer class data from an inline TOML string.
    #[test]
    fn parse_class_data() {
        let raw = r#"
            [base_stats]
            strength = 15
            dexterity = 25
            vitality = 15
            energy = 25
            life = 45
            mana = 25
            stamina = 79
            life_per_vitality = 2.0
            mana_per_energy = 2.0
            life_per_level = 1.5
            mana_per_level = 2.0
            stamina_per_vitality = 1.0
        "#;

        let class: ClassData = toml::from_str(raw).expect("ClassData parse failed");
        assert_eq!(class.base_stats.strength, 15);
        assert_eq!(class.base_stats.energy, 25);
        assert_eq!(class.base_stats.stamina, 79);
        // Floating point values checked with epsilon tolerance
        assert!((class.base_stats.life_per_vitality - 2.0_f32).abs() < f32::EPSILON);
        assert!((class.base_stats.mana_per_level - 2.0_f32).abs() < f32::EPSILON);
        assert!((class.base_stats.life_per_level - 1.5_f32).abs() < f32::EPSILON);
    }

    /// Parses the Raise Skeleton skill from an inline TOML string.
    #[test]
    fn parse_skill_toml() {
        let raw = r#"
            [skill]
            id = "necro_raise_skeleton"
            name = "Raise Skeleton"
            tree = 0
            kind = "Summon"
            damage_type = "Physical"
            max_level = 20
            mana_cost_base = 6.0
            mana_cost_per_level = 0.0
            cooldown_ms = 500
            base_damage_min = 0
            base_damage_max = 0
            damage_per_level = 0
            prerequisites = []
        "#;

        let skill: SkillToml = toml::from_str(raw).expect("SkillToml parse failed");
        assert_eq!(skill.skill.id, "necro_raise_skeleton");
        assert_eq!(skill.skill.name, "Raise Skeleton");
        assert_eq!(skill.skill.tree, 0);
        assert_eq!(skill.skill.kind, "Summon");
        assert_eq!(skill.skill.max_level, 20);
        assert_eq!(skill.skill.cooldown_ms, 500);
        assert!(skill.skill.prerequisites.is_empty());
    }

    /// Parses the Fallen monster from an inline TOML string.
    #[test]
    fn parse_monster_toml() {
        let raw = r#"
            [monster]
            id = "fallen"
            name = "Fallen"
            level = 2
            base_hp = 15
            base_damage_min = 2
            base_damage_max = 4
            defense = 5
            attack_rating = 30
            experience = 12
        "#;

        let monster: MonsterToml = toml::from_str(raw).expect("MonsterToml parse failed");
        assert_eq!(monster.monster.id, "fallen");
        assert_eq!(monster.monster.level, 2);
        assert_eq!(monster.monster.base_hp, 15);
        assert_eq!(monster.monster.base_damage_min, 2);
        assert_eq!(monster.monster.base_damage_max, 4);
        assert_eq!(monster.monster.defense, 5);
        assert_eq!(monster.monster.experience, 12);
    }

    /// Validates that multiple skills loaded from inline TOML have unique IDs.
    #[test]
    fn validation_unique_ids() {
        let skills_raw = [
            r#"
                [skill]
                id = "necro_raise_skeleton"
                name = "Raise Skeleton"
                tree = 0
                kind = "Summon"
                damage_type = "Physical"
                max_level = 20
                mana_cost_base = 6.0
                mana_cost_per_level = 0.0
                cooldown_ms = 500
                base_damage_min = 0
                base_damage_max = 0
                damage_per_level = 0
                prerequisites = []
            "#,
            r#"
                [skill]
                id = "necro_raise_skeletal_mage"
                name = "Raise Skeletal Mage"
                tree = 0
                kind = "Summon"
                damage_type = "Fire"
                max_level = 20
                mana_cost_base = 9.0
                mana_cost_per_level = 0.5
                cooldown_ms = 500
                base_damage_min = 0
                base_damage_max = 0
                damage_per_level = 0
                prerequisites = ["necro_raise_skeleton"]
            "#,
            r#"
                [skill]
                id = "necro_clay_golem"
                name = "Clay Golem"
                tree = 0
                kind = "Summon"
                damage_type = "Physical"
                max_level = 20
                mana_cost_base = 20.0
                mana_cost_per_level = 1.0
                cooldown_ms = 1000
                base_damage_min = 0
                base_damage_max = 0
                damage_per_level = 0
                prerequisites = []
            "#,
            r#"
                [skill]
                id = "necro_teeth"
                name = "Teeth"
                tree = 1
                kind = "Projectile"
                damage_type = "Physical"
                max_level = 20
                mana_cost_base = 4.0
                mana_cost_per_level = 0.3
                cooldown_ms = 0
                base_damage_min = 2
                base_damage_max = 5
                damage_per_level = 2
                prerequisites = []
            "#,
            r#"
                [skill]
                id = "necro_bone_spear"
                name = "Bone Spear"
                tree = 1
                kind = "Projectile"
                damage_type = "Physical"
                max_level = 20
                mana_cost_base = 9.0
                mana_cost_per_level = 0.5
                cooldown_ms = 0
                base_damage_min = 5
                base_damage_max = 10
                damage_per_level = 3
                prerequisites = ["necro_teeth"]
            "#,
            r#"
                [skill]
                id = "necro_amplify_damage"
                name = "Amplify Damage"
                tree = 2
                kind = "Curse"
                damage_type = "Physical"
                max_level = 20
                mana_cost_base = 4.0
                mana_cost_per_level = 0.0
                cooldown_ms = 0
                base_damage_min = 0
                base_damage_max = 0
                damage_per_level = 0
                prerequisites = []
            "#,
        ];

        let parsed: Vec<SkillToml> = skills_raw
            .iter()
            .map(|raw| toml::from_str(raw).expect("SkillToml parse failed"))
            .collect();

        let ids: HashSet<&str> = parsed.iter().map(|s| s.skill.id.as_str()).collect();
        assert_eq!(
            ids.len(),
            parsed.len(),
            "Duplicate skill IDs detected: expected {} unique, got {}",
            parsed.len(),
            ids.len()
        );
    }
}
