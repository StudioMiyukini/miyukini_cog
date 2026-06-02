//! Equipped-gear components.

use std::collections::HashMap;

use mge_gameplay::stats::StatModifier;
use mge_items::item::EquipSlot;

use crate::components::item::ItemInstance;

/// One equipped item plus the slot it occupies.
#[derive(Debug, Clone)]
pub struct EquippedItem {
    pub slot: EquipSlot,
    pub item: ItemInstance,
}

/// Worn equipment, one item per slot.
#[derive(Debug, Clone, Default)]
pub struct EquipmentSlots {
    pub slots: HashMap<EquipSlot, ItemInstance>,
}

impl EquipmentSlots {
    pub fn new() -> Self {
        Self::default()
    }

    /// Equip into the item's natural slot, returning whatever was there.
    pub fn equip(&mut self, item: ItemInstance) -> Result<Option<ItemInstance>, ItemInstance> {
        match item.slot() {
            Some(slot) => Ok(self.slots.insert(slot, item)),
            None => Err(item),
        }
    }

    /// Force-equip `item` into an explicit `slot`, returning whatever was there.
    /// Used to dual-wield a one-handed melee weapon into the off-hand, which the
    /// natural [`ItemInstance::slot`] mapping would otherwise route to MainHand.
    pub fn equip_to(&mut self, slot: EquipSlot, item: ItemInstance) -> Option<ItemInstance> {
        self.slots.insert(slot, item)
    }

    pub fn unequip(&mut self, slot: EquipSlot) -> Option<ItemInstance> {
        self.slots.remove(&slot)
    }

    pub fn get(&self, slot: EquipSlot) -> Option<&ItemInstance> {
        self.slots.get(&slot)
    }

    pub fn iter(&self) -> impl Iterator<Item = EquippedItem> + '_ {
        self.slots.iter().map(|(slot, item)| EquippedItem { slot: *slot, item: item.clone() })
    }

    /// Every stat modifier granted by all worn items.
    pub fn all_modifiers(&self) -> Vec<StatModifier> {
        self.slots.values().flat_map(ItemInstance::stat_modifiers).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }
}

/// Inventory grid width in slots.
pub const INV_WIDTH: usize = 10;
/// Inventory grid height in slots.
pub const INV_HEIGHT: usize = 4;
/// Total backpack capacity.
pub const INV_CAPACITY: usize = INV_WIDTH * INV_HEIGHT;

/// A fixed 10x4 backpack. Slots are row-major; `None` is empty.
#[derive(Debug, Clone)]
pub struct Inventory {
    pub slots: Vec<Option<ItemInstance>>,
    pub gold: u64,
    /// Crafting materials salvaged at the blacksmith.
    pub materials: crate::systems::forge::Materials,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: vec![None; INV_CAPACITY],
            gold: 0,
            materials: crate::systems::forge::Materials::default(),
        }
    }
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    pub fn free(&self) -> usize {
        INV_CAPACITY - self.count()
    }

    pub fn is_full(&self) -> bool {
        self.free() == 0
    }

    /// Add to the first empty slot. Returns the slot index, or `Err(item)` if full.
    pub fn add(&mut self, item: ItemInstance) -> Result<usize, ItemInstance> {
        match self.slots.iter().position(Option::is_none) {
            Some(i) => {
                self.slots[i] = Some(item);
                Ok(i)
            }
            None => Err(item),
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<ItemInstance> {
        self.slots.get_mut(index).and_then(Option::take)
    }

    pub fn get(&self, index: usize) -> Option<&ItemInstance> {
        self.slots.get(index).and_then(Option::as_ref)
    }

    /// Move/swap the contents of two slots (drag & drop).
    pub fn move_slot(&mut self, from: usize, to: usize) -> bool {
        if from >= INV_CAPACITY || to >= INV_CAPACITY || from == to {
            return false;
        }
        self.slots.swap(from, to);
        true
    }

    pub fn add_gold(&mut self, amount: u64) {
        self.gold += amount;
    }
}
