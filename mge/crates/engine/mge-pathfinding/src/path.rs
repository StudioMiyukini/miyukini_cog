// @id: MGE-Pathfinding-AStar @do: pathfind @role: back-end @layer: 2 @human: miyuk

//! A* pathfinder on a tile grid with Chebyshev heuristic.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use crate::coord::TileCoord;
use crate::grid::NavGrid;

/// The result of a pathfinding query.
#[derive(Debug, Clone)]
pub enum PathResult {
    /// A complete path from start to goal (both endpoints included).
    Found(Vec<TileCoord>),
    /// The goal is unreachable (blocked, out of bounds, or iteration budget exhausted).
    Unreachable,
    /// Start and goal are the same tile.
    SamePosition,
}

/// Default iteration limit to prevent infinite loops on degenerate maps.
const DEFAULT_MAX_ITERATIONS: u32 = 10_000;

/// Cardinal cost constant used in the heuristic estimate.
const CARDINAL_COST: u32 = 10;
/// Diagonal cost constant used in the heuristic estimate.
const DIAGONAL_COST: u32 = 14;

/// An A* pathfinder operating on a [`NavGrid`].
///
/// The heuristic is an octile (Chebyshev-weighted) distance to stay
/// admissible on 8-directional grids with cardinal cost 10 / diagonal cost 14.
pub struct AStarPathfinder {
    max_iterations: u32,
}

impl Default for AStarPathfinder {
    fn default() -> Self {
        Self::new()
    }
}

impl AStarPathfinder {
    /// Creates a pathfinder with the default iteration budget (10 000).
    #[inline]
    pub fn new() -> Self {
        Self {
            max_iterations: DEFAULT_MAX_ITERATIONS,
        }
    }

    /// Creates a pathfinder with a custom iteration budget.
    #[inline]
    pub fn with_max_iterations(max: u32) -> Self {
        Self {
            max_iterations: max,
        }
    }

    /// Finds a path from `start` to `goal` on the given `grid`.
    ///
    /// Returns [`PathResult::Found`] with the full tile sequence (including
    /// both endpoints), [`PathResult::SamePosition`] if start equals goal,
    /// or [`PathResult::Unreachable`] otherwise.
    pub fn find_path(&self, grid: &NavGrid, start: TileCoord, goal: TileCoord) -> PathResult {
        // Trivial: same position.
        if start == goal {
            return PathResult::SamePosition;
        }

        // Early-out for out-of-bounds or blocked endpoints.
        if !grid.is_in_bounds(start)
            || !grid.is_in_bounds(goal)
            || grid.is_blocked(start)
            || grid.is_blocked(goal)
        {
            return PathResult::Unreachable;
        }

        // ---------- A* core ----------

        // Min-heap ordered by f_score (via Reverse).
        let mut open: BinaryHeap<Reverse<(u32, i32, i32)>> = BinaryHeap::new();
        let mut g_score: HashMap<TileCoord, u32> = HashMap::new();
        let mut came_from: HashMap<TileCoord, TileCoord> = HashMap::new();

        g_score.insert(start, 0);
        open.push(Reverse((heuristic(start, goal), start.tx, start.ty)));

        let mut iterations: u32 = 0;

        while let Some(Reverse((_f, cx, cy))) = open.pop() {
            iterations += 1;
            if iterations > self.max_iterations {
                return PathResult::Unreachable;
            }

            let current = TileCoord::new(cx, cy);

            if current == goal {
                return PathResult::Found(reconstruct(&came_from, current));
            }

            let Some(&current_g) = g_score.get(&current) else {
                continue;
            };

            for (neighbor, move_cost) in current.neighbors_8() {
                if !grid.is_in_bounds(neighbor) || grid.is_blocked(neighbor) {
                    continue;
                }

                let tentative_g = current_g + move_cost;
                let prev_g = g_score.get(&neighbor).copied().unwrap_or(u32::MAX);

                if tentative_g < prev_g {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g);
                    let f = tentative_g + heuristic(neighbor, goal);
                    open.push(Reverse((f, neighbor.tx, neighbor.ty)));
                }
            }
        }

        PathResult::Unreachable
    }
}

/// Octile heuristic: accounts for the different costs of cardinal vs diagonal moves.
///
/// `h = DIAGONAL_COST * min(dx, dy) + CARDINAL_COST * (max(dx, dy) - min(dx, dy))`
#[inline]
fn heuristic(a: TileCoord, b: TileCoord) -> u32 {
    let dx = (a.tx - b.tx).unsigned_abs();
    let dy = (a.ty - b.ty).unsigned_abs();
    let diag = if dx < dy { dx } else { dy };
    let straight = if dx > dy { dx } else { dy };
    DIAGONAL_COST * diag + CARDINAL_COST * (straight - diag)
}

/// Reconstructs the path from start to `current` by following `came_from` links.
fn reconstruct(came_from: &HashMap<TileCoord, TileCoord>, current: TileCoord) -> Vec<TileCoord> {
    let mut path = vec![current];
    let mut node = current;
    while let Some(&prev) = came_from.get(&node) {
        path.push(prev);
        node = prev;
    }
    path.reverse();
    path
}
