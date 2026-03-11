use serde::{Deserialize, Serialize};

use crate::entity::{Equipment, PotionKind};
use crate::skill_disk::{ScrollItem, SlotContent};

// ---------------------------------------------------------------------------
// Inventory item — anything the player can carry in their backpack
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InvItem {
    Equipment(Equipment),
    Potion(PotionKind),
    Scroll(ScrollItem),
    Gold(u32),
}

impl InvItem {
    /// Short label for rendering (max ~10 bytes).
    pub fn label(&self) -> &'static [u8] {
        match self {
            Self::Equipment(eq) => eq.slot.name(),
            Self::Potion(PotionKind::Health) => b"HP POT",
            Self::Potion(PotionKind::Mana) => b"MP POT",
            Self::Scroll(s) => s.content.short_label(),
            Self::Gold(_) => b"GOLD",
        }
    }

    /// Color tint for rendering.
    pub fn color(&self) -> [f32; 4] {
        match self {
            Self::Equipment(eq) => eq.rarity.color(),
            Self::Potion(PotionKind::Health) => [0.9, 0.25, 0.20, 1.0],
            Self::Potion(PotionKind::Mana) => [0.20, 0.35, 0.90, 1.0],
            Self::Scroll(s) => match s.content {
                SlotContent::Passive { .. } => [0.50, 0.75, 1.0, 1.0],
                SlotContent::Special(_) => [0.95, 0.90, 0.70, 1.0],
                SlotContent::Active { .. } => [0.90, 0.55, 0.55, 1.0],
            },
            Self::Gold(_) => [1.0, 0.85, 0.25, 1.0],
        }
    }

    /// Can this item stack? Only gold stacks.
    pub fn is_gold(&self) -> bool {
        matches!(self, Self::Gold(_))
    }
}

// ---------------------------------------------------------------------------
// Backpack — fixed-size inventory grid
// ---------------------------------------------------------------------------

/// Starting backpack size.
pub const BACKPACK_BASE_SLOTS: usize = 16;
/// Maximum backpack size (with upgrades).
pub const BACKPACK_MAX_SLOTS: usize = 40;
/// Cost per extra row of 4 slots.
pub const BACKPACK_UPGRADE_COST: u32 = 50;
/// Slots per row in the UI grid.
pub const BACKPACK_COLS: usize = 4;

#[derive(Debug, Clone)]
pub struct Backpack {
    /// Inventory slots. `None` = empty.
    pub slots: Vec<Option<InvItem>>,
    /// Current capacity (can be upgraded).
    pub capacity: usize,
}

impl Default for Backpack {
    fn default() -> Self {
        Self::new(BACKPACK_BASE_SLOTS)
    }
}

impl Backpack {
    pub fn new(capacity: usize) -> Self {
        Self {
            slots: vec![None; capacity],
            capacity,
        }
    }

    /// Number of items currently stored.
    pub fn count(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    /// Number of free slots.
    pub fn free(&self) -> usize {
        self.capacity - self.count()
    }

    pub fn is_full(&self) -> bool {
        self.free() == 0
    }

    /// Try to add an item. Gold stacks into existing gold slots first.
    /// Returns `true` if inserted, `false` if no room.
    pub fn add(&mut self, item: InvItem) -> bool {
        // Gold stacking
        if let InvItem::Gold(amount) = item {
            for slot in &mut self.slots {
                if let Some(InvItem::Gold(ref mut g)) = slot {
                    *g += amount;
                    return true;
                }
            }
        }
        // Find first empty slot
        if let Some(slot) = self.slots.iter_mut().find(|s| s.is_none()) {
            *slot = Some(item);
            true
        } else {
            false
        }
    }

    /// Remove item at index. Returns the item if any.
    pub fn remove(&mut self, index: usize) -> Option<InvItem> {
        if index >= self.capacity { return None; }
        self.slots[index].take()
    }

    /// Get item at index (immutable).
    pub fn get(&self, index: usize) -> Option<&InvItem> {
        self.slots.get(index).and_then(|s| s.as_ref())
    }

    /// Expand capacity by one row (4 slots). Returns new capacity.
    pub fn upgrade(&mut self) -> usize {
        let new_cap = (self.capacity + BACKPACK_COLS).min(BACKPACK_MAX_SLOTS);
        if new_cap > self.capacity {
            self.slots.resize(new_cap, None);
            self.capacity = new_cap;
        }
        self.capacity
    }

    /// Number of rows in the grid.
    pub fn rows(&self) -> usize {
        self.capacity.div_ceil(BACKPACK_COLS)
    }

    /// Can the backpack be upgraded further?
    pub fn can_upgrade(&self) -> bool {
        self.capacity < BACKPACK_MAX_SLOTS
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{EquipSlot, ItemRarity};
    use crate::skill_disk::ScrollStat;

    #[test]
    fn backpack_add_remove() {
        let mut bp = Backpack::new(4);
        assert_eq!(bp.free(), 4);
        let eq = InvItem::Equipment(Equipment {
            slot: EquipSlot::Helm, rarity: ItemRarity::Magic, ilvl: 5,
        });
        assert!(bp.add(eq));
        assert_eq!(bp.count(), 1);
        assert_eq!(bp.free(), 3);
        let removed = bp.remove(0);
        assert!(removed.is_some());
        assert_eq!(bp.free(), 4);
    }

    #[test]
    fn backpack_full() {
        let mut bp = Backpack::new(2);
        assert!(bp.add(InvItem::Potion(PotionKind::Health)));
        assert!(bp.add(InvItem::Potion(PotionKind::Mana)));
        assert!(bp.is_full());
        assert!(!bp.add(InvItem::Potion(PotionKind::Health)));
    }

    #[test]
    fn gold_stacks() {
        let mut bp = Backpack::new(4);
        bp.add(InvItem::Gold(10));
        bp.add(InvItem::Gold(25));
        assert_eq!(bp.count(), 1); // stacked into one slot
        if let Some(InvItem::Gold(g)) = bp.get(0) {
            assert_eq!(*g, 35);
        } else {
            panic!("expected gold");
        }
    }

    #[test]
    fn upgrade() {
        let mut bp = Backpack::new(BACKPACK_BASE_SLOTS);
        let old = bp.capacity;
        bp.upgrade();
        assert_eq!(bp.capacity, old + BACKPACK_COLS);
        assert_eq!(bp.slots.len(), bp.capacity);
    }

    #[test]
    fn scroll_item() {
        let mut bp = Backpack::new(4);
        let scroll = InvItem::Scroll(ScrollItem {
            content: SlotContent::Passive { stat: ScrollStat::HpFlat, value: 10.0 },
        });
        assert!(bp.add(scroll));
        assert_eq!(bp.count(), 1);
        let label = bp.get(0).unwrap().label();
        assert_eq!(label, b"+HP");
    }
}
