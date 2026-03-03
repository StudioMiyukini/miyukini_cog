// @id: MGE-ARPG-Sockets @do: socket-system @role: back-end @layer: 3 @human: miyuk
//! Socket system: gem, rune, and jewel insertion into socketed items.
//!
//! Modeled after the Diablo 2 socket system:
//! - Items have 1–6 sockets depending on their base type.
//! - Each socket can hold a [`GemType`] at a given [`GemQuality`], a Rune
//!   identified by its numeric id (1–33), or a magic Jewel.
//! - Sockets are slotted individually via [`SocketedItem::insert`].
//! - All fillers can be removed at once (Hel-rune cube recipe) via
//!   [`SocketedItem::remove_all`].

// ---------------------------------------------------------------------------
// Gem types and quality
// ---------------------------------------------------------------------------

/// The gem variety placed in a socket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GemType {
    /// Fire-themed gem (enhanced fire damage / fire resist).
    Ruby,
    /// Cold-themed gem (enhanced cold damage / cold resist).
    Sapphire,
    /// Lightning-themed gem (enhanced lightning damage / lightning resist).
    Topaz,
    /// Poison-themed gem (enhanced poison damage / poison resist).
    Emerald,
    /// Neutral gem (enhanced damage / all resists).
    Diamond,
    /// Magic-find / experience gem.
    Amethyst,
    /// Undead-themed gem used in runeword recipes.
    Skull,
}

/// Quality grade of a gem affecting the magnitude of its bonuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GemQuality {
    /// Lowest quality — minimal stat bonus.
    Chipped,
    /// Below-average quality.
    Flawed,
    /// Standard quality.
    Normal,
    /// Above-average quality.
    Flawless,
    /// Maximum quality — highest stat bonus.
    Perfect,
}

// ---------------------------------------------------------------------------
// Socket filler
// ---------------------------------------------------------------------------

/// What occupies a single socket on an item.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SocketFiller {
    /// A precious gem of a given type and quality.
    Gem {
        /// The variety of gem.
        gem_type: GemType,
        /// The quality grade of the gem.
        quality: GemQuality,
    },
    /// A rune identified by its numeric id.
    ///
    /// Valid range: 1–33 (matching the 33 runes in the Diablo 2 tradition,
    /// El through Zod).
    Rune {
        /// Numeric rune identifier, 1-indexed.
        rune_id: u8,
    },
    /// A magic jewel with a single randomized stat.
    Jewel {
        /// Name of the stat modifier (e.g. `"fire_resist"`, `"attack_rating"`).
        stat: String,
        /// Rolled value for the stat (can be negative for cursed jewels).
        value: i32,
    },
}

// ---------------------------------------------------------------------------
// Socket error
// ---------------------------------------------------------------------------

/// Errors that can occur when interacting with the socket system.
#[derive(Debug, thiserror::Error)]
pub enum SocketError {
    /// The requested socket index exceeds the item's socket count.
    #[error("Socket index {index} out of range (max {max})")]
    IndexOutOfRange {
        /// The index that was requested.
        index: usize,
        /// The highest valid index (`max_sockets - 1`).
        max: usize,
    },

    /// Attempted to insert into a socket that is already occupied.
    #[error("Socket {index} is already filled")]
    AlreadyFilled {
        /// The socket that is occupied.
        index: usize,
    },

    /// Attempted to remove from a socket that is empty.
    #[error("Socket {index} is empty")]
    Empty {
        /// The socket that is vacant.
        index: usize,
    },
}

// ---------------------------------------------------------------------------
// SocketedItem
// ---------------------------------------------------------------------------

/// An item that has physical sockets which can hold gems, runes, or jewels.
///
/// The `sockets` vector always has exactly `max_sockets` entries; each entry
/// is `None` (empty) or `Some(SocketFiller)` (occupied).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SocketedItem {
    /// Reference to the base item definition (e.g. `"sword"`, `"armor"`).
    pub base_id: String,
    /// Maximum number of sockets this item can have (1–6).
    pub max_sockets: u8,
    /// The socket slots; length always equals `max_sockets`.
    pub sockets: Vec<Option<SocketFiller>>,
}

impl SocketedItem {
    /// Create a new socketed item with all sockets empty.
    ///
    /// `max_sockets` is clamped to the valid range 1–6 to prevent
    /// malformed states.
    #[must_use]
    pub fn new(base_id: impl Into<String>, max_sockets: u8) -> Self {
        let clamped = max_sockets.clamp(1, 6);
        let sockets = vec![None; usize::from(clamped)];
        Self {
            base_id: base_id.into(),
            max_sockets: clamped,
            sockets,
        }
    }

    /// Insert a [`SocketFiller`] into the socket at `index`.
    ///
    /// # Errors
    ///
    /// - [`SocketError::IndexOutOfRange`] if `index >= max_sockets`.
    /// - [`SocketError::AlreadyFilled`] if the socket is already occupied.
    pub fn insert(&mut self, index: usize, filler: SocketFiller) -> Result<(), SocketError> {
        let max = usize::from(self.max_sockets);
        if index >= max {
            return Err(SocketError::IndexOutOfRange { index, max: max - 1 });
        }
        if self.sockets[index].is_some() {
            return Err(SocketError::AlreadyFilled { index });
        }
        self.sockets[index] = Some(filler);
        Ok(())
    }

