//! TCP client for the authoritative Allumina server (with lobby flow).
//!
//! Connection life-cycle:
//! 1. Connect to `<host>:<port>` (defaulting to [`super::DEFAULT_SERVER`] /
//!    [`super::DEFAULT_PORT`]).
//! 2. Send [`ClientMessage::Login`]. On [`ServerMessage::LoginAccepted`] we
//!    enter the **lobby phase** (no avatar yet).
//! 3. Lobby phase: list / create / join games. The headless demo here
//!    auto-creates a public room named "Demo Room" so the full stack stays
//!    exercised without a UI.
//! 4. On [`ServerMessage::JoinedGame`] we transition to the **in-game phase**:
//!    we start receiving world snapshots, drive movement via the
//!    [`Predictor`], and the server applies our intents.
//!
//! The real interactive client (apps/central + dioxus) drives the same
//! transitions but with a UI in place of the demo loop.

use std::collections::HashMap;
use std::io::Read;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};

use crate::components::player::ClassType;
use crate::save::{generate_player_id, list_saves, load_from_path};

use super::interpolation::{InterpBuffer, INTERP_DELAY};
use super::prediction::Predictor;
use super::protocol::{
    self, ClientMessage, Difficulty, EntityKind, GameInfo, ServerMessage,
};
use super::{DEFAULT_PORT, DEFAULT_SERVER};

/// Tile-units/sec used by client-side prediction.
const PLAYER_SPEED: f32 = 4.5;
/// Distance threshold (tiles) above which authoritative position snaps the
/// client; below which it lerps softly.
const RECONCILE_SNAP: f32 = 0.5;
/// Lerp factor used for the soft correction case.
const RECONCILE_LERP: f32 = 0.25;

const INPUT_HZ: f32 = 60.0;
const PING_INTERVAL: Duration = Duration::from_secs(5);
const STATUS_INTERVAL: Duration = Duration::from_secs(1);
/// Name used by the headless demo when it auto-creates a room.
const DEMO_ROOM_NAME: &str = "Demo Room";

/// Local phase of the connection. Mirrors the server-side `ClientPhase` but is
/// only consulted by the headless demo loop and by the message dispatcher to
/// decide how to react to incoming messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClientPhase {
    Authenticating,
    InLobby,
    InGame,
}

