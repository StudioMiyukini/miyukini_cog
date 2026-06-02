//! The winit application: owns the simulation [`App`], the wgpu
//! [`GraphicsState`], and per-frame input. Each `RedrawRequested` advances the
//! sim by the frame delta, rebuilds the sprite batch (tilemap → entities →
//! VFX → HUD/UI), and presents it.

use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use mge_content::{ColorRgba, RenderProfile};
use mge_render::batch::{RenderLayer, SpriteBatch};
use mge_render::GraphicsState;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, NamedKey, PhysicalKey, SmolStr};
use winit::window::{Window, WindowAttributes, WindowId};

use crate::app::{App, GameState};
use crate::components::player::ClassType;
use crate::components::world::ZoneId;
use crate::ecs::{Entity, RenderEvent, SkillFx, VisualKind};
#[cfg(feature = "netcode")]
use crate::network::lobby_client::LobbyNetClient;
#[cfg(feature = "netcode")]
use crate::network::prediction::Predictor;
#[cfg(feature = "netcode")]
use crate::network::protocol::{ClientMessage, EntityKind, ServerMessage};
#[cfg(feature = "netcode")]
use crate::network::snapshot_renderer::{draw_remote_world, RemoteWorld};
#[cfg(feature = "netcode")]
use crate::network::{DEFAULT_PORT, DEFAULT_SERVER};
use crate::plugins::player::PlayerCommand;
use crate::save::{self, SaveData, SaveSummary};
use crate::ui::class_select::{ClassSelectAction, ClassSelectScreen};
#[cfg(feature = "netcode")]
use crate::ui::lobby::{LobbyAction, LobbyScreen};
use crate::ui::title_screen::{draw_save_indicator, TitleAction, TitleScreen};
use crate::ui::UiState;

use super::animation::AnimState;
use super::camera::Camera2D;
use super::feedback::CombatFx;
use super::font;
use super::particles::ParticleSystem;
use super::sprite::{draw_ally_overlays, draw_health_bars, draw_projectiles, draw_world_entities, push_quad};
use super::tilemap_renderer::draw_tilemap;
use super::vertex;

/// Mouse/keyboard input routing and per-state key handlers for [`GameApp`].
mod input;

/// Online (multiplayer) frame path: snapshot apply, prediction, in-game
/// messages, and the multiplayer HUD. Only built with the `netcode` feature.
#[cfg(feature = "netcode")]
mod online;

/// Default window size.
const WIN_W: u32 = 1280;
const WIN_H: u32 = 720;
/// Zoom step per wheel notch.
const ZOOM_STEP: f32 = 0.15;
/// Largest sim step we'll take in one frame (guards against stalls).
const MAX_DT: f32 = 0.1;

/// Drives the game: simulation + renderer + input.
pub struct GameApp {
    sim: Option<App>,
    title: String,

    window: Option<Arc<Window>>,
    window_id: Option<WindowId>,
    gfx: Option<GraphicsState>,
    fatal: Option<anyhow::Error>,

    batch: SpriteBatch,
    viewport: [f32; 2],
    mouse: [f32; 2],
    last_frame: Instant,

    particles: ParticleSystem,
    fx: CombatFx,
    ui: UiState,
    /// Free-running render clock (seconds) for cosmetic animation.
    time: f32,
    /// Which mercenary archetype the next hire (H key) will recruit.
    hire_cycle: usize,

    // ── State machine + screens ───────────────────────────────────────
    state: GameState,
    title_screen: TitleScreen,
    class_select: ClassSelectScreen,
    /// Forced class when `--class` was passed on the command line; skips the
    /// class-select screen on the first New Game.
    forced_class: Option<ClassType>,
    /// Character name + class for the active save (carried into Playing).
    character_name: String,
    character_class: ClassType,
    /// Stable ID for this character (UUID-shaped).
    player_id: String,
    /// Cumulative playtime in seconds.
    play_time_secs: f32,
    /// Seconds since the last auto-save.
    autosave_clock: f32,
    /// Time the "Sauvegarde..." disc stays visible.
    save_indicator: f32,
    /// Previous zone, to detect transitions and force an auto-save.
    last_zone: Option<ZoneId>,
    /// Previous level, to detect level-ups and force an auto-save.
    last_level: u32,
    /// Currently focused pause-menu row.
    pause_cursor: usize,
    /// Currently focused dead-menu row.
    dead_cursor: usize,

    // ── Multiplayer lobby (only present with the `netcode` feature) ──
    #[cfg(feature = "netcode")]
    lobby_screen: Option<LobbyScreen>,
    #[cfg(feature = "netcode")]
    net_client: Option<LobbyNetClient>,
    /// Mirror of the authoritative server world (Some ⇒ online play, drives the
    /// renderer instead of the local `sim`).
    #[cfg(feature = "netcode")]
    remote: Option<RemoteWorld>,
    /// Local prediction for the self avatar in online mode.
    #[cfg(feature = "netcode")]
    predictor: Option<Predictor>,
    /// Time the current online session was joined (snapshot timestamps are
    /// relative to this instant).
    #[cfg(feature = "netcode")]
    online_start: Option<Instant>,
    /// Accumulator that gates the rate at which we resend `MoveIntent` while a
    /// move command is held.
    #[cfg(feature = "netcode")]
    online_input_clock: f32,
    /// Most recent send timestamp for `Ping`, so `Pong` can measure latency.
    #[cfg(feature = "netcode")]
    last_ping_sent_ms: Option<u64>,
    /// Clock controlling automatic pings every `PING_INTERVAL`.
    #[cfg(feature = "netcode")]
    ping_clock: f32,
    /// Set by [`run_game_connect`]: open the lobby right after the window
    /// becomes available, skipping the title screen.
    #[cfg(feature = "netcode")]
    auto_enter_lobby_on_resume: bool,
}

