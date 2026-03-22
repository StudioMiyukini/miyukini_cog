use std::collections::HashMap;

use mge_core::{EngineRuntime, RuntimeConfig, RuntimeStage, SceneSummary};
use mge_net::NetMode;
use mge_proto::{ClientCommand, ServerEvent, PROTOCOL_VERSION};

// ---------------------------------------------------------------------------
// Server bootstrap config
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ServerBootstrap {
    pub net_mode: NetMode,
    pub protocol_version: u32,
    pub max_players: u32,
    pub lobby_name: String,
}

impl Default for ServerBootstrap {
    fn default() -> Self {
        Self {
            net_mode: NetMode::OfflineLocal,
            protocol_version: PROTOCOL_VERSION,
            max_players: 8,
            lobby_name: String::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Connected player session
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PlayerSession {
    pub player_id: u64,
    pub name: String,
    pub zone_id: u32,
    pub pos: [f32; 2],
    pub hp_frac: f32,
    pub mp_frac: f32,
    pub level: u32,
    pub facing_right: bool,
    pub running: bool,
    pub active_skill: u8,
    pub last_input_tick: u64,
}

impl PlayerSession {
    pub fn new(player_id: u64, name: String) -> Self {
        Self {
            player_id,
            name,
            zone_id: 0,
            pos: [12.0, 12.0],
            hp_frac: 1.0,
            mp_frac: 1.0,
            level: 1,
            facing_right: true,
            running: false,
            active_skill: 0,
            last_input_tick: 0,
        }
    }

    /// Build a PlayerSync event for broadcasting this player's state.
    pub fn to_sync_event(&self) -> ServerEvent {
        ServerEvent::PlayerSync {
            player_id: self.player_id,
            pos: self.pos,
            hp_frac: self.hp_frac,
            mp_frac: self.mp_frac,
            level: self.level,
            facing_right: self.facing_right,
            running: self.running,
            active_skill: self.active_skill,
        }
    }
}

// ---------------------------------------------------------------------------
// Authoritative simulation with session management
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct AuthoritativeSim {
    bootstrap: ServerBootstrap,
    runtime: EngineRuntime,
    /// Connected player sessions keyed by player_id.
    sessions: HashMap<u64, PlayerSession>,
    /// Next player ID to assign.
    next_player_id: u64,
    /// Outbound events queued this tick for broadcast.
    outbox: Vec<(u64, ServerEvent)>,
}

impl AuthoritativeSim {
    pub fn new(config: RuntimeConfig, scene: SceneSummary) -> Self {
        let mut runtime = EngineRuntime::new(config, scene);
        runtime.enter_stage(RuntimeStage::Boot);
        Self {
            bootstrap: ServerBootstrap::default(),
            runtime,
            sessions: HashMap::new(),
            next_player_id: 1,
            outbox: Vec::new(),
        }
    }

    pub fn bootstrap(&self) -> &ServerBootstrap {
        &self.bootstrap
    }

    pub fn set_bootstrap(&mut self, bootstrap: ServerBootstrap) {
        self.bootstrap = bootstrap;
    }

    pub fn runtime(&self) -> &EngineRuntime {
        &self.runtime
    }

    pub fn sessions(&self) -> &HashMap<u64, PlayerSession> {
        &self.sessions
    }

    pub fn player_count(&self) -> usize {
        self.sessions.len()
    }

    pub fn is_full(&self) -> bool {
        self.sessions.len() >= self.bootstrap.max_players as usize
    }

    // -----------------------------------------------------------------------
    // Player join / leave
    // -----------------------------------------------------------------------

    /// Register a new player. Returns the assigned player_id, or None if full.
    pub fn player_join(&mut self, name: String) -> Option<u64> {
        if self.is_full() {
            return None;
        }
        let id = self.next_player_id;
        self.next_player_id += 1;
        let session = PlayerSession::new(id, name.clone());
        let zone_id = session.zone_id;
        self.sessions.insert(id, session);

        // Notify all existing players in the same zone
        let join_event = ServerEvent::PlayerJoined {
            player_id: id,
            name,
            level: 1,
        };
        for (&pid, s) in &self.sessions {
            if pid != id && s.zone_id == zone_id {
                self.outbox.push((pid, join_event.clone()));
            }
        }
        Some(id)
    }

    /// Remove a player session.
    pub fn player_leave(&mut self, player_id: u64) {
        if let Some(session) = self.sessions.remove(&player_id) {
            let leave_event = ServerEvent::PlayerLeft { player_id };
            for (&pid, s) in &self.sessions {
                if s.zone_id == session.zone_id {
                    self.outbox.push((pid, leave_event.clone()));
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Command processing
    // -----------------------------------------------------------------------

    /// Process a client command from a specific player.
    pub fn handle_command(&mut self, player_id: u64, cmd: ClientCommand) {
        let tick = self.runtime.tick_index();
        match cmd {
            ClientCommand::Move { target, running } => {
                if let Some(s) = self.sessions.get_mut(&player_id) {
                    s.pos = target;
                    s.running = running;
                    s.last_input_tick = tick;
                }
            }
            ClientCommand::Attack { enemy_index } => {
                if let Some(s) = self.sessions.get(&player_id) {
                    let zone = s.zone_id;
                    let event = ServerEvent::EnemyDamaged {
                        enemy_index,
                        new_hp: 0.0, // actual HP resolved by sim_tick
                        attacker_id: player_id,
                    };
                    let targets: Vec<u64> = self.sessions.iter()
                        .filter(|(&pid, s)| pid != player_id && s.zone_id == zone)
                        .map(|(&pid, _)| pid)
                        .collect();
                    for pid in targets {
                        self.outbox.push((pid, event.clone()));
                    }
                }
            }
            ClientCommand::Chat { message } => {
                if let Some(s) = self.sessions.get(&player_id) {
                    let zone = s.zone_id;
                    let name = s.name.clone();
                    let event = ServerEvent::ChatMessage {
                        player_id,
                        name,
                        message,
                    };
                    let targets: Vec<u64> = self.sessions.iter()
                        .filter(|(&pid, s)| pid != player_id && s.zone_id == zone)
                        .map(|(&pid, _)| pid)
                        .collect();
                    for pid in targets {
                        self.outbox.push((pid, event.clone()));
                    }
                }
            }
            ClientCommand::RequestZoneTransition { to_zone_id, .. } => {
                if let Some(s) = self.sessions.get_mut(&player_id) {
                    let old_zone = s.zone_id;
                    s.zone_id = to_zone_id;
                    // Notify old zone
                    let leave_ev = ServerEvent::PlayerLeft { player_id };
                    let targets: Vec<u64> = self.sessions.iter()
                        .filter(|(&pid, s)| pid != player_id && s.zone_id == old_zone)
                        .map(|(&pid, _)| pid)
                        .collect();
                    for pid in targets {
                        self.outbox.push((pid, leave_ev.clone()));
                    }
                    // Notify new zone
                    let name = s.name.clone();
                    let level = s.level;
                    let join_ev = ServerEvent::PlayerJoined {
                        player_id,
                        name,
                        level,
                    };
                    let new_zone_targets: Vec<u64> = self.sessions.iter()
                        .filter(|(&pid, s)| pid != player_id && s.zone_id == to_zone_id)
                        .map(|(&pid, _)| pid)
                        .collect();
                    for pid in new_zone_targets {
                        self.outbox.push((pid, join_ev.clone()));
                    }
                    self.outbox.push((player_id, ServerEvent::ZoneTransitionComplete { zone_id: to_zone_id }));
                }
            }
            // Other commands: handled by the game loop / sim_tick
            _ => {}
        }
    }

    // -----------------------------------------------------------------------
    // Tick + broadcast
    // -----------------------------------------------------------------------

    /// Run one simulation tick. Returns the tick event + queues player syncs.
    pub fn tick(&mut self) -> ServerEvent {
        self.runtime.tick();
        let tick = self.runtime.tick_index();

        // Broadcast all player positions every 3 ticks (~20Hz at 60fps)
        if tick % 3 == 0 {
            let syncs: Vec<(u32, ServerEvent)> = self.sessions.values()
                .map(|s| (s.zone_id, s.to_sync_event()))
                .collect();
            for (zone, event) in syncs {
                let targets: Vec<u64> = self.sessions.iter()
                    .filter(|(_, s)| s.zone_id == zone)
                    .map(|(&pid, _)| pid)
                    .collect();
                for pid in targets {
                    self.outbox.push((pid, event.clone()));
                }
            }
        }

        ServerEvent::TickAcknowledged { tick }
    }

    /// Drain all outbound events (for sending over network).
    pub fn drain_outbox(&mut self) -> Vec<(u64, ServerEvent)> {
        std::mem::take(&mut self.outbox)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sim() -> AuthoritativeSim {
        let config = RuntimeConfig::default();
        let scene = SceneSummary {
            id: "test".into(),
            biome: "camp".into(),
            ambient_rgb: [0.1, 0.08, 0.05],
        };
        AuthoritativeSim::new(config, scene)
    }

    #[test]
    fn player_join_assigns_id() {
        let mut sim = make_sim();
        let id = sim.player_join("Alice".into());
        assert_eq!(id, Some(1));
        assert_eq!(sim.player_count(), 1);
    }

    #[test]
    fn player_leave_removes_session() {
        let mut sim = make_sim();
        let id = sim.player_join("Bob".into()).unwrap();
        sim.player_leave(id);
        assert_eq!(sim.player_count(), 0);
    }

    #[test]
    fn full_server_rejects_join() {
        let mut sim = make_sim();
        sim.bootstrap.max_players = 1;
        sim.player_join("A".into());
        assert_eq!(sim.player_join("B".into()), None);
    }

    #[test]
    fn move_command_updates_position() {
        let mut sim = make_sim();
        let id = sim.player_join("C".into()).unwrap();
        sim.handle_command(id, ClientCommand::Move {
            target: [50.0, 30.0],
            running: true,
        });
        let s = sim.sessions().get(&id).unwrap();
        assert_eq!(s.pos, [50.0, 30.0]);
        assert!(s.running);
    }

    #[test]
    fn zone_transition_notifies_players() {
        let mut sim = make_sim();
        let a = sim.player_join("A".into()).unwrap();
        let _b = sim.player_join("B".into()).unwrap();
        sim.handle_command(a, ClientCommand::RequestZoneTransition {
            to_zone_id: 5,
            reason: "walk".into(),
        });
        let outbox = sim.drain_outbox();
        assert!(!outbox.is_empty());
    }
}
