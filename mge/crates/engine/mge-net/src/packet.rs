// @id: MGE-Net-Packet @do: protocol @role: back-end @layer: 2 @human: miyuk
//! Generic network packet and game message definitions.

use crate::client_id::ClientId;
use serde::{Deserialize, Serialize};

/// A network packet carrying an arbitrary serializable payload.
///
/// Every packet is stamped with a unique id and the [`ClientId`] of the sender.
#[derive(Debug, Serialize, Deserialize)]
pub struct Packet<T> {
    /// Unique identifier for this packet.
    pub id: uuid::Uuid,
    /// Identifier of the client that sent (or will receive) this packet.
    pub client_id: ClientId,
    /// The actual message payload.
    pub payload: T,
}

impl<T: Serialize + for<'de> Deserialize<'de>> Packet<T> {
    /// Create a new packet with a fresh UUID.
    #[must_use]
    pub fn new(client_id: ClientId, payload: T) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            client_id,
            payload,
        }
    }
}

/// All game messages exchanged between client and server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameMessage {
    // -- Client -> Server --
    /// Player requests a movement delta.
    PlayerMove {
        /// Horizontal displacement.
        dx: f32,
        /// Vertical displacement.
        dy: f32,
    },
    /// Player attacks a target entity.
    PlayerAttack {
        /// Entity id of the attack target.
        target_id: u32,
    },
    /// Player sends a chat message.
    PlayerChat {
        /// Chat text content.
        text: String,
    },
    /// Client ping (latency measurement).
    Ping {
        /// Client-side timestamp in milliseconds.
        timestamp_ms: u64,
    },

    // -- Server -> Client --
    /// Authoritative entity state update.
    EntityUpdate {
        /// Entity identifier.
        entity_id: u32,
        /// World X position.
        x: f32,
        /// World Y position.
        y: f32,
        /// Current hit points.
        hp: i32,
    },
    /// A new entity appears in the world.
    EntitySpawn {
        /// Entity identifier.
        entity_id: u32,
        /// Entity archetype / kind tag.
        kind: String,
        /// Spawn X position.
        x: f32,
        /// Spawn Y position.
        y: f32,
    },
    /// An entity is removed from the world.
    EntityDespawn {
        /// Entity identifier.
        entity_id: u32,
    },
    /// Server pong response.
    Pong {
        /// Echo of the client timestamp.
        timestamp_ms: u64,
        /// Server-side timestamp in milliseconds.
        server_ms: u64,
    },

    // -- Bidirectional --
    /// Graceful disconnect with reason.
    Disconnect {
        /// Human-readable disconnect reason.
        reason: String,
    },
}
