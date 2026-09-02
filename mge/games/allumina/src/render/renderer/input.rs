//! @id     games_allumina_render_renderer_input
//! @do     implementer_game_app
//! @role   ui
//! @layer  core
//! @human  games allumina — module « render renderer input ».

//! Input handling for [`GameApp`]: mouse-press routing, per-state keyboard
//! handlers, and the action dispatchers they feed. Split out of `renderer.rs`
//! to keep the application shell focused on the frame loop and state machine.
//! Methods are `pub(super)` so the sibling impls in `renderer` can call them.

use super::*;

impl GameApp {
    pub(super) fn handle_left_press(&mut self) {
        match self.state {
            GameState::TitleScreen => {
                let action = self.title_screen.click(self.mouse);
                self.dispatch_title_action(action);
            }
            GameState::ClassSelect => {
                let action = self.class_select.click(self.mouse);
                self.dispatch_class_action(action);
            }
            #[cfg(feature = "netcode")]
            GameState::Lobby => {
                let mouse = self.mouse;
                let action = self
                    .lobby_screen
                    .as_mut()
                    .map(|s| s.click(mouse))
                    .unwrap_or(LobbyAction::None);
                self.dispatch_lobby_action(action);
            }
            #[cfg(not(feature = "netcode"))]
            GameState::Lobby => {}
            GameState::Playing => {
                let viewport = self.viewport;
                let mouse = self.mouse;
                let consumed = if let Some(sim) = self.sim.as_mut() {
                    crate::ui::handle_left_press(&mut sim.world, &mut self.ui, viewport, mouse)
                } else {
                    false
                };
                if !consumed {
                    self.on_left_click();
                }
            }
            GameState::Paused => self.handle_pause_click(),
            GameState::Dead => self.handle_dead_click(),
            GameState::Loading => {}
        }
    }

    pub(super) fn handle_pause_click(&mut self) {
        // Hit-test the three rows in the modal area.
        let n = PAUSE_ITEMS.len();
        if let Some(idx) = modal_row_at(self.viewport, n, self.mouse) {
            self.pause_cursor = idx;
            self.activate_pause();
        }
    }

    pub(super) fn handle_dead_click(&mut self) {
        let n = DEAD_ITEMS.len();
        if let Some(idx) = modal_row_at(self.viewport, n, self.mouse) {
            self.dead_cursor = idx;
            self.activate_dead();
        }
    }

    pub(super) fn activate_pause(&mut self) {
        match self.pause_cursor {
            0 => self.state = GameState::Playing,
            1 => {
                self.perform_autosave(0);
            }
            _ => {
                self.perform_autosave(0);
                self.state = GameState::TitleScreen;
                self.title_screen = TitleScreen::new();
                self.sim = None;
            }
        }
    }

    pub(super) fn activate_dead(&mut self) {
        match self.dead_cursor {
            0 => self.respawn_to_village(),
            _ => self.load_last_save(),
        }
    }

    pub(super) fn respawn_to_village(&mut self) {
        let class = self.character_class;
        let name = self.character_name.clone();
        self.start_new_game(class, name);
    }

    pub(super) fn load_last_save(&mut self) {
        let saves = save::list_saves();
        if let Some(s) = saves.first().cloned() {
            self.load_save(&s);
        } else {
            self.respawn_to_village();
        }
    }

    pub(super) fn handle_key(&mut self, event_loop: &ActiveEventLoop, event: winit::event::KeyEvent) {
        let physical = match event.physical_key {
            PhysicalKey::Code(c) => Some(c),
            _ => None,
        };
        let logical_text = event.text.as_ref();

        match self.state {
            GameState::TitleScreen => self.key_title(event_loop, physical),
            GameState::ClassSelect => self.key_class_select(physical, logical_text, &event.logical_key),
            GameState::Loading => {}
            #[cfg(feature = "netcode")]
            GameState::Lobby => self.key_lobby(physical, logical_text, &event.logical_key),
            #[cfg(not(feature = "netcode"))]
            GameState::Lobby => {}
            GameState::Playing => self.key_playing(physical),
            GameState::Paused => self.key_paused(physical),
            GameState::Dead => self.key_dead(physical),
        }
    }

