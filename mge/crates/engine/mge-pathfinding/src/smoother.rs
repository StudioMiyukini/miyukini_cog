// @id: MGE-Pathfinding-Smoother @do: path-smooth @role: back-end @layer: 2 @human: miyuk

//! Path smoothing via line-of-sight checks (Bresenham).

use crate::coord::TileCoord;
use crate::grid::NavGrid;

/// Post-processing pass that removes unnecessary intermediate waypoints
/// from a path by checking direct line-of-sight on the `NavGrid`.
pub struct PathSmoother;

impl PathSmoother {
    /// Returns a smoothed copy of `path` with redundant waypoints removed.
    ///
    /// A waypoint is deemed redundant when there is a clear line-of-sight
    /// (no blocked tiles along the Bresenham line) between the previous
    /// retained waypoint and the waypoint *after* it.
    ///
    /// If the path has fewer than 3 points it is returned unchanged.
    pub fn smooth(path: &[TileCoord], grid: &NavGrid) -> Vec<TileCoord> {
        if path.len() < 3 {
            return path.to_vec();
        }

        let mut smoothed: Vec<TileCoord> = Vec::with_capacity(path.len());
        smoothed.push(path[0]);

        let mut anchor = 0_usize;
        let mut check = 2_usize;

        while check < path.len() {
            if line_of_sight(path[anchor], path[check], grid) {
                // We can skip the intermediate point -- try the next one.
                check += 1;
            } else {
                // Cannot skip: keep the previous point and advance anchor.
                smoothed.push(path[check - 1]);
                anchor = check - 1;
                check += 1;
            }
        }

        // Always include the final destination.
        if smoothed.last() != path.last() {
            if let Some(&last) = path.last() {
                smoothed.push(last);
            }
        }

        smoothed
    }
}

/// Bresenham line-of-sight: returns `true` if every tile on the line
/// from `a` to `b` (inclusive) is passable on `grid`.
fn line_of_sight(a: TileCoord, b: TileCoord, grid: &NavGrid) -> bool {
    let mut x0 = a.tx;
    let mut y0 = a.ty;
    let x1 = b.tx;
    let y1 = b.ty;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx: i32 = if x0 < x1 { 1 } else { -1 };
    let sy: i32 = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        let coord = TileCoord::new(x0, y0);
        if !grid.is_in_bounds(coord) || grid.is_blocked(coord) {
            return false;
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }

    true
}
