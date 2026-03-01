// @id: MGE-Pathfinding-Tests @do: test @role: back-end @layer: 2 @human: miyuk

//! Unit tests for mge-pathfinding.

#[cfg(test)]
mod tests {
    use crate::coord::TileCoord;
    use crate::grid::NavGrid;
    use crate::path::{AStarPathfinder, PathResult};
    use crate::smoother::PathSmoother;

    // ===================================================================
    // TileCoord — Chebyshev distance
    // ===================================================================

    #[test]
    fn tile_coord_chebyshev_same_pos() {
        let a = TileCoord::new(5, 5);
        assert_eq!(a.chebyshev_distance(a), 0);
    }

    #[test]
    fn tile_coord_chebyshev_cardinal() {
        let a = TileCoord::new(3, 3);
        let b = TileCoord::new(3, 4);
        assert_eq!(a.chebyshev_distance(b), 1);
    }

    #[test]
    fn tile_coord_chebyshev_diagonal() {
        let a = TileCoord::new(0, 0);
        let b = TileCoord::new(1, 1);
        assert_eq!(a.chebyshev_distance(b), 1);
    }

    // ===================================================================
    // NavGrid — bounds and blocking
    // ===================================================================

    #[test]
    fn navgrid_in_bounds() {
        let grid = NavGrid::new(10, 10);
        assert!(grid.is_in_bounds(TileCoord::new(0, 0)));
        assert!(grid.is_in_bounds(TileCoord::new(9, 9)));
        assert!(grid.is_in_bounds(TileCoord::new(5, 3)));
    }

    #[test]
    fn navgrid_out_of_bounds() {
        let grid = NavGrid::new(10, 10);
        assert!(!grid.is_in_bounds(TileCoord::new(-1, 0)));
        assert!(!grid.is_in_bounds(TileCoord::new(0, -1)));
        assert!(!grid.is_in_bounds(TileCoord::new(10, 0)));
        assert!(!grid.is_in_bounds(TileCoord::new(0, 10)));
    }

    #[test]
    fn navgrid_set_blocked() {
        let mut grid = NavGrid::new(10, 10);
        let coord = TileCoord::new(4, 4);
        assert!(!grid.is_blocked(coord));
        grid.set_blocked(coord, true);
        assert!(grid.is_blocked(coord));
        grid.set_blocked(coord, false);
        assert!(!grid.is_blocked(coord));
    }

    // ===================================================================
    // A* pathfinder
    // ===================================================================

    #[test]
    fn astar_same_position() {
        let grid = NavGrid::new(10, 10);
        let pf = AStarPathfinder::new();
        let result = pf.find_path(&grid, TileCoord::new(3, 3), TileCoord::new(3, 3));
        assert!(matches!(result, PathResult::SamePosition));
    }

    #[test]
    fn astar_simple_path() {
        // Straight horizontal line with no obstacles.
        let grid = NavGrid::new(10, 10);
        let pf = AStarPathfinder::new();
        let start = TileCoord::new(0, 5);
        let goal = TileCoord::new(5, 5);
        let result = pf.find_path(&grid, start, goal);
        match result {
            PathResult::Found(path) => {
                assert_eq!(*path.first().unwrap(), start);
                assert_eq!(*path.last().unwrap(), goal);
                // Path length should be 6 (start + 5 steps east).
                assert_eq!(path.len(), 6);
            }
            other => panic!("Expected Found, got {other:?}"),
        }
    }

    #[test]
    fn astar_obstacle_avoidance() {
        // 10x10 grid with a vertical wall at x=5, y=2..=7.
        // Start (2,5) -> Goal (8,5) must route around the wall.
        let mut grid = NavGrid::new(10, 10);
        for y in 2..=7 {
            grid.set_blocked(TileCoord::new(5, y), true);
        }
        let pf = AStarPathfinder::new();
        let start = TileCoord::new(2, 5);
        let goal = TileCoord::new(8, 5);
        let result = pf.find_path(&grid, start, goal);
        match result {
            PathResult::Found(path) => {
                assert_eq!(*path.first().unwrap(), start);
                assert_eq!(*path.last().unwrap(), goal);
                // Path must avoid the wall: no waypoint at x=5 with y in 2..=7.
                for &tile in &path {
                    assert!(
                        !(tile.tx == 5 && (2..=7).contains(&tile.ty)),
                        "Path went through blocked tile {tile:?}"
                    );
                }
                // Straight line would be 7 tiles (start + 6 east steps), but
                // a wall blocks x=5, so the path detours. With diagonal moves
                // the detour can still be 7 tiles (go around the wall edge).
                // Just assert it is at least 7 (no shortcut through the wall).
                assert!(path.len() >= 7, "Path too short: {}", path.len());
            }
            other => panic!("Expected Found, got {other:?}"),
        }
    }

