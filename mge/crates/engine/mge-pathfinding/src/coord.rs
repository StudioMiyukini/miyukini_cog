// @id: MGE-Pathfinding-Coord @do: tile-coord @role: back-end @layer: 2 @human: miyuk

//! Tile coordinate type for grid-based pathfinding.

use core::fmt;

/// A position on the tile grid in integer coordinates.
///
/// Used for A* pathfinding on the Sodomight isometric grid.
/// `tx` and `ty` correspond to the column and row indices of the tile map.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileCoord {
    /// Tile column index (horizontal axis in tile space).
    pub tx: i32,
    /// Tile row index (vertical axis in tile space).
    pub ty: i32,
}

impl fmt::Debug for TileCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TileCoord({}, {})", self.tx, self.ty)
    }
}

/// Cost to move to a cardinally adjacent tile (up/down/left/right).
const CARDINAL_COST: u32 = 10;
/// Cost to move to a diagonally adjacent tile (integer approximation of 10 * sqrt(2)).
const DIAGONAL_COST: u32 = 14;

impl TileCoord {
    /// Creates a new tile coordinate.
    #[inline]
    pub const fn new(tx: i32, ty: i32) -> Self {
        Self { tx, ty }
    }

    /// Returns the 8 neighbouring tiles together with their movement cost.
    ///
    /// Cardinal neighbours cost 10, diagonal neighbours cost 14.
    /// The order is: N, NE, E, SE, S, SW, W, NW.
    #[inline]
    pub fn neighbors_8(self) -> [(Self, u32); 8] {
        let x = self.tx;
        let y = self.ty;
        [
            // Cardinals
            (Self::new(x, y - 1), CARDINAL_COST),     // N
            (Self::new(x + 1, y), CARDINAL_COST),     // E
            (Self::new(x, y + 1), CARDINAL_COST),     // S
            (Self::new(x - 1, y), CARDINAL_COST),     // W
            // Diagonals
            (Self::new(x + 1, y - 1), DIAGONAL_COST), // NE
            (Self::new(x + 1, y + 1), DIAGONAL_COST), // SE
            (Self::new(x - 1, y + 1), DIAGONAL_COST), // SW
            (Self::new(x - 1, y - 1), DIAGONAL_COST), // NW
        ]
    }

    /// Chebyshev distance (L-infinity) between two tile coordinates.
    ///
    /// On an 8-directional grid this is the minimum number of moves
    /// (each move being to any of the 8 neighbours) to reach `other`.
    #[inline]
    pub fn chebyshev_distance(self, other: Self) -> u32 {
        let dx = (self.tx - other.tx).unsigned_abs();
        let dy = (self.ty - other.ty).unsigned_abs();
        if dx > dy { dx } else { dy }
    }
}
