//! Definitions de monstres, super-uniques, champion affixes, bosses.
//!
//! <!-- @id: sd-data-monster-def @do: define @role: arpg @layer: 3 @human: miyuk -->
//!
//! Fichiers TOML source :
//! - `data/monsters/act{N}/{monstre}.toml`
//! - `data/monsters/super_uniques/{su}.toml`
//! - `data/monsters/affixes/champion_affixes.toml`

use serde::{Deserialize, Serialize};

use super::common::StatValue;

// =========================================================================
// Monstre de base
// =========================================================================

/// Fichier TOML de monstre (wrapper racine).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterFile {
    /// Definition du monstre.
    pub monster: MonsterDef,
}

/// Definition complete d'un monstre.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterDef {
    /// Identifiant unique (ex: `"fallen"`).
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Type de monstre (ex: `"Demon"`, `"Undead"`).
    #[serde(rename = "type")]
    pub monster_type_tag: String,
    /// Archetype d'IA (ex: `"MeleeSwarm"`, `"RangedKiter"`).
    pub ai_archetype: String,
    /// Sprite de base.
    pub base_sprite: String,
    /// Rayon de hitbox sur la grille.
    pub grid_hitbox_radius: f32,
    /// Stats par difficulte.
    pub stats: MonsterStatsByDifficulty,
    /// Parametres d'IA.
    pub ai_params: MonsterAiParams,
    /// Configuration de drops.
    pub drops: MonsterDrops,
}

/// Stats d'un monstre par difficulte (Normal, Nightmare, Hell).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterStatsByDifficulty {
    /// Stats en Normal.
    pub normal: MonsterStatsBlock,
    /// Stats en Nightmare.
    pub nightmare: MonsterStatsBlock,
    /// Stats en Hell.
    pub hell: MonsterStatsBlock,
}

/// Bloc de stats d'un monstre pour une difficulte donnee.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterStatsBlock {
    /// Niveau du monstre.
    pub level: u8,
    /// Points de vie.
    pub life: i32,
    /// Defense.
    pub defense: i32,
    /// Attack Rating.
    pub attack_rating: i32,
    /// Degats minimum.
    pub damage_min: i32,
    /// Degats maximum.
    pub damage_max: i32,
    /// Experience accordee.
    pub experience: u32,
    /// Resistance au feu.
    pub fire_resist: i32,
    /// Resistance au froid.
    pub cold_resist: i32,
    /// Resistance a la foudre.
    pub lightning_resist: i32,
    /// Resistance au poison.
    pub poison_resist: i32,
    /// Resistance physique.
    pub physical_resist: i32,
    /// Immunites (ex: `["Fire"]`).
    #[serde(default)]
    pub immunities: Vec<String>,
}

/// Parametres d'IA d'un monstre.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterAiParams {
    /// Rayon d'aggro.
    pub aggro_radius: f32,
    /// Rayon de laisse (distance max de poursuite).
    pub leash_radius: f32,
    /// Seuil de fuite en % de PV.
    #[serde(default)]
    pub flee_hp_pct: Option<f32>,
    /// Se rallie-t-il apres la fuite ?
    #[serde(default)]
    pub rally_on_flee: bool,
    /// Cooldown d'attaque en frames.
    pub attack_cooldown: u32,
    /// Skills utilisables par le monstre.
    #[serde(default)]
    pub skills: Vec<MonsterSkillEntry>,
}

/// Entree de skill pour un monstre.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterSkillEntry {
    /// ID du skill utilise.
    pub skill_id: String,
    /// Priorite (1 = plus haute).
    pub priority: u8,
    /// Condition d'utilisation.
    pub condition: MonsterSkillCondition,
    /// Cooldown specifique en frames.
    pub cooldown: u32,
}

/// Condition d'utilisation d'un skill par un monstre (tagged union).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MonsterSkillCondition {
    /// Toujours utilisable.
    Always,
    /// Cible a portee.
    TargetInRange {
        /// Distance maximale.
        range: f32,
    },
    /// PV en dessous d'un seuil.
    HealthBelow {
        /// Pourcentage de PV.
        pct: f32,
    },
    /// Cooldown termine.
    CooldownReady,
    /// Allies a proximite.
    AlliesNearby {
        /// Nombre d'allies requis.
        count: u32,
    },
}

