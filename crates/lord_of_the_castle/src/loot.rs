//! Loot — drop Or, XP, Objets à la mort d'un ennemi (Miyukini Survivor).
//!
//! @id: lord_of_the_castle_loot
//! @do: generate_loot_items_and_merchant_pools
//! @role: logic
//! @layer: domain
//! @human: Système de loot (or, XP, objets), identification, marchand, tables armes/armures/accessoires.
//!
//! Formules :
//! - Or : 30%+chance de drop, qté = 50% à 100% des PV du monstre.
//! - XP : 100% drop, qté = 10%(+chance) à 200%(+chance) des PV, min 1.
//! - Objet : 10%(+chance) de drop, type (slot) seulement ; identification plus tard (roll rareté/nom/préfixe/suffixe).

use serde::{Deserialize, Serialize};

/// Type de loot au sol (pixel jaune = or, bleu = xp, marron = objet).
/// Les objets au sol sont non identifiés (slot seulement).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LootKind {
    Gold(u32),
    Xp(u32),
    /// Objet non identifié : seul le type (slot) est connu jusqu’à identification.
    Item(ItemSlot),
}

/// Entrée d’inventaire : objet non identifié (type seulement) ou identifié (nom, rareté, effets).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InventoryEntry {
    Unidentified(ItemSlot),
    Identified(ItemInstance),
}

/// Un drop au sol : position + contenu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootDrop {
    pub x: f32,
    pub y: f32,
    pub kind: LootKind,
}

/// Entrée en vente chez le marchand : objet identifié (avec prix) ou non identifié (slot + 100 or).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantEntry {
    pub entry: InventoryEntry,
    pub price: u32,
}

/// Slots d’équipement du joueur (armure + mains + munitions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemSlot {
    Head,      // Tête
    Neck,      // Collier
    Shoulders, // Épaules
    Bracer,    // Brassard
    Gloves,    // Gants
    Ring1,     // Bague 1
    Ring2,     // Bague 2
    Chest,     // Torse
    Belt,      // Ceinture
    Legs,      // Jambes
    Feet,      // Pieds
    MainHand,  // Main droite (arme 1H, 2H, tir, bouclier)
    OffHand,   // Main gauche (arme 1H, bouclier)
    Ammo,      // Munitions
    Consumable,
}

impl ItemSlot {
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            ItemSlot::Head => "Tête",
            ItemSlot::Neck => "Collier",
            ItemSlot::Shoulders => "Épaules",
            ItemSlot::Bracer => "Brassard",
            ItemSlot::Gloves => "Gants",
            ItemSlot::Ring1 => "Bague 1",
            ItemSlot::Ring2 => "Bague 2",
            ItemSlot::Chest => "Torse",
            ItemSlot::Belt => "Ceinture",
            ItemSlot::Legs => "Jambes",
            ItemSlot::Feet => "Pieds",
            ItemSlot::MainHand => "Main droite",
            ItemSlot::OffHand => "Main gauche",
            ItemSlot::Ammo => "Munitions",
            ItemSlot::Consumable => "Consommable",
        }
    }

    #[must_use]
    pub fn is_weapon_or_shield(&self) -> bool {
        matches!(self, ItemSlot::MainHand | ItemSlot::OffHand)
    }

    #[must_use]
    pub fn is_ammo(&self) -> bool {
        self == &ItemSlot::Ammo
    }

    /// Slots affichés dans la fenêtre Équipement (tous sauf Consumable).
    #[must_use]
    pub fn equipment_slots() -> &'static [ItemSlot] {
        use ItemSlot::{
            Ammo, Belt, Bracer, Chest, Feet, Gloves, Head, Legs, MainHand, Neck, OffHand, Ring1,
            Ring2, Shoulders,
        };
        const SLOTS: [ItemSlot; 14] = [
            Head, Neck, Shoulders, Bracer, Gloves, Ring1, Ring2, Chest, Belt, Legs, Feet, MainHand,
            OffHand, Ammo,
        ];
        &SLOTS
    }
}

/// Rareté : commun (reste) > magique (10%) > rare (5%) > ultra rare (3%) > unique (0%).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Magical,
    UltraRare,
    Unique,
}

impl ItemRarity {
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            ItemRarity::Common => "Commun",
            ItemRarity::Uncommon => "Peu commun",
            ItemRarity::Rare => "Rare",
            ItemRarity::Magical => "Magique",
            ItemRarity::UltraRare => "Ultra rare",
            ItemRarity::Unique => "Unique",
        }
    }

    /// Couleur de carte (affichage) selon la rareté.
    #[must_use]
    pub fn color_rgb(&self) -> (u8, u8, u8) {
        match self {
            ItemRarity::Common => (160, 160, 160),
            ItemRarity::Uncommon => (100, 180, 100),
            ItemRarity::Rare => (80, 120, 220),
            ItemRarity::Magical => (180, 100, 220),
            ItemRarity::UltraRare => (220, 160, 60),
            ItemRarity::Unique => (255, 215, 0),
        }
    }
}

// ——— Armes : préfixe / suffixe / matériau (jet d100, bonus dégâts % et prix %). ———

/// Préfixe arme : table d100, bonus dégâts et prix de vente.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponPrefix {
    Mauvais,    // -20% dmg / -30% prix / 1-10
    Banal,      // -10% / -20% / 11-25
    Aucun,      // 0% / 0% / 26-60
    Bon,        // +10% / +5% / 61-75
    Solide,     // +15% / +10% / 76-84
    Heroique,   // +20% / +25% / 85-95
    Legendaire, // +25% / +50% / 95-100
}

impl WeaponPrefix {
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            WeaponPrefix::Mauvais => "Mauvais",
            WeaponPrefix::Banal => "Banal",
            WeaponPrefix::Aucun => "",
            WeaponPrefix::Bon => "Bon",
            WeaponPrefix::Solide => "Solide",
            WeaponPrefix::Heroique => "Héroïque",
            WeaponPrefix::Legendaire => "Légendaire",
        }
    }
    #[must_use]
    pub fn damage_pct(&self) -> i32 {
        match self {
            WeaponPrefix::Mauvais => -20,
            WeaponPrefix::Banal => -10,
            WeaponPrefix::Aucun => 0,
            WeaponPrefix::Bon => 10,
            WeaponPrefix::Solide => 15,
            WeaponPrefix::Heroique => 20,
            WeaponPrefix::Legendaire => 25,
        }
    }
    #[must_use]
    pub fn price_pct(&self) -> i32 {
        match self {
            WeaponPrefix::Mauvais => -30,
            WeaponPrefix::Banal => -20,
            WeaponPrefix::Aucun => 0,
            WeaponPrefix::Bon => 5,
            WeaponPrefix::Solide => 10,
            WeaponPrefix::Heroique => 25,
            WeaponPrefix::Legendaire => 50,
        }
    }
    /// Multiplicateur prix achat (marchand). Table : Mauvais 0.7, Banal 0.9, Aucun 1.0, Bon 1.2, Solide 2.0, Héroïque 3.0, Légendaire 5.0.
    #[must_use]
    pub fn achat_mult(&self) -> f32 {
        match self {
            WeaponPrefix::Mauvais => 0.7,
            WeaponPrefix::Banal => 0.9,
            WeaponPrefix::Aucun => 1.0,
            WeaponPrefix::Bon => 1.2,
            WeaponPrefix::Solide => 2.0,
            WeaponPrefix::Heroique => 3.0,
            WeaponPrefix::Legendaire => 5.0,
        }
    }
    /// Multiplicateur prix vente. Même valeurs que achat.
    #[must_use]
    pub fn vente_mult(&self) -> f32 {
        self.achat_mult()
    }
}

/// Suffixe arme : table d100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponSuffix {
    Casse,     // cassé 0.2 / 0.2
    Tordu,     // tordu 0.5 / 0.5
    Emousse,   // émoussé 0.8 / 0.8
    Aucun,     // 0% / 0%
    Aiguise,   // aiguisé 1.1 / 1.1
    Travaille, // travaillé 1.2 / 1.2
    Orne,      // orné 1.4 / 1.4
    Leger,     // léger 1.1 / 1.1
    Renforce,  // renforcé 1.3 / 1.3
    Blinde,    // blindé 1.4 / 1.4
    DeMaitre,  // de maître 1.6 / 1.6
    Unique,    // unique 1.8 / 1.8
}

impl WeaponSuffix {
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            WeaponSuffix::Casse => "cassé",
            WeaponSuffix::Tordu => "tordu",
            WeaponSuffix::Emousse => "émoussé",
            WeaponSuffix::Aucun => "",
            WeaponSuffix::Aiguise => "aiguisé",
            WeaponSuffix::Travaille => "travaillé",
            WeaponSuffix::Orne => "orné",
            WeaponSuffix::Leger => "léger",
            WeaponSuffix::Renforce => "renforcé",
            WeaponSuffix::Blinde => "blindé",
            WeaponSuffix::DeMaitre => "de maître",
            WeaponSuffix::Unique => "unique",
        }
    }
    #[must_use]
    pub fn damage_pct(&self) -> i32 {
        match self {
            WeaponSuffix::Casse => -60,
            WeaponSuffix::Tordu => -40,
            WeaponSuffix::Emousse => -10,
            WeaponSuffix::Aucun => 0,
            WeaponSuffix::Aiguise => 5,
            WeaponSuffix::Travaille => 10,
            WeaponSuffix::Orne => -20,
            WeaponSuffix::Leger => -5,
            WeaponSuffix::Renforce => 15,
            WeaponSuffix::Blinde => 20,
            WeaponSuffix::DeMaitre => 25,
            WeaponSuffix::Unique => 30,
        }
    }
    #[must_use]
    pub fn price_pct(&self) -> i32 {
        match self {
            WeaponSuffix::Casse => -30,
            WeaponSuffix::Tordu => -20,
            WeaponSuffix::Emousse => -10,
            WeaponSuffix::Aucun => 0,
            WeaponSuffix::Aiguise => 5,
            WeaponSuffix::Travaille => 10,
            WeaponSuffix::Orne => 50,
            WeaponSuffix::Leger => 5,
            WeaponSuffix::Renforce => 15,
            WeaponSuffix::Blinde => 25,
            WeaponSuffix::DeMaitre => 50,
            WeaponSuffix::Unique => 80,
        }
    }
    /// Multiplicateur achat/vente (table : cassé 0.2 … de maître 1.6, unique 1.8).
    #[must_use]
    pub fn achat_mult(&self) -> f32 {
        match self {
            WeaponSuffix::Casse => 0.2,
            WeaponSuffix::Tordu => 0.5,
            WeaponSuffix::Emousse => 0.8,
            WeaponSuffix::Aucun => 1.0,
            WeaponSuffix::Aiguise => 1.1,
            WeaponSuffix::Travaille => 1.2,
            WeaponSuffix::Orne => 1.4,
            WeaponSuffix::Leger => 1.1,
            WeaponSuffix::Renforce => 1.3,
            WeaponSuffix::Blinde => 1.4,
            WeaponSuffix::DeMaitre => 1.6,
            WeaponSuffix::Unique => 1.8,
        }
    }
    #[must_use]
    pub fn vente_mult(&self) -> f32 {
        self.achat_mult()
    }
}

