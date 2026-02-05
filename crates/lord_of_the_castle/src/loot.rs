//! Loot — drop Or, XP, Objets à la mort d'un ennemi (Miyukini Survivor).
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

/// Slots d’équipement du joueur (armure + mains + munitions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemSlot {
    Head,       // Tête
    Neck,       // Collier
    Shoulders,  // Épaules
    Bracer,     // Brassard
    Gloves,     // Gants
    Ring1,      // Bague 1
    Ring2,      // Bague 2
    Chest,      // Torse
    Belt,       // Ceinture
    Legs,       // Jambes
    Feet,       // Pieds
    MainHand,   // Main droite (arme 1H, 2H, tir, bouclier)
    OffHand,    // Main gauche (arme 1H, bouclier)
    Ammo,       // Munitions
    Consumable,
}

impl ItemSlot {
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

    pub fn is_weapon_or_shield(&self) -> bool {
        matches!(self, ItemSlot::MainHand | ItemSlot::OffHand)
    }

    pub fn is_ammo(&self) -> bool {
        self == &ItemSlot::Ammo
    }

    /// Slots affichés dans la fenêtre Équipement (tous sauf Consumable).
    pub fn equipment_slots() -> &'static [ItemSlot] {
        use ItemSlot::*;
        const SLOTS: [ItemSlot; 14] = [
            Head, Neck, Shoulders, Bracer, Gloves, Ring1, Ring2,
            Chest, Belt, Legs, Feet, MainHand, OffHand, Ammo,
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

    /// Couleur de carte (egui) selon la rareté.
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
}

/// Suffixe arme : table d100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponSuffix {
    Casse,      // -60% dmg / -30% / 1-8
    Tordu,      // -40% / -20% / 9-15
    Emousse,    // -10% / -10% / 16-20
    Aucun,      // 0% / 0% / 21-43
    Aiguise,    // +5% / +5% / 44-50
    Travaille,  // +10% / +10% / 51-56
    Orne,       // -20% / +50% / 57-60
    Leger,      // -5% / +5% / 61-75
    Renforce,   // +15% / +15% / 76-84
    Blinde,     // +20% / +25% / 85-95
    DeMaitre,   // +25% / +50% / 95-100
}

impl WeaponSuffix {
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
        }
    }
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
        }
    }
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
        }
    }
}

/// Matériau arme : [préfixe][nom][suffixe] en [matériau]. Table d100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponMaterial {
    Bois,    // -20% / 1-10
    Cuivre,  // -10% / 11-20
    Bronze,  // -5% / 21-30
    Fer,     // 0% / 31-75
    Acier,   // +10% / 76-85
    Argent,  // +15% / 86-95
    Adamante, // +25% / 96-100
}

impl WeaponMaterial {
    pub fn label(&self) -> &'static str {
        match self {
            WeaponMaterial::Bois => "bois",
            WeaponMaterial::Cuivre => "cuivre",
            WeaponMaterial::Bronze => "bronze",
            WeaponMaterial::Fer => "fer",
            WeaponMaterial::Acier => "acier",
            WeaponMaterial::Argent => "argent",
            WeaponMaterial::Adamante => "adamante",
        }
    }
    pub fn damage_pct(&self) -> i32 {
        match self {
            WeaponMaterial::Bois => -20,
            WeaponMaterial::Cuivre => -10,
            WeaponMaterial::Bronze => -5,
            WeaponMaterial::Fer => 0,
            WeaponMaterial::Acier => 10,
            WeaponMaterial::Argent => 15,
            WeaponMaterial::Adamante => 25,
        }
    }
}

// ——— Armure : préfixe / suffixe / matériau (multiplicateur armure). Réduction plate, dégâts min = 1. ———

/// Préfixe armure : multiplicateur sur la valeur d’armure de base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmorPrefix {
    Mauvais,   // 0.5
    Banal,     // 0.9
    Aucun,     // 1.0
    Bon,       // 1.1
    Solide,    // 1.2
    Heroique,  // 1.3
    Legendaire, // 1.5
}

impl ArmorPrefix {
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
}

