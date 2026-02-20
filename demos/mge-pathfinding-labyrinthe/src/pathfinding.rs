//! Algorithme A* pour pathfinding — MGE Démo
//! Heuristique euclidienne, 8 directions, coût diagonal √2
//! Fonctionne sur ChunkManager (coordonnées monde) ou Grid (legacy tests)

use std::collections::{BinaryHeap, HashMap};

use crate::grid::{GridNode, Vec2};

const COST_ORTHOGONAL: f32 = 1.0;
const COST_DIAGONAL: f32 = 1.414;

#[derive(Debug, Clone)]
pub struct PathResult {
    pub waypoints: Vec<Vec2>,
    pub total_cost: f32,
    pub found: bool,
}

#[derive(Debug, Clone)]
pub struct PathfindingConfig {
    pub allow_diagonal: bool,
    pub diagonal_cost: f32,
    pub max_iterations: usize,
}

impl Default for PathfindingConfig {
    fn default() -> Self {
        Self {
            allow_diagonal: true,
            diagonal_cost: COST_DIAGONAL,
            max_iterations: 10000,
        }
    }
}

/// Ordre inverse pour BinaryHeap (min-heap sur f)
#[derive(Clone, Copy, PartialEq)]
struct NodeCost {
    node: GridNode,
    f: f32,
}

impl Eq for NodeCost {}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

fn heuristic(a: GridNode, b: GridNode) -> f32 {
    let dx = (b.x - a.x) as f32;
    let dy = (b.y - a.y) as f32;
    (dx * dx + dy * dy).sqrt()
}

/// Trait abstrait pour fournir les données de terrain au pathfinding
pub trait TerrainProvider {
    fn is_walkable(&self, x: i32, y: i32) -> bool;
    fn movement_cost_at(&self, x: i32, y: i32) -> f32;
}

fn neighbors<T: TerrainProvider>(
    node: GridNode,
    terrain: &T,
    config: &PathfindingConfig,
) -> Vec<(GridNode, f32)> {
    let mut result = Vec::with_capacity(8);

    let dirs_4 = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    for (dx, dy) in dirs_4 {
        let nn = GridNode::new(node.x + dx, node.y + dy);
        let terrain_cost = terrain.movement_cost_at(nn.x, nn.y);
        if terrain_cost.is_finite() {
            result.push((nn, COST_ORTHOGONAL * terrain_cost));
        }
    }

    if config.allow_diagonal {
        let dirs_diag = [(1, -1), (1, 1), (-1, 1), (-1, -1)];
        for (dx, dy) in dirs_diag {
            let nn = GridNode::new(node.x + dx, node.y + dy);
            let terrain_cost = terrain.movement_cost_at(nn.x, nn.y);
            if terrain_cost.is_finite() {
                let o1 = terrain.is_walkable(node.x + dx, node.y);
                let o2 = terrain.is_walkable(node.x, node.y + dy);
                if o1 || o2 {
                    result.push((nn, config.diagonal_cost * terrain_cost));
                }
            }
        }
    }

    result
}

