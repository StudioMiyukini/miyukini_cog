//! Authoritative TCP server for Allumina multiplayer with a lobby.
//!
//! Layout:
//! * One [`LobbyServer`] (Arc<Mutex>) holds every active [`GameRoom`] (one per
//!   game) plus the clients currently sitting in the lobby phase.
//! * Each [`GameRoom`] owns its own [`crate::app::App`] (independent
//!   simulation) and its own player list, behind its own [`Mutex`] so per-room
//!   ticks don't block lobby browsing.
//! * One thread accepts TCP connections; per-connection we run a reader thread
//!   (parses framed [`ClientMessage`]s) + writer thread (drains an mpsc of
//!   [`ServerMessage`]s).
//! * One simulation thread iterates over every room, ticks at 60 Hz internally
//!   and broadcasts a full snapshot at 20 Hz.
//!
//! Connection state machine (tracked locally per connection):
//! * `Connected` → on [`ClientMessage::Login`] (with optional save) → `InLobby`.
//! * `InLobby` → on `CreateGame`/`JoinGame` → `InGame { room, entity }`.
//! * `InGame` → on `LeaveGame` (or room destruction) → `InLobby`.
//!
//! Authoritative validation lives next to each handler: save checksum on
//! login, password + capacity on join, movement/skill range/cooldown/mana on
//! intents.

use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};

use crate::components::player::{ClassType, PlayerTag, Stats};
use crate::components::{Combat, Faction, Health, Mana, MoveTarget, Position2D};
use crate::ecs::Entity;
use crate::save::{self, SaveData, AUTOSAVE_INTERVAL_SECS};
use crate::systems::movement::find_path;

use super::protocol::{
    self, ClientMessage, Difficulty, GameInfo, ServerMessage, MAX_FRAME, MAX_PLAYERS, TICK_RATE,
};
use super::{snapshot_message, DEFAULT_PORT};

const PLAYER_SPEED: f32 = 4.5;
const MAX_ATTACK_RANGE: f32 = 32.0;
const MAX_LOGIN_PAYLOAD: usize = 512 * 1024;
const MAX_GAME_NAME_LEN: usize = 48;
const MAX_PASSWORD_LEN: usize = 64;
const MAX_CHAT_LEN: usize = 280;
/// Cap on simultaneous rooms hosted by one server instance.
pub const MAX_ROOMS: usize = 10;
/// How long an empty (non-persistent) room is kept alive before it gets GC'd
/// so a player who briefly disconnects can rejoin the same world.
pub const ROOM_GRACE_SECS: f32 = 300.0;

// ── Types ───────────────────────────────────────────────────────────────────

/// One client sitting in the lobby (post-login, not yet in a game).
struct LobbyClient {
    character_name: String,
    #[allow(dead_code)]
    player_id: String,
    class: ClassType,
    save_data: Option<SaveData>,
    tx: Sender<ServerMessage>,
}

/// One player inside an active room.
struct PlayerConn {
    entity: Entity,
    #[allow(dead_code)]
    player_id: String,
    character_name: String,
    #[allow(dead_code)]
    class: ClassType,
    tx: Sender<ServerMessage>,
}

/// One independently-simulated game session.
struct GameRoom {
    id: u64,
    name: String,
    difficulty: Difficulty,
    max_players: u32,
    /// `None` ⇒ public game; `Some(pw)` ⇒ private.
    password: Option<String>,
    /// Display name of the player who created the room (shown in the lobby
    /// list). Stays stable even if the host disconnects.
    host_name: String,
    /// If `true` the room survives the empty-room grace timer and is never
    /// auto-GC'd.
    persistent: bool,
    app: crate::app::App,
    /// Indexed by entity-id (each player owns one avatar).
    players: HashMap<u64, PlayerConn>,
    tick: u64,
    /// Entities that died during the last sim window — broadcast as
    /// [`ServerMessage::EntityDied`] on the next snapshot tick.
    deaths_pending: VecDeque<u64>,
    /// `Some(when_emptied)` while no players are present; cleared when anyone
    /// rejoins. Used by the sweep loop to decide when to drop the room.
    empty_since: Option<Instant>,
}

impl GameRoom {
    fn info(&self) -> GameInfo {
        let players = self.players.len() as u32;
        let avg_level = if players == 0 {
            0.0
        } else {
            let sum: u32 = self
                .players
                .keys()
                .filter_map(|id| {
                    self.app
                        .world
                        .progression
                        .get(&Entity(*id))
                        .map(|p| p.level())
                })
                .sum();
            sum as f32 / players as f32
        };
        GameInfo {
            id: self.id,
            name: self.name.clone(),
            players,
            max_players: self.max_players,
            difficulty: self.difficulty,
            has_password: self.password.is_some(),
            avg_level,
            host_name: self.host_name.clone(),
        }
    }

    fn broadcast(&self, msg: ServerMessage) {
        for p in self.players.values() {
            let _ = p.tx.send(msg.clone());
        }
    }
}

/// Top-level shared state: every room plus everyone in the lobby.
struct LobbyServer {
    rooms: HashMap<u64, Arc<Mutex<GameRoom>>>,
    next_room_id: u64,
    lobby_clients: HashMap<u64, LobbyClient>,
    next_client_id: u64,
}

impl LobbyServer {
    fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            next_room_id: 1,
            lobby_clients: HashMap::new(),
            next_client_id: 1,
        }
    }

    fn game_list(&self) -> Vec<GameInfo> {
        let mut games: Vec<GameInfo> = self
            .rooms
            .values()
            .filter_map(|r| r.lock().ok().map(|g| g.info()))
            .collect();
        games.sort_by(|a, b| a.id.cmp(&b.id));
        games
    }

    /// Push a fresh [`ServerMessage::GameList`] to every lobby client.
    fn broadcast_lobby_game_list(&self) {
        let list = self.game_list();
        let msg = ServerMessage::GameList { games: list };
        for c in self.lobby_clients.values() {
            let _ = c.tx.send(msg.clone());
        }
    }
}

/// Per-connection local phase.
#[derive(Clone)]
enum ClientPhase {
    #[allow(dead_code)]
    Connected,
    InLobby { client_id: u64 },
    InGame { room_id: u64, entity_id: u64 },
}

// ── Entry point ─────────────────────────────────────────────────────────────