const PAUSE_ITEMS: [&[u8]; 3] = [b"REPRENDRE", b"SAUVEGARDER", b"QUITTER"];
const DEAD_ITEMS: [&[u8]; 2] = [b"RESSUSCITER AU VILLAGE", b"CHARGER DERNIERE SAVE"];

impl GameApp {
    pub fn new(mut sim: Option<App>, title: impl Into<String>, forced_class: Option<ClassType>) -> Self {
        // If a pre-built sim was passed (--class shortcut), recentre the camera
        // on the player and start straight in Playing.
        let mut state = GameState::TitleScreen;
        let mut character_class = ClassType::Soldat;
        let mut last_zone = None;
        let mut last_level = 1;
        if let Some(s) = sim.as_mut() {
            if let Some(p) = s.world.player_entity.and_then(|e| s.world.positions.get(&e).copied()) {
                s.world.camera.center = p;
            }
            state = GameState::Playing;
            character_class = s
                .world
                .player_entity
                .and_then(|e| s.world.classes.get(&e).copied())
                .unwrap_or(ClassType::Soldat);
            last_zone = Some(s.world.zone);
            last_level = s
                .world
                .player_entity
                .and_then(|e| s.world.progression.get(&e))
                .map_or(1, |p| p.level());
        }
        Self {
            sim,
            title: title.into(),
            window: None,
            window_id: None,
            gfx: None,
            fatal: None,
            batch: SpriteBatch::new(),
            viewport: [WIN_W as f32, WIN_H as f32],
            mouse: [0.0, 0.0],
            last_frame: Instant::now(),
            particles: ParticleSystem::new(),
            fx: CombatFx::new(),
            ui: UiState::new(),
            time: 0.0,
            hire_cycle: 0,
            state,
            title_screen: TitleScreen::new(),
            class_select: ClassSelectScreen::new(),
            forced_class,
            character_name: "Héros".to_string(),
            character_class,
            player_id: save::generate_player_id(),
            play_time_secs: 0.0,
            autosave_clock: 0.0,
            save_indicator: 0.0,
            last_zone,
            last_level,
            pause_cursor: 0,
            dead_cursor: 0,
            #[cfg(feature = "netcode")]
            lobby_screen: None,
            #[cfg(feature = "netcode")]
            net_client: None,
            #[cfg(feature = "netcode")]
            remote: None,
            #[cfg(feature = "netcode")]
            predictor: None,
            #[cfg(feature = "netcode")]
            online_start: None,
            #[cfg(feature = "netcode")]
            online_input_clock: 0.0,
            #[cfg(feature = "netcode")]
            last_ping_sent_ms: None,
            #[cfg(feature = "netcode")]
            ping_clock: 0.0,
            #[cfg(feature = "netcode")]
            auto_enter_lobby_on_resume: false,
        }
    }

    fn set_fatal(&mut self, event_loop: &ActiveEventLoop, err: anyhow::Error) {
        self.fatal = Some(err);
        event_loop.exit();
    }

    /// Current render camera snapshot.
    fn camera(&self) -> Camera2D {
        let center = self
            .sim
            .as_ref()
            .map(|s| s.world.camera.center)
            .unwrap_or_else(|| crate::components::Position2D::new(0.0, 0.0));
        let zoom = self.sim.as_ref().map(|s| s.world.camera.zoom).unwrap_or(1.0);
        Camera2D::new(center, zoom, self.viewport).with_time(self.time)
    }

    fn sim_ref(&self) -> Option<&App> {
        self.sim.as_ref()
    }
    fn sim_mut(&mut self) -> Option<&mut App> {
        self.sim.as_mut()
    }

    /// The player entity, if alive/spawned.
    fn player(&self) -> Option<Entity> {
        self.sim.as_ref().and_then(|s| s.world.player_entity)
    }

    // ── State transitions ───────────────────────────────────────────

    /// Start a brand-new run (called after class-select or the --class shortcut).
    fn start_new_game(&mut self, class: ClassType, name: String) {
        let mut sim = crate::build_app(class, 0xA11CE);
        if let Some(p) = sim.world.player_entity.and_then(|e| sim.world.positions.get(&e).copied()) {
            sim.world.camera.center = p;
        }
        self.character_class = class;
        self.character_name = name;
        self.player_id = save::generate_player_id();
        self.play_time_secs = 0.0;
        self.autosave_clock = 0.0;
        self.last_zone = Some(sim.world.zone);
        self.last_level = sim
            .world
            .player_entity
            .and_then(|e| sim.world.progression.get(&e))
            .map_or(1, |p| p.level());
        self.sim = Some(sim);
        self.state = GameState::Playing;
        self.ui.close_all();
        self.last_frame = Instant::now();
    }