/// A* générique sur n'importe quel TerrainProvider
pub fn find_path<T: TerrainProvider>(
    terrain: &T,
    start: GridNode,
    goal: GridNode,
    tile_size: f32,
    config: &PathfindingConfig,
) -> PathResult {
    if !terrain.is_walkable(start.x, start.y) || !terrain.is_walkable(goal.x, goal.y) {
        return PathResult {
            waypoints: Vec::new(),
            total_cost: 0.0,
            found: false,
        };
    }

    if start.x == goal.x && start.y == goal.y {
        return PathResult {
            waypoints: vec![GridNode::new(start.x, start.y).to_vec2(tile_size)],
            total_cost: 0.0,
            found: true,
        };
    }

    let mut open = BinaryHeap::new();
    let mut g_score: HashMap<(i32, i32), f32> = HashMap::new();
    let mut parent: HashMap<(i32, i32), GridNode> = HashMap::new();

    g_score.insert((start.x, start.y), 0.0);
    open.push(NodeCost {
        node: start,
        f: heuristic(start, goal),
    });

    let mut iterations = 0;

    while let Some(NodeCost { node, .. }) = open.pop() {
        iterations += 1;
        if iterations > config.max_iterations {
            break;
        }

        let g_current = *g_score.get(&(node.x, node.y)).unwrap_or(&f32::INFINITY);

        if node.x == goal.x && node.y == goal.y {
            let mut waypoints = Vec::new();
            let mut current = goal;
            while current.x != start.x || current.y != start.y {
                waypoints.push(current.to_vec2(tile_size));
                if let Some(&p) = parent.get(&(current.x, current.y)) {
                    current = p;
                } else {
                    break;
                }
            }
            waypoints.push(start.to_vec2(tile_size));
            waypoints.reverse();

            let total_cost = *g_score.get(&(goal.x, goal.y)).unwrap_or(&0.0);

            return PathResult {
                waypoints,
                total_cost,
                found: true,
            };
        }

        for (neighbor, cost) in neighbors(node, terrain, config) {
            let g_new = g_current + cost;
            let key = (neighbor.x, neighbor.y);
            if g_new < *g_score.get(&key).unwrap_or(&f32::INFINITY) {
                g_score.insert(key, g_new);
                parent.insert(key, node);
                let h = heuristic(neighbor, goal);
                open.push(NodeCost {
                    node: neighbor,
                    f: g_new + h,
                });
            }
        }
    }

    PathResult {
        waypoints: Vec::new(),
        total_cost: 0.0,
        found: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Grid;

    impl TerrainProvider for Grid {
        fn is_walkable(&self, x: i32, y: i32) -> bool {
            self.is_walkable(x, y)
        }
        fn movement_cost_at(&self, x: i32, y: i32) -> f32 {
            self.movement_cost_at(x, y)
        }
    }

    #[test]
    fn chemin_direct_sans_obstacle() {
        let grid = Grid::new(10, 10);
        let start = GridNode::new(1, 1);
        let goal = GridNode::new(5, 5);
        let result = find_path(&grid, start, goal, 32.0, &PathfindingConfig::default());
        assert!(result.found);
        assert!(result.waypoints.len() >= 2);
    }

    #[test]
    fn obstacle_bloque() {
        let mut grid = Grid::new(5, 5);
        grid.set_obstacle(1, 2, true);
        grid.set_obstacle(2, 2, true);
        grid.set_obstacle(3, 2, true);
        let start = GridNode::new(0, 2);
        let goal = GridNode::new(4, 2);
        let result = find_path(&grid, start, goal, 32.0, &PathfindingConfig::default());
        assert!(result.found);
        assert!(result.waypoints.len() > 4);
    }

    #[test]
    fn cible_dans_obstacle() {
        let mut grid = Grid::new(5, 5);
        grid.set_obstacle(2, 2, true);
        let start = GridNode::new(0, 0);
        let goal = GridNode::new(2, 2);
        let result = find_path(&grid, start, goal, 32.0, &PathfindingConfig::default());
        assert!(!result.found);
    }

    #[test]
    fn terrain_cout_prefere_route() {
        use crate::grid::Terrain;

        let mut grid = Grid::new(6, 3);
        for x in 0..6 {
            grid.set_terrain(x, 1, Terrain::Sand);
        }
        let start = GridNode::new(0, 0);
        let goal = GridNode::new(5, 0);
        let result_grass = find_path(&grid, start, goal, 32.0, &PathfindingConfig::default());
        assert!(result_grass.found);
        let cost_grass = result_grass.total_cost;

        let start2 = GridNode::new(0, 1);
        let goal2 = GridNode::new(5, 1);
        let result_sand = find_path(&grid, start2, goal2, 32.0, &PathfindingConfig::default());
        assert!(result_sand.found);
        let cost_sand = result_sand.total_cost;
        assert!(cost_sand > cost_grass, "sable (1.5x) doit couter plus que herbe (1.0x)");
    }
}