impl ArmorSuffix {
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
        }
    }
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
        }
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
}

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
    /// Dégâts de base (armes 1H listées par le spec; min 1).
    pub base_damage: Option<i32>,
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
    /// Épée courte de départ (équipée par défaut en main droite).
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
            base_damage: Some(4),
            block_chance_base: None,
            base_armor: None,
            armor_prefix: None,
            armor_suffix: None,
            armor_material: None,
        }
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

    /// Prix de vente (or) selon rareté, slot et bonus arme.
    pub fn sell_price(&self) -> u32 {
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
        let price = (base * mult) as i32;
        let with_bonus = price + (price * pct_bonus / 100);
        with_bonus.max(1) as u32
    }

    /// Armure effective : base × préfixe × suffixe × matériau (réduction plate ; dégâts min subis = 1).
    pub fn effective_armor(&self) -> i32 {
        let Some(base) = self.base_armor else {
            return 0;
        };
        let p = self.armor_prefix.map(|a| a.mult()).unwrap_or(1.0);
        let s = self.armor_suffix.map(|a| a.mult()).unwrap_or(1.0);
        let m = self.armor_material.map(|a| a.mult()).unwrap_or(1.0);
        (base as f32 * p * s * m).round().max(0.0) as i32
    }

    /// Texte des effets (préfixe/suffixe arme ou générique ou armure).
    pub fn effects_text(&self) -> String {
        if self.slot.is_weapon_or_shield() {
            let mut parts = Vec::new();
            let dmg_pct = self.weapon_damage_pct_bonus();
            if dmg_pct != 0 {
                parts.push(format!("Dégâts : {:+}%", dmg_pct));
            }
            if let Some(block) = self.block_chance_base {
                parts.push(format!("Blocage : {}% (base)", block));
            }
            if let Some(dmg) = self.base_damage {
                parts.push(format!("Dégâts de base : {}", dmg));
            }
            if parts.is_empty() {
                "Aucun effet particulier.".to_string()
            } else {
                parts.join(" — ")
            }
        } else if self.base_armor.is_some() {
            let arm = self.effective_armor();
            format!("Armure : {} (réduction plate, dégâts min subis = 1)", arm)
        } else {
            let mut parts = Vec::new();
            if let Some(ref p) = self.prefix {
                parts.push(format!("Préfixe: {}", p));
            }
            if let Some(ref s) = self.suffix {
                parts.push(format!("Suffixe: {}", s));
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
        let (base_name, base_damage, block_chance_base) = roll_weapon_base(slot, roll);
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
            base_damage: base_damage,
            block_chance_base,
            base_armor: None,
            armor_prefix: None,
            armor_suffix: None,
            armor_material: None,
        }
    } else if slot.is_ammo() {
        let wm = roll_weapon_material(roll);
        let (base_name, _, _) = roll_weapon_base(slot, roll);
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
            base_damage: None,
            block_chance_base: None,
            base_armor: None,
            armor_prefix: None,
            armor_suffix: None,
            armor_material: None,
        }
    } else if matches!(slot, ItemSlot::Chest | ItemSlot::Head | ItemSlot::Gloves | ItemSlot::Shoulders | ItemSlot::Legs | ItemSlot::Feet) {
        let ap = roll_armor_prefix(roll);
        let as_ = roll_armor_suffix(roll);
        let am = roll_armor_material(roll);
        let (base_name, base_armor) = roll_armor_base(slot, roll).unwrap_or_else(|| (String::from("Objet"), 0));
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
            base_damage: None,
            block_chance_base: None,
            base_armor: Some(base_armor),
            armor_prefix: Some(ap),
            armor_suffix: Some(as_),
            armor_material: Some(am),
        }
    } else {
        let (base_name, _) = roll_base_item_identification(slot, rarity, roll, wisdom).unwrap_or_else(|| (String::from("Objet"), slot));
        let prefix = if roll() < 0.3 + wisdom as f32 * 0.2 {
            Some(roll_prefix(roll))
        } else {
            None
        };
        let suffix = if roll() < 0.3 + wisdom as f32 * 0.2 {
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
            base_damage: None,
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
    } else {
        WeaponSuffix::DeMaitre
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
    } else if d < 95 {
        WeaponMaterial::Argent
    } else {
        WeaponMaterial::Adamante
    }
}

