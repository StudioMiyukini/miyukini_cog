//! Registre central de toutes les definitions de jeu.
//!
//! <!-- @id: sd-data-registry @do: define @role: arpg @layer: 3 @human: miyuk -->
//!
//! Le [`GameDataRegistry`] est charge au demarrage depuis le repertoire
//! `data/` et insere comme resource ECS globale.

use std::collections::HashMap;
use std::path::Path;

use crate::loader::{load_toml_directory_recursive, load_toml_file};
use crate::types::{
    AffixDef, AffixTableFile, BossDef, BossFile, BreakpointsFile, ChampionAffixDef,
    ChampionAffixesFile, ClassDef, ClassFile, CubeRecipeDef, CubeRecipesFile, DifficultyConfig,
    DifficultyLevel, ExperienceFile, GemDef, GemsFile, ItemBaseDef, MonsterDef, MonsterFile,
    QuestDef, QuestFile, RuneDef, RunewordDef, RunewordsFile, RunesFile, SetFile, SetHeader,
    ShrineDef, ShrinesFile, SkillDef, SkillFile, SuffixTableFile, SuperUniqueDef, SuperUniqueFile,
    TreasureClassDef, TreasureClassFileDef, UniqueItemDef, UniqueItemsFile, ZoneDef, ZoneFile,
};
use crate::validation::validate_registry;
use crate::DataLoadError;

/// Registre central de toutes les definitions de jeu chargees depuis TOML.
///
/// Insere comme resource ECS globale apres chargement et validation.
#[derive(Debug)]
pub struct GameDataRegistry {
    /// Definitions de classes.
    pub classes: HashMap<String, ClassDef>,
    /// Definitions de skills.
    pub skills: HashMap<String, SkillDef>,
    /// Definitions d'items de base.
    pub item_bases: HashMap<String, ItemBaseDef>,
    /// Prefixes d'affixes.
    pub prefixes: Vec<AffixDef>,
    /// Suffixes d'affixes.
    pub suffixes: Vec<AffixDef>,
    /// Items uniques.
    pub unique_items: HashMap<String, UniqueItemDef>,
    /// Sets d'items.
    pub set_items: HashMap<String, SetHeader>,
    /// Runewords.
    pub runewords: Vec<RunewordDef>,
    /// Runes.
    pub runes: HashMap<String, RuneDef>,
    /// Gemmes.
    pub gems: HashMap<String, GemDef>,
    /// Recettes du cube alchimique.
    pub cube_recipes: Vec<CubeRecipeDef>,
    /// Monstres.
    pub monsters: HashMap<String, MonsterDef>,
    /// Affixes de champion.
    pub champion_affixes: Vec<ChampionAffixDef>,
    /// Super-uniques.
    pub super_uniques: HashMap<String, SuperUniqueDef>,
    /// Bosses.
    pub bosses: HashMap<String, BossDef>,
    /// Zones.
    pub zones: HashMap<String, ZoneDef>,
    /// Treasure classes.
    pub treasure_classes: HashMap<String, TreasureClassDef>,
    /// Quetes.
    pub quests: HashMap<String, QuestDef>,
    /// Tables de breakpoints.
    pub breakpoint_tables: BreakpointsFile,
    /// Table d'experience par niveau.
    pub experience_table: Vec<u64>,
    /// Configuration de difficulte.
    pub difficulty_config: DifficultyConfig,
    /// Definitions de shrines.
    pub shrine_defs: Vec<ShrineDef>,
}

