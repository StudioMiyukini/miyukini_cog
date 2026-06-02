//! Movement integration plus an A* pathfinder over the tilemap.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use crate::components::world::MapData;
use crate::components::Position2D;
use crate::ecs::World;
use crate::render::figures::Direction;

const STRAIGHT: i32 = 10;
const DIAGONAL: i32 = 14;

const NEIGHBORS: [(i32, i32); 8] =
    [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn heuristic(a: (i32, i32), b: (i32, i32)) -> i32 {
    let dx = (a.0 - b.0).abs();
    let dy = (a.1 - b.1).abs();
    STRAIGHT * (dx + dy) + (DIAGONAL - 2 * STRAIGHT) * dx.min(dy)
}

/// A* on the tile grid (8-connected). Returns the path from `start` to `goal`
/// excluding the start tile, or `None` if unreachable.
pub fn find_path(map: &MapData, start: (i32, i32), goal: (i32, i32)) -> Option<Vec<(i32, i32)>> {
    if !map.is_walkable(goal.0, goal.1) || !map.is_walkable(start.0, start.1) {
        return None;
    }
    if start == goal {
        return Some(Vec::new());
    }

    let mut open: BinaryHeap<Reverse<(i32, (i32, i32))>> = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), i32> = HashMap::new();

    g_score.insert(start, 0);
    open.push(Reverse((heuristic(start, goal), start)));

    while let Some(Reverse((_, current))) = open.pop() {
        if current == goal {
            return Some(reconstruct(&came_from, current));
        }
        let current_g = *g_score.get(&current).unwrap_or(&i32::MAX);
        for (dx, dy) in NEIGHBORS {
            let nx = current.0 + dx;
            let ny = current.1 + dy;
            if !map.is_walkable(nx, ny) {
                continue;
            }
            // Forbid cutting through wall corners on diagonals.
            if dx != 0 && dy != 0 && (!map.is_walkable(current.0 + dx, current.1) || !map.is_walkable(current.0, current.1 + dy)) {
                continue;
            }
            let step = if dx != 0 && dy != 0 { DIAGONAL } else { STRAIGHT };
            let tentative = current_g + step;
            let neighbor = (nx, ny);
            if tentative < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative);
                open.push(Reverse((tentative + heuristic(neighbor, goal), neighbor)));
            }
        }
    }
    None
}

fn reconstruct(came_from: &HashMap<(i32, i32), (i32, i32)>, mut current: (i32, i32)) -> Vec<(i32, i32)> {
    let mut path = vec![current];
    while let Some(&prev) = came_from.get(&current) {
        current = prev;
        path.push(current);
    }
    path.pop(); // drop the start tile
    path.reverse();
    path
}

/// Advance every entity along its `MoveTarget` path.
pub fn movement_system(world: &mut World) {
    let dt = world.dt;
    if dt <= 0.0 {
        return;
    }
    for e in world.move_targets.keys().copied().collect::<Vec<_>>() {
        // Knocked-down or being-pushed entities don't follow their own path: the
        // CC systems own their position (or freeze it) until the effect clears.
        if world.knockdowns.contains_key(&e) || world.pushbacks.contains_key(&e) {
            continue;
        }
        let (waypoint, speed, done) = {
            let Some(mt) = world.move_targets.get(&e) else { continue };
            (mt.current_waypoint(), mt.speed, mt.is_done())
        };
        if done {
            world.move_targets.remove(&e);
            continue;
        }
        let Some((wx, wy)) = waypoint else {
            world.move_targets.remove(&e);
            continue;
        };
        let target = Position2D::from_tile(wx, wy);
        let Some(&pos) = world.positions.get(&e) else { continue };
        // Face the way we are travelling.
        if let Some(dir) = Direction::from_world_vector(target.x - pos.x, target.y - pos.y) {
            world.directions.insert(e, dir);
        }
        let dist = pos.distance(target);
        let slow = world.slows.get(&e).map_or(1.0, |s| s.factor.clamp(0.05, 1.0));
        let step = speed * slow * dt;
        if dist <= 1e-4 || step >= dist {
            world.positions.insert(e, target);
            if let Some(mt) = world.move_targets.get_mut(&e) {
                mt.idx += 1;
                if mt.is_done() {
                    world.move_targets.remove(&e);
                }
            }
        } else {
            let nx = pos.x + (target.x - pos.x) / dist * step;
            let ny = pos.y + (target.y - pos.y) / dist * step;
            world.positions.insert(e, Position2D::new(nx, ny));
        }
    }
}