/// Blocking entry point invoked from `main.rs`.
pub fn run_client(host: String, port: u16, class: ClassType) -> Result<()> {
    let host = if host.trim().is_empty() { DEFAULT_SERVER.to_string() } else { host };
    let port = if port == 0 { DEFAULT_PORT } else { port };

    // Try to load the freshest save for this class so we can hand it to the
    // server at login; ignore failures (saves are optional).
    let save_data = list_saves()
        .into_iter()
        .find(|s| s.class == class)
        .and_then(|s| load_from_path(&s.path).ok());
    let character_name = save_data
        .as_ref()
        .map(|s| s.character_name.clone())
        .unwrap_or_else(|| "Wanderer".to_string());
    let player_id = save_data
        .as_ref()
        .map(|s| s.player_id.clone())
        .unwrap_or_else(generate_player_id);

    let addr = (host.as_str(), port)
        .to_socket_addrs()
        .with_context(|| format!("resolve {host}:{port}"))?
        .next()
        .ok_or_else(|| anyhow::anyhow!("no address resolved for {host}:{port}"))?;

    println!("[client] connecting to {host}:{port} ({addr})...");
    let stream = TcpStream::connect_timeout(&addr, Duration::from_secs(10))
        .with_context(|| format!("connect {addr}"))?;
    stream.set_nodelay(true).ok();
    println!("[client] connected.");

    // Reader half — separate thread, parses frames into a channel.
    let (tx_in, rx_in): (Sender<ServerMessage>, Receiver<ServerMessage>) = mpsc::channel();
    let reader_stream = stream.try_clone().context("clone tcp stream")?;
    let reader_handle = std::thread::spawn(move || reader_thread(reader_stream, tx_in));

    let mut writer = stream;

    // Handshake.
    let login = ClientMessage::Login {
        player_id: player_id.clone(),
        character_name: character_name.clone(),
        class,
        save_data,
    };
    protocol::write_message(&mut writer, &login)?;
    println!("[client] login sent as {character_name} ({class:?})");

    // Local state.
    let mut phase = ClientPhase::Authenticating;
    let mut self_id: Option<u64> = None;
    let mut current_room: Option<u64> = None;
    let mut predictor: Option<Predictor> = None;
    let mut remotes: HashMap<u64, InterpBuffer> = HashMap::new();
    let mut last_games: Vec<GameInfo> = Vec::new();
    let mut last_latency_ms: Option<u64> = None;

    // Demo timers.
    let start = Instant::now();
    let input_period = Duration::from_secs_f32(1.0 / INPUT_HZ);
    let mut next_input = Instant::now() + input_period;
    let mut next_ping = Instant::now() + PING_INTERVAL;
    let mut next_status = Instant::now() + STATUS_INTERVAL;
    // One-shot guard so the demo doesn't spam ListGames/CreateGame.
    let mut demo_kickoff_done = false;
    let mut orbit = 0.0_f32;

    loop {
        // Drain server messages (non-blocking).
        loop {
            match rx_in.try_recv() {
                Ok(msg) => {
                    let outcome = handle_server_msg(
                        msg,
                        &mut phase,
                        &mut self_id,
                        &mut current_room,
                        &mut predictor,
                        &mut remotes,
                        &mut last_games,
                        &mut last_latency_ms,
                        start,
                    );
                    match outcome {
                        Outcome::Continue => {}
                        Outcome::Disconnect => {
                            let _ = writer.shutdown(std::net::Shutdown::Both);
                            let _ = reader_handle.join();
                            return Ok(());
                        }
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    println!("[client] connection closed by server");
                    return Ok(());
                }
            }
        }

        // Demo lobby flow: as soon as we hit the lobby, ask for the list, then
        // create a public demo room.
        if phase == ClientPhase::InLobby && !demo_kickoff_done {
            let _ = protocol::write_message(&mut writer, &ClientMessage::ListGames);
            // Reuse an existing demo room if one is already listed; otherwise
            // create a fresh one.
            if let Some(existing) = last_games.iter().find(|g| g.name == DEMO_ROOM_NAME && !g.has_password && g.players < g.max_players) {
                println!("[client] joining existing demo room #{}", existing.id);
                let _ = protocol::write_message(
                    &mut writer,
                    &ClientMessage::JoinGame { game_id: existing.id, password: None },
                );
            } else {
                println!("[client] creating public demo room ({DEMO_ROOM_NAME})");
                let _ = protocol::write_message(
                    &mut writer,
                    &ClientMessage::CreateGame {
                        name: DEMO_ROOM_NAME.into(),
                        max_players: 4,
                        difficulty: Difficulty::Normal,
                        password: None,
                    },
                );
            }
            demo_kickoff_done = true;
        }

        let now = Instant::now();

        // Movement demo runs only in-game.
        if phase == ClientPhase::InGame && now >= next_input {
            if let Some(pred) = predictor.as_mut() {
                orbit += 1.0 / INPUT_HZ;
                let radius = 2.0_f32;
                let target_x = pred.pos[0] + orbit.cos() * radius * 0.05;
                let target_y = pred.pos[1] + orbit.sin() * radius * 0.05;
                let dx = target_x - pred.pos[0];
                let dy = target_y - pred.pos[1];
                let dt = 1.0 / INPUT_HZ;
                pred.push_input(dx, dy, dt);
                let msg = ClientMessage::MoveIntent { target_x, target_y };
                if protocol::write_message(&mut writer, &msg).is_err() {
                    break;
                }
            }
            next_input += input_period;
            if next_input < now {
                next_input = now + input_period;
            }
        }

        if now >= next_ping {
            let ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);
            let _ = protocol::write_message(&mut writer, &ClientMessage::Ping { timestamp: ts });
            next_ping += PING_INTERVAL;
        }

        if now >= next_status {
            let lat = last_latency_ms.map(|m| format!("{m} ms")).unwrap_or_else(|| "?".into());
            match phase {
                ClientPhase::Authenticating => println!("[client] authenticating… latency={lat}"),
                ClientPhase::InLobby => println!(
                    "[client] in lobby — {} game(s) listed, latency={lat}",
                    last_games.len()
                ),
                ClientPhase::InGame => {
                    if let Some(pred) = &predictor {
                        let room = current_room.map(|r| r.to_string()).unwrap_or_else(|| "?".into());
                        println!(
                            "[client] in game #{room} — pos=({:.2},{:.2}) pending={} remotes={} latency={lat}",
                            pred.pos[0],
                            pred.pos[1],
                            pred.pending_len(),
                            remotes.len()
                        );
                    }
                }
            }
            next_status += STATUS_INTERVAL;
        }

        std::thread::sleep(Duration::from_millis(2));
    }
    let _ = current_room;
    let _ = self_id;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Continue,
    Disconnect,
}