    #[cfg(feature = "netcode")]
    pub(super) fn key_lobby(
        &mut self,
        code: Option<KeyCode>,
        text: Option<&SmolStr>,
        logical: &winit::keyboard::Key,
    ) {
        let Some(screen) = self.lobby_screen.as_mut() else { return };
        // Modal-aware key handling: type into the popup if one is open, Esc
        // closes it; Enter submits.
        if screen.modal_active() {
            match code {
                Some(KeyCode::Escape) => {
                    screen.close_modal();
                    return;
                }
                Some(KeyCode::Enter) | Some(KeyCode::NumpadEnter) => {
                    let action = screen.submit_modal();
                    self.dispatch_lobby_action(action);
                    return;
                }
                Some(KeyCode::Backspace) => {
                    screen.type_char('\u{8}');
                    return;
                }
                _ => {}
            }
            if let Some(t) = text {
                for ch in t.chars() {
                    if !ch.is_control() {
                        screen.type_char(ch);
                    }
                }
            } else if let winit::keyboard::Key::Named(NamedKey::Space) = logical {
                screen.type_char(' ');
            }
            return;
        }

        match code {
            Some(KeyCode::Escape) => {
                self.dispatch_lobby_action(LobbyAction::Back);
            }
            Some(KeyCode::F5) => {
                self.dispatch_lobby_action(LobbyAction::Refresh);
            }
            _ => {}
        }
    }