/// Blocking entry point. Binds the TCP listener and spawns simulation +
/// per-connection threads; never returns under normal operation.
pub fn run_server(port: u16) -> Result<()> {
    let port = if port == 0 { DEFAULT_PORT } else { port };
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let listener = TcpListener::bind(addr).with_context(|| format!("bind {addr}"))?;
    println!(
        "[server] Allumina lobby server listening on {addr} ({TICK_RATE} tps, up to {MAX_PLAYERS} players/room)"
    );

    let lobby = Arc::new(Mutex::new(LobbyServer::new()));

    // Simulation + broadcast loop.
    let sim_lobby = lobby.clone();
    std::thread::spawn(move || simulation_loop(sim_lobby));

    // Accept loop on the main thread.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let lobby = lobby.clone();
                std::thread::spawn(move || {
                    if let Err(e) = handle_connection(stream, lobby) {
                        eprintln!("[server] connection ended: {e}");
                    }
                });
            }
            Err(e) => eprintln!("[server] accept error: {e}"),
        }
    }
    Ok(())
}

// ── Simulation loop ─────────────────────────────────────────────────────────

fn simulation_loop(lobby: Arc<Mutex<LobbyServer>>) {
    let sim_dt = 1.0 / 60.0;
    let sim_period = Duration::from_secs_f32(sim_dt);
    let snapshot_period = Duration::from_secs_f64(1.0 / f64::from(TICK_RATE));
    let mut next_sim = Instant::now() + sim_period;
    let mut next_snapshot = Instant::now() + snapshot_period;
    let mut next_autosave = Instant::now() + Duration::from_secs_f32(AUTOSAVE_INTERVAL_SECS);
    let server_start = Instant::now();

    loop {
        let now = Instant::now();
        let next = next_sim.min(next_snapshot).min(next_autosave);
        if next > now {
            std::thread::sleep(next - now);
        }
        let now = Instant::now();

        if now >= next_sim {
            tick_all_rooms(&lobby, sim_dt);
            sweep_expired_rooms(&lobby);
            next_sim += sim_period;
            if next_sim < now {
                next_sim = now + sim_period;
            }
        }
        if now >= next_snapshot {
            broadcast_all_snapshots(&lobby);
            next_snapshot += snapshot_period;
            if next_snapshot < now {
                next_snapshot = now + snapshot_period;
            }
        }
        if now >= next_autosave {
            autosave_all_rooms(&lobby, server_start);
            next_autosave += Duration::from_secs_f32(AUTOSAVE_INTERVAL_SECS);
        }
    }
}

fn rooms_snapshot(lobby: &Arc<Mutex<LobbyServer>>) -> Vec<Arc<Mutex<GameRoom>>> {
    match lobby.lock() {
        Ok(g) => g.rooms.values().cloned().collect(),
        Err(p) => p.into_inner().rooms.values().cloned().collect(),
    }
}

fn tick_all_rooms(lobby: &Arc<Mutex<LobbyServer>>, dt: f32) {
    for room in rooms_snapshot(lobby) {
        let mut r = match room.lock() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        };
        // Skip empty rooms — no point advancing a world nobody is watching.
        if r.players.is_empty() {
            continue;
        }
        let prev_alive: Vec<(Entity, bool)> = r
            .app
            .world
            .health
            .iter()
            .map(|(e, h)| (*e, !h.is_empty()))
            .collect();

        r.app.tick(dt);
        r.tick += 1;
        r.app.world.render_events.clear();
        let _ = r.app.world.drain_log();

        // Broadcast newly-activated waypoints so other players see them.
        let wp_events: Vec<_> = std::mem::take(&mut r.app.world.waypoint_events);
        for (entity, wp_id, wp_name) in wp_events {
            let player_name = r
                .players
                .get(&entity.0)
                .map(|p| p.character_name.clone())
                .unwrap_or_default();
            r.broadcast(ServerMessage::WaypointDiscovered { player_name, wp_name, wp_id });
        }

        for (e, was_alive) in prev_alive {
            if was_alive {
                let still = r.app.world.health.get(&e).is_some_and(|h| !h.is_empty());
                if !still {
                    r.deaths_pending.push_back(e.0);
                }
            }
        }
    }
}

/// Drop rooms that have been empty (and non-persistent) for longer than
/// [`ROOM_GRACE_SECS`]. Notifies remaining lobby clients with `GameClosed`.
fn sweep_expired_rooms(lobby: &Arc<Mutex<LobbyServer>>) {
    let grace = Duration::from_secs_f32(ROOM_GRACE_SECS);
    let now = Instant::now();
    let expired: Vec<u64> = rooms_snapshot(lobby)
        .into_iter()
        .filter_map(|room| {
            let r = match room.lock() {
                Ok(g) => g,
                Err(p) => p.into_inner(),
            };
            if !r.persistent
                && r.players.is_empty()
                && r.empty_since.is_some_and(|t| now.duration_since(t) >= grace)
            {
                Some(r.id)
            } else {
                None
            }
        })
        .collect();
    if expired.is_empty() {
        return;
    }
    let mut l = lobby_lock(lobby);
    for id in &expired {
        l.rooms.remove(id);
        for c in l.lobby_clients.values() {
            let _ = c.tx.send(ServerMessage::GameClosed { game_id: *id });
        }
        println!("[server] room #{id} dropped after grace period");
    }
    l.broadcast_lobby_game_list();
}

fn broadcast_all_snapshots(lobby: &Arc<Mutex<LobbyServer>>) {
    for room in rooms_snapshot(lobby) {
        let mut r = match room.lock() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        };
        if r.players.is_empty() {
            continue;
        }
        let tick = r.tick;
        let snap = snapshot_message(&r.app.world, tick);
        r.broadcast(snap);
        while let Some(id) = r.deaths_pending.pop_front() {
            r.broadcast(ServerMessage::EntityDied { entity_id: id });
        }
    }
}