    /// Load a save file and route into Playing.
    fn load_save(&mut self, summary: &SaveSummary) {
        let data = match save::load_from_path(&summary.path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("save load failed: {e}");
                return;
            }
        };
        self.state = GameState::Loading;
        let zone = ZoneId::from_key(data.current_zone.as_str());
        let mut sim = crate::build_app_in(data.class, 0xA11CE, zone);
        save::apply_to_world(&mut sim.world, &data);
        if let Some(p) = sim.world.player_entity.and_then(|e| sim.world.positions.get(&e).copied()) {
            sim.world.camera.center = p;
        }
        self.character_name = data.character_name.clone();
        self.character_class = data.class;
        self.player_id = data.player_id.clone();
        self.play_time_secs = data.play_time_secs as f32;
        self.autosave_clock = 0.0;
        self.last_zone = Some(zone);
        self.last_level = data.level;
        self.sim = Some(sim);
        self.state = GameState::Playing;
        self.ui.close_all();
        self.last_frame = Instant::now();
    }

    fn perform_autosave(&mut self, slot: u32) {
        let Some(sim) = self.sim.as_ref() else { return };
        let Some(data) = save::snapshot(&sim.world, &self.character_name, &self.player_id, self.play_time_secs as u64)
        else {
            return;
        };
        match save::save_to_slot(&data, slot) {
            Ok(_) => self.save_indicator = save::SAVE_INDICATOR_SECS,
            Err(e) => eprintln!("save failed: {e}"),
        }
    }

    /// Left click → move to the clicked tile (or send `MoveIntent` online).
    fn on_left_click(&mut self) {
        let cam = self.camera();
        let world_pt = cam.screen_to_world(self.mouse[0], self.mouse[1]);
        #[cfg(feature = "netcode")]
        if self.is_online() {
            if let Some(c) = self.net_client.as_mut() {
                let _ = c.send(&ClientMessage::MoveIntent {
                    target_x: world_pt.x,
                    target_y: world_pt.y,
                });
            }
            // Push the input into the predictor so the local avatar moves
            // immediately, without waiting for the next snapshot.
            if let Some(pred) = self.predictor.as_mut() {
                let dx = world_pt.x - pred.pos[0];
                let dy = world_pt.y - pred.pos[1];
                pred.push_input(dx, dy, 1.0 / 60.0);
            }
            return;
        }
        // A click on a chest or the merchant interacts with it (opening when in
        // reach, otherwise walking up to it); anything else is a move order.
        if self.try_world_interact(world_pt) {
            return;
        }
        let (tx, ty) = world_pt.tile();
        let Some(sim) = self.sim_mut() else { return };
        if sim.world.map.in_bounds(tx, ty) {
            sim.world.input.command = Some(PlayerCommand::MoveTo(tx, ty));
        }
    }

    /// If the click landed on a chest or the merchant, open it (when the player
    /// is in reach) or path toward it. Returns whether an interaction was taken.
    fn try_world_interact(&mut self, click: crate::components::Position2D) -> bool {
        use crate::components::Position2D;
        enum Act { Open(Entity), Vendor(Entity), Forge(Entity), Move(i32, i32) }
        let act = {
            let Some(sim) = self.sim.as_ref() else { return false };
            let w = &sim.world;
            let Some(player) = w.player_entity else { return false };
            let Some(ppos) = w.positions.get(&player).copied() else { return false };
            let near = |p: &Position2D| p.distance(click) <= 0.9;
            // Closest unopened chest under the cursor.
            let chest = w
                .chests
                .iter()
                .filter(|(_, c)| !c.opened)
                .map(|(e, c)| (*e, Position2D::new(c.x, c.y)))
                .filter(|(_, p)| near(p))
                .min_by(|a, b| a.1.distance(click).total_cmp(&b.1.distance(click)));
            if let Some((e, p)) = chest {
                Some(if ppos.distance(p) <= 1.4 { Act::Open(e) } else { let (tx, ty) = p.tile(); Act::Move(tx, ty) })
            } else {
                use crate::components::npc::NpcKind;
                // The merchant NPC under the cursor opens a vendor window; the
                // blacksmith opens the forge (salvage) window instead.
                let nearest_npc = |want_merchant: bool| {
                    w.npcs
                        .iter()
                        .filter(|(e, kind)| {
                            if want_merchant {
                                w.merchants.contains_key(*e)
                            } else {
                                matches!(kind, NpcKind::Blacksmith)
                            }
                        })
                        .filter_map(|(e, _)| w.positions.get(e).map(|p| (*e, *p)))
                        .filter(|(_, p)| near(p))
                        .min_by(|a, b| a.1.distance(click).total_cmp(&b.1.distance(click)))
                };
                nearest_npc(true)
                    .map(|(e, p)| if ppos.distance(p) <= 2.5 { Act::Vendor(e) } else { let (tx, ty) = p.tile(); Act::Move(tx, ty) })
                    .or_else(|| {
                        nearest_npc(false).map(|(e, p)| {
                            if ppos.distance(p) <= 2.5 { Act::Forge(e) } else { let (tx, ty) = p.tile(); Act::Move(tx, ty) }
                        })
                    })
            }
        };
        match act {
            Some(Act::Open(e)) => {
                if let Some(s) = self.sim_mut() { s.world.input.command = Some(PlayerCommand::OpenChest(e)); }
                true
            }
            Some(Act::Vendor(e)) => { self.ui.vendor_merchant = Some(e); true }
            Some(Act::Forge(e)) => { self.ui.forge_smith = Some(e); true }
            Some(Act::Move(tx, ty)) => {
                if let Some(s) = self.sim_mut() { s.world.input.command = Some(PlayerCommand::MoveTo(tx, ty)); }
                true
            }
            None => false,
        }
    }

    /// Right click → in a panel, transfer the item (equip/unequip); otherwise
    /// attack the mob under the cursor (online: send AttackTarget).
    fn on_right_click(&mut self) {
        // Panel right-click (equip/unequip) takes priority over a world attack.
        if let Some(sim) = self.sim.as_mut() {
            if crate::ui::handle_right_click(&mut sim.world, &self.ui, self.viewport, self.mouse) {
                return;
            }
        }
        let cam = self.camera();
        let click = cam.screen_to_world(self.mouse[0], self.mouse[1]);
        #[cfg(feature = "netcode")]
        if self.is_online() {
            let now_s = self.online_now();
            let target_id = self.remote.as_ref().and_then(|r| {
                r.entities
                    .values()
                    .filter(|e| matches!(e.kind, EntityKind::Mob | EntityKind::Boss))
                    .filter_map(|e| {
                        let p = e.interpolated_pos(now_s);
                        let d = click.distance(p);
                        if d <= 1.5 {
                            Some((e.id, d))
                        } else {
                            None
                        }
                    })
                    .min_by(|a, b| a.1.total_cmp(&b.1))
                    .map(|(id, _)| id)
            });
            if let (Some(id), Some(c)) = (target_id, self.net_client.as_mut()) {
                let _ = c.send(&ClientMessage::AttackTarget { entity_id: id });
            }
            return;
        }
        let Some(sim) = self.sim_mut() else { return };
        let target = sim
            .world
            .mob_entities()
            .into_iter()
            .filter(|e| sim.world.health.get(e).is_some_and(|h| !h.is_empty()))
            .filter_map(|e| sim.world.positions.get(&e).map(|p| (e, click.distance(*p))))
            .filter(|(_, d)| *d <= 1.5)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(e, _)| e);
        if let Some(t) = target {
            sim.world.input.command = Some(PlayerCommand::Attack(t));
        }
    }

    /// If idle and standing on loot, queue a pickup.
    fn auto_pickup(&mut self) {
        let Some(sim) = self.sim_mut() else { return };
        if sim.world.input.command.is_some() {
            return;
        }
        let Some(player) = sim.world.player_entity else { return };
        let Some(&ppos) = sim.world.positions.get(&player) else { return };
        if let Some(item) = sim
            .world
            .ground_item_entities()
            .into_iter()
            .find(|e| sim.world.positions.get(e).is_some_and(|p| ppos.distance(*p) <= 1.4))
        {
            sim.world.input.command = Some(PlayerCommand::Pickup(item));
        }
    }

    fn sync_animations(&mut self) {
        let Some(sim) = self.sim_mut() else { return };
        let world = &mut sim.world;
        let mut ids: Vec<Entity> = world.mob_entities();
        ids.extend(world.summon_entities());
        ids.extend(world.mercenary_entities());
        if let Some(p) = world.player_entity {
            ids.push(p);
        }
        for e in ids {
            let walking = world.move_targets.contains_key(&e);
            let anim = world.animations.entry(e).or_default();
            anim.set_locomotion(walking);
        }
    }

    fn tick_animations(&mut self, dt: f32) {
        let Some(sim) = self.sim_mut() else { return };
        for anim in sim.world.animations.values_mut() {
            anim.tick(dt);
        }
    }

    fn process_events(&mut self, events: Vec<RenderEvent>) {
        let player = self.sim.as_ref().and_then(|s| s.world.player_entity);
        for ev in events {
            match ev {
                RenderEvent::Damage { attacker, target, pos, amount, crit, dodged } => {
                    if let Some(sim) = self.sim_mut() {
                        if let Some(a) = sim.world.animations.get_mut(&attacker) {
                            a.trigger(AnimState::Attack, crit);
                        }
                    }
                    if dodged {
                        self.fx.miss(pos);
                        continue;
                    }
                    if let Some(sim) = self.sim_mut() {
                        if let Some(t) = sim.world.animations.get_mut(&target) {
                            t.trigger(AnimState::Hit, crit);
                        }
                    }
                    let incoming = Some(target) == player;
                    self.fx.damage(pos, amount, crit, incoming);
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    let drops = (amount / 4.0).clamp(3.0, 16.0) as u32;
                    self.particles.emit_blood(pos, drops);
                    if crit {
                        self.particles.emit_crit(pos);
                    }
                }
                RenderEvent::Block { pos } => {
                    self.fx.block(pos);
                    self.fx.flash(pos, [0.6, 0.78, 1.0, 1.0], 16.0);
                }
                RenderEvent::Death { pos, kind } => {
                    let color = visual_color(kind);
                    let boss = kind == VisualKind::Boss;
                    self.fx.corpse(pos, color, boss);
                    self.particles.emit_burst(pos, color, if boss { 40 } else { 18 }, 4.0);
                    self.fx.flash(pos, vertex::WHITE, if boss { 60.0 } else { 24.0 });
                    if boss {
                        self.fx.ring(pos, [1.0, 0.85, 0.3, 1.0], 90.0);
                    }
                }
                RenderEvent::Loot { pos, rarity } => {
                    self.particles.emit_loot(pos, vertex::rarity_color(rarity));
                }
                RenderEvent::Skill { caster, from, to, fx } => {
                    let state = match fx {
                        SkillFx::Fire
                        | SkillFx::Lightning
                        | SkillFx::Frost
                        | SkillFx::Blink
                        | SkillFx::Buff
                        | SkillFx::Summon
                        | SkillFx::Holy
                        | SkillFx::Heal
                        | SkillFx::Meteor
                        | SkillFx::Poison => AnimState::Cast,
                        _ => AnimState::Attack,
                    };
                    if let Some(sim) = self.sim_mut() {
                        if let Some(a) = sim.world.animations.get_mut(&caster) {
                            a.trigger(state, false);
                        }
                    }
                    super::skill_fx::emit(&mut self.particles, from, to, fx);
                    self.fx.skill(from, to, fx);
                }
                RenderEvent::Interrupt { target, pos } => {
                    if let Some(sim) = self.sim_mut() {
                        if let Some(t) = sim.world.animations.get_mut(&target) {
                            t.trigger(AnimState::Hit, false);
                        }
                    }
                    self.fx.interrupt(pos);
                }
            }
        }
    }

    // ── Per-frame dispatch ──────────────────────────────────────────

    fn frame(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        let dt = (now - self.last_frame).as_secs_f32().min(MAX_DT);
        self.last_frame = now;
        self.time += dt;
        self.save_indicator = (self.save_indicator - dt).max(0.0);

        self.batch.clear();
        match self.state {
            GameState::TitleScreen => self.frame_title(dt),
            GameState::ClassSelect => self.frame_class_select(dt),
            GameState::Loading => {
                fill_screen(&mut self.batch, self.viewport, [0.0, 0.0, 0.0, 1.0]);
                font::draw_text_centered(
                    &mut self.batch,
                    b"CHARGEMENT...",
                    self.viewport[0] * 0.5,
                    self.viewport[1] * 0.5,
                    3.0,
                    [0.95, 0.85, 0.45, 1.0],
                    RenderLayer::UiScreen,
                    0,
                    0,
                );
            }
            #[cfg(feature = "netcode")]
            GameState::Lobby => self.frame_lobby(dt),
            #[cfg(not(feature = "netcode"))]
            GameState::Lobby => self.state = GameState::TitleScreen,
            GameState::Playing => self.frame_playing(dt),
            GameState::Paused => self.frame_paused(dt),
            GameState::Dead => self.frame_dead(dt),
        }
        self.batch.sort();

        if let Some(gfx) = &mut self.gfx {
            if let Err(e) = gfx.render(&self.batch) {
                self.set_fatal(event_loop, e);
            }
        }
    }

    fn frame_title(&mut self, dt: f32) {
        self.title_screen.tick(dt, &mut self.particles);
        self.particles.update(dt);
        self.title_screen.draw(&mut self.batch, self.viewport, &self.particles);
    }

    fn frame_class_select(&mut self, dt: f32) {
        self.class_select.tick(dt);
        self.particles.update(dt);
        self.class_select.draw(&mut self.batch, self.viewport);
    }

    #[cfg(feature = "netcode")]
    fn frame_lobby(&mut self, dt: f32) {
        // Pump inbound network events first so anything the screen reads is
        // current with the latest server state.
        self.pump_lobby_network();

        // Tick the screen; an auto-refresh action is forwarded as ListGames.
        if let Some(screen) = self.lobby_screen.as_mut() {
            let action = screen.tick(dt);
            self.dispatch_lobby_action(action);
        }

        if let Some(screen) = self.lobby_screen.as_mut() {
            screen.draw(&mut self.batch, self.viewport);
        }
    }

    #[cfg(feature = "netcode")]
    fn pump_lobby_network(&mut self) {
        let Some(client) = self.net_client.as_mut() else { return };
        let msgs = match client.drain() {
            Ok(m) => m,
            Err(()) => {
                // Connection lost.
                self.net_client = None;
                if let Some(screen) = self.lobby_screen.as_mut() {
                    screen.set_error("connexion perdue");
                }
                return;
            }
        };
        for msg in msgs {
            self.handle_lobby_message(msg);
        }
    }

    #[cfg(feature = "netcode")]
    fn handle_lobby_message(&mut self, msg: ServerMessage) {
        match msg {
            ServerMessage::LoginAccepted { .. } => {
                if let Some(client) = self.net_client.as_mut() {
                    let _ = client.send(&ClientMessage::ListGames);
                }
            }
            ServerMessage::LoginRejected { reason } => {
                if let Some(screen) = self.lobby_screen.as_mut() {
                    screen.set_error(format!("Login refusé : {reason}"));
                }
                self.net_client = None;
            }
            ServerMessage::GameList { games } => {
                if let Some(screen) = self.lobby_screen.as_mut() {
                    screen.set_games(games);
                }
            }
            ServerMessage::GameCreated { .. } => {
                // The follow-up `JoinedGame` is the real transition cue.
            }
            ServerMessage::JoinedGame { game_id, your_entity_id, tick } => {
                println!(
                    "[client] joined game #{game_id} as entity #{your_entity_id} @ tick {tick}"
                );
                self.enter_online_play(your_entity_id);
            }
            ServerMessage::JoinFailed { reason } => {
                if let Some(screen) = self.lobby_screen.as_mut() {
                    screen.set_error(reason);
                }
            }
            ServerMessage::ReturnedToLobby => {
                self.leave_online_play();
                self.state = GameState::Lobby;
                if self.lobby_screen.is_none() {
                    self.lobby_screen = Some(LobbyScreen::new());
                }
                if let Some(screen) = self.lobby_screen.as_mut() {
                    screen.set_error("De retour au lobby");
                }
                if let Some(client) = self.net_client.as_mut() {
                    let _ = client.send(&ClientMessage::ListGames);
                }
            }
            ServerMessage::GameClosed { .. } => {
                // The next GameList broadcast removes the stale entry.
            }
            ServerMessage::ServerShutdown { reason } => {
                if let Some(screen) = self.lobby_screen.as_mut() {
                    screen.set_error(format!("Serveur arrêté : {reason}"));
                }
                self.net_client = None;
            }
            _ => {
                // In-game traffic we don't surface in the lobby yet.
            }
        }
    }

    #[cfg(feature = "netcode")]
    fn dispatch_lobby_action(&mut self, action: LobbyAction) {
        let Some(client) = self.net_client.as_mut() else {
            // Without a live socket the only reasonable response to a UI
            // action is to bail back to the title.
            if !matches!(action, LobbyAction::None) {
                self.state = GameState::TitleScreen;
                self.lobby_screen = None;
            }
            return;
        };
        match action {
            LobbyAction::None => {}
            LobbyAction::Refresh => {
                let _ = client.send(&ClientMessage::ListGames);
            }
            LobbyAction::JoinPublic { game_id } => {
                let _ = client.send(&ClientMessage::JoinGame { game_id, password: None });
            }
            LobbyAction::JoinPrivate { game_id, password } => {
                let _ = client.send(&ClientMessage::JoinGame { game_id, password: Some(password) });
            }
            LobbyAction::Create { name, max_players, difficulty, password } => {
                let _ = client.send(&ClientMessage::CreateGame { name, max_players, difficulty, password });
            }
            LobbyAction::Back => {
                self.net_client = None;
                self.lobby_screen = None;
                self.state = GameState::TitleScreen;
                self.title_screen = TitleScreen::new();
            }
        }
    }

    /// Transition out of the lobby into a server-driven [`GameState::Playing`]
    /// session, with the renderer reading from [`RemoteWorld`] instead of a
    /// local [`App`] simulation.
    #[cfg(feature = "netcode")]
    fn enter_online_play(&mut self, self_entity_id: u64) {
        // Build a sim only as a static backdrop for the tilemap. We never tick
        // it; the server is authoritative.
        let backdrop = crate::build_app(self.character_class, 0xA11CE);
        self.sim = Some(backdrop);

        let mut remote = RemoteWorld::new();
        remote.set_self_id(self_entity_id);
        self.remote = Some(remote);
        self.predictor = None;
        self.online_start = Some(Instant::now());
        self.online_input_clock = 0.0;
        self.ping_clock = 0.0;
        self.lobby_screen = None;
        self.state = GameState::Playing;
        self.ui.close_all();
        self.last_frame = Instant::now();
    }

    /// Drop all server-driven state (returning to the lobby or quitting).
    #[cfg(feature = "netcode")]
    fn leave_online_play(&mut self) {
        self.remote = None;
        self.predictor = None;
        self.online_start = None;
        self.sim = None;
    }

    /// Seconds elapsed since this online session was joined — used as the
    /// shared clock for snapshot timestamps and interpolation.
    #[cfg(feature = "netcode")]
    fn online_now(&self) -> f64 {
        self.online_start.map_or(0.0, |t| t.elapsed().as_secs_f64())
    }

    /// True when the renderer should drive itself from `remote` rather than `sim`.
    #[cfg(feature = "netcode")]
    fn is_online(&self) -> bool {
        self.remote.is_some() && self.net_client.is_some()
    }

    #[cfg(not(feature = "netcode"))]
    fn is_online(&self) -> bool {
        false
    }

    /// Connect to the multiplayer server and switch into the lobby state.
    #[cfg(feature = "netcode")]
    fn enter_lobby(&mut self) {
        let mut screen = LobbyScreen::new();
        let player_id = self.player_id.clone();
        let character_name = self.character_name.clone();
        let class = self.character_class;
        match LobbyNetClient::connect(DEFAULT_SERVER, DEFAULT_PORT, player_id, character_name, class, None) {
            Ok(client) => {
                self.net_client = Some(client);
                self.state = GameState::Lobby;
            }
            Err(e) => {
                screen.set_error(format!("Connexion échouée : {e}"));
                self.state = GameState::Lobby;
            }
        }
        self.lobby_screen = Some(screen);
    }

    fn frame_playing(&mut self, dt: f32) {
        #[cfg(feature = "netcode")]
        if self.is_online() {
            self.frame_playing_online(dt);
            return;
        }
        self.play_time_secs += dt;
        self.autosave_clock += dt;

        self.auto_pickup();
        if let Some(sim) = self.sim_mut() {
            sim.tick(dt);
        }

        // Death detection.
        let dead = self
            .sim
            .as_ref()
            .and_then(|s| s.world.player_entity)
            .and_then(|p| self.sim.as_ref().and_then(|s| s.world.health.get(&p)).map(|h| h.is_empty()))
            .unwrap_or(false);
        if dead {
            self.state = GameState::Dead;
            self.dead_cursor = 0;
        }

        // Zone-transition auto-save.
        let cur_zone = self.sim.as_ref().map(|s| s.world.zone);
        if cur_zone != self.last_zone {
            self.last_zone = cur_zone;
            self.perform_autosave(0);
        }

        // Level-up auto-save.
        let cur_level = self
            .sim
            .as_ref()
            .and_then(|s| s.world.player_entity)
            .and_then(|p| self.sim.as_ref().and_then(|s| s.world.progression.get(&p)))
            .map(|p| p.level());
        if let Some(l) = cur_level {
            if l > self.last_level {
                self.last_level = l;
                self.perform_autosave(0);
            }
        }

        // Idle-interval auto-save.
        if self.autosave_clock >= save::AUTOSAVE_INTERVAL_SECS {
            self.autosave_clock = 0.0;
            self.perform_autosave(0);
        }

        if let Some(sim) = self.sim_mut() {
            let log = sim.world.drain_log();
            self.ui.push_log(log);
        }

        self.sync_animations();
        self.tick_animations(dt);
        let events = self.sim_mut().map(|s| s.world.drain_render_events()).unwrap_or_default();
        self.process_events(events);
        self.particles.update(dt);
        self.fx.update(dt);

        let cam = self.camera();
        if let Some(world) = self.sim.as_ref().map(|s| &s.world) {
            draw_tilemap(&mut self.batch, &cam, world);
            draw_world_entities(&mut self.batch, &cam, world);
            draw_projectiles(&mut self.batch, &cam, world);
            draw_health_bars(&mut self.batch, &cam, world);
            draw_ally_overlays(&mut self.batch, &cam, world);
        }
        self.particles.draw(&mut self.batch, &cam);
        self.fx.draw(&mut self.batch, &cam);
        if let Some(world) = self.sim.as_ref().map(|s| &s.world) {
            crate::ui::draw(&mut self.batch, world, &mut self.ui, self.viewport, self.mouse, self.time);
        }
        draw_save_indicator(&mut self.batch, self.viewport, self.save_indicator, save::SAVE_INDICATOR_SECS);
    }

    fn frame_paused(&mut self, _dt: f32) {
        // Render the last in-game frame as a still backdrop, then a modal panel.
        let cam = self.camera();
        if let Some(world) = self.sim.as_ref().map(|s| &s.world) {
            draw_tilemap(&mut self.batch, &cam, world);
            draw_world_entities(&mut self.batch, &cam, world);
            draw_projectiles(&mut self.batch, &cam, world);
            draw_health_bars(&mut self.batch, &cam, world);
            draw_ally_overlays(&mut self.batch, &cam, world);
        }
        self.fx.draw(&mut self.batch, &cam);
        if let Some(world) = self.sim.as_ref().map(|s| &s.world) {
            crate::ui::draw(&mut self.batch, world, &mut self.ui, self.viewport, self.mouse, self.time);
        }
        draw_modal(&mut self.batch, self.viewport, b"PAUSE", &PAUSE_ITEMS, self.pause_cursor);
        draw_save_indicator(&mut self.batch, self.viewport, self.save_indicator, save::SAVE_INDICATOR_SECS);
    }

    fn frame_dead(&mut self, _dt: f32) {
        let cam = self.camera();
        if let Some(world) = self.sim.as_ref().map(|s| &s.world) {
            draw_tilemap(&mut self.batch, &cam, world);
            draw_world_entities(&mut self.batch, &cam, world);
            draw_health_bars(&mut self.batch, &cam, world);
        }
        // Crimson overlay.
        fill_screen(&mut self.batch, self.viewport, [0.20, 0.00, 0.00, 0.6]);
        font::draw_text_centered(
            &mut self.batch,
            b"VOUS ETES MORT",
            self.viewport[0] * 0.5,
            self.viewport[1] * 0.35,
            5.0,
            [0.95, 0.30, 0.25, 1.0],
            RenderLayer::UiScreen,
            0,
            100,
        );
        draw_modal(&mut self.batch, self.viewport, b"", &DEAD_ITEMS, self.dead_cursor);
    }
}

