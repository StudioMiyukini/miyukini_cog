//! Tests unitaires pour mge-asset.
//!
//! <!-- @id: sd-data-tests @do: test @role: arpg @layer: 3 @human: miyuk -->

use std::collections::HashMap;
use std::io::Write;

use crate::registry::GameDataRegistry;
use crate::types::*;
use crate::validation::validate_registry;
use crate::DataLoadError;

// =========================================================================
// Tests de deserialisation
// =========================================================================

/// Test deserialisation d'un item de base valide.
#[test]
fn test_deserialize_item_base() {
    let toml_str = r#"
[[weapons]]
id = "long_sword"
name = "Long Sword"
category = "Weapon"
weapon_type = "Sword"
attack_type = "Melee"
tier = "Normal"
damage_min = 3
damage_max = 16
speed = 0
range = 1.0
durability = 44
grid_size = [1, 3]
required_strength = 55
required_dexterity = 0
required_level = 1
quality_level = 20
max_sockets = 2
str_factor = 100
dex_factor = 0
can_be_ethereal = true
throwable = false
class_restriction = ""
"#;

    #[derive(Debug, serde::Deserialize)]
    struct ItemBaseFile {
        weapons: Vec<ItemBaseDef>,
    }

    let file: ItemBaseFile = toml::from_str(toml_str).unwrap();
    assert_eq!(file.weapons.len(), 1);
    assert_eq!(file.weapons[0].id, "long_sword");
    assert_eq!(file.weapons[0].damage_min, Some(3));
    assert_eq!(file.weapons[0].damage_max, Some(16));
    assert_eq!(file.weapons[0].max_sockets, 2);
    assert_eq!(file.weapons[0].grid_size, [1, 3]);
}

/// Test deserialisation d'un monstre valide.
#[test]
fn test_deserialize_monster() {
    let toml_str = r#"
[monster]
id = "fallen"
name = "Fallen"
type = "Demon"
ai_archetype = "MeleeSwarm"
base_sprite = "monsters/fallen"
grid_hitbox_radius = 0.4

[monster.stats.normal]
level = 2
life = 8
defense = 4
attack_rating = 15
damage_min = 1
damage_max = 3
experience = 20
fire_resist = 0
cold_resist = 0
lightning_resist = 0
poison_resist = 0
physical_resist = 0
immunities = []

[monster.stats.nightmare]
level = 36
life = 750
defense = 350
attack_rating = 800
damage_min = 8
damage_max = 16
experience = 3800
fire_resist = 33
cold_resist = 33
lightning_resist = 33
poison_resist = 33
physical_resist = 0
immunities = []

[monster.stats.hell]
level = 67
life = 4500
defense = 950
attack_rating = 2800
damage_min = 18
damage_max = 42
experience = 22000
fire_resist = 50
cold_resist = 50
lightning_resist = 50
poison_resist = 50
physical_resist = 10
immunities = []

[monster.ai_params]
aggro_radius = 10.0
leash_radius = 25.0
flee_hp_pct = 0.3
rally_on_flee = true
attack_cooldown = 15

[[monster.ai_params.skills]]
skill_id = "melee_attack"
priority = 1
condition = { type = "TargetInRange", range = 1.5 }
cooldown = 0

[monster.drops]
loot_table = "tc_act1_normal"
gold_min = 1
gold_max = 5
"#;

    let file: MonsterFile = toml::from_str(toml_str).unwrap();
    assert_eq!(file.monster.id, "fallen");
    assert_eq!(file.monster.monster_type_tag, "Demon");
    assert_eq!(file.monster.stats.normal.level, 2);
    assert_eq!(file.monster.stats.hell.life, 4500);
    assert_eq!(file.monster.ai_params.skills.len(), 1);
}

