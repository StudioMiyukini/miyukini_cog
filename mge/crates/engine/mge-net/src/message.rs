// @id: MGE-Net-Messages @do: message-types @role: back-end @layer: 2 @human: miyuk
//! Separate `ClientMessage` and `ServerMessage` enums.
//!
//! Having distinct types for each direction lets the compiler enforce
//! that server-only variants are never constructed in client code, and
//! enables the security helper [`is_invalid_client_message`] to detect
//! direction violations at runtime.

use serde::{Deserialize, Serialize};

// ── Client → Server ──────────────────────────────────────────────────────────

/// Messages a client is allowed to send to the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    /// Player requests a movement delta.
    Move {
        /// Horizontal displacement.
        dx: f32,
        /// Vertical displacement.
        dy: f32,
    },
    /// Player uses a skill at a world position.
    UseSkill {
        /// Identifier of the skill being used.
        skill_id: String,
        /// World X coordinate of the skill target.
        target_x: f32,
        /// World Y coordinate of the skill target.
        target_y: f32,
    },
    /// Player attempts to pick up an item.
    PickupItem {
        /// Identifier of the item to pick up.
        item_id: String,
    },
    /// Player sends a chat message.
    Chat {
        /// Chat text content.
        text: String,
    },
    /// Player initiates a trade with another player.
    TradeRequest {
        /// Username of the trade target.
        target_player: String,
    },
    /// Player accepts a pending trade offer.
    TradeAccept,
    /// Player cancels a pending trade offer.
    TradeCancel,
    /// Client ping (latency measurement).
    Ping {
        /// Client-side timestamp in milliseconds.
        timestamp_ms: u64,
    },
}

// ── Server → Client ──────────────────────────────────────────────────────────

/// Messages the server sends to clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    /// Bulk entity state snapshot.
    SyncState {
        /// Serialized entity data (opaque byte blob).
        entities: Vec<u8>,
    },
    /// A new entity has appeared in the world.
    SpawnEntity {
        /// Numeric entity identifier.
        entity_id: u64,
        /// Entity archetype / kind tag.
        entity_type: String,
        /// Spawn X position.
        x: f32,
        /// Spawn Y position.
        y: f32,
    },
    /// An entity has been removed from the world.
    DespawnEntity {
        /// Numeric entity identifier.
        entity_id: u64,
    },
    /// A damage event occurred between two entities.
    DamageEvent {
        /// Entity that dealt the damage.
        source: u64,
        /// Entity that received the damage.
        target: u64,
        /// Amount of damage applied.
        amount: i32,
        /// Damage type tag (e.g. `"physical"`, `"fire"`).
        damage_type: String,
    },
    /// An item has dropped in the world.
    LootDrop {
        /// Identifier of the dropped item.
        item_id: String,
        /// World X position of the drop.
        x: f32,
        /// World Y position of the drop.
        y: f32,
        /// Quality tier of the item (e.g. `"common"`, `"rare"`).
        quality: String,
    },
    /// A chat message broadcast to all clients.
    ChatBroadcast {
        /// Username of the sender.
        sender: String,
        /// Chat text content.
        text: String,
    },
    /// Server pong response to a client ping.
    Pong {
        /// Echo of the client timestamp.
        timestamp_ms: u64,
    },
    /// Server-initiated disconnect.
    Disconnect {
        /// Human-readable reason for disconnection.
        reason: String,
    },
}

// ── Parse helpers ─────────────────────────────────────────────────────────────

/// Attempt to deserialize bytes as a [`ClientMessage`].
///
/// Returns `None` if `data` is not valid JSON or does not match any
/// `ClientMessage` variant.
#[must_use]
pub fn parse_client_message(data: &[u8]) -> Option<ClientMessage> {
    serde_json::from_slice(data).ok()
}

/// Attempt to deserialize bytes as a [`ServerMessage`].
///
/// Returns `None` if `data` is not valid JSON or does not match any
/// `ServerMessage` variant.
#[must_use]
pub fn parse_server_message(data: &[u8]) -> Option<ServerMessage> {
    serde_json::from_slice(data).ok()
}

/// Returns `true` when `data` looks like a [`ServerMessage`] but not a
/// [`ClientMessage`] — i.e. a client tried to send a server-direction message.
///
/// Use this as a lightweight security guard on the server receive path.
#[must_use]
pub fn is_invalid_client_message(data: &[u8]) -> bool {
    serde_json::from_slice::<ServerMessage>(data).is_ok()
        && serde_json::from_slice::<ClientMessage>(data).is_err()
}