impl GameDataRegistry {
    /// Charge toutes les donnees depuis le repertoire racine `data/`.
    ///
    /// Les fichiers TOML sont deserialises, valides par references croisees,
    /// puis stockes dans le registre.
    pub fn load_all(data_root: &Path) -> Result<Self, DataLoadError> {
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
        let bosses = load_bosses(&data_root.join("monsters"))?;
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
            bosses,
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

    /// Cree un registre vide (utile pour les tests).
    pub fn empty() -> Self {
        Self {
            classes: HashMap::new(),
            skills: HashMap::new(),
            item_bases: HashMap::new(),
            prefixes: Vec::new(),
            suffixes: Vec::new(),
            unique_items: HashMap::new(),
            set_items: HashMap::new(),
            runewords: Vec::new(),
            runes: HashMap::new(),
            gems: HashMap::new(),
            cube_recipes: Vec::new(),
            monsters: HashMap::new(),
            champion_affixes: Vec::new(),
            super_uniques: HashMap::new(),
            bosses: HashMap::new(),
            zones: HashMap::new(),
            treasure_classes: HashMap::new(),
            quests: HashMap::new(),
            breakpoint_tables: BreakpointsFile {
                fcr: HashMap::new(),
                fhr: HashMap::new(),
                fbr: HashMap::new(),
                ias: HashMap::new(),
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

// =========================================================================
// Fonctions de chargement par domaine
// =========================================================================

/// Charge les definitions de classes depuis `data/classes/`.
fn load_classes(dir: &Path) -> Result<HashMap<String, ClassDef>, DataLoadError> {
    load_toml_directory_recursive(dir, &|file: ClassFile| {
        vec![(file.class.id.clone(), file.class)]
    })
}

/// Charge les definitions de skills depuis `data/skills/`.
fn load_skills(dir: &Path) -> Result<HashMap<String, SkillDef>, DataLoadError> {
    load_toml_directory_recursive(dir, &|file: SkillFile| {
        vec![(file.skill.id.clone(), file.skill)]
    })
}

/// Charge les items de base depuis `data/items/bases/`.
fn load_item_bases(dir: &Path) -> Result<HashMap<String, ItemBaseDef>, DataLoadError> {
    let mut result = HashMap::new();

    if !dir.exists() {
        return Err(DataLoadError::DirectoryNotFound {
            path: dir.display().to_string(),
        });
    }

    let entries = std::fs::read_dir(dir).map_err(|e| DataLoadError::IoError {
        path: dir.display().to_string(),
        message: e.to_string(),
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| DataLoadError::IoError {
            path: dir.display().to_string(),
            message: e.to_string(),
        })?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }

        // Essayer de charger comme fichier d'armes d'abord.
        let content = std::fs::read_to_string(&path).map_err(|e| DataLoadError::IoError {
            path: path.display().to_string(),
            message: e.to_string(),
        })?;

        // Charger generiquement en tant que table TOML.
        let table: toml::Value = toml::from_str(&content).map_err(|e| DataLoadError::ParseError {
            path: path.display().to_string(),
            message: e.to_string(),
        })?;

        // Extraire les items de toutes les cles de tableau.
        if let toml::Value::Table(map) = table {
            for (_key, value) in map {
                if let toml::Value::Array(items) = value {
                    for item_val in items {
                        let item: ItemBaseDef = item_val.try_into().map_err(|e: toml::de::Error| {
                            DataLoadError::ParseError {
                                path: path.display().to_string(),
                                message: e.to_string(),
                            }
                        })?;
                        let id = item.id.clone();
                        if result.contains_key(&id) {
                            return Err(DataLoadError::DuplicateId {
                                id,
                                path: path.display().to_string(),
                            });
                        }
                        result.insert(id, item);
                    }
                }
            }
        }
    }

    Ok(result)
}

/// Charge les affixes (prefixes et suffixes) depuis `data/items/affixes/`.
fn load_affixes(dir: &Path) -> Result<(Vec<AffixDef>, Vec<AffixDef>), DataLoadError> {
    let prefix_path = dir.join("prefixes.toml");
    let suffix_path = dir.join("suffixes.toml");

    let prefixes = if prefix_path.exists() {
        let file: AffixTableFile = load_toml_file(&prefix_path)?;
        file.prefixes
    } else {
        Vec::new()
    };

    let suffixes = if suffix_path.exists() {
        let file: SuffixTableFile = load_toml_file(&suffix_path)?;
        file.suffixes
    } else {
        Vec::new()
    };

    Ok((prefixes, suffixes))
}

/// Charge les items uniques depuis `data/items/uniques/`.
fn load_uniques(dir: &Path) -> Result<HashMap<String, UniqueItemDef>, DataLoadError> {
    let mut result = HashMap::new();

    if !dir.exists() {
        return Err(DataLoadError::DirectoryNotFound {
            path: dir.display().to_string(),
        });
    }

    let entries = std::fs::read_dir(dir).map_err(|e| DataLoadError::IoError {
        path: dir.display().to_string(),
        message: e.to_string(),
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| DataLoadError::IoError {
            path: dir.display().to_string(),
            message: e.to_string(),
        })?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }

        let file: UniqueItemsFile = load_toml_file(&path)?;
        for unique in file.uniques {
            let id = unique.id.clone();
            if result.contains_key(&id) {
                return Err(DataLoadError::DuplicateId {
                    id,
                    path: path.display().to_string(),
                });
            }
            result.insert(id, unique);
        }
    }

    Ok(result)
}

/// Charge les sets depuis `data/items/sets/`.
fn load_sets(dir: &Path) -> Result<HashMap<String, SetHeader>, DataLoadError> {
    load_toml_directory_recursive(dir, &|file: SetFile| {
        vec![(file.set.id.clone(), file.set)]
    })
}

/// Charge les runewords depuis `data/items/runewords.toml`.
fn load_runewords(path: &Path) -> Result<Vec<RunewordDef>, DataLoadError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file: RunewordsFile = load_toml_file(path)?;
    Ok(file.runewords)
}

/// Charge les runes depuis `data/items/runes.toml`.
fn load_runes(path: &Path) -> Result<HashMap<String, RuneDef>, DataLoadError> {
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let file: RunesFile = load_toml_file(path)?;
    let mut result = HashMap::new();
    for rune in file.runes {
        let id = rune.id.clone();
        result.insert(id, rune);
    }
    Ok(result)
}

/// Charge les gemmes depuis `data/items/gems.toml`.
fn load_gems(path: &Path) -> Result<HashMap<String, GemDef>, DataLoadError> {
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let file: GemsFile = load_toml_file(path)?;
    let mut result = HashMap::new();
    for gem in file.gems {
        let id = gem.id.clone();
        result.insert(id, gem);
    }
    Ok(result)
}

/// Charge les recettes du cube depuis `data/items/cube_recipes.toml`.
fn load_cube_recipes(path: &Path) -> Result<Vec<CubeRecipeDef>, DataLoadError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file: CubeRecipesFile = load_toml_file(path)?;
    Ok(file.recipes)
}

/// Charge les monstres depuis `data/monsters/` (excluant super_uniques et affixes).
fn load_monsters(dir: &Path) -> Result<HashMap<String, MonsterDef>, DataLoadError> {
    let mut result = HashMap::new();

    if !dir.exists() {
        return Err(DataLoadError::DirectoryNotFound {
            path: dir.display().to_string(),
        });
    }

    // Parcourir les sous-repertoires act{N}/.
    let entries = std::fs::read_dir(dir).map_err(|e| DataLoadError::IoError {
        path: dir.display().to_string(),
        message: e.to_string(),
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| DataLoadError::IoError {
            path: dir.display().to_string(),
            message: e.to_string(),
        })?;
        let path = entry.path();

        // Ignorer les repertoires speciaux.
        if path.is_dir() {
            let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if dir_name == "super_uniques" || dir_name == "affixes" {
                continue;
            }

            // Charger les monstres dans ce sous-repertoire.
            let sub = load_toml_directory_recursive(&path, &|file: MonsterFile| {
                vec![(file.monster.id.clone(), file.monster)]
            })?;
            for (id, monster) in sub {
                if result.contains_key(&id) {
                    return Err(DataLoadError::DuplicateId {
                        id,
                        path: path.display().to_string(),
                    });
                }
                result.insert(id, monster);
            }
        }
    }

    Ok(result)
}

/// Charge les champion affixes depuis un fichier TOML.
fn load_champion_affixes(path: &Path) -> Result<Vec<ChampionAffixDef>, DataLoadError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file: ChampionAffixesFile = load_toml_file(path)?;
    Ok(file.champion_affixes)
}