/// Test deserialisation d'un skill valide.
#[test]
fn test_deserialize_skill() {
    let toml_str = r#"
[skill]
id = "bone_spear"
name = "Bone Spear"
class = "Mortecian"
tree = "Bone"
tree_position = [2, 3]
required_level = 18
max_level = 20
skill_type = "Active"
target_type = "Direction"
icon_sprite = "ui/skills/necro_bone_spear"

[skill.base]
mana_cost = 7.0
cooldown_frames = 0
damage_type = "Magic"
projectile = true
projectile_speed = 12.0
projectile_sprite = "effects/bone_spear"
aoe_radius = 0.0
piercing = true
pierce_count = -1

[skill.per_level]
mana_cost_increment = 0.5
damage_min_base = 16
damage_min_per_level = 16
damage_max_base = 24
damage_max_per_level = 16

[[skill.synergies]]
skill_id = "teeth"
bonus_per_level = 7

[[skill.synergies]]
skill_id = "bone_wall"
bonus_per_level = 7

[skill.prerequisites]
skills_required = ["teeth", "corpse_explosion"]
"#;

    let file: SkillFile = toml::from_str(toml_str).unwrap();
    assert_eq!(file.skill.id, "bone_spear");
    assert_eq!(file.skill.skill_type, SkillType::Active);
    assert_eq!(file.skill.synergies.len(), 2);
    assert_eq!(file.skill.prerequisites.skills_required.len(), 2);
    assert!(file.skill.base.is_some());
    let base = file.skill.base.as_ref().unwrap();
    assert!(base.piercing);
    assert_eq!(base.pierce_count, -1);
}

/// Test deserialisation d'une zone valide.
#[test]
fn test_deserialize_zone() {
    let toml_str = r#"
[zone]
id = "act1/blood_moor"
name = "Blood Moor"
act = 1
is_town = false
has_waypoint = false
music_track = "music/act1_wilderness"
ambient_track = "ambient/wilderness_day"
tileset = "tiles/act1_wilderness"
map_file = "maps/act1/blood_moor.ldtk"

[zone.area_levels]
normal = 1
nightmare = 36
hell = 67

[zone.connections]
east = "act1/rogue_encampment"
west = "act1/cold_plains"
cave = "act1/den_of_evil"

[[zone.spawn_groups]]
monster_id = "fallen"
min_count = 4
max_count = 8
area_center = [20.0, 15.0]
area_radius = 8.0

[zone.champion_density]
packs_per_area = 2
min_pack_size = 3
max_pack_size = 5
affixes_count = 1

[zone.unique_density]
packs_per_area = 1
minions_min = 3
minions_max = 5
affixes_count_min = 1
affixes_count_max = 3
"#;

    let file: ZoneFile = toml::from_str(toml_str).unwrap();
    assert_eq!(file.zone.id, "act1/blood_moor");
    assert_eq!(file.zone.act, 1);
    assert!(!file.zone.is_town);
    assert_eq!(file.zone.spawn_groups.len(), 1);
    assert_eq!(file.zone.connections.east, Some("act1/rogue_encampment".to_string()));
    assert_eq!(file.zone.connections.cave, Some("act1/den_of_evil".to_string()));
}

/// Test deserialisation d'une quete valide.
#[test]
fn test_deserialize_quest() {
    let toml_str = r#"
[quest]
id = "den_of_evil"
name = "Den of Evil"
act = 1
quest_giver = "akara"
description = "Clear the Den of Evil of all monsters"

[quest.trigger]
type = "NpcTalked"
npc_id = "akara"

[quest.objective]
type = "ClearZone"
zone_id = "act1/den_of_evil"

[quest.completion_trigger]
type = "AllMonstersKilled"
zone_id = "act1/den_of_evil"

[[quest.rewards]]
type = "SkillPoint"
value = 1

[[quest.rewards]]
type = "SkillReset"
value = 1
description = "Free skill/stat reset from Akara"

[quest.dialogue]
accept_script = "scripts/quests/act1/den_of_evil_accept.rhai"
progress_script = "scripts/quests/act1/den_of_evil_progress.rhai"
complete_script = "scripts/quests/act1/den_of_evil_complete.rhai"
"#;

    let file: QuestFile = toml::from_str(toml_str).unwrap();
    assert_eq!(file.quest.id, "den_of_evil");
    assert_eq!(file.quest.rewards.len(), 2);
    assert_eq!(file.quest.rewards[0].reward_type, "SkillPoint");
}