fn autosave_all_rooms(lobby: &Arc<Mutex<LobbyServer>>, server_start: Instant) {
    let session_secs = server_start.elapsed().as_secs();
    for room in rooms_snapshot(lobby) {
        let r = match room.lock() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        };
        if r.players.is_empty() {
            continue;
        }
        let players: Vec<(u64, String, String)> = r
            .players
            .values()
            .map(|p| (p.entity.0, p.character_name.clone(), p.player_id.clone()))
            .collect();
        for (id, name, pid) in players {
            if let Some(data) = save::snapshot(&r.app.world, &name, &pid, session_secs) {
                let _ = save::save_to_slot(&data, 0);
                if let Some(p) = r.players.get(&id) {
                    let slots = data
                        .inventory
                        .iter()
                        .enumerate()
                        .map(|(i, item)| (i as u32, Some(item.clone())))
                        .collect();
                    let _ = p.tx.send(ServerMessage::InventoryUpdate { slots });
                    let _ = p.tx.send(ServerMessage::GoldUpdate { amount: data.gold });
                }
            }
        }
    }
}

// ── Per-connection lifecycle ────────────────────────────────────────────────

fn handle_connection(stream: TcpStream, lobby: Arc<Mutex<LobbyServer>>) -> Result<()> {
    let peer = stream.peer_addr().ok();
    stream.set_nodelay(true).ok();

    let writer_stream = stream.try_clone().context("clone tcp stream")?;
    let mut reader_stream = stream;

    let (tx, rx): (Sender<ServerMessage>, Receiver<ServerMessage>) = mpsc::channel();
    let writer_handle = std::thread::spawn(move || writer_loop(writer_stream, rx));

    // ── Login handshake: first message MUST be Login ──
    let mut buf: Vec<u8> = Vec::with_capacity(4096);
    let login = match read_one_message(&mut reader_stream, &mut buf) {
        Ok(Some(msg)) => msg,
        Ok(None) => {
            drop(tx);
            let _ = writer_handle.join();
            return Ok(());
        }
        Err(e) => {
            drop(tx);
            let _ = writer_handle.join();
            return Err(e);
        }
    };

    let (player_id, character_name, class, save_data) = match login {
        ClientMessage::Login { player_id, character_name, class, save_data } => {
            (player_id, character_name, class, save_data)
        }
        other => {
            let _ = tx.send(ServerMessage::LoginRejected {
                reason: format!("expected Login, got {}", short_kind(&other)),
            });
            drop(tx);
            let _ = writer_handle.join();
            return Ok(());
        }
    };

    // Validate name + save checksum, then enter the lobby.
    if let Err(reason) = validate_login(&character_name, save_data.as_ref()) {
        let _ = tx.send(ServerMessage::LoginRejected { reason });
        drop(tx);
        let _ = writer_handle.join();
        return Ok(());
    }

    let client_id = {
        let mut l = lobby_lock(&lobby);
        let id = l.next_client_id;
        l.next_client_id += 1;
        l.lobby_clients.insert(
            id,
            LobbyClient {
                character_name: character_name.clone(),
                player_id: player_id.clone(),
                class,
                save_data: save_data.clone(),
                tx: tx.clone(),
            },
        );
        id
    };
    let _ = tx.send(ServerMessage::LoginAccepted { your_entity_id: 0, tick: 0 });
    // Welcome the new lobby user with the current game list.
    {
        let l = lobby_lock(&lobby);
        let _ = tx.send(ServerMessage::GameList { games: l.game_list() });
    }
    println!(
        "[server] {character_name} (#{client_id}) joined lobby from {}",
        peer.map(|a| a.to_string()).unwrap_or_else(|| "?".into())
    );

    let mut phase = ClientPhase::InLobby { client_id };

    // ── Main reader loop dispatched by phase ──
    let read_result = reader_loop(&mut reader_stream, &mut buf, &lobby, &mut phase, &tx);

    // Cleanup on disconnect.
    cleanup_phase(&lobby, &phase, &character_name);
    drop(tx);
    let _ = writer_handle.join();
    read_result
}

fn cleanup_phase(lobby: &Arc<Mutex<LobbyServer>>, phase: &ClientPhase, character_name: &str) {
    match phase {
        ClientPhase::Connected => {}
        ClientPhase::InLobby { client_id } => {
            let mut l = lobby_lock(lobby);
            l.lobby_clients.remove(client_id);
            println!("[server] {character_name} (#{client_id}) left lobby");
        }
        ClientPhase::InGame { room_id, entity_id } => {
            remove_player_from_room(lobby, *room_id, *entity_id, character_name);
        }
    }
}

fn remove_player_from_room(
    lobby: &Arc<Mutex<LobbyServer>>,
    room_id: u64,
    entity_id: u64,
    character_name: &str,
) {
    let room = match lobby_lock(lobby).rooms.get(&room_id).cloned() {
        Some(r) => r,
        None => return,
    };
    {
        let mut r = room_lock(&room);
        if let Some(p) = r.players.remove(&entity_id) {
            r.app.world.despawn(p.entity);
            let name = p.character_name.clone();
            r.broadcast(ServerMessage::PlayerLeft { name });
            println!(
                "[server] {character_name} (#{entity_id}) left room {} ({} online in room)",
                r.name,
                r.players.len()
            );
        }
        // Empty + non-persistent rooms enter the grace window; the sim loop
        // sweeps them after `ROOM_GRACE_SECS` so a quick reconnect can hop
        // back into the same world.
        if r.players.is_empty() && !r.persistent && r.empty_since.is_none() {
            r.empty_since = Some(Instant::now());
        }
    }
    lobby_lock(lobby).broadcast_lobby_game_list();
}

fn validate_login(character_name: &str, save: Option<&SaveData>) -> std::result::Result<(), String> {
    if character_name.trim().is_empty() {
        return Err("empty character name".into());
    }
    if character_name.len() > 32 {
        return Err("character name too long".into());
    }
    if let Some(sd) = save {
        verify_save_checksum(sd).map_err(|e| format!("save rejected: {e}"))?;
    }
    Ok(())
}

// ── Reader loop / phase machine ─────────────────────────────────────────────

fn reader_loop(
    stream: &mut TcpStream,
    buf: &mut Vec<u8>,
    lobby: &Arc<Mutex<LobbyServer>>,
    phase: &mut ClientPhase,
    tx: &Sender<ServerMessage>,
) -> Result<()> {
    let mut chunk = [0u8; 4096];
    loop {
        loop {
            match protocol::try_unframe::<ClientMessage>(buf) {
                Ok(Some(msg)) => {
                    if matches!(msg, ClientMessage::Disconnect) {
                        return Ok(());
                    }
                    dispatch_message(lobby, phase, tx, msg);
                }
                Ok(None) => break,
                Err(e) => return Err(anyhow::anyhow!("decode error: {e}")),
            }
        }
        match stream.read(&mut chunk) {
            Ok(0) => return Ok(()),
            Ok(n) => buf.extend_from_slice(&chunk[..n]),
            Err(e) => return Err(e.into()),
        }
    }
}

