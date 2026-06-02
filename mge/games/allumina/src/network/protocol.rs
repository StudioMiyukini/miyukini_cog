//! Wire protocol for the authoritative Allumina server.
//!
//! Transport: TCP (so Cloudflare tunnels relay it without UDP/QUIC quirks),
//! framing: 4-byte little-endian length prefix + bincode payload.
//! All message structs are engine-agnostic so the wire format is stable and
//! testable on its own.

use serde::{Deserialize, Serialize};

use crate::components::player::ClassType;
use crate::save::{SaveData, SavedItem};

/// Snapshot tick rate (server → client) in Hz.
pub const TICK_RATE: u8 = 20;

/// Maximum simultaneous players the server accepts per game.
pub const MAX_PLAYERS: usize = 8;

/// Largest accepted single message frame (1 MiB).
pub const MAX_FRAME: usize = 1 << 20;

// ── Lobby value types ───────────────────────────────────────────────────────

/// Difficulty tier picked at game creation. Drives mob stats / loot tables in
/// the simulation (the server is authoritative on the actual scaling).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Difficulty {
    Normal = 0,
    Cauchemar = 1,
    Enfer = 2,
}

impl Difficulty {
    /// Stable French display label used by the UI list.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Cauchemar => "Cauchemar",
            Self::Enfer => "Enfer",
        }
    }
}

/// Public-facing summary of a [`GameRoom`] shown in the lobby list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameInfo {
    pub id: u64,
    pub name: String,
    pub players: u32,
    pub max_players: u32,
    pub difficulty: Difficulty,
    pub has_password: bool,
    /// Average player level (0 when the room is empty).
    pub avg_level: f32,
    /// Display name of the player who created the room.
    pub host_name: String,
}

// ── Snapshot value types ────────────────────────────────────────────────────

/// Visual category for a replicated entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum EntityKind {
    Player = 0,
    Mob = 1,
    Summon = 2,
    Mercenary = 3,
    Npc = 4,
    Boss = 5,
}

/// One replicated entity in a [`ServerMessage::WorldSnapshot`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub id: u64,
    pub kind: EntityKind,
    pub x: f32,
    pub y: f32,
    pub hp: f64,
    pub hp_max: f64,
    pub name: String,
    /// 0–8 corresponding to [`crate::render::figures::Direction`].
    pub direction: u8,
    /// 0=idle, 1=walk, 2=attack, 3=cast, 4=hit, 5=death.
    pub anim_state: u8,
    /// For players & mercenaries: discriminant of [`ClassType`] (mercs use their
    /// archetype's class for the body silhouette).
    pub class: Option<u8>,
    pub level: Option<u32>,
    /// Main-hand weapon silhouette code ([`crate::render::figures::Weapon::to_code`]).
    /// `None` for non-combatants.
    pub weapon: Option<u8>,
    /// Off-hand appearance: `None` = empty, `Some(255)` = shield, otherwise a
    /// [`Weapon`](crate::render::figures::Weapon) code for a dual-wield weapon.
    pub offhand: Option<u8>,
}

/// `offhand` sentinel meaning "a shield" (vs. a weapon code for dual-wield).
pub const OFFHAND_SHIELD: u8 = 255;

/// One replicated ground item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroundItemSnapshot {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub name: String,
    pub rarity: u8,
    pub kind: String,
}

/// Transient visual effect (particle, AoE ring, projectile) sent alongside
/// snapshots.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectSnapshot {
    pub kind: String,
    pub x: f32,
    pub y: f32,
    /// Free-form bincode-encoded parameters keyed off `kind`.
    pub data: Vec<u8>,
}