/// Matériau arme : [préfixe][nom][suffixe] en [matériau]. Table d100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponMaterial {
    Bois,     // 0.5 / 0.5
    Cuivre,   // 0.5 / 0.5
    Bronze,   // 0.7 / 0.7
    Fer,      // 1.0 / 1.0
    Acier,    // 1.2 / 1.2
    Argent,   // 1.3 / 1.3
    Or,       // 1.6 / 1.6
    Adamante, // 2.0 / 2.0
}

impl WeaponMaterial {
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            WeaponMaterial::Bois => "bois",
            WeaponMaterial::Cuivre => "cuivre",
            WeaponMaterial::Bronze => "bronze",
            WeaponMaterial::Fer => "fer",
            WeaponMaterial::Acier => "acier",
            WeaponMaterial::Argent => "argent",
            WeaponMaterial::Or => "or",
            WeaponMaterial::Adamante => "adamante",
        }
    }
    #[must_use]
    pub fn damage_pct(&self) -> i32 {
        match self {
            WeaponMaterial::Bois => -20,
            WeaponMaterial::Cuivre => -10,
            WeaponMaterial::Bronze => -5,
            WeaponMaterial::Fer => 0,
            WeaponMaterial::Acier => 10,
            WeaponMaterial::Argent => 15,
            WeaponMaterial::Or => 20,
            WeaponMaterial::Adamante => 25,
        }
    }
    /// Multiplicateur achat/vente (table : bois 0.5 … adamante 2.0).
    #[must_use]
    pub fn achat_mult(&self) -> f32 {
        match self {
            WeaponMaterial::Bois => 0.5,
            WeaponMaterial::Cuivre => 0.5,
            WeaponMaterial::Bronze => 0.7,
            WeaponMaterial::Fer => 1.0,
            WeaponMaterial::Acier => 1.2,
            WeaponMaterial::Argent => 1.3,
            WeaponMaterial::Or => 1.6,
            WeaponMaterial::Adamante => 2.0,
        }
    }
    #[must_use]
    pub fn vente_mult(&self) -> f32 {
        self.achat_mult()
    }
}

// ——— Armure : préfixe / suffixe / matériau (multiplicateur armure). Réduction plate, dégâts min = 1. ———

/// Préfixe armure : multiplicateur sur la valeur d’armure de base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmorPrefix {
    Mauvais,    // 0.5
    Banal,      // 0.9
    Aucun,      // 1.0
    Bon,        // 1.1
    Solide,     // 1.2
    Heroique,   // 1.3
    Legendaire, // 1.5
}

impl ArmorPrefix {
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            ArmorPrefix::Mauvais => "Mauvais",
            ArmorPrefix::Banal => "Banal",
            ArmorPrefix::Aucun => "",
            ArmorPrefix::Bon => "Bon",
            ArmorPrefix::Solide => "Solide",
            ArmorPrefix::Heroique => "Héroïque",
            ArmorPrefix::Legendaire => "Légendaire",
        }
    }
    #[must_use]
    pub fn mult(&self) -> f32 {
        match self {
            ArmorPrefix::Mauvais => 0.5,
            ArmorPrefix::Banal => 0.9,
            ArmorPrefix::Aucun => 1.0,
            ArmorPrefix::Bon => 1.1,
            ArmorPrefix::Solide => 1.2,
            ArmorPrefix::Heroique => 1.3,
            ArmorPrefix::Legendaire => 1.5,
        }
    }
    /// Multiplicateur prix achat/vente (tableau-armure.csv).
    #[must_use]
    pub fn achat_mult(&self) -> f32 {
        match self {
            ArmorPrefix::Aucun => 1.0,
            ArmorPrefix::Mauvais => 0.7,
            ArmorPrefix::Banal => 0.9,
            ArmorPrefix::Bon => 1.2,
            ArmorPrefix::Solide => 2.0,
            ArmorPrefix::Heroique => 3.0,
            ArmorPrefix::Legendaire => 5.0,
        }
    }
    #[must_use]
    pub fn vente_mult(&self) -> f32 {
        self.achat_mult()
    }
}

/// Suffixe armure : multiplicateur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmorSuffix {
    Casse,     // 0.5
    Tordu,     // 0.75
    Emousse,   // 0.9
    Aucun,     // 1.0
    Aiguise,   // 1.1
    Travaille, // 1.1
    Orne,      // 1.2
    Leger,     // 0.9
    Renforce,  // 1.3
    Blinde,    // 1.4
    DeMaitre,  // 1.5
    Unique,    // 1.8 (tableau-armure)
}

impl ArmorSuffix {
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            ArmorSuffix::Casse => "cassé",
            ArmorSuffix::Tordu => "tordu",
            ArmorSuffix::Emousse => "émoussé",
            ArmorSuffix::Aucun => "",
            ArmorSuffix::Aiguise => "aiguisé",
            ArmorSuffix::Travaille => "travaillé",
            ArmorSuffix::Orne => "orné",
            ArmorSuffix::Leger => "léger",
            ArmorSuffix::Renforce => "renforcé",
            ArmorSuffix::Blinde => "blindé",
            ArmorSuffix::DeMaitre => "de maître",
            ArmorSuffix::Unique => "unique",
        }
    }
    #[must_use]
    pub fn mult(&self) -> f32 {
        match self {
            ArmorSuffix::Casse => 0.5,
            ArmorSuffix::Tordu => 0.75,
            ArmorSuffix::Emousse => 0.9,
            ArmorSuffix::Aucun => 1.0,
            ArmorSuffix::Aiguise => 1.1,
            ArmorSuffix::Travaille => 1.1,
            ArmorSuffix::Orne => 1.2,
            ArmorSuffix::Leger => 0.9,
            ArmorSuffix::Renforce => 1.3,
            ArmorSuffix::Blinde => 1.4,
            ArmorSuffix::DeMaitre => 1.5,
            ArmorSuffix::Unique => 1.8,
        }
    }
    /// Multiplicateur prix achat/vente (tableau-armure.csv).
    #[must_use]
    pub fn achat_mult(&self) -> f32 {
        match self {
            ArmorSuffix::Aucun => 1.0,
            ArmorSuffix::Casse => 0.2,
            ArmorSuffix::Tordu => 0.5,
            ArmorSuffix::Emousse => 0.8,
            ArmorSuffix::Aiguise => 1.1,
            ArmorSuffix::Travaille => 1.2,
            ArmorSuffix::Orne => 1.4,
            ArmorSuffix::Leger => 1.1,
            ArmorSuffix::Renforce => 1.3,
            ArmorSuffix::Blinde => 1.4,
            ArmorSuffix::DeMaitre => 1.6,
            ArmorSuffix::Unique => 1.8,
        }
    }
    #[must_use]
    pub fn vente_mult(&self) -> f32 {
        self.achat_mult()
    }
}

/// Matériau armure : multiplicateur. (Vide = 1.0 ; or = 1.0.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmorMaterial {
    Bois,     // 0.4
    Cuivre,   // 0.6
    Bronze,   // 0.8
    Fer,      // 1.0
    Acier,    // 1.2
    Argent,   // 1.3
    Or,       // 1.0
    Adamante, // 1.5
}

impl ArmorMaterial {
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            ArmorMaterial::Bois => "bois",
            ArmorMaterial::Cuivre => "cuivre",
            ArmorMaterial::Bronze => "bronze",
            ArmorMaterial::Fer => "fer",
            ArmorMaterial::Acier => "acier",
            ArmorMaterial::Argent => "argent",
            ArmorMaterial::Or => "or",
            ArmorMaterial::Adamante => "adamante",
        }
    }
    #[must_use]
    pub fn mult(&self) -> f32 {
        match self {
            ArmorMaterial::Bois => 0.4,
            ArmorMaterial::Cuivre => 0.6,
            ArmorMaterial::Bronze => 0.8,
            ArmorMaterial::Fer => 1.0,
            ArmorMaterial::Acier => 1.2,
            ArmorMaterial::Argent => 1.3,
            ArmorMaterial::Or => 1.0,
            ArmorMaterial::Adamante => 1.5,
        }
    }
    /// Multiplicateur prix achat/vente (tableau-armure.csv).
    #[must_use]
    pub fn achat_mult(&self) -> f32 {
        match self {
            ArmorMaterial::Bois => 0.5,
            ArmorMaterial::Cuivre => 0.5,
            ArmorMaterial::Bronze => 0.7,
            ArmorMaterial::Fer => 1.0,
            ArmorMaterial::Acier => 1.2,
            ArmorMaterial::Argent => 1.3,
            ArmorMaterial::Or => 1.6,
            ArmorMaterial::Adamante => 2.0,
        }
    }
    #[must_use]
    pub fn vente_mult(&self) -> f32 {
        self.achat_mult()
    }
}

/// Préfixe armure + Dispo% (tableau-armure.csv). Ordre pour retry « ligne suivante ».
const ARMOR_PREFIX_DISPO: &[(ArmorPrefix, f32)] = &[
    (ArmorPrefix::Aucun, 15.0),
    (ArmorPrefix::Mauvais, 30.0),
    (ArmorPrefix::Banal, 40.0),
    (ArmorPrefix::Bon, 25.0),
    (ArmorPrefix::Solide, 10.0),
    (ArmorPrefix::Heroique, 5.0),
    (ArmorPrefix::Legendaire, 100.0),
];

/// Suffixe armure + Dispo%.
const ARMOR_SUFFIX_DISPO: &[(ArmorSuffix, f32)] = &[
    (ArmorSuffix::Aucun, 5.0),
    (ArmorSuffix::Casse, 10.0),
    (ArmorSuffix::Tordu, 20.0),
    (ArmorSuffix::Emousse, 30.0),
    (ArmorSuffix::Aiguise, 20.0),
    (ArmorSuffix::Travaille, 10.0),
    (ArmorSuffix::Orne, 20.0),
    (ArmorSuffix::Leger, 25.0),
    (ArmorSuffix::Renforce, 10.0),
    (ArmorSuffix::Blinde, 5.0),
    (ArmorSuffix::DeMaitre, 5.0),
    (ArmorSuffix::Unique, 100.0),
];

/// Matériau armure + Dispo%.
const ARMOR_MATERIAL_DISPO: &[(ArmorMaterial, f32)] = &[
    (ArmorMaterial::Bois, 25.0),
    (ArmorMaterial::Cuivre, 40.0),
    (ArmorMaterial::Bronze, 50.0),
    (ArmorMaterial::Fer, 40.0),
    (ArmorMaterial::Acier, 20.0),
    (ArmorMaterial::Argent, 10.0),
    (ArmorMaterial::Or, 5.0),
    (ArmorMaterial::Adamante, 100.0),
];