fn dispatch_message(
    lobby: &Arc<Mutex<LobbyServer>>,
    phase: &mut ClientPhase,
    tx: &Sender<ServerMessage>,
    msg: ClientMessage,
) {
    match phase.clone() {
        ClientPhase::Connected => {
            // Login is consumed before reader_loop starts; anything else is a
            // protocol error.
            let _ = tx.send(ServerMessage::JoinFailed { reason: "not authenticated".into() });
        }
        ClientPhase::InLobby { client_id } => match msg {
            ClientMessage::ListGames => {
                let list = lobby_lock(lobby).game_list();
                let _ = tx.send(ServerMessage::GameList { games: list });
            }
            ClientMessage::CreateGame { name, max_players, difficulty, password } => {
                match create_game(lobby, client_id, name, max_players, difficulty, password) {
                    Ok((room_id, entity_id, tick)) => {
                        let _ = tx.send(ServerMessage::GameCreated { game_id: room_id });
                        let _ = tx.send(ServerMessage::JoinedGame {
                            game_id: room_id,
                            your_entity_id: entity_id,
                            tick,
                        });
                        *phase = ClientPhase::InGame { room_id, entity_id };
                    }
                    Err(reason) => {
                        let _ = tx.send(ServerMessage::JoinFailed { reason });
                    }
                }
            }
            ClientMessage::JoinGame { game_id, password } => {
                match join_game(lobby, client_id, game_id, password) {
                    Ok((entity_id, tick)) => {
                        let _ = tx.send(ServerMessage::JoinedGame {
                            game_id,
                            your_entity_id: entity_id,
                            tick,
                        });
                        *phase = ClientPhase::InGame { room_id: game_id, entity_id };
                    }
                    Err(reason) => {
                        let _ = tx.send(ServerMessage::JoinFailed { reason });
                    }
                }
            }
            ClientMessage::ChatMessage { text } => {
                // Lobby-wide chat: send back as an unattributed broadcast to
                // the sender only (full lobby chat would need a separate room).
                let text = clamp_chat(text);
                let _ = tx.send(ServerMessage::ChatBroadcast { sender: "lobby".into(), text });
            }
            ClientMessage::Ping { timestamp } => {
                let _ = tx.send(pong(timestamp));
            }
            ClientMessage::LeaveGame
            | ClientMessage::MoveIntent { .. }
            | ClientMessage::AttackTarget { .. }
            | ClientMessage::UseSkill { .. }
            | ClientMessage::PickupItem { .. }
            | ClientMessage::DropItem { .. }
            | ClientMessage::EquipItem { .. }
            | ClientMessage::UnequipItem { .. }
            | ClientMessage::TeleportToWaypoint { .. } => {
                let _ = tx.send(ServerMessage::JoinFailed {
                    reason: "not in a game".into(),
                });
            }
            ClientMessage::Login { .. } | ClientMessage::Disconnect => {
                // Login already handled; Disconnect drained earlier.
            }
        },
        ClientPhase::InGame { room_id, entity_id } => match msg {
            ClientMessage::LeaveGame => {
                let name_for_log = {
                    let room = lobby_lock(lobby).rooms.get(&room_id).cloned();
                    match room {
                        Some(r) => r
                            .lock()
                            .ok()
                            .and_then(|g| g.players.get(&entity_id).map(|p| p.character_name.clone()))
                            .unwrap_or_default(),
                        None => String::new(),
                    }
                };
                remove_player_from_room(lobby, room_id, entity_id, &name_for_log);
                let _ = tx.send(ServerMessage::ReturnedToLobby);
                let new_client_id = {
                    let mut l = lobby_lock(lobby);
                    let id = l.next_client_id;
                    l.next_client_id += 1;
                    l.lobby_clients.insert(
                        id,
                        LobbyClient {
                            character_name: name_for_log,
                            player_id: String::new(),
                            class: ClassType::Soldat,
                            save_data: None,
                            tx: tx.clone(),
                        },
                    );
                    let _ = tx.send(ServerMessage::GameList { games: l.game_list() });
                    id
                };
                *phase = ClientPhase::InLobby { client_id: new_client_id };
            }
            ClientMessage::ListGames | ClientMessage::CreateGame { .. } | ClientMessage::JoinGame { .. } => {
                let _ = tx.send(ServerMessage::JoinFailed {
                    reason: "already in a game".into(),
                });
            }
            ClientMessage::Ping { timestamp } => {
                let _ = tx.send(pong(timestamp));
            }
            other => apply_intent_in_room(lobby, room_id, entity_id, other),
        },
    }
}

fn pong(client_timestamp: u64) -> ServerMessage {
    let server_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    ServerMessage::Pong { client_timestamp, server_timestamp }
}

fn clamp_chat(text: String) -> String {
    text.chars().take(MAX_CHAT_LEN).collect()
}

// ── Lobby actions ───────────────────────────────────────────────────────────

fn validate_create_args(
    name: &str,
    max_players: u32,
    password: Option<&String>,
) -> std::result::Result<(), String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("empty game name".into());
    }
    if trimmed.len() > MAX_GAME_NAME_LEN {
        return Err("game name too long".into());
    }
    if !(2..=(MAX_PLAYERS as u32)).contains(&max_players) {
        return Err(format!("max_players must be in 2..={MAX_PLAYERS}"));
    }
    if let Some(pw) = password {
        if pw.is_empty() {
            return Err("empty password (use null for public)".into());
        }
        if pw.len() > MAX_PASSWORD_LEN {
            return Err("password too long".into());
        }
    }
    Ok(())
}