// ── Client → Server ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClientMessage {
    Login {
        player_id: String,
        character_name: String,
        class: ClassType,
        save_data: Option<SaveData>,
    },
    /// Ask the server for the current public game list (lobby phase).
    ListGames,
    /// Create a new [`GameRoom`] (lobby phase). On success the server enrols
    /// the caller in that room and replies with [`ServerMessage::GameCreated`]
    /// followed by [`ServerMessage::JoinedGame`].
    CreateGame {
        name: String,
        max_players: u32,
        difficulty: Difficulty,
        /// `None` ⇒ public game; `Some(pw)` ⇒ private (password-gated).
        password: Option<String>,
    },
    /// Join an existing room (lobby phase). The password must match for
    /// private games; pass `None` for public games.
    JoinGame {
        game_id: u64,
        password: Option<String>,
    },
    /// Leave the current game and return to the lobby (in-game phase).
    LeaveGame,
    MoveIntent {
        target_x: f32,
        target_y: f32,
    },
    AttackTarget {
        entity_id: u64,
    },
    UseSkill {
        skill_index: u32,
        target: Option<u64>,
        cursor_x: f32,
        cursor_y: f32,
    },
    PickupItem {
        ground_item_id: u64,
    },
    DropItem {
        inventory_slot: u32,
    },
    EquipItem {
        inventory_slot: u32,
    },
    UnequipItem {
        slot: String,
    },
    ChatMessage {
        text: String,
    },
    /// Request a fast-travel teleport to a waypoint the client believes it has
    /// discovered. The server re-validates against its own activated list
    /// before queuing the 1s cast.
    TeleportToWaypoint {
        wp_id: u32,
    },
    Ping {
        timestamp: u64,
    },
    Disconnect,
}

// ── Server → Client ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServerMessage {
    /// Sent once after a successful [`ClientMessage::Login`]. The client is
    /// now in the lobby phase; `your_entity_id` is `0` until a game is joined.
    LoginAccepted {
        your_entity_id: u64,
        tick: u64,
    },
    LoginRejected {
        reason: String,
    },
    /// Current list of joinable games (sent in response to
    /// [`ClientMessage::ListGames`], and after lobby-state changes).
    GameList {
        games: Vec<GameInfo>,
    },
    /// Acknowledgement of [`ClientMessage::CreateGame`]. Always followed by
    /// [`ServerMessage::JoinedGame`] for the creator.
    GameCreated {
        game_id: u64,
    },
    /// The client has been placed inside the room `game_id` with the given
    /// entity-id; world snapshots will follow.
    JoinedGame {
        game_id: u64,
        your_entity_id: u64,
        tick: u64,
    },
    /// Failed to create or join a room (full, wrong password, unknown id…).
    JoinFailed {
        reason: String,
    },
    /// The client has been returned to the lobby (after `LeaveGame` or the
    /// room being closed). `GameList` typically follows.
    ReturnedToLobby,
    /// A previously-listed room has been destroyed (last player left + grace
    /// period elapsed). Lobby clients should drop it from their cached list.
    GameClosed {
        game_id: u64,
    },
    WorldSnapshot {
        tick: u64,
        entities: Vec<EntitySnapshot>,
        ground_items: Vec<GroundItemSnapshot>,
        effects: Vec<EffectSnapshot>,
    },
    EntityDied {
        entity_id: u64,
    },
    EntitySpawned {
        snapshot: EntitySnapshot,
    },
    LootDropped {
        item: GroundItemSnapshot,
    },
    DamageDealt {
        target: u64,
        amount: f64,
        crit: bool,
    },
    SkillUsed {
        caster: u64,
        skill_id: u32,
        target: Option<u64>,
        pos: (f32, f32),
    },
    InventoryUpdate {
        slots: Vec<(u32, Option<SavedItem>)>,
    },
    EquipmentUpdate {
        slot: String,
        item: Option<SavedItem>,
    },
    GoldUpdate {
        amount: u64,
    },
    ChatBroadcast {
        sender: String,
        text: String,
    },
    PlayerJoined {
        name: String,
        class: String,
    },
    PlayerLeft {
        name: String,
    },
    Pong {
        client_timestamp: u64,
        server_timestamp: u64,
    },
    /// Broadcast when any player activates a waypoint.
    WaypointDiscovered {
        player_name: String,
        wp_name: String,
        wp_id: u32,
    },
    ServerShutdown {
        reason: String,
    },
}

// ── Codec ───────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum CodecError {
    Encode(String),
    Decode(String),
    FrameTooLarge(usize),
}