/// Test deserialisation d'un runeword valide.
#[test]
fn test_deserialize_runeword() {
    let toml_str = r#"
[[runewords]]
id = "spirit"
name = "Spirit"
rune_sequence = ["Tal", "Thul", "Ort", "Amn"]
allowed_types = ["Sword", "Shield"]
required_level = 25
properties = [
    { stat = "all_skills", value = 2 },
    { stat = "faster_cast_rate", min = 25, max = 35 },
    { stat = "mana", min = 89, max = 112 },
]
"#;

    let file: RunewordsFile = toml::from_str(toml_str).unwrap();
    assert_eq!(file.runewords.len(), 1);
    assert_eq!(file.runewords[0].id, "spirit");
    assert_eq!(file.runewords[0].rune_sequence.len(), 4);
    assert_eq!(file.runewords[0].properties.len(), 3);
}

/// Test deserialisation de gemmes.
#[test]
fn test_deserialize_gem() {
    let toml_str = r#"
[[gems]]
id = "perfect_ruby"
name = "Perfect Ruby"
gem_type = "Ruby"
quality = "Perfect"
required_level = 18
grid_size = [1, 1]
in_weapon = [
    { stat = "fire_damage_min", value = 15 },
    { stat = "fire_damage_max", value = 20 },
]
in_armor = [
    { stat = "life", value = 38 },
]
in_helm = [
    { stat = "life", value = 38 },
]
in_shield = [
    { stat = "fire_resist", value = 40 },
]
"#;

    let file: GemsFile = toml::from_str(toml_str).unwrap();
    assert_eq!(file.gems.len(), 1);
    assert_eq!(file.gems[0].id, "perfect_ruby");
    assert_eq!(file.gems[0].quality, GemQuality::Perfect);
    assert_eq!(file.gems[0].in_weapon.len(), 2);
}

/// Test deserialisation d'une rune.
#[test]
fn test_deserialize_rune() {
    let toml_str = r#"
[[runes]]
id = "ber"
name = "Ber"
number = 30
required_level = 63
grid_size = [1, 1]
in_weapon = [
    { stat = "crushing_blow", value = 20 },
]
in_armor = [
    { stat = "damage_reduction", value = 8 },
]
in_shield = [
    { stat = "damage_reduction", value = 8 },
]
in_helm = [
    { stat = "damage_reduction", value = 8 },
]
upgrade_recipe_to = "jah"
upgrade_count = 2
"#;

    let file: RunesFile = toml::from_str(toml_str).unwrap();
    assert_eq!(file.runes.len(), 1);
    assert_eq!(file.runes[0].id, "ber");
    assert_eq!(file.runes[0].number, 30);
    assert_eq!(file.runes[0].upgrade_recipe_to, "jah");
}

/// Test deserialisation d'une classe.
#[test]
fn test_deserialize_class() {
    let toml_str = r#"
[class]
id = "mortecian"
name = "Mortecian"
d2_name = "Necromancer"

[class.base_stats]
strength = 15
dexterity = 25
vitality = 15
energy = 25
life = 45
mana = 25
stamina = 79

[class.per_level]
life_per_level = 1.5
mana_per_level = 2.0
stamina_per_level = 1.0
life_per_vitality = 2
mana_per_energy = 2.0
stamina_per_vitality = 1

[class.combat]
class_base_ar = 7
base_block_frames = 11

[class.skill_trees]
trees = ["Summoning", "PoisonBone", "Curses"]
skills_per_tree = 10
total_skills = 30

[class.sprite]
base_sprite = "characters/necromancer"
attack_sprite = "characters/necromancer_attack"
cast_sprite = "characters/necromancer_cast"
"#;

    let file: ClassFile = toml::from_str(toml_str).unwrap();
    assert_eq!(file.class.id, "mortecian");
    assert_eq!(file.class.base_stats.strength, 15);
    assert_eq!(file.class.base_stats.energy, 25);
    assert_eq!(file.class.skill_trees.trees.len(), 3);
}

/// Test deserialisation de difficulte.
#[test]
fn test_deserialize_difficulty() {
    let toml_str = r#"
[normal]
fire_resist_penalty = 0
cold_resist_penalty = 0
lightning_resist_penalty = 0
poison_resist_penalty = 0
experience_multiplier = 1.0
gold_multiplier = 1.0

[nightmare]
fire_resist_penalty = -40
cold_resist_penalty = -40
lightning_resist_penalty = -40
poison_resist_penalty = -40
experience_multiplier = 1.0
gold_multiplier = 4.0

[hell]
fire_resist_penalty = -100
cold_resist_penalty = -100
lightning_resist_penalty = -100
poison_resist_penalty = -100
experience_multiplier = 1.0
gold_multiplier = 8.0
"#;

    let config: DifficultyConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(config.normal.fire_resist_penalty, 0);
    assert_eq!(config.nightmare.fire_resist_penalty, -40);
    assert_eq!(config.hell.fire_resist_penalty, -100);
    assert_eq!(config.hell.gold_multiplier, 8.0);
}

