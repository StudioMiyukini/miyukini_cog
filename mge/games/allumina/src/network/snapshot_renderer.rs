//! @id     games_allumina_network_snapshot_renderer
//! @do     decode_offhand
//! @role   ui
//! @layer  core
//! @human  games allumina — module « network snapshot renderer ».

//! Snapshot → render bridge for the online multiplayer client.
//!
//! In online mode the simulation runs on the server: the client never ticks an
//! [`crate::ecs::World`], it receives [`crate::network::protocol::WorldSnapshot`]
//! frames at the server's tick rate and renders the *latest two* of them with
//! smooth interpolation between, so the 20 Hz network rate doesn't translate to
//! 20 fps visual updates.
//!
//! [`RemoteWorld`] is the mutable client-side mirror:
//! * `apply_snapshot` ingests one tick (a list of `EntitySnapshot`s + ground
//!   items) — keyed by entity id, it remembers each entity's last and current
//!   position/HP so we can lerp between them.
//! * `tick_animations` advances the local procedural animation controllers so
//!   limbs/bobs feel alive even without movement data.
//! * `interpolated_pos` samples one entity's render position at `now`.
//! * `draw` renders the whole remote scene (figures + name plates + HP bars +
//!   ground items) using the same primitives the offline renderer uses.

use std::collections::{HashMap, VecDeque};

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::player::ClassType;
use crate::components::summon::SummonType;
use crate::components::Position2D;
use crate::render::animation::{AnimState, AnimationController};
use crate::render::camera::Camera2D;
use crate::render::figures::{self, Direction, FigureStyle, Shape, Weapon};
use crate::render::font;
use crate::render::sprite::{self, push_quad, y_sort, BOSS_H, BOSS_W, FIGURE_H, FIGURE_W, ITEM_SIZE};
use crate::render::vertex::{self, Color};

use super::protocol::{EntityKind, EntitySnapshot, GroundItemSnapshot, OFFHAND_SHIELD};

/// Decode a snapshot off-hand byte into `(off-hand weapon, has_shield)`.
fn decode_offhand(off: Option<u8>) -> (Option<Weapon>, bool) {
    match off {
        None => (None, false),
        Some(OFFHAND_SHIELD) => (None, true),
        Some(c) => (Some(Weapon::from_code(c)), false),
    }
}

/// Server tick interval used to lerp between two snapshots (1 / 20 Hz).
pub const SNAPSHOT_INTERVAL: f64 = 1.0 / 20.0;
/// Hard cap on the chat backlog the HUD scrolls.
const CHAT_MAX: usize = 60;

/// One replicated entity, with enough state to draw it with interpolation and
/// run procedural animation locally.
#[derive(Debug, Clone)]
pub struct RemoteEntity {
    pub id: u64,
    pub kind: EntityKind,
    /// Position at the previous snapshot (`prev_t`).
    pub prev_x: f32,
    pub prev_y: f32,
    /// Position at the latest snapshot (`last_t`).
    pub x: f32,
    pub y: f32,
    pub hp: f64,
    pub hp_max: f64,
    pub name: String,
    pub direction: Direction,
    pub class: Option<ClassType>,
    pub level: Option<u32>,
    /// Replicated main-hand weapon silhouette (players & mercenaries).
    pub weapon: Option<Weapon>,
    /// Replicated off-hand weapon (dual-wield); `None` if empty or a shield.
    pub offhand: Option<Weapon>,
    /// Whether the off-hand carries a shield.
    pub has_shield: bool,
    /// Procedural animation driver (idle/walk bob, attack lunge, hit flash).
    pub anim: AnimationController,
    /// Last anim_state we received over the wire (kept so we don't re-trigger
    /// transient states every snapshot).
    last_anim_byte: u8,
    /// Snapshot timestamp the `prev_x`/`prev_y` were captured at.
    pub prev_t: f64,
    /// Snapshot timestamp the current `x`/`y` were captured at.
    pub last_t: f64,
    /// `true` if this is the local player (rendered from prediction, not interp).
    pub is_self: bool,
}