/// Instance d'objet identifiée (armure ou arme : [préfixe][nom][suffixe] en [matériau] pour armes).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemInstance {
    /// Nom affiché.
    pub display_name: String,
    /// Slot équipement.
    pub slot: ItemSlot,
    /// Rareté.
    pub rarity: ItemRarity,
    /// Préfixe (texte générique, armure / ancien système).
    pub prefix: Option<String>,
    /// Suffixe (texte générique).
    pub suffix: Option<String>,
    /// Pour armes : préfixe table d100 (bonus dégâts/prix).
    pub weapon_prefix: Option<WeaponPrefix>,
    /// Pour armes : suffixe table d100.
    pub weapon_suffix: Option<WeaponSuffix>,
    /// Pour armes : matériau (nom en "en [matériau]").
    pub weapon_material: Option<WeaponMaterial>,
    /// Prix d'achat de base (table arme/munition). Rempli à la génération ; anciennes sauvegardes peuvent l'avoir vide.
    #[serde(default)]
    pub base_buy: Option<u32>,
    /// Prix de vente de base (table arme/munition). Utilisé pour sell_price quand présent.
    #[serde(default)]
    pub base_sell: Option<u32>,
    /// Dégâts de base (armes 1H listées par le spec; min 1).
    pub base_damage: Option<i32>,
    /// Arme à distance (true) ou CàC (false). Ignoré si pas une arme.
    #[serde(default)]
    pub is_ranged: bool,
    /// Chance de blocage base (boucliers : 7%+agi, 10%+agi, etc.).
    pub block_chance_base: Option<f32>,
    /// Armure : réduction plate de base (nom d’objet). Dégâts subis min = 1.
    pub base_armor: Option<i32>,
    /// Préfixe armure (multiplicateur).
    pub armor_prefix: Option<ArmorPrefix>,
    /// Suffixe armure (multiplicateur).
    pub armor_suffix: Option<ArmorSuffix>,
    /// Matériau armure (multiplicateur).
    pub armor_material: Option<ArmorMaterial>,
}

impl ItemInstance {
    /// Épée courte de départ (équipée par défaut en main droite). Table : 98 achat, 49 vente.
    #[must_use]
    pub fn default_short_sword() -> Self {
        Self {
            display_name: "Épée courte".to_string(),
            slot: ItemSlot::MainHand,
            rarity: ItemRarity::Common,
            prefix: None,
            suffix: None,
            weapon_prefix: None,
            weapon_suffix: None,
            weapon_material: None,
            base_buy: Some(98),
            base_sell: Some(49),
            base_damage: Some(4),
            is_ranged: false,
            block_chance_base: None,
            base_armor: None,
            armor_prefix: None,
            armor_suffix: None,
            armor_material: None,
        }
    }

    /// Potion de vie (marchand / consommable).
    #[must_use]
    pub fn potion_vie() -> Self {
        Self {
            display_name: "Potion de vie".to_string(),
            slot: ItemSlot::Consumable,
            rarity: ItemRarity::Common,
            prefix: None,
            suffix: None,
            weapon_prefix: None,
            weapon_suffix: None,
            weapon_material: None,
            base_buy: None,
            base_sell: None,
            base_damage: None,
            is_ranged: false,
            block_chance_base: None,
            base_armor: None,
            armor_prefix: None,
            armor_suffix: None,
            armor_material: None,
        }
    }

    /// Potion de mana (marchand / consommable).
    #[must_use]
    pub fn potion_mana() -> Self {
        Self {
            display_name: "Potion de mana".to_string(),
            slot: ItemSlot::Consumable,
            rarity: ItemRarity::Common,
            prefix: None,
            suffix: None,
            weapon_prefix: None,
            weapon_suffix: None,
            weapon_material: None,
            base_buy: None,
            base_sell: None,
            base_damage: None,
            is_ranged: false,
            block_chance_base: None,
            base_armor: None,
            armor_prefix: None,
            armor_suffix: None,
            armor_material: None,
        }
    }

    /// Dégâts effectifs de l’arme (base × (100 + bonus %)/100, min 1). None si pas une arme.
    #[must_use]
    pub fn effective_weapon_damage(&self) -> Option<i32> {
        let base = self.base_damage?;
        let pct = self.weapon_damage_pct_bonus();
        let dmg = base * (100 + pct) / 100;
        Some(dmg.max(1))
    }

    /// Multiplicateur prix de vente (bonus % préfixe + suffixe arme ; matériau n’affecte pas le prix).
    fn price_multiplier_pct(&self) -> i32 {
        let mut pct = 0i32;
        if let Some(wp) = self.weapon_prefix {
            pct += wp.price_pct();
        }
        if let Some(ws) = self.weapon_suffix {
            pct += ws.price_pct();
        }
        pct
    }

    /// Bonus dégâts total % (préfixe + suffixe + matériau) pour armes.
    #[must_use]
    pub fn weapon_damage_pct_bonus(&self) -> i32 {
        let mut pct = 0i32;
        if let Some(wp) = self.weapon_prefix {
            pct += wp.damage_pct();
        }
        if let Some(ws) = self.weapon_suffix {
            pct += ws.damage_pct();
        }
        if let Some(wm) = self.weapon_material {
            pct += wm.damage_pct();
        }
        pct
    }

    /// Prix de vente (or). Si base_sell présent (arme/munition/armure) : base_sell × préfixe × suffixe × matériau ; sinon ancienne formule slot/rareté.
    #[must_use]
    pub fn sell_price(&self) -> u32 {
        if let Some(base) = self.base_sell {
            if self.weapon_prefix.is_some()
                || self.weapon_suffix.is_some()
                || self.weapon_material.is_some()
            {
                let wp = self.weapon_prefix.map_or(1.0, |p| p.vente_mult());
                let ws = self.weapon_suffix.map_or(1.0, |s| s.vente_mult());
                let wm = self.weapon_material.map_or(1.0, |m| m.vente_mult());
                let p = (base as f32 * wp * ws * wm).round().max(0.0) as u32;
                return p.max(1);
            }
            if self.armor_prefix.is_some()
                || self.armor_suffix.is_some()
                || self.armor_material.is_some()
            {
                let ap = self.armor_prefix.map_or(1.0, |p| p.vente_mult());
                let as_ = self.armor_suffix.map_or(1.0, |s| s.vente_mult());
                let am = self.armor_material.map_or(1.0, |m| m.vente_mult());
                let p = (base as f32 * ap * as_ * am).round().max(0.0) as u32;
                return p.max(1);
            }
            return base.max(1);
        }
        let base = match self.slot {
            ItemSlot::Head | ItemSlot::Feet | ItemSlot::Gloves | ItemSlot::Bracer => 15,
            ItemSlot::Neck | ItemSlot::Shoulders | ItemSlot::Belt | ItemSlot::Legs => 20,
            ItemSlot::Ring1 | ItemSlot::Ring2 => 25,
            ItemSlot::Chest => 30,
            ItemSlot::MainHand | ItemSlot::OffHand => 35,
            ItemSlot::Ammo => 8,
            ItemSlot::Consumable => 10,
        };
        let mult = match self.rarity {
            ItemRarity::Common => 1,
            ItemRarity::Uncommon => 2,
            ItemRarity::Rare => 4,
            ItemRarity::Magical => 8,
            ItemRarity::UltraRare => 15,
            ItemRarity::Unique => 30,
        };
        let pct_bonus = self.price_multiplier_pct();
        let price = base * mult;
        let with_bonus = price + (price * pct_bonus / 100);
        with_bonus.max(1) as u32
    }

    /// Prix d'achat (marchand). Si base_buy présent : base_buy × préfixe × suffixe × matériau (arme ou armure) ; sinon 2× sell_price (fallback).
    #[must_use]
    pub fn buy_price(&self) -> u32 {
        if let Some(base) = self.base_buy {
            if self.weapon_prefix.is_some()
                || self.weapon_suffix.is_some()
                || self.weapon_material.is_some()
            {
                let wp = self.weapon_prefix.map_or(1.0, |p| p.achat_mult());
                let ws = self.weapon_suffix.map_or(1.0, |s| s.achat_mult());
                let wm = self.weapon_material.map_or(1.0, |m| m.achat_mult());
                let p = (base as f32 * wp * ws * wm).round().max(0.0) as u32;
                return p.max(1);
            }
            if self.armor_prefix.is_some()
                || self.armor_suffix.is_some()
                || self.armor_material.is_some()
            {
                let ap = self.armor_prefix.map_or(1.0, |p| p.achat_mult());
                let as_ = self.armor_suffix.map_or(1.0, |s| s.achat_mult());
                let am = self.armor_material.map_or(1.0, |m| m.achat_mult());
                let p = (base as f32 * ap * as_ * am).round().max(0.0) as u32;
                return p.max(1);
            }
            return base.max(1);
        }
        self.sell_price().saturating_mul(2)
    }

    /// Armure effective : base × préfixe × suffixe × matériau (réduction plate ; dégâts min subis = 1).
    #[must_use]
    pub fn effective_armor(&self) -> i32 {
        let Some(base) = self.base_armor else {
            return 0;
        };
        let p = self.armor_prefix.map_or(1.0, |a| a.mult());
        let s = self.armor_suffix.map_or(1.0, |a| a.mult());
        let m = self.armor_material.map_or(1.0, |a| a.mult());
        (base as f32 * p * s * m).round().max(0.0) as i32
    }

    /// Texte des effets (préfixe/suffixe arme ou générique ou armure).
    #[must_use]
    pub fn effects_text(&self) -> String {
        if self.slot.is_weapon_or_shield() {
            let mut parts = Vec::new();
            let dmg_pct = self.weapon_damage_pct_bonus();
            if dmg_pct != 0 {
                parts.push(format!("Dégâts : {dmg_pct:+}%"));
            }
            if let Some(block) = self.block_chance_base {
                parts.push(format!("Blocage : {block}% (base)"));
            }
            if let Some(dmg) = self.base_damage {
                parts.push(format!("Dégâts de base : {dmg}"));
            }
            if parts.is_empty() {
                "Aucun effet particulier.".to_string()
            } else {
                parts.join(" — ")
            }
        } else if self.base_armor.is_some() {
            let arm = self.effective_armor();
            format!("Armure : {arm} (réduction plate, dégâts min subis = 1)")
        } else {
            let mut parts = Vec::new();
            if let Some(ref p) = self.prefix {
                parts.push(format!("Préfixe: {p}"));
            }
            if let Some(ref s) = self.suffix {
                parts.push(format!("Suffixe: {s}"));
            }
            if parts.is_empty() {
                "Aucun effet particulier.".to_string()
            } else {
                parts.join(" — ")
            }
        }
    }
}

/// Génère le loot à la mort d'un monstre (position, hp_max du monstre, chance = bonus % joueur).
/// Utilise roll() pour les tirages (0.0..1.0).
pub fn generate_loot(
    x: f32,
    y: f32,
    monster_hp_max: i32,
    chance_pct: i32,
    roll: &mut impl FnMut() -> f32,
) -> Vec<LootDrop> {
    let mut drops = Vec::new();
    let hp = monster_hp_max.max(1) as f32;
    let chance = (chance_pct as f32) / 100.0;

    // Offsets pour que les pixels or/xp/objet ne se superposent pas (6 px d'écart).
    const DROP_OFFSET: f32 = 8.0;

    // Or : 30% + chance. Qté = 50% à 100% des PV (entier).
    let gold_chance = (30.0 + chance_pct as f32) / 100.0;
    if roll() < gold_chance {
        let t = 0.5 + roll() * 0.5; // 50% à 100%
        let qty = (hp * t).max(1.0) as u32;
        drops.push(LootDrop {
            x: x - DROP_OFFSET,
            y,
            kind: LootKind::Gold(qty),
        });
    }

    // XP : 100%. Qté = (10%+chance) à (200%+chance) des PV, minimum 1.
    let xp_min_factor = 0.10 + chance;
    let xp_max_factor = 2.0 + chance;
    let xp_min = (hp * xp_min_factor).max(1.0) as u32;
    let xp_max = (hp * xp_max_factor).max(1.0) as u32;
    let xp_qty = if xp_min >= xp_max {
        xp_min
    } else {
        xp_min + (roll() * (xp_max - xp_min + 1) as f32) as u32
    };
    drops.push(LootDrop {
        x,
        y,
        kind: LootKind::Xp(xp_qty.max(1)),
    });

    // Objet : 10% + chance. Seul le type (slot) est déterminé ; identification plus tard.
    let item_chance = (10.0 + chance_pct as f32) / 100.0;
    if roll() < item_chance {
        let slot = roll_slot(roll);
        drops.push(LootDrop {
            x: x + DROP_OFFSET,
            y,
            kind: LootKind::Item(slot),
        });
    }

    drops
}