/// Create a new room and enrol the calling lobby client into it.
/// Returns `(room_id, entity_id, tick)`.
fn create_game(
    lobby: &Arc<Mutex<LobbyServer>>,
    client_id: u64,
    name: String,
    max_players: u32,
    difficulty: Difficulty,
    password: Option<String>,
) -> std::result::Result<(u64, u64, u64), String> {
    validate_create_args(&name, max_players, password.as_ref())?;

    // Cap simultaneous rooms (peek without holding the lobby lock for the
    // whole create flow — the room registration below re-checks under the
    // same lock to keep the cap honest under concurrent creates).
    if lobby_lock(lobby).rooms.len() >= MAX_ROOMS {
        return Err(format!("server full ({MAX_ROOMS} games already hosted)"));
    }

    // Pull the lobby client out (they're moving into the room).
    let client = {
        let mut l = lobby_lock(lobby);
        if l.rooms.len() >= MAX_ROOMS {
            return Err(format!("server full ({MAX_ROOMS} games already hosted)"));
        }
        l.lobby_clients.remove(&client_id).ok_or("not in lobby")?
    };

    // Build a fresh room (its own simulation).
    let seed = mix_seed(client_id, &name);
    let app = crate::build_app(client.class, seed);
    let room_id = {
        let mut l = lobby_lock(lobby);
        let id = l.next_room_id;
        l.next_room_id += 1;
        id
    };
    let host_name = client.character_name.clone();
    let room = Arc::new(Mutex::new(GameRoom {
        id: room_id,
        name: name.trim().to_string(),
        difficulty,
        max_players,
        password,
        host_name,
        persistent: false,
        app,
        players: HashMap::new(),
        tick: 0,
        deaths_pending: VecDeque::new(),
        empty_since: None,
    }));

    // Register the room first so it's visible if anything goes wrong below.
    {
        let mut l = lobby_lock(lobby);
        l.rooms.insert(room_id, room.clone());
    }

    let (entity_id, tick) = enrol_client_into_room(&room, client);

    // Refresh the game list for everyone still in the lobby.
    lobby_lock(lobby).broadcast_lobby_game_list();

    Ok((room_id, entity_id, tick))
}

fn join_game(
    lobby: &Arc<Mutex<LobbyServer>>,
    client_id: u64,
    game_id: u64,
    password: Option<String>,
) -> std::result::Result<(u64, u64), String> {
    // Resolve the room first (without holding lobby lock during enrolment).
    let room = lobby_lock(lobby)
        .rooms
        .get(&game_id)
        .cloned()
        .ok_or("unknown game")?;

    // Capacity + password check.
    {
        let r = room_lock(&room);
        if r.players.len() as u32 >= r.max_players {
            return Err("game full".into());
        }
        match (&r.password, password.as_ref()) {
            (Some(expected), Some(supplied)) if expected == supplied => {}
            (Some(_), _) => return Err("wrong or missing password".into()),
            (None, _) => {}
        }
    }

    let client = {
        let mut l = lobby_lock(lobby);
        l.lobby_clients.remove(&client_id).ok_or("not in lobby")?
    };

    let (entity_id, tick) = enrol_client_into_room(&room, client);
    lobby_lock(lobby).broadcast_lobby_game_list();
    Ok((entity_id, tick))
}

/// Spawn an avatar for `client` inside `room`, register the [`PlayerConn`],
/// and broadcast the join. Returns the entity-id and current room tick.
fn enrol_client_into_room(room: &Arc<Mutex<GameRoom>>, client: LobbyClient) -> (u64, u64) {
    let mut r = room_lock(room);
    r.empty_since = None;
    let entity = spawn_avatar(&mut r.app, client.class);
    if let Some(sd) = &client.save_data {
        let prev = r.app.world.player_entity;
        r.app.world.player_entity = Some(entity);
        save::apply_to_world(&mut r.app.world, sd);
        r.app.world.player_entity = prev;
    }
    let entity_id = entity.0;
    let tick = r.tick;
    let player_id = client.player_id.clone();
    let character_name = client.character_name.clone();
    let class = client.class;
    let class_str = class_label(class).to_string();
    let tx = client.tx.clone();
    r.players.insert(
        entity_id,
        PlayerConn {
            entity,
            player_id,
            character_name: character_name.clone(),
            class,
            tx,
        },
    );
    r.broadcast(ServerMessage::PlayerJoined {
        name: character_name.clone(),
        class: class_str,
    });
    println!(
        "[server] {character_name} joined room {} as #{entity_id} ({} in room)",
        r.name,
        r.players.len()
    );
    (entity_id, tick)
}

fn spawn_avatar(app: &mut crate::app::App, class: ClassType) -> Entity {
    let world = &mut app.world;
    let e = world.spawn();
    let spawn = world.map.player_spawn;
    world.positions.insert(e, Position2D::from_tile(spawn.0, spawn.1));
    world.health.insert(e, Health::full(120.0));
    world.mana.insert(e, Mana::full(60.0));
    world.factions.insert(e, Faction::Player);
    world.player_tags.insert(e, PlayerTag);
    world.classes.insert(e, class);
    world.stats.insert(e, Stats::new(class, 1));
    world.combat.insert(e, Combat::new(1.8, 0.8));
    e
}

fn mix_seed(client_id: u64, name: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    client_id.hash(&mut h);
    name.hash(&mut h);
    h.finish() ^ 0xA11C_E_DEAD_C0DEu64
}

fn class_label(c: ClassType) -> &'static str {
    match c {
        ClassType::Soldat => "Soldat",
        ClassType::Clerc => "Clerc",
        ClassType::MageDeGuerre => "Mage de Guerre",
        ClassType::Archer => "Archer",
    }
}

// ── In-game intents (only valid inside a room) ──────────────────────────────

