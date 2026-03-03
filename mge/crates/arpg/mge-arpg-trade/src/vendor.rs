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
    /// Ticks until next inventory refresh (0 = refresh now).
    #[serde(default)]
    pub refresh_timer: u32,
}

impl NpcVendor {
    /// Create an empty vendor.
    pub fn new(npc_id: impl Into<String>) -> Self {
        Self {
            npc_id: npc_id.into(),
            items: Vec::new(),
            refresh_timer: 0,
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
        let price_i64 = i64::try_from(price).unwrap_or(i64::MAX);
        wallet
            .remove_gold(price_i64)
            .map_err(|_| TradeError::InsufficientGold {
                have: wallet.gold(),
                need: price_i64,
            })?;

        // Decrement stock if finite.
        if let Some(item) = self.find_item_mut(item_id) {
            if let Some(ref mut stock) = item.stock {
                *stock = stock.saturating_sub(1);
            }
        }

        Ok(item_id.to_string())
    }

    /// Player sells an item to the vendor.
    ///
    /// **SEC-21**: The sell price is read from the vendor's catalogue (`price_sell`),
    /// NOT from the client. If the item is not in the vendor's catalogue, a default
    /// sell price of `base_value / 4` is used.
    ///
    /// # Errors
    ///
    /// Returns [`TradeError::ItemNotFound`] if `item_id` is not recognized by the vendor
    /// and no `fallback_value` is provided.
    pub fn sell_to_vendor(
        &self,
        item_id: &str,
        wallet: &mut Wallet,
        fallback_value: Option<u64>,
    ) -> Result<u64, TradeError> {
        let sell_price = if let Some(vendor_item) = self.find_item(item_id) {
            vendor_item.price_sell
        } else if let Some(base_val) = fallback_value {
            base_val / 4
        } else {
            return Err(TradeError::ItemNotFound {
                item_id: item_id.to_string(),
            });
        };

        let gold_i64 = i64::try_from(sell_price).unwrap_or(i64::MAX);
        let _ = wallet.add_gold(gold_i64);
        Ok(sell_price)
    }

    /// Decrements the refresh timer. Returns `true` when a refresh is due.
    pub fn tick_refresh(&mut self) -> bool {
        if self.refresh_timer == 0 {
            return false;
        }
        self.refresh_timer = self.refresh_timer.saturating_sub(1);
        self.refresh_timer == 0
    }
}

/// Pre-built vendor for Act 1 (Charsi-style).
#[must_use]
pub fn act1_vendor() -> NpcVendor {
    let mut v = NpcVendor::new("charsi");
    v.add_item(VendorItem::new("short_sword", 250, 62));
    v.add_item(VendorItem::new("buckler", 100, 25));
    v.add_item(VendorItem::new("leather_armor", 200, 50));
    v.add_item(VendorItem::new("health_potion", 30, 7));
    v.add_item(VendorItem::new("mana_potion", 40, 10));
    v.add_item(VendorItem::new("scroll_identify", 80, 20));
    v.add_item(VendorItem::new("scroll_tp", 100, 25));
    v
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

    fn wallet_with(gold: i64) -> Wallet {
        let mut w = Wallet::new();
        w.add_gold(gold).unwrap();
        w
    }

    #[test]
    fn test_vendor_buy_success() {
        let mut vendor = make_vendor();
        let mut wallet = wallet_with(500);

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
        let mut wallet = wallet_with(10);

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

        let mut wallet = wallet_with(10_000);
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
        let mut wallet = wallet_with(1000);

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
        let mut wallet = wallet_with(100);

        // SEC-21: price comes from VendorItem.price_sell (40), not from the caller.
        vendor.sell_to_vendor("short_sword", &mut wallet, None).unwrap();
        assert_eq!(wallet.gold(), 140);
    }

    #[test]
    fn vendor_sell_price_from_def() {
        // SEC-21: price comes from vendor definition, not client
        let vendor = make_vendor();
        let mut wallet = wallet_with(0);
        let price = vendor.sell_to_vendor("short_sword", &mut wallet, None).unwrap();
        assert_eq!(price, 40, "sell price must come from VendorItem.price_sell");
        assert_eq!(wallet.gold(), 40);
    }

    #[test]
    fn vendor_sell_fallback_div4() {
        let vendor = make_vendor();
        let mut wallet = wallet_with(0);
        // Item not in vendor catalogue, fallback_value = 200 -> sell price = 200/4 = 50
        let price = vendor
            .sell_to_vendor("unknown_item", &mut wallet, Some(200))
            .unwrap();
        assert_eq!(price, 50, "fallback sell price must be base_value / 4");
    }

    #[test]
    fn vendor_sell_unknown_no_fallback() {
        let vendor = make_vendor();
        let mut wallet = wallet_with(0);
        let result = vendor.sell_to_vendor("unknown_item", &mut wallet, None);
        assert!(result.is_err(), "unknown item with no fallback must error");
    }

    #[test]
    fn act1_vendor_items() {
        let v = act1_vendor();
        assert_eq!(v.items.len(), 7, "Act 1 vendor must have 7 items");
        assert_eq!(v.npc_id, "charsi");
    }

    #[test]
    fn vendor_refresh_timer() {
        let mut v = act1_vendor();
        v.refresh_timer = 3;
        assert!(!v.tick_refresh()); // 3 -> 2
        assert!(!v.tick_refresh()); // 2 -> 1
        assert!(v.tick_refresh()); // 1 -> 0 -> true
    }

    #[test]
    fn test_vendor_find_item_not_found() {
        let vendor = make_vendor();
        assert!(vendor.find_item("legendary_axe").is_none());
    }
}
