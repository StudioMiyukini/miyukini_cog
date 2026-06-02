//! Networking for Allumina.
//!
//! Authoritative TCP server + thin client, gated behind the `netcode` cargo
//! feature so the default game build stays light.
//!
//! Default endpoint: [`DEFAULT_SERVER`]:[`DEFAULT_PORT`] — the home server
//! exposed via a Cloudflare tunnel (which is why we run TCP rather than QUIC).
//! Per-message framing is a 4-byte little-endian length prefix followed by a
//! bincode payload (see [`protocol`]).
//!
//! Submodules:
//! * [`protocol`] — wire types + codec (always built behind the feature).
//! * [`prediction`] — client-side movement prediction.
//! * [`interpolation`] — smoothing remote entities between 20 Hz snapshots.
//! * [`server`] — the authoritative headless server (`--server`).
//! * [`client`] — the TCP client (`--connect <host>`).

pub mod client;
pub mod interpolation;
pub mod lobby_client;
pub mod prediction;
pub mod protocol;
pub mod server;
pub mod snapshot_renderer;
/// WebSocket↔TCP bridge for the browser client (`--features webclient`).
#[cfg(feature = "webclient")]
pub mod ws_bridge;

use crate::ecs::{Entity, World};
use crate::render::figures::Weapon;
use mge_items::item::{EquipSlot, ItemKind};
use mge_items::rarity::Rarity;
use protocol::{EffectSnapshot, EntityKind, EntitySnapshot, GroundItemSnapshot, ServerMessage, OFFHAND_SHIELD};

/// Default hostname for the authoritative server (home machine, exposed via a
/// Cloudflare tunnel).
pub const DEFAULT_SERVER: &str = "allumina.miyukini-home.org";
/// Default TCP port the authoritative server binds and the client dials.
pub const DEFAULT_PORT: u16 = 7777;
/// Snapshot broadcast rate, Hz.
pub const SNAPSHOT_RATE_HZ: u32 = 20;
/// Maximum simultaneous players the server accepts.
pub const MAX_PLAYERS: usize = protocol::MAX_PLAYERS;

fn class_discriminant(c: crate::components::player::ClassType) -> u8 {
    use crate::components::player::ClassType::*;
    match c {
        Soldat => 0,
        Clerc => 1,
        MageDeGuerre => 2,
        Archer => 3,
    }
}

/// Replicated appearance for a combatant: `(main weapon code, off-hand code)`,
/// read from the player's `equipment` or a mercenary's `merc_inventory`.
fn loadout_for(world: &World, e: Entity) -> (Option<u8>, Option<u8>) {
    use crate::systems::combat::equipped_kind;
    let weapon = equipped_kind(world, e, EquipSlot::MainHand).map(|k| Weapon::for_item_kind(k).to_code());
    let offhand = equipped_kind(world, e, EquipSlot::OffHand).map(|k| {
        if k == ItemKind::Shield { OFFHAND_SHIELD } else { Weapon::for_item_kind(k).to_code() }
    });
    (weapon, offhand)
}

/// A mercenary archetype's class silhouette (for the replicated body).
fn merc_class_disc(t: crate::components::mercenary::MercType) -> u8 {
    use crate::components::mercenary::MercType;
    use crate::components::player::ClassType;
    class_discriminant(match t {
        MercType::Guardian => ClassType::Soldat,
        MercType::Archer => ClassType::Archer,
        MercType::Healer => ClassType::Clerc,
        MercType::Mage => ClassType::MageDeGuerre,
    })
}

fn direction_index(d: crate::render::figures::Direction) -> u8 {
    use crate::render::figures::Direction::*;
    match d {
        N => 0,
        NE => 1,
        E => 2,
        SE => 3,
        S => 4,
        SW => 5,
        W => 6,
        NW => 7,
        Idle => 8,
    }
}

fn rarity_index(r: Rarity) -> u8 {
    match r {
        Rarity::Normal => 0,
        Rarity::Magic => 1,
        Rarity::Rare => 2,
        Rarity::Crafted => 3,
        Rarity::Set => 4,
        Rarity::Unique => 5,
    }
}