fn apply_intent_in_room(
    lobby: &Arc<Mutex<LobbyServer>>,
    room_id: u64,
    entity_id: u64,
    msg: ClientMessage,
) {
    let room = match lobby_lock(lobby).rooms.get(&room_id).cloned() {
        Some(r) => r,
        None => return,
    };
    let mut r = room_lock(&room);
    let entity = Entity(entity_id);
    if !r.players.contains_key(&entity_id) {
        return;
    }
    match msg {
        ClientMessage::MoveIntent { target_x, target_y } => {
            apply_move_to(&mut r, entity, target_x, target_y);
        }
        ClientMessage::AttackTarget { entity_id: target_id } => {
            let target = Entity(target_id);
            let valid = r.app.world.positions.contains_key(&target)
                && distance_between(&r, entity, target).is_some_and(|d| d <= MAX_ATTACK_RANGE);
            if valid {
                if let Some(c) = r.app.world.combat.get_mut(&entity) {
                    c.target = Some(target);
                }
            }
        }
        ClientMessage::UseSkill { skill_index, target, cursor_x: _, cursor_y: _ } => {
            try_use_skill(&mut r, entity, skill_index, target);
        }
        ClientMessage::PickupItem { ground_item_id } => {
            try_pickup_item(&mut r, entity, ground_item_id);
        }
        ClientMessage::DropItem { inventory_slot } => {
            try_drop_item(&mut r, entity, inventory_slot);
        }
        ClientMessage::EquipItem { inventory_slot } => {
            try_equip_item(&mut r, entity, inventory_slot);
        }
        ClientMessage::UnequipItem { slot } => {
            try_unequip(&mut r, entity, &slot);
        }
        ClientMessage::ChatMessage { text } => {
            let sender = r
                .players
                .get(&entity_id)
                .map(|p| p.character_name.clone())
                .unwrap_or_default();
            let text = clamp_chat(text);
            r.broadcast(ServerMessage::ChatBroadcast { sender, text });
        }
        ClientMessage::TeleportToWaypoint { wp_id } => {
            let allowed = r
                .app
                .world
                .discovered_wps
                .get(&entity)
                .is_some_and(|d| d.contains(wp_id));
            if allowed {
                crate::systems::waypoint::waypoint_teleport(&mut r.app.world, entity, wp_id);
            }
        }
        _ => {}
    }
}

fn distance_between(r: &GameRoom, a: Entity, b: Entity) -> Option<f32> {
    let pa = r.app.world.positions.get(&a)?;
    let pb = r.app.world.positions.get(&b)?;
    Some(pa.distance(*pb))
}

fn apply_move_to(r: &mut GameRoom, entity: Entity, target_x: f32, target_y: f32) {
    let Some(&pos) = r.app.world.positions.get(&entity) else { return };
    let goal_pos = Position2D::new(target_x, target_y);
    let (gx, gy) = goal_pos.tile();
    if !r.app.world.map.is_walkable(gx, gy) {
        return;
    }
    if let Some(path) = find_path(&r.app.world.map, pos.tile(), (gx, gy)) {
        if !path.is_empty() {
            r.app.world.move_targets.insert(
                entity,
                MoveTarget::new(Position2D::from_tile(gx, gy), path, PLAYER_SPEED),
            );
        }
    }
}

fn try_use_skill(r: &mut GameRoom, entity: Entity, skill_index: u32, target: Option<u64>) {
    let Some(skill_slot) = r.app.world.skills.get(&entity).cloned() else { return };
    let idx = skill_index as usize;
    let Some(skill) = skill_slot.bar.get(idx).cloned() else { return };

    let cd_ready = skill_slot.cooldowns.get(idx).is_some_and(|c| c.is_ready());
    let mana_ok = r.app.world.mana.get(&entity).is_some_and(|m| m.current >= skill.mana_cost);
    if !cd_ready || !mana_ok {
        return;
    }

    if let Some(tid) = target {
        let target_e = Entity(tid);
        let Some(dist) = distance_between(r, entity, target_e) else { return };
        if dist > skill.range {
            return;
        }
    }

    if let Some(slot) = r.app.world.skills.get_mut(&entity) {
        slot.selected = idx;
    }
    if let Some(c) = r.app.world.combat.get_mut(&entity) {
        if let Some(tid) = target {
            c.target = Some(Entity(tid));
        }
    }
    r.app.world.input.command = Some(crate::plugins::player::PlayerCommand::UseSkill {
        idx,
        target: Entity(target.unwrap_or(entity.0)),
    });
}

fn try_pickup_item(r: &mut GameRoom, entity: Entity, ground_item_id: u64) {
    let target = Entity(ground_item_id);
    if !r.app.world.ground_items.contains_key(&target) {
        return;
    }
    if let Some(dist) = distance_between(r, entity, target) {
        if dist > 3.0 {
            return;
        }
    }
    let prev_player = r.app.world.player_entity;
    r.app.world.player_entity = Some(entity);
    r.app.world.input.command = Some(crate::plugins::player::PlayerCommand::Pickup(target));
    let _ = prev_player;
}

fn try_drop_item(r: &mut GameRoom, entity: Entity, slot: u32) {
    let Some(inv) = r.app.world.inventory.get_mut(&entity) else { return };
    let idx = slot as usize;
    if idx >= inv.slots.len() {
        return;
    }
    inv.slots[idx] = None;
}

fn try_equip_item(r: &mut GameRoom, entity: Entity, slot: u32) {
    let idx = slot as usize;
    if !r
        .app
        .world
        .inventory
        .get(&entity)
        .is_some_and(|inv| idx < inv.slots.len())
    {
        return;
    }
    crate::plugins::inventory::equip_from_inventory(&mut r.app.world, entity, idx);
}

fn try_unequip(r: &mut GameRoom, entity: Entity, slot: &str) {
    let Some(eq_slot) = parse_equip_slot(slot) else { return };
    crate::plugins::inventory::unequip_to_inventory(&mut r.app.world, entity, eq_slot);
}

fn parse_equip_slot(s: &str) -> Option<mge_items::item::EquipSlot> {
    use mge_items::item::EquipSlot::*;
    Some(match s {
        "Head" => Head,
        "Chest" => Chest,
        "Belt" => Belt,
        "Gloves" => Gloves,
        "Boots" => Boots,
        "Ring1" => Ring1,
        "Ring2" => Ring2,
        "Amulet" => Amulet,
        "MainHand" => MainHand,
        "OffHand" => OffHand,
        _ => return None,
    })
}

// ── Reader / writer threads ─────────────────────────────────────────────────

fn writer_loop(mut stream: TcpStream, rx: Receiver<ServerMessage>) -> Result<()> {
    while let Ok(msg) = rx.recv() {
        let bytes = protocol::frame(&msg).context("encode")?;
        if stream.write_all(&bytes).is_err() {
            break;
        }
    }
    let _ = stream.shutdown(std::net::Shutdown::Both);
    Ok(())
}

