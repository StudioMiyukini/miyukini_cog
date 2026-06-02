//! Area-of-effect target selection. Every skill that hits "the living enemies
//! in some shape" used to re-spell the same `mob filter → health check →
//! position check` pipeline; this module is the single source of that logic,
//! parameterised by shape (circle / arc / line / rect). Results keep the
//! deterministic `BTreeMap` (entity-id) order, except [`on_line`] which sorts by
//! distance-along so piercing effects strike front-to-back.

use crate::components::Position2D;
use crate::ecs::{Entity, World};

/// Iterate every living mob with a position, in deterministic order.
fn living_mobs(world: &World) -> impl Iterator<Item = (Entity, Position2D)> + '_ {
    world
        .mob_tags
        .keys()
        .copied()
        .filter(|e| world.health.get(e).is_some_and(|h| !h.is_empty()))
        .filter_map(|e| world.positions.get(&e).map(|p| (e, *p)))
}

/// Living mobs whose position satisfies `inside`. The escape hatch for shapes
/// the named helpers don't cover (e.g. the flame wall's oriented rectangle).
#[must_use]
pub fn living_where(world: &World, inside: impl Fn(Position2D) -> bool) -> Vec<Entity> {
    living_mobs(world).filter(|(_, p)| inside(*p)).map(|(e, _)| e).collect()
}

/// Living mobs within `radius` of `center`.
#[must_use]
pub fn in_circle(world: &World, center: Position2D, radius: f32) -> Vec<Entity> {
    living_mobs(world)
        .filter(|(_, p)| center.distance(*p) <= radius)
        .map(|(e, _)| e)
        .collect()
}

/// Living mobs within an axis-aligned rectangle (`half_w` × `half_h` around
/// `center`). Used by ground fields (flame wall, blizzard).
#[must_use]
pub fn in_rect(world: &World, center: Position2D, half_w: f32, half_h: f32) -> Vec<Entity> {
    living_mobs(world)
        .filter(|(_, p)| (p.x - center.x).abs() <= half_w && (p.y - center.y).abs() <= half_h)
        .map(|(e, _)| e)
        .collect()
}

/// Living mobs within `radius` of `center` AND inside the cone of half-angle
/// `arc_deg/2` around the unit aim vector `aim`.
#[must_use]
pub fn in_arc(
    world: &World,
    center: Position2D,
    aim: (f32, f32),
    radius: f32,
    arc_deg: f32,
) -> Vec<Entity> {
    let half = (arc_deg * 0.5).to_radians();
    living_mobs(world)
        .filter(|(_, p)| center.distance(*p) <= radius)
        .filter(|(_, p)| {
            let (dx, dy) = (p.x - center.x, p.y - center.y);
            let len = (dx * dx + dy * dy).sqrt();
            if len <= 1e-3 {
                return true;
            }
            let dot = ((dx / len) * aim.0 + (dy / len) * aim.1).clamp(-1.0, 1.0);
            dot.acos() <= half
        })
        .map(|(e, _)| e)
        .collect()
}

/// Living mobs inside the forward corridor of length `range` and half-width
/// `half_width` along unit `dir` from `from`, sorted nearest-first so piercing
/// effects resolve front-to-back.
#[must_use]
pub fn on_line(
    world: &World,
    from: Position2D,
    dir: (f32, f32),
    range: f32,
    half_width: f32,
) -> Vec<Entity> {
    let mut out: Vec<(Entity, f32)> = living_mobs(world)
        .filter_map(|(e, p)| {
            let (dx, dy) = (p.x - from.x, p.y - from.y);
            let along = dx * dir.0 + dy * dir.1;
            let perp = -dx * dir.1 + dy * dir.0;
            (along > 0.0 && along <= range && perp.abs() <= half_width).then_some((e, along))
        })
        .collect();
    out.sort_by(|a, b| a.1.total_cmp(&b.1));
    out.into_iter().map(|(e, _)| e).collect()
}

/// Position of the nearest living mob to `from` (by squared distance).
#[must_use]
pub fn nearest_pos(world: &World, from: Position2D) -> Option<Position2D> {
    living_mobs(world)
        .map(|(_, p)| p)
        .min_by(|a, b| from.distance_sq(*a).total_cmp(&from.distance_sq(*b)))
}
