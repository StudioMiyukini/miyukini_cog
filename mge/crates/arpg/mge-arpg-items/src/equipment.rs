// @id: MGE-ARPG-Items-Equipment @do: equipment-slots @role: back-end @layer: 3 @human: miyuk
//! Character equipment slots.
//!
//! Manages the 10 named equipment slots a character can wear items in.
//! Equipping an item into an already-occupied slot returns the previous item.

use crate::instance::ItemInstance;
use crate::slot::ItemSlot;

/// The set of equipment slots a character wears.
///
/// Each slot holds at most one `ItemInstance`. Equipping returns the previously
/// equipped item (if any) so the caller can route it to inventory or drop it.
#[derive(Debug, Clone, Default)]
pub struct Equipment {
    helm: Option<ItemInstance>,
    armor: Option<ItemInstance>,
    gloves: Option<ItemInstance>,
    belt: Option<ItemInstance>,
    boots: Option<ItemInstance>,
    amulet: Option<ItemInstance>,
    ring1: Option<ItemInstance>,
    ring2: Option<ItemInstance>,
    weapon_main: Option<ItemInstance>,
    weapon_off: Option<ItemInstance>,
}

impl Equipment {
    /// Create a new equipment set with all slots empty.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return a mutable reference to the slot backing the given `ItemSlot`.
    ///
    /// `Inventory` and `Stash` are not equipment slots; they return `None`.
    fn slot_mut(&mut self, slot: ItemSlot) -> Option<&mut Option<ItemInstance>> {
        match slot {
            ItemSlot::Helm => Some(&mut self.helm),
            ItemSlot::Armor => Some(&mut self.armor),
            ItemSlot::Gloves => Some(&mut self.gloves),
            ItemSlot::Belt => Some(&mut self.belt),
            ItemSlot::Boots => Some(&mut self.boots),
            ItemSlot::Amulet => Some(&mut self.amulet),
            ItemSlot::Ring1 => Some(&mut self.ring1),
            ItemSlot::Ring2 => Some(&mut self.ring2),
            ItemSlot::WeaponMain => Some(&mut self.weapon_main),
            ItemSlot::WeaponOff => Some(&mut self.weapon_off),
            ItemSlot::Inventory | ItemSlot::Stash => None,
        }
    }

    /// Return a reference to the slot backing the given `ItemSlot`.
    fn slot_ref(&self, slot: ItemSlot) -> Option<&Option<ItemInstance>> {
        match slot {
            ItemSlot::Helm => Some(&self.helm),
            ItemSlot::Armor => Some(&self.armor),
            ItemSlot::Gloves => Some(&self.gloves),
            ItemSlot::Belt => Some(&self.belt),
            ItemSlot::Boots => Some(&self.boots),
            ItemSlot::Amulet => Some(&self.amulet),
            ItemSlot::Ring1 => Some(&self.ring1),
            ItemSlot::Ring2 => Some(&self.ring2),
            ItemSlot::WeaponMain => Some(&self.weapon_main),
            ItemSlot::WeaponOff => Some(&self.weapon_off),
            ItemSlot::Inventory | ItemSlot::Stash => None,
        }
    }

    /// Equip an item into the given slot, returning the previously equipped item.
    ///
    /// If `slot` is `Inventory` or `Stash`, the item is returned unchanged
    /// (those are not equipment slots).
    pub fn equip(&mut self, slot: ItemSlot, item: ItemInstance) -> Option<ItemInstance> {
        if let Some(cell) = self.slot_mut(slot) {
            let previous = cell.take();
            *cell = Some(item);
            previous
        } else {
            // Not an equipment slot — give the item back.
            Some(item)
        }
    }

    /// Remove and return the item from the given equipment slot.
    pub fn unequip(&mut self, slot: ItemSlot) -> Option<ItemInstance> {
        self.slot_mut(slot).and_then(Option::take)
    }

    /// Get a reference to the item in the given equipment slot.
    #[must_use]
    pub fn get(&self, slot: ItemSlot) -> Option<&ItemInstance> {
        self.slot_ref(slot).and_then(|cell| cell.as_ref())
    }
}
