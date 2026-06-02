//! Sprite drawing: places world entities into the batch as tinted quads,
//! depth-sorted by isometric Y so nearer entities overdraw farther ones.
//!
//! With no art assets the placeholders are solid colours (player = blue,
//! mobs = red/orange/magenta by behaviour, boss = crimson, items = yellow,
//! tinted by rarity). The actual sort happens in `SpriteBatch::sort` using the
//! `(layer, y_sort, sub_order)` key built here.

use mge_render::batch::{RenderLayer, SortKey, SpriteBatch, SpriteInstance};

use crate::components::mercenary::MercType;
use crate::components::mob::AiBehavior;
use crate::components::npc::NpcKind;
use crate::components::player::ClassType;
use crate::components::summon::SummonType;
use crate::components::Position2D;
use crate::data::mobs::MobDef;
use crate::ecs::{Entity, World};

use super::animation::AnimSample;
use super::camera::Camera2D;
use super::figures::{self, Direction, FigureStyle, Shape, Weapon};
use super::font;
use super::item_icon;
use super::shapes;
use super::vertex::{self, Color};
use super::{ATLAS, MAT, UNIT};

/// Quantise an isometric world position to a stable depth key. Larger
/// `x + y` is closer to the camera (drawn later / on top).
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn y_sort(p: Position2D) -> i32 {
    ((p.x + p.y) * 100.0) as i32
}

/// Push one axis-aligned quad anchored at a *screen* point (top-left origin).
#[allow(clippy::too_many_arguments)]
pub fn push_quad(
    batch: &mut SpriteBatch,
    screen_pos: [f32; 2],
    size: [f32; 2],
    layer: RenderLayer,
    y_sort: i32,
    sub: u32,
    tint: Color,
) {
    batch.push(SpriteInstance {
        screen_pos,
        src: UNIT,
        atlas: ATLAS,
        material: MAT,
        sort_key: SortKey { layer, y_sort, sub_order: sub },
        tint,
        scale: size,
    });
}

/// Base draw size (in unscaled pixels) for a character figure.
pub const FIGURE_W: f32 = 26.0;
pub const FIGURE_H: f32 = 40.0;
/// Base draw size for a boss figure.
pub const BOSS_W: f32 = 46.0;
pub const BOSS_H: f32 = 70.0;
/// Base draw size for a ground item.
pub const ITEM_SIZE: f32 = 14.0;

/// Colour for a mob given its behaviour and boss flag.
#[must_use]
pub fn mob_color(behavior: AiBehavior, is_boss: bool) -> Color {
    if is_boss {
        return vertex::MOB_BOSS;
    }
    match behavior {
        AiBehavior::Melee => vertex::MOB,
        AiBehavior::Ranged => vertex::MOB_RANGED,
        AiBehavior::Caster => vertex::MOB_CASTER,
    }
}

/// Sample an entity's animation pose, or a neutral standing pose if it has no
/// controller (e.g. NPCs).
fn anim_sample(world: &World, e: Entity) -> AnimSample {
    world.animations.get(&e).map_or_else(AnimSample::still, super::animation::AnimationController::sample)
}

/// Figure style for the player, by class (silhouette + body colour + weapon).
fn player_style(world: &World, player: Entity) -> FigureStyle {
    let class = world.classes.get(&player).copied().unwrap_or(ClassType::Soldat);
    let mut s = class_style(class, vertex::SKIN);
    // Dual-wield: show both equipped one-handed weapons (main + mirrored off).
    if crate::systems::combat::is_dual_wield_melee(world, player) {
        if let Some(eq) = world.equipment.get(&player) {
            use mge_items::item::EquipSlot;
            if let Some(w) = eq.get(EquipSlot::MainHand) {
                s.weapon = Weapon::for_item_kind(w.item.base.kind);
            }
            s.offhand = eq.get(EquipSlot::OffHand).map(|w| Weapon::for_item_kind(w.item.base.kind));
        }
    }
    s
}

/// The gilded boss silhouette, shared by the native renderer and the network
/// snapshot renderer so the boss reads identically in solo and multiplayer.
pub(crate) fn boss_style() -> FigureStyle {
    let mut s = FigureStyle::humanoid(vertex::MOB_BOSS, [0.42, 0.10, 0.12, 1.0], Weapon::Bone);
    s.build = 1.45;
    s.aura = Some([0.95, 0.12, 0.10, 1.0]);
    s.outline = Some(vertex::rarity_color("gold"));
    s.eyes = Some([1.0, 0.9, 0.3, 1.0]);
    s
}