impl std::fmt::Display for CodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Encode(e) => write!(f, "encode error: {e}"),
            Self::Decode(e) => write!(f, "decode error: {e}"),
            Self::FrameTooLarge(n) => write!(f, "frame too large: {n} bytes"),
        }
    }
}

impl std::error::Error for CodecError {}

/// Encode a message to raw bincode bytes.
pub fn encode<T: Serialize>(msg: &T) -> Result<Vec<u8>, CodecError> {
    bincode::serialize(msg).map_err(|e| CodecError::Encode(e.to_string()))
}

/// Decode a message from raw bincode bytes.
pub fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, CodecError> {
    bincode::deserialize(bytes).map_err(|e| CodecError::Decode(e.to_string()))
}

/// Encode a message with a 4-byte little-endian length prefix (stream framing).
pub fn frame<T: Serialize>(msg: &T) -> Result<Vec<u8>, CodecError> {
    let body = encode(msg)?;
    if body.len() > MAX_FRAME {
        return Err(CodecError::FrameTooLarge(body.len()));
    }
    let mut out = Vec::with_capacity(4 + body.len());
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&body);
    Ok(out)
}

/// Try to split one length-prefixed frame off the front of `buf`. On success
/// returns the decoded message and removes its bytes from `buf`.
pub fn try_unframe<T: for<'de> Deserialize<'de>>(buf: &mut Vec<u8>) -> Result<Option<T>, CodecError> {
    if buf.len() < 4 {
        return Ok(None);
    }
    let len = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
    if len > MAX_FRAME {
        return Err(CodecError::FrameTooLarge(len));
    }
    if buf.len() < 4 + len {
        return Ok(None);
    }
    let msg = decode::<T>(&buf[4..4 + len])?;
    buf.drain(..4 + len);
    Ok(Some(msg))
}

