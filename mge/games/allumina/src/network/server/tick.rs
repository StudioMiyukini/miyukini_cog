//! Server simulation loop: the fixed-rate tick that advances every active
//! [`GameRoom`], broadcasts snapshots to clients, garbage-collects expired
//! rooms, and runs periodic autosaves. Split out of `server.rs` to separate the
//! orchestration cadence from connection handling.

use super::*;

// ── Simulation loop ─────────────────────────────────────────────────────────

pub(super) fn simulation_loop(lobby: Arc<Mutex<LobbyServer>>) {
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

pub(super) fn rooms_snapshot(lobby: &Arc<Mutex<LobbyServer>>) -> Vec<Arc<Mutex<GameRoom>>> {
    match lobby.lock() {
        Ok(g) => g.rooms.values().cloned().collect(),
        Err(p) => p.into_inner().rooms.values().cloned().collect(),
    }
}

pub(super) fn tick_all_rooms(lobby: &Arc<Mutex<LobbyServer>>, dt: f32) {
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
pub(super) fn sweep_expired_rooms(lobby: &Arc<Mutex<LobbyServer>>) {
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

pub(super) fn broadcast_all_snapshots(lobby: &Arc<Mutex<LobbyServer>>) {
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

pub(super) fn autosave_all_rooms(lobby: &Arc<Mutex<LobbyServer>>, server_start: Instant) {
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
