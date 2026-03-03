// @id: Sodomight-Content-Items @do: act1-item-definitions @role: back-end @layer: 4 @human: miyuk
//! Act 1 item definitions: base item types and templates.
//!
//! This module contains all item-related gameplay data for the Sodomight MVP.
#![allow(clippy::too_many_lines)]

// ---------------------------------------------------------------------------
// Item definitions
// ---------------------------------------------------------------------------

/// Category of a base item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ItemType {
    /// Melee or ranged weapon.
    Weapon,
    /// Head slot armor.
    Helm,
    /// Body armor.
    Armor,
    /// Hand slot armor.
    Gloves,
    /// Waist slot.
    Belt,
    /// Foot slot armor.
    Boots,
    /// Neck accessory.
    Amulet,
    /// Finger accessory.
    Ring,
    /// Off-hand shield.
    Shield,
    /// Consumable potion.
    Potion,
    /// Stackable currency (gold).
    Currency,
}

/// Static definition for a base item.
///
/// Represents the unmodified template before quality tiers (magic, rare, etc.)
/// and affixes are applied.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseItemDef {
    /// Unique item identifier (e.g. `"short_sword"`).
    pub id: String,
    /// Display name.
    pub name: String,
    /// Category of this item.
    pub item_type: ItemType,
    /// Item level -- controls affix pool availability.
    pub ilvl: u8,
    /// Minimum base damage (weapons only, 0 for non-weapons).
    pub base_damage_min: i32,
    /// Maximum base damage (weapons only, 0 for non-weapons).
    pub base_damage_max: i32,
    /// Base defense value (armor pieces only, 0 for weapons).
    pub base_defense: i32,
    /// For potions: amount of HP restored. 0 if not a potion.
    pub restore_hp: i32,
    /// For potions: amount of mana restored. 0 if not a potion.
    pub restore_mana: i32,
    /// Whether this item can stack in inventory.
    pub stackable: bool,
}

/// Returns all Act 1 base item definitions.
///
/// Includes basic weapons, armor pieces, consumables, and gold.
#[must_use]
pub fn act1_items() -> Vec<BaseItemDef> {
    vec![
        BaseItemDef {
            id: "short_sword".into(),
            name: "Short Sword".into(),
            item_type: ItemType::Weapon,
            ilvl: 1,
            base_damage_min: 2,
            base_damage_max: 6,
            base_defense: 0,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "hand_axe".into(),
            name: "Hand Axe".into(),
            item_type: ItemType::Weapon,
            ilvl: 2,
            base_damage_min: 3,
            base_damage_max: 8,
            base_defense: 0,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "club".into(),
            name: "Club".into(),
            item_type: ItemType::Weapon,
            ilvl: 1,
            base_damage_min: 1,
            base_damage_max: 6,
            base_defense: 0,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "buckler".into(),
            name: "Buckler".into(),
            item_type: ItemType::Shield,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 4,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "quilted_armor".into(),
            name: "Quilted Armor".into(),
            item_type: ItemType::Armor,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 8,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "leather_armor".into(),
            name: "Leather Armor".into(),
            item_type: ItemType::Armor,
            ilvl: 3,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 14,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "cap".into(),
            name: "Cap".into(),
            item_type: ItemType::Helm,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 3,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "sash".into(),
            name: "Sash".into(),
            item_type: ItemType::Belt,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 2,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "leather_gloves".into(),
            name: "Leather Gloves".into(),
            item_type: ItemType::Gloves,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 2,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "leather_boots".into(),
            name: "Leather Boots".into(),
            item_type: ItemType::Boots,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 2,
            restore_hp: 0,
            restore_mana: 0,
            stackable: false,
        },
        BaseItemDef {
            id: "minor_health_potion".into(),
            name: "Minor Health Potion".into(),
            item_type: ItemType::Potion,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 0,
            restore_hp: 50,
            restore_mana: 0,
            stackable: true,
        },
        BaseItemDef {
            id: "minor_mana_potion".into(),
            name: "Minor Mana Potion".into(),
            item_type: ItemType::Potion,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 0,
            restore_hp: 0,
            restore_mana: 30,
            stackable: true,
        },
        BaseItemDef {
            id: "gold".into(),
            name: "Gold".into(),
            item_type: ItemType::Currency,
            ilvl: 1,
            base_damage_min: 0,
            base_damage_max: 0,
            base_defense: 0,
            restore_hp: 0,
            restore_mana: 0,
            stackable: true,
        },
    ]
}

// ---------------------------------------------------------------------------
// Lookup helpers
// ---------------------------------------------------------------------------

/// Find a base item definition by its id.
#[must_use]
pub fn find_item(id: &str) -> Option<BaseItemDef> {
    act1_items().into_iter().find(|i| i.id == id)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_act1_items_count() {
        let items = act1_items();
        assert_eq!(items.len(), 13);
    }

    #[test]
    fn test_short_sword_stats() {
        let sword = find_item("short_sword").expect("short_sword must exist");
        assert_eq!(sword.item_type, ItemType::Weapon);
        assert_eq!(sword.ilvl, 1);
        assert_eq!(sword.base_damage_min, 2);
        assert_eq!(sword.base_damage_max, 6);
    }

    #[test]
    fn test_minor_health_potion_restores() {
        let potion = find_item("minor_health_potion").expect("potion must exist");
        assert_eq!(potion.item_type, ItemType::Potion);
        assert_eq!(potion.restore_hp, 50);
        assert!(potion.stackable);
    }

    #[test]
    fn test_gold_is_currency() {
        let gold = find_item("gold").expect("gold must exist");
        assert_eq!(gold.item_type, ItemType::Currency);
        assert!(gold.stackable);
    }
}
