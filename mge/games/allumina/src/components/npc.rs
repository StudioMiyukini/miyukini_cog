//! Non-combat NPCs.

use crate::components::item::ItemInstance;
use crate::components::world::NpcRole;

/// Kind of NPC. `Merchant` is the legacy generic merchant kept for
/// back-compat; every village NPC now also carries a role-specific kind so the
/// renderer can paint a distinctive silhouette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcKind {
    Merchant,
    WeaponMerchant,
    ArmorMerchant,
    PotionMerchant,
    Blacksmith,
    Healer,
    QuestGiver,
    Recruiter,
}

impl NpcKind {
    #[must_use]
    pub fn from_role(role: NpcRole) -> Self {
        match role {
            NpcRole::WeaponMerchant => Self::WeaponMerchant,
            NpcRole::ArmorMerchant => Self::ArmorMerchant,
            NpcRole::PotionMerchant => Self::PotionMerchant,
            NpcRole::Blacksmith => Self::Blacksmith,
            NpcRole::Healer => Self::Healer,
            NpcRole::QuestGiver => Self::QuestGiver,
            NpcRole::Recruiter => Self::Recruiter,
        }
    }

    /// True for NPC roles that operate a buy/sell stall (the merchant UI keys
    /// off this, not the legacy `Merchant` value).
    #[must_use]
    pub fn is_merchant(self) -> bool {
        matches!(
            self,
            Self::Merchant | Self::WeaponMerchant | Self::ArmorMerchant | Self::PotionMerchant
        )
    }
}

/// A merchant's buyable stock.
#[derive(Debug, Clone, Default)]
pub struct Merchant {
    pub stock: Vec<ItemInstance>,
}

/// Authoring metadata for an NPC: their display name + a short interaction
/// line. Rendered above the NPC and surfaced when the player walks near.
#[derive(Debug, Clone)]
pub struct NpcInfo {
    pub name: String,
    pub dialogue: String,
}