impl RemoteEntity {
    fn from_snapshot(snap: &EntitySnapshot, now: f64, is_self: bool) -> Self {
        let dir = direction_from_byte(snap.direction);
        let class = snap.class.and_then(class_from_byte);
        let (offhand, has_shield) = decode_offhand(snap.offhand);
        let mut anim = AnimationController::new();
        anim.state = anim_state_from_byte(snap.anim_state);
        Self {
            id: snap.id,
            kind: snap.kind,
            prev_x: snap.x,
            prev_y: snap.y,
            x: snap.x,
            y: snap.y,
            hp: snap.hp,
            hp_max: snap.hp_max,
            name: snap.name.clone(),
            direction: dir,
            class,
            level: snap.level,
            weapon: snap.weapon.map(Weapon::from_code),
            offhand,
            has_shield,
            anim,
            last_anim_byte: snap.anim_state,
            prev_t: now,
            last_t: now,
            is_self,
        }
    }

    fn update_from_snapshot(&mut self, snap: &EntitySnapshot, now: f64) {
        self.prev_x = self.x;
        self.prev_y = self.y;
        self.prev_t = self.last_t;
        self.x = snap.x;
        self.y = snap.y;
        self.last_t = now;
        self.hp = snap.hp;
        self.hp_max = snap.hp_max;
        self.direction = direction_from_byte(snap.direction);
        self.kind = snap.kind;
        if !snap.name.is_empty() {
            self.name = snap.name.clone();
        }
        self.level = snap.level.or(self.level);
        self.class = snap.class.and_then(class_from_byte).or(self.class);
        if snap.weapon.is_some() {
            self.weapon = snap.weapon.map(Weapon::from_code);
        }
        let (offhand, has_shield) = decode_offhand(snap.offhand);
        self.offhand = offhand;
        self.has_shield = has_shield;

        // Trigger transient states only on a *change* in anim_state. Walk/idle
        // are driven by movement deltas below.
        if snap.anim_state != self.last_anim_byte {
            self.last_anim_byte = snap.anim_state;
            match anim_state_from_byte(snap.anim_state) {
                AnimState::Attack => self.anim.trigger(AnimState::Attack, false),
                AnimState::Cast => self.anim.trigger(AnimState::Cast, false),
                AnimState::Hit => self.anim.trigger(AnimState::Hit, false),
                AnimState::Death => self.anim.trigger(AnimState::Death, false),
                _ => {}
            }
        }
        // Locomotion: walking if we moved this snapshot (cheap heuristic).
        let dx = self.x - self.prev_x;
        let dy = self.y - self.prev_y;
        let moved = (dx * dx + dy * dy).sqrt() > 1e-3;
        self.anim.set_locomotion(moved);
    }

    /// Interpolated position for render time `now`.
    pub fn interpolated_pos(&self, now: f64) -> Position2D {
        if self.last_t <= self.prev_t {
            return Position2D::new(self.x, self.y);
        }
        // Render slightly behind to ensure we always interpolate between known
        // snapshots — fall back to the latest if we'd be past it.
        let span = (self.last_t - self.prev_t).max(1e-6);
        #[allow(clippy::cast_possible_truncation)]
        let t = ((now - self.prev_t) / span).clamp(0.0, 1.0) as f32;
        Position2D::new(self.prev_x + (self.x - self.prev_x) * t, self.prev_y + (self.y - self.prev_y) * t)
    }
}

/// One replicated ground item.
#[derive(Debug, Clone)]
pub struct RemoteGroundItem {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub name: String,
    pub rarity: u8,
}