    /// Remove and return all fillers, leaving every socket empty.
    ///
    /// Mirrors the Hel-rune cube recipe that strips a socketed item of all
    /// its inserted runes/gems. Returns only the fillers that were present
    /// (empty sockets are skipped).
    pub fn remove_all(&mut self) -> Vec<SocketFiller> {
        self.sockets
            .iter_mut()
            .filter_map(Option::take)
            .collect()
    }

    /// Number of sockets that currently contain a filler.
    #[must_use]
    pub fn filled_count(&self) -> usize {
        self.sockets.iter().filter(|s| s.is_some()).count()
    }

    /// Returns `true` when every socket contains a filler.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.filled_count() == usize::from(self.max_sockets)
    }
}

// ---------------------------------------------------------------------------
// max_sockets_for_base
// ---------------------------------------------------------------------------

/// Return the default maximum socket count for a given base item identifier.
///
/// Used when creating a newly dropped item to set [`SocketedItem::max_sockets`].
/// Unknown base types return `1` as a safe default.
#[must_use]
pub fn max_sockets_for_base(base_id: &str) -> u8 {
    match base_id {
        "hand_axe" | "dagger" | "dirk" | "kris" => 2,
        "sword" | "scimitar" | "saber" | "shield" | "buckler" => 3,
        "war_axe" | "military_pick" | "armor" | "plate" => 4,
        _ => 1,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{GemQuality, GemType, SocketError, SocketFiller, SocketedItem};

    fn gem() -> SocketFiller {
        SocketFiller::Gem {
            gem_type: GemType::Ruby,
            quality: GemQuality::Normal,
        }
    }

    fn rune(id: u8) -> SocketFiller {
        SocketFiller::Rune { rune_id: id }
    }

    fn jewel(stat: &str, value: i32) -> SocketFiller {
        SocketFiller::Jewel {
            stat: stat.to_owned(),
            value,
        }
    }

    /// RED → GREEN: inserting a gem increments filled_count.
    #[test]
    fn socket_insert() {
        let mut item = SocketedItem::new("sword", 3);
        item.insert(0, gem()).unwrap();
        assert_eq!(item.filled_count(), 1);
    }

    /// RED → GREEN: filling all sockets then inserting returns AlreadyFilled.
    #[test]
    fn socket_full() {
        let mut item = SocketedItem::new("sword", 3);
        item.insert(0, gem()).unwrap();
        item.insert(1, rune(1)).unwrap();
        item.insert(2, jewel("fire_resist", 15)).unwrap();

        assert!(item.is_full());

        let err = item.insert(1, gem()).unwrap_err();
        assert!(
            matches!(err, SocketError::AlreadyFilled { index: 1 }),
            "expected AlreadyFilled(1), got {err:?}"
        );
    }

    /// RED → GREEN: index beyond max_sockets returns IndexOutOfRange.
    #[test]
    fn socket_out_of_range() {
        let mut item = SocketedItem::new("sword", 3);
        let err = item.insert(3, gem()).unwrap_err();
        assert!(
            matches!(err, SocketError::IndexOutOfRange { index: 3, max: 2 }),
            "expected IndexOutOfRange{{3, 2}}, got {err:?}"
        );
    }

    /// RED → GREEN: remove_all returns all fillers and leaves sockets empty.
    #[test]
    fn socket_remove_all() {
        let mut item = SocketedItem::new("armor", 4);
        item.insert(0, gem()).unwrap();
        item.insert(1, rune(5)).unwrap();
        item.insert(2, jewel("attack_rating", 20)).unwrap();

        let removed = item.remove_all();
        assert_eq!(removed.len(), 3, "expected 3 removed fillers");
        assert_eq!(item.filled_count(), 0, "sockets should be empty after remove_all");
    }

    /// RED → GREEN: is_full returns true only when all sockets are occupied.
    #[test]
    fn socket_is_full() {
        let mut item = SocketedItem::new("buckler", 3);
        assert!(!item.is_full());
        item.insert(0, gem()).unwrap();
        item.insert(1, gem()).unwrap();
        assert!(!item.is_full());
        item.insert(2, gem()).unwrap();
        assert!(item.is_full());
    }

    /// max_sockets_for_base returns expected values for known base types.
    #[test]
    fn max_sockets_lookup() {
        use super::max_sockets_for_base;
        assert_eq!(max_sockets_for_base("sword"), 3);
        assert_eq!(max_sockets_for_base("armor"), 4);
        assert_eq!(max_sockets_for_base("buckler"), 3);
        assert_eq!(max_sockets_for_base("hand_axe"), 2);
        assert_eq!(max_sockets_for_base("unknown_base"), 1);
    }
}
