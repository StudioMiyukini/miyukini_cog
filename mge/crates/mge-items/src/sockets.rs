use serde::{Deserialize, Serialize};

use crate::item::{Item, ItemFamily};
use crate::rarity::Rarity;
use crate::socketables::SocketableKind;

/// An item instance with populated sockets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketedItem {
    pub item_uid: u64,
    pub max_sockets: u8,
    /// Contents of each socket slot (None = empty).
    pub sockets: Vec<Option<SocketableKind>>,
}

impl SocketedItem {
    pub fn new(item: &Item) -> Self {
        let max = item.base.max_sockets;
        Self { item_uid: item.uid, max_sockets: max, sockets: vec![None; usize::from(max)] }
    }

    pub fn empty_count(&self) -> usize {
        self.sockets.iter().filter(|s| s.is_none()).count()
    }

    pub fn filled_count(&self) -> usize {
        self.sockets.iter().filter(|s| s.is_some()).count()
    }

    /// Insert a socketable into the next available slot. Returns false if full.
    pub fn insert(&mut self, kind: SocketableKind) -> bool {
        if let Some(slot) = self.sockets.iter_mut().find(|s| s.is_none()) {
            *slot = Some(kind);
            true
        } else {
            false
        }
    }

    /// Remove all socketables (items are destroyed, D2-style).
    pub fn clear(&mut self) {
        for s in &mut self.sockets {
            *s = None;
        }
    }
}

/// Whether an item may receive additional sockets (Normal/Magic weapons and armour with socket space).
pub fn can_add_sockets(item: &Item) -> bool {
    matches!(item.base.kind.family(), ItemFamily::Weapon | ItemFamily::Armor)
        && matches!(item.rarity, Rarity::Normal | Rarity::Magic)
        && item.base.max_sockets > 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::{BaseItemDef, EquipRequirements, Item, ItemKind};

    fn socketed_sword() -> SocketedItem {
        let base = BaseItemDef {
            id: 1,
            name: "Short Sword".into(),
            kind: ItemKind::Sword,
            max_sockets: 2,
            requirements: EquipRequirements::default(),
            ilvl: 1,
            two_handed: false,
        };
        let item = Item::new(1, base, Rarity::Normal, 1);
        SocketedItem::new(&item)
    }

    #[test]
    fn insert_fills_slot() {
        let mut si = socketed_sword();
        assert_eq!(si.empty_count(), 2);
        assert!(si.insert(SocketableKind::Gem));
        assert_eq!(si.filled_count(), 1);
    }

    #[test]
    fn insert_fails_when_full() {
        let mut si = socketed_sword();
        si.insert(SocketableKind::Gem);
        si.insert(SocketableKind::Gem);
        assert!(!si.insert(SocketableKind::Rune));
    }

    #[test]
    fn clear_empties_all_sockets() {
        let mut si = socketed_sword();
        si.insert(SocketableKind::Gem);
        si.clear();
        assert_eq!(si.empty_count(), 2);
    }
}
