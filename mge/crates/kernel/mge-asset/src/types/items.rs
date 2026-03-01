//! Definitions d'items : bases, affixes, uniques, sets, runewords, runes, gemmes.
//!
//! <!-- @id: sd-data-items @do: define @role: arpg @layer: 3 @human: miyuk -->

use serde::{Deserialize, Serialize};

use super::common::StatValue;

// =========================================================================
// Item de base
// =========================================================================

/// Fichier TOML contenant des items de base de type arme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemBaseWeaponsFile {
    /// Liste des armes definies dans le fichier.
    pub weapons: Vec<ItemBaseDef>,
}

/// Fichier TOML contenant des items de base de type armure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemBaseArmorsFile {
    /// Liste des armures definies dans le fichier.
    #[serde(default)]
    pub armor: Vec<ItemBaseDef>,
    /// Liste des boucliers.
    #[serde(default)]
    pub shields: Vec<ItemBaseDef>,
    /// Liste des casques.
    #[serde(default)]
    pub helms: Vec<ItemBaseDef>,
    /// Liste des gants.
    #[serde(default)]
    pub gloves: Vec<ItemBaseDef>,
    /// Liste des bottes.
    #[serde(default)]
    pub boots: Vec<ItemBaseDef>,
    /// Liste des ceintures.
    #[serde(default)]
    pub belts: Vec<ItemBaseDef>,
    /// Liste des anneaux.
    #[serde(default)]
    pub rings: Vec<ItemBaseDef>,
    /// Liste des amulettes.
    #[serde(default)]
    pub amulets: Vec<ItemBaseDef>,
}

/// Definition d'un item de base (commun a tous les types).
///
/// <!-- @id: sd-data-item-base @do: define @role: arpg @layer: 3 @human: miyuk -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemBaseDef {
    /// Identifiant unique de l'item (ex: `"long_sword"`).
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Categorie d'item (ex: `"Weapon"`, `"Armor"`, `"Shield"`).
    pub category: String,
    /// Type d'arme (armes uniquement).
    #[serde(default)]
    pub weapon_type: Option<String>,
    /// Type d'attaque (armes uniquement).
    #[serde(default)]
    pub attack_type: Option<String>,
    /// Tier d'item : `"Normal"`, `"Exceptional"`, `"Elite"`.
    pub tier: String,
    /// Degats minimum (armes).
    #[serde(default)]
    pub damage_min: Option<i32>,
    /// Degats maximum (armes).
    #[serde(default)]
    pub damage_max: Option<i32>,
    /// Defense minimum (armures).
    #[serde(default)]
    pub defense_min: Option<i32>,
    /// Defense maximum (armures).
    #[serde(default)]
    pub defense_max: Option<i32>,
    /// Weapon Speed Modifier (armes).
    #[serde(default)]
    pub speed: Option<i32>,
    /// Portee en tiles (armes).
    #[serde(default)]
    pub range: Option<f32>,
    /// Durabilite de l'objet.
    pub durability: i32,
    /// Taille dans l'inventaire `[colonnes, rangees]`.
    pub grid_size: [u8; 2],
    /// Force requise.
    pub required_strength: i32,
    /// Dexterite requise.
    pub required_dexterity: i32,
    /// Niveau requis.
    pub required_level: u8,
    /// Quality level (qlvl).
    pub quality_level: u8,
    /// Nombre maximum de sockets.
    pub max_sockets: u8,
    /// Facteur de bonus damage par STR (armes).
    #[serde(default)]
    pub str_factor: Option<i32>,
    /// Facteur de bonus damage par DEX (armes).
    #[serde(default)]
    pub dex_factor: Option<i32>,
    /// Peut etre ethereal ?
    pub can_be_ethereal: bool,
    /// Peut etre lance ?
    #[serde(default)]
    pub throwable: bool,
    /// Restriction de classe (`""` = aucune).
    #[serde(default)]
    pub class_restriction: String,
    /// Chance de block (boucliers).
    #[serde(default)]
    pub block_chance: Option<i32>,
    /// Nombre de rangees de potions (ceintures).
    #[serde(default)]
    pub potion_rows: Option<u8>,
    /// Magic level (circlets, wands, orbes).
    #[serde(default)]
    pub magic_level: Option<i32>,
}

