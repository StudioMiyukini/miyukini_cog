// @id: MGE-Pathfinding-Grid @do: nav-grid @role: back-end @layer: 2 @human: miyuk

//! Navigation grid storing per-tile walkability.

use crate::coord::TileCoord;

/// A rectangular grid of tiles tracking which ones are blocked.
///
/// Internally uses a flat `Vec<bool>` indexed as `ty * width + tx`.
/// All tiles are passable by default upon construction.
pub struct NavGrid {
    width: u32,
    height: u32,
    blocked: Vec<bool>,
}

impl NavGrid {
    /// Creates a new navigation grid where every tile is passable.
    pub fn new(width: u32, height: u32) -> Self {
        let len = (width as usize) * (height as usize);
        Self {
            width,
            height,
            blocked: vec![false; len],
        }
    }

    /// Marks a tile as blocked or passable.
    ///
    /// Does nothing if the coordinate is out of bounds.
    pub fn set_blocked(&mut self, coord: TileCoord, blocked: bool) {
        if let Some(idx) = self.index_of(coord) {
            self.blocked[idx] = blocked;
        }
    }

    /// Returns `true` if the tile is blocked or out of bounds.
    pub fn is_blocked(&self, coord: TileCoord) -> bool {
        match self.index_of(coord) {
            Some(idx) => self.blocked[idx],
            None => true,
        }
    }

    /// Returns `true` if the coordinate lies within the grid boundaries.
    pub fn is_in_bounds(&self, coord: TileCoord) -> bool {
        coord.tx >= 0
            && coord.ty >= 0
            && (coord.tx as u32) < self.width
            && (coord.ty as u32) < self.height
    }

    /// Grid width in tiles.
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Grid height in tiles.
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Converts a valid tile coordinate to its flat index.
    fn index_of(&self, coord: TileCoord) -> Option<usize> {
        if self.is_in_bounds(coord) {
            Some((coord.ty as usize) * (self.width as usize) + (coord.tx as usize))
        } else {
            None
        }
    }
}