fn fill_screen(batch: &mut SpriteBatch, viewport: [f32; 2], color: [f32; 4]) {
    push_quad(batch, [0.0, 0.0], viewport, RenderLayer::UiScreen, 0, 50, color);
}

fn draw_modal(batch: &mut SpriteBatch, viewport: [f32; 2], title: &[u8], items: &[&[u8]], cursor: usize) {
    fill_screen(batch, viewport, [0.0, 0.0, 0.0, 0.62]);
    let w = 380.0;
    let h = 80.0 + items.len() as f32 * 50.0;
    let x = (viewport[0] - w) * 0.5;
    let y = (viewport[1] - h) * 0.5;
    push_quad(batch, [x, y], [w, h], RenderLayer::UiScreen, 0, 80, vertex::PANEL_BG);
    crate::ui::border(batch, [x, y, w, h], 2.0, 81, vertex::PANEL_BORDER);
    if !title.is_empty() {
        font::draw_text_centered(batch, title, x + w * 0.5, y + 16.0, 3.0, [0.95, 0.78, 0.30, 1.0], RenderLayer::UiScreen, 0, 90);
    }
    for (i, lbl) in items.iter().enumerate() {
        let ry = y + 56.0 + i as f32 * 50.0;
        let row = [x + 20.0, ry, w - 40.0, 42.0];
        let hl = i == cursor;
        push_quad(
            batch,
            [row[0], row[1]],
            [row[2], row[3]],
            RenderLayer::UiScreen,
            0,
            85,
            if hl { [0.18, 0.13, 0.05, 0.95] } else { [0.06, 0.06, 0.08, 0.9] },
        );
        crate::ui::border(batch, row, if hl { 2.0 } else { 1.0 }, 86, if hl { [0.95, 0.78, 0.30, 1.0] } else { vertex::PANEL_BORDER });
        font::draw_text_centered(
            batch,
            lbl,
            row[0] + row[2] * 0.5,
            row[1] + (row[3] - 18.0) * 0.5,
            2.5,
            if hl { [1.0, 0.95, 0.65, 1.0] } else { [0.78, 0.74, 0.65, 1.0] },
            RenderLayer::UiScreen,
            0,
            90,
        );
    }
}