// =========================================================================
// Affixes
// =========================================================================

/// Fichier TOML de prefixes d'affixes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffixTableFile {
    /// Liste des prefixes.
    pub prefixes: Vec<AffixDef>,
}

/// Fichier TOML de suffixes d'affixes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuffixTableFile {
    /// Liste des suffixes.
    pub suffixes: Vec<AffixDef>,
}

/// Definition d'un affixe (prefixe ou suffixe).
///
/// <!-- @id: sd-data-affix-def @do: define @role: arpg @layer: 3 @human: miyuk -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffixDef {
    /// Identifiant unique de l'affixe.
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Groupe d'affixe (pas deux du meme groupe sur un item).
    pub group: u32,
    /// Affix level requis (alvl).
    pub level: u8,
    /// Poids de drop (frequence).
    pub frequency: u32,
    /// Categories d'items autorisees.
    pub spawn_on: Vec<String>,
    /// Proprietes de l'affixe.
    pub properties: Vec<AffixPropertyDef>,
}

/// Propriete d'un affixe avec plage min/max.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffixPropertyDef {
    /// Nom de la statistique.
    pub stat: String,
    /// Valeur minimale.
    pub min: i32,
    /// Valeur maximale.
    pub max: i32,
}

// =========================================================================
// Rare Names
// =========================================================================

/// Fichier TOML de noms d'items rares.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RareNamesFile {
    /// Noms pour les armes.
    #[serde(default)]
    pub rare_names_weapon: Vec<RareNameEntry>,
    /// Noms pour les armures.
    #[serde(default)]
    pub rare_names_armor: Vec<RareNameEntry>,
    /// Noms pour les boucliers.
    #[serde(default)]
    pub rare_names_shield: Vec<RareNameEntry>,
    /// Noms pour les anneaux.
    #[serde(default)]
    pub rare_names_ring: Vec<RareNameEntry>,
    /// Noms pour les amulettes.
    #[serde(default)]
    pub rare_names_amulet: Vec<RareNameEntry>,
}

/// Entree de nom rare (prefixe + suffixe).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RareNameEntry {
    /// Prefixe du nom rare.
    pub prefix: String,
    /// Suffixe du nom rare.
    pub suffix: String,
}

// =========================================================================
// Unique Items
// =========================================================================

/// Fichier TOML d'items uniques.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueItemsFile {
    /// Liste des uniques.
    pub uniques: Vec<UniqueItemDef>,
}

/// Definition d'un item unique.
///
/// <!-- @id: sd-data-unique-def @do: define @role: arpg @layer: 3 @human: miyuk -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueItemDef {
    /// Identifiant unique.
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Type de base de l'item (reference vers `ItemBaseDef.id`).
    pub base_type: String,
    /// Quality level.
    pub quality_level: u8,
    /// Niveau requis.
    pub required_level: u8,
    /// Proprietes fixes de l'item unique.
    pub properties: Vec<StatValue>,
}

// =========================================================================
// Set Items
// =========================================================================

/// Fichier TOML de set (wrapper racine).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetFile {
    /// Definition du set.
    pub set: SetHeader,
}

/// Definition complete d'un set d'items.
///
/// <!-- @id: sd-data-set-def @do: define @role: arpg @layer: 3 @human: miyuk -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetHeader {
    /// Identifiant unique du set.
    pub id: String,
    /// Nom affiche du set.
    pub name: String,
    /// Restriction de classe (`""` = aucune).
    pub class_restriction: String,
    /// Nombre total de pieces dans le set.
    pub num_items: u8,
    /// Bonus partiels (par nombre de pieces equipees).
    pub partial_bonuses: Vec<PartialBonus>,
    /// Bonus complet (toutes les pieces equipees).
    pub full_bonus: FullBonus,
    /// Pieces du set.
    pub items: Vec<SetItemDef>,
}

/// Bonus partiel d'un set (active a N pieces).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialBonus {
    /// Nombre de pieces necessaires.
    pub count: u8,
    /// Proprietes accordees.
    pub properties: Vec<StatValue>,
}