/// Identification par soi-même : bonus luck sur le jet de rareté, bonus sagesse sur préfixe/nom/suffixe.
pub fn roll_identification_self(
    roll: &mut impl FnMut() -> f32,
    luck_pct: i32,
    wisdom_pct: i32,
    slot: ItemSlot,
) -> ItemInstance {
    let luck = (luck_pct as f32 / 100.0).min(1.0);
    let wisdom = (wisdom_pct as f32 / 100.0).min(1.0);
    let rarity = roll_rarity_identification(roll, luck);

    if slot.is_weapon_or_shield() {
        let wp = roll_weapon_prefix(roll);
        let ws = roll_weapon_suffix(roll);
        let wm = roll_weapon_material(roll);
        let (base_name, base_damage, block_chance_base, is_ranged, base_buy, base_sell) =
            roll_weapon_base(slot, roll);
        let display_name = build_weapon_display_name(&wp, &base_name, &ws, &wm);
        ItemInstance {
            display_name,
            slot,
            rarity,
            prefix: None,
            suffix: None,
            weapon_prefix: Some(wp),
            weapon_suffix: Some(ws),
            weapon_material: Some(wm),
            base_buy,
            base_sell,
            base_damage,
            is_ranged,
            block_chance_base,
            base_armor: None,
            armor_prefix: None,
            armor_suffix: None,
            armor_material: None,
        }
    } else if slot.is_ammo() {
        let wm = roll_weapon_material(roll);
        let (base_name, _, _, _, base_buy, base_sell) = roll_weapon_base(slot, roll);
        let display_name = format!("{} en {}", base_name, wm.label());
        ItemInstance {
            display_name,
            slot,
            rarity,
            prefix: None,
            suffix: None,
            weapon_prefix: None,
            weapon_suffix: None,
            weapon_material: Some(wm),
            base_buy,
            base_sell,
            base_damage: None,
            is_ranged: false,
            block_chance_base: None,
            base_armor: None,
            armor_prefix: None,
            armor_suffix: None,
            armor_material: None,
        }
    } else if matches!(
        slot,
        ItemSlot::Chest
            | ItemSlot::Head
            | ItemSlot::Gloves
            | ItemSlot::Shoulders
            | ItemSlot::Legs
            | ItemSlot::Feet
    ) {
        let ap = roll_armor_prefix(roll);
        let as_ = roll_armor_suffix(roll);
        let am = roll_armor_material(roll);
        let (base_name, base_armor) =
            roll_armor_base(slot, roll).unwrap_or_else(|| (String::from("Objet"), 0));
        let display_name = build_armor_display_name(&ap, &base_name, &as_, &am);
        ItemInstance {
            display_name,
            slot,
            rarity,
            prefix: None,
            suffix: None,
            weapon_prefix: None,
            weapon_suffix: None,
            weapon_material: None,
            base_buy: None,
            base_sell: None,
            base_damage: None,
            block_chance_base: None,
            base_armor: Some(base_armor),
            armor_prefix: Some(ap),
            armor_suffix: Some(as_),
            armor_material: Some(am),
            is_ranged: false,
        }
    } else {
        let (base_name, _) = roll_base_item_identification(slot, rarity, roll, wisdom)
            .unwrap_or_else(|| (String::from("Objet"), slot));
        let prefix = if roll() < 0.3 + wisdom * 0.2 {
            Some(roll_prefix(roll))
        } else {
            None
        };
        let suffix = if roll() < 0.3 + wisdom * 0.2 {
            Some(roll_suffix(&base_name, roll))
        } else {
            None
        };
        let display_name = build_display_name(&base_name, prefix.as_deref(), suffix.as_deref());
        ItemInstance {
            display_name,
            slot,
            rarity,
            prefix,
            suffix,
            weapon_prefix: None,
            weapon_suffix: None,
            weapon_material: None,
            base_buy: None,
            base_sell: None,
            base_damage: None,
            is_ranged: false,
            block_chance_base: None,
            base_armor: None,
            armor_prefix: None,
            armor_suffix: None,
            armor_material: None,
        }
    }
}

/// Identification par un expert : pas de bonus, même roll.
pub fn roll_identification_expert(roll: &mut impl FnMut() -> f32, slot: ItemSlot) -> ItemInstance {
    roll_identification_self(roll, 0, 0, slot)
}

/// Jet de rareté à l’identification : unique (0%) > ultra rare (3%) > rare (5%) > magique (10%) > commun.
/// Bonus luck : ajouté au jet (effective_roll = roll*100 + luck, cap 100).
fn roll_rarity_identification(roll: &mut impl FnMut() -> f32, luck_bonus: f32) -> ItemRarity {
    let raw = roll();
    let effective = (raw * 100.0 + luck_bonus * 100.0).min(100.0);
    if effective >= 100.0 {
        ItemRarity::Unique
    } else if effective >= 97.0 {
        ItemRarity::UltraRare
    } else if effective >= 92.0 {
        ItemRarity::Rare
    } else if effective >= 82.0 {
        ItemRarity::Magical
    } else if effective >= 60.0 {
        ItemRarity::Uncommon
    } else {
        ItemRarity::Common
    }
}

fn roll_slot(roll: &mut impl FnMut() -> f32) -> ItemSlot {
    let r = roll();
    if r < 0.08 {
        ItemSlot::Head
    } else if r < 0.12 {
        ItemSlot::Neck
    } else if r < 0.16 {
        ItemSlot::Shoulders
    } else if r < 0.20 {
        ItemSlot::Bracer
    } else if r < 0.24 {
        ItemSlot::Gloves
    } else if r < 0.28 {
        ItemSlot::Ring1
    } else if r < 0.32 {
        ItemSlot::Ring2
    } else if r < 0.40 {
        ItemSlot::Chest
    } else if r < 0.44 {
        ItemSlot::Belt
    } else if r < 0.50 {
        ItemSlot::Legs
    } else if r < 0.56 {
        ItemSlot::Feet
    } else if r < 0.72 {
        ItemSlot::MainHand
    } else if r < 0.82 {
        ItemSlot::OffHand
    } else if r < 0.88 {
        ItemSlot::Ammo
    } else {
        ItemSlot::Consumable
    }
}

/// Jet d100 (0-99) à partir de roll() dans [0, 1).
fn roll_d100(roll: &mut impl FnMut() -> f32) -> u32 {
    (roll() * 100.0).min(99.0) as u32
}

fn roll_weapon_prefix(roll: &mut impl FnMut() -> f32) -> WeaponPrefix {
    let d = roll_d100(roll);
    if d < 10 {
        WeaponPrefix::Mauvais
    } else if d < 25 {
        WeaponPrefix::Banal
    } else if d < 60 {
        WeaponPrefix::Aucun
    } else if d < 75 {
        WeaponPrefix::Bon
    } else if d < 84 {
        WeaponPrefix::Solide
    } else if d < 95 {
        WeaponPrefix::Heroique
    } else {
        WeaponPrefix::Legendaire
    }
}

fn roll_weapon_suffix(roll: &mut impl FnMut() -> f32) -> WeaponSuffix {
    let d = roll_d100(roll);
    if d < 8 {
        WeaponSuffix::Casse
    } else if d < 15 {
        WeaponSuffix::Tordu
    } else if d < 20 {
        WeaponSuffix::Emousse
    } else if d < 43 {
        WeaponSuffix::Aucun
    } else if d < 50 {
        WeaponSuffix::Aiguise
    } else if d < 56 {
        WeaponSuffix::Travaille
    } else if d < 60 {
        WeaponSuffix::Orne
    } else if d < 75 {
        WeaponSuffix::Leger
    } else if d < 84 {
        WeaponSuffix::Renforce
    } else if d < 95 {
        WeaponSuffix::Blinde
    } else if d < 99 {
        WeaponSuffix::DeMaitre
    } else {
        WeaponSuffix::Unique
    }
}

fn roll_weapon_material(roll: &mut impl FnMut() -> f32) -> WeaponMaterial {
    let d = roll_d100(roll);
    if d < 10 {
        WeaponMaterial::Bois
    } else if d < 20 {
        WeaponMaterial::Cuivre
    } else if d < 30 {
        WeaponMaterial::Bronze
    } else if d < 75 {
        WeaponMaterial::Fer
    } else if d < 85 {
        WeaponMaterial::Acier
    } else if d < 92 {
        WeaponMaterial::Argent
    } else if d < 97 {
        WeaponMaterial::Or
    } else {
        WeaponMaterial::Adamante
    }
}

