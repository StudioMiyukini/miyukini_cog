//! Online (multiplayer) frame path for [`GameApp`]: pumping the network
//! client, applying server snapshots with local prediction, handling in-game
//! messages, and the multiplayer HUD. Compiled only with the `netcode` feature.

use super::*;

impl GameApp {
    /// Online variant: instead of ticking a local sim, pump the network client,
    /// apply snapshots, integrate predicted local movement, then render the
    /// [`RemoteWorld`] on top of the static tilemap.
    #[cfg(feature = "netcode")]
    pub(super) fn frame_playing_online(&mut self, dt: f32) {
        self.play_time_secs += dt;
        self.ping_clock += dt;

        // 1. Drain server traffic (snapshots, deltas, chat, ping).
        let msgs = match self.net_client.as_mut() {
            Some(c) => c.drain().unwrap_or_default(),
            None => Vec::new(),
        };
        let now_s = self.online_now();
        for msg in msgs {
            self.handle_in_game_message(msg, now_s);
        }

        // 2. Periodic ping so the HUD can show latency.
        if self.ping_clock >= 5.0 {
            self.ping_clock = 0.0;
            let ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);
            self.last_ping_sent_ms = Some(ts);
            if let Some(c) = self.net_client.as_mut() {
                let _ = c.send(&ClientMessage::Ping { timestamp: ts });
            }
        }

        // 3. Animate locally (procedural: bobs, limb swings, transient flashes).
        if let Some(remote) = self.remote.as_mut() {
            remote.tick_animations(dt);
        }
        self.particles.update(dt);
        self.fx.update(dt);

        // 4. Sync predictor's position back into the remote mirror so the camera
        //    and draw step can use it.
        if let (Some(remote), Some(pred)) = (self.remote.as_mut(), self.predictor.as_ref()) {
            remote.set_self_predicted(crate::components::Position2D::new(pred.pos[0], pred.pos[1]));
        }

        // 5. Recentre the world camera on the predicted self so the tilemap
        //    follows the player.
        let focus = self
            .remote
            .as_ref()
            .map(|r| r.camera_focus(now_s))
            .unwrap_or_else(|| crate::components::Position2D::new(0.0, 0.0));
        if let Some(sim) = self.sim_mut() {
            sim.world.camera.center = focus;
        }

        // 6. Render: tilemap from the backdrop sim, then the remote scene.
        let cam = self.camera();
        if let Some(world) = self.sim.as_ref().map(|s| &s.world) {
            draw_tilemap(&mut self.batch, &cam, world);
        }
        if let Some(remote) = self.remote.as_ref() {
            draw_remote_world(&mut self.batch, &cam, remote, now_s);
        }
        self.particles.draw(&mut self.batch, &cam);
        self.fx.draw(&mut self.batch, &cam);

