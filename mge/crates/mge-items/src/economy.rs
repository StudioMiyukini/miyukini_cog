use serde::{Deserialize, Serialize};

use crate::item::{ItemFamily, ItemKind};

/// In-world gold currency.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Gold {
    pub amount: u64,
}

impl Gold {
    /// Maximum gold a character can carry in their personal inventory.
    pub const MAX_INVENTORY: u64 = 10_000;
    /// Maximum gold a character can hold in stash.
    pub const MAX_STASH: u64 = 2_500_000;

    pub fn new(amount: u64) -> Self {
        Self { amount }
    }

    #[must_use]
    pub fn clamp_inventory(self) -> Self {
        Self { amount: self.amount.min(Self::MAX_INVENTORY) }
    }
}

/// Base vendor price for an item kind (in gold).
pub fn base_price(kind: ItemKind) -> u64 {
    match kind {
        ItemKind::Sword | ItemKind::Axe | ItemKind::Mace => 300,
        ItemKind::Dagger => 150,
        ItemKind::Bow | ItemKind::Crossbow | ItemKind::Amulet => 500,
        ItemKind::Staff | ItemKind::Wand | ItemKind::Scepter | ItemKind::GrandCharm => 400,
        ItemKind::Helm | ItemKind::LargeCharm => 200,
        ItemKind::BodyArmor => 600,
        ItemKind::Belt | ItemKind::SmallCharm => 100,
        ItemKind::Gloves | ItemKind::Boots => 120,
        ItemKind::Shield => 250,
        ItemKind::Ring => 350,
        ItemKind::ScrollOfIdentify | ItemKind::TownPortalScroll => 10,
        ItemKind::TomeOfIdentify => 80,
        ItemKind::Key => 5,
        ItemKind::Gem | ItemKind::Rune | ItemKind::Jewel => 50,
        ItemKind::Gold => 1,
    }
}

/// Price to buy from a vendor (scales slightly with item level).
pub fn buy_price(kind: ItemKind, ilvl: u32) -> u64 {
    let scale = 1 + u64::from(ilvl) / 10;
    base_price(kind) * scale
}

/// Price vendor pays when player sells (1/6 of buy price, min 1).
pub fn sell_price(kind: ItemKind, ilvl: u32) -> u64 {
    (buy_price(kind, ilvl) / 6).max(1)
}

/// Flat repair cost for an item, by family.
pub fn repair_cost(kind: ItemKind) -> u64 {
    match kind.family() {
        ItemFamily::Weapon => 80,
        ItemFamily::Armor => 60,
        _ => 10,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buy_price_scales_with_ilvl() {
        let low = buy_price(ItemKind::Sword, 1);
        let high = buy_price(ItemKind::Sword, 20);
        assert!(high >= low);
    }

    #[test]
    fn sell_price_is_fraction_of_buy() {
        let buy = buy_price(ItemKind::Sword, 1);
        let sell = sell_price(ItemKind::Sword, 1);
        assert!(sell < buy);
    }

    #[test]
    fn gold_clamp_inventory() {
        let g = Gold::new(50_000).clamp_inventory();
        assert_eq!(g.amount, Gold::MAX_INVENTORY);
    }

    #[test]
    fn repair_cost_weapon_higher_than_misc() {
        assert!(repair_cost(ItemKind::Sword) > repair_cost(ItemKind::Key));
    }
}