// =========================================================================
// Tests de rejet TOML invalide
// =========================================================================

/// Test rejet d'un champ inconnu dans une zone (deny_unknown_fields desactive
/// car nous n'utilisons pas ce derive partout pour rester flexible).
#[test]
fn test_reject_missing_required_field_zone() {
    let toml_str = r#"
[zone]
id = "test_zone"
name = "Test Zone"
"#;
    // Manque act, area_levels, connections, etc.
    let result = toml::from_str::<ZoneFile>(toml_str);
    assert!(result.is_err(), "Should reject missing required fields");
}

/// Test rejet d'un skill avec champs manquants.
#[test]
fn test_reject_missing_required_field_skill() {
    let toml_str = r#"
[skill]
id = "test_skill"
name = "Test"
"#;
    let result = toml::from_str::<SkillFile>(toml_str);
    assert!(result.is_err(), "Should reject missing required fields");
}

// =========================================================================
// Tests de validation cross-reference
// =========================================================================

/// Test que la validation detecte une synergie vers un skill inexistant.
#[test]
fn test_validation_detects_broken_synergy() {
    let mut registry = GameDataRegistry::empty();

    let mut skills = HashMap::new();
    skills.insert(
        "bone_spear".to_string(),
        SkillDef {
            id: "bone_spear".to_string(),
            name: "Bone Spear".to_string(),
            class: "Mortecian".to_string(),
            tree: "Bone".to_string(),
            tree_position: [2, 3],
            max_level: 20,
            required_level: 18,
            skill_type: SkillType::Active,
            target_type: SkillTargetType::Direction,
            icon_sprite: "test".to_string(),
            base: None,
            per_level: SkillPerLevel {
                mana_cost_increment: None,
                damage_min_base: Some(16),
                damage_min_per_level: Some(16),
                damage_max_base: Some(24),
                damage_max_per_level: Some(16),
                enhanced_damage_base: None,
                enhanced_damage_per_level: None,
                attack_speed_base: None,
                attack_speed_per_level: None,
                attack_rating_pct_base: None,
                attack_rating_pct_per_level: None,
                bonus_life_per_level: None,
                bonus_damage_per_level: None,
                bonus_ar_per_level: None,
                bonus_defense_per_level: None,
            },
            synergies: vec![SkillSynergy {
                skill_id: "nonexistent_skill".to_string(),
                bonus_per_level: 7,
            }],
            prerequisites: SkillPrereqs {
                skills_required: vec!["also_nonexistent".to_string()],
            },
            aura: None,
        },
    );
    registry.skills = skills;

    let result = validate_registry(&registry);
    assert!(result.is_err());

    if let Err(DataLoadError::ValidationErrors { errors }) = result {
        assert!(
            errors.iter().any(|e| e.contains("nonexistent_skill")),
            "Should mention the nonexistent synergy skill"
        );
        assert!(
            errors.iter().any(|e| e.contains("also_nonexistent")),
            "Should mention the nonexistent prerequisite skill"
        );
    } else {
        panic!("Expected ValidationErrors");
    }
}

/// Test que la validation passe pour un registre vide.
#[test]
fn test_validation_passes_empty_registry() {
    let registry = GameDataRegistry::empty();
    let result = validate_registry(&registry);
    assert!(result.is_ok());
}