/// Entrée arme : nom, dégâts base, prix achat, prix vente, dispo% (poids tirage).
struct WeaponEntry(&'static str, i32, u32, u32, u8);

/// Armes à une main (nom, dmg, achat, vente, dispo%). Source : tableau-armes.csv.
const ONE_HAND_WEAPONS: &[WeaponEntry] = &[
    WeaponEntry("Dague", 5, 70, 35, 60),
    WeaponEntry("Hachoir", 8, 70, 35, 50),
    WeaponEntry("Hachette", 9, 112, 56, 40),
    WeaponEntry("Hache", 10, 126, 63, 50),
    WeaponEntry("Hache d'arme", 13, 140, 70, 30),
    WeaponEntry("Bec de Corbin", 8, 140, 70, 10),
    WeaponEntry("Fléau", 12, 168, 84, 40),
    WeaponEntry("Marteau", 9, 126, 63, 40),
    WeaponEntry("Masse", 12, 140, 70, 30),
    WeaponEntry("Cimeterre", 10, 140, 70, 20),
    WeaponEntry("Épée courte", 9, 98, 49, 50),
    WeaponEntry("Épée longue", 13, 140, 70, 40),
    WeaponEntry("Sabre", 10, 140, 70, 30),
    WeaponEntry("Fleuret", 9, 98, 49, 30),
    WeaponEntry("Rapière", 13, 140, 70, 10),
];

/// Armes à deux mains (nom, dmg, achat, vente, dispo%). Source : tableau-armes.csv.
const TWO_HAND_WEAPONS: &[WeaponEntry] = &[
    WeaponEntry("Hache à deux mains", 22, 224, 112, 20),
    WeaponEntry("Maul", 18, 210, 105, 10),
    WeaponEntry("Fléau à deux mains", 21, 196, 98, 20),
    WeaponEntry("Épée à deux mains", 20, 210, 105, 40),
];

/// Armes d'hast (nom, dmg, achat, vente, dispo%). Source : tableau-armes.csv.
const HAST_WEAPONS: &[WeaponEntry] = &[
    WeaponEntry("Bardiche", 23, 224, 112, 10),
    WeaponEntry("Faux de guerre", 17, 196, 98, 15),
    WeaponEntry("Hallebarde", 22, 224, 112, 30),
    WeaponEntry("Lance", 12, 98, 49, 50),
    WeaponEntry("Lance d'arçon", 18, 112, 56, 30),
    WeaponEntry("Lance de cavalerie", 20, 196, 98, 40),
    WeaponEntry("Pique", 16, 126, 63, 40),
    WeaponEntry("Trident", 15, 140, 70, 10),
];

/// Armes de tir (nom, dmg, achat, vente, dispo%). Source : tableau-armes.csv.
const RANGED_WEAPONS: &[WeaponEntry] = &[
    WeaponEntry("Couteau de lancer", 7, 28, 14, 25),
    WeaponEntry("Hache de lancer", 9, 34, 17, 30),
    WeaponEntry("Javelot", 10, 39, 20, 30),
    WeaponEntry("Arc court", 7, 67, 34, 40),
    WeaponEntry("Arc composite", 10, 90, 45, 30),
    WeaponEntry("Arc long", 11, 101, 51, 40),
    WeaponEntry("Arc long composite", 13, 123, 62, 20),
    WeaponEntry("Arbalète", 16, 112, 56, 40),
    WeaponEntry("Arbalète lourde", 18, 157, 79, 30),
    WeaponEntry("Arbalète à répétition", 15, 78, 39, 20),
    WeaponEntry("Rabateur", 9, 90, 45, 50),
    WeaponEntry("Pistolet à silex", 13, 123, 62, 40),
    WeaponEntry("Revolver", 11, 146, 73, 30),
    WeaponEntry("Pistolet automatique", 9, 178, 89, 20),
    WeaponEntry("Fusil de chasse", 14, 203, 102, 40),
    WeaponEntry("Fusil à culasse", 14, 236, 118, 30),
];

/// Munitions : (nom, achat, vente, dispo%). Source : tableau-armes.csv. Vente 0.01 = 0 or, 0.5 = 1 or arrondi.
const AMMO_ENTRIES: &[(&str, u32, u32, u8)] = &[
    ("Flèches", 1, 0, 100),  // vente 0.01 -> 0 or
    ("Carreaux", 2, 1, 100), // vente 0.5 -> 1 or
    ("9mm", 3, 2, 50),
    ("365 magnum", 5, 3, 40),
    ("Cal12", 6, 4, 30),
    ("Cartouche poudre noir", 7, 4, 40),
];

/// Boucliers : (nom, blocage %, achat, vente, dispo%). Tableau armure CSV.
const SHIELD_ENTRIES: &[(&str, f32, u32, u32, u8)] = &[
    ("Targe", 7.0, 80, 20, 50),
    ("Rondache", 10.0, 100, 25, 40),
    ("Ecu", 13.0, 200, 50, 30),
    ("Grand Bouclier", 15.0, 300, 75, 20),
    ("Egide", 18.0, 400, 100, 20),
    ("Pavois", 20.0, 500, 125, 15),
];

/// Préfixe arme + Dispo% (tableau-armes.csv). Ordre utilisé pour retry « ligne suivante ».
const WEAPON_PREFIX_DISPO: &[(WeaponPrefix, f32)] = &[
    (WeaponPrefix::Mauvais, 15.0),
    (WeaponPrefix::Banal, 30.0),
    (WeaponPrefix::Bon, 40.0),
    (WeaponPrefix::Solide, 25.0),
    (WeaponPrefix::Heroique, 10.0),
    (WeaponPrefix::Legendaire, 5.0),
    (WeaponPrefix::Aucun, 100.0),
];

/// Suffixe arme + Dispo%.
const WEAPON_SUFFIX_DISPO: &[(WeaponSuffix, f32)] = &[
    (WeaponSuffix::Casse, 5.0),
    (WeaponSuffix::Tordu, 10.0),
    (WeaponSuffix::Emousse, 20.0),
    (WeaponSuffix::Aiguise, 30.0),
    (WeaponSuffix::Travaille, 20.0),
    (WeaponSuffix::Orne, 10.0),
    (WeaponSuffix::Leger, 20.0),
    (WeaponSuffix::Renforce, 25.0),
    (WeaponSuffix::Blinde, 10.0),
    (WeaponSuffix::DeMaitre, 5.0),
    (WeaponSuffix::Unique, 5.0),
    (WeaponSuffix::Aucun, 100.0),
];

/// Matériau arme + Dispo%.
const WEAPON_MATERIAL_DISPO: &[(WeaponMaterial, f32)] = &[
    (WeaponMaterial::Bois, 15.0),
    (WeaponMaterial::Cuivre, 25.0),
    (WeaponMaterial::Bronze, 40.0),
    (WeaponMaterial::Fer, 50.0),
    (WeaponMaterial::Acier, 40.0),
    (WeaponMaterial::Argent, 20.0),
    (WeaponMaterial::Or, 10.0),
    (WeaponMaterial::Adamante, 5.0),
];

/// Tirage pondéré par Dispo% : P(i) = dispo_i / Σ(dispo). Modificateurs rares = probabilité proportionnelle : on parcourt la liste en ordre, on lance le %, si échec on passe à la ligne suivante, si fin du tableau on reprend au début jusqu’à ce qu’un choix soit fait.
fn pick_with_dispo_retry<T: Copy>(roll: &mut impl FnMut() -> f32, list: &[(T, f32)]) -> T {
    let total: f32 = list.iter().map(|(_, w)| w).sum();
    if total <= 0.0 {
        return list[0].0;
    }
    let mut r = roll() * total;
    for (item, w) in list {
        if r < *w {
            return *item;
        }
        r -= *w;
    }
    list[list.len() - 1].0
}

/// Même logique pour une liste d'entrées arme (dispo = .4).
fn pick_weapon_entry_with_dispo_retry<'a>(
    roll: &mut impl FnMut() -> f32,
    entries: &'a [WeaponEntry],
) -> &'a WeaponEntry {
    let mut idx = 0;
    loop {
        let roll_val = roll() * 100.0;
        if roll_val < f32::from(entries[idx].4) {
            return &entries[idx];
        }
        idx = (idx + 1) % entries.len();
    }
}

/// Même logique pour une liste d'entrées armure (dispo = .4).
fn pick_armor_entry_with_dispo_retry<'a>(
    roll: &mut impl FnMut() -> f32,
    entries: &'a [ArmorEntry],
) -> &'a ArmorEntry {
    let mut idx = 0;
    loop {
        let roll_val = roll() * 100.0;
        if roll_val < f32::from(entries[idx].4) {
            return &entries[idx];
        }
        idx = (idx + 1) % entries.len();
    }
}

/// Munition avec retry Dispo%. Retourne (nom, achat, vente).
fn pick_ammo_entry_with_dispo_retry(roll: &mut impl FnMut() -> f32) -> (&'static str, u32, u32) {
    let list = AMMO_ENTRIES;
    let mut idx = 0;
    loop {
        let roll_val = roll() * 100.0;
        if roll_val < f32::from(list[idx].3) {
            return (list[idx].0, list[idx].1, list[idx].2);
        }
        idx = (idx + 1) % list.len();
    }
}

/// Bouclier avec retry Dispo%. Retourne (nom, block, achat, vente).
fn pick_shield_entry_with_dispo_retry(
    roll: &mut impl FnMut() -> f32,
) -> (&'static str, f32, u32, u32) {
    let list = SHIELD_ENTRIES;
    let mut idx = 0;
    loop {
        let roll_val = roll() * 100.0;
        if roll_val < f32::from(list[idx].4) {
            return (list[idx].0, list[idx].1, list[idx].2, list[idx].3);
        }
        idx = (idx + 1) % list.len();
    }
}

// ——— Armure : tables (nom, réduction plate base). Peau = réduction plate, dégâts min subis = 1. ———

const CHEST_ARMOR: &[(&str, i32)] = &[
    ("Veste en cuir", 1),
    ("Gilet de cuir", 1),
    ("Gilet de combat", 2),
    ("Manteau de cuir", 2),
    ("Redingote", 2),
    ("Gambison", 2),
    ("Plastron cuir", 3),
    ("Cuirasse de cuir", 4),
    ("Cuirasse de cuir lourde", 5),
    ("Chemise de mailles", 7),
    ("Cotte de mailles", 7),
    ("Haubert de mailles", 7),
    ("Cuirasse pare-balles", 6),
    ("Brigandine", 8),
    ("Cuirasse de métal", 9),
    ("Armure pare-balle", 8),
    ("Plastron léger", 11),
    ("Plastron lourd", 12),
    ("Armure légère", 13),
    ("Armure composite", 11),
    ("Armure de plate", 14),
    ("S. armure légère", 15),
    ("S. armure lourde", 16),
];

const HEAD_ARMOR: &[(&str, i32)] = &[
    ("Coiffe de maille", 1),
    ("Haume", 1),
    ("Barbutte", 2),
    ("Salade", 3),
    ("Morion", 4),
    ("Bassinet", 5),
    ("Cervelière", 5),
    ("Armet", 6),
    ("Tassette", 6),
    ("Armet lourd", 7),
];

const GLOVES_ARMOR: &[(&str, i32)] = &[
    ("Gants de cuir", 1),
    ("Gants de cuir lourd", 1),
    ("Mitaine de mailles", 1),
    ("Gants de mailles", 2),
    ("Mitaines de plates", 2),
    ("Gantelets", 2),
    ("Gantelets Lourd", 3),
    ("Gantelets complets", 3),
];

const SHOULDERS_ARMOR: &[(&str, i32)] = &[
    ("Ailettes", 1),
    ("Épaulières de cuir", 1),
    ("Spalières", 2),
    ("Epaulières de plates", 2),
    ("Epaulières complètes", 3),
];

const LEGS_ARMOR: &[(&str, i32)] = &[
    ("Pantalon de cuir", 1),
    ("Pantalon de cuir lourd", 2),
    ("Pantalon de mailles", 3),
    ("Jambière", 4),
    ("Cuissarde", 5),
];

const FEET_ARMOR: &[(&str, i32)] = &[
    ("Bottes", 1),
    ("Bottes lourdes", 1),
    ("Rangers", 2),
    ("Soleret", 2),
    ("Grèves", 3),
];