/// Build the class-specific silhouette (shared by the player and class-themed
/// mercenaries), using the given skin tone. Public to the crate so the network
/// snapshot renderer reuses the exact same silhouette for remote players/mercs.
pub(crate) fn class_style(class: ClassType, skin: vertex::Color) -> FigureStyle {
    match class {
        ClassType::Soldat => {
            let mut s = FigureStyle::humanoid(vertex::CLASS_SOLDAT, skin, Weapon::Sword);
            s.shape = Shape::Soldier;
            s.build = 1.25;
            s
        }
        ClassType::Clerc => {
            let mut s = FigureStyle::humanoid(vertex::CLASS_CLERC, skin, Weapon::Scepter);
            s.shape = Shape::Cleric;
            s.aura = Some([1.0, 0.95, 0.6, 1.0]);
            s
        }
        ClassType::MageDeGuerre => {
            let mut s = FigureStyle::humanoid(vertex::CLASS_MAGE, skin, Weapon::Orb);
            s.shape = Shape::Warmage;
            s
        }
        ClassType::Archer => {
            let mut s = FigureStyle::humanoid(vertex::CLASS_ARCHER, skin, Weapon::Bow);
            s.shape = Shape::Archer;
            s.build = 0.9;
            s
        }
    }
}

/// Figure style for a mercenary, by archetype: a class silhouette tinted toward
/// the light-green companion palette.
fn merc_style(world: &World, e: Entity, merc_type: MercType) -> FigureStyle {
    let class = match merc_type {
        MercType::Guardian => ClassType::Soldat,
        MercType::Archer => ClassType::Archer,
        MercType::Healer => ClassType::Clerc,
        MercType::Mage => ClassType::MageDeGuerre,
    };
    let mut s = class_style(class, vertex::SKIN);
    // Tint the body toward the companion green so friend/foe read at a glance.
    s.primary = vertex::lerp(s.primary, vertex::MERC, 0.55);

    // Reflect the merc's actual off-hand: a shield, or a second weapon when it
    // dual-wields (the Guardian), mirroring the player's paperdoll-driven look.
    use mge_items::item::{EquipSlot, ItemKind};
    if let Some(inv) = world.merc_inventory.get(&e) {
        if crate::systems::combat::is_dual_wield_melee(world, e) {
            if let Some(w) = inv.get(EquipSlot::MainHand) {
                s.weapon = Weapon::for_item_kind(w.item.base.kind);
            }
            s.offhand = inv.get(EquipSlot::OffHand).map(|w| Weapon::for_item_kind(w.item.base.kind));
        } else if inv.get(EquipSlot::OffHand).map(|w| w.item.base.kind) == Some(ItemKind::Shield) {
            s.shield = Some([0.55, 0.45, 0.28, 1.0]);
        }
    }
    s
}

/// Figure style for a hostile mob, mapping behaviour + bulk onto skeleton /
/// zombie / stone-golem / robed-caster archetypes (or the gilded boss).
fn mob_style(behavior: AiBehavior, def: Option<&MobDef>) -> FigureStyle {
    if def.is_some_and(|d| d.is_boss) {
        return boss_style();
    }
    match behavior {
        AiBehavior::Ranged => {
            // Skeletal archer: lean, bone-white, red pinprick eyes.
            let mut s = FigureStyle::humanoid(vertex::SKEL, vertex::SKEL, Weapon::Bow);
            s.build = 0.72;
            s.eyes = Some([0.95, 0.20, 0.20, 1.0]);
            s
        }
        AiBehavior::Caster => {
            let mut s = FigureStyle::humanoid(vertex::CASTER_ROBE, vertex::SKIN, Weapon::Staff);
            s.build = 0.95;
            s.eyes = Some([0.95, 0.65, 1.0, 1.0]);
            s
        }
        AiBehavior::Melee => {
            let heavy = def.is_some_and(|d| d.defense >= 20 || d.max_hp >= 80.0);
            if heavy {
                let mut s = FigureStyle::humanoid(vertex::GOLEM, vertex::GOLEM, Weapon::None);
                s.shape = Shape::Golem;
                s.build = 1.3;
                s.eyes = Some([1.0, 0.55, 0.15, 1.0]);
                s
            } else {
                // Stocky zombie wielding a crude bone club.
                let mut s = FigureStyle::humanoid(vertex::ZOMBIE, [0.46, 0.55, 0.32, 1.0], Weapon::Bone);
                s.build = 1.22;
                s.eyes = Some([0.85, 0.95, 0.35, 1.0]);
                s
            }
        }
    }
}