#[allow(clippy::too_many_arguments)]
fn handle_server_msg(
    msg: ServerMessage,
    phase: &mut ClientPhase,
    self_id: &mut Option<u64>,
    current_room: &mut Option<u64>,
    predictor: &mut Option<Predictor>,
    remotes: &mut HashMap<u64, InterpBuffer>,
    last_games: &mut Vec<GameInfo>,
    last_latency_ms: &mut Option<u64>,
    start: Instant,
) -> Outcome {
    let now_s = start.elapsed().as_secs_f64();
    match msg {
        // ── Lobby / authentication ──
        ServerMessage::LoginAccepted { your_entity_id, tick } => {
            println!("[client] login accepted (entity {your_entity_id}, server tick {tick}) — welcome to the lobby");
            *phase = ClientPhase::InLobby;
        }
        ServerMessage::LoginRejected { reason } => {
            eprintln!("[client] login rejected: {reason}");
            return Outcome::Disconnect;
        }
        ServerMessage::GameList { games } => {
            print_game_list(&games);
            *last_games = games;
        }
        ServerMessage::GameCreated { game_id } => {
            println!("[client] game #{game_id} created");
        }
        ServerMessage::JoinedGame { game_id, your_entity_id, tick } => {
            println!("[client] joined game #{game_id} as entity #{your_entity_id} (tick {tick})");
            *current_room = Some(game_id);
            *self_id = Some(your_entity_id);
            *phase = ClientPhase::InGame;
            // Predictor is initialised by the first snapshot for our entity.
        }
        ServerMessage::JoinFailed { reason } => {
            eprintln!("[client] join/create rejected: {reason}");
        }
        ServerMessage::ReturnedToLobby => {
            println!("[client] back in the lobby");
            *phase = ClientPhase::InLobby;
            *current_room = None;
            *self_id = None;
            *predictor = None;
            remotes.clear();
        }
        ServerMessage::GameClosed { game_id } => {
            last_games.retain(|g| g.id != game_id);
            println!("[client] game #{game_id} closed");
        }

        // ── In-game ──
        ServerMessage::WorldSnapshot { tick: _, entities, ground_items: _, effects: _ } => {
            for snap in entities {
                if Some(snap.id) == *self_id {
                    let auth = [snap.x, snap.y];
                    match predictor.as_mut() {
                        Some(pred) => reconcile_position(pred, auth),
                        None => *predictor = Some(Predictor::new(auth, PLAYER_SPEED)),
                    }
                } else if snap.kind != EntityKind::Npc {
                    remotes.entry(snap.id).or_default().push(now_s, [snap.x, snap.y]);
                }
            }
        }
        ServerMessage::EntityDied { entity_id } => {
            remotes.remove(&entity_id);
        }
        ServerMessage::EntitySpawned { snapshot } => {
            remotes.entry(snapshot.id).or_default().push(now_s, [snapshot.x, snapshot.y]);
        }
        ServerMessage::LootDropped { item } => {
            println!("[client] loot dropped: {} at ({:.1},{:.1})", item.name, item.x, item.y);
        }
        ServerMessage::DamageDealt { target, amount, crit } => {
            if crit {
                println!("[client] *CRIT* {amount:.0} dmg to #{target}");
            }
        }
        ServerMessage::SkillUsed { caster, skill_id, .. } => {
            println!("[client] #{caster} cast skill {skill_id}");
        }
        ServerMessage::InventoryUpdate { slots } => {
            println!("[client] inventory update — {} slot(s) changed", slots.len());
        }
        ServerMessage::EquipmentUpdate { slot, item } => match item {
            Some(it) => println!("[client] equipped {} in {slot}", it.name),
            None => println!("[client] {slot} unequipped"),
        },
        ServerMessage::GoldUpdate { amount } => {
            println!("[client] gold = {amount}");
        }
        ServerMessage::ChatBroadcast { sender, text } => {
            println!("[chat] {sender}: {text}");
        }
        ServerMessage::PlayerJoined { name, class } => {
            println!("[client] {name} ({class}) joined the room");
        }
        ServerMessage::PlayerLeft { name } => {
            println!("[client] {name} left the room");
        }
        ServerMessage::WaypointDiscovered { player_name, wp_name, wp_id: _ } => {
            println!("[client] {player_name} a découvert le waypoint {wp_name}");
        }
        ServerMessage::Pong { client_timestamp, server_timestamp: _ } => {
            let now_ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);
            *last_latency_ms = Some(now_ms.saturating_sub(client_timestamp));
        }
        ServerMessage::ServerShutdown { reason } => {
            eprintln!("[client] server shutdown: {reason}");
            return Outcome::Disconnect;
        }
    }
    let _ = INTERP_DELAY;
    Outcome::Continue
}

