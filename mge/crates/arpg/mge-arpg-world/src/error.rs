// @id: MGE-ARPG-World-Error @do: world-error @role: back-end @layer: 3 @human: miyuk

//! World system error types.

/// Errors that can occur within the ARPG world system.
#[derive(Debug, thiserror::Error)]
pub enum WorldError {
    /// The requested zone does not exist.
    #[error("Zone not found: '{id}'")]
    ZoneNotFound {
        /// The zone identifier that was not found.
        id: String,
    },
    /// Chunk coordinates are outside the valid range.
    #[error("Chunk out of bounds: ({cx}, {cy})")]
    ChunkOutOfBounds {
        /// Chunk x coordinate.
        cx: u32,
        /// Chunk y coordinate.
        cy: u32,
    },
    /// Tile coordinates are outside the valid range for a chunk.
    #[error("Invalid tile coordinate: ({x}, {y})")]
    InvalidTileCoord {
        /// Tile x coordinate.
        x: usize,
        /// Tile y coordinate.
        y: usize,
    },
    /// The requested waypoint does not exist.
    #[error("Waypoint not found: '{id}'")]
    WaypointNotFound {
        /// The waypoint identifier that was not found.
        id: String,
    },
}