/// Figure style for a summoned ally: translucent body + cyan rim + link diamond.
/// Public to the crate so the network renderer reuses the skeleton silhouette.
pub(crate) fn summon_style(summon_type: SummonType) -> FigureStyle {
    let mut s = match summon_type {
        SummonType::Skeleton => FigureStyle::humanoid(vertex::SKEL, vertex::SKEL, Weapon::Bone),
        SummonType::Footman => {
            // A lesser soldier: armoured silhouette at a smaller build.
            let mut g = FigureStyle::humanoid(vertex::CLASS_SOLDAT, vertex::SKIN, Weapon::Sword);
            g.shape = Shape::Soldier;
            g.build = 0.95;
            g
        }
        SummonType::Peasant => {
            // A small brown villager wielding an improvised hayfork.
            let mut g = FigureStyle::humanoid([0.45, 0.30, 0.16, 1.0], vertex::SKIN, Weapon::Pitchfork);
            g.build = 0.7;
            g
        }
        SummonType::Militiaman => {
            // A grey light soldier behind a shield, with a short sword.
            let mut g = FigureStyle::humanoid([0.55, 0.57, 0.62, 1.0], vertex::SKIN, Weapon::ShortSword);
            g.shape = Shape::Soldier;
            g.build = 1.0;
            g.shield = Some([0.64, 0.66, 0.72, 1.0]);
            g
        }
        SummonType::EliteWarrior => {
            // A large, dark heavy knight with broad pauldrons and a great sword.
            let mut g = FigureStyle::humanoid([0.16, 0.16, 0.20, 1.0], vertex::SKIN, Weapon::Sword);
            g.shape = Shape::Soldier;
            g.build = 1.35;
            g.eyes = Some([1.0, 0.35, 0.25, 1.0]);
            g
        }
        SummonType::GuardianAngel => {
            // A radiant robed tank with a bright holy aura.
            let mut g = FigureStyle::humanoid([0.95, 0.93, 0.78, 1.0], vertex::SKIN, Weapon::Scepter);
            g.shape = Shape::Cleric;
            g.aura = Some([1.0, 0.95, 0.65, 1.0]);
            g.build = 1.2;
            g
        }
        SummonType::Cherubin => {
            // A small bright support seraph with a soft warm glow.
            let mut g = FigureStyle::humanoid([1.0, 0.97, 0.85, 1.0], vertex::SKIN, Weapon::None);
            g.shape = Shape::Cleric;
            g.aura = Some([1.0, 0.9, 0.55, 1.0]);
            g.build = 0.55;
            g
        }
        SummonType::Archangel => {
            // A large, brilliant celestial warrior with a flaming sword.
            let mut g = FigureStyle::humanoid([1.0, 0.98, 0.88, 1.0], vertex::SKIN, Weapon::Sword);
            g.shape = Shape::Cleric;
            g.aura = Some([1.0, 0.85, 0.35, 1.0]);
            g.build = 1.4;
            g
        }
        SummonType::Elemental => {
            let mut g = FigureStyle::humanoid([0.95, 0.50, 0.15, 1.0], [1.0, 0.80, 0.35, 1.0], Weapon::Orb);
            g.shape = Shape::Warmage;
            g.eyes = Some([1.0, 0.92, 0.55, 1.0]);
            g
        }
        SummonType::Wolf => {
            let mut g = FigureStyle::humanoid([0.62, 0.72, 0.85, 1.0], [0.7, 0.8, 0.92, 1.0], Weapon::None);
            g.build = 0.8;
            g
        }
        SummonType::Totem => {
            let mut g = FigureStyle::humanoid([0.50, 0.40, 0.72, 1.0], [0.74, 0.62, 0.92, 1.0], Weapon::Staff);
            g.build = 0.7;
            g
        }
    };
    s.outline = Some(vertex::SUMMON);
    s.alpha = 0.78;
    s
}