/// Entrée armure marchand : (nom, armure flat, achat, vente, dispo%). Tableau-armure CSV.
struct ArmorEntry(&'static str, i32, u32, u32, u8);

const CHEST_ARMOR_ENTRIES: &[ArmorEntry] = &[
    ArmorEntry("Veste en cuir", 1, 40, 10, 50),
    ArmorEntry("Gilet de cuir", 1, 80, 20, 50),
    ArmorEntry("Gilet de combat", 2, 350, 87, 5),
    ArmorEntry("Manteau de cuir", 2, 160, 40, 50),
    ArmorEntry("Redingote", 2, 200, 50, 50),
    ArmorEntry("Gambison", 2, 240, 60, 50),
    ArmorEntry("Plastron cuir", 3, 320, 80, 40),
    ArmorEntry("Cuirasse de cuir", 4, 440, 110, 30),
    ArmorEntry("Cuirasse de cuir lourde", 5, 650, 162, 20),
    ArmorEntry("Chemise de mailles", 7, 700, 233, 30),
    ArmorEntry("Cotte de mailles", 7, 800, 266, 40),
    ArmorEntry("Haubert de mailles", 7, 850, 283, 20),
    ArmorEntry("Cuirasse pare-balles", 6, 1260, 420, 5),
    ArmorEntry("Brigandine", 8, 1150, 383, 15),
    ArmorEntry("Cuirasse de métal", 9, 1560, 780, 10),
    ArmorEntry("Armure pare-balle", 8, 1960, 980, 1),
    ArmorEntry("Plastron léger", 11, 1920, 960, 5),
    ArmorEntry("Plastron lourd", 12, 2040, 1020, 3),
    ArmorEntry("Armure légère", 13, 2520, 1260, 2),
    ArmorEntry("Armure composite", 11, 2520, 1260, 2),
    ArmorEntry("Armure de plate", 14, 2660, 1330, 2),
    ArmorEntry("S. armure légère", 15, 3690, 1845, 1),
    ArmorEntry("S. armure lourde", 16, 4300, 2150, 1),
];

const HEAD_ARMOR_ENTRIES: &[ArmorEntry] = &[
    ArmorEntry("Coiffe de maille", 1, 100, 25, 40),
    ArmorEntry("Haume", 1, 120, 30, 40),
    ArmorEntry("Barbutte", 2, 300, 75, 30),
    ArmorEntry("Salade", 3, 450, 112, 20),
    ArmorEntry("Morion", 4, 650, 162, 20),
    ArmorEntry("Bassinet", 5, 940, 235, 10),
    ArmorEntry("Cervelière", 5, 1050, 262, 8),
    ArmorEntry("Armet", 6, 1300, 325, 7),
    ArmorEntry("Tassette", 6, 1580, 395, 5),
    ArmorEntry("Armet lourd", 7, 1950, 487, 3),
];

const GLOVES_ARMOR_ENTRIES: &[ArmorEntry] = &[
    ArmorEntry("Gants de cuir", 1, 50, 12, 50),
    ArmorEntry("Gants de cuir lourd", 1, 100, 25, 40),
    ArmorEntry("Mitaine de mailles", 1, 150, 37, 30),
    ArmorEntry("Gants de mailles", 2, 240, 60, 20),
    ArmorEntry("Mitaines de plates", 2, 300, 75, 10),
    ArmorEntry("Gantelets", 2, 480, 120, 5),
    ArmorEntry("Gantelets Lourd", 3, 700, 175, 5),
    ArmorEntry("Gantelets complets", 3, 880, 220, 3),
];

const SHOULDERS_ARMOR_ENTRIES: &[ArmorEntry] = &[
    ArmorEntry("Ailettes", 1, 50, 12, 60),
    ArmorEntry("Épaulières de cuir", 1, 100, 25, 50),
    ArmorEntry("Spalières", 2, 200, 50, 20),
    ArmorEntry("Epaulières de plates", 2, 350, 87, 10),
    ArmorEntry("Epaulières complètes", 3, 630, 157, 5),
];

const LEGS_ARMOR_ENTRIES: &[ArmorEntry] = &[
    ArmorEntry("Pantalon de cuir", 1, 100, 25, 50),
    ArmorEntry("Pantalon de cuir lourd", 2, 200, 50, 40),
    ArmorEntry("Pantalon de mailles", 3, 360, 90, 20),
    ArmorEntry("Jambière", 4, 810, 202, 10),
    ArmorEntry("Cuissarde", 5, 1210, 302, 5),
];

const FEET_ARMOR_ENTRIES: &[ArmorEntry] = &[
    ArmorEntry("Bottes", 1, 60, 15, 50),
    ArmorEntry("Bottes lourdes", 1, 120, 30, 40),
    ArmorEntry("Rangers", 2, 240, 60, 5),
    ArmorEntry("Soleret", 2, 400, 100, 10),
    ArmorEntry("Grèves", 3, 710, 177, 15),
];

fn roll_armor_prefix(roll: &mut impl FnMut() -> f32) -> ArmorPrefix {
    let d = roll_d100(roll);
    if d < 10 {
        ArmorPrefix::Mauvais
    } else if d < 25 {
        ArmorPrefix::Banal
    } else if d < 60 {
        ArmorPrefix::Aucun
    } else if d < 75 {
        ArmorPrefix::Bon
    } else if d < 84 {
        ArmorPrefix::Solide
    } else if d < 95 {
        ArmorPrefix::Heroique
    } else {
        ArmorPrefix::Legendaire
    }
}

fn roll_armor_suffix(roll: &mut impl FnMut() -> f32) -> ArmorSuffix {
    let d = roll_d100(roll);
    if d < 8 {
        ArmorSuffix::Casse
    } else if d < 15 {
        ArmorSuffix::Tordu
    } else if d < 20 {
        ArmorSuffix::Emousse
    } else if d < 43 {
        ArmorSuffix::Aucun
    } else if d < 50 {
        ArmorSuffix::Aiguise
    } else if d < 56 {
        ArmorSuffix::Travaille
    } else if d < 60 {
        ArmorSuffix::Orne
    } else if d < 75 {
        ArmorSuffix::Leger
    } else if d < 84 {
        ArmorSuffix::Renforce
    } else if d < 95 {
        ArmorSuffix::Blinde
    } else if d < 99 {
        ArmorSuffix::DeMaitre
    } else {
        ArmorSuffix::Unique
    }
}

fn roll_armor_material(roll: &mut impl FnMut() -> f32) -> ArmorMaterial {
    let d = roll_d100(roll);
    if d < 10 {
        ArmorMaterial::Bois
    } else if d < 20 {
        ArmorMaterial::Cuivre
    } else if d < 30 {
        ArmorMaterial::Bronze
    } else if d < 60 {
        ArmorMaterial::Fer
    } else if d < 72 {
        ArmorMaterial::Acier
    } else if d < 85 {
        ArmorMaterial::Argent
    } else if d < 92 {
        ArmorMaterial::Or
    } else {
        ArmorMaterial::Adamante
    }
}

fn roll_armor_base(slot: ItemSlot, roll: &mut impl FnMut() -> f32) -> Option<(String, i32)> {
    let names = match slot {
        ItemSlot::Chest => CHEST_ARMOR,
        ItemSlot::Head => HEAD_ARMOR,
        ItemSlot::Gloves => GLOVES_ARMOR,
        ItemSlot::Shoulders => SHOULDERS_ARMOR,
        ItemSlot::Legs => LEGS_ARMOR,
        ItemSlot::Feet => FEET_ARMOR,
        _ => return None,
    };
    let idx = (roll() * names.len() as f32).min(names.len() as f32 - 1.0) as usize;
    let (name, base) = names[idx];
    Some((name.to_string(), base))
}

fn build_armor_display_name(
    prefix: &ArmorPrefix,
    base: &str,
    suffix: &ArmorSuffix,
    material: &ArmorMaterial,
) -> String {
    let mut parts: Vec<String> = Vec::new();
    let pl = prefix.label();
    if !pl.is_empty() {
        parts.push(pl.to_string());
    }
    parts.push(base.to_string());
    let sl = suffix.label();
    if !sl.is_empty() {
        parts.push(sl.to_string());
    }
    let name = parts.join(" ");
    format!("{} en {}", name, material.label())
}

/// Tirage pondéré par dispo% dans une slice de WeaponEntry. Retourne l'entrée choisie.
fn pick_weighted_weapon<'a>(
    entries: &'a [WeaponEntry],
    roll: &mut impl FnMut() -> f32,
) -> &'a WeaponEntry {
    let total: u32 = entries.iter().map(|e| u32::from(e.4)).sum();
    let r = (roll() * total as f32).min(total as f32 - 0.01).max(0.0) as u32;
    let mut cum = 0u32;
    for e in entries {
        cum += u32::from(e.4);
        if r < cum {
            return e;
        }
    }
    entries.last().unwrap()
}

/// Tirage pondéré bouclier par dispo% (nom, block, buy, sell).
fn pick_weighted_weapon_shield(roll: &mut impl FnMut() -> f32) -> (&'static str, f32, u32, u32) {
    let total: u32 = SHIELD_ENTRIES.iter().map(|e| u32::from(e.4)).sum();
    let r = (roll() * total as f32).min(total as f32 - 0.01).max(0.0) as u32;
    let mut cum = 0u32;
    for e in SHIELD_ENTRIES {
        cum += u32::from(e.4);
        if r < cum {
            return (e.0, e.1, e.2, e.3);
        }
    }
    let e = SHIELD_ENTRIES.last().unwrap();
    (e.0, e.1, e.2, e.3)
}

/// Tirage pondéré pour munitions (nom, achat, vente, dispo).
fn pick_weighted_ammo(roll: &mut impl FnMut() -> f32) -> (&'static str, u32, u32, u8) {
    let entries = AMMO_ENTRIES;
    let total: u32 = entries.iter().map(|e| u32::from(e.3)).sum();
    let r = (roll() * total as f32).min(total as f32 - 0.01).max(0.0) as u32;
    let mut cum = 0u32;
    for e in entries {
        cum += u32::from(e.3);
        if r < cum {
            return *e;
        }
    }
    entries[entries.len() - 1]
}

/// Tirage arme/base : (nom, dmg, block, is_ranged, base_buy, base_sell). Boucliers sans base_buy/base_sell.
fn roll_weapon_base(
    slot: ItemSlot,
    roll: &mut impl FnMut() -> f32,
) -> (
    String,
    Option<i32>,
    Option<f32>,
    bool,
    Option<u32>,
    Option<u32>,
) {
    match slot {
        ItemSlot::MainHand | ItemSlot::OffHand => {
            let total_1h: u32 = ONE_HAND_WEAPONS.iter().map(|e| u32::from(e.4)).sum();
            let total_2h: u32 = TWO_HAND_WEAPONS.iter().map(|e| u32::from(e.4)).sum();
            let total_hast: u32 = HAST_WEAPONS.iter().map(|e| u32::from(e.4)).sum();
            let total_ranged: u32 = RANGED_WEAPONS.iter().map(|e| u32::from(e.4)).sum();
            let total_weapons = total_1h + total_2h + total_hast + total_ranged;
            let total_shields: u32 = SHIELD_ENTRIES.iter().map(|e| u32::from(e.4)).sum();
            let kind_total = total_weapons + total_shields;
            let kind_r = (roll() * kind_total as f32)
                .min(kind_total as f32 - 0.01)
                .max(0.0) as u32;
            if kind_r < total_1h {
                let e = pick_weighted_weapon(ONE_HAND_WEAPONS, roll);
                (
                    e.0.to_string(),
                    Some(e.1.max(1)),
                    None,
                    false,
                    Some(e.2),
                    Some(e.3),
                )
            } else if kind_r < total_1h + total_2h {
                let e = pick_weighted_weapon(TWO_HAND_WEAPONS, roll);
                (
                    e.0.to_string(),
                    Some(e.1.max(1)),
                    None,
                    false,
                    Some(e.2),
                    Some(e.3),
                )
            } else if kind_r < total_1h + total_2h + total_hast {
                let e = pick_weighted_weapon(HAST_WEAPONS, roll);
                (
                    e.0.to_string(),
                    Some(e.1.max(1)),
                    None,
                    false,
                    Some(e.2),
                    Some(e.3),
                )
            } else if kind_r < total_weapons {
                let e = pick_weighted_weapon(RANGED_WEAPONS, roll);
                (
                    e.0.to_string(),
                    Some(e.1.max(1)),
                    None,
                    true,
                    Some(e.2),
                    Some(e.3),
                )
            } else {
                let e = pick_weighted_weapon_shield(roll);
                (
                    e.0.to_string(),
                    None,
                    Some(e.1),
                    false,
                    Some(e.2),
                    Some(e.3),
                )
            }
        }
        ItemSlot::Ammo => {
            let (name, buy, sell, _) = pick_weighted_ammo(roll);
            (name.to_string(), None, None, false, Some(buy), Some(sell))
        }
        _ => (String::from("Objet"), None, None, false, None, None),
    }
}