/// Charge les super-uniques depuis `data/monsters/super_uniques/`.
fn load_super_uniques(dir: &Path) -> Result<HashMap<String, SuperUniqueDef>, DataLoadError> {
    if !dir.exists() {
        return Ok(HashMap::new());
    }
    load_toml_directory_recursive(dir, &|file: SuperUniqueFile| {
        vec![(file.super_unique.id.clone(), file.super_unique)]
    })
}

/// Parcourt recursivement un repertoire a la recherche de fichiers boss TOML.
fn scan_for_bosses(
    dir: &Path,
    result: &mut HashMap<String, BossDef>,
) -> Result<(), DataLoadError> {
    let entries = std::fs::read_dir(dir).map_err(|e| DataLoadError::IoError {
        path: dir.display().to_string(),
        message: e.to_string(),
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| DataLoadError::IoError {
            path: dir.display().to_string(),
            message: e.to_string(),
        })?;
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if dir_name != "super_uniques" && dir_name != "affixes" {
                scan_for_bosses(&path, result)?;
            }
            continue;
        }

        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }

        // Essayer de parser comme `BossFile` ; ignorer silencieusement si ce n'est pas un boss.
        let content = std::fs::read_to_string(&path).map_err(|e| DataLoadError::IoError {
            path: path.display().to_string(),
            message: e.to_string(),
        })?;

        if let Ok(boss_file) = toml::from_str::<BossFile>(&content) {
            let id = boss_file.boss.id.clone();
            result.insert(id, boss_file.boss);
        }
    }

    Ok(())
}