/// Base figure size for a summon: peasants are small, militia normal, elite
/// warriors large, and totems short and squat.
fn summon_base(summon_type: SummonType, totem: bool) -> [f32; 2] {
    if totem {
        return [FIGURE_W * 0.8, FIGURE_H * 0.85];
    }
    match summon_type {
        SummonType::Peasant => [FIGURE_W * 0.78, FIGURE_H * 0.80],
        SummonType::Militiaman => [FIGURE_W * 0.95, FIGURE_H * 0.95],
        SummonType::EliteWarrior => [FIGURE_W * 1.25, FIGURE_H * 1.25],
        _ => [FIGURE_W, FIGURE_H],
    }
}

/// The facing for an entity, defaulting to a front-facing rest pose.
fn entity_dir(world: &World, e: Entity) -> Direction {
    world.directions.get(&e).copied().unwrap_or_default()
}

/// Squash a knocked-down figure flat against the ground (30% height, wider) so a
/// renversement reads at a glance. Returns `base` unchanged otherwise.
fn cc_base(world: &World, e: Entity, base: [f32; 2]) -> [f32; 2] {
    if world.knockdowns.contains_key(&e) {
        [base[0] * 1.3, base[1] * 0.3]
    } else {
        base
    }
}

/// Push every world entity (ground items, mobs, allies, the player) into the batch.
pub fn draw_world_entities(batch: &mut SpriteBatch, cam: &Camera2D, world: &World) {
    // Ground items: levitating, pulsing, rarity-lit item icons.
    for e in world.ground_items.keys().copied() {
        let Some(&pos) = world.positions.get(&e) else { continue };
        if !cam.on_screen(cam.world_to_screen(pos), 64.0) {
            continue;
        }
        let Some(g) = world.ground_items.get(&e) else { continue };
        let kind = g.instance.item.base.kind;
        let rarity = g.instance.rarity();
        item_icon::draw_ground_item(batch, cam, pos, kind, rarity, ITEM_SIZE, e.0 as u32, cam.time);
    }

    // Mobs.
    for e in world.mob_entities() {
        let Some(&pos) = world.positions.get(&e) else { continue };
        if !cam.on_screen(cam.world_to_screen(pos), 120.0) {
            continue;
        }
        let def = world.mob_types.get(&e).and_then(|m| world.data.mob(m.def_id));
        let behavior = world.aggro.get(&e).map_or(AiBehavior::Melee, |a| a.behavior);
        let base = if def.is_some_and(|d| d.is_boss) { [BOSS_W, BOSS_H] } else { [FIGURE_W, FIGURE_H] };
        let base = cc_base(world, e, base);
        let style = mob_style(behavior, def);
        figures::draw(batch, cam, pos, base, &style, &anim_sample(world, e), entity_dir(world, e), e.0 as u32, cam.time);
    }

    // Player.
    if let Some(player) = world.player_entity {
        if let Some(&pos) = world.positions.get(&player) {
            let style = player_style(world, player);
            let base = cc_base(world, player, [FIGURE_W, FIGURE_H]);
            figures::draw(batch, cam, pos, base, &style, &anim_sample(world, player), entity_dir(world, player), 0, cam.time);
        }
    }

    // Summoned allies.
    for e in world.summon_entities() {
        let Some(&pos) = world.positions.get(&e) else { continue };
        if !cam.on_screen(cam.world_to_screen(pos), 120.0) {
            continue;
        }
        let tag = world.summon_tags.get(&e);
        let summon_type = tag.map_or(SummonType::Skeleton, |t| t.summon_type);
        let style = summon_style(summon_type);
        let totem = tag.is_some_and(|t| t.profile.stationary);
        let base = cc_base(world, e, summon_base(summon_type, totem));
        figures::draw(batch, cam, pos, base, &style, &anim_sample(world, e), entity_dir(world, e), e.0 as u32, cam.time);
        draw_link_marker(batch, cam, pos);
    }

    // Mercenaries (hidden while respawning).
    for e in world.mercenary_entities() {
        if world.merc_states.get(&e).is_some_and(|st| st.is_dead()) {
            continue;
        }
        let Some(&pos) = world.positions.get(&e) else { continue };
        if !cam.on_screen(cam.world_to_screen(pos), 120.0) {
            continue;
        }
        let merc_type = world.mercenary_tags.get(&e).map_or(MercType::Guardian, |t| t.merc_type);
        let style = merc_style(world, e, merc_type);
        let base = cc_base(world, e, [FIGURE_W, FIGURE_H]);
        figures::draw(batch, cam, pos, base, &style, &anim_sample(world, e), entity_dir(world, e), e.0 as u32, cam.time);
    }

    // NPCs (village hub): kind-specific silhouette + name plate.
    for (e, kind) in &world.npcs {
        let Some(&pos) = world.positions.get(e) else { continue };
        if !cam.on_screen(cam.world_to_screen(pos), 120.0) {
            continue;
        }
        let style = npc_style(*kind);
        figures::draw(batch, cam, pos, [FIGURE_W, FIGURE_H], &style, &AnimSample::still(), Direction::S, e.0 as u32, cam.time);
        draw_npc_marker_for(batch, cam, pos, *kind);
        if let Some(info) = world.npc_info.get(e) {
            draw_npc_name(batch, cam, pos, &info.name);
        }
    }

    // Deployed traps: pulsing orange ground rings.
    for (e, trap) in &world.traps {
        let Some(&pos) = world.positions.get(e) else { continue };
        let s = cam.world_to_screen(pos);
        if !cam.on_screen(s, 64.0) {
            continue;
        }
        let pulse = 0.6 + 0.4 * (trap.life * 6.0).sin().abs();
        let r = 11.0 * cam.zoom;
        shapes::ring(batch, s[0], s[1], r, 2.5, RenderLayer::Decals, y_sort(pos), e.0 as u32, [1.0, 0.5 * pulse, 0.12, 0.9]);
        shapes::circle(batch, s[0], s[1], r * 0.4, RenderLayer::Decals, y_sort(pos), e.0 as u32, [1.0, 0.65 * pulse, 0.2, 0.8]);
    }
}

