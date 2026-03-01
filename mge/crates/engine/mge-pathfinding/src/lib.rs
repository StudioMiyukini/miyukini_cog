// @id: MGE-Pathfinding-Lib @do: module-root @role: back-end @layer: 2 @human: miyuk

//! mge-pathfinding -- A* tile-based pathfinding for the Sodomight isometric grid.
//!
//! Provides grid-based navigation on an 8-directional tile map with
//! Chebyshev / octile heuristic, iteration-budgeted A*, and optional
//! Bresenham line-of-sight path smoothing.

pub mod coord;
pub mod grid;
pub mod path;
pub mod smoother;

#[cfg(test)]
mod tests;

pub use coord::TileCoord;
pub use grid::NavGrid;
pub use path::{AStarPathfinder, PathResult};
pub use smoother::PathSmoother;

/// Errors that can occur before a pathfinding query starts.
#[derive(Debug, thiserror::Error)]
pub enum PathfindingError {
    /// The start position lies outside the grid boundaries.
    #[error("Start position {0:?} is out of bounds")]
    StartOutOfBounds(TileCoord),
    /// The goal position lies outside the grid boundaries.
    #[error("Goal position {0:?} is out of bounds")]
    GoalOutOfBounds(TileCoord),
}