/// Armes à une main : (nom, dégâts de base min).
const ONE_HAND_WEAPONS: &[(&str, i32)] = &[
    ("Dague", 1),
    ("Hachoir", 2),
    ("Hachette", 3),
    ("Hache", 4),
    ("Hache d'arme", 5),
    ("Bec de Corbin", 6),
    ("Fléau", 7),
    ("Marteau", 4),
    ("Masse", 5),
    ("Cimeterre", 5),
    ("Épée courte", 4),
    ("Épée longue", 6),
    ("Sabre", 6),
    ("Fleuret", 5),
    ("Rapière", 6),
    ("Lance", 5),
];

const TWO_HAND_WEAPONS: &[&str] = &[
    "Hache à deux mains",
    "Maul",
    "Fléau à deux mains",
    "Épée à deux mains",
    "Bardiche",
    "Faux de guerre",
    "Hallebarde",
    "Lance d'arçon",
    "Lance de cavalerie",
    "Pique",
    "Trident",
];

const RANGED_WEAPONS: &[&str] = &[
    "Couteau de lancer",
    "Hache de lancer",
    "Javelot",
    "Arc court",
    "Arc composite",
    "Arc long",
    "Arc long composite",
    "Arbalète",
    "Arbalète lourde",
    "Arbalète à répétition",
    "Rabateur",
    "Pistolet à silex",
    "Revolver",
    "Pistolet automatique",
    "Fusil de chasse",
    "Fusil à culasse",
];

const AMMO_NAMES: &[&str] = &[
    "Flèches",
    "Carreaux",
    "9mm",
    "365 magnum",
    "Cal12",
    "Cartouche poudre noir",
];

