//! @id     games_allumina_components_mercenary
//! @do     definir_merc_type
//! @role   ui
//! @layer  core
//! @human  games allumina — module « components mercenary ».

//! Mercenary components. A mercenary is an NPC companion hired at the village
//! that follows the player, fights at their side, gains a share of their XP, and
//! respawns a while after dying. Behaviour lives in
//! [`crate::systems::mercenary`].

use std::collections::HashMap;

use mge_items::item::EquipSlot;

use crate::components::item::ItemInstance;
use crate::ecs::Entity;

/// The four mercenary archetypes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MercType {
    /// Tanky melee bruiser; soaks hits beside the player.
    Guardian,
    /// Ranged damage dealer with high dodge.
    Archer,
    /// Support that heals the player when they are hurt.
    Healer,
    /// Caster that periodically lobs an elemental AoE.
    Mage,
}

impl MercType {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Guardian => "Guardian",
            Self::Archer => "Archer",
            Self::Healer => "Healer",
            Self::Mage => "Mage",
        }
    }

    pub fn all() -> [MercType; 4] {
        [Self::Guardian, Self::Archer, Self::Healer, Self::Mage]
    }
}

/// Marks an entity as a mercenary companion.
#[derive(Debug, Clone, Copy)]
pub struct MercenaryTag {
    pub merc_type: MercType,
    /// The player that hired this mercenary.
    pub owner: Entity,
    /// Whether the mercenary is currently in the player's service.
    pub hired: bool,
}

impl MercenaryTag {
    pub fn new(merc_type: MercType, owner: Entity, hired: bool) -> Self {
        Self { merc_type, owner, hired }
    }
}

/// A mercenary's worn equipment — the same ten [`EquipSlot`]s the player uses,
/// one item per slot. Items here contribute their stat modifiers to the
/// mercenary's resolved combat numbers.
#[derive(Debug, Clone, Default)]
pub struct MercInventory {
    pub slots: HashMap<EquipSlot, ItemInstance>,
}

impl MercInventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, slot: EquipSlot) -> Option<&ItemInstance> {
        self.slots.get(&slot)
    }

    /// Equip into the item's natural slot, returning whatever was there.
    /// `Err(item)` if the item is not equippable.
    pub fn equip(&mut self, item: ItemInstance) -> Result<Option<ItemInstance>, ItemInstance> {
        match item.slot() {
            Some(slot) => Ok(self.slots.insert(slot, item)),
            None => Err(item),
        }
    }

    pub fn unequip(&mut self, slot: EquipSlot) -> Option<ItemInstance> {
        self.slots.remove(&slot)
    }

    /// Force-equip into an explicit `slot` (e.g. a one-handed weapon into the
    /// off-hand for a dual-wield merc), returning whatever was there.
    pub fn equip_to(&mut self, slot: EquipSlot, item: ItemInstance) -> Option<ItemInstance> {
        self.slots.insert(slot, item)
    }

    /// Iterate over every equipped item.
    pub fn items(&self) -> impl Iterator<Item = &ItemInstance> {
        self.slots.values()
    }
}

/// Live runtime state of a mercenary: progression plus per-ability timers and
/// the death/respawn clock. Kept separate from [`MercenaryTag`] so the tag stays
/// a small descriptor.
#[derive(Debug, Clone, Copy)]
pub struct MercState {
    pub level: u32,
    pub xp: u64,
    /// Player experience total at the last XP sync (the mercenary earns a share
    /// of the player's gains).
    pub player_xp_mark: u64,
    /// Seconds until the next heal is allowed (Healer).
    pub heal_cd: f32,
    /// Seconds until the next AoE cast (Mage).
    pub cast_cd: f32,
    /// Seconds until the mercenary respawns. `> 0.0` means currently dead.
    pub respawn_timer: f32,
}

impl MercState {
    pub fn new(level: u32, player_xp_mark: u64) -> Self {
        Self {
            level,
            xp: 0,
            player_xp_mark,
            heal_cd: 0.0,
            cast_cd: 0.0,
            respawn_timer: 0.0,
        }
    }

    /// Whether the mercenary is currently dead and awaiting respawn.
    pub fn is_dead(&self) -> bool {
        self.respawn_timer > 0.0
    }
}
