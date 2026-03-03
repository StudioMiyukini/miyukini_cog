// @id: MGE-ARPG-ItemDrop @do: item-drop @role: back-end @layer: 3 @human: miyuk
//! Item drop ECS component and bundle (items on the ground).

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

// ---------------------------------------------------------------------------
// ItemDrop — lightweight ECS component for the game world
// ---------------------------------------------------------------------------

/// Lightweight ECS component representing an item lying on the ground.
///
/// Designed to be stored directly in the ECS via `spawn_with_1`.
/// `quality_color` is a linear RGBA value used by the renderer to tint the
/// drop label (matching D2 rarity colours: white, blue, yellow, gold, …).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemDrop {
    /// Identifier of the item type that was dropped (e.g. `"short_sword"`).
    pub item_id: String,
    /// Linear RGBA tint colour for the drop label `[r, g, b, a]`.
    pub quality_color: [f32; 4],
    /// World X coordinate (in world units).
    pub position_x: f32,
    /// World Y coordinate (in world units).
    pub position_y: f32,
    /// Timestamp (in milliseconds) at which this drop was created.
    pub spawn_time_ms: u64,
}

impl ItemDrop {
    /// Creates a new `ItemDrop` component.
    ///
    /// # Arguments
    /// * `item_id`       — identifier of the dropped item type
    /// * `quality_color` — linear RGBA label colour `[r, g, b, a]`
    /// * `x`             — world X position
    /// * `y`             — world Y position
    /// * `time`          — spawn timestamp in milliseconds
    pub fn new(
        item_id: impl Into<String>,
        quality_color: [f32; 4],
        x: f32,
        y: f32,
        time: u64,
    ) -> Self {
        Self {
            item_id: item_id.into(),
            quality_color,
            position_x: x,
            position_y: y,
            spawn_time_ms: time,
        }
    }
}
