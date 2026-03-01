// @id: Sodomight-Client-Error @do: client-error @role: back-end @layer: 4 @human: miyuk
//! Client error types.

/// Top-level client error.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// Connection attempt failed.
    #[error("Connection failed to '{addr}': {message}")]
    ConnectionFailed {
        /// The address that was targeted.
        addr: String,
        /// Description of the failure.
        message: String,
    },
    /// Rendering subsystem failure.
    #[error("Render error: {message}")]
    RenderError {
        /// Description of the render error.
        message: String,
    },
    /// Asset loading failure.
    #[error("Asset error: {message}")]
    AssetError {
        /// Description of the asset error.
        message: String,
    },
}