fn read_one_message(stream: &mut TcpStream, buf: &mut Vec<u8>) -> Result<Option<ClientMessage>> {
    let mut chunk = [0u8; 4096];
    loop {
        match protocol::try_unframe::<ClientMessage>(buf) {
            Ok(Some(msg)) => return Ok(Some(msg)),
            Ok(None) => {}
            Err(e) => return Err(anyhow::anyhow!("decode error: {e}")),
        }
        if buf.len() > MAX_FRAME + MAX_LOGIN_PAYLOAD {
            return Err(anyhow::anyhow!("oversized login payload"));
        }
        match stream.read(&mut chunk) {
            Ok(0) => return Ok(None),
            Ok(n) => buf.extend_from_slice(&chunk[..n]),
            Err(e) => return Err(e.into()),
        }
    }
}

// ── Save / checksum helpers (mirror save.rs internals) ─────────────────────

pub(crate) fn verify_save_checksum(data: &SaveData) -> std::result::Result<(), String> {
    let mut clone = data.clone();
    let claimed = std::mem::take(&mut clone.checksum);
    let body = serde_json::to_string(&clone).map_err(|e| format!("re-encode: {e}"))?;
    let recomputed = save_checksum_fn(&body);
    if claimed == recomputed { Ok(()) } else { Err("checksum mismatch".into()) }
}

fn save_checksum_fn(payload: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    "allumina-v1-anti-tamper-salt".hash(&mut hasher);
    payload.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

// ── Lock helpers ────────────────────────────────────────────────────────────

fn lobby_lock(lobby: &Arc<Mutex<LobbyServer>>) -> std::sync::MutexGuard<'_, LobbyServer> {
    match lobby.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    }
}

fn room_lock(room: &Arc<Mutex<GameRoom>>) -> std::sync::MutexGuard<'_, GameRoom> {
    match room.lock() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    }
}