/// Bonus complet d'un set (toutes les pieces).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullBonus {
    /// Proprietes accordees.
    pub properties: Vec<StatValue>,
}

/// Definition d'une piece individuelle d'un set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetItemDef {
    /// Identifiant unique de la piece.
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Type de base (reference vers `ItemBaseDef.id`).
    pub base_type: String,
    /// Niveau requis.
    pub required_level: u8,
    /// Proprietes fixes de la piece de set.
    pub properties: Vec<StatValue>,
}

// =========================================================================
// Runewords
// =========================================================================

/// Fichier TOML de runewords.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunewordsFile {
    /// Liste des runewords.
    pub runewords: Vec<RunewordDef>,
}

/// Definition d'un runeword.
///
/// <!-- @id: sd-data-runeword-def @do: define @role: arpg @layer: 3 @human: miyuk -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunewordDef {
    /// Identifiant unique.
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Sequence de runes requises (noms).
    pub rune_sequence: Vec<String>,
    /// Types d'items autorises.
    pub allowed_types: Vec<String>,
    /// Niveau requis.
    pub required_level: u8,
    /// Proprietes du runeword.
    pub properties: Vec<StatValue>,
}

// =========================================================================
// Runes
// =========================================================================

/// Fichier TOML de runes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunesFile {
    /// Liste des runes.
    pub runes: Vec<RuneDef>,
}

/// Definition d'une rune.
///
/// <!-- @id: sd-data-rune-def @do: define @role: arpg @layer: 3 @human: miyuk -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuneDef {
    /// Identifiant unique (ex: `"lux"`, `"ber"`).
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Numero de la rune (1-33).
    pub number: u8,
    /// Niveau requis.
    pub required_level: u8,
    /// Taille dans l'inventaire.
    pub grid_size: [u8; 2],
    /// Bonus quand inseree dans une arme.
    pub in_weapon: Vec<StatValue>,
    /// Bonus quand inseree dans une armure.
    pub in_armor: Vec<StatValue>,
    /// Bonus quand inseree dans un bouclier.
    pub in_shield: Vec<StatValue>,
    /// Bonus quand inseree dans un casque.
    pub in_helm: Vec<StatValue>,
    /// Rune cible de la recette d'upgrade (`""` = pas d'upgrade).
    pub upgrade_recipe_to: String,
    /// Nombre de runes necessaires pour upgrade.
    pub upgrade_count: u8,
}

// =========================================================================
// Gemmes
// =========================================================================

/// Fichier TOML de gemmes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GemsFile {
    /// Liste des gemmes.
    pub gems: Vec<GemDef>,
}

/// Qualite d'une gemme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GemQuality {
    /// Ebrechee.
    Chipped,
    /// Imparfaite.
    Flawed,
    /// Normale.
    Normal,
    /// Sans defaut.
    Flawless,
    /// Parfaite.
    Perfect,
}

/// Definition d'une gemme.
///
/// <!-- @id: sd-data-gem-def @do: define @role: arpg @layer: 3 @human: miyuk -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GemDef {
    /// Identifiant unique (ex: `"perfect_amethyst"`).
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Type de gemme (ex: `"Amethyst"`, `"Ruby"`).
    pub gem_type: String,
    /// Qualite de la gemme.
    pub quality: GemQuality,
    /// Niveau requis.
    pub required_level: u8,
    /// Taille dans l'inventaire.
    pub grid_size: [u8; 2],
    /// Bonus quand inseree dans une arme.
    pub in_weapon: Vec<StatValue>,
    /// Bonus quand inseree dans une armure.
    pub in_armor: Vec<StatValue>,
    /// Bonus quand inseree dans un casque.
    pub in_helm: Vec<StatValue>,
    /// Bonus quand inseree dans un bouclier.
    pub in_shield: Vec<StatValue>,
}

// =========================================================================
// Charmes
// =========================================================================

/// Definition d'un charme de base.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharmBaseDef {
    /// Identifiant unique.
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Taille dans l'inventaire.
    pub grid_size: [u8; 2],
    /// Nombre maximum d'affixes.
    pub max_affixes: u8,
}