        // 7. Online HUD overlay (chat, ping, player count).
        self.draw_online_hud();
    }

    /// Handle one server message while in the online [`GameState::Playing`].
    /// Lobby-phase messages are forwarded to [`handle_lobby_message`].
    #[cfg(feature = "netcode")]
    pub(super) fn handle_in_game_message(&mut self, msg: ServerMessage, now: f64) {
        match msg {
            ServerMessage::WorldSnapshot { tick, entities, ground_items, effects: _ } => {
                let self_id = self.remote.as_ref().and_then(|r| r.self_id);
                // Authoritative position for the local entity drives the
                // predictor's reconciliation; we don't have an ack_seq protocol
                // wired in yet so we accept the server position verbatim.
                if let Some(id) = self_id {
                    if let Some(snap) = entities.iter().find(|e| e.id == id) {
                        match self.predictor.as_mut() {
                            Some(pred) => {
                                pred.reconcile([snap.x, snap.y], 0);
                            }
                            None => {
                                self.predictor = Some(Predictor::new([snap.x, snap.y], 4.5));
                            }
                        }
                    }
                }
                if let Some(remote) = self.remote.as_mut() {
                    remote.apply_snapshot(now, tick, &entities, &ground_items);
                }
            }
            ServerMessage::EntitySpawned { snapshot } => {
                if let Some(remote) = self.remote.as_mut() {
                    remote.entity_spawned(&snapshot, now);
                }
            }
            ServerMessage::EntityDied { entity_id } => {
                if let Some(remote) = self.remote.as_mut() {
                    remote.entity_died(entity_id);
                }
            }
            ServerMessage::ChatBroadcast { sender, text } => {
                if let Some(remote) = self.remote.as_mut() {
                    remote.push_chat(format!("{sender}: {text}"));
                }
            }
            ServerMessage::PlayerJoined { name, class } => {
                if let Some(remote) = self.remote.as_mut() {
                    remote.push_chat(format!("{name} ({class}) a rejoint"));
                }
            }
            ServerMessage::PlayerLeft { name } => {
                if let Some(remote) = self.remote.as_mut() {
                    remote.push_chat(format!("{name} est parti"));
                }
            }
            ServerMessage::WaypointDiscovered { player_name, wp_name, .. } => {
                if let Some(remote) = self.remote.as_mut() {
                    remote.push_chat(format!("{player_name} a découvert {wp_name}"));
                }
            }
            ServerMessage::Pong { client_timestamp, .. } => {
                let now_ms = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0);
                if let Some(remote) = self.remote.as_mut() {
                    remote.ping_ms = Some(now_ms.saturating_sub(client_timestamp));
                }
            }
            ServerMessage::ReturnedToLobby => {
                self.leave_online_play();
                self.state = GameState::Lobby;
                self.lobby_screen = Some(LobbyScreen::new());
                if let Some(c) = self.net_client.as_mut() {
                    let _ = c.send(&ClientMessage::ListGames);
                }
            }
            ServerMessage::ServerShutdown { reason } => {
                if let Some(remote) = self.remote.as_mut() {
                    remote.push_chat(format!("Serveur arrêté : {reason}"));
                }
                self.leave_online_play();
                self.net_client = None;
                self.state = GameState::TitleScreen;
                self.title_screen = TitleScreen::new();
            }
            // Inventory / equipment / gold mirror the server snapshot but we
            // don't yet maintain a client-side inventory panel for online play;
            // surface them in the chat log so the player sees the activity.
            ServerMessage::LootDropped { item } => {
                if let Some(remote) = self.remote.as_mut() {
                    remote.push_chat(format!("loot: {} ({:.1},{:.1})", item.name, item.x, item.y));
                }
            }
            ServerMessage::DamageDealt { target, amount, crit } => {
                if crit {
                    if let Some(remote) = self.remote.as_mut() {
                        remote.push_chat(format!("CRIT {amount:.0} → #{target}"));
                    }
                }
            }
            _ => {}
        }
    }

    /// Online HUD: chat backlog (bottom-left), ping (top-right), player count
    /// (top-left). Intentionally minimal — the full UI panels rely on a local
    /// ECS the online client doesn't maintain.
    #[cfg(feature = "netcode")]
    #[allow(clippy::cast_precision_loss)]
    pub(super) fn draw_online_hud(&mut self) {
        let Some(remote) = self.remote.as_ref() else { return };
        // Chat lines: last 6, bottom-left.
        let max_show = 6;
        let n = remote.chat.len().min(max_show);
        let line_h = 14.0;
        let base_y = self.viewport[1] - 24.0 - n as f32 * line_h;
        let mut y = base_y;
        for line in remote.chat.iter().rev().take(max_show).collect::<Vec<_>>().iter().rev() {
            font::draw_text(
                &mut self.batch,
                line.as_bytes(),
                12.0,
                y,
                1.6,
                [0.92, 0.92, 0.75, 1.0],
                RenderLayer::UiScreen,
                0,
                30,
            );
            y += line_h;
        }
        // Player count (top-left).
        let player_count = remote
            .entities
            .values()
            .filter(|e| matches!(e.kind, EntityKind::Player))
            .count();
        let label = format!("Joueurs: {player_count}");
        font::draw_text(
            &mut self.batch,
            label.as_bytes(),
            12.0,
            12.0,
            1.8,
            [0.85, 0.95, 0.85, 1.0],
            RenderLayer::UiScreen,
            0,
            30,
        );
        // Ping (top-right).
        if let Some(ms) = remote.ping_ms {
            let ping_label = format!("{ms} ms");
            let w = font::text_width(ping_label.as_bytes(), 1.8);
            font::draw_text(
                &mut self.batch,
                ping_label.as_bytes(),
                self.viewport[0] - w - 12.0,
                12.0,
                1.8,
                [0.75, 0.95, 1.0, 1.0],
                RenderLayer::UiScreen,
                0,
                30,
            );
        }
    }
}