/// Test que la validation detecte un monstre avec loot table inexistante.
#[test]
fn test_validation_detects_broken_monster_loot() {
    let mut registry = GameDataRegistry::empty();

    let mut monsters = HashMap::new();
    monsters.insert(
        "test_monster".to_string(),
        MonsterDef {
            id: "test_monster".to_string(),
            name: "Test Monster".to_string(),
            monster_type_tag: "Demon".to_string(),
            ai_archetype: "MeleeSwarm".to_string(),
            base_sprite: "test".to_string(),
            grid_hitbox_radius: 0.4,
            stats: MonsterStatsByDifficulty {
                normal: MonsterStatsBlock {
                    level: 1,
                    life: 10,
                    defense: 5,
                    attack_rating: 10,
                    damage_min: 1,
                    damage_max: 3,
                    experience: 10,
                    fire_resist: 0,
                    cold_resist: 0,
                    lightning_resist: 0,
                    poison_resist: 0,
                    physical_resist: 0,
                    immunities: Vec::new(),
                },
                nightmare: MonsterStatsBlock {
                    level: 36,
                    life: 500,
                    defense: 200,
                    attack_rating: 500,
                    damage_min: 5,
                    damage_max: 10,
                    experience: 2000,
                    fire_resist: 33,
                    cold_resist: 33,
                    lightning_resist: 33,
                    poison_resist: 33,
                    physical_resist: 0,
                    immunities: Vec::new(),
                },
                hell: MonsterStatsBlock {
                    level: 67,
                    life: 3000,
                    defense: 800,
                    attack_rating: 2000,
                    damage_min: 15,
                    damage_max: 30,
                    experience: 15000,
                    fire_resist: 50,
                    cold_resist: 50,
                    lightning_resist: 50,
                    poison_resist: 50,
                    physical_resist: 10,
                    immunities: Vec::new(),
                },
            },
            ai_params: MonsterAiParams {
                aggro_radius: 10.0,
                leash_radius: 25.0,
                flee_hp_pct: None,
                rally_on_flee: false,
                attack_cooldown: 15,
                skills: Vec::new(),
            },
            drops: MonsterDrops {
                loot_table: "nonexistent_tc".to_string(),
                gold_min: 1,
                gold_max: 5,
            },
        },
    );
    registry.monsters = monsters;

    let result = validate_registry(&registry);
    assert!(result.is_err());

    if let Err(DataLoadError::ValidationErrors { errors }) = result {
        assert!(errors.iter().any(|e| e.contains("nonexistent_tc")));
    }
}

/// Test que la validation detecte une zone avec monstre inexistant.
#[test]
fn test_validation_detects_broken_zone_spawn() {
    let mut registry = GameDataRegistry::empty();

    let mut zones = HashMap::new();
    zones.insert(
        "test_zone".to_string(),
        ZoneDef {
            id: "test_zone".to_string(),
            name: "Test Zone".to_string(),
            act: 1,
            is_town: false,
            has_waypoint: false,
            music_track: "test".to_string(),
            ambient_track: "test".to_string(),
            tileset: "test".to_string(),
            map_file: "test.ldtk".to_string(),
            area_levels: AreaLevels {
                normal: 1,
                nightmare: 36,
                hell: 67,
            },
            connections: ZoneConnections {
                north: None,
                south: None,
                east: Some("nonexistent_zone".to_string()),
                west: None,
                cave: None,
                dungeon: None,
            },
            spawn_groups: vec![SpawnGroup {
                monster_id: "nonexistent_monster".to_string(),
                min_count: 1,
                max_count: 3,
                area_center: [10.0, 10.0],
                area_radius: 5.0,
            }],
            super_uniques: Vec::new(),
            champion_density: ChampionDensity {
                packs_per_area: 1,
                min_pack_size: 2,
                max_pack_size: 4,
                affixes_count: 1,
            },
            unique_density: UniqueDensity {
                packs_per_area: 1,
                minions_min: 2,
                minions_max: 4,
                affixes_count_min: 1,
                affixes_count_max: 2,
            },
        },
    );
    registry.zones = zones;

    let result = validate_registry(&registry);
    assert!(result.is_err());

    if let Err(DataLoadError::ValidationErrors { errors }) = result {
        assert!(errors.iter().any(|e| e.contains("nonexistent_monster")));
        assert!(errors.iter().any(|e| e.contains("nonexistent_zone")));
    }
}

// =========================================================================
// Tests du loader generique
// =========================================================================