/// Palette colour for a death/feedback visual kind.
fn visual_color(kind: VisualKind) -> vertex::Color {
    match kind {
        VisualKind::Melee => vertex::MOB,
        VisualKind::Ranged => vertex::MOB_RANGED,
        VisualKind::Caster => vertex::MOB_CASTER,
        VisualKind::Boss => vertex::MOB_BOSS,
        VisualKind::Player => vertex::PLAYER,
    }
}

impl ApplicationHandler for GameApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        let attrs: WindowAttributes = Window::default_attributes()
            .with_title(self.title.clone())
            .with_inner_size(PhysicalSize::new(WIN_W, WIN_H));
        let window = match event_loop.create_window(attrs) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                self.set_fatal(event_loop, anyhow::anyhow!("window create failed: {e}"));
                return;
            }
        };
        let profile =
            RenderProfile { clear_rgba: ColorRgba { r: 0.035, g: 0.035, b: 0.05, a: 1.0 } };
        let gfx = match pollster::block_on(GraphicsState::new(window.clone(), &profile)) {
            Ok(g) => g,
            Err(e) => {
                self.set_fatal(event_loop, e);
                return;
            }
        };
        self.window_id = Some(window.id());
        self.window = Some(window);
        self.gfx = Some(gfx);
        self.last_frame = Instant::now();

        #[cfg(feature = "netcode")]
        if self.auto_enter_lobby_on_resume {
            self.auto_enter_lobby_on_resume = false;
            self.enter_lobby();
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if Some(window_id) != self.window_id {
            return;
        }
        match event {
            WindowEvent::CloseRequested => {
                if (self.state == GameState::Playing || self.state == GameState::Paused)
                    && !self.is_online()
                {
                    self.perform_autosave(0);
                }
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(g) = &mut self.gfx {
                    g.resize(size);
                }
                self.viewport = [size.width as f32, size.height as f32];
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse = [position.x as f32, position.y as f32];
                match self.state {
                    GameState::TitleScreen => self.title_screen.hover(self.mouse),
                    _ => {}
                }
            }
            WindowEvent::MouseInput { state: ElementState::Pressed, button, .. } => match button {
                MouseButton::Left => self.handle_left_press(),
                MouseButton::Right => {
                    if self.state == GameState::Playing {
                        self.on_right_click();
                    }
                }
                _ => {}
            },
            WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                if self.state == GameState::Playing {
                    crate::ui::handle_left_release(
                        &mut self.sim.as_mut().unwrap().world,
                        &mut self.ui,
                        self.viewport,
                        self.mouse,
                    );
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if self.state == GameState::Playing {
                    let dy = match delta {
                        MouseScrollDelta::LineDelta(_, y) => y,
                        MouseScrollDelta::PixelDelta(p) => p.y as f32 * 0.05,
                    };
                    if let Some(sim) = self.sim_mut() {
                        if dy > 0.0 {
                            sim.world.camera.zoom_in(ZOOM_STEP);
                        } else if dy < 0.0 {
                            sim.world.camera.zoom_out(ZOOM_STEP);
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state != ElementState::Pressed {
                    return;
                }
                self.handle_key(event_loop, event);
            }
            WindowEvent::RedrawRequested => self.frame(event_loop),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }
}


fn nearest_living_mob(sim: &App, from: Entity) -> Option<Entity> {
    let fpos = *sim.world.positions.get(&from)?;
    sim.world
        .mob_entities()
        .into_iter()
        .filter(|e| sim.world.health.get(e).is_some_and(|h| !h.is_empty()))
        .filter_map(|e| sim.world.positions.get(&e).map(|p| (e, fpos.distance_sq(*p))))
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e)
}

/// Return which row of the centred modal is under `mouse`.
fn modal_row_at(viewport: [f32; 2], n: usize, mouse: [f32; 2]) -> Option<usize> {
    let w = 380.0;
    let h = 80.0 + n as f32 * 50.0;
    let x = (viewport[0] - w) * 0.5;
    let y = (viewport[1] - h) * 0.5;
    for i in 0..n {
        let ry = y + 56.0 + i as f32 * 50.0;
        let row = [x + 20.0, ry, w - 40.0, 42.0];
        if crate::ui::point_in(row, mouse) {
            return Some(i);
        }
    }
    None
}

/// Create the window/event loop and run the game to completion.
pub fn run_game(sim: Option<App>, title: impl Into<String>, forced_class: Option<ClassType>) -> Result<()> {
    let mut app = GameApp::new(sim, title, forced_class);
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut app)?;
    if let Some(err) = app.fatal.take() {
        return Err(err);
    }
    Ok(())
}

/// Same as [`run_game`], but the app boots straight into the multiplayer lobby
/// after creating its window — used by the `--connect` CLI flag.
#[cfg(feature = "netcode")]
pub fn run_game_connect(title: impl Into<String>, class: ClassType) -> Result<()> {
    let mut app = GameApp::new(None, title, Some(class));
    app.auto_enter_lobby_on_resume = true;
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut app)?;
    if let Some(err) = app.fatal.take() {
        return Err(err);
    }
    Ok(())
}

// Allow callers to ignore the optional SaveData parameter in tests.
#[allow(dead_code)]
fn _save_marker(_: Option<SaveData>) {}
