// @id: MGE-ARPG-Trade-Vendor @do: npc-vendor @role: back-end @layer: 3 @human: miyuk
//! NPC vendor: buy and sell items for gold.

use crate::error::TradeError;
use crate::wallet::Wallet;

/// A single item in a vendor's stock.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VendorItem {
    /// Base item type identifier (e.g. `"short_sword"`).
    pub item_id: String,
    /// Gold cost for the player to buy this item from the vendor.
    pub price_buy: u64,
    /// Gold the player receives when selling this item to the vendor.
    pub price_sell: u64,
    /// Remaining stock. `None` means unlimited.
    pub stock: Option<u32>,
}

impl VendorItem {
    /// Create a new unlimited-stock vendor item.
    pub fn new(item_id: impl Into<String>, price_buy: u64, price_sell: u64) -> Self {
        Self {
            item_id: item_id.into(),
            price_buy,
            price_sell,
            stock: None,
        }
    }

    /// Builder: set a finite stock quantity.
    pub fn with_stock(mut self, quantity: u32) -> Self {
        self.stock = Some(quantity);
        self
    }

    /// Returns `true` when the item is available for purchase (unlimited or > 0).
    pub fn is_in_stock(&self) -> bool {
        self.stock.is_none_or(|s| s > 0)
    }
}

/// An NPC vendor that can buy and sell items.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NpcVendor {
    /// Unique identifier for this vendor NPC.
    pub npc_id: String,
    /// Catalogue of items offered by the vendor.
    pub items: Vec<VendorItem>,
}

impl NpcVendor {
    /// Create an empty vendor.
    pub fn new(npc_id: impl Into<String>) -> Self {
        Self {
            npc_id: npc_id.into(),
            items: Vec::new(),
        }
    }

    /// Add an item to the vendor's catalogue.
    pub fn add_item(&mut self, item: VendorItem) {
        self.items.push(item);
    }

    /// Look up a vendor item by its `item_id`.
    pub fn find_item(&self, item_id: &str) -> Option<&VendorItem> {
        self.items.iter().find(|i| i.item_id == item_id)
    }

    fn find_item_mut(&mut self, item_id: &str) -> Option<&mut VendorItem> {
        self.items.iter_mut().find(|i| i.item_id == item_id)
    }

    /// Player buys `item_id` from the vendor.
    ///
    /// Deducts gold from `wallet`, decrements stock (if finite), and returns the
    /// `item_id` of the purchased item.
    pub fn buy(&mut self, item_id: &str, wallet: &mut Wallet) -> Result<String, TradeError> {
        // Look up item and validate stock.
        let price = {
            let item = self.find_item(item_id).ok_or_else(|| TradeError::ItemNotFound {
                item_id: item_id.to_string(),
            })?;
            if !item.is_in_stock() {
                return Err(TradeError::OutOfStock {
                    item_id: item_id.to_string(),
                });
            }
            item.price_buy
        };

        // Withdraw gold (may fail with InsufficientGold).
        wallet.withdraw(price)?;

        // Decrement stock if finite.
        if let Some(item) = self.find_item_mut(item_id) {
            if let Some(ref mut stock) = item.stock {
                *stock = stock.saturating_sub(1);
            }
        }

        Ok(item_id.to_string())
    }

    /// Player sells an item to the vendor. Adds `price` gold to the player's wallet.
    pub fn sell_to_vendor(&self, _item_id: &str, wallet: &mut Wallet, price: u64) {
        wallet.deposit(price);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_vendor() -> NpcVendor {
        let mut v = NpcVendor::new("merchant-01");
        v.add_item(VendorItem::new("short_sword", 100, 40).with_stock(3));
        v.add_item(VendorItem::new("health_pot", 20, 8));
        v
    }

    #[test]
    fn test_vendor_buy_success() {
        let mut vendor = make_vendor();
        let mut wallet = Wallet::new(500);

        let bought = vendor.buy("short_sword", &mut wallet).unwrap();
        assert_eq!(bought, "short_sword");
        assert_eq!(wallet.gold(), 400);

        // Stock should have decremented from 3 to 2.
        let item = vendor.find_item("short_sword").unwrap();
        assert_eq!(item.stock, Some(2));
    }

    #[test]
    fn test_vendor_buy_insufficient_gold() {
        let mut vendor = make_vendor();
        let mut wallet = Wallet::new(10);

        let err = vendor.buy("short_sword", &mut wallet).unwrap_err();
        assert_eq!(
            err,
            TradeError::InsufficientGold {
                have: 10,
                need: 100,
            }
        );
    }

    #[test]
    fn test_vendor_buy_out_of_stock() {
        let mut vendor = NpcVendor::new("merchant-02");
        vendor.add_item(VendorItem::new("rare_gem", 500, 200).with_stock(0));

        let mut wallet = Wallet::new(10_000);
        let err = vendor.buy("rare_gem", &mut wallet).unwrap_err();
        assert_eq!(
            err,
            TradeError::OutOfStock {
                item_id: "rare_gem".to_string(),
            }
        );
    }

    #[test]
    fn test_vendor_buy_unlimited_stock() {
        let mut vendor = make_vendor();
        let mut wallet = Wallet::new(1000);

        // health_pot has unlimited stock -- buy it several times.
        vendor.buy("health_pot", &mut wallet).unwrap();
        vendor.buy("health_pot", &mut wallet).unwrap();
        vendor.buy("health_pot", &mut wallet).unwrap();

        assert_eq!(wallet.gold(), 940); // 1000 - 3 * 20

        // Stock is still None (unlimited).
        let item = vendor.find_item("health_pot").unwrap();
        assert!(item.stock.is_none());
    }

    #[test]
    fn test_vendor_sell_to_vendor() {
        let vendor = make_vendor();
        let mut wallet = Wallet::new(100);

        vendor.sell_to_vendor("short_sword", &mut wallet, 40);
        assert_eq!(wallet.gold(), 140);
    }

    #[test]
    fn test_vendor_find_item_not_found() {
        let vendor = make_vendor();
        assert!(vendor.find_item("legendary_axe").is_none());
    }
}