/// Build a full [`ServerMessage::WorldSnapshot`] from `world` for tick `tick`.
#[must_use]
pub fn snapshot_message(world: &World, tick: u64) -> ServerMessage {
    let mut entities = Vec::new();
    let mut ground_items = Vec::new();

    let push_entity = |entities: &mut Vec<EntitySnapshot>,
                       id: u64,
                       kind: EntityKind,
                       pos: crate::components::Position2D,
                       hp: f32,
                       hp_max: f32,
                       name: String,
                       direction: u8,
                       class: Option<u8>,
                       level: Option<u32>| {
        entities.push(EntitySnapshot {
            id,
            kind,
            x: pos.x,
            y: pos.y,
            hp: hp as f64,
            hp_max: hp_max as f64,
            name,
            direction,
            anim_state: 0,
            class,
            level,
            weapon: None,
            offhand: None,
        });
    };

    // Players.
    for (e, _) in &world.player_tags {
        let Some(&pos) = world.positions.get(e) else { continue };
        let (hp, hp_max) = world.health.get(e).map_or((1.0, 1.0), |h| (h.current, h.max));
        let dir = world.directions.get(e).copied().unwrap_or_default();
        let class = world.classes.get(e).copied().map(class_discriminant);
        let level = world.progression.get(e).map(|p| p.level());
        let name = world
            .classes
            .get(e)
            .map(|c| c.display_name().to_string())
            .unwrap_or_else(|| "Player".to_string());
        push_entity(
            &mut entities,
            e.0,
            EntityKind::Player,
            pos,
            hp,
            hp_max,
            name,
            direction_index(dir),
            class,
            level,
        );
        if let Some(last) = entities.last_mut() {
            (last.weapon, last.offhand) = loadout_for(world, *e);
        }
    }

    // Mobs.
    for e in world.mob_entities() {
        let Some(&pos) = world.positions.get(&e) else { continue };
        let (hp, hp_max) = world.health.get(&e).map_or((1.0, 1.0), |h| (h.current, h.max));
        let def = world.mob_types.get(&e).and_then(|m| world.data.mob(m.def_id));
        let kind = if def.is_some_and(|d| d.is_boss) { EntityKind::Boss } else { EntityKind::Mob };
        let dir = world.directions.get(&e).copied().unwrap_or_default();
        let name = def.map(|d| d.name.clone()).unwrap_or_else(|| "Mob".to_string());
        push_entity(
            &mut entities,
            e.0,
            kind,
            pos,
            hp,
            hp_max,
            name,
            direction_index(dir),
            None,
            None,
        );
    }

    // Summons + mercenaries.
    for e in world.summon_entities() {
        let Some(&pos) = world.positions.get(&e) else { continue };
        let (hp, hp_max) = world.health.get(&e).map_or((1.0, 1.0), |h| (h.current, h.max));
        let dir = world.directions.get(&e).copied().unwrap_or_default();
        push_entity(
            &mut entities,
            e.0,
            EntityKind::Summon,
            pos,
            hp,
            hp_max,
            "Summon".to_string(),
            direction_index(dir),
            None,
            None,
        );
    }
    for e in world.mercenary_entities() {
        let Some(&pos) = world.positions.get(&e) else { continue };
        let (hp, hp_max) = world.health.get(&e).map_or((1.0, 1.0), |h| (h.current, h.max));
        let dir = world.directions.get(&e).copied().unwrap_or_default();
        let class = world.mercenary_tags.get(&e).map(|t| merc_class_disc(t.merc_type));
        let level = world.merc_states.get(&e).map(|s| s.level);
        push_entity(
            &mut entities,
            e.0,
            EntityKind::Mercenary,
            pos,
            hp,
            hp_max,
            "Mercenary".to_string(),
            direction_index(dir),
            class,
            level,
        );
        if let Some(last) = entities.last_mut() {
            (last.weapon, last.offhand) = loadout_for(world, e);
        }
    }

    // NPCs.
    for (e, _kind) in &world.npcs {
        let Some(&pos) = world.positions.get(e) else { continue };
        let name = if world.merchants.contains_key(e) { "Merchant" } else { "NPC" };
        push_entity(
            &mut entities,
            e.0,
            EntityKind::Npc,
            pos,
            1.0,
            1.0,
            name.to_string(),
            direction_index(Default::default()),
            None,
            None,
        );
    }

    // Ground items.
    for e in world.ground_item_entities() {
        let Some(&pos) = world.positions.get(&e) else { continue };
        let Some(gi) = world.ground_items.get(&e) else { continue };
        ground_items.push(GroundItemSnapshot {
            id: e.0,
            x: pos.x,
            y: pos.y,
            name: gi.instance.name.clone(),
            rarity: rarity_index(gi.instance.rarity()),
            kind: format!("{:?}", gi.instance.item.base.kind),
        });
    }

    let effects: Vec<EffectSnapshot> = Vec::new();

    ServerMessage::WorldSnapshot { tick, entities, ground_items, effects }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::maps::act1_map;
    use crate::data::GameData;
    use crate::plugins::player::spawn_player_system;

    #[test]
    fn snapshot_message_includes_player_and_mobs() {
        let mut world = World::new(act1_map(), GameData::load(), 1);
        spawn_player_system(&mut world);
        let mob = world.spawn();
        world.positions.insert(mob, crate::components::Position2D::new(5.0, 5.0));
        world.health.insert(mob, crate::components::Health::full(30.0));
        world.mob_tags.insert(mob, crate::components::mob::MobTag);
        world.mob_types.insert(mob, crate::components::mob::MobType { def_id: 1 });

        let snap = snapshot_message(&world, 3);
        match snap {
            ServerMessage::WorldSnapshot { tick, entities, .. } => {
                assert_eq!(tick, 3);
                assert!(entities.iter().any(|e| e.kind == EntityKind::Player));
                assert!(entities
                    .iter()
                    .any(|e| matches!(e.kind, EntityKind::Mob | EntityKind::Boss)));
            }
            _ => panic!("expected WorldSnapshot"),
        }
    }
}
