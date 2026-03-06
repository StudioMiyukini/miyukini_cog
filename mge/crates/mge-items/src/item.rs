use serde::{Deserialize, Serialize};

use crate::rarity::Rarity;

/// Equipment slot on a character body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipSlot {
    Head,
    Chest,
    Belt,
    Gloves,
    Boots,
    Ring1,
    Ring2,
    Amulet,
    MainHand,
    OffHand,
}

/// Broad item family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemFamily {
    Weapon,
    Armor,
    Jewelry,
    Socketable,
    Charm,
    Misc,
}

/// Specific item kind (weapon subtype, armour type, etc.).
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemKind {
    // Weapons
    Sword,
    Axe,
    Mace,
    Dagger,
    Bow,
    Crossbow,
    Staff,
    Wand,
    Scepter,
    // Armour
    Helm,
    BodyArmor,
    Belt,
    Gloves,
    Boots,
    Shield,
    // Jewelry
    Ring,
    Amulet,
    // Socketables
    Gem,
    Rune,
    Jewel,
    // Charms
    SmallCharm,
    LargeCharm,
    GrandCharm,
    // Misc
    Gold,
    Key,
    ScrollOfIdentify,
    TomeOfIdentify,
    TownPortalScroll,
}

impl ItemKind {
    pub fn family(self) -> ItemFamily {
        match self {
            Self::Sword
            | Self::Axe
            | Self::Mace
            | Self::Dagger
            | Self::Bow
            | Self::Crossbow
            | Self::Staff
            | Self::Wand
            | Self::Scepter => ItemFamily::Weapon,

            Self::Helm
            | Self::BodyArmor
            | Self::Belt
            | Self::Gloves
            | Self::Boots
            | Self::Shield => ItemFamily::Armor,

            Self::Ring | Self::Amulet => ItemFamily::Jewelry,

            Self::Gem | Self::Rune | Self::Jewel => ItemFamily::Socketable,

            Self::SmallCharm | Self::LargeCharm | Self::GrandCharm => ItemFamily::Charm,

            Self::Gold
            | Self::Key
            | Self::ScrollOfIdentify
            | Self::TomeOfIdentify
            | Self::TownPortalScroll => ItemFamily::Misc,
        }
    }

    /// Slot this item occupies when equipped, if equippable.
    pub fn equip_slot(self) -> Option<EquipSlot> {
        match self {
            Self::Helm => Some(EquipSlot::Head),
            Self::BodyArmor => Some(EquipSlot::Chest),
            Self::Belt => Some(EquipSlot::Belt),
            Self::Gloves => Some(EquipSlot::Gloves),
            Self::Boots => Some(EquipSlot::Boots),
            Self::Shield => Some(EquipSlot::OffHand),
            Self::Ring => Some(EquipSlot::Ring1),
            Self::Amulet => Some(EquipSlot::Amulet),
            Self::Sword
            | Self::Axe
            | Self::Mace
            | Self::Dagger
            | Self::Bow
            | Self::Crossbow
            | Self::Staff
            | Self::Wand
            | Self::Scepter => Some(EquipSlot::MainHand),
            _ => None,
        }
    }
}

/// Minimum attribute requirements to equip an item.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct EquipRequirements {
    pub min_strength: u32,
    pub min_dexterity: u32,
    pub min_level: u32,
}

/// Base item definition (shared template; all instances of the same base share this).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseItemDef {
    pub id: u32,
    pub name: String,
    pub kind: ItemKind,
    pub max_sockets: u8,
    pub requirements: EquipRequirements,
    /// Template item level (drives affix pool).
    pub ilvl: u32,
    pub two_handed: bool,
}

/// A single item instance in the world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// Unique instance ID.
    pub uid: u64,
    pub base: BaseItemDef,
    pub rarity: Rarity,
    /// Item level at time of drop (may differ from base ilvl in templates).
    pub ilvl: u32,
    pub identified: bool,
    pub quantity: u32,
}

impl Item {
    pub fn new(uid: u64, base: BaseItemDef, rarity: Rarity, ilvl: u32) -> Self {
        let identified = rarity == Rarity::Normal;
        Self { uid, base, rarity, ilvl, identified, quantity: 1 }
    }

    pub fn is_stackable(&self) -> bool {
        matches!(
            self.base.kind,
            ItemKind::Gold
                | ItemKind::Key
                | ItemKind::ScrollOfIdentify
                | ItemKind::TomeOfIdentify
                | ItemKind::TownPortalScroll
        )
    }

    pub fn can_be_socketed(&self) -> bool {
        matches!(self.base.kind.family(), ItemFamily::Weapon | ItemFamily::Armor)
            && self.base.max_sockets > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sword_base() -> BaseItemDef {
        BaseItemDef {
            id: 1,
            name: "Short Sword".into(),
            kind: ItemKind::Sword,
            max_sockets: 2,
            requirements: EquipRequirements { min_strength: 10, min_dexterity: 0, min_level: 1 },
            ilvl: 1,
            two_handed: false,
        }
    }

    #[test]
    fn sword_family_is_weapon() {
        assert_eq!(ItemKind::Sword.family(), ItemFamily::Weapon);
    }

    #[test]
    fn sword_equip_slot_is_main_hand() {
        assert_eq!(ItemKind::Sword.equip_slot(), Some(EquipSlot::MainHand));
    }

    #[test]
    fn normal_item_identified_on_creation() {
        let item = Item::new(1, sword_base(), Rarity::Normal, 1);
        assert!(item.identified);
    }

    #[test]
    fn magic_item_not_identified_on_creation() {
        let item = Item::new(2, sword_base(), Rarity::Magic, 5);
        assert!(!item.identified);
    }

    #[test]
    fn gold_is_stackable() {
        let base = BaseItemDef {
            id: 99,
            name: "Gold".into(),
            kind: ItemKind::Gold,
            max_sockets: 0,
            requirements: EquipRequirements::default(),
            ilvl: 1,
            two_handed: false,
        };
        let item = Item::new(3, base, Rarity::Normal, 1);
        assert!(item.is_stackable());
    }

    #[test]
    fn sword_can_be_socketed() {
        let item = Item::new(1, sword_base(), Rarity::Normal, 1);
        assert!(item.can_be_socketed());
    }
}