// =========================================================================
// Recettes du Cube Alchimique
// =========================================================================

/// Fichier TOML de recettes du cube.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubeRecipesFile {
    /// Liste des recettes.
    pub recipes: Vec<CubeRecipeDef>,
}

/// Definition d'une recette du cube alchimique.
///
/// <!-- @id: sd-data-cube-recipe @do: define @role: arpg @layer: 3 @human: miyuk -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubeRecipeDef {
    /// Identifiant unique de la recette.
    pub id: String,
    /// Nom affiche.
    pub name: String,
    /// Ingredients requis.
    pub inputs: Vec<CubeInput>,
    /// Resultat de la recette.
    pub output: CubeOutput,
    /// Description de la recette.
    pub description: String,
}

/// Ingredient d'une recette du cube (tagged union).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CubeInput {
    /// Une rune specifique.
    #[serde(rename = "rune")]
    Rune {
        /// ID de la rune.
        id: String,
    },
    /// N runes identiques.
    #[serde(rename = "rune_same")]
    RuneSame {
        /// Nombre requis.
        count: u8,
    },
    /// Une gemme specifique.
    #[serde(rename = "gem")]
    Gem {
        /// ID de la gemme.
        id: String,
    },
    /// N gemmes identiques (meme type et qualite).
    #[serde(rename = "gem_same_type_quality")]
    GemSame {
        /// Nombre requis.
        count: u8,
    },
    /// Un item avec contraintes optionnelles.
    #[serde(rename = "item")]
    Item {
        /// Qualite requise (ex: `"Normal"`, `"Rare"`).
        #[serde(default)]
        quality: Option<String>,
        /// Categorie requise (ex: `"Weapon"`).
        #[serde(default)]
        category: Option<String>,
        /// Type de base requis.
        #[serde(default)]
        base_type: Option<String>,
        /// Tier requis.
        #[serde(default)]
        tier: Option<String>,
    },
    /// Un jewel (sans contrainte).
    #[serde(rename = "jewel")]
    Jewel,
    /// Un scroll specifique.
    #[serde(rename = "scroll")]
    Scroll {
        /// ID du scroll.
        id: String,
    },
}

/// Resultat d'une recette du cube (tagged union).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CubeOutput {
    /// Rune suivante dans la sequence.
    #[serde(rename = "rune_next")]
    RuneNext,
    /// Gemme de qualite superieure.
    #[serde(rename = "gem_next_quality")]
    GemNextQuality,
    /// Modification du dernier ingredient.
    #[serde(rename = "modify_last_input")]
    ModifyLastInput {
        /// Type de modification.
        modification: String,
    },
    /// Re-tirage des affixes.
    #[serde(rename = "reroll_affixes")]
    RerollAffixes {
        /// Qualite cible.
        quality: String,
    },
    /// Upgrade de tier.
    #[serde(rename = "upgrade_tier")]
    UpgradeTier {
        /// Tier cible.
        target_tier: String,
    },
    /// Item craft avec proprietes garanties.
    #[serde(rename = "crafted")]
    Crafted {
        /// Proprietes garanties.
        guaranteed_properties: Vec<StatValue>,
    },
}

// =========================================================================
// Treasure Classes
// =========================================================================

/// Fichier TOML de treasure classes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasureClassFileDef {
    /// Liste des treasure classes.
    pub treasure_classes: Vec<TreasureClassDef>,
}

/// Definition d'une treasure class (table de loot).
///
/// <!-- @id: sd-data-tc-def @do: define @role: arpg @layer: 3 @human: miyuk -->
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasureClassDef {
    /// Identifiant unique.
    pub id: String,
    /// Probabilite de ne rien drop.
    pub no_drop: u32,
    /// Nombre de tirages.
    pub picks: u8,
    /// Entrees de la table.
    pub entries: Vec<TreasureClassEntryDef>,
}

/// Entree dans une treasure class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasureClassEntryDef {
    /// ID de l'item ou d'une autre treasure class.
    pub item_or_tc: String,
    /// Probabilite relative.
    pub probability: u32,
}
