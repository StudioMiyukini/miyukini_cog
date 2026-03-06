use serde::{Deserialize, Serialize};

/// Type of vendor interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VendorAction {
    Buy,
    Sell,
    Repair,
    Identify,
    Gamble,
}

/// A single listing in a vendor's inventory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorListing {
    pub item_id: u64,
    pub name: String,
    pub icon: String,
    pub price: u64,
    pub available: bool,
}

/// Current state of a vendor panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorState {
    pub vendor_name: String,
    pub available_actions: Vec<VendorAction>,
    pub active_action: VendorAction,
    pub listings: Vec<VendorListing>,
    pub player_gold: u64,
}

impl VendorState {
    pub fn new(name: impl Into<String>, actions: Vec<VendorAction>, gold: u64) -> Self {
        let active = actions.first().copied().unwrap_or(VendorAction::Buy);
        Self {
            vendor_name: name.into(),
            available_actions: actions,
            active_action: active,
            listings: Vec::new(),
            player_gold: gold,
        }
    }

    pub fn set_action(&mut self, action: VendorAction) -> bool {
        if self.available_actions.contains(&action) {
            self.active_action = action;
            true
        } else {
            false
        }
    }

    /// Check if the player can afford a listing.
    pub fn can_afford(&self, listing_index: usize) -> bool {
        self.listings.get(listing_index).is_some_and(|l| l.available && l.price <= self.player_gold)
    }

    /// Execute a buy. Returns the item ID if successful.
    pub fn try_buy(&mut self, listing_index: usize) -> Option<u64> {
        if !self.can_afford(listing_index) {
            return None;
        }
        let listing = &self.listings[listing_index];
        let id = listing.item_id;
        let price = listing.price;
        self.player_gold -= price;
        Some(id)
    }

    /// Compute sell price (typically a fraction of buy price).
    pub fn sell_price(base_value: u64) -> u64 {
        base_value / 4
    }

    /// Execute a sell. Adds gold to the player.
    pub fn try_sell(&mut self, base_value: u64) -> u64 {
        let price = Self::sell_price(base_value);
        self.player_gold += price;
        price
    }

    /// Repair cost based on durability deficit.
    pub fn repair_cost(max_durability: u32, current_durability: u32, item_level: u32) -> u64 {
        let deficit = max_durability.saturating_sub(current_durability);
        u64::from(deficit) * u64::from(item_level.max(1))
    }

    /// Identify cost (flat fee per item).
    pub fn identify_cost(item_level: u32) -> u64 {
        u64::from(item_level) * 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vendor_creation() {
        let vendor = VendorState::new(
            "Akara",
            vec![VendorAction::Buy, VendorAction::Sell, VendorAction::Identify],
            500,
        );
        assert_eq!(vendor.vendor_name, "Akara");
        assert_eq!(vendor.active_action, VendorAction::Buy);
    }

    #[test]
    fn buy_deducts_gold() {
        let mut vendor = VendorState::new("Smith", vec![VendorAction::Buy], 100);
        vendor.listings.push(VendorListing {
            item_id: 1,
            name: "Sword".into(),
            icon: "sword".into(),
            price: 50,
            available: true,
        });
        assert!(vendor.can_afford(0));
        let id = vendor.try_buy(0);
        assert_eq!(id, Some(1));
        assert_eq!(vendor.player_gold, 50);
    }

    #[test]
    fn buy_fails_if_too_expensive() {
        let mut vendor = VendorState::new("Smith", vec![VendorAction::Buy], 10);
        vendor.listings.push(VendorListing {
            item_id: 1,
            name: "Shield".into(),
            icon: "shield".into(),
            price: 50,
            available: true,
        });
        assert!(!vendor.can_afford(0));
        assert!(vendor.try_buy(0).is_none());
    }

    #[test]
    fn sell_adds_gold() {
        let mut vendor = VendorState::new("Smith", vec![VendorAction::Sell], 100);
        let earned = vendor.try_sell(200);
        assert_eq!(earned, 50);
        assert_eq!(vendor.player_gold, 150);
    }

    #[test]
    fn repair_and_identify_costs() {
        assert_eq!(VendorState::repair_cost(100, 60, 10), 400);
        assert_eq!(VendorState::identify_cost(20), 100);
    }
}