/// Charge les bosses depuis `data/monsters/` (fichiers boss dans les act).
fn load_bosses(dir: &Path) -> Result<HashMap<String, BossDef>, DataLoadError> {
    let mut result = HashMap::new();

    if !dir.exists() {
        return Ok(result);
    }

    scan_for_bosses(dir, &mut result)?;
    Ok(result)
}

/// Charge les zones depuis `data/zones/`.
fn load_zones(dir: &Path) -> Result<HashMap<String, ZoneDef>, DataLoadError> {
    load_toml_directory_recursive(dir, &|file: ZoneFile| {
        vec![(file.zone.id.clone(), file.zone)]
    })
}

/// Charge les treasure classes depuis un fichier TOML.
fn load_treasure_classes(path: &Path) -> Result<HashMap<String, TreasureClassDef>, DataLoadError> {
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let file: TreasureClassFileDef = load_toml_file(path)?;
    let mut result = HashMap::new();
    for tc in file.treasure_classes {
        let id = tc.id.clone();
        result.insert(id, tc);
    }
    Ok(result)
}

/// Charge les quetes depuis `data/quests/`.
fn load_quests(dir: &Path) -> Result<HashMap<String, QuestDef>, DataLoadError> {
    load_toml_directory_recursive(dir, &|file: QuestFile| {
        vec![(file.quest.id.clone(), file.quest)]
    })
}

/// Charge les breakpoints depuis un fichier TOML.
fn load_breakpoints(path: &Path) -> Result<BreakpointsFile, DataLoadError> {
    if !path.exists() {
        return Ok(BreakpointsFile {
            fcr: HashMap::new(),
            fhr: HashMap::new(),
            fbr: HashMap::new(),
            ias: HashMap::new(),
        });
    }
    load_toml_file(path)
}

/// Charge la table d'experience depuis un fichier TOML.
fn load_experience_table(path: &Path) -> Result<Vec<u64>, DataLoadError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file: ExperienceFile = load_toml_file(path)?;
    Ok(file.experience_table)
}

/// Charge la configuration de difficulte depuis un fichier TOML.
fn load_difficulty(path: &Path) -> Result<DifficultyConfig, DataLoadError> {
    load_toml_file(path)
}

/// Charge les definitions de shrines depuis un fichier TOML.
fn load_shrines(path: &Path) -> Result<Vec<ShrineDef>, DataLoadError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file: ShrinesFile = load_toml_file(path)?;
    Ok(file.shrines)
}