/// Test load_toml_file avec un fichier valide.
#[test]
fn test_load_toml_file_valid() {
    let dir = std::env::temp_dir().join("mge_asset_test_load_valid");
    let _ = std::fs::create_dir_all(&dir);
    let file_path = dir.join("test_class.toml");

    let toml_content = r#"
[class]
id = "test_warrior"
name = "Test Warrior"
d2_name = "Warrior"

[class.base_stats]
strength = 30
dexterity = 20
vitality = 25
energy = 10
life = 55
mana = 10
stamina = 92

[class.per_level]
life_per_level = 2.0
mana_per_level = 1.0
stamina_per_level = 1.0
life_per_vitality = 4
mana_per_energy = 1.0
stamina_per_vitality = 1

[class.combat]
class_base_ar = 10
base_block_frames = 6

[class.skill_trees]
trees = ["Combat", "Defense"]
skills_per_tree = 10
total_skills = 20

[class.sprite]
base_sprite = "characters/warrior"
attack_sprite = "characters/warrior_attack"
cast_sprite = "characters/warrior_cast"
"#;

    let mut f = std::fs::File::create(&file_path).unwrap();
    f.write_all(toml_content.as_bytes()).unwrap();

    let result: Result<ClassFile, _> = crate::loader::load_toml_file(&file_path);
    assert!(result.is_ok());
    let class_file = result.unwrap();
    assert_eq!(class_file.class.id, "test_warrior");
    assert_eq!(class_file.class.base_stats.strength, 30);

    // Nettoyage.
    let _ = std::fs::remove_dir_all(&dir);
}

/// Test load_toml_file avec un fichier inexistant.
#[test]
fn test_load_toml_file_not_found() {
    let result: Result<ClassFile, _> =
        crate::loader::load_toml_file(std::path::Path::new("/nonexistent/path.toml"));
    assert!(result.is_err());
    if let Err(DataLoadError::IoError { path, .. }) = result {
        assert!(path.contains("nonexistent"));
    }
}

/// Test load_toml_file avec du TOML invalide.
#[test]
fn test_load_toml_file_invalid_toml() {
    let dir = std::env::temp_dir().join("mge_asset_test_invalid_toml");
    let _ = std::fs::create_dir_all(&dir);
    let file_path = dir.join("broken.toml");

    let mut f = std::fs::File::create(&file_path).unwrap();
    f.write_all(b"this is not valid [[ toml {{{}}}").unwrap();

    let result: Result<ClassFile, _> = crate::loader::load_toml_file(&file_path);
    assert!(result.is_err());
    if let Err(DataLoadError::ParseError { .. }) = result {
        // OK
    } else {
        panic!("Expected ParseError");
    }

    let _ = std::fs::remove_dir_all(&dir);
}

/// Test load_toml_directory avec un repertoire valide.
#[test]
fn test_load_toml_directory_valid() {
    let dir = std::env::temp_dir().join("mge_asset_test_dir_valid");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    // Creer deux fichiers de classes.
    for (id, name) in &[("warrior", "Warrior"), ("mage", "Mage")] {
        let content = format!(
            r#"
[class]
id = "{id}"
name = "{name}"
d2_name = "{name}"

[class.base_stats]
strength = 20
dexterity = 20
vitality = 20
energy = 20
life = 40
mana = 20
stamina = 80

[class.per_level]
life_per_level = 1.5
mana_per_level = 1.5
stamina_per_level = 1.0
life_per_vitality = 3
mana_per_energy = 1.5
stamina_per_vitality = 1

[class.combat]
class_base_ar = 8
base_block_frames = 9

[class.skill_trees]
trees = ["Tree1"]
skills_per_tree = 10
total_skills = 10

[class.sprite]
base_sprite = "test"
attack_sprite = "test"
cast_sprite = "test"
"#
        );
        let file_path = dir.join(format!("{id}.toml"));
        let mut f = std::fs::File::create(&file_path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    let result = crate::loader::load_toml_directory(&dir, |file: ClassFile| {
        vec![(file.class.id.clone(), file.class)]
    });

    assert!(result.is_ok());
    let map = result.unwrap();
    assert_eq!(map.len(), 2);
    assert!(map.contains_key("warrior"));
    assert!(map.contains_key("mage"));

    let _ = std::fs::remove_dir_all(&dir);
}

/// Test load_toml_directory avec repertoire inexistant.
#[test]
fn test_load_toml_directory_not_found() {
    let result = crate::loader::load_toml_directory(
        std::path::Path::new("/nonexistent/dir"),
        |file: ClassFile| vec![(file.class.id.clone(), file.class)],
    );
    assert!(result.is_err());
    if let Err(DataLoadError::DirectoryNotFound { .. }) = result {
        // OK
    } else {
        panic!("Expected DirectoryNotFound");
    }
}