/// Client-side mirror of the server world.
#[derive(Debug, Default)]
pub struct RemoteWorld {
    pub entities: HashMap<u64, RemoteEntity>,
    pub ground_items: HashMap<u64, RemoteGroundItem>,
    /// Entity id of the local player (matches `JoinedGame::your_entity_id`).
    pub self_id: Option<u64>,
    /// Predicted local position for the self entity (driven by the predictor).
    pub self_predicted: Option<Position2D>,
    /// Chat / event log rolling buffer.
    pub chat: VecDeque<String>,
    /// Last measured round-trip latency in ms.
    pub ping_ms: Option<u64>,
    /// Last server tick we ingested.
    pub last_tick: u64,
}

impl RemoteWorld {
    pub fn new() -> Self {
        Self::default()
    }

    /// Replace local player id (called on JoinedGame).
    pub fn set_self_id(&mut self, id: u64) {
        self.self_id = Some(id);
        if let Some(e) = self.entities.get_mut(&id) {
            e.is_self = true;
        }
    }

    /// Push one log/chat line, respecting the rolling cap.
    pub fn push_chat(&mut self, line: impl Into<String>) {
        self.chat.push_back(line.into());
        while self.chat.len() > CHAT_MAX {
            self.chat.pop_front();
        }
    }

    /// Apply one `WorldSnapshot` to the mirror at absolute time `now` (seconds).
    pub fn apply_snapshot(
        &mut self,
        now: f64,
        tick: u64,
        entities: &[EntitySnapshot],
        ground_items: &[GroundItemSnapshot],
    ) {
        self.last_tick = tick;

        // Mark every known entity as missing; we'll un-mark the ones present in
        // this snapshot, and drop the rest at the end. Players/mobs that
        // disappear out of cull range should fade out naturally — but we don't
        // model that, so we just remove.
        let prev_ids: Vec<u64> = self.entities.keys().copied().collect();
        let mut still_present: HashMap<u64, ()> = HashMap::with_capacity(entities.len());

        for snap in entities {
            still_present.insert(snap.id, ());
            let is_self = Some(snap.id) == self.self_id;
            match self.entities.get_mut(&snap.id) {
                Some(e) => e.update_from_snapshot(snap, now),
                None => {
                    self.entities.insert(snap.id, RemoteEntity::from_snapshot(snap, now, is_self));
                }
            }
        }
        for id in prev_ids {
            if !still_present.contains_key(&id) {
                self.entities.remove(&id);
            }
        }

        // Ground items: server fully replaces the list (it's the authoritative
        // view), so we mirror by recomputing the map.
        self.ground_items.clear();
        for gi in ground_items {
            self.ground_items.insert(
                gi.id,
                RemoteGroundItem { id: gi.id, x: gi.x, y: gi.y, name: gi.name.clone(), rarity: gi.rarity },
            );
        }
    }

    /// Drop an entity (server told us it died, before the next snapshot).
    pub fn entity_died(&mut self, id: u64) {
        if let Some(e) = self.entities.get_mut(&id) {
            e.anim.trigger(AnimState::Death, false);
        }
    }

    /// Insert a freshly spawned entity (between snapshots).
    pub fn entity_spawned(&mut self, snap: &EntitySnapshot, now: f64) {
        let is_self = Some(snap.id) == self.self_id;
        self.entities.insert(snap.id, RemoteEntity::from_snapshot(snap, now, is_self));
    }

    /// Advance every controller's local clock by `dt` seconds.
    pub fn tick_animations(&mut self, dt: f32) {
        for e in self.entities.values_mut() {
            e.anim.tick(dt);
        }
    }

    /// Update the predicted position used to render the local player.
    pub fn set_self_predicted(&mut self, pos: Position2D) {
        self.self_predicted = Some(pos);
    }

    /// Tile to focus the camera on this frame: predicted self when known,
    /// otherwise the interpolated self position, otherwise origin.
    pub fn camera_focus(&self, now: f64) -> Position2D {
        if let Some(p) = self.self_predicted {
            return p;
        }
        if let Some(id) = self.self_id {
            if let Some(e) = self.entities.get(&id) {
                return e.interpolated_pos(now);
            }
        }
        Position2D::new(0.0, 0.0)
    }
}

