// @id: MGE-ARPG-Identify @do: item-identification @role: back-end @layer: 3 @human: miyuk
//! Item identification system.
//!
//! Implements the Diablo 2-style identification mechanic: Magic and higher-quality
//! items drop unidentified and require a Scroll of Identify (or Cain) to reveal
//! their hidden affixes.
//!
//! ## Flow
//!
//! 1. Item drops as `identified = false` (unless `Normal` quality).
//! 2. Player uses a scroll or NPC to call `identify()`.
//! 3. `visible_affixes()` returns the full affix list after identification.

use crate::ItemQuality;

/// An item that may need identification before its affixes are visible.
///
/// Normal items are always pre-identified at creation.
/// All other quality tiers drop unidentified.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IdentifiableItem {
    /// Base item type identifier (e.g. `"long_sword"`, `"cap"`).
    pub base_id: String,
    /// Quality tier of this item.
    pub quality: ItemQuality,
    /// Whether this item's affixes have been revealed.
    pub identified: bool,
    /// Hidden affixes, only visible after identification.
    pub affixes: Vec<String>,
}

/// Errors that can occur during item identification.
#[derive(Debug, thiserror::Error)]
pub enum IdentifyError {
    /// Returned when `identify()` is called on an already-identified item.
    #[error("Item is already identified")]
    AlreadyIdentified,
    /// Returned when `identify()` is called on a Normal-quality item.
    #[error("Normal items do not need identification")]
    NormalItem,
}

impl IdentifiableItem {
    /// Create a new `IdentifiableItem`.
    ///
    /// Normal-quality items are automatically set to `identified = true`.
    /// All other quality tiers start as `identified = false`.
    ///
    /// # Arguments
    ///
    /// * `base_id`  - Base item type identifier.
    /// * `quality`  - Quality tier of the item.
    /// * `affixes`  - Affix strings attached to this item.
    #[must_use]
    pub fn new(base_id: impl Into<String>, quality: ItemQuality, affixes: Vec<String>) -> Self {
        let identified = quality == ItemQuality::Normal;
        Self {
            base_id: base_id.into(),
            quality,
            identified,
            affixes,
        }
    }

    /// Identify this item, revealing its hidden affixes.
    ///
    /// # Errors
    ///
    /// - [`IdentifyError::AlreadyIdentified`] — item was already identified.
    /// - [`IdentifyError::NormalItem`] — Normal items cannot be identified.
    pub fn identify(&mut self) -> Result<(), IdentifyError> {
        if self.quality == ItemQuality::Normal {
            return Err(IdentifyError::NormalItem);
        }
        if self.identified {
            return Err(IdentifyError::AlreadyIdentified);
        }
        self.identified = true;
        Ok(())
    }

    /// Return the visible affixes for this item.
    ///
    /// Returns all affixes if identified, or an empty slice if not.
    #[must_use]
    pub fn visible_affixes(&self) -> &[String] {
        if self.identified {
            &self.affixes
        } else {
            &[]
        }
    }

    /// Return the display name for this item.
    ///
    /// Returns `"Unidentified {base_id}"` when unidentified, or just
    /// `base_id` when identified.
    #[must_use]
    pub fn display_name(&self) -> String {
        if self.identified {
            self.base_id.clone()
        } else {
            format!("Unidentified {}", self.base_id)
        }
    }
}

/// Method used to identify an item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentifyMethod {
    /// Uses a Scroll of Identify (consumed on use).
    Scroll,
    /// Uses NPC Cain (free, available after completing his quest).
    Cain,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn magic_item_with_affixes() -> IdentifiableItem {
        IdentifiableItem::new(
            "long_sword",
            ItemQuality::Magic,
            vec!["affix_fire_res".to_string(), "affix_str".to_string()],
        )
    }

    #[test]
    fn identify_reveals_affixes() {
        let mut item = magic_item_with_affixes();
        // Before identification: affixes are hidden.
        assert_eq!(
            item.visible_affixes().len(),
            0,
            "Unidentified item must expose no affixes"
        );
        // After identification: both affixes are visible.
        item.identify().expect("identify should succeed");
        assert_eq!(
            item.visible_affixes().len(),
            2,
            "Identified item must expose all affixes"
        );
    }

    #[test]
    fn normal_always_identified() {
        let item = IdentifiableItem::new("cap", ItemQuality::Normal, vec![]);
        assert!(
            item.identified,
            "Normal items must be identified from creation"
        );
    }

    #[test]
    fn already_identified_error() {
        let mut item = magic_item_with_affixes();
        item.identify().expect("first identify should succeed");
        let err = item.identify().expect_err("second identify should fail");
        assert!(
            matches!(err, IdentifyError::AlreadyIdentified),
            "Expected AlreadyIdentified, got: {err}"
        );
    }

    #[test]
    fn display_name_unidentified() {
        let item = IdentifiableItem::new("sword", ItemQuality::Magic, vec![]);
        assert_eq!(
            item.display_name(),
            "Unidentified sword",
            "Unidentified display name must be prefixed"
        );
    }

    #[test]
    fn display_name_identified() {
        let mut item = IdentifiableItem::new("sword", ItemQuality::Magic, vec![]);
        item.identify().expect("identify should succeed");
        assert_eq!(
            item.display_name(),
            "sword",
            "Identified display name must equal base_id"
        );
    }

    #[test]
    fn normal_item_identify_error() {
        let mut item = IdentifiableItem::new("cap", ItemQuality::Normal, vec![]);
        let err = item
            .identify()
            .expect_err("Normal item identify must return error");
        assert!(
            matches!(err, IdentifyError::NormalItem),
            "Expected NormalItem error, got: {err}"
        );
    }

    #[test]
    fn visible_affixes_identified_returns_all() {
        let mut item = IdentifiableItem::new(
            "ring",
            ItemQuality::Rare,
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
        );
        item.identify().expect("identify should succeed");
        assert_eq!(item.visible_affixes().len(), 3);
    }
}