fn print_game_list(games: &[GameInfo]) {
    if games.is_empty() {
        println!("[lobby] no games available");
        return;
    }
    println!("[lobby] {} game(s):", games.len());
    for g in games {
        let lock = if g.has_password { "🔒" } else { "  " };
        println!(
            "  {lock} #{:<3} \"{}\" by {} — {}/{} — {} — lvl {:.1}",
            g.id,
            g.name,
            g.host_name,
            g.players,
            g.max_players,
            g.difficulty.label(),
            g.avg_level
        );
    }
}

/// Snap if the authoritative position is far from prediction, otherwise lerp.
fn reconcile_position(pred: &mut Predictor, auth: [f32; 2]) {
    let dx = auth[0] - pred.pos[0];
    let dy = auth[1] - pred.pos[1];
    let dist = (dx * dx + dy * dy).sqrt();
    if dist > RECONCILE_SNAP {
        pred.pos = auth;
    } else {
        pred.pos[0] += dx * RECONCILE_LERP;
        pred.pos[1] += dy * RECONCILE_LERP;
    }
}

fn reader_thread(mut stream: TcpStream, tx: Sender<ServerMessage>) {
    let mut buf: Vec<u8> = Vec::with_capacity(8192);
    let mut chunk = [0u8; 4096];
    loop {
        loop {
            match protocol::try_unframe::<ServerMessage>(&mut buf) {
                Ok(Some(msg)) => {
                    if tx.send(msg).is_err() {
                        return;
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    eprintln!("[client/reader] decode error: {e}");
                    return;
                }
            }
        }
        match stream.read(&mut chunk) {
            Ok(0) => return,
            Ok(n) => buf.extend_from_slice(&chunk[..n]),
            Err(e) => {
                eprintln!("[client/reader] read error: {e}");
                return;
            }
        }
    }
}

/// Sample a remote entity's interpolated position (for a future renderer).
#[must_use]
pub fn sample_remote(remotes: &HashMap<u64, InterpBuffer>, id: u64, now: f64) -> Option<[f32; 2]> {
    remotes.get(&id).and_then(|b| b.sample(now, INTERP_DELAY))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconcile_snaps_when_divergent() {
        let mut pred = Predictor::new([0.0, 0.0], PLAYER_SPEED);
        reconcile_position(&mut pred, [10.0, 0.0]);
        assert!((pred.pos[0] - 10.0).abs() < 1e-4);
    }

    #[test]
    fn reconcile_lerps_when_close() {
        let mut pred = Predictor::new([0.0, 0.0], PLAYER_SPEED);
        reconcile_position(&mut pred, [0.2, 0.0]);
        // Lerp factor 0.25 → moved a quarter of the way.
        assert!((pred.pos[0] - 0.05).abs() < 1e-4, "got {}", pred.pos[0]);
    }

    #[test]
    fn login_accepted_moves_to_lobby() {
        let mut phase = ClientPhase::Authenticating;
        let mut self_id = None;
        let mut room = None;
        let mut pred = None;
        let mut remotes = HashMap::new();
        let mut games = Vec::new();
        let mut lat = None;
        let outcome = handle_server_msg(
            ServerMessage::LoginAccepted { your_entity_id: 0, tick: 0 },
            &mut phase,
            &mut self_id,
            &mut room,
            &mut pred,
            &mut remotes,
            &mut games,
            &mut lat,
            Instant::now(),
        );
        assert_eq!(outcome, Outcome::Continue);
        assert_eq!(phase, ClientPhase::InLobby);
    }

    #[test]
    fn joined_game_moves_to_in_game() {
        let mut phase = ClientPhase::InLobby;
        let mut self_id = None;
        let mut room = None;
        let mut pred = None;
        let mut remotes = HashMap::new();
        let mut games = Vec::new();
        let mut lat = None;
        let _ = handle_server_msg(
            ServerMessage::JoinedGame { game_id: 9, your_entity_id: 5, tick: 0 },
            &mut phase,
            &mut self_id,
            &mut room,
            &mut pred,
            &mut remotes,
            &mut games,
            &mut lat,
            Instant::now(),
        );
        assert_eq!(phase, ClientPhase::InGame);
        assert_eq!(room, Some(9));
        assert_eq!(self_id, Some(5));
    }

    #[test]
    fn returned_to_lobby_clears_in_game_state() {
        let mut phase = ClientPhase::InGame;
        let mut self_id = Some(7u64);
        let mut room = Some(3u64);
        let mut pred = Some(Predictor::new([1.0, 1.0], PLAYER_SPEED));
        let mut remotes: HashMap<u64, InterpBuffer> = HashMap::new();
        remotes.insert(99, InterpBuffer::new());
        let mut games = Vec::new();
        let mut lat = None;
        let _ = handle_server_msg(
            ServerMessage::ReturnedToLobby,
            &mut phase,
            &mut self_id,
            &mut room,
            &mut pred,
            &mut remotes,
            &mut games,
            &mut lat,
            Instant::now(),
        );
        assert_eq!(phase, ClientPhase::InLobby);
        assert_eq!(room, None);
        assert_eq!(self_id, None);
        assert!(pred.is_none());
        assert!(remotes.is_empty());
    }

    #[test]
    fn game_closed_prunes_local_list() {
        let mut phase = ClientPhase::InLobby;
        let mut self_id = None;
        let mut room = None;
        let mut pred = None;
        let mut remotes = HashMap::new();
        let mut games = vec![
            GameInfo {
                id: 1,
                name: "A".into(),
                players: 1,
                max_players: 4,
                difficulty: Difficulty::Normal,
                has_password: false,
                avg_level: 1.0,
                host_name: "host".into(),
            },
            GameInfo {
                id: 2,
                name: "B".into(),
                players: 1,
                max_players: 4,
                difficulty: Difficulty::Normal,
                has_password: false,
                avg_level: 1.0,
                host_name: "host".into(),
            },
        ];
        let mut lat = None;
        let _ = handle_server_msg(
            ServerMessage::GameClosed { game_id: 1 },
            &mut phase,
            &mut self_id,
            &mut room,
            &mut pred,
            &mut remotes,
            &mut games,
            &mut lat,
            Instant::now(),
        );
        assert_eq!(games.len(), 1);
        assert_eq!(games[0].id, 2);
    }
}