// ── Rendering ───────────────────────────────────────────────────────────────

/// Draw every replicated entity + ground item in `remote` for the given camera.
pub fn draw_remote_world(
    batch: &mut SpriteBatch,
    cam: &Camera2D,
    remote: &RemoteWorld,
    now: f64,
) {
    // Ground items: a glowing yellow cube + name plate (no instance metadata,
    // so we render a simple icon — the server is authoritative on the actual
    // item identity, visual richness can come later).
    for gi in remote.ground_items.values() {
        let pos = Position2D::new(gi.x, gi.y);
        let screen = cam.world_to_screen(pos);
        if !cam.on_screen(screen, 64.0) {
            continue;
        }
        let size = ITEM_SIZE * cam.zoom;
        let color = rarity_color(gi.rarity);
        push_quad(
            batch,
            [screen[0] - size * 0.5, screen[1] - size * 0.5],
            [size, size],
            RenderLayer::Entities,
            y_sort(pos),
            (gi.id & 0x7FFF_FFFF) as u32,
            color,
        );
    }

    // Entities: sort by depth (y_sort) is handled by the SpriteBatch.
    for e in remote.entities.values() {
        let pos = if e.is_self {
            // Local player uses predicted/snapshot position directly (the
            // predictor already smooths it). Falls back to interp if predictor
            // hasn't kicked in yet.
            remote.self_predicted.unwrap_or_else(|| e.interpolated_pos(now))
        } else {
            e.interpolated_pos(now)
        };

        let screen = cam.world_to_screen(pos);
        if !cam.on_screen(screen, 160.0) {
            continue;
        }

        let (style, base) = style_for(e);
        let sample = e.anim.sample();
        figures::draw(batch, cam, pos, base, &style, &sample, e.direction, (e.id & 0x7FFF_FFFF) as u32, cam.time);

        // Name + level above the head (other players only — mobs/summons would
        // clutter the screen).
        if matches!(e.kind, EntityKind::Player) && !e.is_self {
            draw_player_label(batch, cam, pos, &e.name, e.level, base[1]);
        }

        // HP bar for any hostile that's alive, and for other players.
        let alive = e.hp > 0.0 && !matches!(e.anim.state, AnimState::Death);
        if alive {
            let hostile = matches!(e.kind, EntityKind::Mob | EntityKind::Boss);
            let other_player = matches!(e.kind, EntityKind::Player) && !e.is_self;
            if hostile || other_player {
                let frac = (e.hp / e.hp_max.max(1.0)).clamp(0.0, 1.0) as f32;
                let fill = if other_player {
                    [0.30, 0.85, 0.30, 1.0]
                } else if matches!(e.kind, EntityKind::Boss) {
                    [0.95, 0.20, 0.20, 1.0]
                } else {
                    vertex::lerp([0.85, 0.15, 0.12, 1.0], [0.30, 0.85, 0.30, 1.0], frac)
                };
                draw_hp_bar(batch, cam, pos, base[0], base[1], frac, fill);
            }
        }
    }
}

