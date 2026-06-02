//! Networking stub. Defines the client/server scaffolding the game will grow
//! into; for the MVP it only drains a local outbox.

use serde::{Deserialize, Serialize};

use crate::app::{App, PhaseId, Plugin};
use crate::ecs::{Entity, World};

/// Connection role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum NetMode {
    #[default]
    Offline,
    Client,
    Server,
}

/// A message that would cross the wire in a networked build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetMessage {
    PlayerMoved { entity: u64, x: f32, y: f32 },
    EntitySpawned { entity: u64 },
    EntityDied { entity: u64 },
    Chat { text: String },
}

/// Local networking state.
#[derive(Debug, Clone, Default)]
pub struct NetworkState {
    pub mode: NetMode,
    pub outbox: Vec<NetMessage>,
}

impl NetworkState {
    pub fn queue(&mut self, msg: NetMessage) {
        self.outbox.push(msg);
    }

    pub fn is_online(&self) -> bool {
        self.mode != NetMode::Offline
    }
}

/// Helper so callers can enqueue a move without poking fields directly.
pub fn announce_move(world: &mut World, entity: Entity, x: f32, y: f32) {
    world.net.queue(NetMessage::PlayerMoved { entity: entity.0, x, y });
}

/// Drain the outbox. In `Offline` mode this just clears it; once a transport
/// exists this is where it would be flushed to the socket.
pub fn network_system(world: &mut World) {
    if world.net.outbox.is_empty() {
        return;
    }
    world.net.outbox.clear();
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PhaseId::Network, network_system);
    }
}