/// Kind-specific figure silhouette for a village NPC.
fn npc_style(kind: NpcKind) -> FigureStyle {
    match kind {
        NpcKind::Merchant | NpcKind::WeaponMerchant => {
            let mut s = FigureStyle::humanoid([0.20, 0.55, 0.55, 1.0], vertex::SKIN, Weapon::Sword);
            s.shape = Shape::Soldier;
            s
        }
        NpcKind::ArmorMerchant => {
            let mut s = FigureStyle::humanoid([0.48, 0.50, 0.58, 1.0], vertex::SKIN, Weapon::None);
            s.shape = Shape::Soldier;
            s.shield = Some([0.74, 0.76, 0.82, 1.0]);
            s
        }
        NpcKind::PotionMerchant => {
            let mut s = FigureStyle::humanoid([0.60, 0.32, 0.78, 1.0], vertex::SKIN, Weapon::Orb);
            s.shape = Shape::Warmage;
            s
        }
        NpcKind::Blacksmith => {
            let mut s = FigureStyle::humanoid([0.28, 0.20, 0.18, 1.0], vertex::SKIN, Weapon::Bone);
            s.shape = Shape::Soldier;
            s.build = 1.30;
            s.eyes = Some([1.0, 0.55, 0.20, 1.0]);
            s
        }
        NpcKind::Healer => {
            let mut s = FigureStyle::humanoid([0.94, 0.92, 0.78, 1.0], vertex::SKIN, Weapon::Scepter);
            s.shape = Shape::Cleric;
            s.aura = Some([1.0, 0.92, 0.55, 1.0]);
            s
        }
        NpcKind::QuestGiver => {
            let mut s = FigureStyle::humanoid([0.85, 0.65, 0.20, 1.0], vertex::SKIN, Weapon::None);
            s.aura = Some([0.95, 0.80, 0.30, 1.0]);
            s
        }
        NpcKind::Recruiter => {
            let mut s = FigureStyle::humanoid([0.20, 0.28, 0.55, 1.0], vertex::SKIN, Weapon::Sword);
            s.shape = Shape::Soldier;
            s.shield = Some([0.62, 0.62, 0.70, 1.0]);
            s.build = 1.18;
            s
        }
    }
}