/// Pick a [`FigureStyle`] + base figure size for one remote entity.
fn style_for(e: &RemoteEntity) -> (FigureStyle, [f32; 2]) {
    match e.kind {
        EntityKind::Player => {
            let mut s = sprite::class_style(e.class.unwrap_or(ClassType::Soldat), vertex::SKIN);
            apply_loadout(&mut s, e);
            (s, [FIGURE_W, FIGURE_H])
        }
        EntityKind::Boss => (sprite::boss_style(), [BOSS_W, BOSS_H]),
        EntityKind::Mob => (mob_style(), [FIGURE_W, FIGURE_H]),
        EntityKind::Summon => {
            (sprite::summon_style(SummonType::Skeleton), [FIGURE_W, FIGURE_H])
        }
        EntityKind::Mercenary => {
            // Build the archetype's body (from the replicated class), tint it the
            // companion green, then apply the replicated weapon / shield loadout.
            let mut s = sprite::class_style(e.class.unwrap_or(ClassType::Soldat), vertex::SKIN);
            s.primary = vertex::lerp(s.primary, vertex::MERC, 0.55);
            apply_loadout(&mut s, e);
            (s, [FIGURE_W, FIGURE_H])
        }
        EntityKind::Npc => (npc_style(), [FIGURE_W, FIGURE_H]),
    }
}

/// Overlay a remote entity's replicated weapon / off-hand onto its base style.
fn apply_loadout(s: &mut FigureStyle, e: &RemoteEntity) {
    if let Some(w) = e.weapon {
        s.weapon = w;
    }
    s.offhand = e.offhand;
    if e.has_shield {
        s.shield = Some([0.55, 0.45, 0.28, 1.0]);
    }
}

fn mob_style() -> FigureStyle {
    let mut s = FigureStyle::humanoid(vertex::ZOMBIE, [0.46, 0.55, 0.32, 1.0], Weapon::Bone);
    s.build = 1.22;
    s.eyes = Some([0.85, 0.95, 0.35, 1.0]);
    s
}

fn npc_style() -> FigureStyle {
    let mut s = FigureStyle::humanoid([0.20, 0.55, 0.55, 1.0], vertex::SKIN, Weapon::None);
    s.shape = Shape::Soldier;
    s
}

#[allow(clippy::cast_possible_truncation)]
fn draw_player_label(
    batch: &mut SpriteBatch,
    cam: &Camera2D,
    pos: Position2D,
    name: &str,
    level: Option<u32>,
    fig_h: f32,
) {
    let s = cam.world_to_screen(pos);
    let cy = s[1] - fig_h * cam.zoom - 26.0;
    let ys = y_sort(pos) + 700;
    let px = (1.6 * cam.zoom).max(1.0);
    let buf = if let Some(lvl) = level {
        format!("{name} [{lvl}]")
    } else {
        name.to_string()
    };
    font::draw_text_centered(
        batch,
        buf.as_bytes(),
        s[0],
        cy,
        px,
        [0.55, 0.95, 0.55, 1.0],
        RenderLayer::UiWorld,
        ys,
        4,
    );
}

