use serde::{Deserialize, Serialize};

use crate::economy::{buy_price, sell_price};
use crate::item::{BaseItemDef, EquipRequirements, Item, ItemKind};
use crate::rarity::Rarity;

/// Type of town vendor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VendorKind {
    Weapon,
    Armor,
    Potion,
    Scroll,
    Gamble,
}

/// A vendor's current item stock.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorCatalog {
    pub vendor: VendorKind,
    pub items: Vec<Item>,
    /// Item level of the current stock (set by area level).
    pub stock_ilvl: u32,
}

impl VendorCatalog {
    pub fn new(vendor: VendorKind, stock_ilvl: u32) -> Self {
        Self { vendor, items: Vec::new(), stock_ilvl }
    }

    /// Replace catalog with a refreshed item list.
    pub fn refresh(&mut self, new_items: Vec<Item>) {
        self.items = new_items;
    }

    /// Buy an item by uid. Returns the item if found and the player can afford it.
    pub fn buy(&mut self, uid: u64, player_gold: &mut u64) -> Option<Item> {
        let pos = self.items.iter().position(|i| i.uid == uid)?;
        let price = buy_price(self.items[pos].base.kind, self.items[pos].ilvl);
        if *player_gold < price {
            return None;
        }
        *player_gold -= price;
        Some(self.items.remove(pos))
    }

    /// Calculate gold the vendor pays for the given item (does not mutate state).
    pub fn sell(&self, item: &Item) -> u64 {
        sell_price(item.base.kind, item.ilvl)
    }
}

/// Identify an item in place (consumes a scroll; scroll removal is caller's responsibility).
pub fn identify(item: &mut Item) {
    item.identified = true;
}

/// Repair cost for an item (delegates to economy).
pub fn repair_item_cost(item: &Item) -> u64 {
    crate::economy::repair_cost(item.base.kind)
}

/// Gamble: returns a magic-rarity item of the given kind.
/// `roll` is in `[0, 1000)` for deterministic testing.
pub fn gamble_item(uid: u64, kind: ItemKind, ilvl: u32, roll: u32) -> Item {
    let rarity = if roll < 50 { Rarity::Rare } else { Rarity::Magic };
    Item::new(
        uid,
        BaseItemDef {
            id: 0,
            name: kind_display_name(kind).into(),
            kind,
            max_sockets: 0,
            requirements: EquipRequirements::default(),
            ilvl,
            two_handed: false,
        },
        rarity,
        ilvl,
    )
}

fn kind_display_name(kind: ItemKind) -> &'static str {
    match kind {
        ItemKind::Ring => "Ring",
        ItemKind::Amulet => "Amulet",
        ItemKind::Belt => "Belt",
        ItemKind::Helm => "Helm",
        ItemKind::Boots => "Boots",
        ItemKind::Gloves => "Gloves",
        _ => "Item",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_item(uid: u64, kind: ItemKind) -> Item {
        Item::new(
            uid,
            BaseItemDef {
                id: uid as u32,
                name: "Test".into(),
                kind,
                max_sockets: 0,
                requirements: EquipRequirements::default(),
                ilvl: 5,
                two_handed: false,
            },
            Rarity::Normal,
            5,
        )
    }

    #[test]
    fn buy_deducts_gold() {
        let mut catalog = VendorCatalog::new(VendorKind::Weapon, 5);
        catalog.items.push(make_item(1, ItemKind::Sword));
        let mut gold = 10_000u64;
        assert!(catalog.buy(1, &mut gold).is_some());
        assert!(gold < 10_000);
    }

    #[test]
    fn buy_fails_insufficient_gold() {
        let mut catalog = VendorCatalog::new(VendorKind::Weapon, 5);
        catalog.items.push(make_item(1, ItemKind::Sword));
        let mut gold = 0u64;
        assert!(catalog.buy(1, &mut gold).is_none());
    }

    #[test]
    fn sell_returns_positive_gold() {
        let catalog = VendorCatalog::new(VendorKind::Weapon, 5);
        let item = make_item(1, ItemKind::Sword);
        assert!(catalog.sell(&item) > 0);
    }

    #[test]
    fn identify_marks_item_identified() {
        let mut item = make_item(1, ItemKind::Sword);
        item.identified = false;
        identify(&mut item);
        assert!(item.identified);
    }

    #[test]
    fn gamble_produces_magic_or_rare() {
        let item = gamble_item(99, ItemKind::Ring, 10, 500);
        assert!(matches!(item.rarity, Rarity::Magic | Rarity::Rare));
    }
}