/// Configuration de drops d'un monstre.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterDrops {
    /// ID de la treasure class.
    pub loot_table: String,
    /// Or minimum.
    pub gold_min: i32,
    /// Or maximum.
    pub gold_max: i32,
}

// =========================================================================
// Super Uniques
// =========================================================================

/// Fichier TOML de super unique (wrapper racine).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperUniqueFile {
    /// Definition du super unique.
    pub super_unique: SuperUniqueDef,
}

/// Definition d'un monstre super-unique.
///
/// <!-- @id: sd-data-super-unique-def @do: define @role: arpg @layer: 3 @human: miyuk -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperUniqueDef {
    /// Identifiant unique.
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// ID du monstre de base.
    pub base_monster: String,
    /// ID de la zone de spawn.
    pub zone: String,
    /// Position de spawn.
    pub position: [f32; 2],
    /// Affixes fixes.
    pub fixed_affixes: Vec<String>,
    /// Multiplicateurs de stats.
    pub multipliers: SuperUniqueMultipliers,
    /// Drops supplementaires.
    pub extra_drops: SuperUniqueDrops,
}

/// Multiplicateurs de stats pour un super unique.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperUniqueMultipliers {
    /// Multiplicateur de vie.
    pub life_mult: f32,
    /// Multiplicateur de degats.
    pub damage_mult: f32,
    /// Multiplicateur d'attack rating.
    pub attack_rating_mult: f32,
    /// Multiplicateur d'experience.
    pub experience_mult: f32,
}

/// Drops supplementaires pour un super unique.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperUniqueDrops {
    /// Items garantis.
    pub guaranteed_drops: Vec<String>,
    /// Treasure class supplementaire.
    pub extra_loot_table: String,
}

// =========================================================================
// Champion Affixes
// =========================================================================

/// Fichier TOML de champion affixes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionAffixesFile {
    /// Liste des affixes de champion.
    pub champion_affixes: Vec<ChampionAffixDef>,
}

/// Definition d'un affixe de champion/unique.
///
/// <!-- @id: sd-data-champion-affix @do: define @role: arpg @layer: 3 @human: miyuk -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionAffixDef {
    /// Identifiant unique.
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Modificateurs de statistiques.
    #[serde(default)]
    pub stat_modifiers: Vec<StatValue>,
    /// Effet a la mort.
    #[serde(default)]
    pub on_death: Option<String>,
    /// Effet au toucher.
    #[serde(default)]
    pub on_hit: Option<String>,
    /// Aura enchantee.
    #[serde(default)]
    pub aura: Option<AuraEnchant>,
    /// Effet visuel.
    #[serde(default)]
    pub visual: String,
}

/// Enchantement d'aura pour un champion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraEnchant {
    /// Type d'aura (ex: `"random"`).
    #[serde(rename = "type")]
    pub aura_type: String,
    /// Auras possibles.
    pub possible_auras: Vec<String>,
}

// =========================================================================
// Bosses
// =========================================================================

/// Fichier TOML de boss (wrapper racine).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossFile {
    /// Definition du boss.
    pub boss: BossDef,
}

/// Definition d'un boss d'acte.
///
/// <!-- @id: sd-data-boss-def @do: define @role: arpg @layer: 3 @human: miyuk -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossDef {
    /// Identifiant unique.
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Acte du boss.
    pub act: u8,
    /// Sprite de base.
    pub base_sprite: String,
    /// Rayon de hitbox.
    pub grid_hitbox_radius: f32,
    /// Stats par difficulte (reutilise le format monstre).
    pub stats: MonsterStatsByDifficulty,
    /// Parametres d'IA.
    pub ai_params: MonsterAiParams,
    /// Phases du boss.
    pub phases: Vec<BossPhase>,
    /// Configuration de drops.
    pub drops: BossDrops,
}

/// Phase d'un boss.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossPhase {
    /// Numero de la phase.
    pub phase: u8,
    /// Seuil de PV pour declencher la phase (1.0 = 100%).
    pub hp_threshold: f32,
    /// Comportement d'IA pendant cette phase.
    pub behavior: String,
    /// Description de la phase.
    pub description: String,
    /// Modificateurs de stats pendant cette phase.
    #[serde(default)]
    pub modifiers: Vec<StatValue>,
}

/// Drops d'un boss.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossDrops {
    /// Treasure class du boss.
    pub loot_table: String,
    /// Drop garanti de quete.
    #[serde(default)]
    pub guaranteed_quest_drop: Option<String>,
}