#[allow(clippy::cast_possible_truncation)]
fn draw_hp_bar(
    batch: &mut SpriteBatch,
    cam: &Camera2D,
    pos: Position2D,
    fig_w: f32,
    fig_h: f32,
    frac: f32,
    fill: Color,
) {
    let s = cam.world_to_screen(pos);
    let bw = fig_w * cam.zoom;
    let bh = (4.0_f32).max(3.0) * cam.zoom.max(0.6);
    let x = s[0] - bw * 0.5;
    let y = s[1] - fig_h * cam.zoom - bh - 3.0;
    let ys = y_sort(pos) + 600;
    push_quad(batch, [x - 1.0, y - 1.0], [bw + 2.0, bh + 2.0], RenderLayer::UiWorld, ys, 0, vertex::BLACK);
    push_quad(batch, [x, y], [bw, bh], RenderLayer::UiWorld, ys, 1, [0.20, 0.05, 0.05, 1.0]);
    push_quad(batch, [x, y], [bw * frac, bh], RenderLayer::UiWorld, ys, 2, fill);
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn direction_from_byte(b: u8) -> Direction {
    match b {
        0 => Direction::N,
        1 => Direction::NE,
        2 => Direction::E,
        3 => Direction::SE,
        4 => Direction::S,
        5 => Direction::SW,
        6 => Direction::W,
        7 => Direction::NW,
        _ => Direction::Idle,
    }
}

fn anim_state_from_byte(b: u8) -> AnimState {
    match b {
        1 => AnimState::Walk,
        2 => AnimState::Attack,
        3 => AnimState::Cast,
        4 => AnimState::Hit,
        5 => AnimState::Death,
        _ => AnimState::Idle,
    }
}

fn class_from_byte(b: u8) -> Option<ClassType> {
    Some(match b {
        0 => ClassType::Soldat,
        1 => ClassType::Clerc,
        2 => ClassType::MageDeGuerre,
        3 => ClassType::Archer,
        _ => return None,
    })
}

fn rarity_color(r: u8) -> Color {
    match r {
        0 => [0.85, 0.85, 0.85, 1.0], // Normal
        1 => [0.45, 0.65, 1.0, 1.0],  // Magic
        2 => [1.0, 0.95, 0.30, 1.0],  // Rare
        3 => [0.55, 0.95, 1.0, 1.0],  // Crafted
        4 => [0.30, 0.95, 0.55, 1.0], // Set
        _ => [0.95, 0.55, 0.20, 1.0], // Unique
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::protocol::{EntityKind, EntitySnapshot, GroundItemSnapshot};

    fn make_snap(id: u64, x: f32, y: f32, kind: EntityKind) -> EntitySnapshot {
        EntitySnapshot {
            id,
            kind,
            x,
            y,
            hp: 50.0,
            hp_max: 100.0,
            name: format!("E{id}"),
            direction: 4,
            anim_state: 0,
            class: None,
            level: None,
            weapon: None,
            offhand: None,
        }
    }

    #[test]
    fn apply_snapshot_inserts_new_entities() {
        let mut w = RemoteWorld::new();
        let entities = vec![
            make_snap(1, 1.0, 1.0, EntityKind::Player),
            make_snap(2, 2.0, 2.0, EntityKind::Mob),
        ];
        w.apply_snapshot(0.0, 1, &entities, &[]);
        assert_eq!(w.entities.len(), 2);
        assert!(w.entities.contains_key(&1));
        assert!(w.entities.contains_key(&2));
    }

    #[test]
    fn apply_snapshot_removes_missing_entities() {
        let mut w = RemoteWorld::new();
        w.apply_snapshot(0.0, 1, &[make_snap(1, 0.0, 0.0, EntityKind::Mob)], &[]);
        w.apply_snapshot(0.1, 2, &[make_snap(2, 0.0, 0.0, EntityKind::Mob)], &[]);
        assert!(!w.entities.contains_key(&1));
        assert!(w.entities.contains_key(&2));
    }

    #[test]
    fn interpolation_midpoint_between_snapshots() {
        let mut w = RemoteWorld::new();
        // Snapshot at t=0.0 at (0, 0).
        w.apply_snapshot(0.0, 1, &[make_snap(7, 0.0, 0.0, EntityKind::Mob)], &[]);
        // Snapshot at t=1.0 at (10, 0).
        w.apply_snapshot(1.0, 2, &[make_snap(7, 10.0, 0.0, EntityKind::Mob)], &[]);
        let p = w.entities[&7].interpolated_pos(0.5);
        // After two snapshots: prev_t=0.0 / x=0, last_t=1.0 / x=10
        // Sampling at now=0.5 → midpoint.
        assert!((p.x - 5.0).abs() < 1e-3, "got {}", p.x);
    }

    #[test]
    fn interpolation_clamps_outside_range() {
        let mut w = RemoteWorld::new();
        w.apply_snapshot(0.0, 1, &[make_snap(7, 0.0, 0.0, EntityKind::Mob)], &[]);
        w.apply_snapshot(1.0, 2, &[make_snap(7, 10.0, 0.0, EntityKind::Mob)], &[]);
        // Beyond `last_t` → clamps to current position.
        let p = w.entities[&7].interpolated_pos(5.0);
        assert!((p.x - 10.0).abs() < 1e-3);
        // Before `prev_t` → clamps to previous position.
        let p2 = w.entities[&7].interpolated_pos(-2.0);
        assert!((p2.x - 0.0).abs() < 1e-3);
    }

    #[test]
    fn self_predicted_takes_priority_over_interp() {
        let mut w = RemoteWorld::new();
        w.set_self_id(42);
        w.apply_snapshot(0.0, 1, &[make_snap(42, 0.0, 0.0, EntityKind::Player)], &[]);
        w.apply_snapshot(1.0, 2, &[make_snap(42, 10.0, 0.0, EntityKind::Player)], &[]);
        w.set_self_predicted(Position2D::new(7.5, 0.0));
        let focus = w.camera_focus(0.5);
        assert!((focus.x - 7.5).abs() < 1e-3);
    }

    #[test]
    fn camera_focus_defaults_when_no_self() {
        let w = RemoteWorld::new();
        let focus = w.camera_focus(0.0);
        assert_eq!(focus.x, 0.0);
        assert_eq!(focus.y, 0.0);
    }

    #[test]
    fn ground_items_mirror_authoritative_list() {
        let mut w = RemoteWorld::new();
        let items = vec![GroundItemSnapshot {
            id: 99,
            x: 3.0,
            y: 3.0,
            name: "Sword".into(),
            rarity: 1,
            kind: "Sword".into(),
        }];
        w.apply_snapshot(0.0, 1, &[], &items);
        assert_eq!(w.ground_items.len(), 1);
        w.apply_snapshot(0.05, 2, &[], &[]);
        assert!(w.ground_items.is_empty());
    }

    #[test]
    fn entity_died_triggers_death_anim() {
        let mut w = RemoteWorld::new();
        w.apply_snapshot(0.0, 1, &[make_snap(3, 0.0, 0.0, EntityKind::Mob)], &[]);
        w.entity_died(3);
        assert_eq!(w.entities[&3].anim.state, AnimState::Death);
    }

    #[test]
    fn moving_entity_enters_walk_animation() {
        let mut w = RemoteWorld::new();
        w.apply_snapshot(0.0, 1, &[make_snap(5, 0.0, 0.0, EntityKind::Mob)], &[]);
        w.apply_snapshot(0.05, 2, &[make_snap(5, 1.0, 0.0, EntityKind::Mob)], &[]);
        assert_eq!(w.entities[&5].anim.state, AnimState::Walk);
    }

    #[test]
    fn chat_buffer_caps_at_max() {
        let mut w = RemoteWorld::new();
        for i in 0..(CHAT_MAX + 5) {
            w.push_chat(format!("line {i}"));
        }
        assert_eq!(w.chat.len(), CHAT_MAX);
        // First few lines have been evicted.
        assert!(w.chat.front().unwrap().starts_with("line 5"));
    }

    /// The serialization path itself is exercised in `protocol::tests`. This
    /// regression ensures `apply_snapshot` survives an EntitySnapshot that
    /// round-tripped through bincode (e.g. a non-UTF8-clean name still decodes
    /// because we use `String`).
    #[test]
    fn snapshot_round_trip_then_apply() {
        use crate::network::protocol::{decode, encode, ServerMessage};
        let msg = ServerMessage::WorldSnapshot {
            tick: 4,
            entities: vec![make_snap(1, 1.5, 2.5, EntityKind::Player)],
            ground_items: vec![],
            effects: vec![],
        };
        let bytes = encode(&msg).unwrap();
        let back: ServerMessage = decode(&bytes).unwrap();
        let ServerMessage::WorldSnapshot { tick, entities, ground_items, .. } = back else {
            panic!("expected WorldSnapshot")
        };
        let mut w = RemoteWorld::new();
        w.apply_snapshot(0.0, tick, &entities, &ground_items);
        assert!(w.entities.contains_key(&1));
        assert_eq!(w.entities[&1].x, 1.5);
    }
}