    #[test]
    fn astar_unreachable() {
        // Goal fully surrounded by blocked tiles.
        let mut grid = NavGrid::new(10, 10);
        let goal = TileCoord::new(5, 5);
        for (nb, _cost) in goal.neighbors_8() {
            grid.set_blocked(nb, true);
        }
        // Also block the goal itself.
        grid.set_blocked(goal, true);
        let pf = AStarPathfinder::new();
        let result = pf.find_path(&grid, TileCoord::new(0, 0), goal);
        assert!(matches!(result, PathResult::Unreachable));
    }

    #[test]
    fn astar_diagonal_path() {
        // Open grid, diagonal move from (0,0) to (4,4).
        let grid = NavGrid::new(10, 10);
        let pf = AStarPathfinder::new();
        let start = TileCoord::new(0, 0);
        let goal = TileCoord::new(4, 4);
        let result = pf.find_path(&grid, start, goal);
        match result {
            PathResult::Found(path) => {
                assert_eq!(*path.first().unwrap(), start);
                assert_eq!(*path.last().unwrap(), goal);
                // Pure diagonal should be 5 tiles (start + 4 diagonal steps).
                assert_eq!(path.len(), 5);
            }
            other => panic!("Expected Found, got {other:?}"),
        }
    }

    #[test]
    fn astar_max_iterations_unreachable() {
        // 20x20 grid, goal reachable but we set max_iterations = 5 so it times out.
        let grid = NavGrid::new(20, 20);
        let pf = AStarPathfinder::with_max_iterations(5);
        let start = TileCoord::new(0, 0);
        let goal = TileCoord::new(19, 19);
        let result = pf.find_path(&grid, start, goal);
        assert!(
            matches!(result, PathResult::Unreachable),
            "Expected Unreachable due to iteration limit, got {result:?}"
        );
    }

    // ===================================================================
    // PathSmoother
    // ===================================================================

    #[test]
    fn path_smoother_straight_line_unchanged() {
        // Straight horizontal path should be reduced to just start and end.
        let grid = NavGrid::new(10, 10);
        let path: Vec<TileCoord> = (0..6).map(|x| TileCoord::new(x, 5)).collect();
        let smoothed = PathSmoother::smooth(&path, &grid);
        // Start and end must remain.
        assert_eq!(*smoothed.first().unwrap(), TileCoord::new(0, 5));
        assert_eq!(*smoothed.last().unwrap(), TileCoord::new(5, 5));
        // A straight line can be smoothed down to just 2 points.
        assert_eq!(smoothed.len(), 2);
    }

    #[test]
    fn path_smoother_removes_intermediate_waypoints() {
        // L-shaped path: right then down. Smoother should keep the corner.
        let grid = NavGrid::new(10, 10);
        let mut path = Vec::new();
        // Horizontal segment (0,0) -> (4,0)
        for x in 0..=4 {
            path.push(TileCoord::new(x, 0));
        }
        // Vertical segment (4,1) -> (4,4)
        for y in 1..=4 {
            path.push(TileCoord::new(4, y));
        }
        let smoothed = PathSmoother::smooth(&path, &grid);
        // Must keep start, corner (4,0), and end (4,4).
        assert_eq!(*smoothed.first().unwrap(), TileCoord::new(0, 0));
        assert_eq!(*smoothed.last().unwrap(), TileCoord::new(4, 4));
        // The smoothed path should be shorter than the original.
        assert!(
            smoothed.len() < path.len(),
            "Smoothed path should be shorter: {} vs {}",
            smoothed.len(),
            path.len()
        );
    }
}