fn short_kind(msg: &ClientMessage) -> &'static str {
    match msg {
        ClientMessage::Login { .. } => "Login",
        ClientMessage::ListGames => "ListGames",
        ClientMessage::CreateGame { .. } => "CreateGame",
        ClientMessage::JoinGame { .. } => "JoinGame",
        ClientMessage::LeaveGame => "LeaveGame",
        ClientMessage::MoveIntent { .. } => "MoveIntent",
        ClientMessage::AttackTarget { .. } => "AttackTarget",
        ClientMessage::UseSkill { .. } => "UseSkill",
        ClientMessage::PickupItem { .. } => "PickupItem",
        ClientMessage::DropItem { .. } => "DropItem",
        ClientMessage::EquipItem { .. } => "EquipItem",
        ClientMessage::UnequipItem { .. } => "UnequipItem",
        ClientMessage::ChatMessage { .. } => "ChatMessage",
        ClientMessage::TeleportToWaypoint { .. } => "TeleportToWaypoint",
        ClientMessage::Ping { .. } => "Ping",
        ClientMessage::Disconnect => "Disconnect",
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    fn fresh_lobby() -> Arc<Mutex<LobbyServer>> {
        Arc::new(Mutex::new(LobbyServer::new()))
    }

    fn enrol_lobby_client(lobby: &Arc<Mutex<LobbyServer>>) -> (u64, Sender<ServerMessage>, Receiver<ServerMessage>) {
        let (tx, rx) = mpsc::channel();
        let mut l = lobby_lock(lobby);
        let id = l.next_client_id;
        l.next_client_id += 1;
        l.lobby_clients.insert(
            id,
            LobbyClient {
                character_name: "Tester".into(),
                player_id: "p-0".into(),
                class: ClassType::Soldat,
                save_data: None,
                tx: tx.clone(),
            },
        );
        (id, tx, rx)
    }

    #[test]
    fn rejects_empty_game_name() {
        assert!(validate_create_args("   ", 4, None).is_err());
    }

    #[test]
    fn rejects_max_players_out_of_range() {
        assert!(validate_create_args("Ok", 1, None).is_err());
        assert!(validate_create_args("Ok", 9, None).is_err());
        assert!(validate_create_args("Ok", 2, None).is_ok());
        assert!(validate_create_args("Ok", MAX_PLAYERS as u32, None).is_ok());
    }

    #[test]
    fn rejects_empty_password() {
        let pw = String::new();
        assert!(validate_create_args("Ok", 4, Some(&pw)).is_err());
    }

    #[test]
    fn rejects_tampered_save_login() {
        // Build a valid save snapshot then tamper a field without rewriting the
        // checksum so the validator should refuse the login.
        let app = crate::build_app(ClassType::Soldat, 0xA11CE);
        let world = &app.world;
        let mut data = save::snapshot(world, "Tester", "p-0", 0).expect("snapshot");
        data.gold = 999_999;
        assert!(validate_login("Tester", Some(&data)).is_err());
    }

    #[test]
    fn create_then_list_returns_room() {
        let lobby = fresh_lobby();
        let (client_id, _tx, _rx) = enrol_lobby_client(&lobby);
        let (room_id, _entity_id, _tick) =
            create_game(&lobby, client_id, "Mon Run".into(), 4, Difficulty::Normal, None)
                .expect("create");
        let list = lobby_lock(&lobby).game_list();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, room_id);
        assert_eq!(list[0].name, "Mon Run");
        assert!(!list[0].has_password);
        assert_eq!(list[0].players, 1);
    }

    #[test]
    fn join_rejects_wrong_password() {
        let lobby = fresh_lobby();
        // Creator with a private game.
        let (creator_id, _tx_c, _rx_c) = enrol_lobby_client(&lobby);
        let (room_id, _e1, _t1) = create_game(
            &lobby,
            creator_id,
            "Privé".into(),
            4,
            Difficulty::Cauchemar,
            Some("secret".into()),
        )
        .expect("create");

        // Joiner.
        let (joiner_id, _tx_j, _rx_j) = enrol_lobby_client(&lobby);
        let err = join_game(&lobby, joiner_id, room_id, Some("wrong".into()));
        assert!(matches!(err, Err(ref r) if r.contains("password")));
    }

    #[test]
    fn join_accepts_correct_password_and_fills_room() {
        let lobby = fresh_lobby();
        let (creator_id, _tx_c, _rx_c) = enrol_lobby_client(&lobby);
        let (room_id, _e1, _t1) = create_game(
            &lobby,
            creator_id,
            "Privé".into(),
            2,
            Difficulty::Normal,
            Some("secret".into()),
        )
        .expect("create");

        let (joiner_id, _tx_j, _rx_j) = enrol_lobby_client(&lobby);
        let (_e2, _t2) = join_game(&lobby, joiner_id, room_id, Some("secret".into())).expect("join");

        // Room now full (2/2): a third attempt should fail capacity.
        let (third_id, _tx_3, _rx_3) = enrol_lobby_client(&lobby);
        let res = join_game(&lobby, third_id, room_id, Some("secret".into()));
        assert!(matches!(res, Err(ref r) if r.contains("full")), "got {res:?}");
    }

    #[test]
    fn list_games_includes_private_with_lock_flag() {
        let lobby = fresh_lobby();
        let (a, _, _) = enrol_lobby_client(&lobby);
        let _ = create_game(&lobby, a, "Pub".into(), 4, Difficulty::Normal, None).unwrap();
        let (b, _, _) = enrol_lobby_client(&lobby);
        let _ = create_game(&lobby, b, "Privé".into(), 4, Difficulty::Enfer, Some("pw".into())).unwrap();
        let list = lobby_lock(&lobby).game_list();
        assert_eq!(list.len(), 2);
        assert!(list.iter().any(|g| g.has_password && g.difficulty == Difficulty::Enfer));
        assert!(list.iter().any(|g| !g.has_password));
    }

    #[test]
    fn unknown_game_id_rejected() {
        let lobby = fresh_lobby();
        let (client, _, _) = enrol_lobby_client(&lobby);
        let res = join_game(&lobby, client, 999, None);
        assert!(matches!(res, Err(ref r) if r.contains("unknown")));
    }

    #[test]
    fn create_game_emits_game_list_broadcast_to_lobby_peers() {
        let lobby = fresh_lobby();
        // Bystander who stays in the lobby.
        let (_observer, _tx_o, rx_o) = enrol_lobby_client(&lobby);
        // Creator (moves into the new room).
        let (creator, _tx_c, _rx_c) = enrol_lobby_client(&lobby);
        let _ = create_game(&lobby, creator, "Run".into(), 4, Difficulty::Normal, None).unwrap();
        // Observer should have received an updated GameList.
        let mut saw_list = false;
        while let Ok(msg) = rx_o.try_recv() {
            if matches!(msg, ServerMessage::GameList { .. }) {
                saw_list = true;
            }
        }
        assert!(saw_list, "expected GameList broadcast to lobby observer");
    }

    #[test]
    fn out_of_range_attack_intent_is_ignored() {
        let lobby = fresh_lobby();
        let (client, _tx, _rx) = enrol_lobby_client(&lobby);
        let (room_id, entity_id, _tick) =
            create_game(&lobby, client, "Solo".into(), 4, Difficulty::Normal, None).unwrap();

        // Plant a far-away dummy target.
        let victim_id = {
            let room = lobby_lock(&lobby).rooms.get(&room_id).cloned().unwrap();
            let mut r = room_lock(&room);
            let v = r.app.world.spawn();
            r.app.world.positions.insert(v, Position2D::new(1_000.0, 1_000.0));
            r.app.world.health.insert(v, Health::full(10.0));
            v.0
        };

        apply_intent_in_room(
            &lobby,
            room_id,
            entity_id,
            ClientMessage::AttackTarget { entity_id: victim_id },
        );

        let room = lobby_lock(&lobby).rooms.get(&room_id).cloned().unwrap();
        let r = room_lock(&room);
        let combat = r.app.world.combat.get(&Entity(entity_id)).expect("combat");
        assert!(combat.target.is_none(), "out-of-range target should be rejected");
    }

    #[test]
    fn empty_room_enters_grace_window_then_sweeps() {
        let lobby = fresh_lobby();
        let (client, _tx, _rx) = enrol_lobby_client(&lobby);
        let (room_id, entity_id, _tick) =
            create_game(&lobby, client, "Solo".into(), 4, Difficulty::Normal, None).unwrap();
        assert_eq!(lobby_lock(&lobby).rooms.len(), 1);
        remove_player_from_room(&lobby, room_id, entity_id, "Tester");
        // Still alive — grace period hasn't elapsed.
        assert_eq!(lobby_lock(&lobby).rooms.len(), 1);
        // Sweep with the room's empty_since artificially aged past the grace
        // threshold.
        {
            let room = lobby_lock(&lobby).rooms.get(&room_id).cloned().unwrap();
            let mut r = room_lock(&room);
            r.empty_since = Some(Instant::now() - Duration::from_secs_f32(ROOM_GRACE_SECS + 1.0));
        }
        sweep_expired_rooms(&lobby);
        assert_eq!(lobby_lock(&lobby).rooms.len(), 0);
    }

    #[test]
    fn rejoin_during_grace_window_revives_room() {
        let lobby = fresh_lobby();
        let (host, _, _) = enrol_lobby_client(&lobby);
        let (room_id, entity_id, _) =
            create_game(&lobby, host, "Persistent".into(), 4, Difficulty::Normal, None).unwrap();
        remove_player_from_room(&lobby, room_id, entity_id, "Tester");
        // Grace timer is set.
        {
            let room = lobby_lock(&lobby).rooms.get(&room_id).cloned().unwrap();
            assert!(room_lock(&room).empty_since.is_some());
        }
        // A new client jumps in before the sweep — empty_since must clear.
        let (joiner, _, _) = enrol_lobby_client(&lobby);
        let (_e, _t) = join_game(&lobby, joiner, room_id, None).expect("rejoin");
        let room = lobby_lock(&lobby).rooms.get(&room_id).cloned().unwrap();
        assert!(room_lock(&room).empty_since.is_none());
    }

    #[test]
    fn server_caps_simultaneous_rooms() {
        let lobby = fresh_lobby();
        for i in 0..MAX_ROOMS {
            let (id, _, _) = enrol_lobby_client(&lobby);
            create_game(&lobby, id, format!("R{i}"), 4, Difficulty::Normal, None)
                .unwrap_or_else(|e| panic!("room {i}: {e}"));
        }
        let (extra, _, _) = enrol_lobby_client(&lobby);
        let err = create_game(&lobby, extra, "Overflow".into(), 4, Difficulty::Normal, None);
        assert!(matches!(err, Err(ref r) if r.contains("full")), "got {err:?}");
    }
}