/// Table de bases par slot et rareté (identification). Retourne (nom de base, slot). MainHand/OffHand/Ammo gérés ailleurs.
fn roll_base_item_identification(
    slot: ItemSlot,
    rarity: ItemRarity,
    roll: &mut impl FnMut() -> f32,
    _wisdom_bonus: f32,
) -> Option<(String, ItemSlot)> {
    let (names, s) = match (slot, rarity) {
        (ItemSlot::Head, ItemRarity::Common) => (
            vec!["Capuche", "Chapeau de cuir", "Bandana"],
            ItemSlot::Head,
        ),
        (ItemSlot::Head, ItemRarity::Uncommon) => {
            (vec!["Heaume léger", "Coiffe de mage"], ItemSlot::Head)
        }
        (ItemSlot::Head, ItemRarity::Rare) => (vec!["Couronne de fer"], ItemSlot::Head),
        (ItemSlot::Head, ItemRarity::Magical) => (vec!["Diadème de clarté"], ItemSlot::Head),
        (ItemSlot::Head, ItemRarity::UltraRare) => (vec!["Couronne du souverain"], ItemSlot::Head),
        (ItemSlot::Head, ItemRarity::Unique) => (vec!["Visage de Nawak"], ItemSlot::Head),
        (ItemSlot::Neck, _) => (vec!["Collier", "Amulette", "Medaillon"], ItemSlot::Neck),
        (ItemSlot::Shoulders, _) => (
            vec!["Spalières", "Épaulières", "Mantelet"],
            ItemSlot::Shoulders,
        ),
        (ItemSlot::Bracer, _) => (
            vec!["Brassard", "Protège-bras", "Manchette"],
            ItemSlot::Bracer,
        ),
        (ItemSlot::Gloves, _) => (vec!["Gants", "Gantelets", "Mitaines"], ItemSlot::Gloves),
        (ItemSlot::Ring1 | ItemSlot::Ring2, _) => (vec!["Bague", "Anneau", "Chevalière"], slot),
        (ItemSlot::Chest, ItemRarity::Common) => (
            vec!["Tunique", "Veste en cuir", "Haubert usé"],
            ItemSlot::Chest,
        ),
        (ItemSlot::Chest, ItemRarity::Uncommon) => (
            vec!["Armure de plaques légère", "Robe de voyage"],
            ItemSlot::Chest,
        ),
        (ItemSlot::Chest, ItemRarity::Rare) => (vec!["Cuirasse du château"], ItemSlot::Chest),
        (ItemSlot::Chest, ItemRarity::Magical) => (vec!["Cuirasse runique"], ItemSlot::Chest),
        (ItemSlot::Chest, ItemRarity::UltraRare) => (vec!["Plastron immortel"], ItemSlot::Chest),
        (ItemSlot::Chest, ItemRarity::Unique) => (vec!["Cœur de pierre"], ItemSlot::Chest),
        (ItemSlot::Belt, _) => (vec!["Ceinture", "Baudrier", "Sash"], ItemSlot::Belt),
        (ItemSlot::Legs, _) => (
            vec!["Jambières", "Grèves", "Pantalon de cuir"],
            ItemSlot::Legs,
        ),
        (ItemSlot::Feet, ItemRarity::Common) => (
            vec!["Bottes en cuir", "Sandales", "Souliers usés"],
            ItemSlot::Feet,
        ),
        (ItemSlot::Feet, ItemRarity::Uncommon) => {
            (vec!["Bottes de marche", "Grèves légères"], ItemSlot::Feet)
        }
        (ItemSlot::Feet, ItemRarity::Rare) => (vec!["Bottes de guerre"], ItemSlot::Feet),
        (ItemSlot::Feet, ItemRarity::Magical) => (vec!["Bottes de vent"], ItemSlot::Feet),
        (ItemSlot::Feet, ItemRarity::UltraRare) => (vec!["Marcheur des ombres"], ItemSlot::Feet),
        (ItemSlot::Feet, ItemRarity::Unique) => (vec!["Pas de géant"], ItemSlot::Feet),
        (ItemSlot::Consumable, ItemRarity::Common) => (
            vec!["Potion de vie mineure", "Pain", "Eau"],
            ItemSlot::Consumable,
        ),
        (ItemSlot::Consumable, ItemRarity::Uncommon) => (
            vec!["Potion de vie", "Elixir de force"],
            ItemSlot::Consumable,
        ),
        (ItemSlot::Consumable, ItemRarity::Rare) => (
            vec!["Potion de vie majeure", "Pierre de rappel"],
            ItemSlot::Consumable,
        ),
        (ItemSlot::Consumable, ItemRarity::Magical) => (
            vec!["Elixir de résistance", "Potion de mana"],
            ItemSlot::Consumable,
        ),
        (ItemSlot::Consumable, ItemRarity::UltraRare) => {
            (vec!["Pierre de renaissance"], ItemSlot::Consumable)
        }
        (ItemSlot::Consumable, ItemRarity::Unique) => {
            (vec!["Larme de Nawak"], ItemSlot::Consumable)
        }
        (ItemSlot::MainHand | ItemSlot::OffHand | ItemSlot::Ammo, _) => return None,
    };
    let idx = (roll() * names.len() as f32).min(names.len() as f32 - 1.0) as usize;
    Some((names[idx].to_string(), s))
}

fn roll_prefix(roll: &mut impl FnMut() -> f32) -> String {
    let prefixes = ["renforcé", "léger", "vieux", "usé", "brillant"];
    let idx = (roll() * prefixes.len() as f32).min(prefixes.len() as f32 - 1.0) as usize;
    prefixes[idx].to_string()
}

/// Suffixes « compatibles » avec une base (ex. bottes → solide, épée → tranchant).
fn roll_suffix(_base_name: &str, roll: &mut impl FnMut() -> f32) -> String {
    let suffixes = ["solide", "rapide", "résistant", "tranchant", "vigilant"];
    let idx = (roll() * suffixes.len() as f32).min(suffixes.len() as f32 - 1.0) as usize;
    suffixes[idx].to_string()
}

fn build_display_name(base: &str, prefix: Option<&str>, suffix: Option<&str>) -> String {
    let mut parts = Vec::new();
    if let Some(p) = prefix {
        if !p.is_empty() {
            parts.push(p);
        }
    }
    parts.push(base);
    if let Some(s) = suffix {
        if !s.is_empty() {
            parts.push(s);
        }
    }
    parts.join(" ")
}

/// [préfixe][nom][suffixe] en [matériau] pour armes.
fn build_weapon_display_name(
    prefix: &WeaponPrefix,
    base: &str,
    suffix: &WeaponSuffix,
    material: &WeaponMaterial,
) -> String {
    let mut parts: Vec<String> = Vec::new();
    let pl = prefix.label();
    if !pl.is_empty() {
        parts.push(pl.to_string());
    }
    parts.push(base.to_string());
    let sl = suffix.label();
    if !sl.is_empty() {
        parts.push(sl.to_string());
    }
    let name = parts.join(" ");
    format!("{} en {}", name, material.label())
}

/// Types d'arme pour la pool marchand : 0=1 main, 1=2 mains, 2=hast, 3=tir, 4=munitions, 5=bouclier.
const MERCHANT_WEAPON_TYPES: usize = 6;

/// Tire 5 types d'arme distincts parmi les 6 (une catégorie exclue au hasard) pour maximiser la variété du marchand.
fn pick_five_distinct_weapon_types(roll: &mut impl FnMut() -> f32) -> [usize; 5] {
    let mut indices = [0usize, 1, 2, 3, 4, 5];
    for i in 0..5 {
        let j = i + (roll() * (MERCHANT_WEAPON_TYPES - i) as f32) as usize;
        indices.swap(i, j);
    }
    [indices[0], indices[1], indices[2], indices[3], indices[4]]
}

