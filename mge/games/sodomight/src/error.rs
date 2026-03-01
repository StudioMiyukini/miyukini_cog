// @id: Sodomight-Game-Error @do: game-error @role: back-end @layer: 4 @human: miyuk
//! Game error types.

/// Top-level game error.
#[derive(Debug, thiserror::Error)]
pub enum GameError {
    /// Platform subsystem failure.
    #[error("Platform error: {message}")]
    PlatformError {
        /// Description of the platform error.
        message: String,
    },
    /// Asset loading failure.
    #[error("Asset loading error: {message}")]
    AssetError {
        /// Description of the asset error.
        message: String,
    },
    /// Save/load failure.
    #[error("Save/load error: {message}")]
    SaveError {
        /// Description of the save error.
        message: String,
    },
    /// Network failure.
    #[error("Network error: {message}")]
    NetworkError {
        /// Description of the network error.
        message: String,
    },
}