/// Boucliers : (nom, blocage base %). Formule jeu : bloc_base + Agi (ex. Targe 7%+agi).
const SHIELD_NAMES: &[(&str, f32)] = &[
    ("Targe", 7.0),
    ("Rondache", 10.0),
    ("Écu", 13.0),
    ("Grand Bouclier", 15.0),
    ("Égide", 18.0),
    ("Pavois", 20.0),
];

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
    } else {
        ArmorSuffix::DeMaitre
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

fn roll_weapon_base(
    slot: ItemSlot,
    roll: &mut impl FnMut() -> f32,
) -> (String, Option<i32>, Option<f32>) {
    match slot {
        ItemSlot::MainHand | ItemSlot::OffHand => {
            let kind = roll();
            if kind < 0.35 {
                let idx = (roll() * ONE_HAND_WEAPONS.len() as f32).min(ONE_HAND_WEAPONS.len() as f32 - 1.0) as usize;
                let (name, dmg) = ONE_HAND_WEAPONS[idx];
                (name.to_string(), Some(dmg.max(1)), None)
            } else if kind < 0.55 {
                let idx = (roll() * TWO_HAND_WEAPONS.len() as f32).min(TWO_HAND_WEAPONS.len() as f32 - 1.0) as usize;
                (TWO_HAND_WEAPONS[idx].to_string(), Some(6), None)
            } else if kind < 0.80 {
                let idx = (roll() * RANGED_WEAPONS.len() as f32).min(RANGED_WEAPONS.len() as f32 - 1.0) as usize;
                (RANGED_WEAPONS[idx].to_string(), Some(4), None)
            } else {
                let idx = (roll() * SHIELD_NAMES.len() as f32).min(SHIELD_NAMES.len() as f32 - 1.0) as usize;
                let (name, block) = SHIELD_NAMES[idx];
                (name.to_string(), None, Some(block))
            }
        }
        ItemSlot::Ammo => {
            let idx = (roll() * AMMO_NAMES.len() as f32).min(AMMO_NAMES.len() as f32 - 1.0) as usize;
            (AMMO_NAMES[idx].to_string(), None, None)
        }
        _ => (String::from("Objet"), None, None),
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
        (ItemSlot::Head, ItemRarity::Common) => (vec!["Capuche", "Chapeau de cuir", "Bandana"], ItemSlot::Head),
        (ItemSlot::Head, ItemRarity::Uncommon) => (vec!["Heaume léger", "Coiffe de mage"], ItemSlot::Head),
        (ItemSlot::Head, ItemRarity::Rare) => (vec!["Couronne de fer"], ItemSlot::Head),
        (ItemSlot::Head, ItemRarity::Magical) => (vec!["Diadème de clarté"], ItemSlot::Head),
        (ItemSlot::Head, ItemRarity::UltraRare) => (vec!["Couronne du souverain"], ItemSlot::Head),
        (ItemSlot::Head, ItemRarity::Unique) => (vec!["Visage de Nawak"], ItemSlot::Head),
        (ItemSlot::Neck, _) => (vec!["Collier", "Amulette", "Medaillon"], ItemSlot::Neck),
        (ItemSlot::Shoulders, _) => (vec!["Spalières", "Épaulières", "Mantelet"], ItemSlot::Shoulders),
        (ItemSlot::Bracer, _) => (vec!["Brassard", "Protège-bras", "Manchette"], ItemSlot::Bracer),
        (ItemSlot::Gloves, _) => (vec!["Gants", "Gantelets", "Mitaines"], ItemSlot::Gloves),
        (ItemSlot::Ring1, _) | (ItemSlot::Ring2, _) => (vec!["Bague", "Anneau", "Chevalière"], slot),
        (ItemSlot::Chest, ItemRarity::Common) => (vec!["Tunique", "Veste en cuir", "Haubert usé"], ItemSlot::Chest),
        (ItemSlot::Chest, ItemRarity::Uncommon) => (vec!["Armure de plaques légère", "Robe de voyage"], ItemSlot::Chest),
        (ItemSlot::Chest, ItemRarity::Rare) => (vec!["Cuirasse du château"], ItemSlot::Chest),
        (ItemSlot::Chest, ItemRarity::Magical) => (vec!["Cuirasse runique"], ItemSlot::Chest),
        (ItemSlot::Chest, ItemRarity::UltraRare) => (vec!["Plastron immortel"], ItemSlot::Chest),
        (ItemSlot::Chest, ItemRarity::Unique) => (vec!["Cœur de pierre"], ItemSlot::Chest),
        (ItemSlot::Belt, _) => (vec!["Ceinture", "Baudrier", "Sash"], ItemSlot::Belt),
        (ItemSlot::Legs, _) => (vec!["Jambières", "Grèves", "Pantalon de cuir"], ItemSlot::Legs),
        (ItemSlot::Feet, ItemRarity::Common) => (vec!["Bottes en cuir", "Sandales", "Souliers usés"], ItemSlot::Feet),
        (ItemSlot::Feet, ItemRarity::Uncommon) => (vec!["Bottes de marche", "Grèves légères"], ItemSlot::Feet),
        (ItemSlot::Feet, ItemRarity::Rare) => (vec!["Bottes de guerre"], ItemSlot::Feet),
        (ItemSlot::Feet, ItemRarity::Magical) => (vec!["Bottes de vent"], ItemSlot::Feet),
        (ItemSlot::Feet, ItemRarity::UltraRare) => (vec!["Marcheur des ombres"], ItemSlot::Feet),
        (ItemSlot::Feet, ItemRarity::Unique) => (vec!["Pas de géant"], ItemSlot::Feet),
        (ItemSlot::Consumable, ItemRarity::Common) => (vec!["Potion de vie mineure", "Pain", "Eau"], ItemSlot::Consumable),
        (ItemSlot::Consumable, ItemRarity::Uncommon) => (vec!["Potion de vie", "Elixir de force"], ItemSlot::Consumable),
        (ItemSlot::Consumable, ItemRarity::Rare) => (vec!["Potion de vie majeure", "Pierre de rappel"], ItemSlot::Consumable),
        (ItemSlot::Consumable, ItemRarity::Magical) => (vec!["Elixir de résistance", "Potion de mana"], ItemSlot::Consumable),
        (ItemSlot::Consumable, ItemRarity::UltraRare) => (vec!["Pierre de renaissance"], ItemSlot::Consumable),
        (ItemSlot::Consumable, ItemRarity::Unique) => (vec!["Larme de Nawak"], ItemSlot::Consumable),
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
