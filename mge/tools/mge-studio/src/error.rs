// @id: MGE-Studio-Error @do: studio-error @role: back-end @layer: 6 @human: miyuk
//! Studio error types.

/// Errors returned by MGE Studio operations.
#[derive(Debug, thiserror::Error)]
pub enum StudioError {
    /// The project file was not found at the given path.
    #[error("Project file not found: '{path}'")]
    ProjectNotFound {
        /// Path that was searched.
        path: String,
    },
    /// The project file had an invalid format.
    #[error("Invalid project format: {message}")]
    InvalidFormat {
        /// Description of the format problem.
        message: String,
    },
    /// An error occurred in the atlas editor.
    #[error("Atlas error: {message}")]
    AtlasError {
        /// Description of the atlas problem.
        message: String,
    },
    /// An error occurred in the animation editor.
    #[error("Animation error: {message}")]
    AnimError {
        /// Description of the animation problem.
        message: String,
    },
    /// An error occurred in the map editor.
    #[error("Map error: {message}")]
    MapError {
        /// Description of the map problem.
        message: String,
    },
}