/// Floating overhead marker, themed by NPC kind so they're scannable at a
/// glance from across the plaza.
#[allow(clippy::cast_possible_truncation)]
fn draw_npc_marker_for(batch: &mut SpriteBatch, cam: &Camera2D, pos: Position2D, kind: NpcKind) {
    let (outer, inner) = match kind {
        NpcKind::Merchant
        | NpcKind::WeaponMerchant
        | NpcKind::ArmorMerchant
        | NpcKind::PotionMerchant => (vertex::XP, [0.95, 0.82, 0.35, 1.0]),
        NpcKind::Blacksmith => ([0.85, 0.45, 0.18, 1.0], [1.0, 0.78, 0.30, 1.0]),
        NpcKind::Healer => ([1.0, 0.95, 0.65, 1.0], [1.0, 1.0, 1.0, 1.0]),
        NpcKind::QuestGiver => ([1.0, 0.85, 0.20, 1.0], [1.0, 1.0, 0.65, 1.0]),
        NpcKind::Recruiter => ([0.30, 0.55, 1.0, 1.0], [0.85, 0.90, 1.0, 1.0]),
    };
    let s = cam.world_to_screen(pos);
    let cy = s[1] - FIGURE_H * cam.zoom - 10.0;
    let r = 6.0 * cam.zoom.max(0.6);
    let ys = y_sort(pos) + 600;
    if matches!(kind, NpcKind::QuestGiver) {
        // Quest givers wear an exclamation: a dot above a tall bar.
        shapes::rect(batch, s[0] - 1.5, cy - r, 3.0, r * 1.4, RenderLayer::UiWorld, ys, 4, outer);
        shapes::circle(batch, s[0], cy + r * 0.7, 2.5, RenderLayer::UiWorld, ys, 5, inner);
    } else {
        shapes::circle(batch, s[0], cy, r, RenderLayer::UiWorld, ys, 4, outer);
        shapes::circle(batch, s[0], cy, r * 0.55, RenderLayer::UiWorld, ys, 5, inner);
    }
}

/// Render an NPC's display name floating just above the marker.
#[allow(clippy::cast_possible_truncation)]
fn draw_npc_name(batch: &mut SpriteBatch, cam: &Camera2D, pos: Position2D, name: &str) {
    let s = cam.world_to_screen(pos);
    let cy = s[1] - FIGURE_H * cam.zoom - 24.0;
    let ys = y_sort(pos) + 601;
    let px = (1.4 * cam.zoom).max(1.0);
    font::draw_text_centered(
        batch,
        name.as_bytes(),
        s[0],
        cy,
        px,
        [1.0, 0.95, 0.78, 1.0],
        RenderLayer::UiWorld,
        ys,
        3,
    );
}

