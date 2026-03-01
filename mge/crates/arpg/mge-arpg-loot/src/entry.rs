//! Drop entries and Treasure Class definitions (D2-style loot tables).
//!
//! A [`TreasureClass`] contains weighted [`DropEntry`] items plus a `no_drop`
//! weight. The loot generator picks from this table according to the weights.

/// A single possible outcome inside a [`TreasureClass`].
///
/// When `is_treasure_class` is `true`, `item_id` references another
/// `TreasureClass` that must be resolved recursively.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DropEntry {
    /// Identifier of the item or nested treasure class.
    pub item_id: String,
    /// Relative weight used during the weighted random pick.
    pub weight: u32,
    /// Minimum quantity dropped (relevant for stackable items such as gold).
    pub min_qty: u32,
    /// Maximum quantity dropped (inclusive).
    pub max_qty: u32,
    /// When `true`, `item_id` names another [`TreasureClass`] instead of an
    /// actual item.
    pub is_treasure_class: bool,
}

/// A D2-style Treasure Class: a weighted drop table with a NoDrop chance.
///
/// `picks` controls how many independent rolls are made on this table.
/// Each roll can yield an item, a nested TC resolution, or nothing (NoDrop).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TreasureClass {
    /// Unique identifier for this treasure class (e.g. `"tc_weapons_normal"`).
    pub id: String,
    /// Number of independent picks to make from this table.
    pub picks: u32,
    /// Weight of the "nothing drops" outcome.
    pub no_drop: u32,
    /// The weighted entries available for each pick.
    pub entries: Vec<DropEntry>,
}

impl TreasureClass {
    /// Returns the sum of all entry weights plus `no_drop`.
    ///
    /// This is the total denominator used for weighted random selection.
    pub fn total_weight(&self) -> u32 {
        let entry_sum: u32 = self.entries.iter().map(|e| e.weight).sum();
        entry_sum.saturating_add(self.no_drop)
    }
}
