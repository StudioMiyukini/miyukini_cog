// @id: MGE-ARPG-Items-Instance @do: item-instance @role: back-end @layer: 3 @human: miyuk
//! Concrete item instance living in the game world.
//!
//! An `ItemInstance` is a fully realized item: it has been generated from a base
//! definition, rolled its affixes, and tracks mutable state like durability.

use crate::quality::{Affix, AffixKind, ItemQuality};

/// A concrete item that exists in the game world or a player's inventory.
///
/// Created from a base item definition with rolled affixes, durability, and
/// identification state. Unidentified items hide their affixes until identified.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemInstance {
    /// Unique identifier for this specific item instance.
    pub instance_id: uuid::Uuid,
    /// Reference to the base item definition (e.g. `"long_sword"`, `"chain_mail"`).
    pub base_id: String,
    /// Quality tier determining affix behavior.
    pub quality: ItemQuality,
    /// Affixes (prefixes and suffixes) rolled on this item.
    pub affixes: Vec<Affix>,
    /// Current durability points remaining.
    pub durability_current: u32,
    /// Maximum durability points for this item.
    pub durability_max: u32,
    /// Whether the item has been identified (affixes revealed).
    pub is_identified: bool,
    /// Current stack size (1 for non-stackable items).
    pub stack_size: u32,
    /// Item level, typically the zone level where it dropped.
    pub ilvl: u32,
}

impl ItemInstance {
    /// Create a new Normal-quality item with no affixes.
    ///
    /// The item starts unidentified with full durability (100/100) and stack size 1.
    #[must_use]
    pub fn new_normal(base_id: String, ilvl: u32) -> Self {
        Self {
            instance_id: uuid::Uuid::new_v4(),
            base_id,
            quality: ItemQuality::Normal,
            affixes: Vec::new(),
            durability_current: 100,
            durability_max: 100,
            is_identified: false,
            stack_size: 1,
            ilvl,
        }
    }

    /// Mark this item as identified, revealing its affixes.
    pub fn identify(&mut self) {
        self.is_identified = true;
    }

    /// Returns `true` if the item's durability has reached zero.
    #[must_use]
    pub fn is_broken(&self) -> bool {
        self.durability_current == 0
    }

    /// Reduce durability by `amount`, clamping to zero.
    pub fn apply_damage(&mut self, amount: u32) {
        self.durability_current = self.durability_current.saturating_sub(amount);
    }

    /// Total number of affixes on this item.
    #[must_use]
    pub fn affix_count(&self) -> usize {
        self.affixes.len()
    }

    /// Iterator over prefix affixes only.
    pub fn prefixes(&self) -> impl Iterator<Item = &Affix> {
        self.affixes.iter().filter(|a| a.kind == AffixKind::Prefix)
    }

    /// Iterator over suffix affixes only.
    pub fn suffixes(&self) -> impl Iterator<Item = &Affix> {
        self.affixes.iter().filter(|a| a.kind == AffixKind::Suffix)
    }
}