/// Draw a floating health bar above each living mob (boss bars are wider).
/// Mobs that fully resist (≥ 1.0) one or more elements get a tiny row of
/// coloured pips above the bar — one per immune element, in
/// fire / frost / lightning / poison order.
#[allow(clippy::cast_possible_truncation)]
pub fn draw_health_bars(batch: &mut SpriteBatch, cam: &Camera2D, world: &World) {
    for e in world.mob_entities() {
        let Some(h) = world.health.get(&e) else { continue };
        if h.is_empty() {
            continue;
        }
        let Some(&pos) = world.positions.get(&e) else { continue };
        let s = cam.world_to_screen(pos);
        if !cam.on_screen(s, 96.0) {
            continue;
        }
        let is_boss = world
            .mob_types
            .get(&e)
            .and_then(|m| world.data.mob(m.def_id))
            .is_some_and(|d| d.is_boss);
        let (bw, fig_h) = if is_boss {
            (BOSS_W * cam.zoom, BOSS_H)
        } else {
            (FIGURE_W * cam.zoom, FIGURE_H)
        };
        let bh = if is_boss { 6.0 } else { 4.0 } * cam.zoom.max(0.6);
        let x = s[0] - bw * 0.5;
        let y = s[1] - fig_h * cam.zoom - bh - 3.0;
        let frac = h.fraction();
        let ys = y_sort(pos) + 500;
        push_quad(batch, [x - 1.0, y - 1.0], [bw + 2.0, bh + 2.0], RenderLayer::UiWorld, ys, 0, vertex::BLACK);
        // Red track, green fill (turns to red as it depletes).
        push_quad(batch, [x, y], [bw, bh], RenderLayer::UiWorld, ys, 1, [0.25, 0.05, 0.05, 1.0]);
        let fill = vertex::lerp([0.85, 0.15, 0.12, 1.0], [0.30, 0.85, 0.30, 1.0], frac);
        push_quad(batch, [x, y], [bw * frac, bh], RenderLayer::UiWorld, ys, 2, fill);

        // Immunity row above the bar. One pip per element fully resisted.
        if let Some(r) = world.elem_resists.get(&e) {
            const IMMUNE: f32 = 1.0 - f32::EPSILON;
            let pips: [(bool, Color); 4] = [
                (r.fire >= IMMUNE, [0.95, 0.35, 0.10, 1.0]),
                (r.frost >= IMMUNE, [0.45, 0.75, 1.0, 1.0]),
                (r.lightning >= IMMUNE, [1.0, 0.95, 0.30, 1.0]),
                (r.poison >= IMMUNE, [0.55, 0.85, 0.30, 1.0]),
            ];
            let pip = (bh * 0.9).max(3.0);
            let gap = pip * 0.4;
            let active: Vec<Color> = pips
                .iter()
                .filter_map(|(on, c)| if *on { Some(*c) } else { None })
                .collect();
            if !active.is_empty() {
                let total_w = active.len() as f32 * pip + (active.len().saturating_sub(1)) as f32 * gap;
                let mut px = s[0] - total_w * 0.5;
                let py = y - pip - 2.0;
                for color in active {
                    push_quad(
                        batch,
                        [px - 0.5, py - 0.5],
                        [pip + 1.0, pip + 1.0],
                        RenderLayer::UiWorld,
                        ys,
                        3,
                        vertex::BLACK,
                    );
                    push_quad(
                        batch,
                        [px, py],
                        [pip, pip],
                        RenderLayer::UiWorld,
                        ys,
                        4,
                        color,
                    );
                    px += pip + gap;
                }
            }
        }
    }
}

/// Draw every in-flight projectile: physical arrows render as a short brown
/// segment oriented along travel, magic projectiles as a small glowing circle.
/// Drawn on [`RenderLayer::Projectiles`], so they sit above tiles/decals and
/// below UI overlays.
#[allow(clippy::cast_possible_truncation)]
pub fn draw_projectiles(batch: &mut SpriteBatch, cam: &Camera2D, world: &World) {
    for (idx, proj) in world.projectiles.iter().enumerate() {
        let pos = Position2D::new(proj.x, proj.y);
        let s = cam.world_to_screen(pos);
        if !cam.on_screen(s, 32.0) {
            continue;
        }
        let ys = y_sort(pos) + 200;
        let sub = idx as u32 & 0xFFFF;
        if proj.is_magic {
            // Magic orb: soft halo + bright core. Colour stays neutral white
            // for plain magic projectiles; future code can pick fire/frost/etc.
            let r = 6.0 * cam.zoom.max(0.6);
            shapes::circle(batch, s[0], s[1], r, RenderLayer::Projectiles, ys, sub, [1.0, 1.0, 0.95, 0.55]);
            shapes::circle(batch, s[0], s[1], r * 0.5, RenderLayer::Projectiles, ys, sub + 1, [1.0, 1.0, 1.0, 1.0]);
        } else {
            // Arrow: short brown segment pointing in the direction of travel.
            let Some(&tpos) = world.positions.get(&proj.target) else { continue };
            let ts = cam.world_to_screen(tpos);
            let dx = ts[0] - s[0];
            let dy = ts[1] - s[1];
            let len = (dx * dx + dy * dy).sqrt().max(0.001);
            let inv = 1.0 / len;
            let half = (10.0 * cam.zoom.max(0.6)).max(4.0);
            let nose = [s[0] + dx * inv * half, s[1] + dy * inv * half];
            let tail = [s[0] - dx * inv * half, s[1] - dy * inv * half];
            shapes::line(batch, tail, nose, 2.5 * cam.zoom.max(0.6), RenderLayer::Projectiles, ys, sub, [0.45, 0.32, 0.18, 1.0]);
            // A small darker tip dot.
            shapes::circle(batch, nose[0], nose[1], 1.4 * cam.zoom.max(0.6), RenderLayer::Projectiles, ys, sub + 1, [0.25, 0.18, 0.10, 1.0]);
        }
    }
}

