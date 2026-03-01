// @id: MGE-Collision-Rich-Error @do: rich-collision-error @role: back-end @layer: 2 @human: miyuk

//! Error types for rich collision shapes.

/// Errors that can occur when constructing or using rich collision shapes.
#[derive(Debug, thiserror::Error)]
pub enum CollisionRichError {
    /// The shape is degenerate (e.g. zero-radius circle, zero-length capsule).
    #[error("Degenerate shape: {message}")]
    DegenerateShape {
        /// Human-readable description of the degeneracy.
        message: String,
    },
    /// An invalid parameter was provided.
    #[error("Invalid parameter: {message}")]
    InvalidParameter {
        /// Human-readable description of the invalid parameter.
        message: String,
    },
}
