// @id: MGE-ARPG-Entity-ItemDrop @do: item-drop-bundle @role: back-end @layer: 3 @human: miyuk
//! Item drop entity bundle (items on the ground).

use crate::components::Position;

/// Identifier of the item type that was dropped.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ItemId(pub String);

impl ItemId {
    /// Creates a new item ID from any string-like value.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the ID as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An item lying on the ground. Picked up when a player walks nearby.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemDropBundle {
    /// Unique instance ID of this drop (for netcode).
    pub instance_id: String,
    /// Which item type this represents.
    pub item_id: ItemId,
    /// World position of the drop.
    pub position: Position,
    /// Owner restriction: only this player can pick up for `pickup_lock_ticks`.
    pub owner_id: Option<String>,
    /// Ticks until the item becomes public (0 = public immediately).
    pub pickup_lock_ticks: u32,
}

impl ItemDropBundle {
    /// Creates a new public item drop with no owner lock.
    pub fn new(
        instance_id: impl Into<String>,
        item_id: impl Into<String>,
        position: Position,
    ) -> Self {
        Self {
            instance_id: instance_id.into(),
            item_id: ItemId::new(item_id),
            position,
            owner_id: None,
            pickup_lock_ticks: 0,
        }
    }

    /// Sets an owner lock: only this player can pick up for `lock_ticks` ticks.
    #[must_use]
    pub fn with_owner(mut self, owner: impl Into<String>, lock_ticks: u32) -> Self {
        self.owner_id = Some(owner.into());
        self.pickup_lock_ticks = lock_ticks;
        self
    }

    /// Returns `true` if the item can be picked up by anyone.
    pub fn is_public(&self) -> bool {
        self.owner_id.is_none() || self.pickup_lock_ticks == 0
    }
}