/// Génère un objet de la pool « armes » pour un type donné (0=1H, 1=2H, 2=hast, 3=tir, 4=munitions, 5=bouclier).
fn generate_merchant_weapon_of_type(
    roll: &mut impl FnMut() -> f32,
    type_idx: usize,
) -> ItemInstance {
    match type_idx {
        0 => {
            let e = pick_weapon_entry_with_dispo_retry(roll, ONE_HAND_WEAPONS);
            let wp = pick_with_dispo_retry(roll, WEAPON_PREFIX_DISPO);
            let ws = pick_with_dispo_retry(roll, WEAPON_SUFFIX_DISPO);
            let wm = pick_with_dispo_retry(roll, WEAPON_MATERIAL_DISPO);
            let display_name = build_weapon_display_name(&wp, e.0, &ws, &wm);
            ItemInstance {
                display_name,
                slot: ItemSlot::MainHand,
                rarity: ItemRarity::Common,
                prefix: None,
                suffix: None,
                weapon_prefix: Some(wp),
                weapon_suffix: Some(ws),
                weapon_material: Some(wm),
                base_buy: Some(e.2),
                base_sell: Some(e.3),
                base_damage: Some(e.1.max(1)),
                is_ranged: false,
                block_chance_base: None,
                base_armor: None,
                armor_prefix: None,
                armor_suffix: None,
                armor_material: None,
            }
        }
        1 => {
            let e = pick_weapon_entry_with_dispo_retry(roll, TWO_HAND_WEAPONS);
            let wp = pick_with_dispo_retry(roll, WEAPON_PREFIX_DISPO);
            let ws = pick_with_dispo_retry(roll, WEAPON_SUFFIX_DISPO);
            let wm = pick_with_dispo_retry(roll, WEAPON_MATERIAL_DISPO);
            let display_name = build_weapon_display_name(&wp, e.0, &ws, &wm);
            ItemInstance {
                display_name,
                slot: ItemSlot::MainHand,
                rarity: ItemRarity::Common,
                prefix: None,
                suffix: None,
                weapon_prefix: Some(wp),
                weapon_suffix: Some(ws),
                weapon_material: Some(wm),
                base_buy: Some(e.2),
                base_sell: Some(e.3),
                base_damage: Some(e.1.max(1)),
                is_ranged: false,
                block_chance_base: None,
                base_armor: None,
                armor_prefix: None,
                armor_suffix: None,
                armor_material: None,
            }
        }
        2 => {
            let e = pick_weapon_entry_with_dispo_retry(roll, HAST_WEAPONS);
            let wp = pick_with_dispo_retry(roll, WEAPON_PREFIX_DISPO);
            let ws = pick_with_dispo_retry(roll, WEAPON_SUFFIX_DISPO);
            let wm = pick_with_dispo_retry(roll, WEAPON_MATERIAL_DISPO);
            let display_name = build_weapon_display_name(&wp, e.0, &ws, &wm);
            ItemInstance {
                display_name,
                slot: ItemSlot::MainHand,
                rarity: ItemRarity::Common,
                prefix: None,
                suffix: None,
                weapon_prefix: Some(wp),
                weapon_suffix: Some(ws),
                weapon_material: Some(wm),
                base_buy: Some(e.2),
                base_sell: Some(e.3),
                base_damage: Some(e.1.max(1)),
                is_ranged: false,
                block_chance_base: None,
                base_armor: None,
                armor_prefix: None,
                armor_suffix: None,
                armor_material: None,
            }
        }
        3 => {
            let e = pick_weapon_entry_with_dispo_retry(roll, RANGED_WEAPONS);
            let wp = pick_with_dispo_retry(roll, WEAPON_PREFIX_DISPO);
            let ws = pick_with_dispo_retry(roll, WEAPON_SUFFIX_DISPO);
            let wm = pick_with_dispo_retry(roll, WEAPON_MATERIAL_DISPO);
            let display_name = build_weapon_display_name(&wp, e.0, &ws, &wm);
            ItemInstance {
                display_name,
                slot: ItemSlot::MainHand,
                rarity: ItemRarity::Common,
                prefix: None,
                suffix: None,
                weapon_prefix: Some(wp),
                weapon_suffix: Some(ws),
                weapon_material: Some(wm),
                base_buy: Some(e.2),
                base_sell: Some(e.3),
                base_damage: Some(e.1.max(1)),
                is_ranged: true,
                block_chance_base: None,
                base_armor: None,
                armor_prefix: None,
                armor_suffix: None,
                armor_material: None,
            }
        }
        4 => {
            let (name, buy, sell) = pick_ammo_entry_with_dispo_retry(roll);
            let wm = pick_with_dispo_retry(roll, WEAPON_MATERIAL_DISPO);
            let display_name = format!("{} en {}", name, wm.label());
            ItemInstance {
                display_name,
                slot: ItemSlot::Ammo,
                rarity: ItemRarity::Common,
                prefix: None,
                suffix: None,
                weapon_prefix: None,
                weapon_suffix: None,
                weapon_material: Some(wm),
                base_buy: Some(buy),
                base_sell: Some(sell),
                base_damage: None,
                is_ranged: false,
                block_chance_base: None,
                base_armor: None,
                armor_prefix: None,
                armor_suffix: None,
                armor_material: None,
            }
        }
        5 => {
            let (name, block, buy, sell) = pick_shield_entry_with_dispo_retry(roll);
            ItemInstance {
                display_name: name.to_string(),
                slot: ItemSlot::OffHand,
                rarity: ItemRarity::Common,
                prefix: None,
                suffix: None,
                weapon_prefix: None,
                weapon_suffix: None,
                weapon_material: None,
                base_buy: Some(buy),
                base_sell: Some(sell),
                base_damage: None,
                is_ranged: false,
                block_chance_base: Some(block),
                base_armor: None,
                armor_prefix: None,
                armor_suffix: None,
                armor_material: None,
            }
        }
        _ => {
            let e = pick_weapon_entry_with_dispo_retry(roll, ONE_HAND_WEAPONS);
            let wp = pick_with_dispo_retry(roll, WEAPON_PREFIX_DISPO);
            let ws = pick_with_dispo_retry(roll, WEAPON_SUFFIX_DISPO);
            let wm = pick_with_dispo_retry(roll, WEAPON_MATERIAL_DISPO);
            let display_name = build_weapon_display_name(&wp, e.0, &ws, &wm);
            ItemInstance {
                display_name,
                slot: ItemSlot::MainHand,
                rarity: ItemRarity::Common,
                prefix: None,
                suffix: None,
                weapon_prefix: Some(wp),
                weapon_suffix: Some(ws),
                weapon_material: Some(wm),
                base_buy: Some(e.2),
                base_sell: Some(e.3),
                base_damage: Some(e.1.max(1)),
                is_ranged: false,
                block_chance_base: None,
                base_armor: None,
                armor_prefix: None,
                armor_suffix: None,
                armor_material: None,
            }
        }
    }
}

/// Types d'armure pour la pool marchand : choix équitable entre torse, tête, gants, épaules, jambes, pieds, bouclier.
const MERCHANT_ARMOR_TYPES: usize = 7;

/// Génère un objet de la pool « armures » : type choisi équitablement, puis base / préfixe / suffixe / matériau avec retry Dispo%.
fn generate_merchant_armor_slot(roll: &mut impl FnMut() -> f32) -> ItemInstance {
    let type_idx =
        (roll() * MERCHANT_ARMOR_TYPES as f32).min(MERCHANT_ARMOR_TYPES as f32 - 0.01) as usize;
    let (slot, entries): (ItemSlot, &[ArmorEntry]) = match type_idx {
        0 => (ItemSlot::Chest, CHEST_ARMOR_ENTRIES),
        1 => (ItemSlot::Head, HEAD_ARMOR_ENTRIES),
        2 => (ItemSlot::Gloves, GLOVES_ARMOR_ENTRIES),
        3 => (ItemSlot::Shoulders, SHOULDERS_ARMOR_ENTRIES),
        4 => (ItemSlot::Legs, LEGS_ARMOR_ENTRIES),
        5 => (ItemSlot::Feet, FEET_ARMOR_ENTRIES),
        _ => {
            let (name, block, buy, sell) = pick_shield_entry_with_dispo_retry(roll);
            return ItemInstance {
                display_name: name.to_string(),
                slot: ItemSlot::OffHand,
                rarity: ItemRarity::Common,
                prefix: None,
                suffix: None,
                weapon_prefix: None,
                weapon_suffix: None,
                weapon_material: None,
                base_buy: Some(buy),
                base_sell: Some(sell),
                base_damage: None,
                is_ranged: false,
                block_chance_base: Some(block),
                base_armor: None,
                armor_prefix: None,
                armor_suffix: None,
                armor_material: None,
            };
        }
    };
    let e = pick_armor_entry_with_dispo_retry(roll, entries);
    let ap = pick_with_dispo_retry(roll, ARMOR_PREFIX_DISPO);
    let as_ = pick_with_dispo_retry(roll, ARMOR_SUFFIX_DISPO);
    let am = pick_with_dispo_retry(roll, ARMOR_MATERIAL_DISPO);
    let display_name = build_armor_display_name(&ap, e.0, &as_, &am);
    ItemInstance {
        display_name,
        slot,
        rarity: ItemRarity::Common,
        prefix: None,
        suffix: None,
        weapon_prefix: None,
        weapon_suffix: None,
        weapon_material: None,
        base_buy: Some(e.2),
        base_sell: Some(e.3),
        base_damage: None,
        is_ranged: false,
        block_chance_base: None,
        base_armor: Some(e.1),
        armor_prefix: Some(ap),
        armor_suffix: Some(as_),
        armor_material: Some(am),
    }
}

/// Types d'accessoires pour la pool marchand : choix équitable entre cou, épaules, brassard, bague, ceinture, consommable.
const MERCHANT_ACCESSORY_TYPES: usize = 6;

/// Génère un objet de la pool « accessoires & potions » : type choisi équitablement, puis roll identification pour ce slot.
fn generate_merchant_accessory_slot(roll: &mut impl FnMut() -> f32) -> ItemInstance {
    let type_idx = (roll() * MERCHANT_ACCESSORY_TYPES as f32)
        .min(MERCHANT_ACCESSORY_TYPES as f32 - 0.01) as usize;
    let slot = match type_idx {
        0 => ItemSlot::Neck,
        1 => ItemSlot::Shoulders,
        2 => ItemSlot::Bracer,
        3 => ItemSlot::Ring1,
        4 => ItemSlot::Belt,
        _ => ItemSlot::Consumable,
    };
    if slot == ItemSlot::Consumable {
        if roll() < 0.5 {
            ItemInstance::potion_vie()
        } else {
            ItemInstance::potion_mana()
        }
    } else {
        roll_identification_self(roll, 0, 0, slot)
    }
}

/// Génère les 3 pools du marchand : 5 armes, 5 armures, 5 accessoires.
/// Armes : 5 types distincts parmi 1 main, 2 mains, hast, tir, munitions, bouclier (une catégorie exclue au hasard) pour maximiser la variété.
/// Armures / accessoires : type choisi équitablement par slot. Un objet aléatoire est non identifié (100 or). 10 % de chance qu'un objet soit rare.
pub fn generate_merchant_pools(
    roll: &mut impl FnMut() -> f32,
) -> (Vec<MerchantEntry>, Vec<MerchantEntry>, Vec<MerchantEntry>) {
    let type_indices = pick_five_distinct_weapon_types(roll);
    let mut weapons: Vec<MerchantEntry> = type_indices
        .iter()
        .map(|&type_idx| {
            let item = generate_merchant_weapon_of_type(roll, type_idx);
            let price = item.buy_price().max(1);
            MerchantEntry {
                entry: InventoryEntry::Identified(item),
                price,
            }
        })
        .collect();
    let mut armor: Vec<MerchantEntry> = (0..5)
        .map(|_| {
            let item = generate_merchant_armor_slot(roll);
            let price = item.buy_price().max(1);
            MerchantEntry {
                entry: InventoryEntry::Identified(item),
                price,
            }
        })
        .collect();
    let mut accessories: Vec<MerchantEntry> = (0..5)
        .map(|_| {
            let item = generate_merchant_accessory_slot(roll);
            let price = item.buy_price().max(1);
            MerchantEntry {
                entry: InventoryEntry::Identified(item),
                price,
            }
        })
        .collect();

    let mut all: Vec<&mut MerchantEntry> = weapons
        .iter_mut()
        .chain(armor.iter_mut())
        .chain(accessories.iter_mut())
        .collect();
    let n = all.len();

    let idx_unidentified = (roll() * n as f32).min(n as f32 - 1.0) as usize;
    let slot_to_hide = match &all[idx_unidentified].entry {
        InventoryEntry::Identified(i) => i.slot,
        InventoryEntry::Unidentified(s) => *s,
    };
    all[idx_unidentified].entry = InventoryEntry::Unidentified(slot_to_hide);
    all[idx_unidentified].price = 100;

    let one_rare = roll() < 0.10;
    if one_rare {
        let idx_rare = (roll() * n as f32).min(n as f32 - 1.0) as usize;
        if idx_rare != idx_unidentified {
            if let InventoryEntry::Identified(ref mut item) = all[idx_rare].entry {
                item.rarity = ItemRarity::Rare;
                all[idx_rare].price = item.buy_price().max(1);
            }
        }
    }

    (weapons, armor, accessories)
}