/// Convenience: encode then `write_all` to any [`std::io::Write`].
pub fn write_message<W: std::io::Write, T: Serialize>(out: &mut W, msg: &T) -> std::io::Result<()> {
    let bytes = frame(msg).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    out.write_all(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_message_round_trip() {
        let m = ClientMessage::MoveIntent { target_x: 3.5, target_y: -2.0 };
        let bytes = encode(&m).unwrap();
        assert_eq!(decode::<ClientMessage>(&bytes).unwrap(), m);
    }

    #[test]
    fn login_round_trip_without_save() {
        let m = ClientMessage::Login {
            player_id: "u-123".into(),
            character_name: "Aeryn".into(),
            class: ClassType::Soldat,
            save_data: None,
        };
        let bytes = encode(&m).unwrap();
        assert_eq!(decode::<ClientMessage>(&bytes).unwrap(), m);
    }

    #[test]
    fn snapshot_round_trip() {
        let snap = ServerMessage::WorldSnapshot {
            tick: 42,
            entities: vec![
                EntitySnapshot {
                    id: 1,
                    kind: EntityKind::Player,
                    x: 1.0,
                    y: 2.0,
                    hp: 50.0,
                    hp_max: 100.0,
                    name: "Hero".into(),
                    direction: 4,
                    anim_state: 1,
                    class: Some(0),
                    level: Some(7),
                    weapon: Some(1),
                    offhand: Some(3),
                },
                EntitySnapshot {
                    id: 2,
                    kind: EntityKind::Boss,
                    x: 4.0,
                    y: 5.0,
                    hp: 800.0,
                    hp_max: 1000.0,
                    name: "Cinder Lord".into(),
                    direction: 0,
                    anim_state: 2,
                    class: None,
                    level: None,
                    weapon: None,
                    offhand: None,
                },
            ],
            ground_items: vec![GroundItemSnapshot {
                id: 9,
                x: 3.0,
                y: 3.0,
                name: "Long Sword".into(),
                rarity: 1,
                kind: "Sword".into(),
            }],
            effects: vec![EffectSnapshot {
                kind: "aoe_ring".into(),
                x: 1.0,
                y: 1.0,
                data: vec![0xDE, 0xAD],
            }],
        };
        let bytes = encode(&snap).unwrap();
        assert_eq!(decode::<ServerMessage>(&bytes).unwrap(), snap);
    }

    #[test]
    fn framing_then_unframing() {
        let a = ClientMessage::Ping { timestamp: 123 };
        let b = ClientMessage::ChatMessage { text: "gg".into() };
        let mut stream = frame(&a).unwrap();
        stream.extend(frame(&b).unwrap());

        let m1: Option<ClientMessage> = try_unframe(&mut stream).unwrap();
        assert_eq!(m1, Some(a));
        let m2: Option<ClientMessage> = try_unframe(&mut stream).unwrap();
        assert_eq!(m2, Some(b));
        let m3: Option<ClientMessage> = try_unframe(&mut stream).unwrap();
        assert_eq!(m3, None);
        assert!(stream.is_empty());
    }

    #[test]
    fn framing_uses_little_endian_length() {
        // A tiny payload (Disconnect serializes to a 4-byte variant index).
        let m = ClientMessage::Disconnect;
        let f = frame(&m).unwrap();
        // First 4 bytes = LE u32 length of body (= 4 for the variant index).
        let len = u32::from_le_bytes([f[0], f[1], f[2], f[3]]) as usize;
        assert_eq!(len + 4, f.len());
    }

    #[test]
    fn partial_frame_waits_for_more_bytes() {
        let a = ClientMessage::Ping { timestamp: 1 };
        let full = frame(&a).unwrap();
        let mut partial = full[..full.len() - 1].to_vec();
        assert_eq!(try_unframe::<ClientMessage>(&mut partial).unwrap(), None);
    }

    #[test]
    fn lobby_messages_round_trip() {
        let create = ClientMessage::CreateGame {
            name: "Salle des héros".into(),
            max_players: 4,
            difficulty: Difficulty::Cauchemar,
            password: Some("hunter2".into()),
        };
        let join = ClientMessage::JoinGame { game_id: 17, password: None };
        for m in [ClientMessage::ListGames, ClientMessage::LeaveGame, create, join] {
            let bytes = encode(&m).unwrap();
            assert_eq!(decode::<ClientMessage>(&bytes).unwrap(), m);
        }
    }

    #[test]
    fn game_list_round_trip() {
        let m = ServerMessage::GameList {
            games: vec![
                GameInfo {
                    id: 1,
                    name: "Public Run".into(),
                    players: 2,
                    max_players: 8,
                    difficulty: Difficulty::Normal,
                    has_password: false,
                    avg_level: 12.5,
                    host_name: "Aeryn".into(),
                },
                GameInfo {
                    id: 2,
                    name: "Private Hardcore".into(),
                    players: 3,
                    max_players: 4,
                    difficulty: Difficulty::Enfer,
                    has_password: true,
                    avg_level: 60.0,
                    host_name: "Kael".into(),
                },
            ],
        };
        let bytes = encode(&m).unwrap();
        assert_eq!(decode::<ServerMessage>(&bytes).unwrap(), m);
    }

    #[test]
    fn game_closed_round_trip() {
        let m = ServerMessage::GameClosed { game_id: 9 };
        let bytes = encode(&m).unwrap();
        assert_eq!(decode::<ServerMessage>(&bytes).unwrap(), m);
    }

    #[test]
    fn joined_game_round_trip() {
        let m = ServerMessage::JoinedGame { game_id: 42, your_entity_id: 7, tick: 100 };
        let bytes = encode(&m).unwrap();
        assert_eq!(decode::<ServerMessage>(&bytes).unwrap(), m);
    }

    #[test]
    fn difficulty_labels_are_french() {
        assert_eq!(Difficulty::Normal.label(), "Normal");
        assert_eq!(Difficulty::Cauchemar.label(), "Cauchemar");
        assert_eq!(Difficulty::Enfer.label(), "Enfer");
    }

    #[test]
    fn oversized_length_prefix_is_rejected() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&((MAX_FRAME as u32) + 1).to_le_bytes());
        let err = try_unframe::<ClientMessage>(&mut buf).unwrap_err();
        assert!(matches!(err, CodecError::FrameTooLarge(_)));
    }
}