    pub(super) fn key_title(&mut self, event_loop: &ActiveEventLoop, code: Option<KeyCode>) {
        match code {
            Some(KeyCode::ArrowUp) => self.title_screen.move_selection(-1),
            Some(KeyCode::ArrowDown) => self.title_screen.move_selection(1),
            Some(KeyCode::Enter) | Some(KeyCode::NumpadEnter) | Some(KeyCode::Space) => {
                let a = self.title_screen.confirm();
                self.dispatch_title_action(a);
                if matches!(self.state, GameState::TitleScreen) {
                    // Stay on title.
                } else if self.sim.is_none() && !matches!(self.state, GameState::ClassSelect | GameState::Loading) {
                    event_loop.exit();
                }
            }
            Some(KeyCode::Escape) => {
                let a = self.title_screen.back();
                if matches!(a, TitleAction::Quit) {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }

    pub(super) fn key_class_select(&mut self, code: Option<KeyCode>, text: Option<&SmolStr>, logical: &winit::keyboard::Key) {
        if self.class_select.editing_name {
            match code {
                Some(KeyCode::Escape) | Some(KeyCode::Tab) | Some(KeyCode::Enter) | Some(KeyCode::NumpadEnter) => {
                    self.class_select.editing_name = false;
                    return;
                }
                Some(KeyCode::Backspace) => {
                    self.class_select.type_char('\u{8}');
                    return;
                }
                _ => {}
            }
            if let Some(t) = text {
                for ch in t.chars() {
                    if ch.is_control() {
                        continue;
                    }
                    self.class_select.type_char(ch);
                }
            } else if let winit::keyboard::Key::Named(NamedKey::Space) = logical {
                self.class_select.type_char(' ');
            }
            return;
        }

        match code {
            Some(KeyCode::ArrowLeft) => self.class_select.move_selection(-1),
            Some(KeyCode::ArrowRight) => self.class_select.move_selection(1),
            Some(KeyCode::Digit1) => self.class_select.selected = Some(0),
            Some(KeyCode::Digit2) => self.class_select.selected = Some(1),
            Some(KeyCode::Digit3) => self.class_select.selected = Some(2),
            Some(KeyCode::Digit4) => self.class_select.selected = Some(3),
            Some(KeyCode::Enter) | Some(KeyCode::NumpadEnter) => {
                let a = self.class_select.confirm();
                self.dispatch_class_action(a);
            }
            Some(KeyCode::Escape) => {
                let a = self.class_select.back();
                self.dispatch_class_action(a);
            }
            _ => {}
        }
    }

    pub(super) fn key_playing(&mut self, code: Option<KeyCode>) {
        let Some(code) = code else { return };
        match code {
            KeyCode::Escape => {
                #[cfg(feature = "netcode")]
                if self.is_online() {
                    if let Some(c) = self.net_client.as_mut() {
                        let _ = c.send(&ClientMessage::LeaveGame);
                    }
                    return;
                }
                if !self.ui.close_all() {
                    self.state = GameState::Paused;
                    self.pause_cursor = 0;
                }
            }
            KeyCode::KeyI => self.ui.show_inventory = !self.ui.show_inventory,
            KeyCode::KeyC => self.ui.show_character = !self.ui.show_character,
            KeyCode::KeyS => self.ui.show_skills = !self.ui.show_skills,
            KeyCode::KeyQ => self.ui.show_quests = !self.ui.show_quests,
            KeyCode::KeyW => self.ui.show_waypoint = !self.ui.show_waypoint,
            KeyCode::KeyE => self.toggle_vendor(),
            KeyCode::Tab => self.ui.show_minimap = !self.ui.show_minimap,
            KeyCode::Equal | KeyCode::NumpadAdd => {
                if let Some(sim) = self.sim_mut() {
                    sim.world.camera.zoom_in(ZOOM_STEP);
                }
            }
            KeyCode::Minus | KeyCode::NumpadSubtract => {
                if let Some(sim) = self.sim_mut() {
                    sim.world.camera.zoom_out(ZOOM_STEP);
                }
            }
            KeyCode::Digit1 => self.use_or_select_skill(0),
            KeyCode::Digit2 => self.use_or_select_skill(1),
            KeyCode::Digit3 => self.use_or_select_skill(2),
            KeyCode::Digit4 => self.use_or_select_skill(3),
            KeyCode::Digit5 => self.use_or_select_skill(4),
            KeyCode::Digit6 => self.use_or_select_skill(5),
            KeyCode::KeyH => self.hire_next_merc(),
            KeyCode::KeyP => self.toggle_merc_panel(),
            KeyCode::KeyK => self.ui.craft_open = !self.ui.craft_open,
            KeyCode::KeyT => self.town_portal(),
            KeyCode::KeyU => self.unsocket_hovered(),
            KeyCode::F5 => self.perform_autosave(0),
            _ => {}
        }
    }

    pub(super) fn key_paused(&mut self, code: Option<KeyCode>) {
        match code {
            Some(KeyCode::Escape) => self.state = GameState::Playing,
            Some(KeyCode::ArrowUp) => {
                let n = PAUSE_ITEMS.len() as i32;
                self.pause_cursor = ((self.pause_cursor as i32 - 1).rem_euclid(n)) as usize;
            }
            Some(KeyCode::ArrowDown) => {
                let n = PAUSE_ITEMS.len() as i32;
                self.pause_cursor = ((self.pause_cursor as i32 + 1).rem_euclid(n)) as usize;
            }
            Some(KeyCode::Enter) | Some(KeyCode::NumpadEnter) => self.activate_pause(),
            _ => {}
        }
    }

    pub(super) fn key_dead(&mut self, code: Option<KeyCode>) {
        match code {
            Some(KeyCode::ArrowUp) => {
                let n = DEAD_ITEMS.len() as i32;
                self.dead_cursor = ((self.dead_cursor as i32 - 1).rem_euclid(n)) as usize;
            }
            Some(KeyCode::ArrowDown) => {
                let n = DEAD_ITEMS.len() as i32;
                self.dead_cursor = ((self.dead_cursor as i32 + 1).rem_euclid(n)) as usize;
            }
            Some(KeyCode::Enter) | Some(KeyCode::NumpadEnter) => self.activate_dead(),
            _ => {}
        }
    }

    pub(super) fn dispatch_title_action(&mut self, action: TitleAction) {
        match action {
            TitleAction::None => {}
            TitleAction::NewGame => {
                if let Some(class) = self.forced_class.take() {
                    let name = self.character_name.clone();
                    self.start_new_game(class, name);
                } else {
                    self.class_select = ClassSelectScreen::new();
                    self.state = GameState::ClassSelect;
                }
            }
            TitleAction::LoadGame(s) => self.load_save(&s),
            #[cfg(feature = "netcode")]
            TitleAction::Multiplayer => self.enter_lobby(),
            #[cfg(not(feature = "netcode"))]
            TitleAction::Multiplayer => {}
            TitleAction::Quit => {
                self.state = GameState::TitleScreen;
                if let Some(w) = &self.window {
                    let _ = w;
                }
                // Signal quit via standard exit path.
                std::process::exit(0);
            }
        }
    }

    pub(super) fn dispatch_class_action(&mut self, action: ClassSelectAction) {
        match action {
            ClassSelectAction::None => {}
            ClassSelectAction::Start { class, name } => self.start_new_game(class, name),
            ClassSelectAction::Back => {
                self.state = GameState::TitleScreen;
            }
        }
    }

    /// Select skill `idx` and cast it.
    pub(super) fn use_or_select_skill(&mut self, idx: usize) {
        #[cfg(feature = "netcode")]
        if self.is_online() {
            let cam = self.camera();
            let world_pt = cam.screen_to_world(self.mouse[0], self.mouse[1]);
            if let Some(c) = self.net_client.as_mut() {
                let _ = c.send(&ClientMessage::UseSkill {
                    skill_index: idx as u32,
                    target: None,
                    cursor_x: world_pt.x,
                    cursor_y: world_pt.y,
                });
            }
            return;
        }
        let Some(player) = self.player() else { return };
        let Some(sim) = self.sim_mut() else { return };
        let n = sim.world.skills.get(&player).map_or(0, |s| s.bar.len());
        if idx >= n {
            return;
        }
        if let Some(s) = sim.world.skills.get_mut(&player) {
            s.select(idx);
        }
        let target = sim
            .world
            .combat
            .get(&player)
            .and_then(|c| c.target)
            .filter(|t| sim.world.health.get(t).is_some_and(|h| !h.is_empty()))
            .or_else(|| nearest_living_mob(sim, player))
            .unwrap_or(player);
        sim.world.input.command = Some(PlayerCommand::UseSkill { idx, target });
    }

    /// Town portal (key `T`): from any zone, remember the spot and warp to the
    /// village; from the village, warp back to the remembered spot. Online play
    /// is server-authoritative, so it's a no-op there.
    pub(super) fn town_portal(&mut self) {
        #[cfg(feature = "netcode")]
        if self.is_online() {
            return;
        }
        let Some(sim) = self.sim_mut() else { return };
        let w = &mut sim.world;
        if w.zone == ZoneId::Village {
            match w.tp_return.take() {
                Some((zone, tile)) => {
                    w.pending_zone = Some((zone, tile));
                    w.log_event("Portail de retour ouvert.");
                }
                None => w.log_event("Tu es deja en ville."),
            }
            return;
        }
        if let Some(tile) = w.player_entity.and_then(|p| w.positions.get(&p)).map(|p| p.tile()) {
            w.tp_return = Some((w.zone, tile));
        }
        // (-1,-1) = arrive at the village's own spawn (see `zone::enter_zone`).
        w.pending_zone = Some((ZoneId::Village, (-1, -1)));
        w.log_event("Portail vers la ville ouvert.");
    }

    /// Remove a gem from the item currently under the cursor (key `U`), paying
    /// the unsocket fee; the gem returns to the backpack.
    pub(super) fn unsocket_hovered(&mut self) {
        #[cfg(feature = "netcode")]
        if self.is_online() {
            return;
        }
        let Some(player) = self.player() else { return };
        let equip = self.ui.hover_equip;
        let inv = self.ui.hover_inv;
        if let Some(sim) = self.sim_mut() {
            if let Some(slot) = equip {
                crate::plugins::inventory::unsocket_equipped(&mut sim.world, player, slot);
            } else if let Some(idx) = inv {
                crate::plugins::inventory::unsocket_in_inventory(&mut sim.world, player, idx);
            }
        }
    }

    /// Toggle the active mercenary's equipment panel (no-op with no companion).
    pub(super) fn toggle_merc_panel(&mut self) {
        if self.ui.merc_panel.take().is_some() {
            return;
        }
        let Some(player) = self.player() else { return };
        self.ui.merc_panel =
            self.sim_mut().and_then(|sim| crate::ui::merc_ui::active_merc(&sim.world, player));
    }

    pub(super) fn hire_next_merc(&mut self) {
        let Some(player) = self.player() else { return };
        let types = crate::components::mercenary::MercType::all();
        let start = self.hire_cycle;
        self.hire_cycle = self.hire_cycle.wrapping_add(1);
        if let Some(sim) = self.sim_mut() {
            // Prefer the next archetype the player doesn't already field, so
            // pressing H builds a varied party instead of bouncing off duplicates.
            let owned: Vec<_> = crate::systems::mercenary::companions(&sim.world, player)
                .iter()
                .filter_map(|e| sim.world.mercenary_tags.get(e).map(|t| t.merc_type))
                .collect();
            let merc_type = (0..types.len())
                .map(|i| types[(start + i) % types.len()])
                .find(|t| !owned.contains(t))
                .unwrap_or(types[start % types.len()]);
            crate::systems::mercenary::recruit_mercenary(&mut sim.world, player, merc_type);
        }
    }

    pub(super) fn toggle_vendor(&mut self) {
        if self.ui.vendor_merchant.is_some() {
            self.ui.vendor_merchant = None;
            return;
        }
        let Some(player) = self.player() else { return };
        let Some(sim) = self.sim_ref() else { return };
        let Some(&ppos) = sim.world.positions.get(&player) else { return };
        self.ui.vendor_merchant = crate::systems::vendor::nearest_merchant(&sim.world, ppos, 2.5);
    }
}