/// Draw the small cyan "link" diamond above a summon, marking it as owned.
#[allow(clippy::cast_possible_truncation)]
pub fn draw_link_marker(batch: &mut SpriteBatch, cam: &Camera2D, pos: Position2D) {
    let s = cam.world_to_screen(pos);
    let size = 8.0 * cam.zoom.max(0.6);
    let top = s[1] - FIGURE_H * cam.zoom - size - 8.0;
    const BANDS: usize = 5;
    let band_h = size * 2.0 / BANDS as f32;
    let ys = y_sort(pos) + 600;
    for i in 0..BANDS {
        #[allow(clippy::cast_precision_loss)]
        let t = (i as f32 / (BANDS as f32 - 1.0) - 0.5).abs() * 2.0; // 1 at ends, 0 centre
        let w = (size * (1.0 - t)).max(1.0);
        let y = top + i as f32 * band_h;
        push_quad(
            batch,
            [s[0] - w * 0.5, y],
            [w, band_h],
            RenderLayer::UiWorld,
            ys,
            5,
            vertex::SUMMON,
        );
    }
}

/// Draw health bars over summons and living mercenaries, plus a name label
/// ("Guardian Lv.3") over each mercenary.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn draw_ally_overlays(batch: &mut SpriteBatch, cam: &Camera2D, world: &World) {
    let mut allies: Vec<Entity> = world.summon_entities();
    allies.extend(
        world
            .mercenary_entities()
            .into_iter()
            .filter(|e| !world.merc_states.get(e).is_some_and(|st| st.is_dead())),
    );

    for e in allies {
        let Some(h) = world.health.get(&e) else { continue };
        if h.is_empty() {
            continue;
        }
        let Some(&pos) = world.positions.get(&e) else { continue };
        let s = cam.world_to_screen(pos);
        if !cam.on_screen(s, 96.0) {
            continue;
        }
        let bw = FIGURE_W * cam.zoom;
        let bh = 4.0 * cam.zoom.max(0.6);
        let x = s[0] - bw * 0.5;
        let y = s[1] - FIGURE_H * cam.zoom - bh - 3.0;
        let ys = y_sort(pos) + 500;
        push_quad(batch, [x - 1.0, y - 1.0], [bw + 2.0, bh + 2.0], RenderLayer::UiWorld, ys, 0, vertex::BLACK);
        push_quad(batch, [x, y], [bw, bh], RenderLayer::UiWorld, ys, 1, [0.10, 0.20, 0.10, 1.0]);
        let frac = h.fraction();
        // Allied fill leans on the ally palette so friend/foe bars read apart.
        let full = world.summon_tags.contains_key(&e);
        let full_color = if full { vertex::SUMMON } else { vertex::MERC };
        let fill = vertex::lerp([0.85, 0.15, 0.12, 1.0], full_color, frac);
        push_quad(batch, [x, y], [bw * frac, bh], RenderLayer::UiWorld, ys, 2, fill);

        // Mercenary name + level above the bar.
        if let Some(tag) = world.mercenary_tags.get(&e) {
            let level = world.merc_states.get(&e).map_or(1, |st| st.level);
            let (lvl_buf, lvl_len) = font::fmt_u32(level);
            let mut label: Vec<u8> = Vec::new();
            label.extend_from_slice(tag.merc_type.display_name().as_bytes());
            label.extend_from_slice(b" Lv.");
            label.extend_from_slice(&lvl_buf[..lvl_len]);
            let px = (1.4 * cam.zoom).max(1.0);
            font::draw_text_centered(
                batch,
                &label,
                s[0],
                y - font::CELL * px - 2.0,
                px,
                vertex::MERC,
                RenderLayer::UiWorld,
                ys,
                3,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deeper_entities_sort_later() {
        let near = y_sort(Position2D::new(5.0, 5.0));
        let far = y_sort(Position2D::new(1.0, 1.0));
        assert!(near > far);
    }

    #[test]
    fn boss_color_overrides_behavior() {
        assert_eq!(mob_color(AiBehavior::Ranged, true), vertex::MOB_BOSS);
        assert_eq!(mob_color(AiBehavior::Ranged, false), vertex::MOB_RANGED);
    }
}
