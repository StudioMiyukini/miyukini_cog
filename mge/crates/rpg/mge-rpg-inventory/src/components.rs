//! @id mge.rpg.inventory.v1.components
//! @role data
//! @layer plugin
//! @domain rpg

use mge_ecs::{Component, EntityId};

/// @id mge.rpg.inventory.v1.component.inventory
/// @fields slots:Vec<Option<EntityId>>,capacity:usize
/// Conteneur d'items d'une entite (sac à dos).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Inventory {
    pub slots: Vec<Option<EntityId>>,
    pub capacity: usize,
}

impl Inventory {
    /// Cree un inventaire vide de capacité donnée.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: vec![None; capacity],
            capacity,
        }
    }

    /// Trouve le premier slot libre.
    pub fn first_free_slot(&self) -> Option<usize> {
        self.slots.iter().position(|s| s.is_none())
    }

    /// Nombre de slots occupes.
    pub fn used_count(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::with_capacity(20)
    }
}

impl Component for Inventory {}

/// Slots d'equipement : head, torso, legs, main, off, acc.
pub const EQUIP_SLOT_COUNT: usize = 6;

/// Index des slots d'equipement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EquipSlot {
    Head = 0,
    Torso = 1,
    Legs = 2,
    Main = 3,
    Off = 4,
    Acc = 5,
}

impl EquipSlot {
    pub fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Self::Head),
            1 => Some(Self::Torso),
            2 => Some(Self::Legs),
            3 => Some(Self::Main),
            4 => Some(Self::Off),
            5 => Some(Self::Acc),
            _ => None,
        }
    }
}

/// @id mge.rpg.inventory.v1.component.equipment
/// @fields slots:[Option<EntityId>;6]
/// Equipement d'une entite (head, torso, legs, main, off, acc).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Equipment {
    pub slots: [Option<EntityId>; EQUIP_SLOT_COUNT],
}

impl Equipment {
    pub fn get(&self, slot: EquipSlot) -> Option<EntityId> {
        self.slots[slot as usize]
    }

    pub fn set(&mut self, slot: EquipSlot, entity: Option<EntityId>) {
        self.slots[slot as usize] = entity;
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Self {
            slots: [None; EQUIP_SLOT_COUNT],
        }
    }
}

impl Component for Equipment {}

/// @id mge.rpg.inventory.v1.component.item
/// @fields type_id:u32,quality:u8,weight:f32,stack:u32
/// Donnees de base d'un item.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Item {
    pub type_id: u32,
    pub quality: u8,
    pub weight: f32,
    pub stack: u32,
}

impl Item {
    pub fn new(type_id: u32, stack: u32) -> Self {
        Self {
            type_id,
            quality: 0,
            weight: 0.0,
            stack,
        }
    }

    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality;
        self
    }

    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }
}

impl Component for Item {}

/// @id mge.rpg.inventory.v1.component.weapondata
/// @fields dmg_min:f64,dmg_max:f64,dmg_type:u8,range:f32
/// Donnees d'arme (attache a une entite Item).
#[derive(Debug, Clone)]
pub struct WeaponData {
    pub dmg_min: f64,
    pub dmg_max: f64,
    pub dmg_type: u8,
    pub range: f32,
}

impl Component for WeaponData {}

/// @id mge.rpg.inventory.v1.component.armordata
/// @fields ar:[f64;3],resistance:f64
/// Donnees d'armure (ar: [tranchant, contondant, perforant]).
#[derive(Debug, Clone)]
pub struct ArmorData {
    pub ar: [f64; 3],
    pub resistance: f64,
}

impl ArmorData {
    /// ar[0]=tranchant, ar[1]=contondant, ar[2]=perforant
    pub fn new(tranchant: f64, contondant: f64, perforant: f64, resistance: f64) -> Self {
        Self {
            ar: [tranchant, contondant, perforant],
            resistance,
        }
    }
}

impl Component for ArmorData {}

/// @id mge.rpg.inventory.v1.lootentry
/// @fields type_id:u32,weight:f64,qty_min:u32,qty_max:u32
/// Entree d'une table de loot (weight = chance relative).
#[derive(Debug, Clone)]
pub struct LootEntry {
    pub type_id: u32,
    pub weight: f64,
    pub qty_min: u32,
    pub qty_max: u32,
}

/// @id mge.rpg.inventory.v1.component.loottable
/// @fields entries:Vec<LootEntry>
/// Table de loot pour generation d'items au drop.
#[derive(Debug, Clone)]
pub struct LootTable {
    pub entries: Vec<LootEntry>,
}

impl LootTable {
    pub fn new(entries: Vec<LootEntry>) -> Self {
        Self { entries }
    }

    pub fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl Default for LootTable {
    fn default() -> Self {
        Self::empty()
    }
}

impl Component for LootTable {}
