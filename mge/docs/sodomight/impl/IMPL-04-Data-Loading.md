<!-- @id: SD-Impl-04 @do: guide @role: back-end @layer: 3 @human: miyuk -->

# IMPL-04 -- Guide de Chargement des Donnees TOML

**Auteur :** Francois (Dev Back-End, Miyukini AI Studio)
**Base :** SD-Tech-Data-Schemas.md (Denis)
**Date :** 2026-02-28
**Statut :** Guide d'implementation -- v1.0

---

## Table des matieres

1. [Architecture du chargeur de donnees](#1-architecture-du-chargeur-de-donnees)
2. [AssetRegistry central](#2-assetregistry-central)
3. [Structs Rust serde par domaine](#3-structs-rust-serde-par-domaine)
4. [Hot-reload avec notify](#4-hot-reload-avec-notify)
5. [Exemples de fichiers TOML complets](#5-exemples-de-fichiers-toml-complets)
6. [Validation cross-reference](#6-validation-cross-reference)
7. [Erreurs explicites](#7-erreurs-explicites)
8. [Tests](#8-tests)

---

## 1. Architecture du chargeur de donnees

### 1.1 Principes

Toutes les donnees de jeu de Sodomight sont definies dans des fichiers TOML.
Le code Rust ne contient aucune valeur de gameplay hardcodee. Le chargement
se fait en deux phases :

1. **Blocking au demarrage** : toutes les definitions (classes, skills, items,
   monstres, zones, quetes, loot tables, config) sont chargees et validees
   avant le lancement de la partie.

2. **Hot-reload en developpement** : un watcher `notify` surveille le repertoire
   `data/` et recharge automatiquement les fichiers modifies. Active uniquement
   avec le feature flag `dev-hotreload`.

### 1.2 Pipeline de chargement

```
Fichier TOML sur disque
    |
    v
std::fs::read_to_string()
    |
    v
toml::from_str::<T>() avec #[serde(deny_unknown_fields)]
    |
    v
Validation metier (ranges, references croisees)
    |
    v
Insertion dans GameDataRegistry (resource ECS)
    |
    v
world.insert_resource(registry)
```

---

## 2. AssetRegistry central

### 2.1 GameDataRegistry

```rust
/// @id: sd-data-registry @do: define @role: arpg @layer: 3
/// Crate: sodomight-game
///
/// Registre central de toutes les definitions de jeu chargees depuis TOML.
/// Insere comme resource ECS globale.
#[derive(Debug)]
pub struct GameDataRegistry {
    pub classes: std::collections::HashMap<String, ClassDef>,
    pub skills: std::collections::HashMap<String, SkillDef>,
    pub item_bases: std::collections::HashMap<String, ItemBaseDef>,
    pub prefixes: Vec<AffixDef>,
    pub suffixes: Vec<AffixDef>,
    pub unique_items: std::collections::HashMap<String, UniqueItemDef>,
    pub set_items: std::collections::HashMap<String, SetDef>,
    pub runewords: Vec<RunewordDef>,
    pub runes: std::collections::HashMap<String, RuneDef>,
    pub gems: std::collections::HashMap<String, GemDef>,
    pub cube_recipes: Vec<CubeRecipeDef>,
    pub monsters: std::collections::HashMap<String, MonsterDef>,
    pub champion_affixes: Vec<ChampionAffixDef>,
    pub super_uniques: std::collections::HashMap<String, SuperUniqueDef>,
    pub zones: std::collections::HashMap<String, ZoneDef>,
    pub treasure_classes: std::collections::HashMap<String, TreasureClassDef>,
    pub quests: std::collections::HashMap<String, QuestDef>,
    pub breakpoint_tables: BreakpointTables,
    pub experience_table: Vec<u64>,
    pub difficulty_config: DifficultyConfig,
    pub shrine_defs: Vec<ShrineDef>,
}

impl GameDataRegistry {
    /// Charge toutes les donnees depuis le repertoire racine.
    pub fn load_all(data_root: &std::path::Path) -> Result<Self, DataLoadError> {
        let classes = load_classes(&data_root.join("classes"))?;
        let skills = load_skills(&data_root.join("skills"))?;
        let item_bases = load_item_bases(&data_root.join("items/bases"))?;
        let (prefixes, suffixes) = load_affixes(&data_root.join("items/affixes"))?;
        let unique_items = load_uniques(&data_root.join("items/uniques"))?;
        let set_items = load_sets(&data_root.join("items/sets"))?;
        let runewords = load_runewords(&data_root.join("items/runewords.toml"))?;
        let runes = load_runes(&data_root.join("items/runes.toml"))?;
        let gems = load_gems(&data_root.join("items/gems.toml"))?;
        let cube_recipes = load_cube_recipes(&data_root.join("items/cube_recipes.toml"))?;
        let monsters = load_monsters(&data_root.join("monsters"))?;
        let champion_affixes =
            load_champion_affixes(&data_root.join("monsters/affixes/champion_affixes.toml"))?;
        let super_uniques = load_super_uniques(&data_root.join("monsters/super_uniques"))?;
        let zones = load_zones(&data_root.join("zones"))?;
        let treasure_classes =
            load_treasure_classes(&data_root.join("loot_tables/treasure_classes.toml"))?;
        let quests = load_quests(&data_root.join("quests"))?;
        let breakpoint_tables =
            load_breakpoints(&data_root.join("config/breakpoints.toml"))?;
        let experience_table =
            load_experience_table(&data_root.join("config/experience.toml"))?;
        let difficulty_config =
            load_difficulty(&data_root.join("config/difficulty.toml"))?;
        let shrine_defs = load_shrines(&data_root.join("config/shrines.toml"))?;

        let registry = Self {
            classes,
            skills,
            item_bases,
            prefixes,
            suffixes,
            unique_items,
            set_items,
            runewords,
            runes,
            gems,
            cube_recipes,
            monsters,
            champion_affixes,
            super_uniques,
            zones,
            treasure_classes,
            quests,
            breakpoint_tables,
            experience_table,
            difficulty_config,
            shrine_defs,
        };

        // Validation cross-references.
        validate_registry(&registry)?;

        Ok(registry)
    }
}
```

### 2.2 Fonction de chargement generique

```rust
/// @id: sd-data-load-generic @do: define @role: arpg @layer: 3
///
/// Charge et deserialise un fichier TOML en struct typee.
pub fn load_toml_file<T: serde::de::DeserializeOwned>(
    path: &std::path::Path,
) -> Result<T, DataLoadError> {
    let content = std::fs::read_to_string(path).map_err(|e| DataLoadError::IoError {
        path: path.display().to_string(),
        source: e.to_string(),
    })?;

    toml::from_str::<T>(&content).map_err(|e| DataLoadError::ParseError {
        path: path.display().to_string(),
        source: e.to_string(),
    })
}

/// Charge tous les fichiers TOML d'un repertoire, deserialise chacun,
/// et les regroupe dans un HashMap indexe par l'ID de chaque entite.
pub fn load_toml_directory<T, F>(
    dir: &std::path::Path,
    extract_entries: F,
) -> Result<std::collections::HashMap<String, T>, DataLoadError>
where
    T: serde::de::DeserializeOwned + Clone + std::fmt::Debug,
    F: Fn(T) -> Vec<(String, T)>,
{
    let mut result = std::collections::HashMap::new();

    if !dir.exists() {
        return Err(DataLoadError::DirectoryNotFound {
            path: dir.display().to_string(),
        });
    }

    for entry in std::fs::read_dir(dir).map_err(|e| DataLoadError::IoError {
        path: dir.display().to_string(),
        source: e.to_string(),
    })? {
        let entry = entry.map_err(|e| DataLoadError::IoError {
            path: dir.display().to_string(),
            source: e.to_string(),
        })?;

        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }

        let parsed: T = load_toml_file(&path)?;
        let entries = extract_entries(parsed);
        for (id, value) in entries {
            if result.contains_key(&id) {
                return Err(DataLoadError::DuplicateId {
                    id,
                    path: path.display().to_string(),
                });
            }
            result.insert(id, value);
        }
    }

    Ok(result)
}
```

---

## 3. Structs Rust serde par domaine

### 3.1 Classes

```rust
/// @id: sd-data-class-def @do: define @role: arpg @layer: 3
/// Fichier : data/classes/{classe}.toml

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClassDef {
    pub id: String,
    pub name: String,
    pub base_strength: i32,
    pub base_dexterity: i32,
    pub base_vitality: i32,
    pub base_energy: i32,
    pub base_life: i32,
    pub base_mana: i32,
    pub base_stamina: i32,
    pub life_per_vitality: f32,
    pub mana_per_energy: f32,
    pub stamina_per_vitality: f32,
    pub life_per_level: f32,
    pub mana_per_level: f32,
    pub stamina_per_level: f32,
    pub class_base_ar: i32,
    pub block_bonus: i32,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub skill_trees: Vec<SkillTreeDef>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkillTreeDef {
    pub id: String,
    pub name: String,
    pub skills: Vec<String>,
}
```

### 3.2 Monstres

```rust
/// @id: sd-data-monster-def @do: define @role: arpg @layer: 3
/// Fichier : data/monsters/act{N}/{monstre}.toml

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MonsterFileDef {
    pub monsters: Vec<MonsterDef>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MonsterDef {
    pub id: String,
    pub name: String,
    pub act: u8,
    pub zone_ids: Vec<String>,
    pub monster_type: String,
    pub ai_archetype: String,
    pub level_normal: u8,
    pub level_nightmare: u8,
    pub level_hell: u8,
    pub hp_normal: i32,
    pub hp_nightmare: i32,
    pub hp_hell: i32,
    pub damage_min: i32,
    pub damage_max: i32,
    pub defense: i32,
    pub attack_rating: i32,
    pub fire_resist_normal: i32,
    pub fire_resist_nightmare: i32,
    pub fire_resist_hell: i32,
    pub cold_resist_normal: i32,
    pub cold_resist_nightmare: i32,
    pub cold_resist_hell: i32,
    pub lightning_resist_normal: i32,
    pub lightning_resist_nightmare: i32,
    pub lightning_resist_hell: i32,
    pub poison_resist_normal: i32,
    pub poison_resist_nightmare: i32,
    pub poison_resist_hell: i32,
    pub physical_resist: i32,
    pub magic_resist: i32,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub attack_speed_frames: u32,
    pub aggro_range: f32,
    pub leash_range: f32,
    pub experience: u64,
    pub treasure_class: String,
    pub drop_table_id: String,
    pub can_be_champion: bool,
    pub hitbox_radius: f32,
    pub sprite_id: String,
}
```

### 3.3 Zones

```rust
/// @id: sd-data-zone-def @do: define @role: arpg @layer: 3
/// Fichier : data/zones/act{N}/{zone}.toml

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ZoneDef {
    pub id: String,
    pub name: String,
    pub act: u8,
    pub area_level_normal: u8,
    pub area_level_nightmare: u8,
    pub area_level_hell: u8,
    pub is_town: bool,
    pub has_waypoint: bool,
    pub waypoint_position: Option<[f32; 2]>,
    pub connections: Vec<ZoneConnectionDef>,
    pub monster_spawns: Vec<MonsterSpawnDef>,
    pub map_file: String,
    pub music_id: String,
    pub ambient_id: String,
    pub lighting: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ZoneConnectionDef {
    pub target_zone_id: String,
    pub portal_type: String,
    pub position: [f32; 2],
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MonsterSpawnDef {
    pub monster_id: String,
    pub count_min: u32,
    pub count_max: u32,
    pub spawn_area: [f32; 4],
    pub respawn_delay_frames: u32,
}
```

### 3.4 Quetes

```rust
/// @id: sd-data-quest-def @do: define @role: arpg @layer: 3
/// Fichier : data/quests/act{N}/{quete}.toml

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestDef {
    pub id: String,
    pub name: String,
    pub act: u8,
    pub quest_giver_npc: String,
    pub objectives: Vec<QuestObjectiveDef>,
    pub rewards: Vec<QuestRewardDef>,
    pub prerequisite_quests: Vec<String>,
    pub script_file: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestObjectiveDef {
    pub objective_type: String,
    pub target_id: String,
    pub count: u32,
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestRewardDef {
    pub reward_type: String,
    pub value: i32,
    pub item_id: Option<String>,
}
```

### 3.5 Runes et Gemmes

```rust
/// @id: sd-data-rune-def @do: define @role: arpg @layer: 3

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuneFileDef {
    pub runes: Vec<RuneDef>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuneDef {
    pub id: String,
    pub name: String,
    pub rune_number: u8,
    pub required_level: u8,
    pub in_weapon: Vec<StatValue>,
    pub in_armor: Vec<StatValue>,
    pub in_helm: Vec<StatValue>,
    pub in_shield: Vec<StatValue>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GemFileDef {
    pub gems: Vec<GemDef>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GemDef {
    pub id: String,
    pub name: String,
    pub gem_type: String,
    pub quality: String,
    pub required_level: u8,
    pub grid_size: [u8; 2],
    pub in_weapon: Vec<StatValue>,
    pub in_armor: Vec<StatValue>,
    pub in_helm: Vec<StatValue>,
    pub in_shield: Vec<StatValue>,
}
```

### 3.6 Treasure Classes et Loot Tables

```rust
/// @id: sd-data-tc-def @do: define @role: arpg @layer: 3

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TreasureClassFileDef {
    pub treasure_classes: Vec<TreasureClassDef>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TreasureClassDef {
    pub id: String,
    pub no_drop: u32,
    pub picks: u8,
    pub entries: Vec<TreasureClassEntryDef>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TreasureClassEntryDef {
    pub item_or_tc: String,
    pub probability: u32,
}
```

### 3.7 Config globale

```rust
/// @id: sd-data-config @do: define @role: arpg @layer: 3

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DifficultyConfig {
    pub normal: DifficultyLevel,
    pub nightmare: DifficultyLevel,
    pub hell: DifficultyLevel,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DifficultyLevel {
    pub fire_resist_penalty: i32,
    pub cold_resist_penalty: i32,
    pub lightning_resist_penalty: i32,
    pub poison_resist_penalty: i32,
    pub experience_multiplier: f32,
    pub gold_multiplier: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BreakpointTables {
    pub fcr: std::collections::HashMap<String, Vec<BreakpointEntry>>,
    pub fhr: std::collections::HashMap<String, Vec<BreakpointEntry>>,
    pub fbr: std::collections::HashMap<String, Vec<BreakpointEntry>>,
    pub ias: std::collections::HashMap<String, Vec<BreakpointEntry>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BreakpointEntry {
    pub threshold: i32,
    pub frames: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShrineDef {
    pub id: String,
    pub name: String,
    pub effect: String,
    pub duration_seconds: f32,
    pub value: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CubeRecipeDef {
    pub id: String,
    pub inputs: Vec<CubeInputDef>,
    pub output: CubeOutputDef,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CubeInputDef {
    pub item_type: String,
    pub quality: Option<String>,
    pub count: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CubeOutputDef {
    pub item_type: String,
    pub quality: Option<String>,
    pub transform: String,
}
```

---

## 4. Hot-reload avec notify

```rust
/// @id: sd-data-hotreload @do: define @role: arpg @layer: 3
/// Crate: mge-asset
///
/// Watcher de fichiers pour recharger les TOML modifies en developpement.
/// Active uniquement avec le feature flag `dev-hotreload`.

use std::sync::mpsc;

/// Evenement de modification de fichier.
#[derive(Debug)]
pub struct FileChangedEvent {
    pub path: std::path::PathBuf,
    pub kind: FileChangeKind,
}

#[derive(Debug)]
pub enum FileChangeKind {
    Modified,
    Created,
    Deleted,
}

/// Demarre le watcher sur un repertoire.
/// Les evenements sont envoyes via un channel mpsc.
pub fn start_file_watcher(
    watch_dir: &std::path::Path,
) -> Result<(notify::RecommendedWatcher, mpsc::Receiver<FileChangedEvent>), DataLoadError> {
    use notify::{Event, EventKind, RecursiveMode, Watcher};

    let (tx, rx) = mpsc::channel();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            for path in event.paths {
                let kind = match event.kind {
                    EventKind::Modify(_) => FileChangeKind::Modified,
                    EventKind::Create(_) => FileChangeKind::Created,
                    EventKind::Remove(_) => FileChangeKind::Deleted,
                    _ => continue,
                };
                let _ = tx.send(FileChangedEvent { path, kind });
            }
        }
    })
    .map_err(|e| DataLoadError::WatcherError {
        source: e.to_string(),
    })?;

    watcher
        .watch(watch_dir, RecursiveMode::Recursive)
        .map_err(|e| DataLoadError::WatcherError {
            source: e.to_string(),
        })?;

    Ok((watcher, rx))
}

/// Systeme PreUpdate qui poll les changements de fichiers et recharge les donnees.
pub fn hot_reload_system(
    rx: &mpsc::Receiver<FileChangedEvent>,
    registry: &mut GameDataRegistry,
    data_root: &std::path::Path,
) {
    while let Ok(event) = rx.try_recv() {
        if let Some(ext) = event.path.extension() {
            if ext == "toml" {
                tracing::info!("Hot-reload: {:?} {:?}", event.kind, event.path);
                // Recharger le fichier specifique.
                // En pratique, on recharge la categorie entiere pour simplifier.
                if let Some(category) = detect_category(&event.path, data_root) {
                    let _ = reload_category(registry, &category, data_root);
                }
            }
        }
    }
}

fn detect_category(path: &std::path::Path, data_root: &std::path::Path) -> Option<String> {
    let relative = path.strip_prefix(data_root).ok()?;
    let first_component = relative.components().next()?;
    Some(first_component.as_os_str().to_string_lossy().to_string())
}

fn reload_category(
    _registry: &mut GameDataRegistry,
    category: &str,
    _data_root: &std::path::Path,
) -> Result<(), DataLoadError> {
    tracing::info!("Reloading category: {category}");
    // Selon la categorie, recharger les donnees correspondantes.
    // Implementation par domaine (classes, skills, items, etc.).
    Ok(())
}
```

---

## 5. Exemples de fichiers TOML complets

### 5.1 Item unique

```toml
# data/items/uniques/weapons.toml (extrait)

[[uniques]]
id = "harlequin_crest"
name = "Harlequin Crest"
base_type = "shako"
quality_level = 69
required_level = 62
properties = [
    { stat = "all_skills", value = 2 },
    { stat = "life_per_level", min = 1, max = 1 },
    { stat = "mana_per_level", min = 1, max = 1 },
    { stat = "damage_reduction", value = 10 },
    { stat = "magic_find", value = 50 },
    { stat = "all_attributes", value = 2 },
]
```

### 5.2 Monstre normal + super unique

```toml
# data/monsters/act1/fallen.toml

[[monsters]]
id = "fallen"
name = "Fallen"
act = 1
zone_ids = ["blood_moor", "cold_plains", "stony_field"]
monster_type = "Demon"
ai_archetype = "coward_melee"
level_normal = 3
level_nightmare = 37
level_hell = 68
hp_normal = 8
hp_nightmare = 480
hp_hell = 2400
damage_min = 2
damage_max = 4
defense = 5
attack_rating = 15
fire_resist_normal = 0
fire_resist_nightmare = 0
fire_resist_hell = 33
cold_resist_normal = 0
cold_resist_nightmare = 0
cold_resist_hell = 33
lightning_resist_normal = 0
lightning_resist_nightmare = 0
lightning_resist_hell = 33
poison_resist_normal = 0
poison_resist_nightmare = 0
poison_resist_hell = 33
physical_resist = 0
magic_resist = 0
walk_speed = 3.0
run_speed = 5.0
attack_speed_frames = 12
aggro_range = 8.0
leash_range = 20.0
experience = 10
treasure_class = "act1_melee_a"
drop_table_id = "tc_act1_normal"
can_be_champion = true
hitbox_radius = 0.3
sprite_id = "fallen_idle"

# data/monsters/super_uniques/rakanishu.toml

[super_unique]
id = "rakanishu"
name = "Rakanishu"
base_monster_id = "carver"
zone_id = "stony_field"
level_normal = 8
level_nightmare = 42
level_hell = 74
fixed_affixes = ["lightning_enchanted", "extra_fast"]
hp_multiplier = 5.0
experience_multiplier = 3.0
treasure_class = "act1_super_a"
always_drop = true
spawn_position = [45.5, 32.0]
```

### 5.3 Skill avec synergies

```toml
# data/skills/necromancer/bone_spear.toml

[skill]
id = "bone_spear"
name = "Bone Spear"
class = "Mortecian"
tree = "poison_bone"
tree_position = [1, 3]
max_level = 20
required_level = 18
prerequisites = ["teeth"]
skill_type = "Ranged"
mana_cost_base = 7.0
mana_cost_per_level = 0.5
cooldown_frames = 0
cast_frames_base = 15
damage_type = "Magic"

[[skill.synergies]]
skill_id = "teeth"
bonus_per_level = 7.0
bonus_type = "damage_percent"

[[skill.synergies]]
skill_id = "bone_wall"
bonus_per_level = 7.0
bonus_type = "damage_percent"

[[skill.synergies]]
skill_id = "bone_prison"
bonus_per_level = 7.0
bonus_type = "damage_percent"

[skill.projectile]
speed = 15.0
max_range = 30.0
pierce_chance = 100
count = 1
spread_angle = 0.0
homing = false

[skill.effect_formula]
base_damage_min = 16
base_damage_max = 24
damage_per_level = 8.0
duration_frames_base = 0
duration_per_level = 0.0
```

### 5.4 Zone avec waypoint et monstres

```toml
# data/zones/act1/blood_moor.toml

[zone]
id = "blood_moor"
name = "Blood Moor"
act = 1
area_level_normal = 1
area_level_nightmare = 36
area_level_hell = 67
is_town = false
has_waypoint = false
map_file = "maps/act1/blood_moor.ldtk"
music_id = "act1_wilderness"
ambient_id = "outdoor_forest"
lighting = "daylight"

[[zone.connections]]
target_zone_id = "rogue_encampment"
portal_type = "path"
position = [5.0, 45.0]

[[zone.connections]]
target_zone_id = "cold_plains"
portal_type = "path"
position = [90.0, 10.0]

[[zone.connections]]
target_zone_id = "den_of_evil"
portal_type = "entrance"
position = [55.0, 25.0]

[[zone.monster_spawns]]
monster_id = "fallen"
count_min = 4
count_max = 8
spawn_area = [20.0, 10.0, 80.0, 40.0]
respawn_delay_frames = 7500

[[zone.monster_spawns]]
monster_id = "quill_rat"
count_min = 3
count_max = 6
spawn_area = [10.0, 5.0, 50.0, 30.0]
respawn_delay_frames = 7500

[[zone.monster_spawns]]
monster_id = "zombie"
count_min = 2
count_max = 5
spawn_area = [40.0, 20.0, 85.0, 45.0]
respawn_delay_frames = 7500
```

### 5.5 Runeword

```toml
# data/items/runewords.toml (extrait)

[[runewords]]
id = "spirit"
name = "Spirit"
rune_sequence = ["Tal", "Thul", "Ort", "Amn"]
allowed_types = ["Sword", "Shield"]
required_level = 25
properties = [
    { stat = "all_skills", value = 2 },
    { stat = "faster_cast_rate", min = 25, max = 35 },
    { stat = "faster_hit_recovery", value = 55 },
    { stat = "vitality", value = 22 },
    { stat = "mana", min = 89, max = 112 },
    { stat = "magic_absorb", min = 3, max = 8 },
]
```

---

## 6. Validation cross-reference

```rust
/// @id: sd-data-validation @do: define @role: arpg @layer: 3
///
/// Valide la coherence des references croisees dans le registre.
/// Appele une fois apres le chargement complet de toutes les donnees.
pub fn validate_registry(registry: &GameDataRegistry) -> Result<(), DataLoadError> {
    let mut errors = Vec::new();

    // 1. Verifier que chaque skill.synergy_id reference un skill existant.
    for (skill_id, skill_def) in &registry.skills {
        for synergy in &skill_def.synergies {
            if !registry.skills.contains_key(&synergy.skill_id) {
                errors.push(format!(
                    "Skill '{skill_id}': synergy '{}' references unknown skill",
                    synergy.skill_id
                ));
            }
        }
        // Verifier les prerequisites.
        for prereq in &skill_def.prerequisites {
            if !registry.skills.contains_key(prereq) {
                errors.push(format!(
                    "Skill '{skill_id}': prerequisite '{prereq}' references unknown skill"
                ));
            }
        }
    }

    // 2. Verifier que chaque monster.drop_table_id reference une TC existante.
    for (monster_id, monster_def) in &registry.monsters {
        if !registry.treasure_classes.contains_key(&monster_def.drop_table_id) {
            errors.push(format!(
                "Monster '{monster_id}': drop_table_id '{}' references unknown treasure class",
                monster_def.drop_table_id
            ));
        }
    }

    // 3. Verifier que chaque zone.monster_spawns.monster_id existe.
    for (zone_id, zone_def) in &registry.zones {
        for spawn in &zone_def.monster_spawns {
            if !registry.monsters.contains_key(&spawn.monster_id) {
                errors.push(format!(
                    "Zone '{zone_id}': monster_spawn '{}' references unknown monster",
                    spawn.monster_id
                ));
            }
        }
        // Verifier les connexions.
        for conn in &zone_def.connections {
            if !registry.zones.contains_key(&conn.target_zone_id) {
                errors.push(format!(
                    "Zone '{zone_id}': connection to '{}' references unknown zone",
                    conn.target_zone_id
                ));
            }
        }
    }

    // 4. Verifier que chaque unique_item.base_type reference un item_base existant.
    for (unique_id, unique_def) in &registry.unique_items {
        if !unique_def.base_type.is_empty()
            && !registry.item_bases.contains_key(&unique_def.base_type)
        {
            errors.push(format!(
                "Unique '{unique_id}': base_type '{}' references unknown item base",
                unique_def.base_type
            ));
        }
    }

    // 5. Verifier les runewords.
    for runeword in &registry.runewords {
        for rune_name in &runeword.rune_sequence {
            let rune_id = rune_name.to_lowercase();
            if !registry.runes.contains_key(&rune_id) {
                errors.push(format!(
                    "Runeword '{}': rune '{}' not found in rune definitions",
                    runeword.id, rune_name
                ));
            }
        }
    }

    // 6. Verifier les quetes.
    for (quest_id, quest_def) in &registry.quests {
        for prereq in &quest_def.prerequisite_quests {
            if !registry.quests.contains_key(prereq) {
                errors.push(format!(
                    "Quest '{quest_id}': prerequisite '{prereq}' references unknown quest"
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(DataLoadError::ValidationErrors { errors })
    }
}
```

---

## 7. Erreurs explicites

```rust
/// @id: sd-data-errors @do: define @role: arpg @layer: 3

#[derive(Debug, thiserror::Error)]
pub enum DataLoadError {
    #[error("IO error reading '{path}': {source}")]
    IoError { path: String, source: String },

    #[error("TOML parse error in '{path}': {source}")]
    ParseError { path: String, source: String },

    #[error("Directory not found: '{path}'")]
    DirectoryNotFound { path: String },

    #[error("Duplicate ID '{id}' found in '{path}'")]
    DuplicateId { id: String, path: String },

    #[error("Watcher error: {source}")]
    WatcherError { source: String },

    #[error("Validation errors:\n{}", errors.join("\n"))]
    ValidationErrors { errors: Vec<String> },

    #[error("Value out of range: {field} = {value} (expected {min}..{max})")]
    ValueOutOfRange {
        field: String,
        value: String,
        min: String,
        max: String,
    },
}
```

---

## 8. Tests

### 8.1 Test deserialisation item valide

```rust
#[cfg(test)]
mod data_tests {
    use super::*;

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

        let file: ItemBaseFile = toml::from_str(toml_str).unwrap();
        assert_eq!(file.weapons.len(), 1);
        assert_eq!(file.weapons[0].id, "long_sword");
        assert_eq!(file.weapons[0].damage_min, Some(3));
        assert_eq!(file.weapons[0].damage_max, Some(16));
        assert_eq!(file.weapons[0].max_sockets, 2);
    }

    #[derive(Debug, serde::Deserialize)]
    struct ItemBaseFile {
        weapons: Vec<ItemBaseDef>,
    }
}
```

### 8.2 Test rejet TOML invalide

```rust
#[cfg(test)]
mod invalid_toml_tests {
    use super::*;

    #[test]
    fn test_reject_unknown_field() {
        let toml_str = r#"
[zone]
id = "test_zone"
name = "Test Zone"
act = 1
area_level_normal = 1
area_level_nightmare = 36
area_level_hell = 67
is_town = false
has_waypoint = false
map_file = "test.ldtk"
music_id = "test"
ambient_id = "test"
lighting = "daylight"
unknown_field = "should_fail"
"#;

        let result = toml::from_str::<ZoneWrapper>(toml_str);
        assert!(result.is_err(), "Should reject unknown fields");
    }

    #[derive(Debug, serde::Deserialize)]
    struct ZoneWrapper {
        #[allow(dead_code)]
        zone: ZoneDef,
    }

    #[test]
    fn test_reject_missing_required_field() {
        let toml_str = r#"
[zone]
id = "test_zone"
name = "Test Zone"
"#;
        // Manque act, area_level_*, etc.
        let result = toml::from_str::<ZoneWrapper>(toml_str);
        assert!(result.is_err(), "Should reject missing required fields");
    }
}
```

### 8.3 Test validation cross-reference

```rust
#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_validation_detects_broken_synergy() {
        let mut registry = create_empty_registry();

        let mut skills = std::collections::HashMap::new();
        skills.insert("bone_spear".to_string(), SkillDef {
            id: "bone_spear".to_string(),
            name: "Bone Spear".to_string(),
            class: "Mortecian".to_string(),
            tree: "poison_bone".to_string(),
            tree_position: [1, 3],
            max_level: 20,
            required_level: 18,
            prerequisites: vec!["teeth".to_string()],
            skill_type: SkillType::Ranged,
            mana_cost_base: 7.0,
            mana_cost_per_level: 0.5,
            cooldown_frames: 0,
            cast_frames_base: 15,
            damage_type: DamageType::Magic,
            synergies: vec![SynergyDef {
                skill_id: "nonexistent_skill".to_string(),
                bonus_per_level: 7.0,
                bonus_type: "damage_percent".to_string(),
            }],
            projectile: None,
            aoe_radius: None,
            effect_formula: SkillFormula {
                base_damage_min: 16,
                base_damage_max: 24,
                damage_per_level: 8.0,
                duration_frames_base: 0,
                duration_per_level: 0.0,
            },
        });
        registry.skills = skills;

        let result = validate_registry(&registry);
        assert!(result.is_err());

        if let Err(DataLoadError::ValidationErrors { errors }) = result {
            assert!(errors.iter().any(|e| e.contains("nonexistent_skill")));
        }
    }

    fn create_empty_registry() -> GameDataRegistry {
        GameDataRegistry {
            classes: std::collections::HashMap::new(),
            skills: std::collections::HashMap::new(),
            item_bases: std::collections::HashMap::new(),
            prefixes: Vec::new(),
            suffixes: Vec::new(),
            unique_items: std::collections::HashMap::new(),
            set_items: std::collections::HashMap::new(),
            runewords: Vec::new(),
            runes: std::collections::HashMap::new(),
            gems: std::collections::HashMap::new(),
            cube_recipes: Vec::new(),
            monsters: std::collections::HashMap::new(),
            champion_affixes: Vec::new(),
            super_uniques: std::collections::HashMap::new(),
            zones: std::collections::HashMap::new(),
            treasure_classes: std::collections::HashMap::new(),
            quests: std::collections::HashMap::new(),
            breakpoint_tables: BreakpointTables {
                fcr: std::collections::HashMap::new(),
                fhr: std::collections::HashMap::new(),
                fbr: std::collections::HashMap::new(),
                ias: std::collections::HashMap::new(),
            },
            experience_table: Vec::new(),
            difficulty_config: DifficultyConfig {
                normal: DifficultyLevel {
                    fire_resist_penalty: 0,
                    cold_resist_penalty: 0,
                    lightning_resist_penalty: 0,
                    poison_resist_penalty: 0,
                    experience_multiplier: 1.0,
                    gold_multiplier: 1.0,
                },
                nightmare: DifficultyLevel {
                    fire_resist_penalty: -40,
                    cold_resist_penalty: -40,
                    lightning_resist_penalty: -40,
                    poison_resist_penalty: -40,
                    experience_multiplier: 1.0,
                    gold_multiplier: 4.0,
                },
                hell: DifficultyLevel {
                    fire_resist_penalty: -100,
                    cold_resist_penalty: -100,
                    lightning_resist_penalty: -100,
                    poison_resist_penalty: -100,
                    experience_multiplier: 1.0,
                    gold_multiplier: 8.0,
                },
            },
            shrine_defs: Vec::new(),
        }
    }
}
```

---

*Document redige par Francois, Dev Back-End -- Miyukini AI Studio*
*Base sur SD-Tech-Data-Schemas.md de Denis*
*Revision : 2026-02-28 v1.0*
